//! Жизненный цикл запущенной сборки.
//!
//! Стейт-машина, живой лог и определение готовности. Всё платформо-нейтрально:
//! спавн и убийство спрятаны за `ProcessSupervisor`.

use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// Сколько строк лога держим в памяти. Хватает, чтобы после возврата
/// на экран инстанса увидеть весь старт целиком, включая трейсбек падения.
const LOG_LIMIT: usize = 5000;

/// Холодный старт с кучей кастомных нод — это реально минуты.
pub const READY_TIMEOUT: Duration = Duration::from_secs(300);

/// Сколько ждём чужой сервер после неожиданного выхода процесса.
/// ComfyUI-Manager после установки нод поднимает новый процесс и гасит
/// старый: наш хэндл теряется, а порт остаётся занятым.
const RESPAWN_GRACE: Duration = Duration::from_secs(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum RunState {
    Stopped,
    Starting,
    Running,
    Stopping,
    /// Процесс завершился сам, и мы его об этом не просили.
    Crashed,
    /// Сервер на порту жив, но управляет им уже не наш процесс.
    Detached,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LogLine {
    /// `stdout` или `stderr`. ComfyUI пишет старт в stderr, и различать
    /// потоки полезно: по stdout видно, что сборка уже работает.
    pub stream: String,
    /// Содержимое не переводится никогда.
    pub text: String,
    /// Строка заменяет предыдущую, а не добавляется. Так ведёт себя tqdm:
    /// он печатает прогресс через `\r`, и без замены сотня обновлений
    /// превращается в сотню строк.
    pub replaces_last: bool,
}

/// Снимок состояния для интерфейса.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RunStatus {
    pub instance_id: String,
    pub state: RunState,
    pub port: Option<u16>,
    pub pid: Option<u32>,
    /// Момент запуска в миллисекундах эпохи.
    pub started_at: Option<f64>,
    /// Секунды до готовности. Появляется, когда сервер ответил.
    pub ready_secs: Option<u32>,
    /// Код выхода, если процесс завершился сам.
    pub exit_code: Option<i32>,
    /// Профиль, которым запускали.
    pub profile_id: Option<String>,
}

impl RunStatus {
    pub fn stopped(instance_id: &str) -> Self {
        Self {
            instance_id: instance_id.to_string(),
            state: RunState::Stopped,
            port: None,
            pid: None,
            started_at: None,
            ready_secs: None,
            exit_code: None,
            profile_id: None,
        }
    }
}

/// Кольцевой буфер строк одного инстанса.
#[derive(Default)]
pub struct LogBuffer(VecDeque<LogLine>);

impl LogBuffer {
    fn push(&mut self, line: LogLine) {
        if line.replaces_last {
            self.0.pop_back();
        }
        self.0.push_back(line);
        while self.0.len() > LOG_LIMIT {
            self.0.pop_front();
        }
    }

    pub fn snapshot(&self) -> Vec<LogLine> {
        self.0.iter().cloned().collect()
    }
}

/// Один запущенный инстанс.
pub struct Running {
    pub status: RunStatus,
    pub log: LogBuffer,
    /// Просили ли мы остановку. Отличает штатный выход от падения.
    pub stopping: bool,
}

/// Состояние всех запусков. Живёт в `tauri::State`.
#[derive(Default)]
pub struct Runtime {
    inner: Mutex<HashMap<String, Arc<Mutex<Running>>>>,
}

impl Runtime {
    pub fn get(&self, id: &str) -> Option<Arc<Mutex<Running>>> {
        self.inner.lock().unwrap().get(id).cloned()
    }

    /// Кладёт готовую ячейку, а не создаёт новую: в неё уже пишут треды
    /// чтения логов, и подмена стоила бы всех строк, пришедших до неё.
    pub fn insert(&self, id: &str, cell: Arc<Mutex<Running>>) {
        self.inner.lock().unwrap().insert(id.to_string(), cell);
    }

    pub fn remove(&self, id: &str) {
        self.inner.lock().unwrap().remove(id);
    }

    /// Состояния всех инстансов, о которых мы что-то знаем.
    pub fn statuses(&self) -> Vec<RunStatus> {
        self.inner
            .lock()
            .unwrap()
            .values()
            .map(|cell| cell.lock().unwrap().status.clone())
            .collect()
    }

    /// Занят ли инстанс: запущен или в процессе запуска либо остановки.
    pub fn is_busy(&self, id: &str) -> bool {
        self.get(id).is_some_and(|cell| {
            matches!(
                cell.lock().unwrap().status.state,
                RunState::Starting | RunState::Running | RunState::Stopping | RunState::Detached
            )
        })
    }
}

/// Разбирает поток на строки, понимая возврат каретки.
///
/// Наивное деление по `\n` превращает прогрессбар tqdm в десятки тысяч
/// строк и забивает буфер так, что настоящий старт из него вытесняется.
#[derive(Default)]
pub struct LineSplitter {
    pending: String,
    /// Предыдущая отданная строка закрыта возвратом каретки, то есть
    /// временная: следующая должна встать на её место.
    transient: bool,
}

impl LineSplitter {
    pub fn feed(&mut self, chunk: &str, mut emit: impl FnMut(String, bool)) {
        for ch in chunk.chars() {
            match ch {
                '\n' => {
                    if self.pending.is_empty() && self.transient {
                        // Перевод строки сразу после прогресса лишь закрывает
                        // его. Пустую строку поверх последнего значения
                        // печатать нельзя — оно исчезнет с экрана.
                        self.transient = false;
                        continue;
                    }
                    emit(std::mem::take(&mut self.pending), self.transient);
                    self.transient = false;
                }
                '\r' => {
                    if self.pending.is_empty() {
                        continue;
                    }
                    emit(std::mem::take(&mut self.pending), self.transient);
                    self.transient = true;
                }
                c => self.pending.push(c),
            }
        }
    }

    /// Остаток без завершающего перевода строки — например, последняя
    /// строка упавшего процесса.
    pub fn flush(&mut self, mut emit: impl FnMut(String, bool)) {
        if !self.pending.is_empty() {
            emit(std::mem::take(&mut self.pending), self.transient);
            self.transient = false;
        }
    }
}

/// Читает поток кусками и отдаёт готовые строки.
///
/// Именно кусками, а не `BufRead::lines()`: тот делит только по `\n`
/// и проглотил бы возврат каретки вместе с прогрессом.
pub fn pump<R: Read>(mut stream: R, mut on_line: impl FnMut(String, bool)) {
    let mut splitter = LineSplitter::default();
    let mut buf = [0u8; 8192];
    loop {
        match stream.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buf[..n]);
                splitter.feed(&chunk, &mut on_line);
            }
        }
    }
    splitter.flush(&mut on_line);
}

