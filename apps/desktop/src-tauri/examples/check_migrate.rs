//! Проверка переноса моделей и уборки дубликатов.
//!
//! Самая опасная часть приложения: единственное место, где мы удаляем
//! файлы моделей. Проверяется на временных папках, ничего пользовательского
//! не трогается.
//!
//! Запуск: cargo run --example check_migrate

use std::fs;
use std::path::{Path, PathBuf};

use cpo_desktop_lib::migrate::{self, MigrateCancel, SameName};

fn main() {
    let root = temp("cpo-migrate");
    let models = root.join("instance").join("models");
    let shared = root.join("shared");

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

    // --- стенд --------------------------------------------------------------

    write(&models.join("checkpoints/put_checkpoints_here"), "");
    write(&models.join("checkpoints/model-a.safetensors"), &"a".repeat(4096));
    write(&models.join("loras/style.safetensors"), &"l".repeat(2048));
    // Модель каталогом, как RMBG-2.0 со снимком HuggingFace.
    write(&models.join("RMBG/RMBG-2.0/model.safetensors"), &"r".repeat(1024));
    write(&models.join("RMBG/RMBG-2.0/.cache/huggingface/CACHEDIR.TAG"), "tag");
    // Поставляется со сборкой — не наше.
    write(&models.join("configs/v1-inference.yaml"), "shipped");
    write(&models.join("custom_nodes/Manager/__init__.py"), "nodes");

    // В общей папке уже лежат три одноимённых: точный дубликат, файл того же
    // размера с иным содержимым, и каталог с тем же составом.
    write(&shared.join("checkpoints/dup.safetensors"), &"d".repeat(4096));
    write(&models.join("checkpoints/dup.safetensors"), &"d".repeat(4096));
    write(&shared.join("checkpoints/twin.safetensors"), &"x".repeat(4096));
    write(&models.join("checkpoints/twin.safetensors"), &"y".repeat(4096));
    write(&shared.join("RMBG/same-dir/f.bin"), &"z".repeat(512));
    write(&models.join("RMBG/same-dir/f.bin"), &"z".repeat(512));

    // --- что видно в скане --------------------------------------------------

    let scan = migrate::scan(&models, &shared);
    let cat = |name: &str| scan.categories.iter().find(|c| c.folder == name);
    let entry = |c: &str, n: &str| {
        cat(c).and_then(|x| x.entries.iter().find(|e| e.name == n)).cloned()
    };

    check("папка моделей прочитана", scan.available, String::new());
    check(
        "configs не предлагается к переносу",
        cat("configs").is_none(),
        String::new(),
    );
    check(
        "custom_nodes не предлагается к переносу",
        cat("custom_nodes").is_none(),
        String::new(),
    );
    check(
        "маркер put_..._here пропущен",
        entry("checkpoints", "put_checkpoints_here").is_none(),
        String::new(),
    );
    check(
        "обычная модель видна",
        entry("checkpoints", "model-a.safetensors").is_some(),
        String::new(),
    );
    check(
        "модель-каталог видна одним элементом",
        entry("RMBG", "RMBG-2.0").map(|e| e.is_dir) == Some(true),
        String::new(),
    );
    check(
        "объём каталога посчитан вместе с .cache",
        entry("RMBG", "RMBG-2.0").map(|e| e.files) == Some(2),
        format!("{:?}", entry("RMBG", "RMBG-2.0").map(|e| e.files)),
    );

    // --- сравнение: три случая ----------------------------------------------

    check(
        "точная копия опознана дубликатом",
        entry("checkpoints", "dup.safetensors").and_then(|e| e.same_name)
            == Some(SameName::Duplicate),
        format!("{:?}", entry("checkpoints", "dup.safetensors").and_then(|e| e.same_name)),
    );
    // Ровно тот случай, ради которого читаются края: размер совпал,
    // содержимое иное.
    check(
        "тот же размер, другое содержимое — разные",
        entry("checkpoints", "twin.safetensors").and_then(|e| e.same_name)
            == Some(SameName::Different),
        format!("{:?}", entry("checkpoints", "twin.safetensors").and_then(|e| e.same_name)),
    );
    check(
        "каталог того же состава — похоже на дубликат",
        entry("RMBG", "same-dir").and_then(|e| e.same_name) == Some(SameName::LikelyDuplicate),
        format!("{:?}", entry("RMBG", "same-dir").and_then(|e| e.same_name)),
    );
    check(
        "разный размер — разные",
        {
            write(&shared.join("loras/style.safetensors"), "короче");
            migrate::compare(
                &models.join("loras/style.safetensors"),
                &shared.join("loras/style.safetensors"),
            ) == SameName::Different
        },
        String::new(),
    );

    // --- перенос ------------------------------------------------------------

    let cancel = MigrateCancel::default();
    let out = migrate::move_all(
        &models,
        &shared,
        &["checkpoints".into(), "RMBG".into(), "loras".into()],
        &cancel,
        |_| {},
    );

    check(
        "свободное имя перенесено",
        !models.join("checkpoints/model-a.safetensors").exists()
            && shared.join("checkpoints/model-a.safetensors").is_file(),
        String::new(),
    );
    check(
        "каталог перенесён целиком, вместе с .cache",
        shared.join("RMBG/RMBG-2.0/.cache/huggingface/CACHEDIR.TAG").is_file()
            && !models.join("RMBG/RMBG-2.0").exists(),
        String::new(),
    );
    check(
        "занятые имена не тронуты в сборке",
        models.join("checkpoints/dup.safetensors").is_file()
            && models.join("checkpoints/twin.safetensors").is_file(),
        String::new(),
    );
    check(
        "чужой файл в общей папке не перезаписан",
        fs::read_to_string(shared.join("checkpoints/twin.safetensors")).unwrap()
            == "x".repeat(4096),
        String::new(),
    );
    check(
        "пропущенные перечислены с вердиктом",
        out.skipped.len() == 4,
        format!("{:?}", out.skipped.iter().map(|s| &s.name).collect::<Vec<_>>()),
    );
    check("сбоев не было", out.failed.is_empty(), format!("{:?}", out.failed));
    check(
        "маркер остался в сборке",
        models.join("checkpoints/put_checkpoints_here").is_file(),
        String::new(),
    );
    check(
        "configs не тронут",
        models.join("configs/v1-inference.yaml").is_file()
            && !shared.join("configs").exists(),
        String::new(),
    );
    check(
        "custom_nodes не тронут",
        models.join("custom_nodes/Manager/__init__.py").is_file()
            && !shared.join("custom_nodes").exists(),
        String::new(),
    );
    check(
        "временных .cpo-partial не осталось",
        !has_partial(&shared),
        String::new(),
    );

    // --- уборка дубликатов --------------------------------------------------

    // Зовём со списком, где есть заведомо разный файл: команда обязана
    // отказать по нему сама, не полагаясь на добросовестность вызывающего.
    let cleanup = migrate::remove_duplicates(
        &models,
        &shared,
        &[
            ("checkpoints".into(), "dup.safetensors".into()),
            ("checkpoints".into(), "twin.safetensors".into()),
            ("RMBG".into(), "same-dir".into()),
        ],
    );

    check(
        "дубликат убран",
        !models.join("checkpoints/dup.safetensors").exists(),
        String::new(),
    );
    check(
        "каталог-дубликат убран",
        !models.join("RMBG/same-dir").exists(),
        String::new(),
    );
    check(
        "РАЗНЫЙ файл с тем же именем НЕ тронут",
        models.join("checkpoints/twin.safetensors").is_file(),
        String::new(),
    );
    check("отказ учтён в отчёте", cleanup.refused == 1, format!("{}", cleanup.refused));
    check(
        "убранное осталось в общей папке",
        shared.join("checkpoints/dup.safetensors").is_file()
            && shared.join("RMBG/same-dir/f.bin").is_file(),
        String::new(),
    );
    check(
        "освобождённый объём посчитан",
        cleanup.freed_bytes > 0.0,
        format!("{}", cleanup.freed_bytes),
    );

    // --- отмена -------------------------------------------------------------

    let models2 = root.join("i2").join("models");
    let shared2 = root.join("s2");
    write(&models2.join("loras/one.safetensors"), &"1".repeat(512));
    write(&models2.join("loras/two.safetensors"), &"2".repeat(512));

    let stop = MigrateCancel::default();
    stop.cancel();
    let cancelled = migrate::move_all(&models2, &shared2, &["loras".into()], &stop, |_| {});
    check("отмена помечена в отчёте", cancelled.cancelled, String::new());
    check(
        "при отмене исходники на месте",
        models2.join("loras/one.safetensors").is_file()
            && models2.join("loras/two.safetensors").is_file(),
        String::new(),
    );

    fs::remove_dir_all(&root).ok();

    println!("\nПроверок провалено: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn write(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().unwrap()).expect("папка");
    fs::write(path, content).expect("файл");
}

fn has_partial(root: &Path) -> bool {
    let Ok(entries) = fs::read_dir(root) else { return false };
    entries.flatten().any(|e| {
        let name = e.file_name().to_string_lossy().to_string();
        name.contains("cpo-partial") || has_partial(&e.path())
    })
}

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    fs::remove_dir_all(&dir).ok();
    dir
}
