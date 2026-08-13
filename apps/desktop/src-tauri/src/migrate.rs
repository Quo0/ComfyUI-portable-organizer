//! Перенос моделей из сборки в общую папку и уборка дубликатов.
//!
//! Отдельным модулем от `shared_models.rs` намеренно: там сканирование
//! и генерация YAML, здесь — единственное место во всём приложении, где мы
//! удаляем файлы моделей. Граница из `CLAUDE.md` допускает ровно два таких
//! случая, оба по явной просьбе пользователя и с перечнем заранее:
//! перенос, где исходник исчезает **после** проверки, что копия на месте,
//! и уборка дубликата, который уже лежит в общей папке.
//!
//! Молча не удаляется ничего и никогда.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// Сколько байт с каждого края сверяем, решая, что файлы одинаковы.
///
/// Полный хэш на моделях по 20 ГБ неприемлем — это записано в плане про
/// анализатор дублей. Два мегабайта чтения стоят миллисекунд и превращают
/// «одно имя и один размер» в почти достоверное совпадение.
const EDGE: u64 = 1024 * 1024;

/// Категории, содержимое которых принадлежит сборке, а не пользователю.
///
/// `configs` ComfyUI поставляет вместе с собой — там `v1-inference.yaml`
/// и подобные. Унести их значит обокрасть установку. `custom_nodes`
/// не шарится вовсе и в общую папку попасть не должен ни при каких
/// обстоятельствах.
const NEVER_MOVE: [&str; 2] = ["configs", "custom_nodes"];

