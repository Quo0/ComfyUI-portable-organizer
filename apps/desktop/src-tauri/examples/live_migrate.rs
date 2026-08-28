//! A run of the move over the given folders, with the same code the app uses.
//!
//! Needed in order to check the move against a real build rather than only
//! against temporary folders: a real one has its own markers, its own
//! `configs` and categories created by custom nodes.
//!
//! Run: cargo run --example live_migrate -- <models-folder> <shared-root>

use std::path::Path;

use cpo_desktop_lib::migrate::{self, MigrateCancel};

fn main() {
    let mut args = std::env::args().skip(1);
    let models = args.next().expect("give the models folder");
    let shared = args.next().expect("give the shared root");
    let (models, shared) = (Path::new(&models), Path::new(&shared));

    let scan = migrate::scan(models, shared);
    println!("categories to move: {}", scan.categories.len());
    println!("files to travel: {}, size: {:.0}", scan.total_files, scan.total_bytes);

    for category in &scan.categories {
        let busy: Vec<_> = category
            .entries
            .iter()
            .filter(|e| e.same_name.is_some())
            .map(|e| format!("{} ({:?})", e.name, e.same_name.unwrap()))
            .collect();
        if !busy.is_empty() {
            println!("  {}: taken names — {}", category.folder, busy.join(", "));
        }
    }

    // Only checkpoints are moved: a check must not shift someone's twenty
    // gigabytes for the sake of a few kilobytes of meaning. The list comes as
    // "category and model" pairs, the way the screen sends them.
    let offer: Vec<(String, String)> = scan
        .categories
        .iter()
        .filter(|c| c.folder == "checkpoints")
        .flat_map(|c| c.entries.iter().map(|e| (c.folder.clone(), e.name.clone())))
        .collect();
    let outcome = migrate::move_all(models, shared, &offer, &MigrateCancel::default(), |p| {
        println!("  {} / {} — {}/{}", p.done, p.total, p.category, p.name)
    });

    println!("moved: {:?}", outcome.moved);
    println!("skipped: {:?}", outcome.skipped.iter().map(|s| (&s.name, s.verdict)).collect::<Vec<_>>());
    println!("failures: {:?}", outcome.failed);
}
