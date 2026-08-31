//! A check of the workflow library and of resolving an instance's folder.
//!
//! In `examples/` rather than in `#[cfg(test)]` for the known reason:
//! `cargo test` in this crate fails while loading the image.
//!
//! Test bed: node tools/fixtures/make-workflow-library.mjs
//!
//! Run: cargo run --example check_workflows

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use cpo_desktop_lib::profiles::{self, LaunchProfile};
use cpo_desktop_lib::workflows;

fn main() {
    let root = fixture();
    if !root.is_dir() {
        eprintln!("There is no test bed: {}", root.display());
        eprintln!("Build it: node tools/fixtures/make-workflow-library.mjs");
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

    let scan = workflows::scan_library(&root);
    let find = |path: &str| scan.items.iter().find(|i| i.path == path);

    check("the library was read", scan.available, String::new());
    check("the manifest parsed", !scan.manifest_broken, String::new());

    // --- what made it into the list and what did not ------------------------

    check(
        "an unrelated file is not shown as a workflow",
        !scan.items.iter().any(|i| i.path.ends_with(".txt")),
        String::new(),
    );
    check(
        "the manifest itself did not get into the list",
        find(workflows::MANIFEST).is_none(),
        String::new(),
    );
    check(
        "a nested subfolder was read",
        find("flux/portrait-v3.json").is_some(),
        String::new(),
    );
    check(
        "the path is written with forward slashes",
        scan.items.iter().all(|i| !i.path.contains('\\')),
        String::new(),
    );

    // --- merging with the manifest ------------------------------------------

    let basic = find("basic-txt2img.json");
    check(
        "the tags from the manifest were picked up",
        basic.map(|i| i.meta.tags.len()) == Some(2),
        format!("{:?}", basic.map(|i| i.meta.tags.clone())),
    );
    check("the favourite mark was picked up", basic.map(|i| i.meta.favorite) == Some(true), String::new());

    // The file lies in the folder with no record of it in the manifest. That
    // is normal, not an error: putting a workflow there through Explorer is a
    // legitimate scenario.
    let orphan = find("sdxl/base-upscale.json");
    check("a file with no manifest record is shown", orphan.is_some(), String::new());
    check(
        "a file with no record has empty tags, and that is not an error",
        orphan.map(|i| i.meta.tags.is_empty() && !i.meta.favorite) == Some(true),
        String::new(),
    );

    // There is a record, there is no file.
    let lost = find("lost/deleted.json");
    check("a record with no file is marked lost", lost.map(|i| i.lost) == Some(true), String::new());
    check(
        "a lost record kept its note",
        lost.map(|i| !i.meta.note.is_empty()) == Some(true),
        String::new(),
    );
    check(
        "a lost record does not pretend to be a whole file",
        lost.map(|i| i.size_bytes == 0.0 && i.nodes.is_empty()) == Some(true),
        String::new(),
    );

    // --- parsing the graph --------------------------------------------------

    check(
        "the nodes of the basic workflow were parsed",
        basic.map(|i| i.nodes.len()) == Some(6),
        format!("{:?}", basic.map(|i| i.nodes.clone())),
    );
    check(
        "repeated classes collapse into a set",
        workflows::node_types(r#"{"nodes":[{"type":"KSampler"},{"type":"KSampler"}]}"#)
            == Some(vec!["KSampler".to_string()]),
        String::new(),
    );
    check(
        "broken JSON is marked but did not bring the list down",
        find("broken.json").map(|i| i.broken) == Some(true),
        String::new(),
    );
    check(
        "JSON without nodes is not a workflow",
        workflows::node_types(r#"{"hello":"world"}"#).is_none(),
        String::new(),
    );
    check(
        "not JSON at all is not a workflow",
        workflows::node_types("not json in the slightest").is_none(),
        String::new(),
    );

    // --- the node diff ------------------------------------------------------

    let available: BTreeSet<String> = [
        "CheckpointLoaderSimple",
        "CLIPTextEncode",
        "EmptyLatentImage",
        "KSampler",
        "VAEDecode",
        "SaveImage",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    check(
        "the basic workflow is missing nothing",
        workflows::missing_nodes(&basic.map(|i| i.nodes.clone()).unwrap_or_default(), &available)
            .is_empty(),
        String::new(),
    );

    let custom = find("flux/portrait-v3.json").map(|i| i.nodes.clone()).unwrap_or_default();
    let missing = workflows::missing_nodes(&custom, &available);
    check(
        "the workflow with custom nodes is missing exactly two",
        missing.len() == 2,
        format!("{missing:?}"),
    );
    check(
        "the list of missing ones is sorted",
        missing.windows(2).all(|w| w[0] <= w[1]),
        format!("{missing:?}"),
    );

    // --- a broken manifest does not carry the files off ---------------------

    let broken_root = temp_dir("cpo-wf-broken");
    std::fs::create_dir_all(&broken_root).ok();
    std::fs::write(broken_root.join("a.json"), r#"{"nodes":[{"type":"KSampler"}]}"#).ok();
    std::fs::write(broken_root.join(workflows::MANIFEST), "{ this is not json").ok();
    let broken = workflows::scan_library(&broken_root);
    check("a broken manifest is marked", broken.manifest_broken, String::new());
    check(
        "a broken manifest did not carry the workflow off",
        broken.items.len() == 1,
        format!("{}", broken.items.len()),
    );
    std::fs::remove_dir_all(&broken_root).ok();

    // --- an unavailable library ---------------------------------------------

    let missing_root = workflows::scan_library(&root.join("no-such-folder"));
    check("a non-existent library is marked unavailable", !missing_root.available, String::new());
    check("an unavailable library does not crash the scanner", missing_root.items.is_empty(), String::new());

    // --- resolving an instance's workflow folder ----------------------------

    let instance = Path::new(r"D:\builds\comfy");
    let plain = profile(vec!["-s", "ComfyUI\\main.py"], r"D:\builds\comfy");
    check(
        "with no flag it takes ComfyUI\\user\\default\\workflows",
        profiles::workflows_dir(&plain, instance)
            == instance.join("ComfyUI").join("user").join("default").join("workflows"),
        profiles::workflows_dir(&plain, instance).display().to_string(),
    );

    let moved = profile(
        vec!["-s", "ComfyUI\\main.py", "--user-directory", r"E:\comfy-user"],
        r"D:\builds\comfy",
    );
    check(
        "--user-directory is respected",
        profiles::workflows_dir(&moved, instance)
            == PathBuf::from(r"E:\comfy-user\default\workflows"),
        profiles::workflows_dir(&moved, instance).display().to_string(),
    );

    let joined = profile(
        vec!["-s", "ComfyUI\\main.py", "--user-directory=E:\\comfy-user"],
        r"D:\builds\comfy",
    );
    check(
        "the equals-sign form is respected too",
        profiles::workflows_dir(&joined, instance)
            == PathBuf::from(r"E:\comfy-user\default\workflows"),
        profiles::workflows_dir(&joined, instance).display().to_string(),
    );

    // A relative path is counted from the working folder, and that is the
    // `.bat`'s directory — exactly as on a double-click launch.
    let relative = profile(
        vec!["-s", "ComfyUI\\main.py", "--user-directory", r"..\shared-user"],
        r"D:\builds\comfy\advanced",
    );
    check(
        "a relative path is counted from the profile's working folder",
        profiles::workflows_dir(&relative, instance)
            == PathBuf::from(r"D:\builds\comfy\shared-user\default\workflows"),
        profiles::workflows_dir(&relative, instance).display().to_string(),
    );

    // --- the --base-directory chain -----------------------------------------
    //
    // This second link was missed back in Phase 2.6: only --user-directory was
    // handled, and a build with just --base-directory kept its workflows
    // somewhere other than where we looked.

    let based = profile(
        vec!["-s", "ComfyUI\\main.py", "--base-directory", r"E:\comfy-base"],
        r"D:\builds\comfy",
    );
    check(
        "--base-directory moves the workflow folder too",
        profiles::workflows_dir(&based, instance)
            == PathBuf::from(r"E:\comfy-base\user\default\workflows"),
        profiles::workflows_dir(&based, instance).display().to_string(),
    );
    check(
        "--base-directory moves the models folder too",
        profiles::models_dir(&based, instance) == PathBuf::from(r"E:\comfy-base\models"),
        profiles::models_dir(&based, instance).display().to_string(),
    );

    // `--user-directory` is declared as "Overrides --base-directory".
    let both = profile(
        vec![
            "-s",
            "ComfyUI\\main.py",
            "--base-directory",
            r"E:\comfy-base",
            "--user-directory",
            r"F:\only-user",
        ],
        r"D:\builds\comfy",
    );
    check(
        "--user-directory beats --base-directory",
        profiles::workflows_dir(&both, instance)
            == PathBuf::from(r"F:\only-user\default\workflows"),
        profiles::workflows_dir(&both, instance).display().to_string(),
    );
    check(
        "but it does not touch the models folder",
        profiles::models_dir(&both, instance) == PathBuf::from(r"E:\comfy-base\models"),
        profiles::models_dir(&both, instance).display().to_string(),
    );

    // --- the models folder --------------------------------------------------

    check(
        "with no flags the models are in ComfyUI\\models",
        profiles::models_dir(&plain, instance)
            == instance.join("ComfyUI").join("models"),
        profiles::models_dir(&plain, instance).display().to_string(),
    );

    let models = profile(
        vec!["-s", "ComfyUI\\main.py", "--base-directory", r"E:\b", "--models-directory", r"G:\models"],
        r"D:\builds\comfy",
    );
    check(
        "--models-directory beats --base-directory",
        profiles::models_dir(&models, instance) == PathBuf::from(r"G:\models"),
        profiles::models_dir(&models, instance).display().to_string(),
    );

    // --- the output folder --------------------------------------------------
    //
    // Needed by the "output folder" button in the embedded tab's toolbar. The
    // chain is the very same, and a mistake in it would open someone else's
    // folder for the user.

    check(
        "with no flags the results are in ComfyUI\\output",
        profiles::output_dir(&plain, instance) == instance.join("ComfyUI").join("output"),
        profiles::output_dir(&plain, instance).display().to_string(),
    );
    check(
        "--base-directory moves the output folder too",
        profiles::output_dir(&based, instance) == PathBuf::from(r"E:\comfy-base\output"),
        profiles::output_dir(&based, instance).display().to_string(),
    );

    let outputs = profile(
        vec![
            "-s",
            "ComfyUI\\main.py",
            "--base-directory",
            r"E:\comfy-base",
            "--output-directory",
            r"..\generated",
        ],
        r"D:\builds\comfy\advanced",
    );
    check(
        "--output-directory beats --base-directory and counts from the working folder",
        profiles::output_dir(&outputs, instance) == PathBuf::from(r"D:\builds\comfy\generated"),
        profiles::output_dir(&outputs, instance).display().to_string(),
    );

    // A name from an input field: a graph pasted as text has no name of its
    // own, and what gets typed lands in a path — so it is checked, not trusted.
    let named = |input: &str| workflows::file_name_from_input(input);
    check(
        "the extension is appended by itself",
        named("portrait-v3").as_deref() == Some("portrait-v3.json"),
        format!("{:?}", named("portrait-v3")),
    );
    check(
        "a typed .json is not doubled",
        named("portrait-v3.json").as_deref() == Some("portrait-v3.json"),
        format!("{:?}", named("portrait-v3.json")),
    );
    // The name here is deliberately non-ASCII: workflow names are typed by the
    // user in their own language, and a name outside Latin-1 has to survive
    // the trim and reach the path intact. Keep it non-ASCII when editing.
    check(
        "the surrounding spaces are trimmed",
        named("  ночной город  ").as_deref() == Some("ночной город.json"),
        format!("{:?}", named("  ночной город  ")),
    );
    check(
        "an empty name is rejected",
        named("   ").is_none() && named(".json").is_none(),
        String::new(),
    );
    check(
        "escaping the library is rejected",
        named(r"..\..\evil").is_none() && named("sdxl/base").is_none(),
        String::new(),
    );
    check(
        "the characters Windows forbids are rejected",
        [r#"a:b"#, "a*b", "a?b", "a\"b", "a<b", "a>b", "a|b", "a\nb"]
            .iter()
            .all(|n| named(n).is_none()),
        String::new(),
    );
    check(
        "a dot at either edge is rejected",
        named(".hidden").is_none() && named("tail.").is_none(),
        String::new(),
    );

    println!("\nChecks failed: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn profile(args: Vec<&str>, cwd: &str) -> LaunchProfile {
    LaunchProfile {
        id: "run.bat".into(),
        name: "run".into(),
        advanced: false,
        python_path: r"D:\builds\comfy\python_embeded\python.exe".into(),
        args: args.into_iter().map(String::from).collect(),
        cwd: cwd.into(),
        env: Default::default(),
        fallback: false,
    }
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    dir
}

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("tools")
        .join("fixtures")
        .join("workflow-library")
}
