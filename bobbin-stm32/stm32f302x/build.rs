extern crate bobbin_chip as chip;

fn main() {
    chip::builder::build("chip-src/lib.rx","src/chip/mod.rs").unwrap();
}

