//! A run of the whole lifecycle against the test bed.
//!
//! It checks what clicking will not show: that the log arrives live, that
//! progress with `\r` does not swell into hundreds of lines, that a crash is
//! told apart from an ordinary stop, that the port is released, and that a
//! hang runs into the timeout rather than hanging forever.
//!
//! Run: cargo run --example check_run

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

use cpo_desktop_lib::discovery::{InstanceDiscovery, windows_portable::WindowsPortable};
use cpo_desktop_lib::instances::{Accent, Instance};
use cpo_desktop_lib::process::{self, LogLine, RunState};
use cpo_desktop_lib::{ports, run};

fn main() {
    let root = fixture_root();
    println!("[CHECK] test bed: {}", root.display());

    let instance = instance_at(&root, 8231);
    let profiles = run::profiles_of(&instance);
    println!("[CHECK] profiles found: {}", profiles.len());
    for p in &profiles {
        println!("    {} {}", p.id, if p.fallback { "(cmd /c)" } else { "" });
    }

    let mut failures = 0;
    failures += scenario_normal(&instance, &profiles);
    failures += scenario_crash(&instance, &profiles);
    failures += scenario_hang(&instance, &profiles);
    failures += scenario_restart(&instance, &profiles);
    failures += scenario_odd_path();
    failures += scenario_custom_profile(&instance);
    failures += scenario_owner_of_port();

    println!();
    if failures == 0 {
        println!("[CHECK] everything matched");
    } else {
        println!("[CHECK] failures: {failures}");
        std::process::exit(1);
    }
}

/// An ordinary start: readiness, a live log, an ordinary stop.
fn scenario_normal(instance: &Instance, profiles: &[cpo_desktop_lib::profiles::LaunchProfile]) -> u32 {
    println!("\n[CHECK] --- an ordinary start");
    let profile = pick(profiles, "run_fake.bat");
    let (lines, sink) = collector();
    let (tx, rx) = mpsc::channel();

    let outcome = run::start(instance, profile, None, sink, move |exit| {
        let _ = tx.send(exit);
    })
    .expect("it did not start");

    let port = outcome.status.port.expect("no port was handed out");
    let cell = outcome.cell.clone();
    let ready = process::wait_ready(port, Duration::from_secs(60), || {
        matches!(cell.lock().unwrap().status.state, RunState::Starting)
    });

    let mut failures = 0;
    failures += check("the server came to readiness", ready.is_ok());
    if let Ok(secs) = ready {
        println!("       ready in {secs} s, port {port}");
    }

    let snapshot = lines.lock().unwrap().clone();
    failures += check(
        "the log arrived live",
        snapshot.iter().any(|l| l.text.contains("To see the GUI go to")),
    );
    failures += check(
        "the startup is written to stderr",
        snapshot.iter().any(|l| l.stream == "stderr"),
    );
    failures += check(
        "stdout is read as well",
        snapshot.iter().any(|l| l.stream == "stdout"),
    );

    // Two hundred progress updates have to collapse: only a handful of
    // "Loading nodes" lines should be left in the buffer, not two hundred.
    let buffered = outcome.cell.lock().unwrap().log.snapshot();
    let progress_lines = buffered.iter().filter(|l| l.text.starts_with("Loading nodes")).count();
    failures += check(
        &format!("progress with \\r collapsed ({progress_lines} lines left)"),
        progress_lines <= 2,
    );
    failures += check(
        "the last progress value was kept",
        buffered.iter().any(|l| l.text.contains("200/200")),
    );

    run::stop(&outcome.cell).expect("it did not stop");
    let exit = rx.recv_timeout(Duration::from_secs(20));
    failures += check(
        "the stop was recognised as an ordinary one",
        matches!(exit, Ok(run::Exit::Requested)),
    );
    failures += check("the port was released", ports::is_free(port));
    failures
}

/// A crash: the process leaves on its own, and that must not look like a stop.
fn scenario_crash(instance: &Instance, profiles: &[cpo_desktop_lib::profiles::LaunchProfile]) -> u32 {
    println!("\n[CHECK] --- a crash");
    let profile = pick(profiles, "run_fake_crash.bat");
    let (lines, sink) = collector();
    let (tx, rx) = mpsc::channel();

    let outcome = run::start(instance, profile, None, sink, move |exit| {
        let _ = tx.send(exit);
    })
    .expect("it did not start");

    // We wait longer than the window for spotting someone else's server after
    // the exit.
    let exit = rx.recv_timeout(Duration::from_secs(40));
    let mut failures = 0;
    failures += check(
        "the crash was recognised as a crash",
        matches!(exit, Ok(run::Exit::Crashed(Some(1)))),
    );
    failures += check(
        "the traceback made it into the log",
        lines.lock().unwrap().iter().any(|l| l.text.contains("RuntimeError")),
    );
    failures += check("the port is free", ports::is_free(outcome.status.port.unwrap()));
    failures
}

