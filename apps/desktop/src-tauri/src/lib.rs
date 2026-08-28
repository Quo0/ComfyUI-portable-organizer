// Copyright (C) 2026 Andrew Blokhin
// SPDX-License-Identifier: GPL-3.0-only

//! The assembly point of the app: Tauri commands as thin wrappers over the
//! modules.
//!
//! The list of commands and events for `tauri-specta`, from which
//! `src/bindings.ts` is generated, lives here too.

// The modules are public: the examples in examples/ walk through them, and
// that is how extraction is verified against a real archive.
pub mod comfy_api;
pub mod discovery;
pub mod duplicates;
pub mod error;
pub mod installer;
pub mod migrate;
pub mod instances;
pub mod ports;
pub mod process;
pub mod profiles;
pub mod run;
pub mod settings;
pub mod shared_models;
pub mod supervise;
pub mod tray;
pub mod update;
pub mod webview;
pub mod workflows;

use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_specta::Event;

use crate::discovery::windows_portable::WindowsPortable;
use crate::error::AppError;
use crate::installer::history::ArchiveRecord;
use crate::installer::{
    ArchiveInfo, InstallCancel, InstallLock, InstallProgress, InstallTarget, TargetCheck,
};
use crate::instances::{
    Accent, Instance, InstanceEdit, ProbeResult, SizeJobs, Sized_,
};
use crate::process::{RunState, RunStatus, Runtime};
use crate::profiles::LaunchProfile;
use crate::settings::{Bootstrap, LibrarySettings, UiSettings};
use crate::migrate::ModelsScan;
use crate::workflows::{LibraryScan, WorkflowMeta};
use crate::shared_models::{
    ApplyMode, InstanceFileInfo, InstanceFileState, InstanceShared, RootScan, SharedSettings,
};

/// The settings read at startup: theme, language, rail state, plus the system
/// locale and the paths for the "About" section.
#[tauri::command]
#[specta::specta]
async fn load_bootstrap(app: tauri::AppHandle) -> Result<Bootstrap, AppError> {
    settings::load(&app)
}

#[tauri::command]
#[specta::specta]
async fn save_settings(app: tauri::AppHandle, settings: UiSettings) -> Result<(), AppError> {
    settings::save(&app, &settings)
}

// ---------------------------------------------------------- the registry

#[tauri::command]
#[specta::specta]
async fn list_instances(app: tauri::AppHandle) -> Result<Vec<Instance>, AppError> {
    instances::list(&app)
}

/// Checks the chosen folder and suggests a name, a port and a colour along the
/// way.
///
/// The check and the suggestions arrive in one response: the add screen shows
/// them together, and there is no point splitting that into three calls.
#[tauri::command]
#[specta::specta]
async fn probe_folder(app: tauri::AppHandle, path: String) -> Result<ProbeResult, AppError> {
    instances::probe(&app, &WindowsPortable, &path)
}

#[tauri::command]
#[specta::specta]
async fn suggest_accent(app: tauri::AppHandle) -> Result<Accent, AppError> {
    instances::suggest_accent(&app)
}

#[tauri::command]
#[specta::specta]
async fn add_instance(
    app: tauri::AppHandle,
    path: String,
    edit: InstanceEdit,
) -> Result<Instance, AppError> {
    // The folder is checked afresh rather than taken from the probe_folder
    // response: it could have been renamed between the picker screen and the
    // save.
    let probe = <WindowsPortable as crate::discovery::InstanceDiscovery>::probe(
        &WindowsPortable,
        std::path::Path::new(&path),
    )?;
    instances::add(&app, probe, edit, None)
}

#[tauri::command]
#[specta::specta]
async fn update_instance(
    app: tauri::AppHandle,
    id: String,
    edit: InstanceEdit,
) -> Result<Instance, AppError> {
    instances::update(&app, &id, edit)
}

/// Removes an instance from the registry. The folder on disk stays untouched.
#[tauri::command]
#[specta::specta]
async fn remove_instance(app: tauri::AppHandle, id: String) -> Result<(), AppError> {
    instances::remove(&app, &id)
}

// ------------------------------------------------------ the install wizard

#[tauri::command]
#[specta::specta]
async fn probe_archive(path: String) -> Result<ArchiveInfo, AppError> {
    installer::probe_archive(&path)
}

#[tauri::command]
#[specta::specta]
async fn check_targets(
    info: ArchiveInfo,
    targets: Vec<InstallTarget>,
) -> Result<Vec<TargetCheck>, AppError> {
    Ok(installer::check_targets(&info, &targets))
}

#[tauri::command]
#[specta::specta]
async fn archive_history(app: tauri::AppHandle) -> Result<Vec<ArchiveRecord>, AppError> {
    installer::history::list(&app)
}

#[tauri::command]
#[specta::specta]
async fn forget_archive(app: tauri::AppHandle, path: String) -> Result<(), AppError> {
    installer::history::forget(&app, &path)
}

/// Unpacks the archive into the targets and registers them.
///
/// The command is `async`, so the minutes of extraction do not block the main
/// thread. Progress goes as events: there is nothing to return it in, a
/// command answers once.
#[tauri::command]
#[specta::specta]
async fn run_install(
    app: tauri::AppHandle,
    lock: tauri::State<'_, InstallLock>,
    cancel: tauri::State<'_, InstallCancel>,
    info: ArchiveInfo,
    targets: Vec<InstallTarget>,
) -> Result<Vec<Instance>, AppError> {
    let _guard = lock.acquire()?;
    cancel.reset();

    // First of all, before any work: only the IPC time passes between the
    // click and this event, and the screen stops being silent immediately.
    // Without it the first seconds looked like a freeze — the checks, opening
    // the archive and unfolding the LZMA2 dictionary all happen quietly.
    let first_name = targets.first().map(|t| t.name.clone()).unwrap_or_default();
    let _ = installer::InstallProgress::stage(
        installer::InstallPhase::Preparing,
        1,
        targets.len() as u32,
        &first_name,
    )
    .emit(&app);

    // The checks are repeated right before the work: between the targets
    // screen and the start, the disk could have run out of space and the
    // folder could have appeared.
    let blocking: Vec<AppError> = installer::check_targets(&info, &targets)
        .into_iter()
        .flat_map(|c| c.errors)
        .collect();
    if let Some(first) = blocking.into_iter().next() {
        return Err(first);
    }

    // The minutes of extraction go to a thread of their own rather than to an
    // async runtime worker: otherwise `cancel_install` and the other commands
    // compete for that same worker the whole time.
    let emitter = app.clone();
    let work_info = info.clone();
    let work_targets = targets.clone();
    let cancel_flag = cancel.share();
    let outcome = tauri::async_runtime::spawn_blocking(move || {
        installer::run(&work_info, &work_targets, &cancel_flag, |progress| {
            let _ = progress.emit(&emitter);
        })
    })
    .await
    .map_err(|e| AppError::because("installer.extractFailed", e))?;
    outcome?;

    // Registration re-checks every target and runs `python --version` for each
    // — that is another couple of seconds after the files have run out.
    // Previously the screen sat at a hundred per cent with the caption of the
    // last file.
    let _ = installer::InstallProgress::stage(
        installer::InstallPhase::Registering,
        targets.len() as u32,
        targets.len() as u32,
        &first_name,
    )
    .emit(&app);

    installer::history::remember(&app, &info)?;
    register_targets(&app, &info, &targets)
}

