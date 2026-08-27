//! Recognising a ComfyUI installation on disk.
//!
//! One of the two traits where platform dependence lives. Everything above it
//! works with a `Probe` and knows nothing about `python_embeded` or `.bat`
//! files. When Linux comes around, a second implementation will appear and the
//! calling code will not change.

pub mod windows_portable;

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// A launch option that was found. In Phase 1 this is only the fact that the
/// file exists: parsing a `.bat` into an editable profile is Phase 2's job.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FoundProfile {
    /// Path relative to the instance root: `run_nvidia_gpu.bat`,
    /// `advanced\run_nvidia_gpu_disable_api_nodes.bat`.
    pub id: String,
    /// File name without the extension. Not translated: it is a file name.
    pub name: String,
    /// From the `advanced\` folder — such options are worth showing separately.
    pub advanced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Probe {
    /// The canonical path. Duplicate detection between instances goes by this,
    /// not by what the user typed.
    pub path: String,
    pub comfy_version: Option<String>,
    pub python_version: Option<String>,
    pub profiles: Vec<FoundProfile>,
}

pub trait InstanceDiscovery: Send + Sync {
    /// Checks the folder and collects everything known about it.
    ///
    /// The error must say what was missing: "the folder did not fit" without
    /// a reason leaves the user guessing, and the typical reason is that they
    /// picked a level above or below the right one.
    fn probe(&self, path: &Path) -> Result<Probe, AppError>;
}
