//! A check of the API client against a running ComfyUI.
//!
//! What is checked is **our** code, not ComfyUI's behaviour: the same
//! functions the app uses go to a real server. The wrapper script
//! `tools/check-workflows-live.mjs` brings a build up and passes the port here.
//!
//! Run: cargo run --example check_comfy_live -- <port>

use cpo_desktop_lib::comfy_api::{Client, UploadOutcome};
use cpo_desktop_lib::workflows;

const WORKFLOW: &str = r#"{"nodes":[{"id":1,"type":"KSampler"},{"id":2,"type":"SaveImage"}],"links":[],"version":0.4}"#;
const NAME: &str = "cpo-live-check.json";

fn main() {
    let port: u16 = std::env::args()
        .nth(1)
        .and_then(|p| p.parse().ok())
        .expect("give the port: cargo run --example check_comfy_live -- 8189");

    let client = Client::new(port);
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

    // --- the listing --------------------------------------------------------

    // In a fresh build there is no workflows folder at all, and ComfyUI answers
    // with a 404. For us that is an empty library, not an error.
    let before = client.list_workflows();
    check(
        "the listing worked (a 404 \"no folder\" counts as success too)",
        before.is_ok(),
        format!("{:?}", before.as_ref().err()),
    );
    let before = before.unwrap_or_default();

    // --- the upload ---------------------------------------------------------

    let first = client.upload_workflow(NAME, WORKFLOW, false);
    check(
        "the workflow was uploaded",
        matches!(first, Ok(UploadOutcome::Written)),
        format!("{first:?}"),
    );

    let after = client.list_workflows().unwrap_or_default();
    check(
        "the uploaded workflow appeared in the listing",
        after.iter().any(|f| f.path.ends_with(NAME)),
        format!("{:?}", after.iter().map(|f| &f.path).collect::<Vec<_>>()),
    );
    check(
        "the listing grew by exactly one",
        after.len() == before.len() + 1,
        format!("was {}, became {}", before.len(), after.len()),
    );

    // --- the name conflict --------------------------------------------------

    // The same file, overwrite=false. There must be no silent overwrite: this
    // is the only mechanism by which we protect someone else's workflow.
    let second = client.upload_workflow(NAME, WORKFLOW, false);
    check(
        "a repeat upload yields a conflict, not a quiet overwrite",
        matches!(second, Ok(UploadOutcome::Conflict)),
        format!("{second:?}"),
    );

    let forced = client.upload_workflow(NAME, WORKFLOW, true);
    check(
        "with overwriting permitted the upload goes through",
        matches!(forced, Ok(UploadOutcome::Written)),
        format!("{forced:?}"),
    );

    // --- reading ------------------------------------------------------------

    let read = client.read_workflow(NAME);
    check(
        "the workflow reads back byte for byte",
        read.as_deref().ok() == Some(WORKFLOW),
        format!("{:?}", read.as_ref().map(|s| s.len())),
    );

    // --- the nodes ----------------------------------------------------------

    let keys = client.object_info_keys();
    check("object_info was received", keys.is_ok(), format!("{:?}", keys.as_ref().err()));
    let keys = keys.unwrap_or_default();
    check("plenty of node classes arrived", keys.len() > 100, format!("{}", keys.len()));
    check("the base classes are in place", keys.contains("KSampler"), String::new());

    let nodes = workflows::node_types(WORKFLOW).unwrap_or_default();
    check(
        "a workflow on stock nodes has no missing nodes",
        workflows::missing_nodes(&nodes, &keys).is_empty(),
        String::new(),
    );

    let invented = vec!["KSampler".to_string(), "AThoroughlyInventedNode".to_string()];
    check(
        "an invented node is recognised as missing",
        workflows::missing_nodes(&invented, &keys) == vec!["AThoroughlyInventedNode".to_string()],
        String::new(),
    );

    println!("\nChecks failed: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