/// Registers the unpacked targets, filling in the source.
fn register_targets(
    app: &tauri::AppHandle,
    info: &ArchiveInfo,
    targets: &[InstallTarget],
) -> Result<Vec<Instance>, AppError> {
    let source = instances::InstallSource {
        archive_path: info.path.clone(),
        archive_label: info.label.clone(),
        installed_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as f64)
            .unwrap_or(0.0),
    };

    let mut created = Vec::new();
    for target in targets {
        let probe = <WindowsPortable as crate::discovery::InstanceDiscovery>::probe(
            &WindowsPortable,
            std::path::Path::new(&target.path),
        )?;
        let edit = InstanceEdit {
            name: target.name.clone(),
            description: target.description.clone(),
            accent: target.accent.clone(),
            preferred_port: target.preferred_port,
        };
        created.push(instances::add(app, probe, edit, Some(source.clone()))?);
    }
    Ok(created)
}

/// Asks the wizard to stop. It is checked between files, so a cancelled
/// install leaves no half-unpacked tree behind.
#[tauri::command]
#[specta::specta]
async fn cancel_install(cancel: tauri::State<'_, InstallCancel>) -> Result<(), AppError> {
    cancel.request();
    Ok(())
}


// ------------------------------------------------------ launching a build

/// The event carrying the next log line of a running instance.
///
/// Named with a `Run` prefix so as not to collide with the spike's
/// `SpikeLog`: tauri-specta derives the event name from the struct name, and
/// identical names would have drifted apart silently.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
#[serde(rename_all = "camelCase")]
pub struct RunLog {
    pub instance_id: String,
    pub line: crate::process::LogLine,
}

/// The state-change event. It also arrives when the user did nothing: a crash
/// and a self-restart have to be visible immediately.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct RunChanged(pub RunStatus);

/// An instance's launch profiles, parsed from its `.bat` right now.
#[tauri::command]
#[specta::specta]
async fn instance_profiles(
    app: tauri::AppHandle,
    id: String,
) -> Result<Vec<LaunchProfile>, AppError> {
    let instance = instances::list(&app)?
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", &id))?;
    Ok(run::profiles_of(&instance))
}

/// The final launch command — the one that will actually go to the system.
///
/// Shown in the argument editor: `--port` and `--disable-auto-launch` are
/// added by us, and without a preview the user would be arguing with the
/// invisible.
#[tauri::command]
#[specta::specta]
async fn preview_command(
    app: tauri::AppHandle,
    id: String,
    profile_id: String,
) -> Result<Vec<String>, AppError> {
    let instance = instances::list(&app)?
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", &id))?;

    let all = run::profiles_of(&instance);
    let profile = all
        .iter()
        .find(|p| p.id == profile_id)
        .ok_or_else(|| AppError::new("run.noProfiles"))?;

    // The port shown is the preferred one: the real one is handed out at
    // launch, and a specific number must not be promised in advance.
    let mut line = vec![profile.python_path.clone()];
    line.extend(profiles::apply_runtime_args(
        &profile.args,
        instance.preferred_port,
        None,
    ));
    Ok(line)
}

/// Saves a custom launch profile.
#[tauri::command]
#[specta::specta]
async fn save_custom_profile(
    app: tauri::AppHandle,
    id: String,
    profile: instances::CustomProfile,
) -> Result<Instance, AppError> {
    instances::save_profile(&app, &id, profile)
}

#[tauri::command]
#[specta::specta]
async fn remove_custom_profile(
    app: tauri::AppHandle,
    id: String,
    profile_id: String,
) -> Result<Instance, AppError> {
    instances::remove_profile(&app, &id, &profile_id)
}

#[tauri::command]
#[specta::specta]
async fn run_statuses(runtime: tauri::State<'_, Runtime>) -> Result<Vec<RunStatus>, AppError> {
    Ok(runtime.statuses())
}

/// The whole accumulated log of an instance.
///
/// Needed so that coming back to the instance screen shows the startup in
/// full: the events only catch what happens while the screen is open.
#[tauri::command]
#[specta::specta]
async fn run_log(
    runtime: tauri::State<'_, Runtime>,
    id: String,
) -> Result<Vec<crate::process::LogLine>, AppError> {
    Ok(runtime
        .get(&id)
        .map(|cell| cell.lock().unwrap().log.snapshot())
        .unwrap_or_default())
}

// ------------------------------------- moving models into the shared folder

/// This build's models and which of them are already in the shared folder.
#[tauri::command]
#[specta::specta]
async fn scan_instance_models(
    app: tauri::AppHandle,
    id: String,
) -> Result<ModelsScan, AppError> {
    let (models, shared) = migrate_paths(&app, &id)?;
    tauri::async_runtime::spawn_blocking(move || migrate::scan(&models, &shared))
        .await
        .map_err(|e| AppError::because("migrate.readFailed", e))
}

/// Moves the selected models into the shared folder.
///
/// The selection arrives as "category and model" pairs, the same as for the
/// cleanup: on screen the toggle sits next to every model, not next to a whole
/// category.
///
/// Refuses for a running build: files must not be taken out from under a
/// running ComfyUI — it holds them open and already resolved the paths at
/// startup.
#[tauri::command]
#[specta::specta]
async fn migrate_models(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    cancel: tauri::State<'_, migrate::MigrateCancel>,
    id: String,
    items: Vec<(String, String)>,
) -> Result<migrate::MigrateOutcome, AppError> {
    if running_port(&runtime, &id).is_some() {
        return Err(AppError::new("migrate.instanceRunning"));
    }
    let (models, shared) = migrate_paths(&app, &id)?;
    cancel.reset();

    // Space is checked before the start, not halfway through: learning about a
    // shortage on the nineteenth gigabyte out of twenty is the worst possible
    // outcome.
    let scan = migrate::scan(&models, &shared);
    let need: f64 = scan
        .categories
        .iter()
        .flat_map(|c| {
            c.entries.iter().filter(|e| {
                e.same_name.is_none()
                    && items.iter().any(|(cat, name)| cat == &c.folder && name == &e.name)
            })
        })
        .map(|e| e.size_bytes)
        .sum();
    if !migrate::enough_space(&shared, need) {
        return Err(AppError::with("migrate.noSpace", "path", shared.display()));
    }

    // Moving tens of gigabytes goes to a thread of its own rather than to an
    // async runtime worker: otherwise the cancellation competes for that same
    // worker.
    let emitter = app.clone();
    let flag = cancel.share();
    tauri::async_runtime::spawn_blocking(move || {
        migrate::move_all(&models, &shared, &items, &flag, |progress| {
            let _ = progress.emit(&emitter);
        })
    })
    .await
    .map_err(|e| AppError::because("migrate.writeFailed", e))
}

#[tauri::command]
#[specta::specta]
async fn cancel_migrate(cancel: tauri::State<'_, migrate::MigrateCancel>) -> Result<(), AppError> {
    cancel.cancel();
    Ok(())
}

/// Removes from a build what already lies in the shared folder.
///
/// Three conditions are checked here and not only on the screen: the build is
/// stopped, it is connected to shared models, and every entry has been
/// recognised as a duplicate afresh inside `remove_duplicates`. Deletion is
/// not the place where input data can be trusted.
#[tauri::command]
#[specta::specta]
async fn remove_duplicate_models(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    items: Vec<(String, String)>,
) -> Result<migrate::CleanupOutcome, AppError> {
    if running_port(&runtime, &id).is_some() {
        return Err(AppError::new("migrate.instanceRunning"));
    }
    let instance = find_instance(&app, &id)?;
    if !instance.shared.enabled {
        // Not connected — the deletion would leave it with no models at all.
        // That is not a cleanup, that is breakage.
        return Err(AppError::new("migrate.notConnected"));
    }

    let (models, shared) = migrate_paths(&app, &id)?;
    tauri::async_runtime::spawn_blocking(move || {
        migrate::remove_duplicates(&models, &shared, &items)
    })
    .await
    .map_err(|e| AppError::because("migrate.removeFailed", e))
}