/// A hang: neither readiness nor an exit. The timeout has to fire.
fn scenario_hang(instance: &Instance, profiles: &[cpo_desktop_lib::profiles::LaunchProfile]) -> u32 {
    println!("\n[CHECK] --- a hang");
    let profile = pick(profiles, r"advanced\run_fake_hang.bat");
    let (_lines, sink) = collector();
    let (tx, _rx) = mpsc::channel();

    let outcome = run::start(instance, profile, None, sink, move |exit| {
        let _ = tx.send(exit);
    })
    .expect("it did not start");

    let port = outcome.status.port.unwrap();
    let cell = outcome.cell.clone();
    // A short timeout instead of the five-minute one: we are checking the
    // mechanism, not our patience.
    let ready = process::wait_ready(port, Duration::from_secs(4), || {
        matches!(cell.lock().unwrap().status.state, RunState::Starting)
    });

    let mut failures = 0;
    failures += check("the hang ran into the timeout", ready.is_err());
    if let Err(e) = &ready {
        failures += check("the error code is the readiness timeout", e.code == "run.readyTimeout");
    }

    run::stop(&outcome.cell).expect("it did not stop");
    failures += check("the hung process was killed", ports::is_free(port));
    failures
}

/// A self-restart: the process leaves, but the port stays occupied by someone
/// else's server. That is how ComfyUI-Manager behaves after installing nodes,
/// and it must not be confused with a crash — the user needs different words.
fn scenario_restart(instance: &Instance, profiles: &[cpo_desktop_lib::profiles::LaunchProfile]) -> u32 {
    println!("\n[CHECK] --- a self-restart");
    let profile = pick(profiles, r"advanced\run_fake_restart.bat");
    let (_lines, sink) = collector();
    let (tx, rx) = mpsc::channel();

    let outcome = run::start(instance, profile, None, sink, move |exit| {
        let _ = tx.send(exit);
    })
    .expect("it did not start");
    let port = outcome.status.port.unwrap();

    // The stub lives for eight seconds, then brings a copy up and leaves.
    let exit = rx.recv_timeout(Duration::from_secs(60));
    let mut failures = 0;
    failures += check(
        "the restart was recognised as a loss of control, not a crash",
        matches!(exit, Ok(run::Exit::Detached)),
    );
    failures += check("someone else's server holds the port", !ports::is_free(port));

    // Tidying up after ourselves: we no longer have a handle on that process.
    let _ = std::process::Command::new("cmd")
        .args(["/c", "taskkill", "/F", "/IM", "python.exe"])
        .status();
    failures
}

/// A path with a space and non-ASCII characters.
///
/// The trap is named in the plan on a line of its own, yet there was no check
/// for it. What breaks on it is exactly what breaks nowhere else: quoting at
/// spawn time and resolving `..\` from `advanced\`.
fn scenario_odd_path() -> u32 {
    println!("\n[CHECK] --- a path with a space and non-ASCII characters");
    let root = odd_fixture_root();
    let Some(root) = root else {
        println!(" FAIL  there is no copy of the test bed — build it with `node tools/fixtures/make-fixture.mjs`");
        return 1;
    };
    println!("       {}", root.display());

    let instance = instance_at(&root, 8241);
    let profiles = run::profiles_of(&instance);
    let mut failures = 0;

    // Inside `advanced\` the interpreter is written as
    // `..\python_embeded\...`, and it would not be found from the instance
    // root.
    let deep = pick(&profiles, r"advanced\run_fake_hang.bat");
    failures += check(
        "the interpreter from advanced resolves on a path with a space",
        std::path::Path::new(&deep.python_path).is_file(),
    );

    let profile = pick(&profiles, "run_fake.bat");
    let (lines, sink) = collector();
    let (tx, rx) = mpsc::channel();

    let outcome = run::start(&instance, profile, None, sink, move |exit| {
        let _ = tx.send(exit);
    })
    .expect("it did not start");

    let port = outcome.status.port.expect("no port was handed out");
    let cell = outcome.cell.clone();
    let ready = process::wait_ready(port, Duration::from_secs(60), || {
        matches!(cell.lock().unwrap().status.state, RunState::Starting)
    });

    failures += check("a build on such a path started", ready.is_ok());
    failures += check(
        "the log is read",
        lines.lock().unwrap().iter().any(|l| l.text.contains("To see the GUI go to")),
    );

    run::stop(&outcome.cell).expect("it did not stop");
    let exit = rx.recv_timeout(Duration::from_secs(20));
    failures += check(
        "the stop was recognised as an ordinary one",
        matches!(exit, Ok(run::Exit::Requested)),
    );
    failures += check("the port was released", ports::is_free(port));
    failures
}

