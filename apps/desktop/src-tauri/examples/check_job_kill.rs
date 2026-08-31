//! The Job Object must not take the update installer with it.
//!
//! The updater plugin launches the NSIS installer with `ShellExecuteW` and
//! calls `std::process::exit(0)` on the very next line. Our installer asks for
//! no elevation, so `ShellExecuteW` creates it as our own child, and the child
//! inherits our job. Our exit closes the last handle to that job, and
//! `KILL_ON_JOB_CLOSE` kills the installer microseconds after it started: the
//! window disappears, no installer UI ever appears, the old version stays.
//!
//! `release_job_object` clears the job's limits from the plugin's
//! `on_before_exit` hook — this check proves that it does the job, and that
//! without it the process really is killed.
//!
//! The check re-executes itself, because the process under test has to die for
//! any of this to mean anything: the driver spawns an inner copy, the inner
//! copy puts itself in a job, starts a delayed marker writer through
//! `ShellExecuteW` and exits. Whether the marker appears is the answer.
//!
//! `ShellExecuteW` rather than `Command::spawn` is deliberate: it also proves
//! the assumption the whole diagnosis rests on — that ShellExecute creates the
//! process itself instead of handing it to the shell, which is what makes the
//! job inherit at all.
//!
//! A console window flashes twice while it runs. That is the marker writer,
//! launched the same way the installer is.
//!
//! Run: cargo run --example check_job_kill

#[cfg(windows)]
fn main() {
    use std::process::Command;
    use std::time::Duration;

    // Longer than the marker writer's delay, so that the marker — if anyone is
    // still alive to write it — has certainly appeared by the time we look.
    const WAIT: Duration = Duration::from_secs(6);

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--stage") {
        inner(&args);
        return;
    }

    let exe = std::env::current_exe().expect("the path to ourselves");
    let mut failures = 0;

    for (mode, want_marker, what) in [
        (
            "keep",
            false,
            "with the limits kept the child is killed by our exit",
        ),
        (
            "release",
            true,
            "after release_job_object the child survives",
        ),
    ] {
        let marker = std::env::temp_dir().join(format!("cpo-check-job-kill-{mode}.txt"));
        let _ = std::fs::remove_file(&marker);

        let status = Command::new(&exe)
            .args(["--stage", "inner", "--mode", mode, "--marker"])
            .arg(&marker)
            .status()
            .expect("the inner stage starts");
        assert!(status.success(), "the inner stage exited with {status}");

        std::thread::sleep(WAIT);

        let got = marker.exists();
        let ok = got == want_marker;
        println!(
            "{} {what} — marker {}",
            if ok { "  OK  " } else { " FAIL " },
            if got { "written" } else { "absent" }
        );
        failures += u32::from(!ok);

        let _ = std::fs::remove_file(&marker);
    }

    println!("\nChecks failed: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

/// The stage that dies: the same sequence the updater plugin performs.
#[cfg(windows)]
fn inner(args: &[String]) {
    use cpo_desktop_lib::supervise::windows::{install_job_object, release_job_object};

    let value = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .unwrap_or_else(|| panic!("{name} without a value"))
            .clone()
    };
    let mode = value("--mode");
    let marker = value("--marker");

    install_job_object().expect("the job is created");

    // `ping` rather than `timeout`: timeout refuses to run whenever stdin is
    // redirected, and here it is not worth depending on who owns the console.
    // Four pings ≈ three seconds — long enough that the marker is written after
    // we are already gone.
    shell_execute(
        "cmd.exe",
        &format!("/c ping -n 4 127.0.0.1 >nul & echo ok > \"{marker}\""),
    );

    if mode == "release" {
        release_job_object().expect("the limits are cleared");
    }

    // The line the plugin has here, and the reason for the whole check.
    std::process::exit(0);
}

/// `ShellExecuteW` with `SW_SHOW` — exactly the call from `updater.rs`.
#[cfg(windows)]
fn shell_execute(file: &str, parameters: &str) {
    use windows_sys::Win32::UI::Shell::ShellExecuteW;
    use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOW;

    let wide = |s: &str| s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
    let operation = wide("open");
    let file = wide(file);
    let parameters = wide(parameters);

    // SAFETY: all four strings are NUL-terminated and outlive the call; the
    // return value is an error code disguised as a handle and is deliberately
    // ignored — a failure to start shows up as an absent marker anyway.
    unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            parameters.as_ptr(),
            std::ptr::null(),
            SW_SHOW,
        )
    };
}

#[cfg(not(windows))]
fn main() {
    println!("  SKIP  Job Objects exist only on Windows");
}
