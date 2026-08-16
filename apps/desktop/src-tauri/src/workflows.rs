//! Библиотека воркфлоу: чтение папки, манифест, разбор графа.
//!
//! Находки из исходников ComfyUI, определившие устройство, собраны
//! в `plan/workflows.md`; здесь только то, что влияет на код.
//!
//! **Библиотека — это папка с файлами.** Манифест лишь обогащает её тегами,
//! заметками и пометкой «избранное». Отсюда правило устойчивости, которое
//! проходит через весь модуль: файл без записи валиден и показывается как
//! есть; запись без файла помечается потерянной, но не удаляется молча.
//! Пользователь вправе копировать и удалять файлы через проводник, и ничего
//! от этого сломаться не должно.
//!
//! Манифест лежит **в самой библиотеке**, а не в данных приложения: она
//! обязана пережить переустановку приложения и переезд на другую машину.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// Имя манифеста внутри папки библиотеки.
pub const MANIFEST: &str = "_library.json";

/// Запись манифеста. Всё в ней необязательное: манифест дополняет файл,
/// а не описывает его.
#[derive(Debug, Clone, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct WorkflowMeta {
    pub favorite: bool,
    pub tags: Vec<String>,
    pub note: String,
    /// Миллисекунды эпохи. Дату форматирует фронт по правилам локали.
    pub added_at: Option<f64>,
    /// Из какого инстанса забран, если забирали через нас.
    pub source_instance_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub version: u32,
    /// Ключ — путь файла относительно корня библиотеки, прямыми слэшами.
    pub items: BTreeMap<String, WorkflowMeta>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self { version: 1, items: BTreeMap::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LibraryItem {
    /// Путь относительно корня библиотеки, прямыми слэшами. Он же ключ
    /// манифеста и то, что видит пользователь.
    pub path: String,
    /// Имя файла без расширения — для показа.
    pub name: String,
    pub meta: WorkflowMeta,
    /// Файла нет, а запись в манифесте есть.
    pub lost: bool,
    /// Файл не разобрался как воркфлоу: битый JSON или JSON без `nodes`.
    /// Не ошибка библиотеки — показываем и даём убрать.
    pub broken: bool,
    /// Классы нод из графа. У потерянных и битых пусто.
    pub nodes: Vec<String>,
    pub size_bytes: f64,
    pub modified_at: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LibraryScan {
    pub path: String,
    /// Папки нет или она не читается. Не ошибка: библиотеку можно задать
    /// заранее, а внешний диск — отключить.
    pub available: bool,
    pub items: Vec<LibraryItem>,
    /// Манифест не разобрался. Файлы при этом на месте и показаны —
    /// повреждение тегов не имеет права уносить сами воркфлоу.
    pub manifest_broken: bool,
}

/// Путь к манифесту.
pub fn manifest_path(root: &Path) -> PathBuf {
    root.join(MANIFEST)
}

/// Читает манифест. Битый и отсутствующий одинаково дают пустой —
/// разница только в том, что про битый надо сказать пользователю.
pub fn read_manifest(root: &Path) -> (Manifest, bool) {
    let path = manifest_path(root);
    if !path.exists() {
        return (Manifest::default(), false);
    }
    match std::fs::read_to_string(&path) {
        Ok(text) => match serde_json::from_str::<Manifest>(&text) {
            Ok(manifest) => (manifest, false),
            Err(_) => (Manifest::default(), true),
        },
        Err(_) => (Manifest::default(), true),
    }
}

pub fn write_manifest(root: &Path, manifest: &Manifest) -> Result<(), AppError> {
    let text = serde_json::to_string_pretty(manifest)
        .map_err(|e| AppError::because("workflows.manifestWriteFailed", e))?;
    std::fs::create_dir_all(root)
        .map_err(|e| AppError::because("workflows.manifestWriteFailed", e))?;
    std::fs::write(manifest_path(root), format!("{text}\n"))
        .map_err(|e| AppError::because("workflows.manifestWriteFailed", e))
}

/// Что лежит в библиотеке под тем же именем.
///
/// `None` снаружи означает «имени в библиотеке нет вовсе» — забирать можно
/// без разговоров.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum LibraryMatch {
    /// Тот же воркфлоу. Забирать нечего, он уже там.
    Same,
    /// Имя занято, а содержимое разошлось. Это **разные работы**,
    /// и молча приравнивать их одну к другой нельзя.
    Diverged,
}

/// Воркфлоу сборки вместе с вердиктом по библиотеке.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstanceWorkflow {
    pub path: String,
    pub library: Option<LibraryMatch>,
}

