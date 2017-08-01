extern crate bobbin_chip as chip;

fn main() {
    chip::builder::build("chip-src/mod.rx","src/chip/mod.rs").unwrap();
}