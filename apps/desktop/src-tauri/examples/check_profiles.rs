//! A check of the `.bat` parsing against the test bed.
//!
//! These are tests moved into an example out of necessity: `cargo test` does
//! not run in this crate — the test binary fails at load with
//! STATUS_ENTRYPOINT_NOT_FOUND, and removing `cdylib` from crate-type does not
//! help. The limitation is written down in `plan/notes/phase-0-spike.md`; once
//! it is lifted, the checks move into `#[cfg(test)]` unchanged in substance.
//!
//! Run: cargo run --example check_profiles

use std::path::PathBuf;

use cpo_desktop_lib::profiles::{apply_runtime_args, parse_bat};

fn main() {
    let fixture = fixture_root();
    println!("[CHECK] test bed: {}", fixture.display());
    assert!(
        fixture.join(r"python_embeded\python.exe").is_file(),
        "the test bed is not built: run node tools/fixtures/make-fixture.mjs"
    );

    let mut failures = 0;

    // An ordinary one-liner from the root.
    let normal = parse_bat(&fixture, "run_fake.bat", false);
    failures += check("run_fake: parsed", !normal.fallback);
    failures += check(
        "run_fake: the interpreter from the root",
        normal.python_path.ends_with(r"fake-instance\python_embeded\python.exe"),
    );
    failures += check(
        "run_fake: the arguments in full",
        normal.args
            == vec![
                "-s",
                r"ComfyUI\main.py",
                "--windows-standalone-build",
                "--cpo-mode",
                "normal",
            ],
    );
    failures += check(
        "run_fake: the working folder is the root",
        PathBuf::from(&normal.cwd) == fixture,
    );

    // Quotes around the paths and a `::` comment.
    let quoted = parse_bat(&fixture, "run_fake_crash.bat", false);
    failures += check("run_fake_crash: parsed", !quoted.fallback);
    failures += check(
        "run_fake_crash: the quotes were stripped",
        quoted.args.contains(&r"ComfyUI\main.py".to_string()),
    );

    // `advanced\` with paths through `..\` — this case is what the whole thing
    // was started for.
    let advanced = parse_bat(&fixture, r"advanced\run_fake_hang.bat", true);
    failures += check("advanced: parsed", !advanced.fallback);
    failures += check(
        "advanced: the ..\\ collapsed down to the instance root",
        advanced.python_path.ends_with(r"fake-instance\python_embeded\python.exe")
            && !advanced.python_path.contains(".."),
    );
    failures += check(
        "advanced: the working folder is advanced itself",
        PathBuf::from(&advanced.cwd) == fixture.join("advanced"),
    );

    // A launch through a variable: it cannot be parsed, the fallback is
    // mandatory.
    let via_var = parse_bat(&fixture, "run_fake_via_var.bat", false);
    failures += check("via_var: fell back to cmd /c", via_var.fallback);
    failures += check("via_var: the command is cmd", via_var.python_path == "cmd");
    failures += check(
        "via_var: the set was collected into env",
        via_var.env.get("CPO_FIXTURE").map(String::as_str) == Some("1"),
    );

    // Mutating the arguments before the start.
    let mutated = apply_runtime_args(&normal.args, 8231, None);
    failures += check(
        "the port was appended",
        mutated.windows(2).any(|w| w[0] == "--port" && w[1] == "8231"),
    );
    failures += check(
        "the browser is forbidden",
        mutated.iter().filter(|a| *a == "--disable-auto-launch").count() == 1,
    );
    failures += check(
        "with no shared models there is no config flag",
        !mutated.iter().any(|a| a == "--extra-model-paths-config"),
    );

    let already = vec![
        "-s".to_string(),
        "main.py".to_string(),
        "--port".to_string(),
        "8188".to_string(),
        "--disable-auto-launch".to_string(),
    ];
    failures += check(
        "the previous --port was cut out together with its value",
        apply_runtime_args(&already, 8300, None)
            == vec!["-s", "main.py", "--port", "8300", "--disable-auto-launch"],
    );

    // Shared models: our config is appended as a separate occurrence of the
    // flag.
    let shared = apply_runtime_args(&already, 8300, Some(r"C:\data\shared-models.yaml"));
    failures += check(
        "the config path comes right after its own flag",
        shared
            .windows(2)
            .any(|w| w[0] == "--extra-model-paths-config" && w[1] == r"C:\data\shared-models.yaml"),
    );

    // The flag has `action='append'`, the files are applied one after another.
    // Cutting out someone else's would mean silently taking away a setting
    // that was made by hand.
    let with_own = vec![
        "-s".to_string(),
        "main.py".to_string(),
        "--extra-model-paths-config".to_string(),
        "my_paths.yaml".to_string(),
    ];
    let merged = apply_runtime_args(&with_own, 8300, Some("ours.yaml"));
    failures += check(
        "someone else's --extra-model-paths-config was preserved",
        merged.iter().any(|a| a == "my_paths.yaml"),
    );
    failures += check(
        "the config flag occurs twice, not glued into one occurrence",
        merged.iter().filter(|a| *a == "--extra-model-paths-config").count() == 2,
    );
    // Our path has to follow our own flag. Appended to someone else's
    // occurrence it would load too, but with no such flag in the `.bat` it
    // would become a positional argument and bring down the parse of the whole
    // command line.
    failures += check(
        "our path follows our own flag",
        merged.iter().position(|a| a == "ours.yaml")
            == merged.iter().rposition(|a| a == "--extra-model-paths-config").map(|i| i + 1),
    );

    println!();
    if failures == 0 {
        println!("[CHECK] everything matched");
    } else {
        println!("[CHECK] failures: {failures}");
        std::process::exit(1);
    }
}

fn check(what: &str, ok: bool) -> u32 {
    println!("{} {what}", if ok { "  ok  " } else { " FAIL " });
    u32::from(!ok)
}

/// The test bed lies next to the sources, not where the example was run from.
fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../tools/fixtures/fake-instance")
        .canonicalize()
        .map(|p| PathBuf::from(p.display().to_string().trim_start_matches(r"\\?\")))
        .expect("the test bed was not found")
}
