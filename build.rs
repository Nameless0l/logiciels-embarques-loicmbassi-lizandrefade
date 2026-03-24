fn main() {
    println!(
        "cargo:rustc-link-search=native={}/lib",
        env!("CARGO_MANIFEST_DIR").replace("\\", "/")
    );
}
