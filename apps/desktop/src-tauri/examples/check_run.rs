//! Прогон жизненного цикла на стенде.
//!
//! Проверяет то, что кликами не увидишь: что лог приходит живьём, что
//! прогресс с `\r` не разрастается в сотни строк, что падение отличается
//! от штатной остановки, что порт освобождается, а зависание упирается
//! в таймаут, а не висит вечно.
//!
//! Запуск: cargo run --example check_run

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
    println!("[ПРОВЕРКА] стенд: {}", root.display());

    let probe = WindowsPortable
        .probe(&root)
        .expect("стенд не прошёл валидацию — соберите его make-fixture.mjs");

    let instance = Instance {
        id: "fixture".into(),
        name: "Стенд".into(),
        description: String::new(),
        path: probe.path.clone(),
        accent: Accent::Teal,
        preferred_port: 8231,
        comfy_version: probe.comfy_version.clone(),
        python_version: probe.python_version.clone(),
        profiles: probe.profiles.clone(),
        created_at: 0.0,
        source: None,
        shared: Default::default(),
        size_bytes: None,
        size_measured_at: None,
        available: true,
    };

    let profiles = run::profiles_of(&instance);
    println!("[ПРОВЕРКА] профилей найдено: {}", profiles.len());
    for p in &profiles {
        println!("    {} {}", p.id, if p.fallback { "(cmd /c)" } else { "" });
    }

    let mut failures = 0;
    failures += scenario_normal(&instance, &profiles);
    failures += scenario_crash(&instance, &profiles);
    failures += scenario_hang(&instance, &profiles);
    failures += scenario_restart(&instance, &profiles);

    println!();
    if failures == 0 {
        println!("[ПРОВЕРКА] всё сошлось");
    } else {
        println!("[ПРОВЕРКА] провалов: {failures}");
        std::process::exit(1);
    }
}

/// Обычный старт: готовность, живой лог, штатная остановка.
fn scenario_normal(instance: &Instance, profiles: &[cpo_desktop_lib::profiles::LaunchProfile]) -> u32 {
    println!("\n[ПРОВЕРКА] --- обычный старт");
    let profile = pick(profiles, "run_fake.bat");
    let (lines, sink) = collector();
    let (tx, rx) = mpsc::channel();

    let outcome = run::start(instance, profile, None, sink, move |exit| {
        let _ = tx.send(exit);
    })
    .expect("не запустился");

    let port = outcome.status.port.expect("порт не выдан");
    let cell = outcome.cell.clone();
    let ready = process::wait_ready(port, Duration::from_secs(60), || {
        matches!(cell.lock().unwrap().status.state, RunState::Starting)
    });

    let mut failures = 0;
    failures += check("сервер пришёл в готовность", ready.is_ok());
    if let Ok(secs) = ready {
        println!("       готов за {secs} с, порт {port}");
    }

    let snapshot = lines.lock().unwrap().clone();
    failures += check(
        "лог пришёл живьём",
        snapshot.iter().any(|l| l.text.contains("To see the GUI go to")),
    );
    failures += check(
        "старт пишется в stderr",
        snapshot.iter().any(|l| l.stream == "stderr"),
    );
    failures += check(
        "stdout тоже читается",
        snapshot.iter().any(|l| l.stream == "stdout"),
    );

    // Двести обновлений прогресса обязаны схлопнуться: строк с «Loading nodes»
    // в буфере должно остаться считанные единицы, а не двести.
    let buffered = outcome.cell.lock().unwrap().log.snapshot();
    let progress_lines = buffered.iter().filter(|l| l.text.starts_with("Loading nodes")).count();
    failures += check(
        &format!("прогресс с \\r схлопнут (строк осталось {progress_lines})"),
        progress_lines <= 2,
    );
    failures += check(
        "последнее значение прогресса сохранилось",
        buffered.iter().any(|l| l.text.contains("200/200")),
    );

    run::stop(&outcome.cell).expect("не остановился");
    let exit = rx.recv_timeout(Duration::from_secs(20));
    failures += check(
        "остановка опознана как штатная",
        matches!(exit, Ok(run::Exit::Requested)),
    );
    failures += check("порт освободился", ports::is_free(port));
    failures
}

