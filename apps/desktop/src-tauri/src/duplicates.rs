//! Отчёт о дубликатах моделей по всем сборкам сразу.
//!
//! **Только отчёт.** Ни одного действия над файлами: не удаляет,
//! не переносит, не делает ссылок. Это записано в плане отдельной строкой
//! и нарушать нельзя — уборка дублей уже существует своей командой,
//! начинается пользователем на своём экране и видит перечень заранее.
//!
//! Смысл отчёта — показать цену зоопарка: один чекпоинт весит от двух
//! до двадцати гигабайт, и при пяти установках счёт идёт на сотни.
//!
//! Полный хэш не считаем, как и везде в проекте: на файлах такого размера
//! он неприемлем. Совпадение имени и размера — основание для разговора,
//! а не приговор, и об этом сказано прямо в интерфейсе.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// Одна копия модели.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Copy_ {
    /// Имя сборки либо общей папки. Пользователь думает местами,
    /// а не идентификаторами.
    pub source: String,
    pub path: String,
    pub size_bytes: f64,
}

/// Модель, встречающаяся больше чем в одном месте.
///
/// Группируем по паре «категория и имя», а не по одному имени: один
/// и тот же файл под `loras` и под `checkpoints` — это разные роли,
/// и сводить их в одну строку значило бы предлагать выбор, которого нет.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DupGroup {
    /// Имя файла или каталога модели. Не переводится.
    pub name: String,
    pub category: String,
    pub copies: Vec<Copy_>,
    /// Сколько занято сверх одной копии. У разных размеров смысла не имеет
    /// и потому равно нулю.
    pub wasted_bytes: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DuplicatesReport {
    /// Совпали и имя, и размер. Почти наверняка одно и то же.
    pub duplicates: Vec<DupGroup>,
    /// Одно имя, разные размеры. **Это не дубликаты** — совпадение имени
    /// содержимого не доказывает, и в сумму потерь они не входят.
    pub name_clashes: Vec<DupGroup>,
    pub wasted_bytes: f64,
    /// Папки, до которых не добрались: сборка недоступна, папки моделей
    /// нет, читать не дали. Молчать о них нельзя — отчёт выглядел бы полным.
    pub skipped: Vec<String>,
    pub scanned_places: u32,
    pub cancelled: bool,
}

/// Флаг отмены. Тот же приём, что у `MigrateCancel` и `InstallCancel`.
#[derive(Default, Clone)]
pub struct ScanCancel(Arc<AtomicBool>);

impl ScanCancel {
    pub fn reset(&self) {
        self.0.store(false, Ordering::Relaxed);
    }
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Где искать. Имя — то, что увидит пользователь.
pub struct Place {
    pub name: String,
    pub models_dir: std::path::PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct DupProgress {
    pub done: u32,
    pub total: u32,
    /// Что обходим прямо сейчас. Молчаливая пауза читается как зависание.
    pub place: String,
}

/// Строит отчёт.
///
/// Единица обхода — запись верхнего уровня категории, ровно как у переноса:
/// `RMBG-2.0` хранится каталогом со снимком HuggingFace, и разбирать такое
/// по файлам значит сравнивать `config.json` с `config.json`.
pub fn scan(
    places: &[Place],
    cancel: &ScanCancel,
    on_progress: impl Fn(DupProgress),
) -> DuplicatesReport {
    let mut report = DuplicatesReport::default();
    // Ключ — категория и имя в нижнем регистре: Windows не различает
    // регистр, и две копии одного файла легко пишутся по-разному.
    let mut found: HashMap<String, Bucket> = HashMap::new();
    let total = places.len() as u32;

    for (index, place) in places.iter().enumerate() {
        if cancel.is_cancelled() {
            report.cancelled = true;
            break;
        }
        on_progress(DupProgress {
            done: index as u32,
            total,
            place: place.name.clone(),
        });

        if !place.models_dir.is_dir() {
            report.skipped.push(place.name.clone());
            continue;
        }
        report.scanned_places += 1;
        collect(place, &mut found, &mut report.skipped);
    }

    on_progress(DupProgress { done: total, total, place: String::new() });

    for bucket in found.into_values() {
        if bucket.copies.len() < 2 {
            continue;
        }
        let first = bucket.copies[0].size_bytes;
        let same_size = bucket.copies.iter().all(|c| c.size_bytes == first);

        if same_size {
            let wasted = first * (bucket.copies.len() - 1) as f64;
            report.wasted_bytes += wasted;
            report.duplicates.push(DupGroup {
                name: bucket.name,
                category: bucket.category,
                copies: bucket.copies,
                wasted_bytes: wasted,
            });
        } else {
            // Разные размеры при одном имени — предупреждение, а не находка.
            // В сумму потерь такое не входит: неизвестно, что удалять,
            // да и удалять здесь нечего вовсе.
            report.name_clashes.push(DupGroup {
                name: bucket.name,
                category: bucket.category,
                copies: bucket.copies,
                wasted_bytes: 0.0,
            });
        }
    }

    // Самое дорогое сверху: с него пользователь и начнёт.
    report
        .duplicates
        .sort_by(|a, b| b.wasted_bytes.total_cmp(&a.wasted_bytes));
    report.name_clashes.sort_by(|a, b| a.name.cmp(&b.name));
    report
}

/// Накопитель по одной паре «категория и имя».
struct Bucket {
    name: String,
    category: String,
    copies: Vec<Copy_>,
}

/// Обходит одну папку моделей: категории верхнего уровня и записи в них.
fn collect(place: &Place, found: &mut HashMap<String, Bucket>, skipped: &mut Vec<String>) {
    let Ok(categories) = std::fs::read_dir(&place.models_dir) else {
        skipped.push(place.name.clone());
        return;
    };

    for category in categories.flatten() {
        let Ok(kind) = category.file_type() else { continue };
        if !kind.is_dir() {
            continue;
        }
        let folder = category.file_name().to_string_lossy().to_string();
        // `custom_nodes` не шарится и моделью не является; `configs`
        // поставляется вместе со сборкой и совпадает у всех по определению —
        // показывать его в отчёте о дублях значит забить отчёт шумом.
        if folder == "custom_nodes" || folder == "configs" {
            continue;
        }

        let Ok(entries) = std::fs::read_dir(category.path()) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let (size, _) = crate::migrate::measure(&path);
            // Маркеры `put_..._here` лежат в каждой пустой категории каждой
            // сборки. Формально они дубликаты, по сути — шум нулевого размера.
            if crate::migrate::is_placeholder(&path, size) {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            found
                .entry(format!("{}|{}", folder.to_lowercase(), name.to_lowercase()))
                .or_insert_with(|| Bucket {
                    name: name.clone(),
                    category: folder.clone(),
                    copies: Vec::new(),
                })
                .copies
                .push(Copy_ {
                    source: place.name.clone(),
                    path: path.display().to_string(),
                    size_bytes: size as f64,
                });
        }
    }
}