/// A custom profile: the name and the arguments are its own, everything else
/// comes from the base one.
fn scenario_custom_profile(instance: &Instance) -> u32 {
    println!("\n[CHECK] --- a custom profile on top of a .bat");
    let mut with_custom = instance.clone();
    with_custom.custom_profiles = vec![
        cpo_desktop_lib::instances::CustomProfile {
            id: "custom:1".into(),
            name: "Mine".into(),
            base_id: "run_fake.bat".into(),
            args: vec!["-s".into(), "ComfyUI\\main.py".into(), "--lowvram".into()],
        },
        // The base one is gone: such a profile must not be launched at random.
        cpo_desktop_lib::instances::CustomProfile {
            id: "custom:2".into(),
            name: "Orphan".into(),
            base_id: "no-such.bat".into(),
            args: vec!["--cpu".into()],
        },
    ];

    let all = run::profiles_of(&with_custom);
    let mut failures = 0;
    let mine = all.iter().find(|p| p.id == "custom:1");
    failures += check("the custom profile appeared in the list", mine.is_some());

    if let Some(mine) = mine {
        let base = pick(&run::profiles_of(instance), "run_fake.bat").clone();
        failures += check(
            "the interpreter was taken from the base one",
            mine.python_path == base.python_path,
        );
        failures += check("the working folder was taken from the base one", mine.cwd == base.cwd);
        failures += check(
            "the arguments are its own",
            mine.args.iter().any(|a| a == "--lowvram"),
        );
    }

    failures += check(
        "a profile whose base is gone does not substitute another",
        !all.iter().any(|p| p.id == "custom:2"),
    );
    failures
}

/// The port's owner through the connection table.
///
/// Reconnecting to a server that ComfyUI-Manager restarted by itself rests on
/// this: we lost the process's PID, and there is nowhere else to get it from.
/// We check it on ourselves — we listen on a port and expect our own
/// identifier back.
fn scenario_owner_of_port() -> u32 {
    println!("\n[CHECK] --- the port's owner");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("the port was not taken");
    let port = listener.local_addr().unwrap().port();

    let found = cpo_desktop_lib::supervise::windows::pid_listening_on(port);
    let mut failures = 0;
    failures += check(
        &format!("our own listening port {port} is recognised as ours ({found:?})"),
        found == Some(std::process::id()),
    );

    drop(listener);
    failures += check(
        "a released port has no owner",
        cpo_desktop_lib::supervise::windows::pid_listening_on(port).is_none(),
    );
    failures
}

type Lines = Arc<std::sync::Mutex<Vec<LogLine>>>;

fn collector() -> (Lines, Arc<dyn Fn(LogLine) + Send + Sync>) {
    let lines: Lines = Arc::new(std::sync::Mutex::new(Vec::new()));
    let sink = lines.clone();
    (
        lines,
        Arc::new(move |line: LogLine| sink.lock().unwrap().push(line)),
    )
}

fn pick<'a>(
    profiles: &'a [cpo_desktop_lib::profiles::LaunchProfile],
    id: &str,
) -> &'a cpo_desktop_lib::profiles::LaunchProfile {
    profiles
        .iter()
        .find(|p| p.id == id)
        .unwrap_or_else(|| panic!("there is no profile {id}"))
}

fn check(what: &str, ok: bool) -> u32 {
    println!("{} {what}", if ok { "  ok  " } else { " FAIL " });
    u32::from(!ok)
}

/// An instance on top of any test-bed folder.
fn instance_at(root: &std::path::Path, port: u16) -> Instance {
    let probe = WindowsPortable
        .probe(root)
        .expect("the test bed failed validation — build it with make-fixture.mjs");

    Instance {
        id: "fixture".into(),
        name: "Fixture".into(),
        description: String::new(),
        path: probe.path.clone(),
        accent: Accent::named("teal"),
        preferred_port: port,
        comfy_version: probe.comfy_version.clone(),
        python_version: probe.python_version.clone(),
        profiles: probe.profiles.clone(),
        created_at: 0.0,
        source: None,
        shared: Default::default(),
        custom_profiles: Vec::new(),
        last_started_at: None,
        size_bytes: None,
        size_measured_at: None,
        available: true,
    }
}

fn fixture_root() -> PathBuf {
    let _ = HashMap::<String, String>::new();
    fixtures().join("fake-instance").canonicalize()
        .map(strip_verbatim)
        .expect("the test bed was not found")
}

/// A copy of the test bed on a path with a space and non-ASCII characters. It
/// may be absent if the test bed was built before this check appeared.
///
/// The folder name stays non-ASCII on purpose — that is the whole point of the
/// scenario — and it has to match what `tools/fixtures/make-fixture.mjs`
/// creates, so rename it in both places or in neither.
fn odd_fixture_root() -> Option<PathBuf> {
    fixtures()
        .join("стенд с пробелом")
        .canonicalize()
        .ok()
        .map(strip_verbatim)
}

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../tools/fixtures")
}

/// `canonicalize` hands back a verbatim `\\?\` path, which must not be shown.
fn strip_verbatim(p: PathBuf) -> PathBuf {
    PathBuf::from(p.display().to_string().trim_start_matches(r"\\?\"))
}
