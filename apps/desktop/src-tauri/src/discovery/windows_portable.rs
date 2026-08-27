//! A ComfyUI portable build on Windows: `ComfyUI_windows_portable`.
//!
//! Verified against a real 0.30.2 installation. The markers a folder is
//! recognised by are chosen so that a half-extracted tree does not pass them:
//! `main.py` is far from the first thing to appear in the archive.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::AppError;

use super::{FoundProfile, InstanceDiscovery, Probe};

/// The bundled interpreter. Its presence is what tells a portable build apart
/// from a repository clone, which we do not know how to launch.
const PYTHON_REL: &str = r"python_embeded\python.exe";
const MAIN_PY_REL: &str = r"ComfyUI\main.py";
const VERSION_REL: &str = r"ComfyUI\comfyui_version.py";

/// The folder with "advanced" launch options. Scanned alongside the root.
const ADVANCED_DIR: &str = "advanced";
/// Updating the build. These `.bat` files do not start a server and have no
/// place among the profiles.
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

/// The folder did not fit — and we have to say where to look instead.
///
/// Missing by one level happens constantly: above sits the archive's folder,
/// below sits `ComfyUI\`. Both look plausible, and neither fits.
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

/// The only suitable subfolder. If there are several, there is nothing to
/// suggest: the choice is the user's.
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

/// Strips the `\\?\` prefix that `canonicalize` adds on Windows.
///
/// The path is shown to the user and stored in the registry: with the prefix it
/// looks broken, and pasted back into Explorer it does not work.
fn canonical(path: &Path) -> String {
    match fs::canonicalize(path) {
        Ok(p) => dunce::simplified(&p).display().to_string(),
        Err(_) => path.display().to_string(),
    }
}

/// `__version__ = "0.30.2"` from the file the build generates.
fn read_comfy_version(file: &Path) -> Option<String> {
    let text = fs::read_to_string(file).ok()?;
    text.lines()
        .find_map(|line| line.split_once("__version__"))
        .and_then(|(_, rest)| rest.split('"').nth(1))
        .map(str::to_string)
}

/// `python.exe --version` → `Python 3.12.10`.
///
/// We read both streams: before 3.4 the version was printed to stderr, and
/// running into an ancient interpreter in someone else's build is entirely
/// possible.
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

/// `.bat` files in the root and in `advanced\`. The `update\` folder is not
/// scanned at all.
fn scan_profiles(root: &Path) -> Vec<FoundProfile> {
    let mut profiles = Vec::new();
    collect_bats(root, root, false, &mut profiles);
    collect_bats(&root.join(ADVANCED_DIR), root, true, &mut profiles);
    // Directory traversal order depends on the file system, and the list is
    // shown to the user — we sort it so that it does not jump around.
    profiles.sort_by(|a, b| a.id.cmp(&b.id));
    profiles
}

fn collect_bats(dir: &Path, root: &Path, advanced: bool, out: &mut Vec<FoundProfile>) {
    debug_assert!(!dir.ends_with(UPDATE_DIR), "the update folder is not scanned");

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
