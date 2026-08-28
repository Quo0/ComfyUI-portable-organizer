//! The rule "ours stays in the tab, someone else's goes to the browser".
//!
//! The pure part — the decision about a URL — is what gets checked. The
//! embedding itself cannot be checked from here: it needs a window, which is
//! why it lives in the list of manual checks. But the rule is the easiest
//! thing to get wrong: too narrow and ComfyUI itself is sent to the browser,
//! too wide and a login page stays in the tab, where there is neither an
//! address bar nor a back button.
//!
//! Run: cargo run --example check_webview

use cpo_desktop_lib::webview::{internal, label};
use tauri::Url;

fn main() {
    let mut failures = 0;
    const PORT: u16 = 8188;

    let mut case = |what: &str, url: &str, want: bool| {
        let parsed = Url::parse(url).expect("a valid URL");
        let got = internal(&parsed, PORT);
        let ok = got == want;
        println!("{} {what} — {url}", if ok { "  OK  " } else { " FAIL " });
        failures += u32::from(!ok);
    };

    case("the interface itself stays in the tab", "http://127.0.0.1:8188", true);
    case("a page inside the build stays", "http://127.0.0.1:8188/templates", true);
    case("an API request stays", "http://127.0.0.1:8188/api/userdata", true);

    // Downloads and "Save image" in WebView2 go through these schemes.
    // A ban would turn exporting from ComfyUI into silence.
    case("blob stays", "blob:http://127.0.0.1:8188/9f1c", true);
    case("data stays", "data:text/plain,hi", true);
    case("about:blank stays", "about:blank", true);

    case("the documentation goes to the browser", "https://docs.comfy.org/", false);
    case("the API-nodes login goes to the browser", "https://platform.comfy.org/login", false);
    // A neighbouring instance is someone else's server too: it has a tab of
    // its own.
    case("another port on localhost is foreign", "http://127.0.0.1:8189/", false);
    // The same port but a different host: an identical start of the string
    // must not fool us.
    case("another host with the same port is foreign", "http://example.com:8188/", false);
    // Exactly our address at the start of the string, yet the host belongs to
    // someone else: what stands before the `@` is a username. A prefix
    // comparison let this through.
    case(
        "our address as a username is foreign",
        "http://127.0.0.1:8188@example.com/",
        false,
    );
    // A different scheme means a different server from the one we brought up.
    case("https to the same address is foreign", "https://127.0.0.1:8188/", false);

    let id = "i1755000000000-2";
    let ok = label(id) == format!("comfy-{id}");
    println!("{} the tab label — {}", if ok { "  OK  " } else { " FAIL " }, label(id));
    failures += u32::from(!ok);

    println!("\nChecks failed: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
