//! Проверка клиента API против работающего ComfyUI.
//!
//! Проверяется **наш** код, а не поведение ComfyUI: те же функции, которыми
//! пользуется приложение, ходят на настоящий сервер. Скрипт-обёртка
//! `tools/check-workflows-live.mjs` поднимает сборку и передаёт сюда порт.
//!
//! Запуск: cargo run --example check_comfy_live -- <порт>

use cpo_desktop_lib::comfy_api::{Client, UploadOutcome};
use cpo_desktop_lib::workflows;

const WORKFLOW: &str = r#"{"nodes":[{"id":1,"type":"KSampler"},{"id":2,"type":"SaveImage"}],"links":[],"version":0.4}"#;
const NAME: &str = "cpo-live-check.json";

fn main() {
    let port: u16 = std::env::args()
        .nth(1)
        .and_then(|p| p.parse().ok())
        .expect("укажите порт: cargo run --example check_comfy_live -- 8189");

    let client = Client::new(port);
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

    // --- листинг ------------------------------------------------------------

    // В свежей сборке папки workflows нет вовсе, и ComfyUI отвечает 404.
    // Для нас это пустая библиотека, а не ошибка.
    let before = client.list_workflows();
    check(
        "листинг отработал (404 «папки нет» — тоже успех)",
        before.is_ok(),
        format!("{:?}", before.as_ref().err()),
    );
    let before = before.unwrap_or_default();

    // --- заливка ------------------------------------------------------------

    let first = client.upload_workflow(NAME, WORKFLOW, false);
    check(
        "воркфлоу залит",
        matches!(first, Ok(UploadOutcome::Written)),
        format!("{first:?}"),
    );

    let after = client.list_workflows().unwrap_or_default();
    check(
        "залитый воркфлоу появился в листинге",
        after.iter().any(|f| f.path.ends_with(NAME)),
        format!("{:?}", after.iter().map(|f| &f.path).collect::<Vec<_>>()),
    );
    check(
        "листинг вырос ровно на один",
        after.len() == before.len() + 1,
        format!("было {}, стало {}", before.len(), after.len()),
    );

    // --- конфликт имён ------------------------------------------------------

    // Тот же файл, overwrite=false. Молчаливой перезаписи быть не должно:
    // это единственный механизм, которым мы защищаем чужой воркфлоу.
    let second = client.upload_workflow(NAME, WORKFLOW, false);
    check(
        "повторная заливка даёт конфликт, а не тихую перезапись",
        matches!(second, Ok(UploadOutcome::Conflict)),
        format!("{second:?}"),
    );

    let forced = client.upload_workflow(NAME, WORKFLOW, true);
    check(
        "с разрешением перезаписи заливка проходит",
        matches!(forced, Ok(UploadOutcome::Written)),
        format!("{forced:?}"),
    );

    // --- чтение -------------------------------------------------------------

    let read = client.read_workflow(NAME);
    check(
        "воркфлоу читается обратно байт в байт",
        read.as_deref().ok() == Some(WORKFLOW),
        format!("{:?}", read.as_ref().map(|s| s.len())),
    );

    // --- ноды ---------------------------------------------------------------

    let keys = client.object_info_keys();
    check("object_info получен", keys.is_ok(), format!("{:?}", keys.as_ref().err()));
    let keys = keys.unwrap_or_default();
    check("классов нод пришло много", keys.len() > 100, format!("{}", keys.len()));
    check("базовые классы на месте", keys.contains("KSampler"), String::new());

    let nodes = workflows::node_types(WORKFLOW).unwrap_or_default();
    check(
        "у воркфлоу на стоковых нодах недостающих нод нет",
        workflows::missing_nodes(&nodes, &keys).is_empty(),
        String::new(),
    );

    let invented = vec!["KSampler".to_string(), "СовершенноВыдуманнаяНода".to_string()];
    check(
        "выдуманная нода опознана как недостающая",
        workflows::missing_nodes(&invented, &keys) == vec!["СовершенноВыдуманнаяНода".to_string()],
        String::new(),
    );

    println!("\nПроверок провалено: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
