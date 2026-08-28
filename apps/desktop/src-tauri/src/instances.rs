//! The instance registry.
//!
//! The app knows about instances but does not own them: registering remembers
//! the path and the metadata, the folder on disk stays untouched. Removing
//! from the registry erases nothing either — that is this section's main
//! promise.
//!
//! Storage is `instances.json` in `app_data_dir()` via `tauri-plugin-store`,
//! next to `settings.json`. No hardcoded path: the installer cleans the folder
//! strictly by bundle identifier, and a literal would drift away from it.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri_plugin_store::StoreExt;

use crate::discovery::{FoundProfile, InstanceDiscovery, Probe};
use crate::error::AppError;
use crate::shared_models::InstanceShared;

const STORE_FILE: &str = "instances.json";
const KEY_LIST: &str = "instances";

/// The default port — the same one ComfyUI itself uses.
pub const DEFAULT_PORT: u16 = 8188;

/// An instance's accent colour.
///
/// Stored either as the name of a palette token or as an `#rrggbb` value if
/// the user picked their own. The name is better and stays the default choice:
/// a token has its own value in each theme, and its legibility is verified. A
/// custom colour is the same in both themes and the user answers for it — but
/// there is no reason to forbid them a colour of their own.
///
/// A tuple struct rather than an enum: `serde` writes it transparently, so a
/// registry written before this change reads exactly as it did, and `specta`
/// exports it as the inner type itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct Accent(pub String);

impl Accent {
    pub fn named(name: &str) -> Self {
        Self(name.to_string())
    }

    /// A colour from the palette or a custom one as `#rrggbb`. Everything
    /// else is refused: this value lands in the markup as it is.
    fn valid(&self) -> bool {
        if PALETTE.contains(&self.0.as_str()) {
            return true;
        }
        let hex = self.0.strip_prefix('#').unwrap_or("");
        hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
    }
}

/// The order colours are handed out to new instances in: one after another,
/// round-robin. Also the list of names considered valid.
pub const PALETTE: [&str; 8] = [
    "teal", "indigo", "ember", "moss", "azure", "orchid", "rose", "amber",
];

/// Where the instance came from. Filled in by the install wizard in Phase 1.5;
/// folders added by hand have none of this.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstallSource {
    pub archive_path: String,
    pub archive_label: String,
    /// Epoch milliseconds. The frontend formats the date: it has to follow the
    /// chosen language, and a string from Rust cannot do that.
    pub installed_at: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub description: String,
    pub path: String,
    pub accent: Accent,
    pub preferred_port: u16,
    pub comfy_version: Option<String>,
    pub python_version: Option<String>,
    pub profiles: Vec<FoundProfile>,
    pub created_at: f64,
    pub source: Option<InstallSource>,

    /// The connection to shared model storage. `#[serde(default)]` is not
    /// decoration: a registry written before Phase 2.5 has no such field, and
    /// without the default the parse of the whole file would fail — that is,
    /// the user would lose their list of instances because a new setting
    /// appeared.
    #[serde(default)]
    pub shared: InstanceShared,

    /// Profiles assembled by the user on top of those parsed from `.bat`.
    ///
    /// Ours, not an edit of someone else's: we never touch a `.bat`, and the
    /// parse is re-read on every launch — there would be nowhere to hold an
    /// edit to it. `#[serde(default)]` for the same reason as on `shared`: a
    /// registry written before this phase has no such field.
    #[serde(default)]
    pub custom_profiles: Vec<CustomProfile>,

    /// When the build was last launched, in epoch milliseconds.
    ///
    /// `None` means not once since the app started recording this. The
    /// frontend formats the date: it has to follow the chosen language.
    #[serde(default)]
    pub last_started_at: Option<f64>,

    /// Size on disk. `f64` rather than `u64`: specta forbids exporting
    /// integers that do not fit into a JavaScript number without losing
    /// precision. Bytes up to 9 petabytes are exactly representable in f64,
    /// which is more than enough.
    pub size_bytes: Option<f64>,
    pub size_measured_at: Option<f64>,

    /// The folder is in place. Not really stored: recomputed on every read of
    /// the registry. An instance whose folder has disappeared is marked
    /// unavailable but does not vanish from the list — otherwise the user
    /// concludes the app lost their build.
    pub available: bool,
}

