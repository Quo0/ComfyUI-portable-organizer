//! A check of a single claim: does `fs::rename` survive an existing
//! destination folder?
//!
//! The install wizard extracts into a temporary `<dest>.cpo-partial` and
//! renames it at the end. The checks deliberately allow an **existing empty**
//! destination folder — and that is exactly where everything broke.
//!
//! Run: cargo run --example check_rename

use std::fs;

fn main() {
    let base = std::env::temp_dir().join("cpo-rename-check");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(base.join("src")).expect("the source folder was not created");
    fs::write(base.join("src").join("file.txt"), b"x").expect("the file was not written");

    // Case one: the destination does not exist.
    let missing = base.join("missing");
    let first = fs::rename(base.join("src"), &missing);
    println!("destination absent      -> {:?}", first.as_ref().map(|_| "ok"));

    // Case two: the destination exists and is empty — exactly what the user
    // does when they create the folder in advance.
    fs::create_dir_all(base.join("src2")).expect("the source folder was not created");
    fs::write(base.join("src2").join("file.txt"), b"x").expect("the file was not written");
    let existing = base.join("existing");
    fs::create_dir_all(&existing).expect("the destination folder was not created");
    let second = fs::rename(base.join("src2"), &existing);
    println!(
        "destination exists, empty -> {}",
        match &second {
            Ok(()) => "ok".to_string(),
            Err(e) => format!("ERROR {e}"),
        }
    );

    let _ = fs::remove_dir_all(&base);

    if second.is_err() {
        println!("\nConfirmed: rename onto an existing folder does not work,");
        println!("it has to be removed before the rename.");
    } else {
        println!("\nrename coped by itself — removing the folder in installer.rs is redundant.");
    }
}