/// Чем оказался элемент, чьё имя уже занято в общей папке.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SameName {
    /// Совпали размер и края — почти наверняка тот же файл.
    Duplicate,
    /// Каталог: совпали суммарный объём и число файлов. Основание слабее,
    /// поэтому и называется иначе.
    LikelyDuplicate,
    /// Размеры или края разошлись. **Удалять нельзя ни при каких условиях:**
    /// это разные файлы, которым не повезло с именем.
    Different,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ModelEntry {
    /// Имя элемента внутри категории. Файл или каталог целиком.
    pub name: String,
    pub is_dir: bool,
    pub size_bytes: f64,
    pub files: u32,
    /// Занято ли это имя в общей папке и чем оно там оказалось.
    pub same_name: Option<SameName>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ModelCategory {
    pub folder: String,
    pub entries: Vec<ModelEntry>,
    pub size_bytes: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ModelsScan {
    pub path: String,
    pub available: bool,
    pub categories: Vec<ModelCategory>,
    /// Сколько всего перенесётся и сколько это займёт.
    pub total_files: u32,
    pub total_bytes: f64,
}

/// Флаг отмены, общий с командой прерывания. Тот же приём, что
/// у `InstallCancel` в инсталляторе.
#[derive(Default, Clone)]
pub struct MigrateCancel(Arc<AtomicBool>);

impl MigrateCancel {
    pub fn reset(&self) {
        self.0.store(false, Ordering::Relaxed);
    }
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    pub fn share(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MigrateOutcome {
    pub moved: Vec<String>,
    /// Пропущенные из-за занятого имени, с вердиктом по каждому.
    pub skipped: Vec<Skipped>,
    /// Не удалось, с причиной. Сбой на одном не отменяет остальные.
    pub failed: Vec<Failed>,
    pub moved_bytes: f64,
    pub cancelled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Skipped {
    pub category: String,
    pub name: String,
    pub verdict: SameName,
    pub size_bytes: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Failed {
    pub category: String,
    pub name: String,
    pub reason: String,
}

/// Файл-маркер, который ComfyUI кладёт в пустую категорию.
///
/// Отличаем по имени и нулевому размеру сразу: пользовательский файл
/// с таким именем возможен, но не нулевой.
pub(crate) fn is_placeholder(path: &Path, size: u64) -> bool {
    size == 0
        && path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("put_") && n.ends_with("_here"))
            .unwrap_or(false)
}

/// Размер и число файлов в дереве. Каталог считается целиком.
pub(crate) fn measure(path: &Path) -> (u64, u32) {
    let Ok(meta) = std::fs::metadata(path) else {
        return (0, 0);
    };
    if meta.is_file() {
        return (meta.len(), 1);
    }

    let mut bytes = 0u64;
    let mut files = 0u32;
    let mut stack = vec![path.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let Ok(kind) = entry.file_type() else { continue };
            if kind.is_dir() {
                stack.push(entry.path());
            } else if let Ok(m) = entry.metadata() {
                bytes = bytes.saturating_add(m.len());
                files = files.saturating_add(1);
            }
        }
    }
    (bytes, files)
}

/// Одинаковы ли края файлов.
///
/// Читаем по мегабайту с начала и с конца. Файлы короче двух мегабайт
/// сверяем целиком — это дешевле, чем считать смещения.
fn same_edges(a: &Path, b: &Path, size: u64) -> bool {
    use std::io::{Read, Seek, SeekFrom};

    let (Ok(mut fa), Ok(mut fb)) = (std::fs::File::open(a), std::fs::File::open(b)) else {
        return false;
    };

    let head = EDGE.min(size) as usize;
    let mut ba = vec![0u8; head];
    let mut bb = vec![0u8; head];
    if fa.read_exact(&mut ba).is_err() || fb.read_exact(&mut bb).is_err() || ba != bb {
        return false;
    }
    if size <= EDGE * 2 {
        return true;
    }

    let tail = SeekFrom::End(-(EDGE as i64));
    if fa.seek(tail).is_err() || fb.seek(tail).is_err() {
        return false;
    }
    let mut ta = vec![0u8; EDGE as usize];
    let mut tb = vec![0u8; EDGE as usize];
    if fa.read_exact(&mut ta).is_err() || fb.read_exact(&mut tb).is_err() {
        return false;
    }
    ta == tb
}

/// Что за элемент лежит в общей папке под тем же именем.
pub fn compare(local: &Path, shared: &Path) -> SameName {
    let (Ok(a), Ok(b)) = (std::fs::metadata(local), std::fs::metadata(shared)) else {
        return SameName::Different;
    };

    if a.is_dir() != b.is_dir() {
        return SameName::Different;
    }

    if a.is_file() {
        if a.len() != b.len() {
            return SameName::Different;
        }
        return if same_edges(local, shared, a.len()) {
            SameName::Duplicate
        } else {
            // Размер тот же, содержимое иное. Ровно ради этого случая
            // края и читаются.
            SameName::Different
        };
    }

    let (bytes_a, files_a) = measure(local);
    let (bytes_b, files_b) = measure(shared);
    if bytes_a == bytes_b && files_a == files_b {
        SameName::LikelyDuplicate
    } else {
        SameName::Different
    }
}

/// Читает модели сборки и сверяет их с общей папкой.
pub fn scan(models_dir: &Path, shared_root: &Path) -> ModelsScan {
    let display = models_dir.display().to_string();
    if !models_dir.is_dir() {
        return ModelsScan {
            path: display,
            available: false,
            categories: Vec::new(),
            total_files: 0,
            total_bytes: 0.0,
        };
    }

    let mut categories = Vec::new();
    let Ok(dirs) = std::fs::read_dir(models_dir) else {
        return ModelsScan {
            path: display,
            available: false,
            categories: Vec::new(),
            total_files: 0,
            total_bytes: 0.0,
        };
    };

    for dir in dirs.flatten() {
        if !dir.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }
        let folder = dir.file_name().to_string_lossy().to_string();
        if NEVER_MOVE.contains(&folder.as_str()) {
            continue;
        }

        let mut entries = Vec::new();
        let Ok(items) = std::fs::read_dir(dir.path()) else { continue };

        for item in items.flatten() {
            let path = item.path();
            let is_dir = item.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let (size_bytes, files) = measure(&path);

            if !is_dir && is_placeholder(&path, size_bytes) {
                continue;
            }

            let name = item.file_name().to_string_lossy().to_string();
            let twin = shared_root.join(&folder).join(&name);
            let same_name = twin.exists().then(|| compare(&path, &twin));

            entries.push(ModelEntry {
                name,
                is_dir,
                size_bytes: size_bytes as f64,
                files,
                same_name,
            });
        }

        if entries.is_empty() {
            continue;
        }
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        let size_bytes = entries.iter().map(|e| e.size_bytes).sum();
        categories.push(ModelCategory { folder, entries, size_bytes });
    }

    categories.sort_by(|a, b| a.folder.cmp(&b.folder));

    // В счёт идёт только то, что действительно поедет: занятые имена
    // остаются на месте.
    let movable = |e: &&ModelEntry| e.same_name.is_none();
    let total_files = categories
        .iter()
        .flat_map(|c| c.entries.iter().filter(movable))
        .map(|e| e.files)
        .sum();
    let total_bytes = categories
        .iter()
        .flat_map(|c| c.entries.iter().filter(movable))
        .map(|e| e.size_bytes)
        .sum();

    ModelsScan { path: display, available: true, categories, total_files, total_bytes }
}

/// Ход переноса. Считается по элементам, а не по байтам: на одном томе
/// перенос мгновенен, и полоса по байтам прыгала бы бессмысленно.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct MigrateProgress {
    pub done: u32,
    pub total: u32,
    pub category: String,
    pub name: String,
}

/// Переносит выбранные категории в общую папку.
///
/// Занятые имена не трогаются вовсе: чужая модель на 20 ГБ дороже дубля.
pub fn move_all(
    models_dir: &Path,
    shared_root: &Path,
    categories: &[String],
    cancel: &MigrateCancel,
    mut on_progress: impl FnMut(MigrateProgress),
) -> MigrateOutcome {
    let scan = scan(models_dir, shared_root);
    let chosen: Vec<&ModelCategory> = scan
        .categories
        .iter()
        .filter(|c| categories.iter().any(|w| w == &c.folder))
        .collect();

    let total: u32 = chosen.iter().map(|c| c.entries.len() as u32).sum();
    let mut out = MigrateOutcome {
        moved: Vec::new(),
        skipped: Vec::new(),
        failed: Vec::new(),
        moved_bytes: 0.0,
        cancelled: false,
    };
    let mut done = 0u32;

    for category in chosen {
        let target_dir = shared_root.join(&category.folder);
        for entry in &category.entries {
            if cancel.is_cancelled() {
                out.cancelled = true;
                return out;
            }
            done += 1;
            on_progress(MigrateProgress {
                done,
                total,
                category: category.folder.clone(),
                name: entry.name.clone(),
            });

            if let Some(verdict) = entry.same_name {
                out.skipped.push(Skipped {
                    category: category.folder.clone(),
                    name: entry.name.clone(),
                    verdict,
                    size_bytes: entry.size_bytes,
                });
                continue;
            }

            let from = models_dir.join(&category.folder).join(&entry.name);
            let to = target_dir.join(&entry.name);
            match move_entry(&from, &to) {
                Ok(()) => {
                    out.moved.push(format!("{}/{}", category.folder, entry.name));
                    out.moved_bytes += entry.size_bytes;
                }
                // Сбой на одном элементе не отменяет остальные: категорий
                // десятки, и бросать всё из-за одного занятого файла глупо.
                Err(e) => out.failed.push(Failed {
                    category: category.folder.clone(),
                    name: entry.name.clone(),
                    reason: e.code,
                }),
            }
        }
    }

    out
}

/// Переносит один элемент.
///
/// На одном томе — переименование, мгновенно и без риска. Между томами
/// сначала копия во временное имя, потом сверка, потом постановка на место
/// и **только потом** удаление исходника: пока копия не проверена, у нас
/// на руках должен оставаться оригинал.
fn move_entry(from: &Path, to: &Path) -> Result<(), AppError> {
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AppError::because("migrate.writeFailed", e))?;
    }

    if std::fs::rename(from, to).is_ok() {
        return Ok(());
    }

    // Тот же приём, что у инсталлятора: недоделанное всегда носит имя,
    // по которому его видно и не жалко убрать.
    let staging = to.with_extension("cpo-partial");
    let _ = remove_any(&staging);

    copy_any(from, &staging).inspect_err(|_| {
        let _ = remove_any(&staging);
    })?;

    let (want, _) = measure(from);
    let (got, _) = measure(&staging);
    if want != got {
        let _ = remove_any(&staging);
        return Err(AppError::new("migrate.verifyFailed"));
    }

    std::fs::rename(&staging, to).map_err(|e| {
        let _ = remove_any(&staging);
        AppError::because("migrate.writeFailed", e)
    })?;

    remove_any(from)
}

