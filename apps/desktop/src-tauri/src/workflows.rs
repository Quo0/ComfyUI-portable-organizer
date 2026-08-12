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
