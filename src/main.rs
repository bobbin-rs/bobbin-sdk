extern crate chip;
extern crate xml;
extern crate clap;
extern crate svd2chip;

use svd2chip::*;
use chip::writer::{Context, write_device};

use xml::reader::EventReader;

use std::fs::File;
use std::io::{BufReader};

use clap::{Arg, App};

fn main() {
    let matches = App::new("svd2chip")
        .arg(Arg::with_name("input"))
        .get_matches();

    if !matches.is_present("input") {
        println!("{}", matches.usage());
        std::process::exit(1);
    }
    let input = matches.value_of("input").unwrap();
    let mut out = std::io::stdout();

    let file = File::open(input).unwrap();
    let file = BufReader::new(file);
    let mut reader = EventReader::new(file);
    let doc = read_document(&mut reader).unwrap();
    let dev = doc.device;

    let mut ctx = Context {
        out: &mut out,
    };

    write_device(&mut ctx, 0, &dev).unwrap();
}