/// A build's models folder and the shared root.
fn migrate_paths(
    app: &tauri::AppHandle,
    id: &str,
) -> Result<(std::path::PathBuf, std::path::PathBuf), AppError> {
    let instance = find_instance(app, id)?;
    if !instance.available {
        return Err(AppError::with("instances.missing", "path", &instance.path));
    }

    let shared = migrate::first_root(&settings::load_shared(app)?)
        .ok_or_else(|| AppError::new("shared.noRoots"))?;

    let root = std::path::Path::new(&instance.path);
    let models = match run::profiles_of(&instance).first() {
        Some(profile) => profiles::models_dir(profile, root),
        None => root.join("ComfyUI").join("models"),
    };
    Ok((models, shared))
}

// --------------------------------------------------- the workflow library

/// Where to put a fresh library if the user did not choose themselves.
///
/// Next to the shared models root — those usually lie on a spacious drive, and
/// keeping workflows in the same place is sensible. There is no hard link: the
/// library works without shared models, so if there are none we simply stay
/// silent.
#[tauri::command]
#[specta::specta]
async fn suggest_library_path(app: tauri::AppHandle) -> Result<Option<String>, AppError> {
    let shared = settings::load_shared(&app)?;
    Ok(shared
        .roots
        .first()
        .and_then(|r| std::path::Path::new(&r.path).parent().map(|p| p.to_path_buf()))
        .map(|p| p.join("workflows").display().to_string()))
}

#[tauri::command]
#[specta::specta]
async fn load_library_settings(app: tauri::AppHandle) -> Result<LibrarySettings, AppError> {
    settings::load_library(&app)
}

#[tauri::command]
#[specta::specta]
async fn save_library_settings(
    app: tauri::AppHandle,
    library: LibrarySettings,
) -> Result<(), AppError> {
    settings::save_library(&app, &library)
}

/// Reads the whole library.
///
/// In a blocking thread: walking the tree over a couple of hundred workflows
/// means thousands of disk accesses plus parsing every JSON for the sake of
/// the node list.
#[tauri::command]
#[specta::specta]
async fn scan_library(path: String) -> Result<LibraryScan, AppError> {
    tauri::async_runtime::spawn_blocking(move || {
        workflows::scan_library(std::path::Path::new(&path))
    })
    .await
    .map_err(|e| AppError::because("workflows.scanFailed", e))
}

/// Puts a file from disk into the library.
///
/// Accepts both `.json` and `.png`: an image from the `output` folder carries
/// the graph with it, and "drag the picture in" is the most common way of
/// getting back to a successful generation. In both cases what lands in the
/// library is a `.json` — that is what will later travel to an instance.
///
/// Not a workflow means a refusal with an explanation: the library has to stay
/// a library of workflows, not a dumping ground for JSON.
#[tauri::command]
#[specta::specta]
async fn add_workflow_file(
    library: String,
    source: String,
    rel: Option<String>,
    overwrite: bool,
) -> Result<String, AppError> {
    let path = std::path::Path::new(&source);
    let is_png = path
        .extension()
        .map(|e| e.eq_ignore_ascii_case("png"))
        .unwrap_or(false);

    let content = if is_png {
        let bytes = std::fs::read(path).map_err(|e| AppError::because("workflows.readFailed", e))?;
        workflows::workflow_from_png(&bytes)
            .ok_or_else(|| AppError::with("workflows.noGraphInImage", "path", &source))?
    } else {
        std::fs::read_to_string(path).map_err(|e| AppError::because("workflows.readFailed", e))?
    };

    if workflows::node_types(&content).is_none() {
        return Err(AppError::with("workflows.notAWorkflow", "path", &source));
    }

    let name = rel.unwrap_or_else(|| {
        let stem = path
            .file_stem()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "workflow".to_string());
        if is_png {
            format!("{stem}.json")
        } else {
            path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "workflow.json".to_string())
        }
    });

    let target = std::path::Path::new(&library).join(&name);
    if target.exists() && !overwrite {
        return Err(AppError::with("workflows.nameTaken", "path", &name));
    }
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AppError::because("workflows.writeFailed", e))?;
    }
    std::fs::write(&target, content)
        .map_err(|e| AppError::because("workflows.writeFailed", e))?;

    Ok(name)
}

/// Puts a graph that arrived as text into the library.
///
/// The same end of the road as for a file, only the source differs: workflows
/// are sent as text — in a chat, on a forum — more often than as files, and
/// saving what was sent into a file merely to pick it in a dialog right after
/// is a wasted lap.
///
/// The name comes from an input field, so it is checked here rather than
/// trusted: it lands in a path. There is no overwriting here either — a taken
/// name is a refusal, not a replacement: replacing would mean erasing one
/// piece of work with another.
#[tauri::command]
#[specta::specta]
async fn add_workflow_text(
    library: String,
    name: String,
    content: String,
) -> Result<String, AppError> {
    let rel = workflows::file_name_from_input(&name)
        .ok_or_else(|| AppError::with("workflows.badName", "name", &name))?;

    if workflows::node_types(&content).is_none() {
        return Err(AppError::with("workflows.notAWorkflow", "path", &rel));
    }

    let target = std::path::Path::new(&library).join(&rel);
    if target.exists() {
        return Err(AppError::with("workflows.nameTaken", "path", &rel));
    }
    std::fs::write(&target, content).map_err(|e| AppError::because("workflows.writeFailed", e))?;

    Ok(rel)
}

/// Edits a manifest record: favourite, tags, note.
#[tauri::command]
#[specta::specta]
async fn set_workflow_meta(
    library: String,
    rel: String,
    meta: WorkflowMeta,
) -> Result<(), AppError> {
    let root = std::path::Path::new(&library);
    let (mut manifest, _) = workflows::read_manifest(root);
    manifest.items.insert(rel, meta);
    workflows::write_manifest(root, &manifest)
}

/// Removes a lost record from the manifest.
///
/// The record only: this command does not touch files at all — it is called
/// precisely when the file is already gone.
#[tauri::command]
#[specta::specta]
async fn forget_workflow(library: String, rel: String) -> Result<(), AppError> {
    let root = std::path::Path::new(&library);
    let (mut manifest, _) = workflows::read_manifest(root);
    manifest.items.remove(&rel);
    workflows::write_manifest(root, &manifest)
}

/// The workflows lying inside a build.
///
/// For a running one we ask over the API, for a stopped one we read the
/// folder. The difference is not cosmetic: for a running one the answer
/// accounts for what it saved a minute ago, and for a stopped one there is no
/// other source anyway.
#[tauri::command]
#[specta::specta]
async fn instance_workflows(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    library: String,
) -> Result<Vec<workflows::InstanceWorkflow>, AppError> {
    let instance = find_instance(&app, &id)?;
    let port = running_port(&runtime, &id);

    tauri::async_runtime::spawn_blocking(move || {
        let client = port.map(comfy_api::Client::new);
        let names: Vec<String> = match &client {
            Some(client) => client.list_workflows()?.into_iter().map(|f| f.path).collect(),
            None => local_workflow_names(&instance),
        };

        let root = std::path::Path::new(&library);
        let dir = local_workflows_dir(&instance);

        let mut out = Vec::with_capacity(names.len());
        for path in names {
            // Only those whose name is taken in the library get read. For the
            // rest there is nothing to compare against, and the list runs into
            // hundreds of files — and for a running build every read is an
            // HTTP request.
            let twin = root.join(&path);
            let verdict = if library.is_empty() || !twin.is_file() {
                None
            } else {
                let mine = match &client {
                    Some(client) => client.read_workflow(&path).ok(),
                    None => std::fs::read_to_string(dir.join(&path)).ok(),
                };
                match (mine, std::fs::read_to_string(&twin).ok()) {
                    (Some(a), Some(b)) if workflows::same_workflow(&a, &b) => {
                        Some(workflows::LibraryMatch::Same)
                    }
                    // If it did not read, we treat them as diverged. That keeps
                    // the button working: "we could not compare" is no reason
                    // to declare someone's work already saved.
                    _ => Some(workflows::LibraryMatch::Diverged),
                }
            };
            out.push(workflows::InstanceWorkflow { path, library: verdict });
        }

        Ok(out)
    })
    .await
    .map_err(|e| AppError::because("workflows.scanFailed", e))?
}

