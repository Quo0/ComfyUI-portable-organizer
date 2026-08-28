//! A report on duplicate models across every build at once.
//!
//! **A report and nothing else.** Not one action on files: it does not delete,
//! does not move, does not make links. This is written into the plan as a line
//! of its own and may not be broken — cleaning up duplicates already exists as
//! a command of its own, is started by the user on their own screen, and shows
//! the list in advance.
//!
//! The point of the report is to show the price of the zoo: a single
//! checkpoint weighs from two to twenty gigabytes, and with five installations
//! the count runs into the hundreds.
//!
//! No full hash is computed, as everywhere in this project: on files of this
//! size it is unacceptable. Matching name and size is grounds for a
//! conversation, not a verdict, and the interface says so outright.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// One copy of a model.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Copy_ {
    /// The name of the build or of the shared folder. The user thinks in
    /// places, not in identifiers.
    pub source: String,
    pub path: String,
    pub size_bytes: f64,
}

/// A model occurring in more than one place.
///
/// Grouped by the pair "category and name" rather than by name alone: one and
/// the same file under `loras` and under `checkpoints` means two different
/// roles, and merging them into one row would be offering a choice that does
/// not exist.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DupGroup {
    /// The name of the model's file or directory. Not translated.
    pub name: String,
    pub category: String,
    pub copies: Vec<Copy_>,
    /// How much is taken up beyond a single copy. Meaningless when the sizes
    /// differ, and therefore zero in that case.
    pub wasted_bytes: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DuplicatesReport {
    /// Both the name and the size matched. Almost certainly the same thing.
    pub duplicates: Vec<DupGroup>,
    /// One name, different sizes. **These are not duplicates** — a matching
    /// name proves nothing about the contents, and they are not counted into
    /// the wasted total.
    pub name_clashes: Vec<DupGroup>,
    pub wasted_bytes: f64,
    /// Folders we never reached: the build is unavailable, there is no models
    /// folder, reading was refused. Staying silent about them is not allowed —
    /// the report would look complete.
    pub skipped: Vec<String>,
    pub scanned_places: u32,
    pub cancelled: bool,
}

/// The cancellation flag. The same trick as in `MigrateCancel` and
/// `InstallCancel`.
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

/// Where to look. The name is what the user will see.
pub struct Place {
    pub name: String,
    pub models_dir: std::path::PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
#[serde(rename_all = "camelCase")]
pub struct DupProgress {
    pub done: u32,
    pub total: u32,
    /// What is being walked right now. A silent pause reads as a freeze.
    pub place: String,
}

/// Builds the report.
///
/// The unit of the walk is a top-level entry of a category, exactly as in the
/// move: `RMBG-2.0` is stored as a directory holding a HuggingFace snapshot,
/// and taking such a thing apart file by file means comparing `config.json`
/// with `config.json`.
pub fn scan(
    places: &[Place],
    cancel: &ScanCancel,
    on_progress: impl Fn(DupProgress),
) -> DuplicatesReport {
    let mut report = DuplicatesReport::default();
    // The key is the category and the name in lower case: Windows does not
    // distinguish case, and two copies of one file are easily spelled
    // differently.
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
            // Different sizes under one name are a warning, not a find. Such a
            // thing is not counted into the wasted total: it is unknown what
            // to delete, and there is nothing to delete here at all.
            report.name_clashes.push(DupGroup {
                name: bucket.name,
                category: bucket.category,
                copies: bucket.copies,
                wasted_bytes: 0.0,
            });
        }
    }

    // The most expensive on top: that is where the user will start.
    report
        .duplicates
        .sort_by(|a, b| b.wasted_bytes.total_cmp(&a.wasted_bytes));
    report.name_clashes.sort_by(|a, b| a.name.cmp(&b.name));
    report
}

/// The accumulator for one "category and name" pair.
struct Bucket {
    name: String,
    category: String,
    copies: Vec<Copy_>,
}

/// Walks one models folder: the top-level categories and the entries in them.
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
        // `custom_nodes` is not shared and is not a model; `configs` ships
        // with the build and matches everywhere by definition — showing it in
        // a duplicates report means burying the report in noise.
        if folder == "custom_nodes" || folder == "configs" {
            continue;
        }

        let Ok(entries) = std::fs::read_dir(category.path()) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let (size, _) = crate::migrate::measure(&path);
            // The `put_..._here` markers lie in every empty category of every
            // build. Formally they are duplicates; in substance they are noise
            // of zero size.
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
