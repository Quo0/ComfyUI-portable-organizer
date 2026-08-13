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
use crate::shared_models::InstanceShared;

const STORE_FILE: &str = "instances.json";
const KEY_LIST: &str = "instances";

/// Порт по умолчанию — тот же, что у самого ComfyUI.
pub const DEFAULT_PORT: u16 = 8188;

/// Акцентный цвет инстанса.
///
/// Хранится либо именем токена палитры, либо значением `#rrggbb`, если
/// пользователь выбрал свой. Имя лучше и остаётся выбором по умолчанию:
/// у токена своё значение в каждой теме, и читаемость проверена. Свой
/// цвет одинаков в обеих темах, и отвечает за него уже пользователь —
/// но запрещать ему собственный цвет не за что.
///
/// Кортежная структура, а не перечисление: `serde` пишет её прозрачно,
/// то есть реестр, записанный до этой правки, читается как был,
/// а `specta` экспортирует её самим внутренним типом.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct Accent(pub String);

impl Accent {
    pub fn named(name: &str) -> Self {
        Self(name.to_string())
    }

    /// Цвет из палитры или свой в виде `#rrggbb`. Всё прочее — отказ:
    /// в разметку это значение попадает как есть.
    fn valid(&self) -> bool {
        if PALETTE.contains(&self.0.as_str()) {
            return true;
        }
        let hex = self.0.strip_prefix('#').unwrap_or("");
        hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
    }
}

/// Порядок выдачи цветов новым инстансам: подряд, по кругу.
/// Он же — список имён, которые считаются валидными.
pub const PALETTE: [&str; 8] = [
    "teal", "indigo", "ember", "moss", "azure", "orchid", "rose", "amber",
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

    /// Подключение к общему хранилищу моделей. `#[serde(default)]` не
    /// украшение: реестр, записанный до Фазы 2.5, поля не содержит, и без
    /// умолчания разбор всего файла упал бы — то есть пользователь потерял
    /// бы список инстансов из-за появления новой настройки.
    #[serde(default)]
    pub shared: InstanceShared,

    /// Профили, собранные пользователем поверх разобранных из `.bat`.
    ///
    /// Свои, а не правка чужих: `.bat` мы не трогаем никогда, а разбор
    /// перечитывается при каждом запуске — правку в него было бы негде
    /// удержать. `#[serde(default)]` по той же причине, что у `shared`:
    /// реестр, записанный до этой фазы, поля не содержит.
    #[serde(default)]
    pub custom_profiles: Vec<CustomProfile>,

    /// Когда сборку запускали в последний раз, в миллисекундах эпохи.
    ///
    /// `None` — ни разу с тех пор, как приложение это записывает. Дату
    /// форматирует фронт: она обязана следовать выбранному языку.
    #[serde(default)]
    pub last_started_at: Option<f64>,

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

/// Свой профиль запуска.
///
/// Хранятся только имя и аргументы. Интерпретатор, рабочая папка и `env`
/// берутся у базового профиля **в момент запуска**: пользователь мог
/// поправить `.bat` руками, и запомненная копия однажды разошлась бы
/// с тем, что лежит на диске.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CustomProfile {
    /// `custom:<число>`. Префикс отличает свой профиль от имени `.bat`.
    pub id: String,
    pub name: String,
    /// Профиль из `.bat`, у которого берётся всё остальное. Если он исчез —
    /// свой профиль показывается сломанным, а не запускается наугад.
    pub base_id: String,
    pub args: Vec<String>,
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
    Accent::named(PALETTE[list.len() % PALETTE.len()])
}

pub fn suggest_accent(app: &tauri::AppHandle) -> Result<Accent, AppError> {
    Ok(next_accent(&read_all(app)?))
}

/// Регистрирует папку. `source` заполняет мастер установки — по нему
/// в карточке видно, из какого архива развёрнут инстанс; у добавленных
/// вручную его нет.
pub fn add(
    app: &tauri::AppHandle,
    probe: Probe,
    edit: InstanceEdit,
    source: Option<InstallSource>,
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
        source,
        shared: InstanceShared::default(),
        custom_profiles: Vec::new(),
        last_started_at: None,
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

/// Записывает подключение к общим моделям.
///
/// Отдельно от `update`: тот принимает форму редактирования инстанса и
/// перетёр бы имя с описанием значениями, которых у вызывающего нет.
pub fn set_shared(
    app: &tauri::AppHandle,
    id: &str,
    shared: InstanceShared,
) -> Result<(), AppError> {
    let mut list = read_all(app)?;
    let instance = list
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    instance.shared = shared;
    write_all(app, &list)
}

/// Отмечает, что сборку запустили.
///
/// Пишется при удачном старте, а не при попытке: «последний запуск»
/// в списке отвечает на вопрос «когда я этим пользовался», и неудачная
/// попытка ответа на него не даёт.
///
/// Ошибку записи глотаем на уровне вызывающего: не сумели запомнить дату —
/// не повод не запускать сборку.
pub fn mark_started(app: &tauri::AppHandle, id: &str) -> Result<(), AppError> {
    let mut list = read_all(app)?;
    let instance = list
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    instance.last_started_at = Some(now_ms());
    write_all(app, &list)
}

/// Сохраняет свой профиль: новый или поверх существующего с тем же id.
///
/// Пустой `id` означает новый профиль — номер выдаём здесь, чтобы фронт
/// не изобретал уникальность в обход реестра.
pub fn save_profile(
    app: &tauri::AppHandle,
    id: &str,
    mut profile: CustomProfile,
) -> Result<Instance, AppError> {
    if profile.name.trim().is_empty() {
        return Err(AppError::new("instances.emptyName"));
    }

    let mut list = read_all(app)?;
    let instance = list
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    if profile.id.is_empty() {
        let mut n = instance.custom_profiles.len() + 1;
        while instance.custom_profiles.iter().any(|p| p.id == format!("custom:{n}")) {
            n += 1;
        }
        profile.id = format!("custom:{n}");
    }

    match instance.custom_profiles.iter_mut().find(|p| p.id == profile.id) {
        Some(existing) => *existing = profile,
        None => instance.custom_profiles.push(profile),
    }

    let updated = instance.clone();
    write_all(app, &list)?;
    Ok(updated)
}

/// Удаляет свой профиль. Профили из `.bat` удалить нельзя — их не мы завели.
pub fn remove_profile(
    app: &tauri::AppHandle,
    id: &str,
    profile_id: &str,
) -> Result<Instance, AppError> {
    let mut list = read_all(app)?;
    let instance = list
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    instance.custom_profiles.retain(|p| p.id != profile_id);
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
    // Цвет уходит прямо в разметку значением CSS-переменной, и пускать
    // туда произвольную строку нельзя.
    if !edit.accent.valid() {
        return Err(AppError::with("instances.badAccent", "value", &edit.accent.0));
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