/// A build's workflow folder — for the "show in Explorer" button.
///
/// Existence is checked here: a stopped build that has not saved anything yet
/// has no such folder at all, and there is nothing to call Explorer with.
#[tauri::command]
#[specta::specta]
async fn instance_workflows_dir(
    app: tauri::AppHandle,
    id: String,
) -> Result<workflows::InstanceWorkflowsDir, AppError> {
    let instance = find_instance(&app, &id)?;
    let dir = local_workflows_dir(&instance);
    Ok(workflows::InstanceWorkflowsDir {
        available: dir.is_dir(),
        path: dir.to_string_lossy().into_owned(),
    })
}

/// Moves a workflow from a build into the library: none of it stays in the
/// build.
///
/// The order here is the only protection against losing someone's work, and it
/// is the same as for moving models: write the copy, **read it back and
/// compare**, and only then remove the source. Until the copy is verified, the
/// original has to stay in hand.
///
/// There is no overwriting at all. A taken name used to raise a "replace?"
/// question, and it was harmless while taking meant copying: whatever the
/// answer, the workflow stayed in the build. For a move the price of the
/// answer is different — "replace" would erase one piece of work with another,
/// leaving a copy of neither.
///
/// Instead of replacing there is `target`: take it under a free name. A taken
/// name stopped being a dead end, yet diverged versions stay two different
/// files rather than one on top of the other.
///
/// For a running build the source is removed by the build itself, through its
/// own API: the workflow folder belongs to it, and it knows nothing of edits
/// from outside.
#[tauri::command]
#[specta::specta]
async fn pull_workflow(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    rel: String,
    library: String,
    target: Option<String>,
) -> Result<String, AppError> {
    let instance = find_instance(&app, &id)?;
    let port = running_port(&runtime, &id);

    tauri::async_runtime::spawn_blocking(move || {
        // `rel` is where it lies in the build, `dest` is the name it will lie
        // under in the library. They always match except when taking a
        // diverged version.
        let dest = target.unwrap_or_else(|| rel.clone());

        let local = local_workflows_dir(&instance).join(&rel);
        let content = match port {
            Some(port) => comfy_api::Client::new(port).read_workflow(&rel)?,
            None => std::fs::read_to_string(&local)
                .map_err(|e| AppError::because("workflows.readFailed", e))?,
        };

        if workflows::node_types(&content).is_none() {
            return Err(AppError::with("workflows.notAWorkflow", "path", &rel));
        }

        let target = std::path::Path::new(&library).join(&dest);
        if target.exists() {
            return Err(AppError::with("workflows.nameTaken", "path", &dest));
        }
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::because("workflows.writeFailed", e))?;
        }
        std::fs::write(&target, &content)
            .map_err(|e| AppError::because("workflows.writeFailed", e))?;

        // A comparison, not trust in a successful `write`: a deletion comes
        // next, and it has to rest on what was read back from disk, not on the
        // write having returned no error.
        let written = std::fs::read_to_string(&target)
            .map_err(|e| AppError::because("workflows.verifyFailed", e))?;
        if written != content {
            let _ = std::fs::remove_file(&target);
            return Err(AppError::new("workflows.verifyFailed"));
        }

        // We remember where it was taken from: half a year later that is the
        // only way to understand why this workflow demands exactly these
        // nodes.
        let root = std::path::Path::new(&library);
        let (mut manifest, _) = workflows::read_manifest(root);
        let entry = manifest.items.entry(dest.clone()).or_default();
        entry.source_instance_id = Some(id.clone());
        if entry.added_at.is_none() {
            entry.added_at = Some(now_ms());
        }
        workflows::write_manifest(root, &manifest)?;

        // The copy is in place and verified — now the source can be removed.
        // A failure at this step loses nothing: the workflow ends up both in
        // the build and in the library, and the comparison on the next read of
        // the list will recognise them as the same and grey the button out.
        match port {
            Some(port) => comfy_api::Client::new(port).delete_workflow(&rel)?,
            None => std::fs::remove_file(&local)
                .map_err(|e| AppError::because("workflows.removeFailed", e))?,
        }

        Ok(dest)
    })
    .await
    .map_err(|e| AppError::because("workflows.writeFailed", e))?
}

/// Puts a workflow from the library into a build.
///
/// For a running one it goes through the API with `overwrite=false`, and **a
/// 409 comes back as a fork in the road**, not as an error: silently erasing
/// someone else's workflow is not allowed. For a stopped one it goes as a
/// file, with the same existence check.
#[tauri::command]
#[specta::specta]
async fn push_workflow(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    library: String,
    rel: String,
    overwrite: bool,
) -> Result<PushOutcome, AppError> {
    let instance = find_instance(&app, &id)?;
    if !instance.available {
        return Err(AppError::with("instances.missing", "path", &instance.path));
    }
    let port = running_port(&runtime, &id);

    tauri::async_runtime::spawn_blocking(move || {
        let content = std::fs::read_to_string(std::path::Path::new(&library).join(&rel))
            .map_err(|e| AppError::because("workflows.readFailed", e))?;

        match port {
            Some(port) => {
                match comfy_api::Client::new(port).upload_workflow(&rel, &content, overwrite)? {
                    comfy_api::UploadOutcome::Written => Ok(PushOutcome::Written),
                    comfy_api::UploadOutcome::Conflict => Ok(PushOutcome::Conflict),
                }
            }
            None => {
                let dir = local_workflows_dir(&instance);
                let target = dir.join(&rel);
                if target.exists() && !overwrite {
                    return Ok(PushOutcome::Conflict);
                }
                // The folder may not exist at all: ComfyUI creates it lazily,
                // on the first save. We create it ourselves.
                if let Some(parent) = target.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| AppError::because("workflows.writeFailed", e))?;
                }
                std::fs::write(&target, content)
                    .map_err(|e| AppError::because("workflows.writeFailed", e))?;
                Ok(PushOutcome::Written)
            }
        }
    })
    .await
    .map_err(|e| AppError::because("workflows.writeFailed", e))?
}

