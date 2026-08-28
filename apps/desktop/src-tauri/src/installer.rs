//! The install wizard: parsing the archive, extracting, cloning the tree.
//!
//! The app does not download the archive itself — the user picks the source.
//! Nor does it update an existing instance in place: the wizard unpacks a new
//! one alongside, and the old builds stay untouched.
//!
//! The decoder decision was made by measurement, not by feel: `sevenz-rust2`
//! is three times slower than 7-Zip on a real archive (238 s against 81), and
//! that has been accepted. The details and the numbers are in
//! `plan/installer.md`.

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

/// Until the extraction is finished, the folder is named like this. Validating
/// an instance means the presence of `python_embeded\python.exe` and
/// `ComfyUI\main.py`, and a half-unpacked tree would pass it. Without the
/// temporary name, an interrupted install would leave behind a broken instance
/// that the app considers working.
const PARTIAL_SUFFIX: &str = ".cpo-partial";

/// The margin on top of the archive header: the file system spends space on
/// directory records, and tens of thousands of small files get rounded up to
/// the cluster size.
const SPACE_MARGIN: f64 = 1.1;

/// The threshold for the long-path warning. Beyond it, ordinary programs —
/// ComfyUI itself, pip, python — start stumbling over MAX_PATH, even if our
/// extraction copes thanks to verbatim paths.
const MAX_PATH: usize = 260;

/// More than ten events a second the interface will not show anyway, while on
/// 56 thousand files they load the IPC noticeably.
const PROGRESS_INTERVAL_MS: u128 = 100;

