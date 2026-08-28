//! The Phase 1.5 spike: is `sevenz-rust2` good enough to unpack a real
//! archive?
//!
//! There is exactly one question: LZMA2 in pure Rust may turn out to be
//! several times slower than 7-Zip, in which case `7za.exe` would have to be
//! bundled — plus a couple of megabytes on the installer and a mention of LGPL
//! in the licences. That has to be decided by measurement, not by feel.
//!
//! Along the way it is a prototype of what will become `installer.rs`:
//! stripping the root folder, verbatim paths and streaming progress are all
//! already here.
//!
//! Run:
//!   cargo run --release --example spike_7z -- <archive.7z> <where-to>

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use sevenz_rust2::{ArchiveReader, Password};

fn main() {
    let mut args = std::env::args().skip(1);
    let archive = args.next().expect("give the path to the .7z");
    let dest = args.next().expect("give the destination folder");

    let dest = PathBuf::from(&dest);

    // The cleanup is not a service detail but part of the wizard: a
    // cancellation and a crash both have to remove <dest>.cpo-partial. It was
    // verified that the ordinary path is not enough: both `rmdir /s /q` and
    // `fs::remove_dir_all` stumble over MAX_PATH on the deepest files and
    // return "directory not empty".
    if archive == "--clean" {
        println!("[SPIKE] removing {}", dest.display());
        let started = Instant::now();
        fs::remove_dir_all(verbatim(&dest)).expect("the folder could not be removed");
        println!("[SPIKE] removed in {:.1} s", started.elapsed().as_secs_f32());
        return;
    }

    if dest.exists() {
        println!("[SPIKE] cleaning up the previous run: {}", dest.display());
        fs::remove_dir_all(verbatim(&dest)).expect("the previous folder could not be removed");
    }

    // ------------------------------------------------- parsing the header
    let started = Instant::now();
    let reader = ArchiveReader::open(&archive, Password::empty()).expect("the archive did not open");
    let header_secs = started.elapsed().as_secs_f32();

    let entries = reader.archive().files.clone();
    let files = entries.iter().filter(|e| !e.is_directory).count();
    let dirs = entries.len() - files;
    let total: u64 = entries.iter().map(|e| e.size).sum();
    let root = single_root(&entries);

    println!("[SPIKE] the header was parsed in {header_secs:.2} s");
    println!("[SPIKE] entries: {files} files, {dirs} folders");
    println!("[SPIKE] uncompressed size: {:.2} GB", total as f64 / 1024f64.powi(3));
    println!("[SPIKE] root folder: {root:?}");
    println!(
        "[SPIKE] longest path after the root is stripped: {} characters",
        entries
            .iter()
            .map(|e| strip_root(&e.name, root.as_deref()).chars().count())
            .max()
            .unwrap_or(0)
    );

    // ------------------------------------------------- the extraction
    let mut reader = ArchiveReader::open(&archive, Password::empty()).expect("the archive did not open");

    // The third argument is the decoder's thread count. The question is
    // settled by measurement: the archive is assembled as one block
    // (Solid=+, Blocks=1), and there is most likely nothing there to spread a
    // single LZMA2 stream across cores with.
    if let Some(threads) = args.next().and_then(|t| t.parse::<u32>().ok()) {
        println!("[SPIKE] decoder threads: {threads}");
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

            // Progress once a second: 56 thousand lines in the console by
            // themselves slow the run noticeably and distort the measurement.
            if last_report.elapsed().as_secs() >= 1 {
                last_report = Instant::now();
                let pct = done as f64 / total as f64 * 100.0;
                print!("\r[SPIKE] {pct:5.1}%  {}", rel);
                let _ = io::stdout().flush();
            }
            Ok(true)
        })
        .expect("the extraction failed");

    let secs = started.elapsed().as_secs_f32();
    println!(
        "\n[SPIKE] extracted in {secs:.1} s — {:.1} MB/s",
        done as f64 / 1024f64.powi(2) / secs as f64
    );
}

/// The archive's single root folder. Its name is set by the user, so it is
/// stripped from the paths — which also takes 25 characters off the length.
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

/// A verbatim `\\?\` path, which lifts the 260-character MAX_PATH limit.
///
/// `std::fs` does not add it by itself: the deepest file in the archive is 206
/// characters relative to the root, and a destination longer than fifty
/// characters would break the extraction without any warning at all.
///
/// **Forward slashes have to become backslashes.** Verbatim means "pass to the
/// kernel as is": the ordinary path normalisation is switched off along with
/// the limit, and a `/` from an archive entry's name makes the path invalid —
/// error 123 without a single hint of the reason.
fn verbatim(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        let text = path.display().to_string();
        if text.starts_with(r"\\?\") {
            return path.to_path_buf();
        }
        // The prefix only works with an absolute path free of "." and "..".
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