/// Падение: процесс уходит сам, и это не должно выглядеть как остановка.
fn scenario_crash(instance: &Instance, profiles: &[cpo_desktop_lib::profiles::LaunchProfile]) -> u32 {
    println!("\n[ПРОВЕРКА] --- падение");
    let profile = pick(profiles, "run_fake_crash.bat");
    let (lines, sink) = collector();
    let (tx, rx) = mpsc::channel();

    let outcome = run::start(instance, profile, None, sink, move |exit| {
        let _ = tx.send(exit);
    })
    .expect("не запустился");

    // Ждём дольше, чем окно ожидания чужого сервера после выхода.
    let exit = rx.recv_timeout(Duration::from_secs(40));
    let mut failures = 0;
    failures += check(
        "падение опознано как падение",
        matches!(exit, Ok(run::Exit::Crashed(Some(1)))),
    );
    failures += check(
        "трейсбек попал в лог",
        lines.lock().unwrap().iter().any(|l| l.text.contains("RuntimeError")),
    );
    failures += check("порт свободен", ports::is_free(outcome.status.port.unwrap()));
    failures
}

/// Зависание: ни готовности, ни выхода. Обязан сработать таймаут.
fn scenario_hang(instance: &Instance, profiles: &[cpo_desktop_lib::profiles::LaunchProfile]) -> u32 {
    println!("\n[ПРОВЕРКА] --- зависание");
    let profile = pick(profiles, r"advanced\run_fake_hang.bat");
    let (_lines, sink) = collector();
    let (tx, _rx) = mpsc::channel();

    let outcome = run::start(instance, profile, None, sink, move |exit| {
        let _ = tx.send(exit);
    })
    .expect("не запустился");

    let port = outcome.status.port.unwrap();
    let cell = outcome.cell.clone();
    // Короткий таймаут вместо пятиминутного: проверяем механизм, а не терпение.
    let ready = process::wait_ready(port, Duration::from_secs(4), || {
        matches!(cell.lock().unwrap().status.state, RunState::Starting)
    });

    let mut failures = 0;
    failures += check("зависание упёрлось в таймаут", ready.is_err());
    if let Err(e) = &ready {
        failures += check("код ошибки — таймаут готовности", e.code == "run.readyTimeout");
    }

    run::stop(&outcome.cell).expect("не остановился");
    failures += check("зависший процесс убит", ports::is_free(port));
    failures
}

/// Самоперезапуск: процесс уходит, но порт остаётся занятым чужим сервером.
/// Так ведёт себя ComfyUI-Manager после установки нод, и путать это
/// с падением нельзя — пользователю нужны разные слова.
fn scenario_restart(instance: &Instance, profiles: &[cpo_desktop_lib::profiles::LaunchProfile]) -> u32 {
    println!("\n[ПРОВЕРКА] --- самоперезапуск");
    let profile = pick(profiles, r"advanced\run_fake_restart.bat");
    let (_lines, sink) = collector();
    let (tx, rx) = mpsc::channel();

    let outcome = run::start(instance, profile, None, sink, move |exit| {
        let _ = tx.send(exit);
    })
    .expect("не запустился");
    let port = outcome.status.port.unwrap();

    // Заглушка живёт восемь секунд, потом поднимает копию и уходит.
    let exit = rx.recv_timeout(Duration::from_secs(60));
    let mut failures = 0;
    failures += check(
        "перезапуск опознан как потеря контроля, а не падение",
        matches!(exit, Ok(run::Exit::Detached)),
    );
    failures += check("чужой сервер держит порт", !ports::is_free(port));

    // Прибираем за собой: нашего хэндла на этот процесс уже нет.
    let _ = std::process::Command::new("cmd")
        .args(["/c", "taskkill", "/F", "/IM", "python.exe"])
        .status();
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
        .unwrap_or_else(|| panic!("нет профиля {id}"))
}

fn check(what: &str, ok: bool) -> u32 {
    println!("{} {what}", if ok { "  ok " } else { "ПРОВАЛ" });
    u32::from(!ok)
}

fn fixture_root() -> PathBuf {
    let _ = HashMap::<String, String>::new();
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../tools/fixtures/fake-instance")
        .canonicalize()
        .map(|p| PathBuf::from(p.display().to_string().trim_start_matches(r"\\?\")))
        .expect("стенд не найден")
}
