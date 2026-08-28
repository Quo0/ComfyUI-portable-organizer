//! The "file inside the instance" mode: recognition, backing up someone
//! else's, restoring.
//!
//! The most dangerous part of Phase 2.5: here the app writes into someone
//! else's installation. The section's promise — "we do not change someone
//! else's settings silently" — is checked exactly here, and checked against a
//! real file system, because the mistake looks like a hand-written config
//! that got lost.
//!
//! Works in a temporary folder and touches nothing belonging to the user.
//!
//! Run: cargo run --example check_instance_file

use std::fs;
use std::path::{Path, PathBuf};

use cpo_desktop_lib::shared_models::{self, InstanceFileState};

const FOREIGN: &str = "comfyui:\n  base_path: D:/my/models\n  checkpoints: checkpoints/\n";

fn main() {
    let root = temp_root();
    let config = shared_models::instance_config_path(&root);
    fs::create_dir_all(config.parent().unwrap()).expect("create the test-bed folder");

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

    let ours = format!("{}\ncpo_shared_0:\n  base_path: D:/shared\n", shared_models::MARKER);

    // --- there is no file ---------------------------------------------------

    check(
        "empty folder: no file found",
        shared_models::inspect_instance_file(&root).state == InstanceFileState::Absent,
        String::new(),
    );

    let backup = shared_models::write_instance_file(&root, &ours, 1).expect("the write");
    check("empty folder: no copy was needed", backup.is_none(), String::new());
    check("empty folder: the file was written", config.is_file(), String::new());

    // --- the file is ours ---------------------------------------------------

    check(
        "our own file is recognised",
        shared_models::inspect_instance_file(&root).state == InstanceFileState::Ours,
        String::new(),
    );

    let updated = format!("{}\ncpo_shared_0:\n  base_path: D:/other\n", shared_models::MARKER);
    let backup = shared_models::write_instance_file(&root, &updated, 2).expect("the update");
    check("our own file was updated without a copy", backup.is_none(), String::new());
    check(
        "our own file really was overwritten",
        fs::read_to_string(&config).unwrap().contains("D:/other"),
        String::new(),
    );

    // --- disconnecting: our file is removed ---------------------------------

    shared_models::remove_instance_file(&root).expect("the removal");
    check("disconnecting removed our file", !config.exists(), String::new());

    // --- the file is someone else's -----------------------------------------

    fs::write(&config, FOREIGN).expect("put someone else's file in place");
    check(
        "someone else's file is recognised as theirs",
        shared_models::inspect_instance_file(&root).state == InstanceFileState::Foreign,
        String::new(),
    );
    check(
        "the contents of someone else's file are handed over for display",
        shared_models::inspect_instance_file(&root).content.as_deref() == Some(FOREIGN),
        String::new(),
    );

    let backup = shared_models::write_instance_file(&root, &ours, 100).expect("replacing theirs");
    let backup_path = backup.clone().expect("there has to be a copy");
    check("someone else's file was replaced with a copy made", backup.is_some(), backup_path.clone());
    check(
        "the copy holds exactly what was lying there",
        fs::read_to_string(&backup_path).unwrap() == FOREIGN,
        String::new(),
    );

    // --- disconnecting: the copy comes back ---------------------------------

    shared_models::remove_instance_file(&root).expect("removal with restoration");
    check(
        "the previous config came back into place",
        fs::read_to_string(&config).ok().as_deref() == Some(FOREIGN),
        String::new(),
    );
    check("the copy was removed after the restoration", !Path::new(&backup_path).exists(), String::new());

    // --- someone else's file is not deleted ---------------------------------

    // What lies there is someone else's (just restored). Disconnecting has to
    // leave it alone: since it is not ours, it was put there after us.
    shared_models::remove_instance_file(&root).expect("disconnecting with a foreign file");
    check(
        "someone else's file was not deleted on disconnect",
        fs::read_to_string(&config).ok().as_deref() == Some(FOREIGN),
        String::new(),
    );

    // --- the freshest copy --------------------------------------------------

    fs::remove_file(&config).ok();
    shared_models::write_instance_file(&root, &ours, 1).expect("the write");
    fs::write(shared_models::backup_path(&config, 5), "old\n").expect("the old copy");
    fs::write(shared_models::backup_path(&config, 40), "fresh\n").expect("the fresh copy");
    shared_models::remove_instance_file(&root).expect("the removal");
    check(
        "the freshest copy is the one restored",
        fs::read_to_string(&config).ok().as_deref() == Some("fresh\n"),
        fs::read_to_string(&config).unwrap_or_default().trim().to_string(),
    );

    fs::remove_dir_all(&root).ok();

    println!("\nChecks failed: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn temp_root() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cpo-instance-file-{}", std::process::id()));
    fs::remove_dir_all(&dir).ok();
    dir
}
