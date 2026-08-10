//! Мастер установки: разбор архива, распаковка, клонирование дерева.
//!
//! Приложение не скачивает архив само — источник выбирает пользователь.
//! И не обновляет существующий инстанс на месте: мастер разворачивает новое
//! рядом, старые сборки остаются нетронутыми.
//!
//! Решение по декодеру принято замером, а не ощущением: `sevenz-rust2`
//! втрое медленнее 7-Zip на реальном архиве (238 с против 81), и это принято.
//! Подробности и цифры — в PLAN.md, раздел «Мастер установки».

use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sevenz_rust2::{ArchiveEntry, ArchiveReader, Password};
use tauri_specta::Event;

use crate::error::AppError;

/// Пока распаковка не закончена, папка называется так. Валидация инстанса —
/// это наличие `python_embeded\python.exe` и `ComfyUI\main.py`, и
/// полураспакованное дерево её пройдёт. Без временного имени прерванная
/// установка оставила бы битый инстанс, который приложение сочтёт рабочим.
const PARTIAL_SUFFIX: &str = ".cpo-partial";

/// Запас поверх заголовка архива: файловая система тратит место на записи
/// каталогов, а десятки тысяч мелких файлов округляются вверх до кластера.
const SPACE_MARGIN: f64 = 1.1;

/// Порог предупреждения о длинном пути. За ним обычные программы —
/// сам ComfyUI, pip, python — начинают спотыкаться о MAX_PATH, даже если
/// наша распаковка справится за счёт verbatim-путей.
const MAX_PATH: usize = 260;

/// Событий больше десяти в секунду интерфейс всё равно не покажет, а IPC
/// на 56 тысячах файлов заметно нагружает.
const PROGRESS_INTERVAL_MS: u128 = 100;

