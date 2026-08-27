//! The supervisor implementation for Windows.
//!
//! The key part is a Job Object with `KILL_ON_JOB_CLOSE`. The app puts
//! **itself** into the job once at startup, and job membership is inherited by
//! descendants. So no special handling is needed for each child process: it
//! lands in the same job automatically, and closing the last handle — that is,
//! the death of our process, orderly or not — takes everyone with it.
//!
//! The plan called for `CREATE_SUSPENDED` → `AssignProcessToJobObject` →
//! `ResumeThread` for every child. That route had to be abandoned for
//! a specific reason: `std::process::Command` does not hand out the main
//! thread's handle, and without it there is nothing to resume. Building our own
//! `CreateProcessW` with all the pipe plumbing just for that would mean
//! rewriting half of `std::process` — while job inheritance gives exactly the
//! same guarantee.

use std::collections::HashMap;
use std::process::{Child, Command, Stdio};

use crate::error::AppError;

use super::{ProcessSupervisor, SpawnRequest};

/// Hides the child process's console window. Without it a black terminal would
/// pop up over the UI on every launch.
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub struct WindowsSupervisor;

impl ProcessSupervisor for WindowsSupervisor {
    fn spawn(&self, request: &SpawnRequest) -> Result<Child, AppError> {
        let mut cmd = Command::new(&request.program);
        cmd.args(&request.args)
            .current_dir(&request.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            // stdin is closed deliberately: `pause` at the end of a .bat and any
            // input() inside would otherwise hang waiting for a key there is
            // nowhere to press.
            .stdin(Stdio::null());

        for (key, value) in &request.env {
            cmd.env(key, value);
        }
        apply_python_env(&mut cmd, &request.env);

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        cmd.spawn()
            .map_err(|e| AppError::because("run.spawnFailed", e))
    }

    fn kill_tree(&self, pid: u32) -> Result<(), AppError> {
        // On Windows you cannot send SIGINT to someone else's process, and
        // `Child::kill` kills only the head of the tree. taskkill /T walks the
        // whole subtree.
        let mut cmd = Command::new("taskkill");
        cmd.args(["/PID", &pid.to_string(), "/T", "/F"])
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        match cmd.status() {
            // 128 — the process is already gone. Not an error: goal achieved.
            Ok(status) if status.success() || status.code() == Some(128) => Ok(()),
            Ok(status) => Err(AppError::with(
                "run.stopFailed",
                "reason",
                format!("taskkill returned {}", status.code().unwrap_or(-1)),
            )),
            Err(e) => Err(AppError::because("run.stopFailed", e)),
        }
    }
}

/// The variables without which the log arrives in one lump at the end.
///
/// Verified by the Phase 0 spike: without `PYTHONUNBUFFERED`, stdout redirected
/// into a pipe is buffered in blocks, and the first minutes of startup look
/// like a hang. User `set` lines from the `.bat` are not overwritten — if
/// someone set their own, they had a reason.
fn apply_python_env(cmd: &mut Command, existing: &HashMap<String, String>) {
    if !existing.contains_key("PYTHONUNBUFFERED") {
        cmd.env("PYTHONUNBUFFERED", "1");
    }
    if !existing.contains_key("PYTHONIOENCODING") {
        cmd.env("PYTHONIOENCODING", "utf-8");
    }
}

/// Puts the current process into a job its descendants cannot escape.
///
/// Called once at app startup. The job handle is deliberately "leaked": while
/// it is open the job is alive, and it closes together with the process — which
/// is exactly the moment the system kills all descendants.
#[cfg(windows)]
pub fn install_job_object() -> Result<(), String> {
    use std::mem::{size_of, zeroed};
    use windows_sys::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject,
        JobObjectExtendedLimitInformation, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    };
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    // SAFETY: every call is plain Win32 with no lifetime trickery; the struct
    // is zero-initialised, as the documentation requires.
    unsafe {
        let job = CreateJobObjectW(std::ptr::null(), std::ptr::null());
        if job.is_null() {
            return Err("CreateJobObjectW returned null".into());
        }

        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = zeroed();
        info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

        let ok = SetInformationJobObject(
            job,
            JobObjectExtendedLimitInformation,
            std::ptr::addr_of!(info).cast(),
            size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        );
        if ok == 0 {
            return Err("SetInformationJobObject failed".into());
        }

        if AssignProcessToJobObject(job, GetCurrentProcess()) == 0 {
            return Err("AssignProcessToJobObject failed".into());
        }
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn install_job_object() -> Result<(), String> {
    Ok(())
}

/// Who is listening on this port on the loopback interface.
///
/// Needed in exactly one case: after installing nodes, ComfyUI-Manager shuts
/// the server down and brings up a new process. Our handle is lost in the
/// process, the port stays taken, and we do not know the PID — meaning there is
/// nothing left to stop the build with from the app.
///
/// The table is fetched whole and once: there are hundreds of connections on
/// a machine, but the call costs tens of microseconds and has to be made once
/// per reconnection.
#[cfg(windows)]
pub fn pid_listening_on(port: u16) -> Option<u32> {
    use std::mem::size_of;
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        GetExtendedTcpTable, MIB_TCPTABLE_OWNER_PID, TCP_TABLE_OWNER_PID_LISTENER,
    };
    use windows_sys::Win32::Networking::WinSock::AF_INET;

    // SAFETY: the buffer is allocated for the size the system itself reported,
    // and exactly as many rows are read as it declared in dwNumEntries.
    unsafe {
        let mut size: u32 = 0;
        // The first call only learns the required size and therefore must
        // return a buffer-overflow error.
        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut size,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_LISTENER,
            0,
        );
        if size == 0 {
            return None;
        }

        let mut buffer = vec![0u8; size as usize];
        let rc = GetExtendedTcpTable(
            buffer.as_mut_ptr().cast(),
            &mut size,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_LISTENER,
            0,
        );
        if rc != 0 {
            return None;
        }

        let table = &*(buffer.as_ptr() as *const MIB_TCPTABLE_OWNER_PID);
        let rows = std::slice::from_raw_parts(
            buffer
                .as_ptr()
                .add(size_of::<u32>())
                .cast::<windows_sys::Win32::NetworkManagement::IpHelper::MIB_TCPROW_OWNER_PID>(),
            table.dwNumEntries as usize,
        );

        rows.iter()
            .find(|row| local_port(row.dwLocalPort) == port)
            .map(|row| row.dwOwningPid)
    }
}

/// In the table the port sits in network byte order, and in a 32-bit field.
#[cfg(windows)]
fn local_port(raw: u32) -> u16 {
    u16::from_be((raw & 0xFFFF) as u16)
}

#[cfg(not(windows))]
pub fn pid_listening_on(_port: u16) -> Option<u32> {
    None
}