/// A custom launch profile.
///
/// Only the name and the arguments are stored. The interpreter, the working
/// folder and `env` are taken from the base profile **at launch time**: the
/// user may have edited the `.bat` by hand, and a remembered copy would one
/// day drift away from what is on disk.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CustomProfile {
    /// `custom:<number>`. The prefix tells a custom profile apart from a
    /// `.bat` name.
    pub id: String,
    pub name: String,
    /// The `.bat` profile everything else is taken from. If it has
    /// disappeared, the custom profile is shown as broken rather than launched
    /// at random.
    pub base_id: String,
    pub args: Vec<String>,
}

/// The metadata the user edits. A type of its own so that "rename" cannot
/// accidentally rewrite the path or the version.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstanceEdit {
    pub name: String,
    pub description: String,
    pub accent: Accent,
    pub preferred_port: u16,
}

/// The size measurements running right now. A second walk of the same tree
/// speeds nothing up while doubling the load on the disk.
#[derive(Default)]
pub struct SizeJobs(Mutex<HashSet<String>>);

impl SizeJobs {
    fn start(&self, id: &str) -> bool {
        self.0.lock().unwrap().insert(id.to_string())
    }

    fn finish(&self, id: &str) {
        self.0.lock().unwrap().remove(id);
    }
}

fn now_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0)
}

fn read_all(app: &tauri::AppHandle) -> Result<Vec<Instance>, AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("instances.loadFailed", e))?;

    // A corrupt registry file must not get in the way of startup: showing an
    // empty list is more honest than not opening at all. The file is rewritten
    // by the very first edit.
    let mut list: Vec<Instance> = store
        .get(KEY_LIST)
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    for instance in &mut list {
        instance.available = folder_ok(Path::new(&instance.path));
    }
    Ok(list)
}

fn write_all(app: &tauri::AppHandle, list: &[Instance]) -> Result<(), AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::because("instances.saveFailed", e))?;
    let value = serde_json::to_value(list)
        .map_err(|e| AppError::because("instances.saveFailed", e))?;
    store.set(KEY_LIST, value);
    store
        .save()
        .map_err(|e| AppError::because("instances.saveFailed", e))
}

fn folder_ok(path: &Path) -> bool {
    path.join(r"python_embeded\python.exe").is_file()
        && path.join(r"ComfyUI\main.py").is_file()
}

pub fn list(app: &tauri::AppHandle) -> Result<Vec<Instance>, AppError> {
    read_all(app)
}

