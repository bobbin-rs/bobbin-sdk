extern crate xml;
extern crate clap;
extern crate svd2chip;


use svd2chip::*;

use xml::reader::{EventReader};

use std::fs::File;
use std::io::{BufReader, Write};

use clap::{Arg, App};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let matches = App::new("svd2chip")
        .arg(Arg::with_name("input"))
        .get_matches();
    if !matches.is_present("input") {
        println!("{}", matches.usage());
    }
    let input = matches.value_of("input").unwrap();
    let mut out = std::io::stdout();

    let file = File::open(input).unwrap();
    let file = BufReader::new(file);
    let mut reader = EventReader::new(file);
    let doc = read_document(&mut reader).unwrap();
    let dev = doc.device;
    write_device(&mut out, 0, &dev).unwrap();
}

fn write_device<W: Write>(out: &mut W, depth: usize, d: &Device) -> std::io::Result<()> {
    try!(writeln!(out, "{}(device", indent(depth)));
    try!(writeln!(out, "{}(name {})", indent(depth + 1), d.name));
    if let Some(ref desc) = d.description {        
        try!(writeln!(out, "{}(description {:?})", indent(depth + 1), desc));
    }
    for p in d.peripherals.iter() {
        try!(write_peripheral(out, depth + 1, p));
    }
    try!(writeln!(out, "{})", indent(depth)));
    Ok(())
}

fn write_peripheral<W: Write>(out: &mut W, depth: usize, d: &Peripheral) -> std::io::Result<()> {
    try!(writeln!(out, "{}(peripheral", indent(depth)));
    try!(writeln!(out, "{}(name {})", indent(depth + 1), d.name));
    if let Some(ref desc) = d.description {        
        try!(writeln!(out, "{}(description {:?})", indent(depth + 1), desc));
    }
    for p in d.registers.iter() {
        try!(write_register(out, depth + 1, p));
    }
    try!(writeln!(out, "{})", indent(depth)));
    Ok(())
}

fn write_register<W: Write>(out: &mut W, depth: usize, d: &Register) -> std::io::Result<()> {
    try!(writeln!(out, "{}(register", indent(depth)));
    try!(writeln!(out, "{}(name {})", indent(depth + 1), d.name));
    if let Some(ref desc) = d.description {        
        try!(writeln!(out, "{}(description {:?})", indent(depth + 1), desc));
    }
    for p in d.fields.iter() {
        try!(write_field(out, depth + 1, p));
    }
    try!(writeln!(out, "{})", indent(depth)));
    Ok(())
}

fn write_field<W: Write>(out: &mut W, depth: usize, d: &Field) -> std::io::Result<()> {
    try!(writeln!(out, "{}(field", indent(depth)));
    try!(writeln!(out, "{}(name {})", indent(depth + 1), d.name));
    try!(writeln!(out, "{}(bits {})", indent(depth + 1), d.bits));
    if let Some(ref access) = d.access {
        try!(writeln!(out, "{}(access {})", indent(depth + 1), access));
    }
    if let Some(ref desc) = d.description {        
        try!(writeln!(out, "{}(description {:?})", indent(depth + 1), desc));
    }
    try!(writeln!(out, "{})", indent(depth)));
    Ok(())
}