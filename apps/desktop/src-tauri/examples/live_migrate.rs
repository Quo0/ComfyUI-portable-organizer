//! Прогон переноса по указанным папкам — тем же кодом, что и приложение.
//!
//! Нужен, чтобы проверить перенос на настоящей сборке, а не только
//! на временных папках: там свои маркеры, свой `configs` и категории,
//! заведённые кастомными нодами.
//!
//! Запуск: cargo run --example live_migrate -- <папка-моделей> <общий-корень>

use std::path::Path;

use cpo_desktop_lib::migrate::{self, MigrateCancel};

fn main() {
    let mut args = std::env::args().skip(1);
    let models = args.next().expect("укажите папку моделей");
    let shared = args.next().expect("укажите общий корень");
    let (models, shared) = (Path::new(&models), Path::new(&shared));

    let scan = migrate::scan(models, shared);
    println!("категорий к переносу: {}", scan.categories.len());
    println!("поедет файлов: {}, объём: {:.0}", scan.total_files, scan.total_bytes);

    for category in &scan.categories {
        let busy: Vec<_> = category
            .entries
            .iter()
            .filter(|e| e.same_name.is_some())
            .map(|e| format!("{} ({:?})", e.name, e.same_name.unwrap()))
            .collect();
        if !busy.is_empty() {
            println!("  {}: занятые имена — {}", category.folder, busy.join(", "));
        }
    }

    // Переносим только checkpoints: проверка не должна двигать чужие
    // двадцать гигабайт ради нескольких килобайт смысла. Перечень —
    // парами «категория и модель», как их шлёт экран.
    let offer: Vec<(String, String)> = scan
        .categories
        .iter()
        .filter(|c| c.folder == "checkpoints")
        .flat_map(|c| c.entries.iter().map(|e| (c.folder.clone(), e.name.clone())))
        .collect();
    let outcome = migrate::move_all(models, shared, &offer, &MigrateCancel::default(), |p| {
        println!("  {} / {} — {}/{}", p.done, p.total, p.category, p.name)
    });

    println!("перенесено: {:?}", outcome.moved);
    println!("пропущено: {:?}", outcome.skipped.iter().map(|s| (&s.name, s.verdict)).collect::<Vec<_>>());
    println!("сбоев: {:?}", outcome.failed);
}
