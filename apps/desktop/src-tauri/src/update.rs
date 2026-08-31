//! App updates: checking, downloading, installing.
//!
//! The only thing the app sends outwards is a request for the `latest.json`
//! manifest with the current version number in the address. That is why the
//! check is switchable, and why the automatic check at startup stays silent
//! about network failures: with no network the app has to work as usual rather
//! than greet the user with an error they had nothing to do with.
//!
//! **Installation never happens silently.** On Windows the installer closes
//! the app by force, and the child processes live in a Job Object with
//! `KILL_ON_JOB_CLOSE` and go down with us. An update in the middle of a
//! generation would cost the user their queue and minutes of cold start, so
//! the fate of running builds is decided by them, not by us.
//!
//! **That same Job Object nearly killed the update itself.** The installer is
//! launched with `ShellExecuteW` and inherits our job, and the
//! `std::process::exit(0)` on the next line closes it — so `install` clears the
//! job's limits from the plugin's `on_before_exit` hook, which runs in between.
//! See `supervise::windows::release_job_object`. The hook has to call
//! `cleanup_before_exit` by hand: `updater_builder` sets one of its own for
//! exactly that, and `on_before_exit` replaces rather than chains, so ours
//! would otherwise leave the tray icon behind. `check` needs none of this and
//! keeps using the plain `app.updater()`.
//!
//! The update signature is verified by the plugin itself against the `pubkey`
//! from the configuration: a mismatch means the install never starts. This is
//! not code signing and has no effect on SmartScreen — different mechanisms.

use serde::{Deserialize, Serialize};
use tauri_plugin_updater::UpdaterExt;
use tauri_specta::Event;

use crate::error::AppError;

/// A newer version that was found.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub version: String,
    /// The one installed right now. Next to the new one so the screen does not
    /// have to assemble the pair from two sources.
    pub current_version: String,
    /// The release body: this version's section of `CHANGELOG.md`. Not
    /// translated — it is release text, not an interface string.
    pub notes: Option<String>,
    /// Epoch milliseconds, like the rest of the dates in the registry: the
    /// frontend formats them by locale rules, not Rust.
    pub date: Option<f64>,
}

/// Download progress. An event rather than a command response: a command
/// answers once, while the install runs for tens of seconds.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgress {
    pub downloaded: f64,
    /// `None` means the server sent no length. The bar is indeterminate then.
    pub total: Option<f64>,
}

/// Asks for the manifest. `None` means the latest version is installed.
pub async fn check(app: &tauri::AppHandle) -> Result<Option<UpdateInfo>, AppError> {
    let current = app.package_info().version.to_string();

    let updater = app
        .updater()
        .map_err(|e| AppError::because("update.checkFailed", e))?;

    let found = updater
        .check()
        .await
        .map_err(|e| AppError::because("update.checkFailed", e))?;

    Ok(found.map(|update| UpdateInfo {
        version: update.version.clone(),
        current_version: current,
        notes: update.body.clone(),
        date: update
            .date
            .map(|d| d.unix_timestamp() as f64 * 1000.0),
    }))
}

/// Downloads and installs the update, then restarts the app.
///
/// The check is repeated here instead of being taken from the `check`
/// response: the update object lives inside the plugin and does not cross the
/// IPC boundary, and time passes between the screen and the press.
pub async fn install(app: &tauri::AppHandle) -> Result<(), AppError> {
    let updater = app
        .updater_builder()
        .on_before_exit({
            let app = app.clone();
            move || {
                if let Err(e) = crate::supervise::windows::release_job_object() {
                    eprintln!("[CPO] job object was not released: {e}");
                }
                app.cleanup_before_exit();
            }
        })
        .build()
        .map_err(|e| AppError::because("update.installFailed", e))?;

    let update = updater
        .check()
        .await
        .map_err(|e| AppError::because("update.checkFailed", e))?
        .ok_or_else(|| AppError::new("update.notAvailable"))?;

    let emitter = app.clone();
    let mut downloaded: u64 = 0;
    update
        .download_and_install(
            move |chunk, total| {
                downloaded += chunk as u64;
                let _ = UpdateProgress {
                    downloaded: downloaded as f64,
                    total: total.map(|t| t as f64),
                }
                .emit(&emitter);
            },
            || {},
        )
        .await
        .map_err(|e| AppError::because("update.installFailed", e))?;

    // We only get here on platforms where the installer did not take control
    // for itself. The restart does not return.
    app.restart()
}
