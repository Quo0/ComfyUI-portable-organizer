//! Calls to the HTTP API of a running ComfyUI instance.
//!
//! **It has to be Rust going here, not our frontend.** A `fetch` from our
//! webview to `127.0.0.1:<port>` is cross-site, and `origin_only_middleware`
//! (`server.py:159-197`) cuts such requests off. This is the same barrier that
//! got `<iframe>` rejected back in Phase 0; the only way around it is
//! `--enable-cors-header`, which switches the protection off entirely —
//! rejected in the same place.
//!
//! The client is `ureq` without TLS: we go strictly to the loopback, there is
//! nothing to encrypt, and a synchronous call fits the rule "logic in plain
//! functions, commands are thin async wrappers".
//!
//! `process::wait_ready` was deliberately left on a bare `TcpStream`: one tiny
//! poll every half a second, no reason to rewrite what works.

use std::collections::BTreeSet;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// ComfyUI's answer to a listing request with `full_info=true`.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoteFile {
    /// The path relative to the requested folder, with forward slashes —
    /// that is how `get_file_info` hands it over (`app/user_manager.py:29`).
    pub path: String,
    pub size: f64,
    /// Epoch milliseconds.
    pub modified: f64,
}

/// How the upload ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum UploadOutcome {
    Written,
    /// A file with this name already exists and overwriting was not allowed.
    /// Not an error but a fork in the road: we ask the user.
    Conflict,
}

pub struct Client {
    base: String,
    agent: ureq::Agent,
}

impl Client {
    pub fn new(port: u16) -> Self {
        // The timeout is modest but not zero: a server on the loopback answers
        // instantly, whereas one stuck in startup does not answer at all, and
        // waiting for it forever is not an option.
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(30)))
            .build()
            .into();
        Self { base: format!("http://127.0.0.1:{port}"), agent }
    }

    /// The list of a build's workflows.
    ///
    /// The recursive v1 variant rather than `/v2/userdata`: users lay their
    /// workflows out in subfolders, and one recursive call is more honest than
    /// a level-by-level walk. `full_info` gives the size and modification
    /// time.
    ///
    /// **A 404 means "the folder is not there yet", not an error.** ComfyUI
    /// creates `user/default/workflows` lazily, on the first save, and in a
    /// fresh build it simply does not exist.
    pub fn list_workflows(&self) -> Result<Vec<RemoteFile>, AppError> {
        let url = format!("{}/userdata?dir=workflows&recurse=true&full_info=true", self.base);
        let body = match self.get(&url) {
            Ok(body) => body,
            Err(AppError { code, .. }) if code == "comfy.notFound" => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };

        let files: Vec<RemoteFile> = serde_json::from_str(&body)
            .map_err(|e| AppError::because("comfy.badResponse", e))?;

        // Unrelated files do turn up in the workflow folder: ComfyUI puts
        // housekeeping files there too. They do not go into the workflow list.
        Ok(files.into_iter().filter(|f| f.path.ends_with(".json")).collect())
    }

    pub fn read_workflow(&self, rel: &str) -> Result<String, AppError> {
        self.get(&format!("{}/userdata/{}", self.base, encode(&format!("workflows/{rel}"))))
    }

    /// Uploads a workflow into the build.
    ///
    /// `overwrite = false` yields a **409** on a name collision
    /// (`app/user_manager.py:397`). That is precisely the mechanism by which
    /// we prevent silently erasing someone else's workflow.
    pub fn upload_workflow(
        &self,
        rel: &str,
        content: &str,
        overwrite: bool,
    ) -> Result<UploadOutcome, AppError> {
        let url = format!(
            "{}/userdata/{}?overwrite={overwrite}",
            self.base,
            encode(&format!("workflows/{rel}"))
        );

        match self.agent.post(&url).send(content) {
            Ok(_) => Ok(UploadOutcome::Written),
            Err(ureq::Error::StatusCode(409)) => Ok(UploadOutcome::Conflict),
            Err(e) => Err(AppError::because("comfy.uploadFailed", e)),
        }
    }

    /// Removes a workflow from the build (`app/user_manager.py:427`).
    ///
    /// For a running build the file is removed by its own hands rather than
    /// out from under it: ComfyUI holds the workflow folder as its own and
    /// knows nothing of edits from outside.
    ///
    /// A missing file is not an error. It could have been taken away after we
    /// read the list, and there is no point complaining about the outcome we
    /// wanted.
    pub fn delete_workflow(&self, rel: &str) -> Result<(), AppError> {
        let url = format!("{}/userdata/{}", self.base, encode(&format!("workflows/{rel}")));
        match self.agent.delete(&url).call() {
            Ok(_) => Ok(()),
            Err(ureq::Error::StatusCode(404)) => Ok(()),
            Err(e) => Err(AppError::because("workflows.removeFailed", e)),
        }
    }

    /// The set of node classes available to this build.
    ///
    /// The answer is a multi-megabyte JSON with the schemas of every node, and
    /// all we need from it are the top-level keys. We parse it whole (there is
    /// no other way), but hand out a single set of names: that is what gets
    /// cached and compared.
    pub fn object_info_keys(&self) -> Result<BTreeSet<String>, AppError> {
        let body = self.get_large(&format!("{}/object_info", self.base))?;
        let value: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| AppError::because("comfy.badResponse", e))?;

        let map = value
            .as_object()
            .ok_or_else(|| AppError::new("comfy.badResponse"))?;

        Ok(map.keys().cloned().collect())
    }

    fn get(&self, url: &str) -> Result<String, AppError> {
        self.read(url, 8 * 1024 * 1024)
    }

    /// A separate limit for `/object_info`: on a build with fifty-odd node
    /// packs the answer runs past tens of megabytes, and ureq's default would
    /// truncate it silently.
    fn get_large(&self, url: &str) -> Result<String, AppError> {
        self.read(url, 256 * 1024 * 1024)
    }

    fn read(&self, url: &str, limit: u64) -> Result<String, AppError> {
        match self.agent.get(url).call() {
            Ok(mut res) => res
                .body_mut()
                .with_config()
                .limit(limit)
                .read_to_string()
                .map_err(|e| AppError::because("comfy.badResponse", e)),
            Err(ureq::Error::StatusCode(404)) => Err(AppError::new("comfy.notFound")),
            Err(e) => Err(AppError::because("comfy.unreachable", e)),
        }
    }
}

