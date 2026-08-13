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

    let instance = instance_at(&root, 8231);
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
    failures += scenario_odd_path();
    failures += scenario_custom_profile(&instance);
    failures += scenario_owner_of_port();

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

/// Путь с пробелом и кириллицей.
///
/// Грабля названа в плане отдельной строкой, а проверки на неё не было.
/// Ломается на ней ровно то, что не ломается больше нигде: квотирование
/// при спавне и резолв `..\` из `advanced\`.
fn scenario_odd_path() -> u32 {
    println!("\n[ПРОВЕРКА] --- путь с пробелом и кириллицей");
    let root = odd_fixture_root();
    let Some(root) = root else {
        println!("ПРОВАЛ копии стенда нет — соберите её `node tools/fixtures/make-fixture.mjs`");
        return 1;
    };
    println!("       {}", root.display());

    let instance = instance_at(&root, 8241);
    let profiles = run::profiles_of(&instance);
    let mut failures = 0;

    // Внутри `advanced\` интерпретатор указан как `..\python_embeded\...`,
    // и от корня инстанса он не нашёлся бы.
    let deep = pick(&profiles, r"advanced\run_fake_hang.bat");
    failures += check(
        "интерпретатор из advanced резолвится по пути с пробелом",
        std::path::Path::new(&deep.python_path).is_file(),
    );

    let profile = pick(&profiles, "run_fake.bat");
    let (lines, sink) = collector();
    let (tx, rx) = mpsc::channel();

    let outcome = run::start(&instance, profile, None, sink, move |exit| {
        let _ = tx.send(exit);
    })
    .expect("не запустился");

    let port = outcome.status.port.expect("порт не выдан");
    let cell = outcome.cell.clone();
    let ready = process::wait_ready(port, Duration::from_secs(60), || {
        matches!(cell.lock().unwrap().status.state, RunState::Starting)
    });

    failures += check("сборка по такому пути стартовала", ready.is_ok());
    failures += check(
        "лог читается",
        lines.lock().unwrap().iter().any(|l| l.text.contains("To see the GUI go to")),
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

/// Свой профиль: имя и аргументы свои, всё остальное — базового.
fn scenario_custom_profile(instance: &Instance) -> u32 {
    println!("\n[ПРОВЕРКА] --- свой профиль поверх .bat");
    let mut with_custom = instance.clone();
    with_custom.custom_profiles = vec![
        cpo_desktop_lib::instances::CustomProfile {
            id: "custom:1".into(),
            name: "Свой".into(),
            base_id: "run_fake.bat".into(),
            args: vec!["-s".into(), "ComfyUI\\main.py".into(), "--lowvram".into()],
        },
        // Базовый исчез: такой профиль запускать наугад нельзя.
        cpo_desktop_lib::instances::CustomProfile {
            id: "custom:2".into(),
            name: "Сирота".into(),
            base_id: "нет-такого.bat".into(),
            args: vec!["--cpu".into()],
        },
    ];

    let all = run::profiles_of(&with_custom);
    let mut failures = 0;
    let mine = all.iter().find(|p| p.id == "custom:1");
    failures += check("свой профиль появился в списке", mine.is_some());

    if let Some(mine) = mine {
        let base = pick(&run::profiles_of(instance), "run_fake.bat").clone();
        failures += check(
            "интерпретатор взят у базового",
            mine.python_path == base.python_path,
        );
        failures += check("рабочая папка взята у базового", mine.cwd == base.cwd);
        failures += check(
            "аргументы свои",
            mine.args.iter().any(|a| a == "--lowvram"),
        );
    }

    failures += check(
        "профиль с исчезнувшим базовым не подставляет чужой",
        !all.iter().any(|p| p.id == "custom:2"),
    );
    failures
}

/// Владелец порта по таблице соединений.
///
/// На этом держится переподключение к серверу, который ComfyUI-Manager
/// перезапустил сам: PID процесса мы потеряли, и взять его больше неоткуда.
/// Проверяем на себе — слушаем порт сами и ждём собственный идентификатор.
fn scenario_owner_of_port() -> u32 {
    println!("\n[ПРОВЕРКА] --- владелец порта");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("порт не занялся");
    let port = listener.local_addr().unwrap().port();

    let found = cpo_desktop_lib::supervise::windows::pid_listening_on(port);
    let mut failures = 0;
    failures += check(
        &format!("свой слушающий порт {port} опознан как наш ({found:?})"),
        found == Some(std::process::id()),
    );

    drop(listener);
    failures += check(
        "у освободившегося порта владельца нет",
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
        .unwrap_or_else(|| panic!("нет профиля {id}"))
}

fn check(what: &str, ok: bool) -> u32 {
    println!("{} {what}", if ok { "  ok " } else { "ПРОВАЛ" });
    u32::from(!ok)
}

/// Инстанс поверх любой папки стенда.
fn instance_at(root: &std::path::Path, port: u16) -> Instance {
    let probe = WindowsPortable
        .probe(root)
        .expect("стенд не прошёл валидацию — соберите его make-fixture.mjs");

    Instance {
        id: "fixture".into(),
        name: "Стенд".into(),
        description: String::new(),
        path: probe.path.clone(),
        accent: Accent::Teal,
        preferred_port: port,
        comfy_version: probe.comfy_version.clone(),
        python_version: probe.python_version.clone(),
        profiles: probe.profiles.clone(),
        created_at: 0.0,
        source: None,
        shared: Default::default(),
        custom_profiles: Vec::new(),
        size_bytes: None,
        size_measured_at: None,
        available: true,
    }
}

fn fixture_root() -> PathBuf {
    let _ = HashMap::<String, String>::new();
    fixtures().join("fake-instance").canonicalize()
        .map(strip_verbatim)
        .expect("стенд не найден")
}

/// Копия стенда по пути с пробелом и кириллицей. Её может не быть, если
/// стенд собирали до появления этой проверки.
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

/// `canonicalize` отдаёт verbatim-путь `\\?\`, показывать который нельзя.
fn strip_verbatim(p: PathBuf) -> PathBuf {
    PathBuf::from(p.display().to_string().trim_start_matches(r"\\?\"))
}
