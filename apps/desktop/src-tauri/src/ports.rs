//! Handing out ports.
//!
//! The port is needed before launch: it goes into the command line arguments,
//! and there is nowhere to learn it after the fact from someone else's process.

use std::net::TcpListener;
use std::time::{Duration, Instant};

/// Below 1024 requires administrator rights, above 65535 there are no ports.
const FIRST: u16 = 1024;

/// Whether the port is free right now.
///
/// Checked with a real `bind` rather than against a list of connections: only
/// that answers the question we actually have — can we take it.
pub fn is_free(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

/// The port to launch on: the preferred one if free, otherwise the nearest one
/// above it.
///
/// Between the check and the process start there is a gap in which someone else
/// can take the port. There is nothing to narrow it with: we cannot hand our
/// socket to another process. So a taken port is not a fault of the handout but
/// the reason a build will not come up, and that is visible in the log.
pub fn pick(preferred: u16) -> Option<u16> {
    let start = preferred.max(FIRST);
    (start..=u16::MAX).find(|p| is_free(*p))
}

/// Waits for the port to be released after a stop.
///
/// A process manages to die before the system lets go of its socket. Without
/// this wait, restarting an instance runs into its own previous port and looks
/// like a random glitch.
pub fn wait_released(port: u16, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if is_free(port) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    is_free(port)
}
