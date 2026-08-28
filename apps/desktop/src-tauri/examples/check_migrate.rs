//! A check of the model move and the duplicate cleanup.
//!
//! The most dangerous part of the app: the only place where we delete model
//! files. Checked against temporary folders; nothing belonging to the user is
//! touched.
//!
//! Run: cargo run --example check_migrate

use std::fs;
use std::path::{Path, PathBuf};

use cpo_desktop_lib::migrate::{self, MigrateCancel, SameName};

fn main() {
    let root = temp("cpo-migrate");
    let models = root.join("instance").join("models");
    let shared = root.join("shared");

    let mut failures = 0;
    let mut check = |name: &str, ok: bool, detail: String| {
        println!("{} {name}{}", if ok { "  OK  " } else { " FAIL " }, if detail.is_empty() {
            String::new()
        } else {
            format!(" — {detail}")
        });
        if !ok {
            failures += 1;
        }
    };

    // --- the test bed -------------------------------------------------------

    write(&models.join("checkpoints/put_checkpoints_here"), "");
    write(&models.join("checkpoints/model-a.safetensors"), &"a".repeat(4096));
    write(&models.join("loras/style.safetensors"), &"l".repeat(2048));
    // A model as a directory, like RMBG-2.0 with its HuggingFace snapshot.
    write(&models.join("RMBG/RMBG-2.0/model.safetensors"), &"r".repeat(1024));
    write(&models.join("RMBG/RMBG-2.0/.cache/huggingface/CACHEDIR.TAG"), "tag");
    // Ships with the build — not ours.
    write(&models.join("configs/v1-inference.yaml"), "shipped");
    write(&models.join("custom_nodes/Manager/__init__.py"), "nodes");

    // Three same-named things already lie in the shared folder: an exact
    // duplicate, a file of the same size with different contents, and a
    // directory with the same composition.
    write(&shared.join("checkpoints/dup.safetensors"), &"d".repeat(4096));
    write(&models.join("checkpoints/dup.safetensors"), &"d".repeat(4096));
    write(&shared.join("checkpoints/twin.safetensors"), &"x".repeat(4096));
    write(&models.join("checkpoints/twin.safetensors"), &"y".repeat(4096));
    write(&shared.join("RMBG/same-dir/f.bin"), &"z".repeat(512));
    write(&models.join("RMBG/same-dir/f.bin"), &"z".repeat(512));

    // --- what the scan shows ------------------------------------------------

    let scan = migrate::scan(&models, &shared);
    let cat = |name: &str| scan.categories.iter().find(|c| c.folder == name);
    let entry = |c: &str, n: &str| {
        cat(c).and_then(|x| x.entries.iter().find(|e| e.name == n)).cloned()
    };

    check("the models folder was read", scan.available, String::new());
    check(
        "configs is not offered for the move",
        cat("configs").is_none(),
        String::new(),
    );
    check(
        "custom_nodes is not offered for the move",
        cat("custom_nodes").is_none(),
        String::new(),
    );
    check(
        "the put_..._here marker is skipped",
        entry("checkpoints", "put_checkpoints_here").is_none(),
        String::new(),
    );
    check(
        "an ordinary model is visible",
        entry("checkpoints", "model-a.safetensors").is_some(),
        String::new(),
    );
    check(
        "a model held as a directory is visible as one entry",
        entry("RMBG", "RMBG-2.0").map(|e| e.is_dir) == Some(true),
        String::new(),
    );
    check(
        "the directory's size was counted together with .cache",
        entry("RMBG", "RMBG-2.0").map(|e| e.files) == Some(2),
        format!("{:?}", entry("RMBG", "RMBG-2.0").map(|e| e.files)),
    );

    // --- the comparison: three cases ----------------------------------------

    check(
        "an exact copy is recognised as a duplicate",
        entry("checkpoints", "dup.safetensors").and_then(|e| e.same_name)
            == Some(SameName::Duplicate),
        format!("{:?}", entry("checkpoints", "dup.safetensors").and_then(|e| e.same_name)),
    );
    // Exactly the case the edges are read for: the size matched, the contents
    // are different.
    check(
        "same size, different contents — different",
        entry("checkpoints", "twin.safetensors").and_then(|e| e.same_name)
            == Some(SameName::Different),
        format!("{:?}", entry("checkpoints", "twin.safetensors").and_then(|e| e.same_name)),
    );
    check(
        "a directory of the same composition — likely a duplicate",
        entry("RMBG", "same-dir").and_then(|e| e.same_name) == Some(SameName::LikelyDuplicate),
        format!("{:?}", entry("RMBG", "same-dir").and_then(|e| e.same_name)),
    );
    check(
        "different size — different",
        {
            write(&shared.join("loras/style.safetensors"), "shorter");
            migrate::compare(
                &models.join("loras/style.safetensors"),
                &shared.join("loras/style.safetensors"),
            ) == SameName::Different
        },
        String::new(),
    );

    // --- the move -----------------------------------------------------------

    let cancel = MigrateCancel::default();
    // The list is built from the scan: the move takes "category and model"
    // pairs. We offer the whole contents of three categories at once, taken
    // names included — skipping those is the move's own job, not the caller's.
    let offer: Vec<(String, String)> = migrate::scan(&models, &shared)
        .categories
        .iter()
        .filter(|c| ["checkpoints", "RMBG", "loras"].contains(&c.folder.as_str()))
        .flat_map(|c| c.entries.iter().map(|e| (c.folder.clone(), e.name.clone())))
        .collect();
    let out = migrate::move_all(&models, &shared, &offer, &cancel, |_| {});

    check(
        "a free name was moved",
        !models.join("checkpoints/model-a.safetensors").exists()
            && shared.join("checkpoints/model-a.safetensors").is_file(),
        String::new(),
    );
    check(
        "the directory was moved whole, .cache and all",
        shared.join("RMBG/RMBG-2.0/.cache/huggingface/CACHEDIR.TAG").is_file()
            && !models.join("RMBG/RMBG-2.0").exists(),
        String::new(),
    );
    check(
        "the taken names were not touched in the build",
        models.join("checkpoints/dup.safetensors").is_file()
            && models.join("checkpoints/twin.safetensors").is_file(),
        String::new(),
    );
    check(
        "someone else's file in the shared folder was not overwritten",
        fs::read_to_string(shared.join("checkpoints/twin.safetensors")).unwrap()
            == "x".repeat(4096),
        String::new(),
    );
    check(
        "the skipped ones are listed with a verdict",
        out.skipped.len() == 4,
        format!("{:?}", out.skipped.iter().map(|s| &s.name).collect::<Vec<_>>()),
    );
    check("there were no failures", out.failed.is_empty(), format!("{:?}", out.failed));
    check(
        "the marker stayed in the build",
        models.join("checkpoints/put_checkpoints_here").is_file(),
        String::new(),
    );
    check(
        "configs was not touched",
        models.join("configs/v1-inference.yaml").is_file()
            && !shared.join("configs").exists(),
        String::new(),
    );
    check(
        "custom_nodes was not touched",
        models.join("custom_nodes/Manager/__init__.py").is_file()
            && !shared.join("custom_nodes").exists(),
        String::new(),
    );
    check(
        "no temporary .cpo-partial is left",
        !has_partial(&shared),
        String::new(),
    );

    // --- the duplicate cleanup ----------------------------------------------

    // We call it with a list that includes a knowingly different file: the
    // command has to refuse that one itself, without relying on the caller's
    // good faith.
    let cleanup = migrate::remove_duplicates(
        &models,
        &shared,
        &[
            ("checkpoints".into(), "dup.safetensors".into()),
            ("checkpoints".into(), "twin.safetensors".into()),
            ("RMBG".into(), "same-dir".into()),
        ],
    );

    check(
        "the duplicate was removed",
        !models.join("checkpoints/dup.safetensors").exists(),
        String::new(),
    );
    check(
        "the duplicate directory was removed",
        !models.join("RMBG/same-dir").exists(),
        String::new(),
    );
    check(
        "the DIFFERENT file with the same name was NOT touched",
        models.join("checkpoints/twin.safetensors").is_file(),
        String::new(),
    );
    check("the refusal is accounted for in the report", cleanup.refused == 1, format!("{}", cleanup.refused));
    check(
        "what was removed stayed in the shared folder",
        shared.join("checkpoints/dup.safetensors").is_file()
            && shared.join("RMBG/same-dir/f.bin").is_file(),
        String::new(),
    );
    check(
        "the freed size was counted",
        cleanup.freed_bytes > 0.0,
        format!("{}", cleanup.freed_bytes),
    );

    // --- cancellation -------------------------------------------------------

    let models2 = root.join("i2").join("models");
    let shared2 = root.join("s2");
    write(&models2.join("loras/one.safetensors"), &"1".repeat(512));
    write(&models2.join("loras/two.safetensors"), &"2".repeat(512));

    let stop = MigrateCancel::default();
    stop.cancel();
    let cancelled = migrate::move_all(
        &models2,
        &shared2,
        &[
            ("loras".into(), "one.safetensors".into()),
            ("loras".into(), "two.safetensors".into()),
        ],
        &stop,
        |_| {},
    );
    check("the cancellation is marked in the report", cancelled.cancelled, String::new());
    check(
        "on cancellation the sources are in place",
        models2.join("loras/one.safetensors").is_file()
            && models2.join("loras/two.safetensors").is_file(),
        String::new(),
    );

    fs::remove_dir_all(&root).ok();

    println!("\nChecks failed: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn write(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().unwrap()).expect("the folder");
    fs::write(path, content).expect("the file");
}

fn has_partial(root: &Path) -> bool {
    let Ok(entries) = fs::read_dir(root) else { return false };
    entries.flatten().any(|e| {
        let name = e.file_name().to_string_lossy().to_string();
        name.contains("cpo-partial") || has_partial(&e.path())
    })
}

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    fs::remove_dir_all(&dir).ok();
    dir
}
