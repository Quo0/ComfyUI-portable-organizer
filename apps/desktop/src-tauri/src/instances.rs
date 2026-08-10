//! Реестр инстансов.
//!
//! Приложение знает об инстансах, но ими не владеет: регистрация запоминает
//! путь и метаданные, папка на диске остаётся нетронутой. Удаление из реестра
//! тоже ничего не стирает — это главное обещание раздела.
//!
//! Хранилище — `instances.json` в `app_data_dir()` через `tauri-plugin-store`,
//! рядом с `settings.json`. Никакого хардкода пути: инсталлятор чистит папку
//! строго по bundle identifier, и литерал разошёлся бы с ним.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri_plugin_store::StoreExt;

use crate::discovery::{FoundProfile, InstanceDiscovery, Probe};
use crate::error::AppError;

const STORE_FILE: &str = "instances.json";
const KEY_LIST: &str = "instances";

/// Порт по умолчанию — тот же, что у самого ComfyUI.
pub const DEFAULT_PORT: u16 = 8188;

/// Акцентный цвет инстанса. Хранится именем токена, а не значением:
/// в тёмной теме у каждого своё значение, и записанный hex оказался бы
/// нечитаемым в одной из тем.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum Accent {
    Ember,
    Amber,
    Moss,
    Teal,
    Azure,
    Indigo,
    Orchid,
    Rose,
}

/// Порядок выдачи цветов новым инстансам: подряд, по кругу.
const ACCENT_CYCLE: [Accent; 8] = [
    Accent::Teal,
    Accent::Indigo,
    Accent::Ember,
    Accent::Moss,
    Accent::Azure,
    Accent::Orchid,
    Accent::Rose,
    Accent::Amber,
];

/// Откуда взялся инстанс. Заполняет мастер установки в Фазе 1.5;
/// у добавленных вручную папок этого нет.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstallSource {
    pub archive_path: String,
    pub archive_label: String,
    /// Миллисекунды эпохи. Дату форматирует фронт: она обязана следовать
    /// выбранному языку, а строка из Rust этого не умеет.
    pub installed_at: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub description: String,
    pub path: String,
    pub accent: Accent,
    pub preferred_port: u16,
    pub comfy_version: Option<String>,
    pub python_version: Option<String>,
    pub profiles: Vec<FoundProfile>,
    pub created_at: f64,
    pub source: Option<InstallSource>,

    /// Размер на диске. `f64`, а не `u64`: specta запрещает экспорт целых,
    /// не помещающихся в число JavaScript без потери точности. Байты
    /// до 9 петабайт в f64 представимы точно, этого хватит с запасом.
    pub size_bytes: Option<f64>,
    pub size_measured_at: Option<f64>,

    /// Папка на месте. Не хранится по-настоящему: пересчитывается при
    /// каждом чтении реестра. Инстанс с исчезнувшей папкой помечается
    /// недоступным, но из списка не пропадает — иначе пользователь решит,
    /// что приложение потеряло его сборку.
    pub available: bool,
}

/// Метаданные, которые правит пользователь. Отдельным типом, чтобы
/// «переименовать» не могло случайно переписать путь или версию.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstanceEdit {
    pub name: String,
    pub description: String,
    pub accent: Accent,
    pub preferred_port: u16,
}

/// Идущие сейчас подсчёты размера. Второй запуск обхода того же дерева
/// ничего не ускорит, зато удвоит нагрузку на диск.
#[derive(Default)]
pub struct SizeJobs(Mutex<HashSet<String>>);

impl SizeJobs {
    fn start(&self, id: &str) -> bool {
        self.0.lock().unwrap().insert(id.to_string())
    }

    fn finish(&self, id: &str) {
        self.0.lock().unwrap().remove(id);
    }
}

fn now_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0)
}

