//! The tray, and closing the window while servers are running.
//!
//! The close button means something different in this app than in an ordinary
//! one: behind it there may be a build with a model loaded into VRAM and a
//! queue of generations. Closing the window silently means killing that too,
//! because the child processes live in a Job Object and go down with us.
//!
//! So the window closes either when there is nothing running or after an
//! explicit choice by the user. The choice is shown as a **screen**, not a
//! dialog: with a build running, the tab's native window sits on top of our
//! HTML and nothing can cover it — first we hide the tabs, then ask the
//! frontend to navigate to the exit screen.

use std::sync::Mutex;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WindowEvent, Wry};
use tauri_specta::Event;

use crate::process::{RunState, Runtime};

/// Tray menu items, kept around so their text can be rewritten when the
/// language changes.
#[derive(Default)]
pub struct TrayItems(Mutex<Option<Items>>);

struct Items {
    show: MenuItem<Wry>,
    stop: MenuItem<Wry>,
    quit: MenuItem<Wry>,
}

/// Tray menu labels. They come from the frontend: the translation lives in
/// the locales, not in Rust, and has to change along with the language.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TrayLabels {
    pub show: String,
    pub stop_all: String,
    pub quit: String,
}

/// Which builds closing the app would take down with it.
///
/// `Detached` is not included: we do not control that server, and our exit
/// will not touch it.
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

/// The "user is closing the window while servers are running" event.
///
/// The route is owned by the frontend: driving navigation from Rust would
/// create a second source of truth about which screen is where.
#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type, Event)]
pub struct QuitRequested;

/// Builds the tray.
///
/// The labels are set in English — the same ones as in `en.json`, the source
/// of truth. The frontend sends the translation right after startup, so the
/// English ones are only visible to someone who opens the menu within the
/// first few milliseconds.
pub fn install(app: &tauri::AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let stop = MenuItem::with_id(app, "stop", "Stop all", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &stop, &quit])?;

    TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().cloned().expect("window icon"))
        .tooltip("ComfyUI Portable Organizer")
        .menu(&menu)
        // Menu on right click only: the left one shows the window, which is
        // the expected tray behaviour on Windows.
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

/// Rewrites the menu labels for the current interface language.
pub fn set_labels(app: &tauri::AppHandle, labels: &TrayLabels) {
    let state = app.state::<TrayItems>();
    let guard = state.0.lock().unwrap();
    let Some(items) = guard.as_ref() else { return };
    let _ = items.show.set_text(&labels.show);
    let _ = items.stop.set_text(&labels.stop_all);
    let _ = items.quit.set_text(&labels.quit);
}

/// Shows the window and brings it to the front.
///
/// Three actions rather than one: a window collapsed into the tray is hidden,
/// one collapsed into the taskbar is minimized, and without both calls "show"
/// only works in one of the two cases.
pub fn reveal(app: &tauri::AppHandle) {
    if let Some(window) = app.get_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// Stops everything that is running. Errors are swallowed: this is the last
/// line before exit, and there is no one left to show them to.
pub fn stop_all(app: &tauri::AppHandle) {
    let runtime = app.state::<Runtime>();
    for id in busy(&runtime) {
        if let Some(cell) = runtime.get(&id) {
            let _ = crate::run::stop(&cell);
        }
    }
}

/// Attached to the window in `setup`.
pub fn on_window_event(window: &tauri::Window, event: &WindowEvent) {
    let WindowEvent::CloseRequested { api, .. } = event else {
        return;
    };

    let app = window.app_handle();
    if busy(&app.state::<Runtime>()).is_empty() {
        return;
    }

    api.prevent_close();
    // Remove the tabs first: while the native window is on screen, showing
    // the user anything at all is physically impossible.
    crate::webview::hide_all(app);
    reveal(app);
    let _ = QuitRequested.emit(app);
}
