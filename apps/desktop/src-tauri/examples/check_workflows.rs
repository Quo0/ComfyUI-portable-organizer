//! Проверка библиотеки воркфлоу и резолва папки инстанса.
//!
//! В `examples/`, а не в `#[cfg(test)]`, по известной причине: `cargo test`
//! в этом крейте падает на загрузке образа (`plan/notes/phase-25-shared-models.md`).
//!
//! Стенд: node tools/fixtures/make-workflow-library.mjs
//!
//! Запуск: cargo run --example check_workflows

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use cpo_desktop_lib::profiles::{self, LaunchProfile};
use cpo_desktop_lib::workflows;

fn main() {
    let root = fixture();
    if !root.is_dir() {
        eprintln!("Стенда нет: {}", root.display());
        eprintln!("Соберите его: node tools/fixtures/make-workflow-library.mjs");
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

    let scan = workflows::scan_library(&root);
    let find = |path: &str| scan.items.iter().find(|i| i.path == path);

    check("библиотека прочитана", scan.available, String::new());
    check("манифест разобрался", !scan.manifest_broken, String::new());

    // --- что попало в список, а что нет -----------------------------------

    check(
        "посторонний файл не показан воркфлоу",
        !scan.items.iter().any(|i| i.path.ends_with(".txt")),
        String::new(),
    );
    check(
        "сам манифест в список не попал",
        find(workflows::MANIFEST).is_none(),
        String::new(),
    );
    check(
        "вложенная подпапка прочитана",
        find("flux/portrait-v3.json").is_some(),
        String::new(),
    );
    check(
        "путь записан прямыми слэшами",
        scan.items.iter().all(|i| !i.path.contains('\\')),
        String::new(),
    );

    // --- слияние с манифестом ---------------------------------------------

    let basic = find("basic-txt2img.json");
    check(
        "теги из манифеста подхвачены",
        basic.map(|i| i.meta.tags.len()) == Some(2),
        format!("{:?}", basic.map(|i| i.meta.tags.clone())),
    );
    check("избранное подхвачено", basic.map(|i| i.meta.favorite) == Some(true), String::new());

    // Файл лежит в папке, записи о нём в манифесте нет. Это норма,
    // а не ошибка: положить воркфлоу через проводник — законный сценарий.
    let orphan = find("sdxl/base-upscale.json");
    check("файл без записи в манифесте показан", orphan.is_some(), String::new());
    check(
        "у файла без записи пустые теги, и это не ошибка",
        orphan.map(|i| i.meta.tags.is_empty() && !i.meta.favorite) == Some(true),
        String::new(),
    );

    // Запись есть, файла нет.
    let lost = find("lost/deleted.json");
    check("запись без файла помечена потерянной", lost.map(|i| i.lost) == Some(true), String::new());
    check(
        "у потерянной записи сохранена заметка",
        lost.map(|i| !i.meta.note.is_empty()) == Some(true),
        String::new(),
    );
    check(
        "потерянная запись не притворяется целым файлом",
        lost.map(|i| i.size_bytes == 0.0 && i.nodes.is_empty()) == Some(true),
        String::new(),
    );

    // --- разбор графа ------------------------------------------------------

    check(
        "ноды базового воркфлоу разобраны",
        basic.map(|i| i.nodes.len()) == Some(6),
        format!("{:?}", basic.map(|i| i.nodes.clone())),
    );
    check(
        "повторы классов схлопнуты в набор",
        workflows::node_types(r#"{"nodes":[{"type":"KSampler"},{"type":"KSampler"}]}"#)
            == Some(vec!["KSampler".to_string()]),
        String::new(),
    );
    check(
        "битый JSON помечен, но список не уронил",
        find("broken.json").map(|i| i.broken) == Some(true),
        String::new(),
    );
    check(
        "JSON без nodes — не воркфлоу",
        workflows::node_types(r#"{"hello":"world"}"#).is_none(),
        String::new(),
    );
    check(
        "не-JSON — не воркфлоу",
        workflows::node_types("не json вовсе").is_none(),
        String::new(),
    );

    // --- дифф нод ----------------------------------------------------------

    let available: BTreeSet<String> = [
        "CheckpointLoaderSimple",
        "CLIPTextEncode",
        "EmptyLatentImage",
        "KSampler",
        "VAEDecode",
        "SaveImage",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    check(
        "у базового воркфлоу не хватает ничего",
        workflows::missing_nodes(&basic.map(|i| i.nodes.clone()).unwrap_or_default(), &available)
            .is_empty(),
        String::new(),
    );

    let custom = find("flux/portrait-v3.json").map(|i| i.nodes.clone()).unwrap_or_default();
    let missing = workflows::missing_nodes(&custom, &available);
    check(
        "у воркфлоу с кастомными нодами не хватает ровно двух",
        missing.len() == 2,
        format!("{missing:?}"),
    );
    check(
        "список недостающих отсортирован",
        missing.windows(2).all(|w| w[0] <= w[1]),
        format!("{missing:?}"),
    );

    // --- битый манифест не уносит файлы ------------------------------------

    let broken_root = temp_dir("cpo-wf-broken");
    std::fs::create_dir_all(&broken_root).ok();
    std::fs::write(broken_root.join("a.json"), r#"{"nodes":[{"type":"KSampler"}]}"#).ok();
    std::fs::write(broken_root.join(workflows::MANIFEST), "{ это не json").ok();
    let broken = workflows::scan_library(&broken_root);
    check("битый манифест помечен", broken.manifest_broken, String::new());
    check(
        "битый манифест не унёс воркфлоу",
        broken.items.len() == 1,
        format!("{}", broken.items.len()),
    );
    std::fs::remove_dir_all(&broken_root).ok();

    // --- недоступная библиотека -------------------------------------------

    let missing_root = workflows::scan_library(&root.join("нет-такой-папки"));
    check("несуществующая библиотека помечена недоступной", !missing_root.available, String::new());
    check("недоступная библиотека не роняет сканер", missing_root.items.is_empty(), String::new());

    // --- резолв папки воркфлоу инстанса ------------------------------------

    let instance = Path::new(r"D:\builds\comfy");
    let plain = profile(vec!["-s", "ComfyUI\\main.py"], r"D:\builds\comfy");
    check(
        "без флага берётся ComfyUI\\user\\default\\workflows",
        profiles::workflows_dir(&plain, instance)
            == instance.join("ComfyUI").join("user").join("default").join("workflows"),
        profiles::workflows_dir(&plain, instance).display().to_string(),
    );

    let moved = profile(
        vec!["-s", "ComfyUI\\main.py", "--user-directory", r"E:\comfy-user"],
        r"D:\builds\comfy",
    );
    check(
        "--user-directory уважается",
        profiles::workflows_dir(&moved, instance)
            == PathBuf::from(r"E:\comfy-user\default\workflows"),
        profiles::workflows_dir(&moved, instance).display().to_string(),
    );

    let joined = profile(
        vec!["-s", "ComfyUI\\main.py", "--user-directory=E:\\comfy-user"],
        r"D:\builds\comfy",
    );
    check(
        "форма через знак равенства тоже уважается",
        profiles::workflows_dir(&joined, instance)
            == PathBuf::from(r"E:\comfy-user\default\workflows"),
        profiles::workflows_dir(&joined, instance).display().to_string(),
    );

    // Относительный путь считается от рабочей папки, а она — директория
    // `.bat`, ровно как при запуске двойным кликом.
    let relative = profile(
        vec!["-s", "ComfyUI\\main.py", "--user-directory", r"..\shared-user"],
        r"D:\builds\comfy\advanced",
    );
    check(
        "относительный путь считается от рабочей папки профиля",
        profiles::workflows_dir(&relative, instance)
            == PathBuf::from(r"D:\builds\comfy\shared-user\default\workflows"),
        profiles::workflows_dir(&relative, instance).display().to_string(),
    );

    println!("\nПроверок провалено: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn profile(args: Vec<&str>, cwd: &str) -> LaunchProfile {
    LaunchProfile {
        id: "run.bat".into(),
        name: "run".into(),
        advanced: false,
        python_path: r"D:\builds\comfy\python_embeded\python.exe".into(),
        args: args.into_iter().map(String::from).collect(),
        cwd: cwd.into(),
        env: Default::default(),
        fallback: false,
    }
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    dir
}

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("tools")
        .join("fixtures")
        .join("workflow-library")
}
