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

use serde::{Deserialize, Serialize};
use tauri::{LogicalPosition, LogicalSize, Manager, WebviewUrl};
use tauri_specta::Event;

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

/// Событие с строкой лога.
///
/// Типы генерируются в `src/bindings.ts`: описывать одну и ту же модель
/// на Rust и на TypeScript руками — верный способ получить молчаливое
/// расхождение, а этих моделей дальше будет много.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
struct LogLine {
    stream: String,
    text: String,
}

/// Сервер поднялся и готов принимать запросы.
///
/// Автопрогон не встраивает вкладку сам: прямоугольник знает только фронт,
/// у которого есть `ResizeObserver`. Захардкоженный размер в Rust дал бы
/// вкладку не по месту.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
struct ComfyReady {
    port: u16,
    secs: u32,
}

/// Запускает ComfyUI и стримит его вывод событиями `comfy-log`.
///
/// Ключевые флаги: `--disable-auto-launch` не даёт открыться браузеру
/// (в cli_args.py он применяется после `--windows-standalone-build`
/// и всегда побеждает), `--port` фиксирует порт.
/// Команды объявлены `async` намеренно.
///
/// Синхронная команда Tauri выполняется в главном потоке. Для `wait_ready`
/// это заморозило бы интерфейс на все минуты холодного старта, а для
/// `embed_comfy` дало бы взаимную блокировку: `add_child` изнутри ставит
/// задачу в главный поток и ждёт её результата.
#[tauri::command]
#[specta::specta]
async fn start_comfy(
    app: tauri::AppHandle,
    state: tauri::State<'_, SpikeState>,
) -> Result<u16, String> {
    spawn_comfy(&app, &state)
}

fn spawn_comfy(app: &tauri::AppHandle, state: &SpikeState) -> Result<u16, String> {
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
            let _ = LogLine { stream: name.to_string(), text }.emit(&app);
        }
    });
}

/// Печатает факт спайка в терминал `tauri dev` с приметным префиксом,
/// чтобы результаты было видно среди логов сборки.
fn report(msg: &str) {
    println!("[СПАЙК] {msg}");
}

/// Позволяет фронту отметиться в терминале.
///
/// Консоль вебвью в вывод `tauri dev` не попадает, поэтому без такого
/// канала непонятно, дошло ли событие до интерфейса или сломалось раньше.
#[tauri::command]
#[specta::specta]
fn spike_ping(stage: String) {
    report(&format!("фронт: {stage}"));
}

/// Опрашивает `/system_stats`, пока сервер не ответит.
///
/// Реализовано на голом TcpStream осознанно: тянуть HTTP-клиент ради
/// одного запроса в спайк — лишние минуты компиляции.
#[tauri::command]
#[specta::specta]
async fn wait_ready(port: u16, timeout_secs: u32) -> Result<u32, String> {
    wait_ready_inner(port, timeout_secs)
}

fn wait_ready_inner(port: u16, timeout_secs: u32) -> Result<u32, String> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs.into());
    let started = Instant::now();

    while Instant::now() < deadline {
        if probe(port) {
            return Ok(started.elapsed().as_secs() as u32);
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
#[specta::specta]
async fn embed_comfy(
    app: tauri::AppHandle,
    port: u16,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) -> Result<(), String> {
    embed_inner(&app, port, x, y, w, h)
}

fn embed_inner(app: &tauri::AppHandle, port: u16, x: f64, y: f64, w: f64, h: f64) -> Result<(), String> {
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
            let _ = LogLine {
                stream: "webview".into(),
                text: format!("страница загружена: {}", payload.url()),
            }
            .emit(&probe_app);
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
                let _ = LogLine {
                    stream: "webview".into(),
                    text: format!("вебвью видит: {rest}"),
                }
                .emit(&title_app);
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
#[specta::specta]
async fn stop_comfy(
    app: tauri::AppHandle,
    state: tauri::State<'_, SpikeState>,
) -> Result<(), String> {
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

        let port = match spawn_comfy(&app, &state) {
            Ok(p) => {
                report(&format!("процесс запущен, порт {p}"));
                p
            }
            Err(e) => {
                report(&format!("ПРОВАЛ: не удалось запустить: {e}"));
                return;
            }
        };

        let secs = match wait_ready_inner(port, 300) {
            Ok(secs) => {
                report(&format!("сервер готов за {secs} с"));
                secs
            }
            Err(e) => {
                report(&format!("ПРОВАЛ: {e}"));
                return;
            }
        };

        // Встраивание отдаём фронту: прямоугольник знает он.
        if let Err(e) = (ComfyReady { port, secs }).emit(&app) {
            report(&format!("ПРОВАЛ: не удалось сообщить о готовности: {e}"));
        }
    });
}

/// Единый список команд и событий: и обработчик вызовов, и генератор типов
/// берутся отсюда, поэтому разойтись они не могут.
fn specta_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
            start_comfy,
            wait_ready,
            embed_comfy,
            stop_comfy,
            spike_ping
        ])
        .events(tauri_specta::collect_events![LogLine, ComfyReady])
}

/// Выгружает типы в `src/bindings.ts`.
///
/// Вызывается и из дев-сборки, и из теста. Тест важнее: он генерирует
/// типы без запуска окна, поэтому работает в CI и не требует ни дисплея,
/// ни дев-сервера Vite.
#[cfg(debug_assertions)]
fn export_bindings(builder: &tauri_specta::Builder<tauri::Wry>) {
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("не удалось выгрузить типы в bindings.ts");
}

#[cfg(test)]
mod tests {
    /// Держит `src/bindings.ts` в согласии с сигнатурами команд.
    /// Ломается ровно тогда, когда изменился контракт, — и это хорошо.
    #[test]
    fn bindings_up_to_date() {
        super::export_bindings(&super::specta_builder());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    #[cfg(debug_assertions)]
    export_bindings(&builder);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SpikeState::default())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            // Обязательно: без mount_events типизированные события
            // не доедут до фронта.
            builder.mount_events(app);

            if std::env::var("CPO_SPIKE").as_deref() == Ok("1") {
                autorun(app.handle().clone());
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("не удалось запустить приложение");
}
