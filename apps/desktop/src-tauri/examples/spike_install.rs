//! A run of the whole install wizard, bypassing the interface.
//!
//! Clicking cannot check this: the extraction takes minutes, and what needs
//! checking is what the eye cannot see — that the temporary `.cpo-partial`
//! folder appears and disappears, that the rename is atomic, that the second
//! target is copied from the first rather than extracted again.
//!
//! Run:
//!   cargo run --release --example spike_install -- <archive.7z> <target1> [target2]

use std::path::Path;
use std::time::Instant;

use cpo_desktop_lib::installer::{self, InstallCancel, InstallTarget};
use cpo_desktop_lib::instances::Accent;

fn main() {
    let mut args = std::env::args().skip(1);
    let archive = args.next().expect("give the path to the .7z");
    let paths: Vec<String> = args.collect();
    assert!(!paths.is_empty(), "give at least one target");

    let started = Instant::now();
    let info = installer::probe_archive(&archive).expect("the archive did not parse");
    println!(
        "[SPIKE] header in {:.2} s: {} files, {:.2} GB, root {:?}",
        started.elapsed().as_secs_f32(),
        info.files,
        info.total_uncompressed / 1024f64.powi(3),
        info.single_root
    );

    let targets: Vec<InstallTarget> = paths
        .iter()
        .enumerate()
        .map(|(i, path)| InstallTarget {
            path: path.clone(),
            name: format!("Spike {}", i + 1),
            description: String::new(),
            accent: Accent::named("teal"),
            preferred_port: 8188 + i as u16,
        })
        .collect();

    // The checks before the work: space, folder emptiness, path length.
    for check in installer::check_targets(&info, &targets) {
        for e in &check.errors {
            println!("[SPIKE] ERROR {}: {} {:?}", check.path, e.code, e.params);
        }
        for w in &check.warnings {
            println!("[SPIKE] warning {}: {} {:?}", check.path, w.code, w.params);
        }
    }

    // The first target is created in advance as an empty folder. That is
    // exactly what the user does, and exactly what the final rename broke on:
    // replacing an existing directory through MoveFileEx does not work.
    std::fs::create_dir_all(&paths[0]).expect("the destination folder was not created");
    println!("[SPIKE] the first target was created empty in advance");

    let cancel = InstallCancel::default();
    let started = Instant::now();
    let mut last_phase = String::new();

    let outcome = installer::run(&info, &targets, &cancel, |p| {
        let phase = format!("{:?} {}/{}", p.phase, p.target, p.targets);
        if phase != last_phase {
            last_phase = phase.clone();
            println!("\n[SPIKE] phase: {phase} — {}", p.target_name);
        }
        // Per cent by files, bytes alongside: on this archive the two diverge
        // so far that by bytes the progress looks frozen.
        print!(
            "\r[SPIKE] {:5.1}%  {}/{} files, {:.2}/{:.2} GB  {}",
            p.done_files as f64 / p.total_files as f64 * 100.0,
            p.done_files,
            p.total_files,
            p.done_bytes / 1024f64.powi(3),
            p.total_bytes / 1024f64.powi(3),
            p.current
        );
    });

    println!();
    match outcome {
        Ok(()) => println!("[SPIKE] done in {:.1} s", started.elapsed().as_secs_f32()),
        Err(e) => {
            println!("[SPIKE] FAILED: {} {:?}", e.code, e.params);
            return;
        }
    }

    // The main atomicity check: no temporary folders are left, and every
    // target passes instance validation.
    for path in &paths {
        let partial = format!("{path}.cpo-partial");
        println!(
            "[SPIKE] {path}: temporary folder {}, valid {}",
            if Path::new(&partial).exists() { "LEFT BEHIND" } else { "removed" },
            valid(path)
        );
    }
}

fn valid(path: &str) -> bool {
    Path::new(path).join(r"python_embeded\python.exe").is_file()
        && Path::new(path).join(r"ComfyUI\main.py").is_file()
}
