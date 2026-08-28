//! Launch profiles: parsing `.bat` and preparing the command.
//!
//! A portable build is started by a one-liner of the form
//! `.\python_embeded\python.exe -s ComfyUI\main.py --windows-standalone-build`,
//! dressed up with `echo` and `pause`. We pull the interpreter and the
//! arguments out of it in order to launch python directly: going through
//! `cmd /c` loses the real PID and with it the ability to stop the server.
//!
//! If the parse fails we do not invent anything, but honestly fall back to
//! `cmd /c <file>`. The build still starts, it is just less manageable.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Words that mean nothing in a launch one-liner.
const NOISE: [&str; 10] = [
    "echo", "pause", "rem", "cls", "title", "color", "chcp", "setlocal", "endlocal", "exit",
];

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LaunchProfile {
    /// The `.bat` path relative to the instance root. Also the identifier.
    pub id: String,
    /// The file name without extension. Not translated.
    pub name: String,
    pub advanced: bool,
    /// Absolute path to the interpreter.
    pub python_path: String,
    pub args: Vec<String>,
    /// The working folder is the `.bat`'s own directory. That is exactly what
    /// gets substituted when the file is launched by double click, and what
    /// the `..\` inside it are counted from.
    pub cwd: String,
    pub env: HashMap<String, String>,
    /// The parse failed and the launch goes through `cmd /c`. In the interface
    /// that is a reason to warn: stopping such a process is less reliable.
    pub fallback: bool,
}

/// Parses a `.bat` into a launch profile.
///
/// `root` is the instance root, `rel` the file's path relative to it.
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

    // The encoding of a `.bat` is undefined: files come in cp866 because of
    // non-Latin comments. We read loosely — the lines we care about are ASCII
    // anyway.
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
        // The launch line is recognised by the interpreter's name. A launch
        // through a variable (`%PY% -s ...`) will not land here — and rightly
        // so: we do not take on expanding arbitrary batch.
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
        // The arguments are left as they are: relative paths inside them are
        // counted from the working folder, which for us is the `.bat`'s
        // directory.
        args: tokens.map(|t| expand(&t, &env)).collect(),
        cwd: dir.display().to_string(),
        env,
        fallback: false,
    }
}

/// A flag's value in the profile arguments, in both spellings.
///
/// The last occurrence wins — argparse behaves the same way.
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

/// The root ComfyUI counts its folders from.
///
/// `--base-directory` (`comfy/cli_args.py:70`) moves models, `custom_nodes`,
/// `input`, `output`, `temp` and `user` all at once; without it the root is
/// ComfyUI's own folder (`folder_paths.py:16-18`).
fn base_dir(profile: &LaunchProfile, instance_root: &Path) -> PathBuf {
    match flag_value(profile, "--base-directory") {
        Some(value) => resolve(Path::new(&profile.cwd), &value),
        None => instance_root.join("ComfyUI"),
    }
}

/// Where this build keeps its workflows.
///
/// **The path must not be assumed,** and there are two flags here, arranged in
/// a chain: `--user-directory` (`cli_args.py:254`) beats `--base-directory`,
/// and that one sets the root `user/` is counted from
/// (`folder_paths.py:72`).
///
/// The second link of this chain was missed back in Phase 2.6: only
/// `--user-directory` was handled, and a build launched with just
/// `--base-directory` kept its workflows somewhere other than where we looked.
///
/// A relative path in the flag is counted from the working folder, which for
/// us is the `.bat`'s directory — exactly as on a double-click launch.
///
/// The folder may not exist: ComfyUI creates it lazily, on the first save.
/// Checking for existence is not our concern here — the caller either creates
/// the tree or treats it as empty.
pub fn workflows_dir(profile: &LaunchProfile, instance_root: &Path) -> PathBuf {
    let user = match flag_value(profile, "--user-directory") {
        Some(value) => resolve(Path::new(&profile.cwd), &value),
        None => base_dir(profile, instance_root).join("user"),
    };

    // `default` is the public default user folder
    // (`app/user_manager.py:79`). We do not support ComfyUI's multi-user mode
    // and do not pretend to.
    user.join("default").join("workflows")
}

/// Where this build keeps its models.
///
/// The same chain as for workflows, in the same order
/// (`folder_paths.py:20-23`): `--models-directory` beats `--base-directory`,
/// that one sets the root, otherwise `<instance>\ComfyUI\models`.
pub fn models_dir(profile: &LaunchProfile, instance_root: &Path) -> PathBuf {
    match flag_value(profile, "--models-directory") {
        Some(value) => resolve(Path::new(&profile.cwd), &value),
        None => base_dir(profile, instance_root).join("models"),
    }
}

/// Where this build puts generation results.
///
/// The same chain: `--output-directory` beats `--base-directory`
/// (`cli_args.py:72`, `main.py:147`), otherwise `<base>\output`
/// (`folder_paths.py:69`).
///
/// The folder may not exist: ComfyUI does not create it before the first
/// generation. We will not create it on the user's behalf — nothing appears
/// inside someone else's installation by our will.
pub fn output_dir(profile: &LaunchProfile, instance_root: &Path) -> PathBuf {
    match flag_value(profile, "--output-directory") {
        Some(value) => resolve(Path::new(&profile.cwd), &value),
        None => base_dir(profile, instance_root).join("output"),
    }
}

/// Splits a line into tokens, respecting quotes.
///
/// The quotes are not decoration: a path like `"C:\Program Files\..."` would
/// fall apart into two arguments without them, and the launch would fail.
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

/// Expands `%VAR%` from the collected `set`s and `%~dp0` — the `.bat`'s
/// directory.
fn expand(token: &str, env: &HashMap<String, String>) -> String {
    let mut result = token.to_string();
    for (key, value) in env {
        result = result.replace(&format!("%{key}%"), value);
    }
    result
}

/// A relative path is counted from the `.bat`'s directory.
///
/// For `advanced\*.bat` this is essential: the interpreter there is written as
/// `..\python_embeded\python.exe`, and it will not be found from the instance
/// root.
fn resolve(dir: &Path, token: &str) -> PathBuf {
    let path = Path::new(token);
    if path.is_absolute() {
        return path.to_path_buf();
    }
    normalize(&dir.join(path))
}

/// Removes `.` and `..` from the middle of a path.
///
/// `std::fs::canonicalize` would return a verbatim path with `\\?\`, which
/// must not be shown to the user, and `Path::join` does not collapse `..` by
/// itself.
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

/// Prepares the arguments for launch: our own port, a ban on the browser and,
/// if the instance is connected to shared models in flag mode, the path to our
/// config.
///
/// An existing `--port` is cut out together with its value — otherwise the
/// build takes the port from the `.bat` rather than the one we handed out, and
/// two instances end up fighting. `--disable-auto-launch` is applied after
/// `--windows-standalone-build` in cli_args.py and always wins.
///
/// The `--extra-model-paths-config` from the `.bat`, on the other hand, we
/// **do not touch**: the flag has `action='append'`, the files are applied one
/// after another, and we simply add ours alongside. Cutting out someone else's
/// would mean silently taking away a setting the user made by hand.
///
/// Our value goes as a separate occurrence of the flag, not as a bare path
/// appended at the end. Verified against argparse from a real build: an
/// appended path would load too — `main.py:134` unfolds the occurrences via
/// `itertools.chain`, and the order is preserved either way. Something else
/// breaks: if the `.bat` has no such flag at all, a bare path becomes a
/// positional argument and argparse rejects the whole command line. A separate
/// occurrence does not depend on what is written in the `.bat`.
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
