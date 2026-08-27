fn main() {
    // Icons do not end up among the build's dependencies: `tauri_build::build()`
    // registers only tauri.conf.json and the capabilities. Replacing files in
    // icons/ therefore rebuilds nothing, and the app starts with the old icon
    // baked into the previous binary — which looks like a cache that does not
    // exist. So we register the folder ourselves.
    println!("cargo:rerun-if-changed=icons");
    tauri_build::build()
}
