//! The embedded tab: a child webview with the ComfyUI interface.
//!
//! The child webview is a native window **on top of** our HTML. Everything
//! else follows from that: it has no auto-layout, the frontend computes the
//! rectangle, covering it with our markup is physically impossible, and two
//! visible tabs would lie one on top of the other.
//!
//! An `<iframe>` would get a 403 here: ComfyUI's `origin_only_middleware` cuts
//! off everything with `Sec-Fetch-Site: cross-site`. A child webview loads the
//! page as a top-level navigation — the middleware lets it through without a
//! single concession in the server settings.

use tauri::{LogicalPosition, LogicalSize, Manager, Url, WebviewUrl};

use crate::error::AppError;

/// The label prefix. It is how all our tabs are found among the window's
/// webviews.
const PREFIX: &str = "comfy-";

/// The webview label of an instance.
///
/// An instance identifier is `i<milliseconds>[-N]` (`instances::new_id`) —
/// Latin letters, digits and a hyphen. There is nothing to escape.
pub fn label(id: &str) -> String {
    format!("{PREFIX}{id}")
}

/// The rectangle of the content area in logical pixels.
///
/// Comes from `getBoundingClientRect()` of the slot on the frontend: CSS
/// pixels and logical ones are the same thing, so display scaling needs no
/// recalculation.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

fn failed(e: impl ToString) -> AppError {
    AppError::because("webview.embedFailed", e)
}

/// Shows an instance's tab, creating it on the first call.
///
/// The other tabs are hidden **within this same call**. Switching between two
/// running builds goes through one and the same Vue component — only the route
/// parameter changes — and leaving the order "show the new one, hide the old
/// one" to the frontend would produce a frame with two webviews on top of each
/// other.
pub fn show(app: &tauri::AppHandle, id: &str, port: u16, rect: Rect) -> Result<(), AppError> {
    let want = label(id);
    hide_others(app, Some(&want));

    if let Some(existing) = app.get_webview(&want) {
        existing
            .set_position(LogicalPosition::new(rect.x, rect.y))
            .map_err(failed)?;
        existing
            .set_size(LogicalSize::new(rect.w, rect.h))
            .map_err(failed)?;
        // Showing and placing in one action: otherwise the tab flashes at the
        // rectangle where it was left last time.
        existing.show().map_err(failed)?;
        let _ = existing.set_focus();
        return Ok(());
    }

    let window = app
        .get_window("main")
        .ok_or_else(|| failed("no main window"))?;

    let url = format!("http://127.0.0.1:{port}")
        .parse()
        .map_err(|_| failed("bad URL"))?;

    let nav_app = app.clone();
    let new_window_app = app.clone();

    let builder = tauri::webview::WebviewBuilder::new(want.clone(), WebviewUrl::External(url))
        // Otherwise Tauri intercepts the system drop, and dragging images and
        // workflows onto the ComfyUI canvas stops working.
        .disable_drag_drop_handler()
        .on_navigation(move |url| {
            if internal(url, port) {
                return true;
            }
            // The user does not expect a window with no address bar and no
            // back button: a documentation link goes to their browser.
            open_external(&nav_app, url.as_str());
            false
        })
        // A separate mechanism: `target="_blank"` and `window.open` arrive
        // here, not in `on_navigation`. We always refuse, our own origin
        // included — we have no separate window for a popup, and placing one
        // over the embedded area is impossible.
        .on_new_window(move |url, _features| {
            open_external(&new_window_app, url.as_str());
            tauri::webview::NewWindowResponse::Deny
        });

    window
        .add_child(
            builder,
            LogicalPosition::new(rect.x, rect.y),
            LogicalSize::new(rect.w, rect.h),
        )
        .map_err(failed)?;

    if let Some(view) = app.get_webview(&want) {
        let _ = view.set_focus();
    }
    Ok(())
}

/// Whether this navigation is our own.
///
/// Besides our own address we let `about:`, `blob:` and `data:` through:
/// downloads and "Save image" in WebView2 rest on them, and a ban would turn
/// exporting from ComfyUI into silence.
///
/// Parsed parts are compared, not the start of the string. A prefix can be
/// fooled in a couple of ways at once: `http://127.0.0.1:8188@example.com/`
/// starts with exactly our address, yet its host belongs to someone else —
/// that part is a username there.
pub fn internal(url: &Url, port: u16) -> bool {
    if matches!(url.scheme(), "about" | "blob" | "data") {
        return true;
    }
    url.scheme() == "http"
        && url.host_str() == Some("127.0.0.1")
        && url.port() == Some(port)
        && url.username().is_empty()
}

fn open_external(app: &tauri::AppHandle, url: &str) {
    use tauri_plugin_opener::OpenerExt;
    if let Err(e) = app.opener().open_url(url, None::<&str>) {
        eprintln!("[CPO] failed to open the link in an external browser: {e}");
    }
}

/// Moves an already created tab. No tab means quietly nothing:
/// `ResizeObserver` fires before it is created as well.
pub fn place(app: &tauri::AppHandle, id: &str, rect: Rect) -> Result<(), AppError> {
    let Some(view) = app.get_webview(&label(id)) else {
        return Ok(());
    };
    view.set_position(LogicalPosition::new(rect.x, rect.y))
        .map_err(failed)?;
    view.set_size(LogicalSize::new(rect.w, rect.h))
        .map_err(failed)?;
    Ok(())
}

/// Hides every tab: leaving for another section or opening the log console.
///
/// The processes keep running meanwhile, and an unsaved graph stays in the
/// live page — only an explicit command stops a build.
pub fn hide_all(app: &tauri::AppHandle) {
    hide_others(app, None);
}

fn hide_others(app: &tauri::AppHandle, keep: Option<&str>) {
    for (name, view) in app.webviews() {
        if name.starts_with(PREFIX) && Some(name.as_str()) != keep {
            let _ = view.hide();
        }
    }
}

/// Closes an instance's tab.
///
/// Called when the process is gone: the port dies with it, and a live tab
/// would show a WebView2 error page instead of the interface. On a restart the
/// port may change as well — the tab must be created anew rather than reused.
pub fn close(app: &tauri::AppHandle, id: &str) {
    if let Some(view) = app.get_webview(&label(id)) {
        let _ = view.close();
    }
}

/// Reloads the tab's page.
///
/// A cheap answer to "the interface did not finish loading" and to "the
/// workflow was added but is not in the list": a running ComfyUI does not
/// re-read the list by itself.
pub fn reload(app: &tauri::AppHandle, id: &str) -> Result<(), AppError> {
    let view = app
        .get_webview(&label(id))
        .ok_or_else(|| AppError::new("webview.noTab"))?;
    view.eval("location.reload()").map_err(failed)
}