pub fn push_line(running: &Arc<Mutex<Running>>, stream: &str, text: String, replaces: bool) -> LogLine {
    let line = LogLine { stream: stream.to_string(), text, replaces_last: replaces };
    running.lock().unwrap().log.push(line.clone());
    line
}

/// Опрашивает `/system_stats`, пока сервер не ответит.
///
/// Голый TcpStream вместо HTTP-клиента: один запрос раз в полсекунды
/// не стоит минут компиляции лишней зависимости.
pub fn probe(port: u16) -> bool {
    let addr = format!("127.0.0.1:{port}");
    let Ok(parsed) = addr.parse() else { return false };
    let Ok(mut sock) = TcpStream::connect_timeout(&parsed, Duration::from_millis(500)) else {
        return false;
    };
    let _ = sock.set_read_timeout(Some(Duration::from_millis(1500)));

    let req = format!("GET /system_stats HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
    if sock.write_all(req.as_bytes()).is_err() {
        return false;
    }
    let mut head = [0u8; 32];
    match sock.read(&mut head) {
        Ok(n) => String::from_utf8_lossy(&head[..n]).contains("200"),
        Err(_) => false,
    }
}

/// Ждёт готовности, пока не выйдет время или не попросят остановиться.
pub fn wait_ready(
    port: u16,
    timeout: Duration,
    keep_going: impl Fn() -> bool,
) -> Result<u32, AppError> {
    let started = Instant::now();
    while started.elapsed() < timeout {
        if !keep_going() {
            return Err(AppError::new("run.cancelled"));
        }
        if probe(port) {
            return Ok(started.elapsed().as_secs() as u32);
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    Err(AppError::with(
        "run.readyTimeout",
        "secs",
        timeout.as_secs(),
    ))
}

/// После неожиданного выхода процесса проверяет, не поднял ли кто-то
/// сервер заново.
///
/// Так ведёт себя ComfyUI-Manager после установки нод. Отличать этот
/// случай от падения обязательно: пользователю надо сказать не «упало»,
/// а «сервер перезапустился вне нашего контроля».
pub fn detached_after_exit(port: u16) -> bool {
    let deadline = Instant::now() + RESPAWN_GRACE;
    while Instant::now() < deadline {
        if probe(port) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    false
}

pub fn now_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0)
}
