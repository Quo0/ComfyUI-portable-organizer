//! Moving models from a build into the shared folder, and cleaning up
//! duplicates.
//!
//! A module separate from `shared_models.rs` deliberately: scanning and YAML
//! generation live there, while this is the only place in the entire app where
//! we delete model files. The boundary in `CLAUDE.md` allows exactly two such
//! cases, both on the user's explicit request and with the list shown in
//! advance: the move, where the source disappears **after** verifying the copy
//! is in place, and cleaning up a duplicate that already lies in the shared
//! folder.
//!
//! Nothing is ever deleted silently.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// How many bytes from each edge are compared when deciding that two files are
/// the same.
///
/// A full hash on 20 GB models is unacceptable — that is written down in the
/// plan for the duplicate analyser. Two megabytes of reading cost milliseconds
/// and turn "one name and one size" into a near-certain match.
const EDGE: u64 = 1024 * 1024;

/// Categories whose contents belong to the build, not to the user.
///
/// ComfyUI ships `configs` with itself — `v1-inference.yaml` and the like live
/// there. Taking them away means robbing the installation. `custom_nodes` is
/// not shared at all and must never end up in the shared folder under any
/// circumstances.
const NEVER_MOVE: [&str; 2] = ["configs", "custom_nodes"];

/// What an entry whose name is already taken in the shared folder turned out
/// to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SameName {
    /// The size and the edges matched — almost certainly the same file.
    Duplicate,
    /// A directory: the total size and the file count matched. The grounds are
    /// weaker, which is why it is named differently.
    LikelyDuplicate,
    /// The sizes or the edges diverged. **Must not be deleted under any
    /// conditions:** these are different files that were unlucky with the name.
    Different,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ModelEntry {
    /// The entry's name inside the category. A file or a whole directory.
    pub name: String,
    pub is_dir: bool,
    pub size_bytes: f64,
    pub files: u32,
    /// Whether this name is taken in the shared folder, and what it turned out
    /// to be there.
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
    /// How much will move in total and how much space it takes.
    pub total_files: u32,
    pub total_bytes: f64,
}

/// The cancellation flag, shared with the abort command. The same trick as in
/// the installer's `InstallCancel`.
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
    /// Skipped because the name was taken, with a verdict for each.
    pub skipped: Vec<Skipped>,
    /// Failed, with a reason. A failure on one does not cancel the rest.
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

/// The marker file ComfyUI puts into an empty category.
///
/// Recognised by name and zero size at once: a user file with such a name is
/// possible, but not a zero-length one.
pub(crate) fn is_placeholder(path: &Path, size: u64) -> bool {
    size == 0
        && path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("put_") && n.ends_with("_here"))
            .unwrap_or(false)
}

/// The size and file count of a tree. A directory is counted whole.
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

/// Whether the files' edges are identical.
///
/// A megabyte is read from the start and from the end. Files shorter than two
/// megabytes are compared whole — that is cheaper than working out offsets.
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

/// What kind of entry lies in the shared folder under the same name.
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
            // Same size, different contents. This is exactly the case the
            // edges are read for.
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

/// Reads a build's models and compares them against the shared folder.
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

    // Only what will actually move is counted: taken names stay where they
    // are.
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

/// Progress of the move. Counted in entries rather than bytes: within one
/// volume a move is instantaneous, and a byte-based bar would jump about
/// meaninglessly.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct MigrateProgress {
    pub done: u32,
    pub total: u32,
    pub category: String,
    pub name: String,
}

/// Moves the selected models into the shared folder.
///
/// The selection arrives as "category and model" pairs rather than as category
/// names: on screen there is a toggle next to every model, and one lora out of
/// twenty may be needed by the build locally. The list is checked against a
/// fresh scan — whatever is no longer in the folder simply will not be found
/// and will not go into the move.
///
/// Taken names are not touched at all: someone else's 20 GB model is worth
/// more than a duplicate.
pub fn move_all(
    models_dir: &Path,
    shared_root: &Path,
    items: &[(String, String)],
    cancel: &MigrateCancel,
    mut on_progress: impl FnMut(MigrateProgress),
) -> MigrateOutcome {
    let scan = scan(models_dir, shared_root);
    let wanted: std::collections::HashSet<(&str, &str)> = items
        .iter()
        .map(|(category, name)| (category.as_str(), name.as_str()))
        .collect();
    let chosen: Vec<(&ModelCategory, Vec<&ModelEntry>)> = scan
        .categories
        .iter()
        .map(|c| {
            let picked = c
                .entries
                .iter()
                .filter(|e| wanted.contains(&(c.folder.as_str(), e.name.as_str())))
                .collect::<Vec<_>>();
            (c, picked)
        })
        .filter(|(_, picked)| !picked.is_empty())
        .collect();

    let total: u32 = chosen.iter().map(|(_, picked)| picked.len() as u32).sum();
    let mut out = MigrateOutcome {
        moved: Vec::new(),
        skipped: Vec::new(),
        failed: Vec::new(),
        moved_bytes: 0.0,
        cancelled: false,
    };
    let mut done = 0u32;

    for (category, picked) in chosen {
        let target_dir = shared_root.join(&category.folder);
        for entry in picked {
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
                // A failure on one entry does not cancel the rest: there are
                // dozens of categories, and abandoning everything because of
                // one locked file would be silly.
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

/// Moves a single entry.
///
/// Within one volume it is a rename: instant and without risk. Across volumes
/// it is first a copy under a temporary name, then verification, then putting
/// it in place, and **only then** deleting the source: until the copy has been
/// verified, the original has to stay in our hands.
fn move_entry(from: &Path, to: &Path) -> Result<(), AppError> {
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AppError::because("migrate.writeFailed", e))?;
    }

    if std::fs::rename(from, to).is_ok() {
        return Ok(());
    }

    // The same trick as in the installer: anything unfinished always carries a
    // name that makes it visible and safe to remove.
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
    /// How many entries were refused because they are not duplicates.
    pub refused: u32,
}

/// Removes from a build what already lies in the shared folder.
///
/// **The verdict is recomputed here from scratch** rather than taken from the
/// list that arrived from the frontend. This is the only protection against
/// deleting a file that is not a duplicate: the contents could have changed
/// between showing the list and pressing the button, and input data must not
/// be trusted in a delete operation at all.
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

/// Whether the target volume has as much free space as we are about to move.
///
/// Within one volume a move is a rename and no space is needed at all; the
/// check only matters for a move between drives. If the free space could not
/// be determined, we do not stand in the way: refusing on the grounds of not
/// knowing is worse than an attempt that fails honestly on the write.
pub fn enough_space(shared_root: &Path, need_bytes: f64) -> bool {
    match crate::installer::free_space(&shared_root.display().to_string()) {
        Some(free) => free >= need_bytes,
        None => true,
    }
}

/// The shared folder: the first enabled root.
pub fn first_root(settings: &crate::shared_models::SharedSettings) -> Option<PathBuf> {
    settings
        .roots
        .iter()
        .find(|r| r.enabled)
        .map(|r| PathBuf::from(&r.path))
}