/// A workflow's compatibility with every registered build.
///
/// Computed for all of them at once: the user is choosing where to put it, and
/// the comparison has to happen on one screen rather than by visiting
/// instances one at a time.
#[tauri::command]
#[specta::specta]
async fn workflow_compat(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    rel: String,
    nodes: Vec<String>,
) -> Result<Vec<InstanceCompat>, AppError> {
    let instances = instances::list(&app)?;
    let cache_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| AppError::because("workflows.scanFailed", e))?;

    let ports: Vec<(String, Option<u16>)> =
        instances.iter().map(|i| (i.id.clone(), running_port(&runtime, &i.id))).collect();

    tauri::async_runtime::spawn_blocking(move || {
        let mut out = Vec::new();
        for (instance, (_, port)) in instances.iter().zip(ports) {
            // Three states, and the third must not be passed off as the first:
            // a build we know nothing about is "unknown", not "all good". A
            // green tick without grounds is worse than no tick at all.
            let (available, source) = match port {
                Some(port) => match comfy_api::Client::new(port).object_info_keys() {
                    Ok(keys) => {
                        comfy_api::cache::write(&cache_dir, &instance.id, &keys);
                        (Some(keys), CompatSource::Live)
                    }
                    Err(_) => (None, CompatSource::Unknown),
                },
                None => match comfy_api::cache::read(&cache_dir, &instance.id) {
                    Some(snapshot) => (Some(snapshot.nodes), CompatSource::Cached),
                    None => (None, CompatSource::Unknown),
                },
            };

            out.push(InstanceCompat {
                instance_id: instance.id.clone(),
                source,
                missing: available
                    .as_ref()
                    .map(|keys| workflows::missing_nodes(&nodes, keys))
                    .unwrap_or_default(),
                // There is no point asking over HTTP even for a running build:
                // ComfyUI keeps workflows as files, and the file system gives
                // the same answer in both states.
                present: local_workflows_dir(instance).join(&rel).is_file(),
            });
        }
        Ok(out)
    })
    .await
    .map_err(|e| AppError::because("workflows.scanFailed", e))?
}

/// Where the knowledge about a build's nodes came from.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum CompatSource {
    /// Asked of a running build right now.
    Live,
    /// From the snapshot of the last launch.
    Cached,
    /// The build is not running and has never been launched under us.
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstanceCompat {
    pub instance_id: String,
    pub source: CompatSource,
    /// Empty under `Unknown` — and that means "unknown", not "everything is
    /// there". Telling them apart is the interface's job, not the reader's.
    pub missing: Vec<String>,
    /// This workflow already lies in the build.
    ///
    /// Computed, not remembered. The interface used to know only about our own
    /// clicks within the current session, and after a re-entry it showed "add"
    /// for a build where the file was already present.
    pub present: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum PushOutcome {
    Written,
    /// The name is taken. A fork in the road, not an error: we ask the user.
    Conflict,
}

/// The port of a running build, if it is running.
fn running_port(runtime: &Runtime, id: &str) -> Option<u16> {
    runtime
        .get(id)
        .and_then(|cell| {
            let running = cell.lock().unwrap();
            matches!(running.status.state, RunState::Running).then_some(running.status.port)
        })
        .flatten()
}

/// The workflow folder of a stopped build, by its first profile.
fn local_workflows_dir(instance: &Instance) -> std::path::PathBuf {
    let root = std::path::Path::new(&instance.path);
    match run::profiles_of(instance).first() {
        Some(profile) => profiles::workflows_dir(profile, root),
        // There are no profiles at all — we take ComfyUI's default.
        None => root.join("ComfyUI").join("user").join("default").join("workflows"),
    }
}

/// The workflow names of a stopped build. The folder may not exist — that is
/// emptiness, not an error.
fn local_workflow_names(instance: &Instance) -> Vec<String> {
    let dir = local_workflows_dir(instance);
    let scan = workflows::scan_library(&dir);
    scan.items.into_iter().filter(|i| !i.lost).map(|i| i.path).collect()
}

fn now_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0)
}

// --------------------------------------------------------- shared models

#[tauri::command]
#[specta::specta]
async fn load_shared_settings(app: tauri::AppHandle) -> Result<SharedSettings, AppError> {
    settings::load_shared(&app)
}

#[tauri::command]
#[specta::specta]
async fn save_shared_settings(
    app: tauri::AppHandle,
    shared: SharedSettings,
) -> Result<(), AppError> {
    settings::save_shared(&app, &shared)
}

/// Scans the folder and returns the categories found.
///
/// The tree walk goes in a blocking thread: on a shared folder of hundreds of
/// gigabytes that is tens of thousands of metadata accesses, and holding the
/// main thread on them is not allowed — the interface would freeze exactly
/// when the user is waiting for an answer.
#[tauri::command]
#[specta::specta]
async fn scan_shared_root(path: String) -> Result<RootScan, AppError> {
    tauri::async_runtime::spawn_blocking(move || {
        shared_models::scan_root(std::path::Path::new(&path))
    })
    .await
    .map_err(|e| AppError::because("shared.scanFailed", e))
}

/// The YAML that the current settings will produce.
///
/// The preview is not cosmetic: the user has to see what exactly will land in
/// the config before it lands in their build.
#[tauri::command]
#[specta::specta]
async fn preview_shared_yaml(shared: SharedSettings) -> Result<String, AppError> {
    tauri::async_runtime::spawn_blocking(move || shared_models::render_settings(&shared).yaml)
        .await
        .map_err(|e| AppError::because("shared.scanFailed", e))
}

/// Creates the missing standard subfolders in the shared root.
#[tauri::command]
#[specta::specta]
async fn create_shared_folders(path: String, names: Vec<String>) -> Result<RootScan, AppError> {
    tauri::async_runtime::spawn_blocking(move || {
        let root = std::path::Path::new(&path);
        for name in &names {
            // The name comes from our own list of suggestions, but checking is
            // cheaper than one day creating a folder a level above the root.
            if name.contains(['/', '\\', ':']) || name.starts_with('.') {
                continue;
            }
            let _ = std::fs::create_dir_all(root.join(name));
        }
        shared_models::scan_root(root)
    })
    .await
    .map_err(|e| AppError::because("shared.scanFailed", e))
}

/// What is sitting in an instance's `extra_model_paths.yaml`.
#[tauri::command]
#[specta::specta]
async fn inspect_instance_config(
    app: tauri::AppHandle,
    id: String,
) -> Result<InstanceFileInfo, AppError> {
    let instance = find_instance(&app, &id)?;
    Ok(shared_models::inspect_instance_file(std::path::Path::new(&instance.path)))
}

/// Connects an instance to shared models.
///
/// `confirm_overwrite` applies only to the "file inside the instance" mode and
/// only to the case where someone else's file already lies there. Without
/// consent the command refuses with the code `shared.foreignConfig`, and the
/// frontend shows the comparison screen.
#[tauri::command]
#[specta::specta]
async fn connect_shared(
    app: tauri::AppHandle,
    id: String,
    apply_mode: ApplyMode,
    confirm_overwrite: bool,
) -> Result<Option<String>, AppError> {
    let instance = find_instance(&app, &id)?;
    let settings = settings::load_shared(&app)?;
    let rendered = shared_models::render_settings(&settings);

    if rendered.empty {
        return Err(AppError::new("shared.noRoots"));
    }

    let mut backup = None;
    if apply_mode == ApplyMode::InstanceFile {
        let root = std::path::Path::new(&instance.path);
        let info = shared_models::inspect_instance_file(root);
        if info.state == InstanceFileState::Foreign && !confirm_overwrite {
            return Err(AppError::with("shared.foreignConfig", "path", info.path));
        }
        backup = shared_models::write_instance_file(root, &rendered.yaml, now_secs())?;
    }

    instances::set_shared(&app, &id, InstanceShared { enabled: true, apply_mode })?;
    Ok(backup)
}

/// Disconnects an instance from shared models.
///
/// In the "file inside the instance" mode it removes our file and puts the
/// saved copy of someone else's back. The models in the shared folder are
/// never touched.
#[tauri::command]
#[specta::specta]
async fn disconnect_shared(app: tauri::AppHandle, id: String) -> Result<(), AppError> {
    let instance = find_instance(&app, &id)?;

    if instance.shared.apply_mode == ApplyMode::InstanceFile {
        shared_models::remove_instance_file(std::path::Path::new(&instance.path))?;
    }

    instances::set_shared(
        &app,
        &id,
        InstanceShared { enabled: false, apply_mode: instance.shared.apply_mode },
    )
}

