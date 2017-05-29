extern crate bobbin_chip as chip;

fn main() {
    chip::builder::build("chip-src/lib.rx","src/lib.rs").unwrap();
}