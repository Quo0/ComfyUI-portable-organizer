//! Проверка одного утверждения: переживает ли `fs::rename` существующую
//! папку назначения.
//!
//! Мастер установки распаковывает во временную `<dest>.cpo-partial`
//! и в конце переименовывает её. Проверки при этом осознанно разрешают
//! **пустую существующую** папку назначения — и вот на ней всё и ломалось.
//!
//! Запуск: cargo run --example check_rename

use std::fs;

fn main() {
    let base = std::env::temp_dir().join("cpo-rename-check");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(base.join("src")).expect("не создалась исходная папка");
    fs::write(base.join("src").join("file.txt"), b"x").expect("не записался файл");

    // Случай первый: назначения нет.
    let missing = base.join("missing");
    let first = fs::rename(base.join("src"), &missing);
    println!("назначения нет      -> {:?}", first.as_ref().map(|_| "ok"));

    // Случай второй: назначение есть и пусто — ровно то, что делает
    // пользователь, создавая папку заранее.
    fs::create_dir_all(base.join("src2")).expect("не создалась исходная папка");
    fs::write(base.join("src2").join("file.txt"), b"x").expect("не записался файл");
    let existing = base.join("existing");
    fs::create_dir_all(&existing).expect("не создалась папка назначения");
    let second = fs::rename(base.join("src2"), &existing);
    println!(
        "назначение есть, пусто -> {}",
        match &second {
            Ok(()) => "ok".to_string(),
            Err(e) => format!("ОШИБКА {e}"),
        }
    );

    let _ = fs::remove_dir_all(&base);

    if second.is_err() {
        println!("\nПодтверждено: rename на существующую папку не работает,");
        println!("её нужно снимать до переименования.");
    } else {
        println!("\nrename справился сам — снятие папки в installer.rs избыточно.");
    }
}