fn find_instance(app: &tauri::AppHandle, id: &str) -> Result<Instance, AppError> {
    instances::list(app)?
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))
}

/// Epoch seconds — the stamp in a backup's name.
fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Prepares shared models for an instance launch.
///
/// Returns the config path for `--extra-model-paths-config` if the instance is
/// connected in flag mode. In the "file inside the instance" mode no flag is
/// needed: the file already lies in the build folder and ComfyUI picks it up
/// by itself.
fn prepare_shared(
    app: &tauri::AppHandle,
    instance: &Instance,
    without_shared: bool,
) -> Result<Option<String>, AppError> {
    if without_shared || !instance.shared.enabled {
        return Ok(None);
    }

    let settings = settings::load_shared(app)?;
    let rendered = shared_models::render_settings(&settings);

    // The check happens before the launch, not after: external drives do get
    // unplugged, and the user should not learn about it from a "model not
    // found" in the middle of their work.
    if let Some(path) = rendered.unavailable.first() {
        return Err(AppError::with("shared.rootUnavailable", "path", path));
    }
    if rendered.empty {
        return Ok(None);
    }

    if instance.shared.apply_mode == ApplyMode::InstanceFile {
        // The file may have gone stale: the user created a new category in the
        // shared folder while the config stayed as it was. We update it — but
        // only our own. Someone else's in this place means it was put there
        // after us, and we have no right to touch it on an ordinary launch.
        let root = std::path::Path::new(&instance.path);
        if shared_models::inspect_instance_file(root).state == InstanceFileState::Ours {
            shared_models::write_config(
                &shared_models::instance_config_path(root),
                &rendered.yaml,
            )?;
        }
        return Ok(None);
    }

    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::because("shared.writeFailed", e))?;
    let path = shared_models::flag_config_path(&dir);
    shared_models::write_config(&path, &rendered.yaml)?;
    Ok(Some(path.display().to_string()))
}

/// The user's consents, without which the launch refuses to proceed.
///
/// Both forks work the same way: the first call arrives with empty consents
/// and gets a refusal with a code, the frontend unfolds the choice in place,
/// and the repeat call arrives with the answer. A modal cannot be put here
/// (the z-order discipline), and a toast has no buttons.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct StartOptions {
    /// Launch even if the shared models root is unavailable.
    pub without_shared: bool,
    /// Launch even if another build is already running.
    pub allow_multiple: bool,
}

/// Launches an instance.
///
/// An unavailable shared root cannot simply be ignored: a user who kept their
/// models on an external drive would get a "model not found" in the middle of
/// their work and conclude the app is broken.
///
/// A second build on the same graphics card means an out-of-VRAM failure once
/// the generation is already queued, and understanding it from ComfyUI's side
/// is impossible.
#[tauri::command]
#[specta::specta]
async fn start_instance(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    profile_id: Option<String>,
    options: StartOptions,
) -> Result<RunStatus, AppError> {
    start_inner(&app, &runtime, id, profile_id, options)
}

fn start_inner(
    app: &tauri::AppHandle,
    runtime: &Runtime,
    id: String,
    profile_id: Option<String>,
    options: StartOptions,
) -> Result<RunStatus, AppError> {
    if runtime.is_busy(&id) {
        return Err(AppError::new("run.alreadyRunning"));
    }

    if !options.allow_multiple {
        if let Some(other) = other_running(app, runtime, &id) {
            return Err(AppError::with("run.otherRunning", "name", other));
        }
    }

    let instance = instances::list(app)?
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", &id))?;
    if !instance.available {
        return Err(AppError::with("instances.missing", "path", &instance.path));
    }

    let all = run::profiles_of(&instance);
    let profile = profile_id
        .as_deref()
        .and_then(|want| all.iter().find(|p| p.id == want))
        .or_else(|| all.first())
        .ok_or_else(|| AppError::new("run.noProfiles"))?
        .clone();

    let shared_config = prepare_shared(app, &instance, options.without_shared)?;

    let emitter = app.clone();
    let on_line = std::sync::Arc::new({
        let id = id.clone();
        move |line: crate::process::LogLine| {
            let _ = RunLog { instance_id: id.clone(), line }.emit(&emitter);
        }
    });

    let exit_app = app.clone();
    let exit_id = id.clone();
    let outcome = run::start(&instance, &profile, shared_config.as_deref(), on_line, move |exit| {
        finish(&exit_app, &exit_id, exit);
    })?;

    runtime.insert(&id, outcome.cell.clone());
    // The last-launch date is informational, and a failure to write it is no
    // obstacle to the launch: the build has already started.
    let _ = instances::mark_started(app, &id);
    let _ = RunChanged(outcome.status.clone()).emit(app);

    // Readiness is awaited in the background: the command has to return at
    // once, or the interface shows not one line until the cold start is over.
    let ready_app = app.clone();
    let ready_cell = outcome.cell.clone();
    let port = outcome.status.port.unwrap_or_default();
    std::thread::spawn(move || {
        let keep = {
            let cell = ready_cell.clone();
            move || {
                matches!(cell.lock().unwrap().status.state, RunState::Starting)
            }
        };
        match crate::process::wait_ready(port, crate::process::READY_TIMEOUT, keep) {
            Ok(secs) => {
                let mut running = ready_cell.lock().unwrap();
                if running.status.state == RunState::Starting {
                    running.status.state = RunState::Running;
                    running.status.ready_secs = Some(secs);
                    let _ = RunChanged(running.status.clone()).emit(&ready_app);
                }
            }
            Err(_) => {
                let mut running = ready_cell.lock().unwrap();
                if running.status.state == RunState::Starting {
                    running.status.state = RunState::Crashed;
                    let _ = RunChanged(running.status.clone()).emit(&ready_app);
                }
            }
        }
    });

    Ok(outcome.status)
}

/// The name of another running build, if there is one.
///
/// The name, not the identifier: the message is shown to the user, and
/// "i17550…" tells them nothing. The instance may have disappeared from the
/// registry — then we make do with the identifier, as long as we do not stay
/// silent.
fn other_running(app: &tauri::AppHandle, runtime: &Runtime, id: &str) -> Option<String> {
    let busy = tray::busy(runtime);
    let other = busy.into_iter().find(|other| other != id)?;
    let name = instances::list(app)
        .ok()
        .and_then(|all| all.into_iter().find(|i| i.id == other).map(|i| i.name));
    Some(name.unwrap_or(other))
}

/// Works out how the process ended and reports it upwards.
fn finish(app: &tauri::AppHandle, id: &str, exit: run::Exit) {
    let runtime = app.state::<Runtime>();
    let Some(cell) = runtime.get(id) else { return };
    let mut running = cell.lock().unwrap();
    let detached = matches!(exit, run::Exit::Detached);

    match exit {
        run::Exit::Requested => {
            running.status.state = RunState::Stopped;
            running.status.pid = None;
        }
        run::Exit::Crashed(code) => {
            running.status.state = RunState::Crashed;
            running.status.exit_code = code;
            running.status.pid = None;
        }
        run::Exit::Detached => {
            // The server is alive but not ours. We can no longer control it,
            // and pretending we can is not allowed.
            running.status.state = RunState::Detached;
            running.status.pid = None;
        }
    }

    // The tab is closed by whoever knows the process is over. The port died
    // with it, and a tab left behind would show a WebView2 error page instead
    // of the interface. The exception is `Detached`: there the server on the
    // port is alive, it is simply no longer our handle controlling it, and
    // there is no reason to take a working interface away from the user.
    if !detached {
        webview::close(app, id);
    }

    let _ = RunChanged(running.status.clone()).emit(app);
}

