//! Трей и закрытие окна при работающих серверах.
//!
//! Крестик у этого приложения означает не то же, что у обычного: за ним
//! может стоять сборка с загруженной в видеопамять моделью и очередью
//! генераций. Закрыть окно молча — значит убить и её, потому что дочерние
//! процессы живут в Job Object и уходят вместе с нами.
//!
//! Поэтому окно закрывается либо когда работать нечему, либо после
//! явного выбора пользователя. Выбор показывается **экраном**, а не
//! диалогом: у работающей сборки поверх нашего HTML лежит нативное окно
//! вкладки, и перекрыть его нечем — сначала прячем вкладки, потом просим
//! фронт увести на экран выхода.

use std::sync::Mutex;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WindowEvent, Wry};
use tauri_specta::Event;

use crate::process::{RunState, Runtime};

/// Пункты меню трея, чтобы переписывать их текст при смене языка.
#[derive(Default)]
pub struct TrayItems(Mutex<Option<Items>>);

struct Items {
    show: MenuItem<Wry>,
    stop: MenuItem<Wry>,
    quit: MenuItem<Wry>,
}

/// Подписи меню трея. Приходят с фронта: перевод живёт в локалях, а не
/// в Rust, и меняться обязан вместе с языком.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TrayLabels {
    pub show: String,
    pub stop_all: String,
    pub quit: String,
}

/// Есть ли сборки, которые закрытие приложения унесёт с собой.
///
/// `Detached` сюда не входит: тем сервером мы не управляем, и наш выход
/// его не заденет.
pub fn busy(runtime: &Runtime) -> Vec<String> {
    runtime
        .statuses()
        .into_iter()
        .filter(|s| {
            matches!(
                s.state,
                RunState::Starting | RunState::Running | RunState::Stopping
            )
        })
        .map(|s| s.instance_id)
        .collect()
}

/// Событие «пользователь закрывает окно, а серверы работают».
///
/// Роутом владеет фронт: вести навигацию из Rust значило бы завести
/// второй источник правды о том, где какой экран.
#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type, Event)]
pub struct QuitRequested;

/// Строит трей.
///
/// Подписи ставятся английские — те же, что в `en.json`, источнике правды.
/// Фронт присылает перевод сразу после запуска, и увидеть английский
/// в меню можно только успев открыть его в первые миллисекунды.
pub fn install(app: &tauri::AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let stop = MenuItem::with_id(app, "stop", "Stop all", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &stop, &quit])?;

    TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().cloned().expect("иконка окна"))
        .tooltip("ComfyUI Portable Organizer")
        .menu(&menu)
        // Меню только по правой кнопке: левая показывает окно, это
        // ожидаемое поведение трея на Windows.
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => reveal(app),
            "stop" => stop_all(app),
            "quit" => {
                stop_all(app);
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                reveal(tray.app_handle());
            }
        })
        .build(app)?;

    *app.state::<TrayItems>().0.lock().unwrap() = Some(Items { show, stop, quit });
    Ok(())
}

/// Переписывает подписи меню под текущий язык интерфейса.
pub fn set_labels(app: &tauri::AppHandle, labels: &TrayLabels) {
    let state = app.state::<TrayItems>();
    let guard = state.0.lock().unwrap();
    let Some(items) = guard.as_ref() else { return };
    let _ = items.show.set_text(&labels.show);
    let _ = items.stop.set_text(&labels.stop_all);
    let _ = items.quit.set_text(&labels.quit);
}

/// Показывает окно и поднимает его наверх.
///
/// Три действия, а не одно: свёрнутое в трей окно спрятано, свёрнутое
/// в панель задач — минимизировано, и без обоих вызовов «показать»
/// сработает только в одном из случаев.
pub fn reveal(app: &tauri::AppHandle) {
    if let Some(window) = app.get_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// Останавливает всё, что работает. Ошибки глотаем: это последний рубеж
/// перед выходом, и показывать их уже некому.
pub fn stop_all(app: &tauri::AppHandle) {
    let runtime = app.state::<Runtime>();
    for id in busy(&runtime) {
        if let Some(cell) = runtime.get(&id) {
            let _ = crate::run::stop(&cell);
        }
    }
}

/// Вешается на окно в `setup`.
pub fn on_window_event(window: &tauri::Window, event: &WindowEvent) {
    let WindowEvent::CloseRequested { api, .. } = event else {
        return;
    };

    let app = window.app_handle();
    if busy(&app.state::<Runtime>()).is_empty() {
        return;
    }

    api.prevent_close();
    // Сначала убрать вкладки: пока нативное окно на экране, показать
    // пользователю хоть что-нибудь физически невозможно.
    crate::webview::hide_all(app);
    reveal(app);
    let _ = QuitRequested.emit(app);
}
