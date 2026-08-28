//! Shell settings and everything the frontend needs to know at startup.
//!
//! Storage is `tauri-plugin-store`, file `settings.json` in `app_data_dir()`.
//! Phase 1 puts the instance registry on the same mechanism, so inventing our
//! own format for the sake of two fields makes no sense.
//!
//! The dev build and the installed one share the data folder: they have the
//! same bundle identifier. Splitting them into separate folders is a Phase 1
//! task — that is where `tauri.dev.conf.json` with its own identifier appears.

use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_store::StoreExt;

use crate::error::AppError;
use crate::shared_models::SharedSettings;

/// File name inside `app_data_dir()`.
const STORE_FILE: &str = "settings.json";
/// Top-level key. Other settings sections will sit next to it later.
const KEY_UI: &str = "ui";
/// Shared model storage. A key of its own rather than a field inside `ui`:
/// this is a data setting, not an appearance one, and it is required to
/// survive a reset of the appearance.
const KEY_SHARED: &str = "sharedModels";
/// The workflow library. Separate from shared models: it works without them,
/// and tying them to one key would tie the fates of the settings together too.
const KEY_LIBRARY: &str = "workflowLibrary";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum ThemeChoice {
    Light,
    Dark,
    /// Follow the system.
    System,
}

/// `default` on the container is mandatory, not decoration: the settings file
/// was written by a previous version of the app and has none of the new
/// fields. Without it, parsing the whole struct fails on the first unfamiliar
/// field, and the calling code reads that failure as "there are no settings" —
/// silently resetting theme and language along with the field that was merely
/// added.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct UiSettings {
    pub theme: ThemeChoice,
    /// `None` until the user picks a language explicitly: detection by system
    /// applies while it is unset. Telling "never picked" apart from "picked
    /// English" is mandatory, otherwise a change of the Windows language stops
    /// being taken into account.
    pub locale: Option<String>,
    pub rail_collapsed: bool,
    /// Whether to check for updates at startup. The app's only outgoing
    /// request, hence switchable — `NFR-355`.
    pub check_updates: bool,
}

impl Default for UiSettings {
    fn default() -> Self {
        // Dark is the default appearance for the first launch.
        Self {
            theme: ThemeChoice::Dark,
            locale: None,
            rail_collapsed: false,
            check_updates: true,
        }
    }
}

/// Everything the frontend asks for once at startup.
///
/// One call rather than three: startup must not wait for three IPC round
/// trips, or the interface has time to blink in the wrong theme.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Bootstrap {
    pub settings: UiSettings,
    pub shared_models: SharedSettings,
    /// The system language, `ru-RU` for example. The first step of language
    /// detection; after it the frontend looks at `navigator.language` and
    /// falls back to English.
    pub system_locale: Option<String>,
    /// Shown in the "About" section together with the "open folder" button.
    pub app_data_dir: String,
    /// Derived data: the node snapshot cache and WebView2 data. A line of its
    /// own because there are two folders, and both of them get deleted — the
    /// user should not have to guess that the second one exists.
    pub app_local_data_dir: String,
    pub version: String,
}

pub fn load(app: &tauri::AppHandle) -> Result<Bootstrap, AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("settings.loadFailed", e))?;

    // A corrupt or outdated file must not get in the way of startup: settings
    // are not user data, recreating them from defaults costs nothing.
    let settings = store
        .get(KEY_UI)
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let app_data_dir = app
        .path()
        .app_data_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    let app_local_data_dir = app
        .path()
        .app_local_data_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    let shared_models = store
        .get(KEY_SHARED)
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    Ok(Bootstrap {
        settings,
        shared_models,
        system_locale: tauri_plugin_os::locale(),
        app_data_dir,
        app_local_data_dir,
        version: app.package_info().version.to_string(),
    })
}

/// Shared models are read separately: they are asked for after startup as
/// well — on every visit to the settings screen and before every instance
/// launch.
pub fn load_shared(app: &tauri::AppHandle) -> Result<SharedSettings, AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("settings.loadFailed", e))?;

    Ok(store
        .get(KEY_SHARED)
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default())
}

pub fn save_shared(app: &tauri::AppHandle, shared: &SharedSettings) -> Result<(), AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("settings.saveFailed", e))?;

    let value = serde_json::to_value(shared)
        .map_err(|e| AppError::because("settings.saveFailed", e))?;

    store.set(KEY_SHARED, value);
    store
        .save()
        .map_err(|e| AppError::because("settings.saveFailed", e))
}

/// Workflow library settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct LibrarySettings {
    /// Empty means the library is not set. We do not substitute a default
    /// into the model: "never picked" and "picked exactly this" have to stay
    /// distinguishable, otherwise moving the shared models root would silently
    /// drag the library along with it.
    pub path: String,
}

pub fn load_library(app: &tauri::AppHandle) -> Result<LibrarySettings, AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("settings.loadFailed", e))?;

    Ok(store
        .get(KEY_LIBRARY)
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default())
}

pub fn save_library(app: &tauri::AppHandle, library: &LibrarySettings) -> Result<(), AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("settings.saveFailed", e))?;

    let value = serde_json::to_value(library)
        .map_err(|e| AppError::because("settings.saveFailed", e))?;

    store.set(KEY_LIBRARY, value);
    store
        .save()
        .map_err(|e| AppError::because("settings.saveFailed", e))
}

pub fn save(app: &tauri::AppHandle, settings: &UiSettings) -> Result<(), AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("settings.saveFailed", e))?;

    let value = serde_json::to_value(settings)
        .map_err(|e| AppError::because("settings.saveFailed", e))?;

    store.set(KEY_UI, value);
    store
        .save()
        .map_err(|e| AppError::because("settings.saveFailed", e))
}
