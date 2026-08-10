//! Прогон мастера установки целиком, минуя интерфейс.
//!
//! Кликами такое не проверить: распаковка идёт минуты, а проверить надо
//! то, что глазами не видно — что временная папка `.cpo-partial` появляется
//! и исчезает, что переименование атомарно, что вторая цель копируется
//! с первой, а не распаковывается заново.
//!
//! Запуск:
//!   cargo run --release --example spike_install -- <архив.7z> <цель1> [цель2]

use std::path::Path;
use std::time::Instant;

use cpo_desktop_lib::installer::{self, InstallCancel, InstallTarget};
use cpo_desktop_lib::instances::Accent;

fn main() {
    let mut args = std::env::args().skip(1);
    let archive = args.next().expect("укажите путь к .7z");
    let paths: Vec<String> = args.collect();
    assert!(!paths.is_empty(), "укажите хотя бы одну цель");

    let started = Instant::now();
    let info = installer::probe_archive(&archive).expect("архив не разобрался");
    println!(
        "[СПАЙК] заголовок за {:.2} с: {} файлов, {:.2} ГБ, корень {:?}",
        started.elapsed().as_secs_f32(),
        info.files,
        info.total_uncompressed / 1024f64.powi(3),
        info.single_root
    );

    let targets: Vec<InstallTarget> = paths
        .iter()
        .enumerate()
        .map(|(i, path)| InstallTarget {
            path: path.clone(),
            name: format!("Спайк {}", i + 1),
            description: String::new(),
            accent: Accent::Teal,
            preferred_port: 8188 + i as u16,
        })
        .collect();

    // Проверки до работы: место, пустота папок, длина пути.
    for check in installer::check_targets(&info, &targets) {
        for e in &check.errors {
            println!("[СПАЙК] ОШИБКА {}: {} {:?}", check.path, e.code, e.params);
        }
        for w in &check.warnings {
            println!("[СПАЙК] предупреждение {}: {} {:?}", check.path, w.code, w.params);
        }
    }

    let cancel = InstallCancel::default();
    let started = Instant::now();
    let mut last_phase = String::new();

    let outcome = installer::run(&info, &targets, &cancel, |p| {
        let phase = format!("{:?} {}/{}", p.phase, p.target, p.targets);
        if phase != last_phase {
            last_phase = phase.clone();
            println!("\n[СПАЙК] фаза: {phase} — {}", p.target_name);
        }
        print!(
            "\r[СПАЙК] {:5.1}%  {}",
            p.done_bytes / p.total_bytes * 100.0,
            p.current
        );
    });

    println!();
    match outcome {
        Ok(()) => println!("[СПАЙК] готово за {:.1} с", started.elapsed().as_secs_f32()),
        Err(e) => {
            println!("[СПАЙК] ПРОВАЛ: {} {:?}", e.code, e.params);
            return;
        }
    }

    // Главная проверка атомарности: временных папок не осталось, а каждая
    // цель проходит валидацию инстанса.
    for path in &paths {
        let partial = format!("{path}.cpo-partial");
        println!(
            "[СПАЙК] {path}: временная папка {}, валиден {}",
            if Path::new(&partial).exists() { "ОСТАЛАСЬ" } else { "убрана" },
            valid(path)
        );
    }
}

fn valid(path: &str) -> bool {
    Path::new(path).join(r"python_embeded\python.exe").is_file()
        && Path::new(path).join(r"ComfyUI\main.py").is_file()
}
