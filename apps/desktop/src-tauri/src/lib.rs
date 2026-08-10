//! Спайк Фазы 0.
//!
//! Задача — доказать четыре вещи, на которых держится весь замысел:
//!   1. ComfyUI запускается напрямую через python.exe, без .bat;
//!   2. браузер при этом не открывается и окно консоли не всплывает;
//!   3. stderr стримится в интерфейс живьём, а не после завершения;
//!   4. дочерний вебвью грузит ComfyUI без 403 от origin-middleware.
//!
//! Всё здесь временное: путь захардкожен, состояние примитивное,
//! обработка ошибок минимальная. Настоящая архитектура — с Фазы 1.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::Serialize;
use tauri::{Emitter, LogicalPosition, LogicalSize, Manager, WebviewUrl};

/// Реальная установка для спайка. Захардкожена намеренно: реестр появится в Фазе 1.
const INSTANCE_DIR: &str =
    r"d:\program_files\comfyui\ComfyUI_windows_portable_nvidia\ComfyUI_windows_portable";
const PORT: u16 = 8188;

/// Скрывает окно консоли у дочернего процесса. Без него при каждом запуске
/// поверх интерфейса всплывал бы чёрный терминал.
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[derive(Default)]
struct SpikeState {
    child: Mutex<Option<Child>>,
}

#[derive(Clone, Serialize)]
struct LogLine {
    stream: &'static str,
    text: String,
}

/// Запускает ComfyUI и стримит его вывод событиями `comfy-log`.
///
/// Ключевые флаги: `--disable-auto-launch` не даёт открыться браузеру
/// (в cli_args.py он применяется после `--windows-standalone-build`
/// и всегда побеждает), `--port` фиксирует порт.
#[tauri::command]
fn start_comfy(app: tauri::AppHandle, state: tauri::State<'_, SpikeState>) -> Result<u16, String> {
    if state.child.lock().unwrap().is_some() {
        return Err("Уже запущен".into());
    }

    let python = format!(r"{INSTANCE_DIR}\python_embeded\python.exe");
    let main_py = format!(r"{INSTANCE_DIR}\ComfyUI\main.py");

    let mut cmd = Command::new(&python);
    cmd.args([
        "-s",
        &main_py,
        "--windows-standalone-build",
        "--port",
        &PORT.to_string(),
        "--disable-auto-launch",
    ])
    .current_dir(INSTANCE_DIR)
    // Без этого stdout буферизуется блоками при перенаправлении в пайп,
    // и первые минуты старта выглядят как зависание.
    .env("PYTHONUNBUFFERED", "1")
    .env("PYTHONIOENCODING", "utf-8")
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = cmd.spawn().map_err(|e| format!("Не удалось запустить: {e}"))?;

    // ComfyUI пишет основную часть старта в stderr, а не в stdout,
    // поэтому читаем оба потока.
    if let Some(out) = child.stdout.take() {
        pump(app.clone(), out, "stdout");
    }
    if let Some(err) = child.stderr.take() {
        pump(app.clone(), err, "stderr");
    }

    *state.child.lock().unwrap() = Some(child);
    Ok(PORT)
}

/// Читает поток построчно в отдельном треде и шлёт каждую строку во фронт.
///
/// Первая строка дополнительно печатается в терминал: по ней видно,
/// действительно ли стриминг живой, или вывод пришёл пачкой в конце.
fn pump<R: Read + Send + 'static>(app: tauri::AppHandle, stream: R, name: &'static str) {
    std::thread::spawn(move || {
        let started = Instant::now();
        let mut first = true;
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            let Ok(text) = line else { break };
            if first {
                first = false;
                report(&format!(
                    "первая строка {name} через {:.1} с: {}",
                    started.elapsed().as_secs_f32(),
                    text.chars().take(60).collect::<String>()
                ));
            }
            let _ = app.emit("comfy-log", LogLine { stream: name, text });
        }
    });
}

/// Печатает факт спайка в терминал `tauri dev` с приметным префиксом,
/// чтобы результаты было видно среди логов сборки.
fn report(msg: &str) {
    println!("[СПАЙК] {msg}");
}