/// Где у сборки лежат её воркфлоу.
///
/// Отдельно от списка, а не полем в нём: у запущенной сборки список
/// приходит по HTTP и папки не касается вовсе, а показать её в проводнике
/// нужно одинаково в обоих случаях.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstanceWorkflowsDir {
    pub path: String,
    /// Папки может не быть: ComfyUI заводит её лениво, при первом
    /// сохранении. Это не ошибка — просто показывать нечего.
    pub available: bool,
}

/// Один ли это воркфлоу.
///
/// Сначала байты, потом разобранный JSON, и второй шаг обязателен:
/// ComfyUI переписывает файл при каждом сохранении — меняются отступы,
/// порядок ключей, координаты нод округляются иначе. Без сверки по графу
/// почти любой уже забранный воркфлоу объявлялся бы разошедшимся, а цена
/// такой ошибки — лишняя копия в библиотеке под именем «(2)».
///
/// Полное чтение здесь по карману: воркфлоу это килобайты, а не модели
/// по двадцать гигабайт, и хитрости со сверкой краёв тут не нужны.
pub fn same_workflow(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    match (
        serde_json::from_str::<serde_json::Value>(a),
        serde_json::from_str::<serde_json::Value>(b),
    ) {
        (Ok(x), Ok(y)) => x == y,
        // Хоть один не разобрался — сверять нечем. Пусть решает
        // пользователь: «разошлись» оставляет кнопку рабочей.
        _ => false,
    }
}

/// Классы нод графа.
///
/// `None` означает «это не воркфлоу»: либо JSON не разобрался, либо в нём
/// нет массива `nodes`. Служит и проверкой при добавлении файла в библиотеку.
pub fn node_types(json: &str) -> Option<Vec<String>> {
    let value: serde_json::Value = serde_json::from_str(json).ok()?;
    let nodes = value.get("nodes")?.as_array()?;

    // Множество, а не список: один и тот же класс встречается в графе
    // десятки раз, а для проверки совместимости важен только набор.
    let mut types = BTreeSet::new();
    for node in nodes {
        if let Some(kind) = node.get("type").and_then(|t| t.as_str()) {
            types.insert(kind.to_string());
        }
    }
    Some(types.into_iter().collect())
}

/// Достаёт граф из PNG, сгенерированного ComfyUI.
///
/// Картинка из папки `output` носит граф с собой: ComfyUI кладёт его
/// в текстовый чанк `workflow` (`PngInfo.add_text` в `nodes.py`). Рядом
/// лежит `prompt` — это API-формат, другой по структуре, и библиотеке
/// он не нужен: в неё идёт то, что открывается в редакторе.
///
/// Разбор ручной, без единой зависимости: формат чанков PNG — это длина,
/// тип, данные и CRC, и тянуть ради этого крейт-декодер изображений
/// значило бы тянуть заодно zlib и полдюжины кодеков.
///
/// `None` — граф не найден. Битый или обрезанный файл сюда же: обход
/// идёт по объявленным длинам и за пределы буфера не выходит.
pub fn workflow_from_png(bytes: &[u8]) -> Option<String> {
    const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    if bytes.len() < SIGNATURE.len() || bytes[..8] != SIGNATURE {
        return None;
    }

    let mut at = SIGNATURE.len();
    while at + 8 <= bytes.len() {
        let len = u32::from_be_bytes(bytes[at..at + 4].try_into().ok()?) as usize;
        let kind = &bytes[at + 4..at + 8];
        let start = at + 8;
        let end = start.checked_add(len)?;
        if end > bytes.len() {
            return None;
        }

        // tEXt: ключ, нулевой байт, значение. zTXt и iTXt сжаты — ComfyUI
        // ими не пользуется, и разжимать их ради гипотезы мы не будем.
        if kind == b"tEXt" {
            let data = &bytes[start..end];
            if let Some(sep) = data.iter().position(|b| *b == 0) {
                if &data[..sep] == b"workflow" {
                    return String::from_utf8(data[sep + 1..].to_vec()).ok();
                }
            }
        }
        if kind == b"IEND" {
            return None;
        }
        // Плюс четыре байта CRC.
        at = end + 4;
    }
    None
}

/// Чего не хватает в инстансе, чтобы открыть этот воркфлоу.
///
/// Порядок сохраняется от `node_types`, то есть алфавитный: список идёт
/// пользователю на глаза, и прыгать между вызовами он не должен.
pub fn missing_nodes(workflow: &[String], available: &BTreeSet<String>) -> Vec<String> {
    workflow.iter().filter(|t| !available.contains(*t)).cloned().collect()
}

