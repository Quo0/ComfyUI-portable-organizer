//! Профили запуска: разбор `.bat` и подготовка команды.
//!
//! Портабл-сборка запускается однострочником вида
//! `.\python_embeded\python.exe -s ComfyUI\main.py --windows-standalone-build`,
//! обвешанным `echo` и `pause`. Мы вытаскиваем из него интерпретатор
//! и аргументы, чтобы запускать python напрямую: через `cmd /c` мы теряем
//! настоящий PID и вместе с ним возможность остановить сервер.
//!
//! Если разобрать не удалось — не выдумываем, а честно откатываемся
//! на `cmd /c <файл>`. Сборка запустится, просто управлять ею будет хуже.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Строки, которые в однострочнике запуска не значат ничего.
const NOISE: [&str; 10] = [
    "echo", "pause", "rem", "cls", "title", "color", "chcp", "setlocal", "endlocal", "exit",
];

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LaunchProfile {
    /// Путь `.bat` относительно корня инстанса. Он же идентификатор.
    pub id: String,
    /// Имя файла без расширения. Не переводится.
    pub name: String,
    pub advanced: bool,
    /// Абсолютный путь к интерпретатору.
    pub python_path: String,
    pub args: Vec<String>,
    /// Рабочая папка — директория самого `.bat`. Именно она подставляется,
    /// когда файл запускают двойным кликом, и от неё считаются `..\` внутри.
    pub cwd: String,
    pub env: HashMap<String, String>,
    /// Разобрать не удалось, запуск пойдёт через `cmd /c`. В интерфейсе
    /// это повод предупредить: остановка такого процесса менее надёжна.
    pub fallback: bool,
}

/// Разбирает `.bat` в профиль запуска.
///
/// `root` — корень инстанса, `rel` — путь файла относительно него.
pub fn parse_bat(root: &Path, rel: &str, advanced: bool) -> LaunchProfile {
    let file = root.join(rel);
    let dir = file.parent().unwrap_or(root).to_path_buf();
    let name = Path::new(rel)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(rel)
        .to_string();

    let fallback = |env: HashMap<String, String>| LaunchProfile {
        id: rel.to_string(),
        name: name.clone(),
        advanced,
        python_path: "cmd".to_string(),
        args: vec!["/c".to_string(), file.display().to_string()],
        cwd: dir.display().to_string(),
        env,
        fallback: true,
    };

    // Кодировка `.bat` не определена: файлы бывают в cp866 из-за русских
    // комментариев. Читаем лениво — интересующие нас строки всё равно ASCII.
    let Ok(bytes) = std::fs::read(&file) else {
        return fallback(HashMap::new());
    };
    let text = String::from_utf8_lossy(&bytes);

    let mut env = HashMap::new();
    let mut command: Option<String> = None;

    for raw in text.lines() {
        let line = raw.trim().trim_start_matches('@').trim();
        if line.is_empty() || line.starts_with("::") {
            continue;
        }
        let first = line.split_whitespace().next().unwrap_or("").to_lowercase();
        if NOISE.contains(&first.as_str()) {
            continue;
        }
        if first == "set" {
            if let Some((key, value)) = line[3..].trim().split_once('=') {
                env.insert(key.trim().to_string(), value.trim().to_string());
            }
            continue;
        }
        // Строка запуска опознаётся по имени интерпретатора. Запуск через
        // переменную (`%PY% -s ...`) сюда не попадёт — и это правильно:
        // раскрывать произвольный batch мы не беремся.
        if line.to_lowercase().contains("python.exe") {
            command = Some(line.to_string());
            break;
        }
    }

    let Some(command) = command else {
        return fallback(env);
    };

    let mut tokens = tokenize(&command).into_iter();
    let Some(interpreter) = tokens.next() else {
        return fallback(env);
    };

    let python = resolve(&dir, &expand(&interpreter, &env));
    if !python.is_file() {
        return fallback(env);
    }

    LaunchProfile {
        id: rel.to_string(),
        name,
        advanced,
        python_path: python.display().to_string(),
        // Аргументы оставляем как есть: относительные пути внутри них
        // считаются от рабочей папки, а она у нас — директория `.bat`.
        args: tokens.map(|t| expand(&t, &env)).collect(),
        cwd: dir.display().to_string(),
        env,
        fallback: false,
    }
}

/// Значение флага в аргументах профиля, в обеих формах записи.
///
/// Последнее вхождение побеждает — так же ведёт себя argparse.
fn flag_value(profile: &LaunchProfile, name: &str) -> Option<String> {
    let joined = format!("{name}=");
    let mut found = None;
    let mut args = profile.args.iter();

    while let Some(arg) = args.next() {
        if arg == name {
            found = args.next().cloned();
        } else if let Some(value) = arg.strip_prefix(&joined) {
            found = Some(value.to_string());
        }
    }
    found
}

/// Корень, от которого ComfyUI считает свои папки.
///
/// `--base-directory` (`comfy/cli_args.py:70`) переносит разом модели,
/// `custom_nodes`, `input`, `output`, `temp` и `user`; без него корнем
/// служит папка самого ComfyUI (`folder_paths.py:16-18`).
fn base_dir(profile: &LaunchProfile, instance_root: &Path) -> PathBuf {
    match flag_value(profile, "--base-directory") {
        Some(value) => resolve(Path::new(&profile.cwd), &value),
        None => instance_root.join("ComfyUI"),
    }
}

