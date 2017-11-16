extern crate bobbin_chip as chip;
extern crate xml;
extern crate clap;
extern crate bobbin_svd;

use bobbin_svd::*;
use chip::writer::{Context, write_device};

use xml::reader::EventReader;

use std::fs::File;
use std::io::{BufReader, BufWriter};

use clap::{Arg, App};

fn main() {
    let matches = App::new("svd2chip")
        .arg(Arg::with_name("input"))
        .arg(Arg::with_name("output"))
        .get_matches();

    let input = if let Some(input) = matches.value_of("input") {
        BufReader::new(File::open(input).unwrap())
    } else {
        println!("{}", matches.usage());
        std::process::exit(1);
    };

    let mut output = if let Some(output) = matches.value_of("output") {
        BufWriter::new(File::create(output).unwrap())
    } else {
        println!("{}", matches.usage());
        std::process::exit(1);
    };

    let mut reader = EventReader::new(input);
    let doc = read_document(&mut reader).unwrap();
    let dev = doc.device;

    let mut ctx = Context {
        out: &mut output,
    };

    write_device(&mut ctx, 0, &dev).unwrap();
}
