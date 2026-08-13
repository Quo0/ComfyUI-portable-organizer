//! Извлечение графа из PNG, сгенерированного ComfyUI.
//!
//! Проверяется на настоящей картинке из папки `output`, а не на собранной
//! руками: чанк туда кладёт сам ComfyUI, и придуманный образец доказывал бы
//! только то, что мы умеем читать свою же выдумку.
//!
//! Запуск: cargo run --example check_png [путь-к-png]

use std::path::PathBuf;

use cpo_desktop_lib::workflows;

/// Папка `output` рабочей сборки. Перекрывается аргументом.
const DEFAULT: &str =
    r"d:\program_files\comfyui\ComfyUI_windows_portable_nvidia\ComfyUI_windows_portable\ComfyUI\output";

fn main() {
    let mut failures = 0;

    // --- настоящая картинка ------------------------------------------------
    match real_png() {
        Some(path) => {
            println!("[ПРОВЕРКА] картинка: {}", path.display());
            let bytes = std::fs::read(&path).expect("не читается");
            let graph = workflows::workflow_from_png(&bytes);
            failures += check("граф найден в tEXt-чанке", graph.is_some(), String::new());

            if let Some(graph) = graph {
                let types = workflows::node_types(&graph);
                failures += check(
                    "найденное разбирается как воркфлоу",
                    types.is_some(),
                    String::new(),
                );
                if let Some(types) = types {
                    println!("       классов нод: {}", types.len());
                    failures += check("ноды в графе есть", !types.is_empty(), String::new());
                }
            }
        }
        None => {
            println!("[ПРОВЕРКА] настоящей картинки не нашлось — этот кусок пропущен");
            println!("           путь: {DEFAULT}");
        }
    }

    // --- то, что графом не является ---------------------------------------
    let signature = [0x89u8, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

    let mut bare = signature.to_vec();
    bare.extend_from_slice(&0u32.to_be_bytes());
    bare.extend_from_slice(b"IEND");
    bare.extend_from_slice(&[0, 0, 0, 0]);
    failures += check(
        "PNG без графа — отказ, а не выдумка",
        workflows::workflow_from_png(&bare).is_none(),
        String::new(),
    );

    failures += check(
        "не PNG вовсе — отказ",
        workflows::workflow_from_png(b"{\"nodes\":[]}").is_none(),
        String::new(),
    );

    // Обрезанный файл: длина чанка объявлена больше, чем осталось байт.
    // Наивный разбор здесь вышел бы за буфер и уронил приложение.
    let mut truncated = signature.to_vec();
    truncated.extend_from_slice(&999_999u32.to_be_bytes());
    truncated.extend_from_slice(b"tEXt");
    truncated.extend_from_slice(b"workflow\0{\"nodes\"");
    failures += check(
        "обрезанный файл не роняет разбор",
        workflows::workflow_from_png(&truncated).is_none(),
        String::new(),
    );

    // Чанк с другим ключом брать нельзя: `prompt` — это API-формат,
    // в редакторе он не открывается.
    let mut prompt_only = signature.to_vec();
    let payload = b"prompt\0{\"1\":{\"class_type\":\"KSampler\"}}";
    prompt_only.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    prompt_only.extend_from_slice(b"tEXt");
    prompt_only.extend_from_slice(payload);
    prompt_only.extend_from_slice(&[0, 0, 0, 0]);
    prompt_only.extend_from_slice(&0u32.to_be_bytes());
    prompt_only.extend_from_slice(b"IEND");
    prompt_only.extend_from_slice(&[0, 0, 0, 0]);
    failures += check(
        "чанк prompt не выдаётся за граф",
        workflows::workflow_from_png(&prompt_only).is_none(),
        String::new(),
    );

    println!("\nПроверок провалено: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn real_png() -> Option<PathBuf> {
    let dir = std::env::args().nth(1).unwrap_or_else(|| DEFAULT.to_string());
    let dir = PathBuf::from(dir);
    if dir.is_file() {
        return Some(dir);
    }
    let mut names: Vec<PathBuf> = std::fs::read_dir(&dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e.eq_ignore_ascii_case("png")).unwrap_or(false))
        .collect();
    names.sort();
    names.into_iter().next()
}

fn check(what: &str, ok: bool, detail: String) -> u32 {
    println!(
        "{} {what}{}",
        if ok { "  OK  " } else { "ПРОВАЛ" },
        if detail.is_empty() { String::new() } else { format!(" — {detail}") }
    );
    u32::from(!ok)
}