/// Где эта сборка хранит свои воркфлоу.
///
/// **Предполагать путь нельзя,** и флагов тут два, выстроенных цепочкой:
/// `--user-directory` (`cli_args.py:254`) бьёт `--base-directory`, а тот
/// задаёт корень, от которого считается `user/` (`folder_paths.py:72`).
///
/// Второе звено этой цепочки я в Фазе 2.6 пропустил: разбирался только
/// `--user-directory`, и сборка, запущенная с одним `--base-directory`,
/// хранила воркфлоу не там, где мы их искали.
///
/// Относительный путь во флаге считается от рабочей папки, а она у нас —
/// директория `.bat`, ровно как при запуске двойным кликом.
///
/// Папки может не существовать: ComfyUI создаёт её лениво, при первом
/// сохранении. Проверять существование здесь не наша забота — вызывающий
/// либо создаёт дерево, либо считает пустым.
pub fn workflows_dir(profile: &LaunchProfile, instance_root: &Path) -> PathBuf {
    let user = match flag_value(profile, "--user-directory") {
        Some(value) => resolve(Path::new(&profile.cwd), &value),
        None => base_dir(profile, instance_root).join("user"),
    };

    // `default` — публичная папка пользователя по умолчанию
    // (`app/user_manager.py:79`). Многопользовательский режим ComfyUI
    // мы не поддерживаем и не притворяемся, что поддерживаем.
    user.join("default").join("workflows")
}

/// Где эта сборка хранит свои модели.
///
/// Та же цепочка, что у воркфлоу, и в том же порядке
/// (`folder_paths.py:20-23`): `--models-directory` бьёт
/// `--base-directory`, тот задаёт корень, иначе `<instance>\ComfyUI\models`.
pub fn models_dir(profile: &LaunchProfile, instance_root: &Path) -> PathBuf {
    match flag_value(profile, "--models-directory") {
        Some(value) => resolve(Path::new(&profile.cwd), &value),
        None => base_dir(profile, instance_root).join("models"),
    }
}

/// Разбивает строку на токены, уважая кавычки.
///
/// Кавычки нужны не для красоты: путь вида `"C:\Program Files\..."`
/// без них распадётся на два аргумента, и запуск провалится.
fn tokenize(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quoted = false;

    for ch in line.chars() {
        match ch {
            '"' => quoted = !quoted,
            c if c.is_whitespace() && !quoted => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            c => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

/// Раскрывает `%VAR%` из собранных `set` и `%~dp0` — директорию `.bat`.
fn expand(token: &str, env: &HashMap<String, String>) -> String {
    let mut result = token.to_string();
    for (key, value) in env {
        result = result.replace(&format!("%{key}%"), value);
    }
    result
}

/// Относительный путь считается от директории `.bat`.
///
/// Для `advanced\*.bat` это принципиально: там интерпретатор указан
/// как `..\python_embeded\python.exe`, и от корня инстанса он не найдётся.
fn resolve(dir: &Path, token: &str) -> PathBuf {
    let path = Path::new(token);
    if path.is_absolute() {
        return path.to_path_buf();
    }
    normalize(&dir.join(path))
}

/// Убирает `.` и `..` из середины пути.
///
/// `std::fs::canonicalize` вернул бы verbatim-путь с `\\?\`, который
/// показывать пользователю нельзя, а `Path::join` сам `..` не схлопывает.
fn normalize(path: &Path) -> PathBuf {
    let mut parts: Vec<std::ffi::OsString> = Vec::new();
    for part in path.components() {
        use std::path::Component;
        match part {
            Component::CurDir => {}
            Component::ParentDir => {
                parts.pop();
            }
            other => parts.push(other.as_os_str().to_os_string()),
        }
    }
    parts.iter().fold(PathBuf::new(), |acc, p| acc.join(p))
}

/// Готовит аргументы к запуску: свой порт, запрет на браузер и, если инстанс
/// подключён к общим моделям в режиме флага, путь к нашему конфигу.
///
/// Существующий `--port` вырезается вместе со значением — иначе сборка
/// займёт порт из `.bat`, а не выданный нами, и два инстанса подерутся.
/// `--disable-auto-launch` в cli_args.py применяется после
/// `--windows-standalone-build` и всегда побеждает.
///
/// А вот `--extra-model-paths-config` из `.bat` мы **не трогаем**: у флага
/// `action='append'`, файлы применяются подряд, и своё мы просто дописываем
/// рядом. Вырезать чужое значило бы молча отобрать настройку, которую
/// пользователь завёл руками.
///
/// Своё значение идёт отдельным вхождением флага, а не голым путём в конец.
/// Проверено на argparse из реальной сборки: приписанный путь тоже
/// загрузился бы — `main.py:134` разворачивает вхождения через
/// `itertools.chain`, и порядок сохраняется в обоих случаях. Ломается другое:
/// если в `.bat` этого флага нет вовсе, голый путь становится позиционным
/// аргументом, и argparse отвергает всю командную строку. Отдельное
/// вхождение не зависит от того, что написано в `.bat`.
pub fn apply_runtime_args(args: &[String], port: u16, shared_config: Option<&str>) -> Vec<String> {
    let mut result = Vec::with_capacity(args.len() + 5);
    let mut skip_value = false;

    for arg in args {
        if skip_value {
            skip_value = false;
            continue;
        }
        if arg == "--port" {
            skip_value = true;
            continue;
        }
        if arg.starts_with("--port=") || arg == "--disable-auto-launch" {
            continue;
        }
        result.push(arg.clone());
    }

    result.push("--port".to_string());
    result.push(port.to_string());
    result.push("--disable-auto-launch".to_string());

    if let Some(config) = shared_config {
        result.push("--extra-model-paths-config".to_string());
        result.push(config.to_string());
    }

    result
}
