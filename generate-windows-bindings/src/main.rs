fn main() {
    static MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
    let in_file = format!("{MANIFEST_DIR}/../src/windows_sys.txt");
    let out_file = format!("{MANIFEST_DIR}/../src/windows_sys.rs");

    windows_bindgen::bindgen([
        "--flat",    // Use a flat namespace
        "--sys",     // Use windows-sys style bindings
        "--no-deps", // Don't depend on any crates except windows-link
        "--out",
        &out_file,
        "--filter",
        "--etc",
        &in_file,
    ])
    .unwrap();
}