fn copy_any(from: &Path, to: &Path) -> Result<(), AppError> {
    let meta = std::fs::metadata(from).map_err(|e| AppError::because("migrate.readFailed", e))?;
    if meta.is_file() {
        std::fs::copy(from, to).map_err(|e| AppError::because("migrate.writeFailed", e))?;
        return Ok(());
    }

    std::fs::create_dir_all(to).map_err(|e| AppError::because("migrate.writeFailed", e))?;
    let entries = std::fs::read_dir(from).map_err(|e| AppError::because("migrate.readFailed", e))?;
    for entry in entries.flatten() {
        copy_any(&entry.path(), &to.join(entry.file_name()))?;
    }
    Ok(())
}

fn remove_any(path: &Path) -> Result<(), AppError> {
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return Ok(());
    };
    let result = if meta.is_dir() {
        std::fs::remove_dir_all(path)
    } else {
        std::fs::remove_file(path)
    };
    result.map_err(|e| AppError::because("migrate.removeFailed", e))
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CleanupOutcome {
    pub removed: Vec<String>,
    pub freed_bytes: f64,
    pub failed: Vec<Failed>,
    /// Сколько элементов отклонено, потому что дубликатами не являются.
    pub refused: u32,
}

/// Убирает из сборки то, что уже лежит в общей папке.
///
/// **Вердикт пересчитывается здесь заново**, а не берётся из списка,
/// пришедшего с фронта. Это единственная защита от того, чтобы удалить
/// файл, который дубликатом не является: между показом перечня и нажатием
/// кнопки содержимое могло смениться, да и доверять входным данным
/// в операции удаления нельзя вовсе.
pub fn remove_duplicates(
    models_dir: &Path,
    shared_root: &Path,
    items: &[(String, String)],
) -> CleanupOutcome {
    let mut out = CleanupOutcome {
        removed: Vec::new(),
        freed_bytes: 0.0,
        failed: Vec::new(),
        refused: 0,
    };

    for (category, name) in items {
        let local = models_dir.join(category).join(name);
        let shared = shared_root.join(category).join(name);

        if !local.exists() || !shared.exists() {
            out.refused += 1;
            continue;
        }
        if matches!(compare(&local, &shared), SameName::Different) {
            out.refused += 1;
            continue;
        }

        let (bytes, _) = measure(&local);
        match remove_any(&local) {
            Ok(()) => {
                out.removed.push(format!("{category}/{name}"));
                out.freed_bytes += bytes as f64;
            }
            Err(e) => out.failed.push(Failed {
                category: category.clone(),
                name: name.clone(),
                reason: e.code,
            }),
        }
    }

    out
}

/// Свободно ли на целевом томе столько, сколько собираемся перенести.
///
/// На одном томе перенос — переименование, и место не нужно вовсе;
/// проверка нужна только для переезда между дисками.
/// Не сумели узнать свободное место — не мешаем: отказ на основании
/// незнания хуже, чем попытка, которая честно провалится на записи.
pub fn enough_space(shared_root: &Path, need_bytes: f64) -> bool {
    match crate::installer::free_space(&shared_root.display().to_string()) {
        Some(free) => free >= need_bytes,
        None => true,
    }
}

/// Общая папка: первый включённый корень.
pub fn first_root(settings: &crate::shared_models::SharedSettings) -> Option<PathBuf> {
    settings
        .roots
        .iter()
        .find(|r| r.enabled)
        .map(|r| PathBuf::from(&r.path))
}
