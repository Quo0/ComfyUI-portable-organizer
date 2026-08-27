//! The lifecycle of a running build.
//!
//! The state machine, the live log and readiness detection. All of it is
//! platform-neutral: spawning and killing are hidden behind `ProcessSupervisor`.

use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// How many log lines we keep in memory. Enough that after coming back to the
/// instance screen the whole startup is visible, crash traceback included.
const LOG_LIMIT: usize = 5000;

/// A cold start with a pile of custom nodes really does take minutes.
pub const READY_TIMEOUT: Duration = Duration::from_secs(300);

/// How long we wait for someone else's server after an unexpected process exit.
/// After installing nodes, ComfyUI-Manager brings up a new process and kills the
/// old one: our handle is lost while the port stays taken.
const RESPAWN_GRACE: Duration = Duration::from_secs(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum RunState {
    Stopped,
    Starting,
    Running,
    Stopping,
    /// The process exited on its own, and we did not ask it to.
    Crashed,
    /// The server on the port is alive, but our process no longer controls it.
    Detached,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LogLine {
    /// `stdout` or `stderr`. ComfyUI writes its startup to stderr, and telling
    /// the streams apart is useful: stdout shows the build is already working.
    pub stream: String,
    /// The contents are never translated.
    pub text: String,
    /// The line replaces the previous one instead of being appended. That is how
    /// tqdm behaves: it prints progress with a carriage return, and without
    /// replacement a hundred updates turn into a hundred lines.
    pub replaces_last: bool,
}

/// A state snapshot for the UI.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RunStatus {
    pub instance_id: String,
    pub state: RunState,
    pub port: Option<u16>,
    pub pid: Option<u32>,
    /// The launch moment, in epoch milliseconds.
    pub started_at: Option<f64>,
    /// Seconds until readiness. Appears once the server has answered.
    pub ready_secs: Option<u32>,
    /// The exit code, if the process exited on its own.
    pub exit_code: Option<i32>,
    /// The profile it was launched with.
    pub profile_id: Option<String>,
}

impl RunStatus {
    pub fn stopped(instance_id: &str) -> Self {
        Self {
            instance_id: instance_id.to_string(),
            state: RunState::Stopped,
            port: None,
            pid: None,
            started_at: None,
            ready_secs: None,
            exit_code: None,
            profile_id: None,
        }
    }
}

/// A ring buffer of one instance's lines.
#[derive(Default)]
pub struct LogBuffer(VecDeque<LogLine>);

impl LogBuffer {
    fn push(&mut self, line: LogLine) {
        if line.replaces_last {
            self.0.pop_back();
        }
        self.0.push_back(line);
        while self.0.len() > LOG_LIMIT {
            self.0.pop_front();
        }
    }

    pub fn snapshot(&self) -> Vec<LogLine> {
        self.0.iter().cloned().collect()
    }
}

/// A single running instance.
pub struct Running {
    pub status: RunStatus,
    pub log: LogBuffer,
    /// Whether we asked for the stop. Tells an orderly exit from a crash.
    pub stopping: bool,
}

/// The state of every launch. Lives in `tauri::State`.
#[derive(Default)]
pub struct Runtime {
    inner: Mutex<HashMap<String, Arc<Mutex<Running>>>>,
}

impl Runtime {
    pub fn get(&self, id: &str) -> Option<Arc<Mutex<Running>>> {
        self.inner.lock().unwrap().get(id).cloned()
    }

    /// Stores a ready-made cell rather than creating a new one: the log reader
    /// threads already write into it, and swapping it would cost every line that
    /// arrived before the swap.
    pub fn insert(&self, id: &str, cell: Arc<Mutex<Running>>) {
        self.inner.lock().unwrap().insert(id.to_string(), cell);
    }

    pub fn remove(&self, id: &str) {
        self.inner.lock().unwrap().remove(id);
    }

    /// The states of every instance we know anything about.
    pub fn statuses(&self) -> Vec<RunStatus> {
        self.inner
            .lock()
            .unwrap()
            .values()
            .map(|cell| cell.lock().unwrap().status.clone())
            .collect()
    }

    /// Whether the instance is busy: running, or starting, or stopping.
    pub fn is_busy(&self, id: &str) -> bool {
        self.get(id).is_some_and(|cell| {
            matches!(
                cell.lock().unwrap().status.state,
                RunState::Starting | RunState::Running | RunState::Stopping | RunState::Detached
            )
        })
    }
}

/// Splits a stream into lines, understanding carriage returns.
///
/// A naive split on newlines turns a tqdm progress bar into tens of thousands
/// of lines and floods the buffer so badly that the real startup is pushed out
/// of it.
#[derive(Default)]
pub struct LineSplitter {
    pending: String,
    /// The previously emitted line was closed by a carriage return, meaning it
    /// is transient: the next one must take its place.
    transient: bool,
}

impl LineSplitter {
    pub fn feed(&mut self, chunk: &str, mut emit: impl FnMut(String, bool)) {
        for ch in chunk.chars() {
            match ch {
                '\n' => {
                    if self.pending.is_empty() && self.transient {
                        // A newline right after progress merely closes it. An
                        // empty line must not be printed over the last value —
                        // it would disappear from the screen.
                        self.transient = false;
                        continue;
                    }
                    emit(std::mem::take(&mut self.pending), self.transient);
                    self.transient = false;
                }
                '\r' => {
                    if self.pending.is_empty() {
                        continue;
                    }
                    emit(std::mem::take(&mut self.pending), self.transient);
                    self.transient = true;
                }
                c => self.pending.push(c),
            }
        }
    }

    /// The remainder with no trailing newline — for example the last line of
    /// a crashed process.
    pub fn flush(&mut self, mut emit: impl FnMut(String, bool)) {
        if !self.pending.is_empty() {
            emit(std::mem::take(&mut self.pending), self.transient);
            self.transient = false;
        }
    }
}

/// Reads the stream in chunks and emits finished lines.
///
/// In chunks specifically, not with `BufRead::lines()`: that one splits only on
/// newlines and would swallow the carriage return along with the progress.
pub fn pump<R: Read>(mut stream: R, mut on_line: impl FnMut(String, bool)) {
    let mut splitter = LineSplitter::default();
    let mut buf = [0u8; 8192];
    loop {
        match stream.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buf[..n]);
                splitter.feed(&chunk, &mut on_line);
            }
        }
    }
    splitter.flush(&mut on_line);
}

