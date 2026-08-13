//! Отчёт о дубликатах на временных папках.
//!
//! Главное здесь — не «нашлись ли дубли», а что **не** попало в отчёт:
//! одноимённые файлы разного размера не должны считаться дублями и не
//! должны входить в сумму потерь. Ошибка в эту сторону превратила бы
//! отчёт в основание удалить не то.
//!
//! Запуск: cargo run --example check_duplicates

use std::fs;
use std::path::{Path, PathBuf};

use cpo_desktop_lib::duplicates::{self, Place, ScanCancel};

fn main() {
    let root = temp_dir("cpo-dups");
    let a = root.join("сборка A");
    let b = root.join("build B");
    let c = root.join("shared");

    // Один и тот же чекпоинт в трёх местах — настоящий дубль.
    put(&a.join("checkpoints/sdxl.safetensors"), 4096);
    put(&b.join("checkpoints/sdxl.safetensors"), 4096);
    put(&c.join("checkpoints/sdxl.safetensors"), 4096);

    // Одно имя, разные размеры: разные модели, которым не повезло с именем.
    put(&a.join("loras/style.safetensors"), 1000);
    put(&b.join("loras/style.safetensors"), 2000);

    // Одно имя в разных категориях — это разные роли, не дубль.
    put(&a.join("vae/thing.pt"), 500);
    put(&b.join("upscale_models/thing.pt"), 500);

    // Маркеры пустых категорий есть в каждой сборке. Формально совпадают
    // идеально, по сути — шум нулевого размера.
    put(&a.join("embeddings/put_embeddings_here"), 0);
    put(&b.join("embeddings/put_embeddings_here"), 0);

    // configs поставляется вместе со сборкой и совпадает у всех.
    put(&a.join("configs/v1-inference.yaml"), 300);
    put(&b.join("configs/v1-inference.yaml"), 300);

    // Модель каталогом: считается целиком, как и при переносе.
    put(&a.join("RMBG/RMBG-2.0/model.safetensors"), 700);
    put(&a.join("RMBG/RMBG-2.0/config.json"), 100);
    put(&b.join("RMBG/RMBG-2.0/model.safetensors"), 700);
    put(&b.join("RMBG/RMBG-2.0/config.json"), 100);

    let places = vec![
        Place { name: "сборка A".into(), models_dir: a.clone() },
        Place { name: "build B".into(), models_dir: b.clone() },
        Place { name: "общая папка".into(), models_dir: c.clone() },
        Place { name: "пропавшая".into(), models_dir: root.join("нет-такой") },
    ];

    // Счётчик через ячейку: сканер принимает `Fn`, потому что то же
    // замыкание в приложении шлёт событие и состояния не держит.
    let ticks = std::cell::Cell::new(0u32);
    let report = duplicates::scan(&places, &ScanCancel::default(), |_| {
        ticks.set(ticks.get() + 1)
    });
    let ticks = ticks.get();

    let mut failures = 0;
    let dup = |name: &str| report.duplicates.iter().find(|g| g.name == name);

    failures += check(
        "чекпоинт в трёх местах опознан дублем",
        dup("sdxl.safetensors").map(|g| g.copies.len()) == Some(3),
        format!("{:?}", dup("sdxl.safetensors").map(|g| g.copies.len())),
    );
    failures += check(
        "впустую посчитано за вычетом одной копии",
        dup("sdxl.safetensors").map(|g| g.wasted_bytes) == Some(8192.0),
        format!("{:?}", dup("sdxl.safetensors").map(|g| g.wasted_bytes)),
    );
    failures += check(
        "каталог-модель посчитан целиком",
        dup("RMBG-2.0").map(|g| g.wasted_bytes) == Some(800.0),
        format!("{:?}", dup("RMBG-2.0").map(|g| g.wasted_bytes)),
    );

    failures += check(
        "разные размеры при одном имени — не дубль",
        dup("style.safetensors").is_none(),
        String::new(),
    );
    failures += check(
        "и попали в отдельный список",
        report.name_clashes.iter().any(|g| g.name == "style.safetensors"),
        String::new(),
    );
    failures += check(
        "в сумму потерь они не входят",
        report.wasted_bytes == 8192.0 + 800.0,
        format!("{}", report.wasted_bytes),
    );

    failures += check(
        "одно имя в разных категориях дублем не считается",
        !report.duplicates.iter().any(|g| g.name == "thing.pt"),
        String::new(),
    );
    failures += check(
        "маркеры put_..._here пропущены",
        !report.duplicates.iter().any(|g| g.name.starts_with("put_")),
        String::new(),
    );
    failures += check(
        "configs в отчёт не идёт",
        !report.duplicates.iter().any(|g| g.category == "configs"),
        String::new(),
    );

    failures += check(
        "недоступная папка названа в пропущенных",
        report.skipped.contains(&"пропавшая".to_string()),
        format!("{:?}", report.skipped),
    );
    failures += check(
        "остальные места обойдены",
        report.scanned_places == 3,
        format!("{}", report.scanned_places),
    );
    failures += check("прогресс приходил", ticks >= 4, format!("{ticks}"));
    failures += check("отчёт не помечен прерванным", !report.cancelled, String::new());

    // Самое дорогое сверху: с него пользователь и начнёт.
    failures += check(
        "группы отсортированы по потерям",
        report.duplicates.first().map(|g| g.name.clone()) == Some("sdxl.safetensors".into()),
        String::new(),
    );

    // --- отмена -----------------------------------------------------------
    let cancel = ScanCancel::default();
    cancel.cancel();
    let stopped = duplicates::scan(&places, &cancel, |_| {});
    failures += check("отмена помечена в отчёте", stopped.cancelled, String::new());
    failures += check(
        "прерванный обход ничего не насчитал",
        stopped.duplicates.is_empty(),
        String::new(),
    );

    // --- ни один файл не тронут -------------------------------------------
    failures += check(
        "все файлы на месте: отчёт ничего не удаляет",
        a.join("checkpoints/sdxl.safetensors").is_file()
            && b.join("checkpoints/sdxl.safetensors").is_file()
            && c.join("checkpoints/sdxl.safetensors").is_file()
            && a.join("loras/style.safetensors").is_file(),
        String::new(),
    );

    fs::remove_dir_all(&root).ok();

    println!("\nПроверок провалено: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn check(what: &str, ok: bool, detail: String) -> u32 {
    println!(
        "{} {what}{}",
        if ok { "  OK  " } else { "ПРОВАЛ" },
        if detail.is_empty() { String::new() } else { format!(" — {detail}") }
    );
    u32::from(!ok)
}

fn put(path: &Path, size: usize) {
    fs::create_dir_all(path.parent().expect("есть родитель")).ok();
    fs::write(path, vec![b'x'; size]).expect("файл не записался");
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    fs::remove_dir_all(&dir).ok();
    dir
}
