//! A check of the shared-folder scanner and the YAML generator.
//!
//! It lives in `examples/` rather than in `#[cfg(test)]` for the same reason
//! as `check_profiles`: `cargo test` in this crate fails while loading the
//! test binary with `STATUS_ENTRYPOINT_NOT_FOUND`. The limitation is written
//! down in `plan/notes/phase-0-spike.md`.
//!
//! The test bed is built by `node tools/fixtures/make-shared-root.mjs`.
//!
//! Run: cargo run --example check_shared

use std::path::PathBuf;

use cpo_desktop_lib::shared_models::{
    self, ApplyMode, CategoryStatus, InstanceShared, SharedSettings,
};

fn main() {
    let root = fixture();
    if !root.is_dir() {
        eprintln!("There is no test bed: {}", root.display());
        eprintln!("Build it: node tools/fixtures/make-shared-root.mjs");
        std::process::exit(1);
    }

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

    let scan = shared_models::scan_root(&root);

    check("the root is available", scan.available, String::new());

    let find = |folder: &str| scan.categories.iter().find(|c| c.folder == folder);

    // --- recognition --------------------------------------------------------

    let checkpoints = find("checkpoints");
    check(
        "checkpoints is recognised",
        checkpoints.map(|c| c.status) == Some(CategoryStatus::Recognized),
        format!("{:?}", checkpoints.map(|c| c.status)),
    );
    check(
        "checkpoints counted its files",
        checkpoints.map(|c| c.files) == Some(2),
        format!("{:?}", checkpoints.map(|c| c.files)),
    );

    // --- map_legacy ---------------------------------------------------------

    check(
        "unet goes under the diffusion_models key",
        find("unet").and_then(|c| c.key.as_deref()) == Some("diffusion_models"),
        format!("{:?}", find("unet").and_then(|c| c.key.clone())),
    );
    check(
        "clip goes under the text_encoders key",
        find("clip").and_then(|c| c.key.as_deref()) == Some("text_encoders"),
        format!("{:?}", find("clip").and_then(|c| c.key.clone())),
    );

    // --- the blacklist ------------------------------------------------------

    let blocked = find("custom_nodes");
    check(
        "custom_nodes is marked as excluded",
        blocked.map(|c| c.status) == Some(CategoryStatus::Blocked),
        format!("{:?}", blocked.map(|c| c.status)),
    );
    check(
        "custom_nodes has no key",
        blocked.map(|c| c.key.is_none()) == Some(true),
        String::new(),
    );

    // --- the unrecognised ---------------------------------------------------

    check(
        "a folder with an arbitrary name is marked unrecognised",
        find("my notes").map(|c| c.status) == Some(CategoryStatus::Unknown),
        format!("{:?}", find("my notes").map(|c| c.status)),
    );
    check(
        "an unrecognised folder still gets a key",
        find("my notes").and_then(|c| c.key.as_deref()) == Some("my notes"),
        String::new(),
    );

    // --- a file in the root is not a category -------------------------------

    check(
        "README.txt in the root was not taken for a category",
        find("README.txt").is_none(),
        String::new(),
    );

    // --- offering the missing ones ------------------------------------------

    check(
        "it does not offer to create diffusion_models when unet is present",
        !scan.missing.iter().any(|m| m == "diffusion_models"),
        format!("{:?}", scan.missing),
    );

    // --- the size does not count the excluded -------------------------------

    let blocked_files = blocked.map(|c| c.files).unwrap_or(0);
    check(
        "blacklisted files do not enter the overall count",
        scan.total_files
            == scan
                .categories
                .iter()
                .filter(|c| c.status != CategoryStatus::Blocked)
                .map(|c| c.files)
                .sum::<u32>()
            && blocked_files > 0,
        format!("{} in total, {} in custom_nodes", scan.total_files, blocked_files),
    );

    // --- the YAML -----------------------------------------------------------

    let yaml = shared_models::render_yaml(&[(&scan, "shared")], true);
    println!("\n--- the generated YAML ---\n{yaml}---\n");

    check("the marker of our own file is there", shared_models::is_ours(&yaml), String::new());
    check(
        "someone else's file is not taken for ours",
        !shared_models::is_ours("comfyui:\n  base_path: D:/models\n"),
        String::new(),
    );
    check("the section name is our own", yaml.contains("cpo_shared_0:"), String::new());
    check("is_default was passed through", yaml.contains("is_default: true"), String::new());
    let base_path = yaml
        .lines()
        .find(|l| l.trim_start().starts_with("base_path:"))
        .unwrap_or_default();
    check("base_path with forward slashes", !base_path.contains('\\'), base_path.to_string());
    // The path travels into another application's config and into the
    // interface: a `..` in the middle both works worse and reads as unfinished.
    check("base_path without `..`", !base_path.contains(".."), base_path.to_string());
    check(
        "custom_nodes did not get into the YAML in any form",
        !yaml.contains("custom_nodes"),
        String::new(),
    );

    // The subtlest branch: two folders have to come together under one key as
    // a multi-line block, otherwise the second one is silently lost.
    let merged_ok = yaml.contains("  diffusion_models: |\n    unet/\n    diffusion_models/\n");
    check("diffusion_models and unet were merged into one key", merged_ok, String::new());

    // The order within a key decides the target for new downloads: that is
    // `paths[0]`. With is_default the paths are inserted at the front, i.e.
    // the order is reversed — so the canonical folder has to be written last.
    check(
        "with is_default the canonical folder is written last",
        yaml.contains("  text_encoders: |\n    clip/\n    text_encoders/\n"),
        String::new(),
    );

    // Without is_default the paths are appended at the end and the order is
    // preserved — so the canonical one has to come first.
    let appended = shared_models::render_yaml(&[(&scan, "shared")], false);
    check(
        "without is_default the canonical folder is written first",
        appended.contains("  text_encoders: |\n    text_encoders/\n    clip/\n"),
        String::new(),
    );

    let single_ok = yaml.contains("  checkpoints: checkpoints/\n");
    check("a lone category is written on one line", single_ok, String::new());

    check(
        "the text_encoders key occurs exactly once",
        yaml.matches("  text_encoders:").count() == 1,
        format!("{}", yaml.matches("  text_encoders:").count()),
    );

    // --- the default values -------------------------------------------------

    check(
        "by default we download into the shared folder",
        SharedSettings::default().make_default_target,
        String::new(),
    );
    check(
        "by default the mode does not touch the instance's folder",
        InstanceShared::default().apply_mode == ApplyMode::Flag,
        String::new(),
    );
    check(
        "by default the instance is not connected",
        !InstanceShared::default().enabled,
        String::new(),
    );

    // --- an unavailable root ------------------------------------------------

    let missing = shared_models::scan_root(&root.join("no-such-folder"));
    check("a non-existent root is marked unavailable", !missing.available, String::new());
    check("an unavailable root does not crash the scanner", missing.categories.is_empty(), String::new());

    println!("\nChecks failed: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn fixture() -> PathBuf {
    // From `src-tauri` to the repository root is two levels up.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("tools")
        .join("fixtures")
        .join("shared-models")
}