// ------------------------------------------------------------- модель

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveInfo {
    pub path: String,
    /// Имя файла. Показывается в карточке инстанса как источник.
    pub label: String,
    pub size_bytes: f64,
    /// Миллисекунды эпохи. Вместе с размером опознаёт подмену файла.
    pub mtime: f64,
    pub files: u32,
    pub folders: u32,
    pub total_uncompressed: f64,
    /// Единственная корневая папка архива. Её имя задаёт пользователь,
    /// поэтому из путей она срезается — заодно минус 25 символов к длине.
    pub single_root: Option<String>,
    /// Самый длинный путь внутри архива после среза корня.
    pub longest_entry: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstallTarget {
    pub path: String,
    pub name: String,
    pub description: String,
    pub accent: crate::instances::Accent,
    pub preferred_port: u16,
}

/// Что не так с целью. Разделены осознанно: с предупреждением установку
/// начать можно, с ошибкой — нет.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TargetCheck {
    pub path: String,
    pub errors: Vec<AppError>,
    pub warnings: Vec<AppError>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum InstallPhase {
    /// Распаковка архива в первую цель.
    Extracting,
    /// Копирование готового дерева в остальные цели.
    Copying,
    /// Регистрация в реестре.
    Registering,
}

#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
#[serde(rename_all = "camelCase")]
pub struct InstallProgress {
    pub phase: InstallPhase,
    /// Номер цели, начиная с единицы, и сколько их всего.
    pub target: u32,
    pub targets: u32,
    pub target_name: String,
    /// Путь текущего файла внутри инстанса. Не переводится.
    pub current: String,
    pub done_bytes: f64,
    pub total_bytes: f64,
}

/// Отмена мастера. Проверяется между файлами: прерывать распаковку одного
/// файла посреди потока незачем, самый крупный в архиве — считанные мегабайты.
#[derive(Default)]
pub struct InstallCancel(AtomicBool);

impl InstallCancel {
    pub fn request(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    pub fn reset(&self) {
        self.0.store(false, Ordering::Relaxed);
    }

    fn requested(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Идёт ли установка прямо сейчас. Две одновременные распакуют одно поверх
/// другого и подерутся за диск.
#[derive(Default)]
pub struct InstallLock(Mutex<bool>);

// ------------------------------------------------------------- разбор

pub fn probe_archive(path: &str) -> Result<ArchiveInfo, AppError> {
    let file = Path::new(path);
    let meta = fs::metadata(file)
        .map_err(|e| AppError::because("installer.archiveUnreadable", e))?;

    let reader = ArchiveReader::open(file, Password::empty())
        .map_err(|e| AppError::because("installer.archiveUnreadable", e))?;

    let entries = &reader.archive().files;
    let root = single_root(entries);
    let files = entries.iter().filter(|e| !e.is_directory).count() as u32;

    Ok(ArchiveInfo {
        path: path.to_string(),
        label: file
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(path)
            .to_string(),
        size_bytes: meta.len() as f64,
        mtime: mtime_ms(&meta),
        files,
        folders: entries.len() as u32 - files,
        total_uncompressed: entries.iter().map(|e| e.size).sum::<u64>() as f64,
        longest_entry: entries
            .iter()
            .map(|e| strip_root(&e.name, root.as_deref()).chars().count())
            .max()
            .unwrap_or(0) as u32,
        single_root: root,
    })
}

fn mtime_ms(meta: &fs::Metadata) -> f64 {
    meta.modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0)
}

fn single_root(entries: &[ArchiveEntry]) -> Option<String> {
    let mut root: Option<String> = None;
    for entry in entries {
        let first = entry.name.split(['/', '\\']).next().unwrap_or("");
        if first.is_empty() {
            return None;
        }
        match &root {
            None => root = Some(first.to_string()),
            Some(known) if known != first => return None,
            _ => {}
        }
    }
    root
}

fn strip_root<'a>(name: &'a str, root: Option<&str>) -> &'a str {
    let Some(root) = root else { return name };
    name.strip_prefix(root)
        .map(|rest| rest.trim_start_matches(['/', '\\']))
        .unwrap_or(name)
}

// ------------------------------------------------------------- проверки

/// Проверяет цели до начала работы: место, пустоту папок, длину пути.
///
/// Ошибки и предупреждения разделены: длинный путь установку не ломает
/// благодаря verbatim-путям, но ломает всё, что запустится потом.
pub fn check_targets(info: &ArchiveInfo, targets: &[InstallTarget]) -> Vec<TargetCheck> {
    let mut checks = Vec::new();

    for target in targets {
        let path = Path::new(&target.path);
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        if target.path.trim().is_empty() {
            errors.push(AppError::new("installer.emptyPath"));
        } else if !path.is_absolute() {
            errors.push(AppError::with("installer.notAbsolute", "path", &target.path));
        }

        // Непустая папка — почти наверняка чужие данные. Сносить их мы
        // не будем и распаковываться поверх тоже: получится смесь.
        if path.is_dir() {
            let empty = fs::read_dir(path)
                .map(|mut d| d.next().is_none())
                .unwrap_or(false);
            if !empty {
                errors.push(AppError::with("installer.notEmpty", "path", &target.path));
            }
        } else if path.exists() {
            errors.push(AppError::with("installer.notADirectory", "path", &target.path));
        }

        // Один и тот же путь дважды — распаковка и копирование подрались бы
        // за одни файлы.
        if targets
            .iter()
            .filter(|t| t.path.eq_ignore_ascii_case(&target.path))
            .count()
            > 1
        {
            errors.push(AppError::with("installer.duplicateTarget", "path", &target.path));
        }

        let projected = target.path.chars().count() + 1 + info.longest_entry as usize;
        if projected > MAX_PATH {
            warnings.push(AppError::with(
                "installer.longPath",
                "chars",
                projected,
            ));
        }

        checks.push(TargetCheck { path: target.path.clone(), errors, warnings });
    }

    // Свободное место считается на том, а не на папку: две цели на одном
    // диске требуют вдвое больше.
    let needed = info.total_uncompressed * SPACE_MARGIN;
    for check in &mut checks {
        let path = Path::new(&check.path);
        let Some(root) = volume_root(path) else { continue };
        let same_volume = targets
            .iter()
            .filter(|t| volume_root(Path::new(&t.path)).as_deref() == Some(&root))
            .count() as f64;
        let Some(free) = free_space(&root) else { continue };

        if free < needed * same_volume {
            // Гигабайты, а не байты: сообщение читает человек. Разделитель
            // дробной части здесь остаётся точкой — единственное место, где
            // число не проходит через локаль, и ради него тащить форматирование
            // в бэкенд не стоит.
            check.errors.push(AppError::with(
                "installer.noSpace",
                "needed",
                format!("{:.1}", needed * same_volume / 1024f64.powi(3)),
            ));
        }
    }

    checks
}

/// Корень тома: `D:\`. Папки назначения может ещё не быть, поэтому берём
/// не саму папку, а начало пути.
fn volume_root(path: &Path) -> Option<String> {
    let text = path.display().to_string();
    let mut chars = text.chars();
    let letter = chars.next()?;
    if chars.next()? != ':' {
        return None;
    }
    Some(format!(r"{}:\", letter.to_ascii_uppercase()))
}

#[cfg(windows)]
fn free_space(root: &str) -> Option<f64> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;

    let wide: Vec<u16> = std::ffi::OsStr::new(root)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut available: u64 = 0;
    // SAFETY: строка завершена нулём, указатели живут дольше вызова.
    let ok = unsafe {
        GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &mut available,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    if ok == 0 {
        None
    } else {
        Some(available as f64)
    }
}

#[cfg(not(windows))]
fn free_space(_root: &str) -> Option<f64> {
    None
}

// ------------------------------------------------------------- работа

/// Распаковывает архив в первую цель и копирует дерево в остальные.
///
/// Распаковываем один раз, дальше копируем: декомпрессия упирается в CPU
/// и стоит вчетверо дороже копирования готового дерева. При двух-трёх
/// целях это экономит минуты.
pub fn run<F>(
    info: &ArchiveInfo,
    targets: &[InstallTarget],
    cancel: &InstallCancel,
    mut report: F,
) -> Result<(), AppError>
where
    F: FnMut(InstallProgress),
{
    let Some(first) = targets.first() else {
        return Err(AppError::new("installer.noTargets"));
    };

    let first_dest = PathBuf::from(&first.path);
    extract(info, &first_dest, first, targets.len() as u32, cancel, &mut report)?;

    for (index, target) in targets.iter().enumerate().skip(1) {
        clone_tree(
            &first_dest,
            &PathBuf::from(&target.path),
            info,
            target,
            index as u32 + 1,
            targets.len() as u32,
            cancel,
            &mut report,
        )?;
    }

    Ok(())
}

fn extract<F>(
    info: &ArchiveInfo,
    dest: &Path,
    target: &InstallTarget,
    targets: u32,
    cancel: &InstallCancel,
    report: &mut F,
) -> Result<(), AppError>
where
    F: FnMut(InstallProgress),
{
    let partial = partial_of(dest);
    // Прошлая попытка могла оборваться — начинаем с чистого места.
    remove_tree(&partial)?;

    let outcome = extract_into(info, &partial, target, targets, cancel, report);

    if outcome.is_err() || cancel.requested() {
        // Отмена и падение убирают временную папку: битому дереву,
        // которое пройдёт валидацию инстанса, тут делать нечего.
        let _ = remove_tree(&partial);
        return outcome.and(Err(AppError::new("installer.cancelled")));
    }

    fs::create_dir_all(dest.parent().unwrap_or(dest))
        .map_err(|e| AppError::because("installer.writeFailed", e))?;
    fs::rename(verbatim(&partial), verbatim(dest))
        .map_err(|e| AppError::because("installer.writeFailed", e))
}

fn extract_into<F>(
    info: &ArchiveInfo,
    partial: &Path,
    target: &InstallTarget,
    targets: u32,
    cancel: &InstallCancel,
    report: &mut F,
) -> Result<(), AppError>
where
    F: FnMut(InstallProgress),
{
    let mut reader = ArchiveReader::open(&info.path, Password::empty())
        .map_err(|e| AppError::because("installer.archiveUnreadable", e))?;

    fs::create_dir_all(verbatim(partial))
        .map_err(|e| AppError::because("installer.writeFailed", e))?;

    let root = info.single_root.clone();
    let total = info.total_uncompressed;
    let mut done = 0f64;
    let mut last = Instant::now();

    reader
        .for_each_entries(|entry, stream| {
            if cancel.requested() {
                return Ok(false);
            }

            let rel = strip_root(&entry.name, root.as_deref()).to_string();
            if rel.is_empty() {
                return Ok(true);
            }
            let out = verbatim(&partial.join(&rel));

            if entry.is_directory {
                fs::create_dir_all(&out)?;
                return Ok(true);
            }
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut file = File::create(&out)?;
            done += io::copy(stream, &mut file)? as f64;

            if last.elapsed().as_millis() >= PROGRESS_INTERVAL_MS {
                last = Instant::now();
                report(InstallProgress {
                    phase: InstallPhase::Extracting,
                    target: 1,
                    targets,
                    target_name: target.name.clone(),
                    current: rel,
                    done_bytes: done,
                    total_bytes: total,
                });
            }
            Ok(true)
        })
        .map_err(|e| AppError::because("installer.extractFailed", e))?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn clone_tree<F>(
    from: &Path,
    to: &Path,
    info: &ArchiveInfo,
    target: &InstallTarget,
    index: u32,
    targets: u32,
    cancel: &InstallCancel,
    report: &mut F,
) -> Result<(), AppError>
where
    F: FnMut(InstallProgress),
{
    let partial = partial_of(to);
    remove_tree(&partial)?;

    let outcome = copy_into(from, &partial, info, target, index, targets, cancel, report);

    if outcome.is_err() || cancel.requested() {
        let _ = remove_tree(&partial);
        return outcome.and(Err(AppError::new("installer.cancelled")));
    }

    fs::rename(verbatim(&partial), verbatim(to))
        .map_err(|e| AppError::because("installer.writeFailed", e))
}

#[allow(clippy::too_many_arguments)]
fn copy_into<F>(
    from: &Path,
    to: &Path,
    info: &ArchiveInfo,
    target: &InstallTarget,
    index: u32,
    targets: u32,
    cancel: &InstallCancel,
    report: &mut F,
) -> Result<(), AppError>
where
    F: FnMut(InstallProgress),
{
    fs::create_dir_all(verbatim(to))
        .map_err(|e| AppError::because("installer.writeFailed", e))?;

    let mut done = 0f64;
    let mut last = Instant::now();
    // Обход без рекурсии: глубина дерева питона непредсказуема, а переполнение
    // стека здесь было бы падением всего приложения.
    let mut stack = vec![PathBuf::new()];

    while let Some(rel_dir) = stack.pop() {
        if cancel.requested() {
            return Ok(());
        }
        let source_dir = verbatim(&from.join(&rel_dir));
        let entries = fs::read_dir(&source_dir)
            .map_err(|e| AppError::because("installer.copyFailed", e))?;

        for entry in entries.flatten() {
            if cancel.requested() {
                return Ok(());
            }
            let name = entry.file_name();
            let rel = rel_dir.join(&name);
            let out = verbatim(&to.join(&rel));

            let meta = entry
                .metadata()
                .map_err(|e| AppError::because("installer.copyFailed", e))?;

            if meta.is_dir() {
                fs::create_dir_all(&out)
                    .map_err(|e| AppError::because("installer.copyFailed", e))?;
                stack.push(rel);
                continue;
            }

            fs::copy(entry.path(), &out)
                .map_err(|e| AppError::because("installer.copyFailed", e))?;
            done += meta.len() as f64;

            if last.elapsed().as_millis() >= PROGRESS_INTERVAL_MS {
                last = Instant::now();
                report(InstallProgress {
                    phase: InstallPhase::Copying,
                    target: index,
                    targets,
                    target_name: target.name.clone(),
                    current: rel.display().to_string(),
                    done_bytes: done,
                    total_bytes: info.total_uncompressed,
                });
            }
        }
    }

    Ok(())
}

fn partial_of(dest: &Path) -> PathBuf {
    let mut name = dest.as_os_str().to_os_string();
    name.push(PARTIAL_SUFFIX);
    PathBuf::from(name)
}

/// Удаляет дерево, повторяя попытки.
///
/// Одиночного вызова мало: сразу после распаковки часть файлов держит
/// антивирус, и удаление возвращает то «папка не пуста», то «доступ
/// запрещён», причём набор запертых файлов меняется от попытки к попытке.
/// Проверено на реальном архиве — см. PLAN.md, «Что ещё вскрыл спайк».
fn remove_tree(path: &Path) -> Result<(), AppError> {
    if !path.exists() {
        return Ok(());
    }
    let mut last_error = None;
    for attempt in 0..10 {
        match fs::remove_dir_all(verbatim(path)) {
            Ok(()) => return Ok(()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(e) => last_error = Some(e),
        }
        std::thread::sleep(std::time::Duration::from_millis(300 * (attempt + 1)));
    }
    Err(AppError::because(
        "installer.cleanupFailed",
        last_error.map(|e| e.to_string()).unwrap_or_default(),
    ))
}

/// Verbatim-путь `\\?\`, снимающий лимит MAX_PATH.
///
/// Прямые слэши обязаны стать обратными: verbatim означает «передать ядру
/// как есть», обычная нормализация отключается вместе с лимитом, и `/`
/// из имён записей архива даёт ошибку 123 без намёка на причину.
fn verbatim(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        let text = path.display().to_string();
        if text.starts_with(r"\\?\") {
            return path.to_path_buf();
        }
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default().join(path)
        };
        let normalized = absolute.display().to_string().replace('/', r"\");
        return PathBuf::from(format!(r"\\?\{normalized}"));
    }
    #[cfg(not(windows))]
    path.to_path_buf()
}

// ------------------------------------------------------- история архивов

/// Хранится именно история, а не последний путь: пользователь держит
/// несколько версий сборки и разворачивает их рядом.
pub mod history {
    use super::*;
    use tauri_plugin_store::StoreExt;

    const STORE_FILE: &str = "installer.json";
    const KEY: &str = "archives";
    const LIMIT: usize = 10;

    #[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
    #[serde(rename_all = "camelCase")]
    pub struct ArchiveRecord {
        pub path: String,
        pub label: String,
        pub size_bytes: f64,
        pub mtime: f64,
        pub last_used_at: f64,
        /// Файл на месте и не изменился с прошлого раза. Пересчитывается
        /// при каждом чтении: архив могли удалить или подменить.
        pub available: bool,
    }

    pub fn list(app: &tauri::AppHandle) -> Result<Vec<ArchiveRecord>, AppError> {
        let store = app
            .store(STORE_FILE)
            .map_err(|e| AppError::because("installer.historyFailed", e))?;

        let mut list: Vec<ArchiveRecord> = store
            .get(KEY)
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        for record in &mut list {
            record.available = match fs::metadata(&record.path) {
                Ok(meta) => {
                    meta.len() as f64 == record.size_bytes && mtime_ms(&meta) == record.mtime
                }
                Err(_) => false,
            };
        }
        Ok(list)
    }

    pub fn remember(app: &tauri::AppHandle, info: &ArchiveInfo) -> Result<(), AppError> {
        let mut list = list(app)?;
        list.retain(|r| !r.path.eq_ignore_ascii_case(&info.path));
        list.insert(
            0,
            ArchiveRecord {
                path: info.path.clone(),
                label: info.label.clone(),
                size_bytes: info.size_bytes,
                mtime: info.mtime,
                last_used_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis() as f64)
                    .unwrap_or(0.0),
                available: true,
            },
        );
        list.truncate(LIMIT);
        write(app, &list)
    }

    pub fn forget(app: &tauri::AppHandle, path: &str) -> Result<(), AppError> {
        let mut list = list(app)?;
        list.retain(|r| !r.path.eq_ignore_ascii_case(path));
        write(app, &list)
    }

    fn write(app: &tauri::AppHandle, list: &[ArchiveRecord]) -> Result<(), AppError> {
        let store = app
            .store(STORE_FILE)
            .map_err(|e| AppError::because("installer.historyFailed", e))?;
        let value = serde_json::to_value(list)
            .map_err(|e| AppError::because("installer.historyFailed", e))?;
        store.set(KEY, value);
        store
            .save()
            .map_err(|e| AppError::because("installer.historyFailed", e))
    }
}

impl InstallLock {
    /// Возвращает страж, снимающий блокировку при выходе из области.
    pub fn acquire(&self) -> Result<InstallGuard<'_>, AppError> {
        let mut busy = self.0.lock().unwrap();
        if *busy {
            return Err(AppError::new("installer.busy"));
        }
        *busy = true;
        Ok(InstallGuard(&self.0))
    }
}

pub struct InstallGuard<'a>(&'a Mutex<bool>);

impl Drop for InstallGuard<'_> {
    fn drop(&mut self) {
        *self.0.lock().unwrap() = false;
    }
}
