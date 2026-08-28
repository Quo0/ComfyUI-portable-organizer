//! Assembling a launch: profile → port → process → readiness.
//!
//! The state machine lives here too: `Stopped → Starting → Running → Stopping
//! → Stopped`, plus two side outcomes — `Crashed`, when the process left on
//! its own, and `Detached`, when the server on the port is alive but no longer
//! the one we started.

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::error::AppError;
use crate::instances::Instance;
use crate::ports;
use crate::process::{
    self, LogLine, RunState, RunStatus, Running,
};
use crate::profiles::{self, LaunchProfile};
use crate::supervise::{windows::WindowsSupervisor, ProcessSupervisor, SpawnRequest};

/// An instance's profiles: those parsed from `.bat` plus those assembled by
/// the user.
///
/// The parse is redone every time rather than taken from the registry: the
/// user may have edited the `.bat` by hand, and showing them a stale parse
/// would be lying.
///
/// A custom profile stores only a name and arguments, taking everything else
/// from the base one — here and now, for the same reason.
pub fn profiles_of(instance: &Instance) -> Vec<LaunchProfile> {
    let root = Path::new(&instance.path);
    let base: Vec<LaunchProfile> = instance
        .profiles
        .iter()
        .map(|found| profiles::parse_bat(root, &found.id, found.advanced))
        .collect();

    let mut all = base.clone();
    for custom in &instance.custom_profiles {
        // The base `.bat` may have disappeared along with a build update.
        // Silently substituting another one is not allowed: what launches
        // would not be what was asked for.
        let Some(source) = base.iter().find(|p| p.id == custom.base_id) else {
            continue;
        };
        all.push(LaunchProfile {
            id: custom.id.clone(),
            name: custom.name.clone(),
            args: custom.args.clone(),
            ..source.clone()
        });
    }
    all
}

/// What to do next once the process has finished.
pub enum Exit {
    /// We asked it to stop ourselves.
    Requested,
    /// It left on its own, and the server on the port does not answer.
    Crashed(Option<i32>),
    /// It left on its own, but someone is holding the port: almost certainly
    /// ComfyUI-Manager brought the server back up after installing nodes.
    Detached,
}

pub struct StartOutcome {
    pub status: RunStatus,
    pub cell: Arc<Mutex<Running>>,
}

/// Prepares the command and starts the process.
///
/// Returns right after the spawn: waiting for readiness here is not allowed,
/// or the caller sees not one line of the log until the cold start is over.
/// `shared_config` is the path to our `extra_model_paths.yaml` for the "flag"
/// mode. In the "file inside the instance" mode there is none: the file
/// already sits in the build folder and ComfyUI picks it up by itself.
pub fn start(
    instance: &Instance,
    profile: &LaunchProfile,
    shared_config: Option<&str>,
    on_line: Arc<dyn Fn(LogLine) + Send + Sync>,
    on_exit: impl FnOnce(Exit) + Send + 'static,
) -> Result<StartOutcome, AppError> {
    let port = ports::pick(instance.preferred_port)
        .ok_or_else(|| AppError::new("run.noFreePort"))?;

    let args = profiles::apply_runtime_args(&profile.args, port, shared_config);
    let request = SpawnRequest {
        program: profile.python_path.clone(),
        args,
        cwd: profile.cwd.clone(),
        env: profile.env.clone(),
    };

    let mut child = WindowsSupervisor.spawn(&request)?;
    let pid = child.id();

    let status = RunStatus {
        instance_id: instance.id.clone(),
        state: RunState::Starting,
        port: Some(port),
        pid: Some(pid),
        started_at: Some(process::now_ms()),
        ready_secs: None,
        exit_code: None,
        profile_id: Some(profile.id.clone()),
    };

    let cell = Arc::new(Mutex::new(Running {
        status: status.clone(),
        log: process::LogBuffer::default(),
        stopping: false,
    }));

    // ComfyUI writes the bulk of its startup to stderr rather than stdout, so
    // we read both streams — each in its own thread, otherwise one waits for
    // the other and the log goes out of order.
    if let Some(stream) = child.stdout.take() {
        spawn_pump("stdout", stream, cell.clone(), on_line.clone());
    }
    if let Some(stream) = child.stderr.take() {
        spawn_pump("stderr", stream, cell.clone(), on_line.clone());
    }

    // Waiting for exit goes in its own thread: `wait` blocks, and we have to
    // learn about a crash immediately, not when the user presses "Stop".
    let watch = cell.clone();
    std::thread::spawn(move || {
        let code = child.wait().ok().and_then(|s| s.code());
        let requested = watch.lock().unwrap().stopping;
        let exit = if requested {
            Exit::Requested
        } else if process::detached_after_exit(port) {
            Exit::Detached
        } else {
            Exit::Crashed(code)
        };
        on_exit(exit);
    });

    Ok(StartOutcome { status, cell })
}

/// Starts a thread reading one of the process's streams.
fn spawn_pump<R: std::io::Read + Send + 'static>(
    name: &'static str,
    stream: R,
    cell: Arc<Mutex<Running>>,
    sink: Arc<dyn Fn(LogLine) + Send + Sync>,
) {
    std::thread::spawn(move || {
        process::pump(stream, |text, replaces| {
            let line = process::push_line(&cell, name, text, replaces);
            sink(line);
        });
    });
}

/// Stops the instance and waits for the port to be released.
///
/// The wait is mandatory: the process manages to die before the system lets
/// go of its socket, and an immediate restart runs into its own previous port.
pub fn stop(cell: &Arc<Mutex<Running>>) -> Result<(), AppError> {
    let (pid, port) = {
        let mut running = cell.lock().unwrap();
        running.stopping = true;
        running.status.state = RunState::Stopping;
        (running.status.pid, running.status.port)
    };

    let Some(pid) = pid else {
        return Err(AppError::new("run.notRunning"));
    };
    WindowsSupervisor.kill_tree(pid)?;

    if let Some(port) = port {
        ports::wait_released(port, Duration::from_secs(10));
    }
    Ok(())
}
