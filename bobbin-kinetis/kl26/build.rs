extern crate bobbin_build;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    bobbin_build::setup_linker();
}