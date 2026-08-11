//! Проверка разбора `.bat` на стенде.
//!
//! Это тесты, вынесенные в пример по вынужденной причине: `cargo test`
//! в этом крейте не запускается — тестовый бинарь падает при загрузке
//! с STATUS_ENTRYPOINT_NOT_FOUND, и снятие `cdylib` из crate-type
//! не помогает. Ограничение записано в `plan/notes/phase-0-spike.md`; когда оно снимется,
//! проверки переедут в `#[cfg(test)]` без изменений по существу.
//!
//! Запуск: cargo run --example check_profiles

use std::path::PathBuf;

use cpo_desktop_lib::profiles::{apply_runtime_args, parse_bat};

fn main() {
    let fixture = fixture_root();
    println!("[ПРОВЕРКА] стенд: {}", fixture.display());
    assert!(
        fixture.join(r"python_embeded\python.exe").is_file(),
        "стенд не собран: запустите node tools/fixtures/make-fixture.mjs"
    );

    let mut failures = 0;

    // Обычный однострочник из корня.
    let normal = parse_bat(&fixture, "run_fake.bat", false);
    failures += check("run_fake: разобран", !normal.fallback);
    failures += check(
        "run_fake: интерпретатор из корня",
        normal.python_path.ends_with(r"fake-instance\python_embeded\python.exe"),
    );
    failures += check(
        "run_fake: аргументы целиком",
        normal.args
            == vec![
                "-s",
                r"ComfyUI\main.py",
                "--windows-standalone-build",
                "--cpo-mode",
                "normal",
            ],
    );
    failures += check(
        "run_fake: рабочая папка — корень",
        PathBuf::from(&normal.cwd) == fixture,
    );

    // Кавычки вокруг путей и комментарий `::`.
    let quoted = parse_bat(&fixture, "run_fake_crash.bat", false);
    failures += check("run_fake_crash: разобран", !quoted.fallback);
    failures += check(
        "run_fake_crash: кавычки сняты",
        quoted.args.contains(&r"ComfyUI\main.py".to_string()),
    );

    // `advanced\` с путями через `..\` — ради этого случая всё и затевалось.
    let advanced = parse_bat(&fixture, r"advanced\run_fake_hang.bat", true);
    failures += check("advanced: разобран", !advanced.fallback);
    failures += check(
        "advanced: ..\\ схлопнут до корня инстанса",
        advanced.python_path.ends_with(r"fake-instance\python_embeded\python.exe")
            && !advanced.python_path.contains(".."),
    );
    failures += check(
        "advanced: рабочая папка — сам advanced",
        PathBuf::from(&advanced.cwd) == fixture.join("advanced"),
    );

    // Запуск через переменную: разобрать нельзя, откат обязателен.
    let via_var = parse_bat(&fixture, "run_fake_via_var.bat", false);
    failures += check("via_var: откатился на cmd /c", via_var.fallback);
    failures += check("via_var: команда — cmd", via_var.python_path == "cmd");
    failures += check(
        "via_var: set собран в env",
        via_var.env.get("CPO_FIXTURE").map(String::as_str) == Some("1"),
    );

    // Мутация аргументов перед стартом.
    let mutated = apply_runtime_args(&normal.args, 8231, None);
    failures += check(
        "порт дописан",
        mutated.windows(2).any(|w| w[0] == "--port" && w[1] == "8231"),
    );
    failures += check(
        "браузер запрещён",
        mutated.iter().filter(|a| *a == "--disable-auto-launch").count() == 1,
    );
    failures += check(
        "без общих моделей флага конфига нет",
        !mutated.iter().any(|a| a == "--extra-model-paths-config"),
    );

    let already = vec![
        "-s".to_string(),
        "main.py".to_string(),
        "--port".to_string(),
        "8188".to_string(),
        "--disable-auto-launch".to_string(),
    ];
    failures += check(
        "прежний --port вырезан вместе со значением",
        apply_runtime_args(&already, 8300, None)
            == vec!["-s", "main.py", "--port", "8300", "--disable-auto-launch"],
    );

    // Общие модели: свой конфиг дописывается отдельным вхождением флага.
    let shared = apply_runtime_args(&already, 8300, Some(r"C:\data\shared-models.yaml"));
    failures += check(
        "путь конфига идёт сразу за своим флагом",
        shared
            .windows(2)
            .any(|w| w[0] == "--extra-model-paths-config" && w[1] == r"C:\data\shared-models.yaml"),
    );

    // У флага `action='append'`, файлы применяются подряд. Вырезать чужой
    // значило бы молча отобрать настройку, заведённую руками.
    let with_own = vec![
        "-s".to_string(),
        "main.py".to_string(),
        "--extra-model-paths-config".to_string(),
        "my_paths.yaml".to_string(),
    ];
    let merged = apply_runtime_args(&with_own, 8300, Some("ours.yaml"));
    failures += check(
        "чужой --extra-model-paths-config сохранён",
        merged.iter().any(|a| a == "my_paths.yaml"),
    );
    failures += check(
        "флаг конфига встречается дважды, а не склеен в одно вхождение",
        merged.iter().filter(|a| *a == "--extra-model-paths-config").count() == 2,
    );
    // Свой путь обязан идти следом за своим же флагом. Приписанный к чужому
    // вхождению он тоже загрузился бы, но при отсутствии флага в `.bat` стал
    // бы позиционным аргументом и уронил бы разбор всей командной строки.
    failures += check(
        "свой путь идёт следом за своим же флагом",
        merged.iter().position(|a| a == "ours.yaml")
            == merged.iter().rposition(|a| a == "--extra-model-paths-config").map(|i| i + 1),
    );

    println!();
    if failures == 0 {
        println!("[ПРОВЕРКА] всё сошлось");
    } else {
        println!("[ПРОВЕРКА] провалов: {failures}");
        std::process::exit(1);
    }
}

fn check(what: &str, ok: bool) -> u32 {
    println!("{} {what}", if ok { "  ok " } else { "ПРОВАЛ" });
    u32::from(!ok)
}

/// Стенд лежит рядом с исходниками, а не там, откуда запущен пример.
fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../tools/fixtures/fake-instance")
        .canonicalize()
        .map(|p| PathBuf::from(p.display().to_string().trim_start_matches(r"\\?\")))
        .expect("стенд не найден")
}
