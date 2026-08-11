//! Проверка сканера общей папки и генератора YAML.
//!
//! Живёт в `examples/`, а не в `#[cfg(test)]`, по той же причине, что
//! `check_profiles`: `cargo test` в этом крейте падает при загрузке
//! тестового бинаря с `STATUS_ENTRYPOINT_NOT_FOUND`. Ограничение записано
//! в `plan/notes/phase-0-spike.md`.
//!
//! Стенд собирается `node tools/fixtures/make-shared-root.mjs`.
//!
//! Запуск: cargo run --example check_shared

use std::path::PathBuf;

use cpo_desktop_lib::shared_models::{
    self, ApplyMode, CategoryStatus, InstanceShared, SharedSettings,
};

fn main() {
    let root = fixture();
    if !root.is_dir() {
        eprintln!("Стенда нет: {}", root.display());
        eprintln!("Соберите его: node tools/fixtures/make-shared-root.mjs");
        std::process::exit(1);
    }

    let mut failures = 0;
    let mut check = |name: &str, ok: bool, detail: String| {
        println!("{} {name}{}", if ok { "  OK  " } else { "ПРОВАЛ" }, if detail.is_empty() {
            String::new()
        } else {
            format!(" — {detail}")
        });
        if !ok {
            failures += 1;
        }
    };

    let scan = shared_models::scan_root(&root);

    check("корень доступен", scan.available, String::new());

    let find = |folder: &str| scan.categories.iter().find(|c| c.folder == folder);

    // --- распознавание ----------------------------------------------------

    let checkpoints = find("checkpoints");
    check(
        "checkpoints распознан",
        checkpoints.map(|c| c.status) == Some(CategoryStatus::Recognized),
        format!("{:?}", checkpoints.map(|c| c.status)),
    );
    check(
        "checkpoints посчитал файлы",
        checkpoints.map(|c| c.files) == Some(2),
        format!("{:?}", checkpoints.map(|c| c.files)),
    );

    // --- map_legacy -------------------------------------------------------

    check(
        "unet уезжает под ключом diffusion_models",
        find("unet").and_then(|c| c.key.as_deref()) == Some("diffusion_models"),
        format!("{:?}", find("unet").and_then(|c| c.key.clone())),
    );
    check(
        "clip уезжает под ключом text_encoders",
        find("clip").and_then(|c| c.key.as_deref()) == Some("text_encoders"),
        format!("{:?}", find("clip").and_then(|c| c.key.clone())),
    );

    // --- чёрный список ----------------------------------------------------

    let blocked = find("custom_nodes");
    check(
        "custom_nodes помечен исключённым",
        blocked.map(|c| c.status) == Some(CategoryStatus::Blocked),
        format!("{:?}", blocked.map(|c| c.status)),
    );
    check(
        "у custom_nodes нет ключа",
        blocked.map(|c| c.key.is_none()) == Some(true),
        String::new(),
    );

    // --- нераспознанное ---------------------------------------------------

    check(
        "папка с произвольным именем помечена нераспознанной",
        find("my notes").map(|c| c.status) == Some(CategoryStatus::Unknown),
        format!("{:?}", find("my notes").map(|c| c.status)),
    );
    check(
        "нераспознанная папка всё равно получает ключ",
        find("my notes").and_then(|c| c.key.as_deref()) == Some("my notes"),
        String::new(),
    );

    // --- файл в корне не категория ---------------------------------------

    check(
        "README.txt в корне не принят за категорию",
        find("README.txt").is_none(),
        String::new(),
    );

    // --- предложение недостающих ------------------------------------------

    check(
        "не предлагает завести diffusion_models при наличии unet",
        !scan.missing.iter().any(|m| m == "diffusion_models"),
        format!("{:?}", scan.missing),
    );

    // --- объём не считает исключённое -------------------------------------

    let blocked_files = blocked.map(|c| c.files).unwrap_or(0);
    check(
        "файлы из чёрного списка не входят в общий счёт",
        scan.total_files
            == scan
                .categories
                .iter()
                .filter(|c| c.status != CategoryStatus::Blocked)
                .map(|c| c.files)
                .sum::<u32>()
            && blocked_files > 0,
        format!("всего {}, в custom_nodes {}", scan.total_files, blocked_files),
    );

    // --- YAML -------------------------------------------------------------

    let yaml = shared_models::render_yaml(&[(&scan, "shared")], true);
    println!("\n--- сгенерированный YAML ---\n{yaml}---\n");

    check("есть маркер своего файла", shared_models::is_ours(&yaml), String::new());
    check(
        "чужой файл своим не считается",
        !shared_models::is_ours("comfyui:\n  base_path: D:/models\n"),
        String::new(),
    );
    check("имя секции своё", yaml.contains("cpo_shared_0:"), String::new());
    check("is_default проброшен", yaml.contains("is_default: true"), String::new());
    let base_path = yaml
        .lines()
        .find(|l| l.trim_start().starts_with("base_path:"))
        .unwrap_or_default();
    check("base_path прямыми слэшами", !base_path.contains('\\'), base_path.to_string());
    // Путь уезжает в конфиг чужого приложения и в интерфейс: `..` в середине
    // и работает хуже, и читается как недоделка.
    check("base_path без `..`", !base_path.contains(".."), base_path.to_string());
    check(
        "custom_nodes не попал в YAML ни под каким видом",
        !yaml.contains("custom_nodes"),
        String::new(),
    );

    // Самая тонкая ветвь: две папки обязаны съехаться в один ключ
    // многострочным блоком, иначе вторая молча потеряется.
    let merged_ok = yaml.contains("  diffusion_models: |\n    diffusion_models/\n    unet/\n");
    check("diffusion_models и unet слиты в один ключ", merged_ok, String::new());
    // Порядок внутри ключа — это порядок поиска: `add_model_folder_path`
    // складывает пути подряд. Каноническая папка обязана идти первой,
    // иначе приоритет определялся бы алфавитом, то есть случайно.
    check(
        "внутри ключа каноническая папка впереди устаревшей",
        yaml.contains("  text_encoders: |\n    text_encoders/\n    clip/\n"),
        String::new(),
    );

    let single_ok = yaml.contains("  checkpoints: checkpoints/\n");
    check("одиночная категория пишется строкой", single_ok, String::new());

    check(
        "ключ text_encoders встречается ровно один раз",
        yaml.matches("  text_encoders:").count() == 1,
        format!("{}", yaml.matches("  text_encoders:").count()),
    );

    // --- значения по умолчанию -------------------------------------------

    check(
        "по умолчанию скачиваем в общую папку",
        SharedSettings::default().make_default_target,
        String::new(),
    );
    check(
        "по умолчанию режим не трогает папку инстанса",
        InstanceShared::default().apply_mode == ApplyMode::Flag,
        String::new(),
    );
    check(
        "по умолчанию инстанс не подключён",
        !InstanceShared::default().enabled,
        String::new(),
    );

    // --- недоступный корень -----------------------------------------------

    let missing = shared_models::scan_root(&root.join("нет-такой-папки"));
    check("несуществующий корень помечен недоступным", !missing.available, String::new());
    check("недоступный корень не роняет сканер", missing.categories.is_empty(), String::new());

    println!("\nПроверок провалено: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn fixture() -> PathBuf {
    // От `src-tauri` до корня репозитория — два уровня.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("tools")
        .join("fixtures")
        .join("shared-models")
}
