use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Pass our linker script to the top crate
    let script = "tm4c1294.ld";
    let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::copy(src.join(script), out.join(script)).unwrap();
    println!("cargo:rustc-link-search={}", out.display());    
}
