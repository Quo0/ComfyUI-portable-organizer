//! Режим «файл в инстансе»: распознавание, бэкап чужого, восстановление.
//!
//! Самая опасная часть Фазы 2.5: здесь приложение пишет в чужую установку.
//! Обещание раздела — «не меняем чужие настройки молча» — проверяется
//! именно тут, и проверяется на настоящей файловой системе, потому что
//! ошибка выглядит как потерянный руками написанный конфиг.
//!
//! Работает во временной папке, ничего пользовательского не трогает.
//!
//! Запуск: cargo run --example check_instance_file

use std::fs;
use std::path::{Path, PathBuf};

use cpo_desktop_lib::shared_models::{self, InstanceFileState};

const FOREIGN: &str = "comfyui:\n  base_path: D:/my/models\n  checkpoints: checkpoints/\n";

fn main() {
    let root = temp_root();
    let config = shared_models::instance_config_path(&root);
    fs::create_dir_all(config.parent().unwrap()).expect("создать папку стенда");

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

    let ours = format!("{}\ncpo_shared_0:\n  base_path: D:/shared\n", shared_models::MARKER);

    // --- файла нет ---------------------------------------------------------

    check(
        "пустая папка: файл не найден",
        shared_models::inspect_instance_file(&root).state == InstanceFileState::Absent,
        String::new(),
    );

    let backup = shared_models::write_instance_file(&root, &ours, 1).expect("запись");
    check("пустая папка: копия не понадобилась", backup.is_none(), String::new());
    check("пустая папка: файл записан", config.is_file(), String::new());

    // --- файл наш ----------------------------------------------------------

    check(
        "свой файл узнан",
        shared_models::inspect_instance_file(&root).state == InstanceFileState::Ours,
        String::new(),
    );

    let updated = format!("{}\ncpo_shared_0:\n  base_path: D:/other\n", shared_models::MARKER);
    let backup = shared_models::write_instance_file(&root, &updated, 2).expect("обновление");
    check("свой файл обновлён без копии", backup.is_none(), String::new());
    check(
        "свой файл действительно перезаписан",
        fs::read_to_string(&config).unwrap().contains("D:/other"),
        String::new(),
    );

    // --- отключение: наш файл убран ---------------------------------------

    shared_models::remove_instance_file(&root).expect("удаление");
    check("отключение убрало наш файл", !config.exists(), String::new());

    // --- файл чужой --------------------------------------------------------

    fs::write(&config, FOREIGN).expect("положить чужой файл");
    check(
        "чужой файл узнан чужим",
        shared_models::inspect_instance_file(&root).state == InstanceFileState::Foreign,
        String::new(),
    );
    check(
        "содержимое чужого файла отдаётся для показа",
        shared_models::inspect_instance_file(&root).content.as_deref() == Some(FOREIGN),
        String::new(),
    );

    let backup = shared_models::write_instance_file(&root, &ours, 100).expect("замена чужого");
    let backup_path = backup.clone().expect("копия обязана быть");
    check("чужой файл заменён с копией", backup.is_some(), backup_path.clone());
    check(
        "копия содержит именно то, что лежало",
        fs::read_to_string(&backup_path).unwrap() == FOREIGN,
        String::new(),
    );

    // --- отключение: копия возвращается ------------------------------------

    shared_models::remove_instance_file(&root).expect("удаление с восстановлением");
    check(
        "прежний конфиг вернулся на место",
        fs::read_to_string(&config).ok().as_deref() == Some(FOREIGN),
        String::new(),
    );
    check("копия после восстановления убрана", !Path::new(&backup_path).exists(), String::new());

    // --- чужой файл не удаляется -------------------------------------------

    // На месте лежит чужой (только что восстановленный). Отключение обязано
    // его не тронуть: раз он не наш, значит его положили после нас.
    shared_models::remove_instance_file(&root).expect("отключение при чужом файле");
    check(
        "чужой файл при отключении не удалён",
        fs::read_to_string(&config).ok().as_deref() == Some(FOREIGN),
        String::new(),
    );

    // --- самая свежая копия ------------------------------------------------

    fs::remove_file(&config).ok();
    shared_models::write_instance_file(&root, &ours, 1).expect("запись");
    fs::write(shared_models::backup_path(&config, 5), "старая\n").expect("старая копия");
    fs::write(shared_models::backup_path(&config, 40), "свежая\n").expect("свежая копия");
    shared_models::remove_instance_file(&root).expect("удаление");
    check(
        "восстанавливается самая свежая копия",
        fs::read_to_string(&config).ok().as_deref() == Some("свежая\n"),
        fs::read_to_string(&config).unwrap_or_default().trim().to_string(),
    );

    fs::remove_dir_all(&root).ok();

    println!("\nПроверок провалено: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn temp_root() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cpo-instance-file-{}", std::process::id()));
    fs::remove_dir_all(&dir).ok();
    dir
}