/// A snapshot of an instance's available nodes.
///
/// There is no one to ask on a stopped build, and an "unknown" to every
/// compatibility question is useless. So on every successful start we put the
/// set of classes aside, and for a stopped build we answer from it, honestly
/// marking the answer as data from the last launch.
///
/// The cache is derived: losing it is painless, it comes back on the very next
/// start. Hence `app_local_data_dir` rather than the data folder — on a clean
/// uninstall of the app it costs nothing to lose.
pub mod cache {
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Snapshot {
        pub taken_at: f64,
        pub nodes: BTreeSet<String>,
    }

    fn path(dir: &Path, instance_id: &str) -> PathBuf {
        // The instance's name does not go into the path: it is arbitrary and
        // it changes. The identifier is ours, from the registry, and is safe
        // as a file name.
        dir.join("nodes").join(format!("{instance_id}.json"))
    }

    pub fn write(dir: &Path, instance_id: &str, nodes: &BTreeSet<String>) {
        let snapshot = Snapshot {
            taken_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as f64)
                .unwrap_or(0.0),
            nodes: nodes.clone(),
        };
        let file = path(dir, instance_id);
        // Silently: failing to write the cache loses convenience, not data.
        if let Some(parent) = file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(text) = serde_json::to_string(&snapshot) {
            let _ = std::fs::write(file, text);
        }
    }

    pub fn read(dir: &Path, instance_id: &str) -> Option<Snapshot> {
        let text = std::fs::read_to_string(path(dir, instance_id)).ok()?;
        serde_json::from_str(&text).ok()
    }
}

/// Percent-encoding of a path segment.
///
/// A slash has to turn into `%2F`: the path travels as a single URL segment,
/// and ComfyUI unfolds it back (`app/user_manager.py:88`). Our own
/// implementation instead of a dependency — the character set is tiny, and
/// pulling in another crate for it is pointless.
fn encode(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    for byte in path.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char)
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}
