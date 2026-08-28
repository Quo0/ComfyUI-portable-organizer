//! The duplicate report over temporary folders.
//!
//! The main thing here is not "were duplicates found" but what did **not**
//! make it into the report: same-named files of different sizes must not count
//! as duplicates and must not enter the wasted total. An error in that
//! direction would turn the report into grounds for deleting the wrong thing.
//!
//! Run: cargo run --example check_duplicates

use std::fs;
use std::path::{Path, PathBuf};

use cpo_desktop_lib::duplicates::{self, Place, ScanCancel};

fn main() {
    let root = temp_dir("cpo-dups");
    // The first build's name is deliberately non-ASCII: builds live wherever
    // the user put them, and a path outside Latin-1 has to be walked exactly
    // like any other. Keep it non-ASCII when editing this fixture.
    let a = root.join("сборка A");
    let b = root.join("build B");
    let c = root.join("shared");

    // One and the same checkpoint in three places — a real duplicate.
    put(&a.join("checkpoints/sdxl.safetensors"), 4096);
    put(&b.join("checkpoints/sdxl.safetensors"), 4096);
    put(&c.join("checkpoints/sdxl.safetensors"), 4096);

    // One name, different sizes: different models that were unlucky with the
    // name.
    put(&a.join("loras/style.safetensors"), 1000);
    put(&b.join("loras/style.safetensors"), 2000);

    // One name in different categories means different roles, not a duplicate.
    put(&a.join("vae/thing.pt"), 500);
    put(&b.join("upscale_models/thing.pt"), 500);

    // The markers of empty categories exist in every build. Formally they
    // match perfectly; in substance they are noise of zero size.
    put(&a.join("embeddings/put_embeddings_here"), 0);
    put(&b.join("embeddings/put_embeddings_here"), 0);

    // configs ships with the build and matches everywhere.
    put(&a.join("configs/v1-inference.yaml"), 300);
    put(&b.join("configs/v1-inference.yaml"), 300);

    // A model as a directory: counted whole, the same as in the move.
    put(&a.join("RMBG/RMBG-2.0/model.safetensors"), 700);
    put(&a.join("RMBG/RMBG-2.0/config.json"), 100);
    put(&b.join("RMBG/RMBG-2.0/model.safetensors"), 700);
    put(&b.join("RMBG/RMBG-2.0/config.json"), 100);

    let places = vec![
        Place { name: "сборка A".into(), models_dir: a.clone() },
        Place { name: "build B".into(), models_dir: b.clone() },
        Place { name: "shared folder".into(), models_dir: c.clone() },
        Place { name: "vanished".into(), models_dir: root.join("no-such-thing") },
    ];

    // The counter goes through a cell: the scanner takes an `Fn`, because the
    // same closure in the app emits an event and holds no state.
    let ticks = std::cell::Cell::new(0u32);
    let report = duplicates::scan(&places, &ScanCancel::default(), |_| {
        ticks.set(ticks.get() + 1)
    });
    let ticks = ticks.get();

    let mut failures = 0;
    let dup = |name: &str| report.duplicates.iter().find(|g| g.name == name);

    failures += check(
        "a checkpoint in three places is recognised as a duplicate",
        dup("sdxl.safetensors").map(|g| g.copies.len()) == Some(3),
        format!("{:?}", dup("sdxl.safetensors").map(|g| g.copies.len())),
    );
    failures += check(
        "the waste is counted minus one copy",
        dup("sdxl.safetensors").map(|g| g.wasted_bytes) == Some(8192.0),
        format!("{:?}", dup("sdxl.safetensors").map(|g| g.wasted_bytes)),
    );
    failures += check(
        "a model held as a directory is counted whole",
        dup("RMBG-2.0").map(|g| g.wasted_bytes) == Some(800.0),
        format!("{:?}", dup("RMBG-2.0").map(|g| g.wasted_bytes)),
    );

    failures += check(
        "different sizes under one name are not a duplicate",
        dup("style.safetensors").is_none(),
        String::new(),
    );
    failures += check(
        "and they landed in the separate list",
        report.name_clashes.iter().any(|g| g.name == "style.safetensors"),
        String::new(),
    );
    failures += check(
        "they do not enter the wasted total",
        report.wasted_bytes == 8192.0 + 800.0,
        format!("{}", report.wasted_bytes),
    );

    failures += check(
        "one name in different categories does not count as a duplicate",
        !report.duplicates.iter().any(|g| g.name == "thing.pt"),
        String::new(),
    );
    failures += check(
        "the put_..._here markers are skipped",
        !report.duplicates.iter().any(|g| g.name.starts_with("put_")),
        String::new(),
    );
    failures += check(
        "configs does not go into the report",
        !report.duplicates.iter().any(|g| g.category == "configs"),
        String::new(),
    );

    failures += check(
        "the unavailable folder is named among the skipped",
        report.skipped.contains(&"vanished".to_string()),
        format!("{:?}", report.skipped),
    );
    failures += check(
        "the remaining places were walked",
        report.scanned_places == 3,
        format!("{}", report.scanned_places),
    );
    failures += check("progress kept arriving", ticks >= 4, format!("{ticks}"));
    failures += check("the report is not marked as interrupted", !report.cancelled, String::new());

    // The most expensive on top: that is where the user will start.
    failures += check(
        "the groups are sorted by waste",
        report.duplicates.first().map(|g| g.name.clone()) == Some("sdxl.safetensors".into()),
        String::new(),
    );

    // --- cancellation -------------------------------------------------------
    let cancel = ScanCancel::default();
    cancel.cancel();
    let stopped = duplicates::scan(&places, &cancel, |_| {});
    failures += check("the cancellation is marked in the report", stopped.cancelled, String::new());
    failures += check(
        "an interrupted walk counted nothing",
        stopped.duplicates.is_empty(),
        String::new(),
    );

    // --- not a single file was touched --------------------------------------
    failures += check(
        "every file is in place: the report deletes nothing",
        a.join("checkpoints/sdxl.safetensors").is_file()
            && b.join("checkpoints/sdxl.safetensors").is_file()
            && c.join("checkpoints/sdxl.safetensors").is_file()
            && a.join("loras/style.safetensors").is_file(),
        String::new(),
    );

    fs::remove_dir_all(&root).ok();

    println!("\nChecks failed: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn check(what: &str, ok: bool, detail: String) -> u32 {
    println!(
        "{} {what}{}",
        if ok { "  OK  " } else { " FAIL " },
        if detail.is_empty() { String::new() } else { format!(" — {detail}") }
    );
    u32::from(!ok)
}

fn put(path: &Path, size: usize) {
    fs::create_dir_all(path.parent().expect("there is a parent")).ok();
    fs::write(path, vec![b'x'; size]).expect("the file was not written");
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    fs::remove_dir_all(&dir).ok();
    dir
}