/// Опрашивает `/system_stats`, пока сервер не ответит.
///
/// Реализовано на голом TcpStream осознанно: тянуть HTTP-клиент ради
/// одного запроса в спайк — лишние минуты компиляции.
#[tauri::command]
fn wait_ready(port: u16, timeout_secs: u64) -> Result<u64, String> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    let started = Instant::now();

    while Instant::now() < deadline {
        if probe(port) {
            return Ok(started.elapsed().as_secs());
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    Err(format!("Сервер не ответил за {timeout_secs} с"))
}

fn probe(port: u16) -> bool {
    let addr = format!("127.0.0.1:{port}");
    let Ok(mut sock) = TcpStream::connect_timeout(
        &addr.parse().expect("валидный адрес"),
        Duration::from_millis(500),
    ) else {
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

/// Главная проверка фазы: ComfyUI внутри нашего окна.
///
/// `<iframe>` здесь получил бы 403 — origin_only_middleware режет всё
/// с `Sec-Fetch-Site: cross-site`. Дочерний вебвью грузит страницу как
/// навигацию верхнего уровня, и middleware пропускает без единого
/// послабления в настройках сервера.
#[tauri::command]
fn embed_comfy(app: tauri::AppHandle, port: u16, x: f64, y: f64, w: f64, h: f64) -> Result<(), String> {
    let window = app.get_window("main").ok_or("Нет окна main")?;

    if let Some(existing) = app.get_webview("comfy") {
        existing
            .set_position(LogicalPosition::new(x, y))
            .map_err(|e| e.to_string())?;
        existing
            .set_size(LogicalSize::new(w, h))
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let url = format!("http://127.0.0.1:{port}")
        .parse()
        .map_err(|_| "Плохой URL")?;

    let probe_app = app.clone();
    let title_app = app.clone();

    let builder = tauri::webview::WebviewBuilder::new("comfy", WebviewUrl::External(url))
        // Иначе Tauri перехватит системный дроп, и перетаскивание картинок
        // и воркфлоу на холст ComfyUI перестанет работать.
        .disable_drag_drop_handler()
        .on_page_load(move |view, payload| {
            report(&format!("вкладка загрузила {}", payload.url()));
            let _ = probe_app.emit(
                "comfy-log",
                LogLine {
                    stream: "webview",
                    text: format!("страница загружена: {}", payload.url()),
                },
            );
            // Заголовок — единственный канал обратно из чужого origin:
            // наш IPC там не доступен. Кладём в него начало текста страницы,
            // чтобы увидеть, отдал ли сервер интерфейс или 403.
            let _ = view.eval(
                "document.title = 'CPO|' + document.title + '|' \
                 + (document.body ? document.body.innerText.slice(0, 90) : '(нет body)')",
            );
        })
        .on_document_title_changed(move |_view, title| {
            if let Some(rest) = title.strip_prefix("CPO|") {
                // Главный результат фазы: что реально отдал сервер вкладке.
                // Если бы origin-middleware отбила запрос, здесь было бы 403.
                report(&format!("вкладка видит: {rest}"));
                let _ = title_app.emit(
                    "comfy-log",
                    LogLine {
                        stream: "webview",
                        text: format!("вебвью видит: {rest}"),
                    },
                );
            }
        });

    window
        .add_child(
            builder,
            LogicalPosition::new(x, y),
            LogicalSize::new(w, h),
        )
        .map_err(|e| format!("Не удалось создать вебвью: {e}"))?;

    Ok(())
}

#[tauri::command]
fn stop_comfy(app: tauri::AppHandle, state: tauri::State<'_, SpikeState>) -> Result<(), String> {
    if let Some(view) = app.get_webview("comfy") {
        let _ = view.close();
    }
    if let Some(mut child) = state.child.lock().unwrap().take() {
        // На Windows послать SIGINT чужому процессу нельзя, поэтому
        // в Фазе 2 здесь будет taskkill /T /F. Для спайка достаточно kill.
        let _ = child.kill();
        let _ = child.wait();
    }
    Ok(())
}

/// Прогоняет спайк целиком без участия человека.
///
/// Включается переменной `CPO_SPIKE=1`. Нужен потому, что проверить
/// результат кликом по кнопке можно только руками, а решение фазы
/// хочется получать воспроизводимо и в логе.
fn autorun(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        report("автопрогон включён (CPO_SPIKE=1)");
        let state = app.state::<SpikeState>();

        let port = match start_comfy(app.clone(), state) {
            Ok(p) => {
                report(&format!("процесс запущен, порт {p}"));
                p
            }
            Err(e) => {
                report(&format!("ПРОВАЛ: не удалось запустить: {e}"));
                return;
            }
        };

        match wait_ready(port, 300) {
            Ok(secs) => report(&format!("сервер готов за {secs} с")),
            Err(e) => {
                report(&format!("ПРОВАЛ: {e}"));
                return;
            }
        }

        // Прямоугольник произвольный: во фронте его считает ResizeObserver,
        // здесь важно лишь то, что вебвью создаётся и грузит страницу.
        if let Err(e) = embed_comfy(app.clone(), port, 0.0, 46.0, 1100.0, 460.0) {
            report(&format!("ПРОВАЛ: не удалось встроить вкладку: {e}"));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SpikeState::default())
        .invoke_handler(tauri::generate_handler![
            start_comfy,
            wait_ready,
            embed_comfy,
            stop_comfy
        ])
        .setup(|app| {
            if std::env::var("CPO_SPIKE").as_deref() == Ok("1") {
                autorun(app.handle().clone());
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("не удалось запустить приложение");
}