/// Checks a folder and reports along the way whether it is already registered.
pub fn probe(
    app: &tauri::AppHandle,
    discovery: &dyn InstanceDiscovery,
    path: &str,
) -> Result<ProbeResult, AppError> {
    let probe = discovery.probe(&PathBuf::from(path))?;
    let existing = read_all(app)?
        .into_iter()
        .find(|i| same_path(&i.path, &probe.path))
        .map(|i| i.id);

    Ok(ProbeResult {
        existing_id: existing,
        suggested_name: suggest_name(&probe.path),
        suggested_port: suggest_port(&read_all(app)?),
        probe,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ProbeResult {
    pub probe: Probe,
    /// The folder is already in the registry. We do not create a second
    /// instance for it — we show the existing one.
    pub existing_id: Option<String>,
    pub suggested_name: String,
    pub suggested_port: u16,
}

/// Windows does not distinguish case in paths, and the user may pick the same
/// folder from somewhere else and get a different spelling.
fn same_path(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

fn suggest_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("ComfyUI")
        .to_string()
}

/// The first port not taken by other instances' preferences.
///
/// This is not a check for occupancy in the system: that happens at launch, in
/// Phase 2. The task here is more modest — not to suggest the same port twice.
fn suggest_port(list: &[Instance]) -> u16 {
    let taken: HashSet<u16> = list.iter().map(|i| i.preferred_port).collect();
    (DEFAULT_PORT..=u16::MAX)
        .find(|p| !taken.contains(p))
        .unwrap_or(DEFAULT_PORT)
}

fn next_accent(list: &[Instance]) -> Accent {
    Accent::named(PALETTE[list.len() % PALETTE.len()])
}

pub fn suggest_accent(app: &tauri::AppHandle) -> Result<Accent, AppError> {
    Ok(next_accent(&read_all(app)?))
}

/// Registers a folder. `source` is filled in by the install wizard — it is
/// what makes the card show which archive the instance was unpacked from;
/// those added by hand have none.
pub fn add(
    app: &tauri::AppHandle,
    probe: Probe,
    edit: InstanceEdit,
    source: Option<InstallSource>,
) -> Result<Instance, AppError> {
    let mut list = read_all(app)?;

    if let Some(existing) = list.iter().find(|i| same_path(&i.path, &probe.path)) {
        return Err(AppError::with("instances.duplicate", "name", &existing.name));
    }
    validate(&edit)?;

    let instance = Instance {
        id: new_id(&list),
        name: edit.name,
        description: edit.description,
        path: probe.path,
        accent: edit.accent,
        preferred_port: edit.preferred_port,
        comfy_version: probe.comfy_version,
        python_version: probe.python_version,
        profiles: probe.profiles,
        created_at: now_ms(),
        source,
        shared: InstanceShared::default(),
        custom_profiles: Vec::new(),
        last_started_at: None,
        size_bytes: None,
        size_measured_at: None,
        available: true,
    };

    list.push(instance.clone());
    write_all(app, &list)?;
    Ok(instance)
}

pub fn update(
    app: &tauri::AppHandle,
    id: &str,
    edit: InstanceEdit,
) -> Result<Instance, AppError> {
    validate(&edit)?;

    let mut list = read_all(app)?;
    let instance = list
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    instance.name = edit.name;
    instance.description = edit.description;
    instance.accent = edit.accent;
    // The port applies from the next launch: the current process has already
    // taken its own, and there is nothing to change it with on the fly.
    instance.preferred_port = edit.preferred_port;

    let updated = instance.clone();
    write_all(app, &list)?;
    Ok(updated)
}

/// Records the connection to shared models.
///
/// Separate from `update`: that one takes the instance edit form and would
/// overwrite the name and description with values the caller does not have.
pub fn set_shared(
    app: &tauri::AppHandle,
    id: &str,
    shared: InstanceShared,
) -> Result<(), AppError> {
    let mut list = read_all(app)?;
    let instance = list
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    instance.shared = shared;
    write_all(app, &list)
}

/// Marks that the build was launched.
///
/// Written on a successful start, not on an attempt: "last launch" in the list
/// answers the question "when did I use this", and a failed attempt gives no
/// answer to it.
///
/// A write error is swallowed at the caller's level: failing to remember the
/// date is no reason not to launch the build.
pub fn mark_started(app: &tauri::AppHandle, id: &str) -> Result<(), AppError> {
    let mut list = read_all(app)?;
    let instance = list
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    instance.last_started_at = Some(now_ms());
    write_all(app, &list)
}

/// Saves a custom profile: a new one, or over an existing one with the same
/// id.
///
/// An empty `id` means a new profile — the number is handed out here so the
/// frontend does not invent uniqueness behind the registry's back.
pub fn save_profile(
    app: &tauri::AppHandle,
    id: &str,
    mut profile: CustomProfile,
) -> Result<Instance, AppError> {
    if profile.name.trim().is_empty() {
        return Err(AppError::new("instances.emptyName"));
    }

    let mut list = read_all(app)?;
    let instance = list
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    if profile.id.is_empty() {
        let mut n = instance.custom_profiles.len() + 1;
        while instance.custom_profiles.iter().any(|p| p.id == format!("custom:{n}")) {
            n += 1;
        }
        profile.id = format!("custom:{n}");
    }

    match instance.custom_profiles.iter_mut().find(|p| p.id == profile.id) {
        Some(existing) => *existing = profile,
        None => instance.custom_profiles.push(profile),
    }

    let updated = instance.clone();
    write_all(app, &list)?;
    Ok(updated)
}

/// Deletes a custom profile. Profiles from `.bat` cannot be deleted — we are
/// not the ones who created them.
pub fn remove_profile(
    app: &tauri::AppHandle,
    id: &str,
    profile_id: &str,
) -> Result<Instance, AppError> {
    let mut list = read_all(app)?;
    let instance = list
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    instance.custom_profiles.retain(|p| p.id != profile_id);
    let updated = instance.clone();
    write_all(app, &list)?;
    Ok(updated)
}

/// Removes an instance from the registry. Never touches the folder on disk.
pub fn remove(app: &tauri::AppHandle, id: &str) -> Result<(), AppError> {
    let mut list = read_all(app)?;
    let before = list.len();
    list.retain(|i| i.id != id);
    if list.len() == before {
        return Err(AppError::with("instances.notFound", "id", id));
    }
    write_all(app, &list)
}

fn validate(edit: &InstanceEdit) -> Result<(), AppError> {
    if edit.name.trim().is_empty() {
        return Err(AppError::new("instances.emptyName"));
    }
    // The colour goes straight into the markup as the value of a CSS
    // variable, and an arbitrary string must not be let in there.
    if !edit.accent.valid() {
        return Err(AppError::with("instances.badAccent", "value", &edit.accent.0));
    }
    // Ports below 1024 require administrator rights, and zero means "any free
    // one" — it cannot serve as a preference.
    if edit.preferred_port < 1024 {
        return Err(AppError::with("instances.portRange", "min", 1024));
    }
    Ok(())
}

fn new_id(list: &[Instance]) -> String {
    let mut candidate = format!("i{}", now_ms() as u64);
    let mut suffix = 0;
    while list.iter().any(|i| i.id == candidate) {
        suffix += 1;
        candidate = format!("i{}-{suffix}", now_ms() as u64);
    }
    candidate
}

/// Measures the size of the tree and saves the result into the registry.
///
/// In the background only: 52 GB took more than five minutes to walk, and on
/// the main thread that would look like a frozen app. The result is cached so
/// the next time the screen opens it can be shown immediately.
pub fn measure_size(
    app: &tauri::AppHandle,
    jobs: &SizeJobs,
    id: &str,
) -> Result<Option<Sized_>, AppError> {
    let list = read_all(app)?;
    let instance = list
        .iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))?;

    if !instance.available {
        return Err(AppError::with("instances.missing", "path", &instance.path));
    }
    if !jobs.start(id) {
        // A walk is already running — a second one speeds nothing up.
        return Ok(None);
    }

    let bytes = dir_size(Path::new(&instance.path));
    jobs.finish(id);

    // The registry is re-read from scratch: while the walk was running, the
    // user could have renamed this instance or deleted another one.
    let mut list = read_all(app)?;
    let Some(instance) = list.iter_mut().find(|i| i.id == id) else {
        return Ok(None);
    };
    let measured = Sized_ {
        id: id.to_string(),
        bytes,
        measured_at: now_ms(),
    };
    instance.size_bytes = Some(measured.bytes);
    instance.size_measured_at = Some(measured.measured_at);
    write_all(app, &list)?;

    Ok(Some(measured))
}

/// The measurement result. The trailing underscore is there because `Sized` is
/// taken by the Rust prelude.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Sized_ {
    pub id: String,
    pub bytes: f64,
    pub measured_at: f64,
}

/// Walking the tree without recursion: with custom nodes, the nesting depth of
/// a ComfyUI install is unpredictable, and a stack overflow here would be a
/// crash of the entire app.
fn dir_size(root: &Path) -> f64 {
    let mut total: u64 = 0;
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let Ok(meta) = entry.metadata() else { continue };
            if meta.is_dir() {
                // Symbolic links are not followed: the shared models folder is
                // attached by a link, and its weight does not belong to the
                // instance.
                if !meta.is_symlink() {
                    stack.push(entry.path());
                }
            } else {
                total = total.saturating_add(meta.len());
            }
        }
    }
    total as f64
}
