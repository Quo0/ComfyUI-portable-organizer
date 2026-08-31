//! The workflow library: reading the folder, the manifest, parsing the graph.
//!
//! Only the findings from the ComfyUI sources that affect the code are written
//! down here.
//!
//! **The library is a folder with files.** The manifest merely enriches it
//! with tags, notes and a "favourite" mark. Hence the robustness rule running
//! through the whole module: a file with no record is valid and shown as it
//! is; a record with no file is marked lost but never deleted silently. The
//! user is entitled to copy and delete files through Explorer, and nothing
//! should break because of it.
//!
//! The manifest lies **inside the library itself**, not in the app's data: the
//! library has to survive a reinstall of the app and a move to another machine.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// The manifest's name inside the library folder.
pub const MANIFEST: &str = "_library.json";

/// A manifest record. Everything in it is optional: the manifest supplements a
/// file rather than describing it.
#[derive(Debug, Clone, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct WorkflowMeta {
    pub favorite: bool,
    pub tags: Vec<String>,
    pub note: String,
    /// Epoch milliseconds. The frontend formats the date by locale rules.
    pub added_at: Option<f64>,
    /// Which instance it was taken from, if it was taken through us.
    pub source_instance_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub version: u32,
    /// The key is the file's path relative to the library root, with forward
    /// slashes.
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
    /// The path relative to the library root, with forward slashes. Also the
    /// manifest key and what the user sees.
    pub path: String,
    /// The file name without extension — for display.
    pub name: String,
    pub meta: WorkflowMeta,
    /// There is no file, but there is a manifest record.
    pub lost: bool,
    /// The file did not parse as a workflow: broken JSON, or JSON with no
    /// `nodes`. Not a library error — we show it and let it be removed.
    pub broken: bool,
    /// The graph's node classes. Empty for lost and broken ones.
    pub nodes: Vec<String>,
    pub size_bytes: f64,
    pub modified_at: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LibraryScan {
    pub path: String,
    /// The folder is absent or will not be read. Not an error: the library can
    /// be set ahead of time, and an external drive can be unplugged.
    pub available: bool,
    pub items: Vec<LibraryItem>,
    /// The manifest did not parse. The files are in place and shown meanwhile
    /// — damage to the tags has no right to carry off the workflows
    /// themselves.
    pub manifest_broken: bool,
}

/// The path to the manifest.
pub fn manifest_path(root: &Path) -> PathBuf {
    root.join(MANIFEST)
}

/// Reads the manifest. A broken one and a missing one alike yield an empty
/// manifest — the only difference is that the user has to be told about the
/// broken one.
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

/// What lies in the library under the same name.
///
/// `None` on the outside means "the name is not in the library at all" — it
/// can be taken without further conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum LibraryMatch {
    /// The same workflow. There is nothing to take, it is already there.
    Same,
    /// The name is taken and the contents diverged. These are **different
    /// pieces of work**, and equating one with the other silently is not
    /// allowed.
    Diverged,
}

/// A build's workflow together with the verdict against the library.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstanceWorkflow {
    pub path: String,
    pub library: Option<LibraryMatch>,
}

/// Where a build keeps its workflows.
///
/// Separate from the list rather than a field in it: for a running build the
/// list arrives over HTTP and does not touch the folder at all, yet showing
/// that folder in Explorer has to work the same in both cases.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstanceWorkflowsDir {
    pub path: String,
    /// The folder may not exist: ComfyUI creates it lazily, on the first save.
    /// That is not an error — there is simply nothing to show.
    pub available: bool,
}

/// Whether these are one and the same workflow.
///
/// Bytes first, then the parsed JSON, and the second step is mandatory:
/// ComfyUI rewrites the file on every save — the indentation changes, the key
/// order changes, node coordinates get rounded differently. Without comparing
/// by graph, nearly every already-taken workflow would be declared diverged,
/// and the price of that mistake is a spare copy in the library named "(2)".
///
/// Reading in full is affordable here: a workflow is kilobytes, not a
/// twenty-gigabyte model, and the edge-comparison tricks are not needed.
pub fn same_workflow(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    match (
        serde_json::from_str::<serde_json::Value>(a),
        serde_json::from_str::<serde_json::Value>(b),
    ) {
        (Ok(x), Ok(y)) => x == y,
        // If even one failed to parse, there is nothing to compare by. Let the
        // user decide: "diverged" keeps the button working.
        _ => false,
    }
}