#[tauri::command]
#[specta::specta]
async fn stop_instance(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
) -> Result<(), AppError> {
    stop_inner(&app, &runtime, &id)
}

fn stop_inner(app: &tauri::AppHandle, runtime: &Runtime, id: &str) -> Result<(), AppError> {
    let cell = runtime
        .get(id)
        .ok_or_else(|| AppError::new("run.notRunning"))?;
    let _ = RunChanged(RunStatus {
        state: RunState::Stopping,
        ..cell.lock().unwrap().status.clone()
    })
    .emit(app);
    run::stop(&cell)
}

/// Takes control of a server that restarted itself.
///
/// After installing nodes, ComfyUI-Manager shuts the server down and brings a
/// new process up. Our handle is lost, the state becomes `Detached`, and from
/// then on the app can only watch: it does not know the new process's PID.
///
/// We find the port's owner through the connection table and record its PID.
/// After that everything ordinary works again — stopping and the tab alike.
#[tauri::command]
#[specta::specta]
async fn adopt_instance(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
) -> Result<RunStatus, AppError> {
    let cell = runtime
        .get(&id)
        .ok_or_else(|| AppError::new("run.notRunning"))?;

    let port = {
        let running = cell.lock().unwrap();
        if running.status.state != RunState::Detached {
            return Err(AppError::new("run.notDetached"));
        }
        running.status.port.ok_or_else(|| AppError::new("run.notRunning"))?
    };

    // The port could have been released while the user was reading the
    // message.
    if !crate::process::probe(port) {
        return Err(AppError::new("run.notRunning"));
    }

    let pid = supervise::windows::pid_listening_on(port)
        .ok_or_else(|| AppError::with("run.ownerUnknown", "port", port))?;

    let status = {
        let mut running = cell.lock().unwrap();
        running.status.state = RunState::Running;
        running.status.pid = Some(pid);
        running.status.exit_code = None;
        running.status.clone()
    };
    let _ = RunChanged(status.clone()).emit(&app);
    Ok(status)
}

/// Stops an instance and brings it back up with the same profile.
///
/// Done in Rust rather than as two calls from the frontend: `stop` returns as
/// soon as the port is released, while the state is moved to `Stopped` by the
/// process-waiting thread — and a start sent immediately after would run into
/// `run.alreadyRunning`. Here we simply wait for the transition to finish.
#[tauri::command]
#[specta::specta]
async fn restart_instance(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
) -> Result<RunStatus, AppError> {
    let profile_id = runtime
        .get(&id)
        .and_then(|cell| cell.lock().unwrap().status.profile_id.clone());

    stop_inner(&app, &runtime, &id)?;

    // We wait for the process-waiting thread to work out how it ended. By this
    // point the port has already been released: `run::stop` sees to that.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while runtime.is_busy(&id) && std::time::Instant::now() < deadline {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // A restart is a continuation of what was already running: there is no
    // reason to ask again about shared models or about the neighbouring build,
    // the user already gave those consents when they launched it.
    start_inner(
        &app,
        &runtime,
        id,
        profile_id,
        StartOptions { without_shared: false, allow_multiple: true },
    )
}

/// Measures an instance's size on disk.
///
/// The command is `async`, so it runs off the main thread and the interface
/// does not freeze for all the minutes of the walk. `None` means a measurement
/// is already running.
#[tauri::command]
#[specta::specta]
async fn measure_instance_size(
    app: tauri::AppHandle,
    jobs: tauri::State<'_, SizeJobs>,
    id: String,
) -> Result<Option<Sized_>, AppError> {
    instances::measure_size(&app, &jobs, &id)
}

// ------------------------------------------------------- the embedded tab

/// Shows an instance's tab, creating it on the first call.
///
/// The embedding commands have to be `async`. A synchronous Tauri command runs
/// on the main thread, and `add_child` from inside it queues work onto that
/// same thread and waits for its result — which is a deadlock without a single
/// error in the log.
#[tauri::command]
#[specta::specta]
async fn show_comfy(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    rect: webview::Rect,
) -> Result<(), AppError> {
    // The port is taken from the run state, not from the frontend: it was
    // handed out by us and changes on every launch.
    let port = runtime
        .get(&id)
        .and_then(|cell| cell.lock().unwrap().status.port)
        .ok_or_else(|| AppError::new("run.notRunning"))?;
    webview::show(&app, &id, port, rect)
}

/// Moves the tab along with the content area.
#[tauri::command]
#[specta::specta]
async fn place_comfy(
    app: tauri::AppHandle,
    id: String,
    rect: webview::Rect,
) -> Result<(), AppError> {
    webview::place(&app, &id, rect)
}

/// Hides every tab: leaving for another section, opening the log console.
#[tauri::command]
#[specta::specta]
async fn hide_comfy(app: tauri::AppHandle) -> Result<(), AppError> {
    webview::hide_all(&app);
    Ok(())
}

#[tauri::command]
#[specta::specta]
async fn reload_comfy(app: tauri::AppHandle, id: String) -> Result<(), AppError> {
    webview::reload(&app, &id)
}

// --------------------------------------------------- the duplicate report

/// Builds a report on duplicated models across every build at once.
///
/// The command **does nothing to files** and never will: cleaning duplicates
/// up lives in a command of its own, on its own screen and with its own list.
#[tauri::command]
#[specta::specta]
async fn scan_duplicates(
    app: tauri::AppHandle,
    cancel: tauri::State<'_, duplicates::ScanCancel>,
) -> Result<duplicates::DuplicatesReport, AppError> {
    cancel.reset();

    let mut places = Vec::new();
    for instance in instances::list(&app)? {
        if !instance.available {
            // An unavailable folder must not be skipped silently: the report
            // would look complete. We put it in as a place with a
            // non-existent path — the scanner files it under skipped itself.
            places.push(duplicates::Place {
                name: instance.name.clone(),
                models_dir: std::path::PathBuf::from(&instance.path),
            });
            continue;
        }
        let profiles = run::profiles_of(&instance);
        let Some(profile) = profiles.first() else { continue };
        places.push(duplicates::Place {
            name: instance.name.clone(),
            models_dir: profiles::models_dir(profile, std::path::Path::new(&instance.path)),
        });
    }

    // The shared folder is a place just the same: a model lying both there and
    // in a build is a duplicate in exactly the same sense.
    let shared = settings::load_shared(&app)?;
    if let Some(root) = migrate::first_root(&shared) {
        places.push(duplicates::Place {
            name: root.display().to_string(),
            models_dir: root,
        });
    }

    let emitter = app.clone();
    Ok(duplicates::scan(&places, &cancel, move |progress| {
        let _ = progress.emit(&emitter);
    }))
}

#[tauri::command]
#[specta::specta]
async fn cancel_duplicates_scan(
    cancel: tauri::State<'_, duplicates::ScanCancel>,
) -> Result<(), AppError> {
    cancel.cancel();
    Ok(())
}

// ------------------------------------------------------ the tray and exit

/// The tray menu labels for the current language.
///
/// The tray menu is native, `t()` cannot reach it, and translating strings in
/// Rust is forbidden by the project's rules. So the text comes from the
/// frontend — and comes again on every change of language.
#[tauri::command]
#[specta::specta]
async fn set_tray_labels(app: tauri::AppHandle, labels: tray::TrayLabels) -> Result<(), AppError> {
    tray::set_labels(&app, &labels);
    Ok(())
}