// ------------------------------------------------------------- the model

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveInfo {
    pub path: String,
    /// The file name. Shown on the instance card as the source.
    pub label: String,
    pub size_bytes: f64,
    /// Epoch milliseconds. Together with the size it recognises a swapped
    /// file.
    pub mtime: f64,
    pub files: u32,
    pub folders: u32,
    pub total_uncompressed: f64,
    /// The archive's single root folder. Its name is set by the user, so it is
    /// stripped from the paths — which also takes 25 characters off the
    /// length.
    pub single_root: Option<String>,
    /// The longest path inside the archive after the root is stripped.
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

/// What is wrong with a target. The two are separated deliberately: with a
/// warning the install can start, with an error it cannot.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TargetCheck {
    pub path: String,
    pub errors: Vec<AppError>,
    pub warnings: Vec<AppError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum InstallPhase {
    /// The checks, opening the archive, unfolding the 768 MB LZMA2 dictionary.
    /// It takes seconds, and would look like silence if left unannounced.
    Preparing,
    /// Cleaning up after an interrupted attempt. Separate from `Preparing`
    /// because it is the only preparation that can drag on: tens of thousands
    /// of files, plus retries around the ones the antivirus is holding.
    Cleaning,
    /// Extracting the archive into the first target.
    Extracting,
    /// Copying the finished tree into the remaining targets.
    Copying,
    /// Registration in the registry: re-checking every target and running
    /// `python --version`. Seconds again, and not free again.
    Registering,
}

impl InstallPhase {
    /// The phases with no fraction done: there is nothing to show beyond the
    /// fact that work is happening.
    pub fn is_indeterminate(self) -> bool {
        matches!(self, Self::Preparing | Self::Cleaning | Self::Registering)
    }
}

#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
#[serde(rename_all = "camelCase")]
pub struct InstallProgress {
    pub phase: InstallPhase,
    /// The target's number, starting from one, and how many there are.
    pub target: u32,
    pub targets: u32,
    pub target_name: String,
    /// The current file's path inside the instance. Not translated.
    pub current: String,

    /// Files, not bytes — that is the honest measure of progress on this
    /// archive.
    ///
    /// The tail of the build is `site-packages` with tens of thousands of
    /// files a couple of kilobytes each. At the 98% mark of the bytes, less
    /// than half the files are done, and the bar stands exactly where the
    /// longest part of the work is happening: the time goes not into bytes but
    /// into creating files and having each one checked by the antivirus.
    /// Measured on an interrupted run: 27,906 files out of 61,895 at 4.0 GB
    /// out of 4.1 GB.
    pub done_files: u32,
    pub total_files: u32,

    /// The bytes remain, but as a caption alongside now, not as the bar.
    pub done_bytes: f64,
    pub total_bytes: f64,
}

impl InstallProgress {
    /// A phase event with no counters. The zeros here mean "there is nothing
    /// to count", not "nothing has been done": the frontend works that out
    /// from `phase` and draws a running bar instead of a fraction.
    pub fn stage(phase: InstallPhase, target: u32, targets: u32, name: &str) -> Self {
        Self {
            phase,
            target,
            targets,
            target_name: name.to_string(),
            current: String::new(),
            done_files: 0,
            total_files: 0,
            done_bytes: 0.0,
            total_bytes: 0.0,
        }
    }
}

/// Cancelling the wizard. Checked between files: there is no point
/// interrupting the extraction of a single file mid-stream, the largest one in
/// the archive is a handful of megabytes. The flag sits behind an `Arc`
/// because the work goes to a separate thread while the cancel command stays
/// in the app state: both have to look at the same bit.
#[derive(Default, Clone)]
pub struct InstallCancel(std::sync::Arc<AtomicBool>);

impl InstallCancel {
    pub fn request(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    pub fn reset(&self) {
        self.0.store(false, Ordering::Relaxed);
    }

    /// A copy sharing the same flag. For handing to the worker thread.
    pub fn share(&self) -> Self {
        self.clone()
    }

    fn requested(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Whether an install is running right now. Two at once would unpack one on
/// top of the other and fight over the disk.
#[derive(Default)]
pub struct InstallLock(Mutex<bool>);

// ----------------------------------------------------------- the parsing

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

// ----------------------------------------------------------- the checks

/// Checks the targets before the work starts: space, folder emptiness, path
/// length.
///
/// Errors and warnings are separated: a long path does not break the install
/// thanks to verbatim paths, but it breaks everything that runs afterwards.
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

        // A non-empty folder is almost certainly someone else's data. We will
        // not wipe it, and we will not unpack over it either: the result would
        // be a mixture.
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

        // The same path twice — the extraction and the copying would fight
        // over the same files.
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

    // Free space is counted per volume, not per folder: two targets on one
    // drive need twice as much.
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
            // Gigabytes rather than bytes: a human reads this message. The
            // decimal separator stays a dot here — this is the one place where
            // a number does not go through the locale, and dragging formatting
            // into the backend for its sake is not worth it.
            check.errors.push(AppError::with(
                "installer.noSpace",
                "needed",
                format!("{:.1}", needed * same_volume / 1024f64.powi(3)),
            ));
        }
    }

    checks
}

/// The volume root: `D:\`. The destination folder may not exist yet, so we
/// take the start of the path rather than the folder itself.
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
pub(crate) fn free_space(root: &str) -> Option<f64> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;

    let wide: Vec<u16> = std::ffi::OsStr::new(root)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut available: u64 = 0;
    // SAFETY: the string is nul-terminated, and the pointers outlive the call.
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
pub(crate) fn free_space(_root: &str) -> Option<f64> {
    None
}

// ------------------------------------------------------------- the work

/// Extracts the archive into the first target and copies the tree into the
/// rest.
///
/// We extract once and copy afterwards: decompression is CPU-bound and costs
/// four times as much as copying a finished tree. With two or three targets
/// that saves minutes.
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
    // A previous attempt may have been cut short — we start from a clean
    // place. The cleanup here can take minutes, so we announce it separately.
    if partial.exists() {
        report(InstallProgress::stage(
            InstallPhase::Cleaning,
            1,
            targets,
            &target.name,
        ));
    }
    remove_tree(&partial)?;

    // Opening the archive and unfolding the dictionary — a few more seconds of
    // silence.
    report(InstallProgress::stage(
        InstallPhase::Preparing,
        1,
        targets,
        &target.name,
    ));
    let outcome = extract_into(info, &partial, target, targets, cancel, report);

    if outcome.is_err() || cancel.requested() {
        // A cancellation and a crash both remove the temporary folder: a
        // broken tree that would pass instance validation has no business
        // being here.
        //
        // The cleanup is announced before it starts. Fifty thousand files take
        // a minute or longer to delete, and without this event the screen
        // stands still on the last progress frame — that is, it looks frozen
        // right after "Cancel" is pressed, and the button gets pressed again.
        report(InstallProgress::stage(
            InstallPhase::Cleaning,
            1,
            targets,
            &target.name,
        ));
        let _ = remove_tree(&partial);
        return outcome.and(Err(AppError::new("installer.cancelled")));
    }

    fs::create_dir_all(dest.parent().unwrap_or(dest))
        .map_err(|e| AppError::because("installer.writeFailed", e))?;
    // Renaming over an existing empty folder works — verified by
    // `examples/check_rename.rs` on our own Rust and Windows pairing. There is
    // no need to remove the folder beforehand.
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
    let mut files = 0u32;
    let mut last = Instant::now();
    // The archive entries come grouped by directory, so the parent is almost
    // always the same as the previous file's. Without this memory,
    // create_dir_all would be called fifty-six thousand times, walking every
    // component of the path each time.
    let mut last_parent: Option<PathBuf> = None;

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
                last_parent = Some(out);
                return Ok(true);
            }
            if let Some(parent) = out.parent() {
                if last_parent.as_deref() != Some(parent) {
                    fs::create_dir_all(parent)?;
                    last_parent = Some(parent.to_path_buf());
                }
            }

            let mut file = File::create(&out)?;
            done += io::copy(stream, &mut file)? as f64;
            files += 1;

            if last.elapsed().as_millis() >= PROGRESS_INTERVAL_MS {
                last = Instant::now();
                report(InstallProgress {
                    phase: InstallPhase::Extracting,
                    target: 1,
                    targets,
                    target_name: target.name.clone(),
                    current: rel,
                    done_files: files,
                    total_files: info.files,
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
    if partial.exists() {
        report(InstallProgress::stage(
            InstallPhase::Cleaning,
            index,
            targets,
            &target.name,
        ));
    }
    remove_tree(&partial)?;

    report(InstallProgress::stage(
        InstallPhase::Preparing,
        index,
        targets,
        &target.name,
    ));
    let outcome = copy_into(from, &partial, info, target, index, targets, cancel, report);

    if outcome.is_err() || cancel.requested() {
        // The same cleanup and the same message as during extraction: a copy
        // gets cancelled just as often and stays silent just as long.
        report(InstallProgress::stage(
            InstallPhase::Cleaning,
            index,
            targets,
            &target.name,
        ));
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
    let mut files = 0u32;
    let mut last = Instant::now();
    // Walking without recursion: the depth of a python tree is unpredictable,
    // and a stack overflow here would be a crash of the entire app.
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
            files += 1;

            if last.elapsed().as_millis() >= PROGRESS_INTERVAL_MS {
                last = Instant::now();
                report(InstallProgress {
                    phase: InstallPhase::Copying,
                    target: index,
                    targets,
                    target_name: target.name.clone(),
                    current: rel.display().to_string(),
                    done_files: files,
                    total_files: info.files,
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

/// Deletes a tree, retrying.
///
/// A single call is not enough: right after extraction the antivirus is
/// holding some of the files, and the deletion returns now "directory not
/// empty", now "access denied", with the set of locked files changing from
/// attempt to attempt. Verified on a real archive — see `plan/installer.md`,
/// "What else the spike uncovered".
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

/// A verbatim `\\?\` path, which lifts the MAX_PATH limit.
///
/// Forward slashes have to become backslashes: verbatim means "pass to the
/// kernel as is", the ordinary normalisation is switched off along with the
/// limit, and a `/` from an archive entry's name yields error 123 without a
/// hint of the reason.
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

// -------------------------------------------------- the archive history

/// It is a history that is stored, not the last path: the user keeps several
/// versions of a build and unpacks them side by side.
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
        /// The file is in place and unchanged since last time. Recomputed on
        /// every read: the archive could have been deleted or swapped.
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
    /// Returns a guard that releases the lock when it goes out of scope.
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