/// Читает библиотеку целиком.
pub fn scan_library(root: &Path) -> LibraryScan {
    let display = root.display().to_string();

    if !root.is_dir() {
        return LibraryScan {
            path: display,
            available: false,
            items: Vec::new(),
            manifest_broken: false,
        };
    }

    let (manifest, manifest_broken) = read_manifest(root);

    let mut items: Vec<LibraryItem> = Vec::new();
    let mut seen = BTreeSet::new();

    for path in collect_json(root) {
        let rel = relative(root, &path);
        seen.insert(rel.clone());

        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let nodes = node_types(&text);
        let meta = manifest.items.get(&rel).cloned().unwrap_or_default();
        let (size_bytes, modified_at) = stat(&path);

        items.push(LibraryItem {
            name: display_name(&rel),
            path: rel,
            meta,
            lost: false,
            broken: nodes.is_none(),
            nodes: nodes.unwrap_or_default(),
            size_bytes,
            modified_at,
        });
    }

    // Записи, которым не нашлось файла. Молча выкидывать нельзя: заметку
    // и теги писал пользователь, и он вправе узнать, что файл исчез.
    for (rel, meta) in &manifest.items {
        if seen.contains(rel) {
            continue;
        }
        items.push(LibraryItem {
            name: display_name(rel),
            path: rel.clone(),
            meta: meta.clone(),
            lost: true,
            broken: false,
            nodes: Vec::new(),
            size_bytes: 0.0,
            modified_at: None,
        });
    }

    // Порядок фиксированный: `read_dir` его не обещает, а список, прыгающий
    // между открытиями экрана, читать невозможно.
    items.sort_by(|a, b| a.path.cmp(&b.path));

    LibraryScan { path: display, available: true, items, manifest_broken }
}

/// Все `.json` в дереве, кроме самого манифеста.
///
/// Вложенные папки поддерживаются: пользователь раскладывает библиотеку
/// как ему удобно, и плоский список это сломал бы.
fn collect_json(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(kind) = entry.file_type() else { continue };
            if kind.is_dir() {
                stack.push(path);
                continue;
            }
            // Посторонние файлы в списке воркфлоу не показываем: положить
            // README рядом со своими графами — обычное дело.
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if path.file_name().and_then(|n| n.to_str()) == Some(MANIFEST) {
                continue;
            }
            found.push(path);
        }
    }
    found
}

/// Путь относительно корня, прямыми слэшами.
///
/// Прямые и на Windows: это ключ манифеста, а манифест переезжает между
/// машинами вместе с папкой.
fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Имя для показа: путь без расширения.
fn display_name(rel: &str) -> String {
    rel.strip_suffix(".json").unwrap_or(rel).to_string()
}

/// Имя файла из того, что набрал пользователь.
///
/// Нужно там, где имя приходит не от файловой системы, а из поля ввода:
/// у графа, вставленного текстом, своего имени нет вовсе.
///
/// `None` — имя не годится, и разбираться дальше нечего. Отсев жёсткий
/// намеренно: имя попадает в путь, и `..\..\` в нём — это запись мимо
/// библиотеки. Разделители пути отвергаются целиком, а не вырезаются:
/// молча превратить `sdxl/base` в `sdxlbase` значит сохранить не туда,
/// куда просили, и не сказать об этом.
pub fn file_name_from_input(input: &str) -> Option<String> {
    let name = input.trim();
    // Расширение снимается и возвращается своё: набранное руками «.jsn»
    // или «.json.json» — описка, а не выбор.
    let stem = name
        .strip_suffix(".json")
        .or_else(|| name.strip_suffix(".JSON"))
        .unwrap_or(name)
        .trim_end();

    if stem.is_empty() {
        return None;
    }
    // Запрещённое в именах файлов Windows плюс точка в начале и в конце:
    // первая прячет файл в проводнике, вторая молча отбрасывается самой ОС.
    const FORBIDDEN: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if stem.contains(FORBIDDEN)
        || stem.chars().any(|c| (c as u32) < 0x20)
        || stem.starts_with('.')
        || stem.ends_with('.')
    {
        return None;
    }

    Some(format!("{stem}.json"))
}

/// Размер и время правки. `f64` — по тому же ограничению specta на целые.
fn stat(path: &Path) -> (f64, Option<f64>) {
    let Ok(meta) = std::fs::metadata(path) else {
        return (0.0, None);
    };
    let modified = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as f64);
    (meta.len() as f64, modified)
}