/// The instances that closing the app would take down with it.
#[tauri::command]
#[specta::specta]
async fn busy_instances(runtime: tauri::State<'_, Runtime>) -> Result<Vec<String>, AppError> {
    Ok(tray::busy(&runtime))
}

/// Stop everything and quit.
#[tauri::command]
#[specta::specta]
async fn stop_all_and_quit(app: tauri::AppHandle) -> Result<(), AppError> {
    tray::stop_all(&app);
    app.exit(0);
    Ok(())
}

/// Collapse into the tray, leaving the servers running.
#[tauri::command]
#[specta::specta]
async fn hide_to_tray(app: tauri::AppHandle) -> Result<(), AppError> {
    webview::hide_all(&app);
    if let Some(window) = app.get_window("main") {
        window
            .hide()
            .map_err(|e| AppError::because("webview.embedFailed", e))?;
    }
    Ok(())
}

// ------------------------------------------------------------- updating

/// Asks whether a new version is out. `None` means the latest is installed.
///
/// Returns the error as it is and leaves the muting to the caller: the
/// automatic check at startup stays silent about network failures, a manual
/// one shows them. There is nothing in Rust to tell those apart, whereas the
/// frontend can see who pressed the button.
#[tauri::command]
#[specta::specta]
async fn check_update(app: tauri::AppHandle) -> Result<Option<update::UpdateInfo>, AppError> {
    update::check(&app).await
}

/// Installs the update and restarts the app.
///
/// The "builds are running" fork works the same way as the multi-launch guard:
/// the first call arrives with `stop_running: false` and gets a refusal with a
/// code, the frontend unfolds the choice in place, and the repeat call arrives
/// with the answer. Silently shutting down someone's generation queue is not
/// allowed — the Windows installer will close us by force, and the Job Object
/// will take every build down with it.
#[tauri::command]
#[specta::specta]
async fn install_update(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    stop_running: bool,
) -> Result<(), AppError> {
    let busy = tray::busy(&runtime);
    if !busy.is_empty() {
        if !stop_running {
            return Err(AppError::with("update.instancesRunning", "names", busy_names(&app, &busy)));
        }
        tray::stop_all(&app);
    }

    update::install(&app).await
}

/// The names of the running builds, comma-separated — for the message to the
/// user.
///
/// Names, not identifiers: "i17550…" would tell them nothing. A build that
/// vanished from the registry makes do with its identifier, as long as we do
/// not stay silent.
fn busy_names(app: &tauri::AppHandle, ids: &[String]) -> String {
    let all = instances::list(app).unwrap_or_default();
    ids.iter()
        .map(|id| {
            all.iter()
                .find(|i| &i.id == id)
                .map(|i| i.name.clone())
                .unwrap_or_else(|| id.clone())
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// An instance's generation output folder.
///
/// `None` means "the folder is not there yet": ComfyUI does not create it
/// before the first generation, and there is nothing to open. We will not
/// create it on the user's behalf — nothing appears inside someone else's
/// installation by our will.
#[tauri::command]
#[specta::specta]
async fn instance_output_dir(
    app: tauri::AppHandle,
    id: String,
    profile_id: Option<String>,
) -> Result<Option<String>, AppError> {
    let instance = instances::list(&app)?
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", &id))?;

    let all = run::profiles_of(&instance);
    let profile = profile_id
        .as_deref()
        .and_then(|want| all.iter().find(|p| p.id == want))
        .or_else(|| all.first())
        .ok_or_else(|| AppError::new("run.noProfiles"))?;

    let dir = profiles::output_dir(profile, std::path::Path::new(&instance.path));
    Ok(dir.is_dir().then(|| dir.display().to_string()))
}

/// A single list of commands and events: both the invoke handler and the type
/// generator are taken from here, so the two cannot drift apart.
fn specta_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
            load_bootstrap,
            save_settings,
            list_instances,
            probe_folder,
            suggest_accent,
            add_instance,
            update_instance,
            remove_instance,
            measure_instance_size,
            probe_archive,
            check_targets,
            archive_history,
            forget_archive,
            run_install,
            cancel_install,
            instance_profiles,
            preview_command,
            save_custom_profile,
            remove_custom_profile,
            scan_instance_models,
            migrate_models,
            cancel_migrate,
            remove_duplicate_models,
            suggest_library_path,
            load_library_settings,
            save_library_settings,
            scan_library,
            add_workflow_file,
            add_workflow_text,
            set_workflow_meta,
            forget_workflow,
            instance_workflows,
            instance_workflows_dir,
            pull_workflow,
            push_workflow,
            workflow_compat,
            load_shared_settings,
            save_shared_settings,
            scan_shared_root,
            preview_shared_yaml,
            create_shared_folders,
            inspect_instance_config,
            connect_shared,
            disconnect_shared,
            run_statuses,
            run_log,
            start_instance,
            stop_instance,
            restart_instance,
            adopt_instance,
            show_comfy,
            place_comfy,
            hide_comfy,
            reload_comfy,
            instance_output_dir,
            scan_duplicates,
            cancel_duplicates_scan,
            set_tray_labels,
            busy_instances,
            stop_all_and_quit,
            hide_to_tray,
            check_update,
            install_update
        ])
        .events(tauri_specta::collect_events![
            InstallProgress,
            migrate::MigrateProgress,
            RunLog,
            RunChanged,
            duplicates::DupProgress,
            tray::QuitRequested,
            update::UpdateProgress
        ])
}

/// Exports the types into `src/bindings.ts`.
///
/// Called both from the dev build and from the test. The test matters more: it
/// generates the types without opening a window, so it works in CI and needs
/// neither a display nor the Vite dev server.
#[cfg(debug_assertions)]
fn export_bindings(builder: &tauri_specta::Builder<tauri::Wry>) {
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("failed to export the types into bindings.ts");
}

#[cfg(test)]
mod tests {
    /// Keeps `src/bindings.ts` in agreement with the command signatures.
    /// It breaks exactly when the contract has changed — and that is good.
    #[test]
    fn bindings_up_to_date() {
        super::export_bindings(&super::specta_builder());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    #[cfg(debug_assertions)]
    export_bindings(&builder);

    tauri::Builder::default()
        // First, as the plugin requires: a second instance has to learn about
        // the first before it manages to create anything. A second window
        // would mean a second Job Object and a second writer to the same
        // registry.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            tray::reveal(app);
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        // The updater asks for the manifest only on our command: the webview
        // has no permission to call the plugin directly, and there is nothing
        // on the frontend side to get around the "builds are running" guard
        // with.
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(SizeJobs::default())
        .manage(InstallLock::default())
        .manage(InstallCancel::default())
        .manage(migrate::MigrateCancel::default())
        .manage(duplicates::ScanCancel::default())
        .manage(Runtime::default())
        .manage(tray::TrayItems::default())
        .invoke_handler(builder.invoke_handler())
        .on_window_event(tray::on_window_event)
        .setup(move |app| {
            // Mandatory: without mount_events the typed events never reach the
            // frontend.
            builder.mount_events(app);

            // The Job Object before anything else: builds launched before it
            // is installed will outlive a crash of the app and leave the VRAM
            // occupied.
            if let Err(e) = supervise::windows::install_job_object() {
                eprintln!("[CPO] job object was not created: {e}. Child processes may outlive the app.");
            }

            if let Err(e) = tray::install(app.handle()) {
                // The app works without a tray, there is simply nowhere left
                // to "collapse" into. No reason to fail the launch over it.
                eprintln!("[CPO] tray was not created: {e}");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to start the application");
}