pub fn push_line(running: &Arc<Mutex<Running>>, stream: &str, text: String, replaces: bool) -> LogLine {
    let line = LogLine { stream: stream.to_string(), text, replaces_last: replaces };
    running.lock().unwrap().log.push(line.clone());
    line
}

/// Polls `/system_stats` until the server answers.
///
/// A bare TcpStream instead of an HTTP client: one request every half second is
/// not worth the minutes of compiling an extra dependency.
pub fn probe(port: u16) -> bool {
    let addr = format!("127.0.0.1:{port}");
    let Ok(parsed) = addr.parse() else { return false };
    let Ok(mut sock) = TcpStream::connect_timeout(&parsed, Duration::from_millis(500)) else {
        return false;
    };
    let _ = sock.set_read_timeout(Some(Duration::from_millis(1500)));

    let req = format!("GET /system_stats HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
    if sock.write_all(req.as_bytes()).is_err() {
        return false;
    }
    let mut head = [0u8; 32];
    match sock.read(&mut head) {
        Ok(n) => String::from_utf8_lossy(&head[..n]).contains("200"),
        Err(_) => false,
    }
}

/// Waits for readiness until the time runs out or a stop is requested.
pub fn wait_ready(
    port: u16,
    timeout: Duration,
    keep_going: impl Fn() -> bool,
) -> Result<u32, AppError> {
    let started = Instant::now();
    while started.elapsed() < timeout {
        if !keep_going() {
            return Err(AppError::new("run.cancelled"));
        }
        if probe(port) {
            return Ok(started.elapsed().as_secs() as u32);
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    Err(AppError::with(
        "run.readyTimeout",
        "secs",
        timeout.as_secs(),
    ))
}

/// After an unexpected process exit, checks whether someone brought the server
/// back up.
///
/// That is how ComfyUI-Manager behaves after installing nodes. Telling this case
/// apart from a crash is mandatory: the user must be told not "it crashed" but
/// "the server restarted outside our control".
pub fn detached_after_exit(port: u16) -> bool {
    let deadline = Instant::now() + RESPAWN_GRACE;
    while Instant::now() < deadline {
        if probe(port) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    false
}

pub fn now_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0)
}
