//! Starting and stopping child processes.
//!
//! The second of the two traits where platform dependence lives. The contract
//! is deliberately narrow: everything above it knows only "spawn", "kill the
//! tree" and the guarantee that "children die with the parent".
//!
//! That guarantee is not cosmetic. Python with a model loaded holds video
//! memory, and an orphaned process left behind by an app crash makes the GPU
//! unusable until a reboot — while having no window, so the user has no way to
//! find it.

pub mod windows;

use std::collections::HashMap;
use std::process::Child;

use crate::error::AppError;

pub struct SpawnRequest {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub env: HashMap<String, String>,
}

pub trait ProcessSupervisor: Send + Sync {
    /// Spawns the process with stdout and stderr captured.
    fn spawn(&self, request: &SpawnRequest) -> Result<Child, AppError>;

    /// Kills the process together with its entire subtree.
    ///
    /// The tree, not a single process: a portable build starts python, which
    /// starts its own workers, and killing the head would leave them alive.
    fn kill_tree(&self, pid: u32) -> Result<(), AppError>;
}