/// The graph's node classes.
///
/// `None` means "this is not a workflow": either the JSON failed to parse, or
/// it has no `nodes` array. It doubles as the check when adding a file to the
/// library.
pub fn node_types(json: &str) -> Option<Vec<String>> {
    let value: serde_json::Value = serde_json::from_str(json).ok()?;
    let nodes = value.get("nodes")?.as_array()?;

    // A set rather than a list: one and the same class occurs dozens of times
    // in a graph, and only the set matters for a compatibility check.
    let mut types = BTreeSet::new();
    for node in nodes {
        if let Some(kind) = node.get("type").and_then(|t| t.as_str()) {
            types.insert(kind.to_string());
        }
    }
    Some(types.into_iter().collect())
}

/// Extracts the graph from a PNG generated by ComfyUI.
///
/// An image from the `output` folder carries the graph with it: ComfyUI puts
/// it into the `workflow` text chunk (`PngInfo.add_text` in `nodes.py`). Next
/// to it lies `prompt` — that is the API format, different in structure, and
/// the library does not need it: what goes in there is what opens in the
/// editor.
///
/// The parsing is by hand, without a single dependency: the PNG chunk format
/// is a length, a type, the data and a CRC, and pulling in an image decoder
/// crate for that would mean pulling in zlib and half a dozen codecs along
/// with it.
///
/// `None` means the graph was not found. A broken or truncated file lands here
/// too: the walk follows the declared lengths and never runs past the buffer.
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

        // tEXt: a key, a zero byte, a value. zTXt and iTXt are compressed —
        // ComfyUI does not use them, and we will not decompress them on a
        // hunch.
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
        // Plus the four CRC bytes.
        at = end + 4;
    }
    None
}

/// What the instance is missing in order to open this workflow.
///
/// The order is preserved from `node_types`, i.e. alphabetical: the list goes
/// before the user's eyes, and it must not jump around between calls.
pub fn missing_nodes(workflow: &[String], available: &BTreeSet<String>) -> Vec<String> {
    workflow.iter().filter(|t| !available.contains(*t)).cloned().collect()
}

/// Reads the whole library.
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

    // Records that found no file. Throwing them away silently is not allowed:
    // the note and the tags were written by the user, and they are entitled to
    // learn that the file is gone.
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

    // The order is fixed: `read_dir` promises none, and a list that jumps
    // around between openings of the screen is impossible to read.
    items.sort_by(|a, b| a.path.cmp(&b.path));

    LibraryScan { path: display, available: true, items, manifest_broken }
}

/// Every `.json` in the tree except the manifest itself.
///
/// Nested folders are supported: the user arranges the library however suits
/// them, and a flat list would break that.
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
            // Unrelated files are not shown in the workflow list: putting a
            // README next to one's graphs is a common thing.
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

/// The path relative to the root, with forward slashes.
///
/// Forward ones on Windows too: this is the manifest key, and the manifest
/// travels between machines along with the folder.
fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// The display name: the path without the extension.
fn display_name(rel: &str) -> String {
    rel.strip_suffix(".json").unwrap_or(rel).to_string()
}

/// A file name out of what the user typed.
///
/// Needed where the name comes not from the file system but from an input
/// field: a graph pasted as text has no name of its own at all.
///
/// `None` means the name will not do and there is nothing further to work out.
/// The filtering is strict deliberately: the name lands in a path, and a
/// `..\..\` in it means writing outside the library. Path separators are
/// rejected wholesale rather than stripped: silently turning `sdxl/base` into
/// `sdxlbase` means saving somewhere other than asked and not saying so.
pub fn file_name_from_input(input: &str) -> Option<String> {
    let name = input.trim();
    // The extension is stripped and ours is put back: a hand-typed ".jsn" or
    // ".json.json" is a slip, not a choice.
    let stem = name
        .strip_suffix(".json")
        .or_else(|| name.strip_suffix(".JSON"))
        .unwrap_or(name)
        .trim_end();

    if stem.is_empty() {
        return None;
    }
    // What Windows forbids in file names, plus a dot at the start and at the
    // end: the first hides the file in Explorer, the second is silently
    // dropped by the OS itself.
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

/// Size and modification time. `f64` because of the same specta restriction on
/// integers.
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
