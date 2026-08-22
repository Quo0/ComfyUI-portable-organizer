//! Настройки оболочки и всё, что фронту нужно знать при старте.
//!
//! Хранилище — `tauri-plugin-store`, файл `settings.json` в `app_data_dir()`.
//! Тот же механизм в Фазе 1 возьмёт на себя реестр инстансов, поэтому
//! заводить ради двух полей собственный формат смысла нет.
//!
//! Дев-сборка и установленная делят папку данных: у них один
//! bundle identifier. Разведение по разным папкам — задача Фазы 1,
//! там появится `tauri.dev.conf.json` со своим идентификатором.

use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_store::StoreExt;

use crate::error::AppError;
use crate::shared_models::SharedSettings;

/// Имя файла в `app_data_dir()`.
const STORE_FILE: &str = "settings.json";
/// Ключ верхнего уровня. Дальше рядом лягут другие разделы настроек.
const KEY_UI: &str = "ui";
/// Общее хранилище моделей. Отдельным ключом, а не полем внутри `ui`:
/// это настройка данных, а не оформления, и переживать сброс оформления
/// она обязана.
const KEY_SHARED: &str = "sharedModels";
/// Библиотека воркфлоу. Отдельно от общих моделей: она работает и без них,
/// и связывать их одним ключом значило бы связать и судьбы настроек.
const KEY_LIBRARY: &str = "workflowLibrary";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum ThemeChoice {
    Light,
    Dark,
    /// Следовать системе.
    System,
}

/// `default` на контейнере обязателен, а не украшение: файл настроек
/// пишется прошлой версией приложения и новых полей не содержит. Без него
/// разбор всей структуры проваливается на первом же незнакомом поле,
/// а вызывающий код трактует провал как «настроек нет» — и молча сбрасывает
/// тему с языком заодно с тем полем, которое всего лишь добавили.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct UiSettings {
    pub theme: ThemeChoice,
    /// `None`, пока пользователь не выбрал язык явно: тогда работает
    /// определение по системе. Отличать «не выбирал» от «выбрал английский»
    /// обязательно, иначе смена языка Windows перестанет учитываться.
    pub locale: Option<String>,
    pub rail_collapsed: bool,
    /// Проверять ли обновления при запуске. Единственное исходящее
    /// обращение приложения, поэтому выключаемое — `NFR-355`.
    pub check_updates: bool,
}

impl Default for UiSettings {
    fn default() -> Self {
        // Тёмная — оформление по умолчанию для первого запуска.
        Self {
            theme: ThemeChoice::Dark,
            locale: None,
            rail_collapsed: false,
            check_updates: true,
        }
    }
}

/// Всё, что фронт спрашивает один раз при запуске.
///
/// Одним вызовом, а не тремя: старт не должен ждать три круга IPC,
/// иначе интерфейс успевает моргнуть чужой темой.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Bootstrap {
    pub settings: UiSettings,
    pub shared_models: SharedSettings,
    /// Язык системы, например `ru-RU`. Первый шаг определения языка;
    /// дальше фронт смотрит на `navigator.language` и падает в английский.
    pub system_locale: Option<String>,
    /// Показывается в разделе «О приложении» вместе с кнопкой «открыть папку».
    pub app_data_dir: String,
    /// Производное: кэш снимков нод и данные WebView2. Отдельной строкой,
    /// потому что папки две, и удаляются они тоже обе — пользователь
    /// не должен догадываться о существовании второй.
    pub app_local_data_dir: String,
    pub version: String,
}

pub fn load(app: &tauri::AppHandle) -> Result<Bootstrap, AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("settings.loadFailed", e))?;

    // Битый или устаревший файл не должен мешать запуску: настройки —
    // не данные пользователя, их не жалко пересоздать значениями по умолчанию.
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

/// Общие модели читаются отдельно: их спрашивают и после старта —
/// при каждом заходе на экран настроек и перед каждым запуском инстанса.
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

/// Настройки библиотеки воркфлоу.
#[derive(Debug, Clone, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct LibrarySettings {
    /// Пусто — библиотека не задана. Дефолт не подставляем в модель:
    /// «не выбирал» и «выбрал вот это» обязаны различаться, иначе смена
    /// корня общих моделей молча уводила бы библиотеку следом.
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
