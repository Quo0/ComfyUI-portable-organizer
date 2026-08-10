//! Портабл-сборка ComfyUI под Windows: `ComfyUI_windows_portable`.
//!
//! Проверено на реальной установке 0.30.2. Признаки, по которым папка
//! опознаётся, выбраны так, чтобы полураспакованное дерево их не прошло:
//! `main.py` появляется в архиве далеко не первым.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::AppError;

use super::{FoundProfile, InstanceDiscovery, Probe};

/// Встроенный интерпретатор. Его наличие отличает портабл-сборку
/// от клона репозитория, который мы запускать не умеем.
const PYTHON_REL: &str = r"python_embeded\python.exe";
const MAIN_PY_REL: &str = r"ComfyUI\main.py";
const VERSION_REL: &str = r"ComfyUI\comfyui_version.py";

/// Папка с вариантами запуска «для продвинутых». Сканируется наравне с корнем.
const ADVANCED_DIR: &str = "advanced";
/// Обновление сборки. Эти `.bat` не запускают сервер, и в профилях им не место.
const UPDATE_DIR: &str = "update";

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub struct WindowsPortable;

impl InstanceDiscovery for WindowsPortable {
    fn probe(&self, path: &Path) -> Result<Probe, AppError> {
        if !path.is_dir() {
            return Err(AppError::with(
                "discovery.notADirectory",
                "path",
                path.display(),
            ));
        }

        let python = path.join(PYTHON_REL);
        let main_py = path.join(MAIN_PY_REL);

        if !python.is_file() || !main_py.is_file() {
            return Err(mismatch(path, python.is_file(), main_py.is_file()));
        }

        Ok(Probe {
            path: canonical(path),
            comfy_version: read_comfy_version(&path.join(VERSION_REL)),
            python_version: read_python_version(&python),
            profiles: scan_profiles(path),
        })
    }
}

/// Папка не подошла — и надо сказать, куда смотреть.
///
/// Промах на уровень встречается постоянно: сверху лежит папка архива,
/// снизу — `ComfyUI\`. Обе выглядят правдоподобно, и обе не подходят.
fn mismatch(path: &Path, has_python: bool, has_main: bool) -> AppError {
    if let Some(child) = valid_child(path) {
        return AppError::with("discovery.tryChild", "suggested", canonical(&child));
    }
    if let Some(parent) = path.parent().filter(|p| is_valid(p)) {
        return AppError::with("discovery.tryParent", "suggested", canonical(parent));
    }

    match (has_python, has_main) {
        (false, true) => AppError::with("discovery.noPython", "expected", PYTHON_REL),
        (true, false) => AppError::with("discovery.noMainPy", "expected", MAIN_PY_REL),
        _ => AppError::with("discovery.notComfy", "path", path.display()),
    }
}

fn is_valid(path: &Path) -> bool {
    path.join(PYTHON_REL).is_file() && path.join(MAIN_PY_REL).is_file()
}

/// Единственная подходящая подпапка. Если их несколько, подсказывать нечего:
/// выбор за пользователем.
fn valid_child(path: &Path) -> Option<PathBuf> {
    let mut found = None;
    for entry in fs::read_dir(path).ok()?.flatten() {
        let child = entry.path();
        if !child.is_dir() || !is_valid(&child) {
            continue;
        }
        if found.is_some() {
            return None;
        }
        found = Some(child);
    }
    found
}

/// Убирает `\\?\`, который добавляет `canonicalize` на Windows.
///
/// Путь показывается пользователю и хранится в реестре: с префиксом он
/// выглядит поломанным, а вставленный обратно в проводник — не работает.
fn canonical(path: &Path) -> String {
    match fs::canonicalize(path) {
        Ok(p) => dunce::simplified(&p).display().to_string(),
        Err(_) => path.display().to_string(),
    }
}

/// `__version__ = "0.30.2"` из сгенерированного сборкой файла.
fn read_comfy_version(file: &Path) -> Option<String> {
    let text = fs::read_to_string(file).ok()?;
    text.lines()
        .find_map(|line| line.split_once("__version__"))
        .and_then(|(_, rest)| rest.split('"').nth(1))
        .map(str::to_string)
}

/// `python.exe --version` → `Python 3.12.10`.
///
/// Читаем оба потока: до 3.4 версия печаталась в stderr, и встретить
/// древний интерпретатор в чужой сборке вполне возможно.
fn read_python_version(python: &Path) -> Option<String> {
    let mut cmd = Command::new(python);
    cmd.arg("--version");

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let out = cmd.output().ok()?;
    let text = if out.stdout.is_empty() { out.stderr } else { out.stdout };
    let text = String::from_utf8_lossy(&text).trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text.trim_start_matches("Python").trim().to_string())
    }
}

/// `.bat` в корне и в `advanced\`. Папка `update\` не сканируется вовсе.
fn scan_profiles(root: &Path) -> Vec<FoundProfile> {
    let mut profiles = Vec::new();
    collect_bats(root, root, false, &mut profiles);
    collect_bats(&root.join(ADVANCED_DIR), root, true, &mut profiles);
    // Порядок обхода каталога зависит от файловой системы, а список
    // показывается пользователю — сортируем, чтобы он не прыгал.
    profiles.sort_by(|a, b| a.id.cmp(&b.id));
    profiles
}

fn collect_bats(dir: &Path, root: &Path, advanced: bool, out: &mut Vec<FoundProfile>) {
    debug_assert!(!dir.ends_with(UPDATE_DIR), "папка обновления не сканируется");

    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let file = entry.path();
        if !file.is_file() {
            continue;
        }
        if !file
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("bat"))
        {
            continue;
        }
        let Some(stem) = file.file_stem().and_then(|s| s.to_str()) else { continue };
        let Ok(rel) = file.strip_prefix(root) else { continue };

        out.push(FoundProfile {
            id: rel.display().to_string(),
            name: stem.to_string(),
            advanced,
        });
    }
}
