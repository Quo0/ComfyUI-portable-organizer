//! Спайк Фазы 1.5: годится ли `sevenz-rust2` для распаковки реального архива.
//!
//! Вопрос ровно один: LZMA2 в чистом Rust может оказаться в разы медленнее
//! 7-Zip, и тогда придётся бандлить `7za.exe` — плюс пара мегабайт
//! к инсталлятору и упоминание LGPL в лицензиях. Решать это надо замером,
//! а не ощущением.
//!
//! Заодно прототип того, что станет `installer.rs`: срез корневой папки,
//! verbatim-пути и стриминг прогресса — всё здесь уже есть.
//!
//! Запуск:
//!   cargo run --release --example spike_7z -- <архив.7z> <куда>

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use sevenz_rust2::{ArchiveReader, Password};

fn main() {
    let mut args = std::env::args().skip(1);
    let archive = args.next().expect("укажите путь к .7z");
    let dest = args.next().expect("укажите папку назначения");

    let dest = PathBuf::from(&dest);

    // Уборка — не сервисная мелочь, а часть мастера: отмена и падение обязаны
    // убрать <dest>.cpo-partial. Проверено, что обычного пути мало: и
    // `rmdir /s /q`, и `fs::remove_dir_all` спотыкаются о MAX_PATH на самых
    // глубоких файлах и возвращают «папка не пуста».
    if archive == "--clean" {
        println!("[СПАЙК] убираю {}", dest.display());
        let started = Instant::now();
        fs::remove_dir_all(verbatim(&dest)).expect("не удалось убрать папку");
        println!("[СПАЙК] убрано за {:.1} с", started.elapsed().as_secs_f32());
        return;
    }

    if dest.exists() {
        println!("[СПАЙК] чищу прежний прогон: {}", dest.display());
        fs::remove_dir_all(verbatim(&dest)).expect("не удалось убрать прежнюю папку");
    }

    // ---------------------------------------------------- разбор заголовка
    let started = Instant::now();
    let reader = ArchiveReader::open(&archive, Password::empty()).expect("архив не открылся");
    let header_secs = started.elapsed().as_secs_f32();

    let entries = reader.archive().files.clone();
    let files = entries.iter().filter(|e| !e.is_directory).count();
    let dirs = entries.len() - files;
    let total: u64 = entries.iter().map(|e| e.size).sum();
    let root = single_root(&entries);

    println!("[СПАЙК] заголовок разобран за {header_secs:.2} с");
    println!("[СПАЙК] записей: {files} файлов, {dirs} папок");
    println!("[СПАЙК] несжатый объём: {:.2} ГБ", total as f64 / 1024f64.powi(3));
    println!("[СПАЙК] корневая папка: {root:?}");
    println!(
        "[СПАЙК] самый длинный путь после среза корня: {} символов",
        entries
            .iter()
            .map(|e| strip_root(&e.name, root.as_deref()).chars().count())
            .max()
            .unwrap_or(0)
    );

    // ---------------------------------------------------- распаковка
    let mut reader = ArchiveReader::open(&archive, Password::empty()).expect("архив не открылся");

    // Третий аргумент — число потоков декодера. Вопрос закрывается замером:
    // архив собран одним блоком (Solid=+, Blocks=1), и разложить единый поток
    // LZMA2 по ядрам, скорее всего, не на чем.
    if let Some(threads) = args.next().and_then(|t| t.parse::<u32>().ok()) {
        println!("[СПАЙК] потоков декодера: {threads}");
        reader.set_thread_count(threads);
    }

    let started = Instant::now();
    let mut done: u64 = 0;
    let mut last_report = Instant::now();

    reader
        .for_each_entries(|entry, stream| {
            let rel = strip_root(&entry.name, root.as_deref());
            if rel.is_empty() {
                return Ok(true);
            }
            let target = verbatim(&dest.join(&rel));

            if entry.is_directory {
                fs::create_dir_all(&target)?;
                return Ok(true);
            }
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut file = File::create(&target)?;
            let written = io::copy(stream, &mut file)?;
            done += written;

            // Прогресс раз в секунду: 56 тысяч строк в консоли сами по себе
            // заметно тормозят прогон и искажают замер.
            if last_report.elapsed().as_secs() >= 1 {
                last_report = Instant::now();
                let pct = done as f64 / total as f64 * 100.0;
                print!("\r[СПАЙК] {pct:5.1}%  {}", rel);
                let _ = io::stdout().flush();
            }
            Ok(true)
        })
        .expect("распаковка не удалась");

    let secs = started.elapsed().as_secs_f32();
    println!(
        "\n[СПАЙК] распаковано за {secs:.1} с — {:.1} МБ/с",
        done as f64 / 1024f64.powi(2) / secs as f64
    );
}

/// Единственная корневая папка архива. Её имя задаёт пользователь,
/// поэтому из путей она срезается — заодно минус 25 символов к длине.
fn single_root(entries: &[sevenz_rust2::ArchiveEntry]) -> Option<String> {
    let mut root: Option<String> = None;
    for entry in entries {
        let first = entry.name.split(['/', '\\']).next().unwrap_or("");
        if first.is_empty() {
            return None;
        }
        match &root {
            None => root = Some(first.to_string()),
            Some(known) if known != first => return None,
            _ => {}
        }
    }
    root
}

fn strip_root<'a>(name: &'a str, root: Option<&str>) -> &'a str {
    let Some(root) = root else { return name };
    name.strip_prefix(root)
        .map(|rest| rest.trim_start_matches(['/', '\\']))
        .unwrap_or(name)
}

/// Verbatim-путь `\\?\`, снимающий лимит MAX_PATH в 260 символов.
///
/// `std::fs` сам его не добавляет: самый глубокий файл архива — 206 символов
/// относительно корня, и назначение длиннее полусотни символов ломало бы
/// распаковку без всякого предупреждения.
///
/// **Прямые слэши обязаны стать обратными.** Verbatim означает «передать ядру
/// как есть»: обычная нормализация путей отключается вместе с лимитом,
/// и `/` из имён записей архива превращает путь в невалидный — ошибка 123
/// без единого намёка на причину.
fn verbatim(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        let text = path.display().to_string();
        if text.starts_with(r"\\?\") {
            return path.to_path_buf();
        }
        // Префикс работает только с абсолютным путём без «.» и «..».
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default().join(path)
        };
        let normalized = absolute.display().to_string().replace('/', r"\");
        return PathBuf::from(format!(r"\\?\{normalized}"));
    }
    #[cfg(not(windows))]
    path.to_path_buf()
}