fn read_all(app: &tauri::AppHandle) -> Result<Vec<Instance>, AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("instances.loadFailed", e))?;

    // Битый файл реестра не должен мешать запуску: показать пустой список
    // честнее, чем не открыться вовсе. Файл перезапишется первой же правкой.
    let mut list: Vec<Instance> = store
        .get(KEY_LIST)
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    for instance in &mut list {
        instance.available = folder_ok(Path::new(&instance.path));
    }
    Ok(list)
}

fn write_all(app: &tauri::AppHandle, list: &[Instance]) -> Result<(), AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("instances.saveFailed", e))?;
    let value = serde_json::to_value(list)
        .map_err(|e| AppError::because("instances.saveFailed", e))?;
    store.set(KEY_LIST, value);
    store
        .save()
        .map_err(|e| AppError::because("instances.saveFailed", e))
}

fn folder_ok(path: &Path) -> bool {
    path.join(r"python_embeded\python.exe").is_file()
        && path.join(r"ComfyUI\main.py").is_file()
}

pub fn list(app: &tauri::AppHandle) -> Result<Vec<Instance>, AppError> {
    read_all(app)
}

/// Проверяет папку и заодно сообщает, не зарегистрирована ли она уже.
pub fn probe(
    app: &tauri::AppHandle,
    discovery: &dyn InstanceDiscovery,
    path: &str,
) -> Result<ProbeResult, AppError> {
    let probe = discovery.probe(&PathBuf::from(path))?;
    let existing = read_all(app)?
        .into_iter()
        .find(|i| same_path(&i.path, &probe.path))
        .map(|i| i.id);

    Ok(ProbeResult {
        existing_id: existing,
        suggested_name: suggest_name(&probe.path),
        suggested_port: suggest_port(&read_all(app)?),
        probe,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ProbeResult {
    pub probe: Probe,
    /// Папка уже в реестре. Второй инстанс на неё не заводим — показываем
    /// существующий.
    pub existing_id: Option<String>,
    pub suggested_name: String,
    pub suggested_port: u16,
}

/// Windows не различает регистр в путях, а пользователь может выбрать
/// ту же папку из другого места и получить другое написание.
fn same_path(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

fn suggest_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("ComfyUI")
        .to_string()
}

/// Первый порт, не занятый предпочтениями других инстансов.
///
/// Это не проверка занятости в системе: она делается при старте, в Фазе 2.
/// Здесь задача скромнее — не предлагать один и тот же порт дважды.
fn suggest_port(list: &[Instance]) -> u16 {
    let taken: HashSet<u16> = list.iter().map(|i| i.preferred_port).collect();
    (DEFAULT_PORT..=u16::MAX)
        .find(|p| !taken.contains(p))
        .unwrap_or(DEFAULT_PORT)
}

fn next_accent(list: &[Instance]) -> Accent {
    ACCENT_CYCLE[list.len() % ACCENT_CYCLE.len()]
}

pub fn suggest_accent(app: &tauri::AppHandle) -> Result<Accent, AppError> {
    Ok(next_accent(&read_all(app)?))
}

pub fn add(
    app: &tauri::AppHandle,
    probe: Probe,
    edit: InstanceEdit,
) -> Result<Instance, AppError> {
    let mut list = read_all(app)?;

    if let Some(existing) = list.iter().find(|i| same_path(&i.path, &probe.path)) {
        return Err(AppError::with("instances.duplicate", "name", &existing.name));
    }
    validate(&edit)?;

    let instance = Instance {
        id: new_id(&list),
        name: edit.name,
        description: edit.description,
        path: probe.path,
        accent: edit.accent,
        preferred_port: edit.preferred_port,
        comfy_version: probe.comfy_version,
        python_version: probe.python_version,
        profiles: probe.profiles,
        created_at: now_ms(),
        source: None,
        size_bytes: None,
        size_measured_at: None,
        available: true,
    };

    list.push(instance.clone());
    write_all(app, &list)?;
    Ok(instance)
}

pub fn update(
    app: &tauri::AppHandle,
    id: &str,
    edit: InstanceEdit,
) -> Result<Instance, AppError> {
    validate(&edit)?;

    let mut list = read_all(app)?;
    let instance = list
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    instance.name = edit.name;
    instance.description = edit.description;
    instance.accent = edit.accent;
    // Порт применится со следующего запуска: текущий процесс уже занял свой,
    // и менять его на лету нечем.
    instance.preferred_port = edit.preferred_port;

    let updated = instance.clone();
    write_all(app, &list)?;
    Ok(updated)
}

/// Убирает инстанс из реестра. Папку на диске не трогает — никогда.
pub fn remove(app: &tauri::AppHandle, id: &str) -> Result<(), AppError> {
    let mut list = read_all(app)?;
    let before = list.len();
    list.retain(|i| i.id != id);
    if list.len() == before {
        return Err(AppError::with("instances.notFound", "id", id));
    }
    write_all(app, &list)
}

fn validate(edit: &InstanceEdit) -> Result<(), AppError> {
    if edit.name.trim().is_empty() {
        return Err(AppError::new("instances.emptyName"));
    }
    // Порты ниже 1024 требуют прав администратора, а ноль означает
    // «любой свободный» — предпочтением он быть не может.
    if edit.preferred_port < 1024 {
        return Err(AppError::with("instances.portRange", "min", 1024));
    }
    Ok(())
}

fn new_id(list: &[Instance]) -> String {
    let mut candidate = format!("i{}", now_ms() as u64);
    let mut suffix = 0;
    while list.iter().any(|i| i.id == candidate) {
        suffix += 1;
        candidate = format!("i{}-{suffix}", now_ms() as u64);
    }
    candidate
}

/// Считает размер дерева и сохраняет результат в реестр.
///
/// Только в фоне: 52 ГБ обходились больше пяти минут, и на главном потоке
/// это выглядело бы как зависшее приложение. Результат кэшируется, чтобы
/// при следующем открытии экрана показать его сразу.
pub fn measure_size(
    app: &tauri::AppHandle,
    jobs: &SizeJobs,
    id: &str,
) -> Result<Option<Sized_>, AppError> {
    let list = read_all(app)?;
    let instance = list
        .iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    if !instance.available {
        return Err(AppError::with("instances.missing", "path", &instance.path));
    }
    if !jobs.start(id) {
        // Обход уже идёт — второй ничего не ускорит.
        return Ok(None);
    }

    let bytes = dir_size(Path::new(&instance.path));
    jobs.finish(id);

    // Реестр перечитывается заново: пока шёл обход, пользователь мог
    // переименовать инстанс или удалить другой.
    let mut list = read_all(app)?;
    let Some(instance) = list.iter_mut().find(|i| i.id == id) else {
        return Ok(None);
    };
    let measured = Sized_ {
        id: id.to_string(),
        bytes,
        measured_at: now_ms(),
    };
    instance.size_bytes = Some(measured.bytes);
    instance.size_measured_at = Some(measured.measured_at);
    write_all(app, &list)?;

    Ok(Some(measured))
}

/// Результат подсчёта. Имя с подчёркиванием — `Sized` занято в прелюдии Rust.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Sized_ {
    pub id: String,
    pub bytes: f64,
    pub measured_at: f64,
}

/// Обход дерева без рекурсии: у ComfyUI с кастомными нодами глубина
/// вложенности непредсказуема, а переполнение стека здесь было бы
/// падением всего приложения.
fn dir_size(root: &Path) -> f64 {
    let mut total: u64 = 0;
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let Ok(meta) = entry.metadata() else { continue };
            if meta.is_dir() {
                // Символические ссылки не разворачиваем: общая папка моделей
                // подключается ссылкой, и её вес принадлежит не инстансу.
                if !meta.is_symlink() {
                    stack.push(entry.path());
                }
            } else {
                total = total.saturating_add(meta.len());
            }
        }
    }
    total as f64
}
