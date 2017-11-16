extern crate bobbin_chip as chip;
extern crate xml;
extern crate clap;
extern crate bobbin_svd;

use bobbin_svd::*;
use chip::writer::*;

use xml::reader::EventReader;

use std::path::Path;
use std::fs::{File, create_dir};
use std::io::{BufReader, BufWriter};

use clap::{Arg, App};

fn main() {
    let matches = App::new("svd2chip")
        .arg(Arg::with_name("input"))
        .arg(Arg::with_name("output"))
        .get_matches();

    let input_path = if let Some(input) = matches.value_of("input") {
        Path::new(input)
    } else {
        println!("{}", matches.usage());
        std::process::exit(1);
    };

    let output_dir = if let Some(output) = matches.value_of("output") {
        Path::new(output)
    } else {
        println!("{}", matches.usage());
        std::process::exit(1);
    };    
    if !output_dir.is_dir() {
        println!("Output argument must be directory");
        std::process::exit(1);
    }
    
    let output_path = output_dir.with_file_name("lib.rx");        

    let mut reader = EventReader::new(BufReader::new(File::open(input_path).unwrap()));
    let mut writer = BufWriter::new(File::create(&output_path).unwrap());
    let doc = read_document(&mut reader).unwrap();
    let dev = doc.device;

    let mut ctx = Context {
        out: &mut writer,
    };

    write_device_open(&mut ctx, 0, &dev).unwrap();
    for p in dev.peripherals.iter() {
        let periph_name = &p.name.to_lowercase();
        write_peripheral_include(&mut ctx, 1, &format!("periph/{}.rx", periph_name)).unwrap();
    }
    write_device_close(&mut ctx, 0, &dev).unwrap();

    let periph_dir = output_dir.join("periph");
    if !periph_dir.exists() {
        create_dir(&periph_dir).unwrap();
    }

    for p in dev.peripherals.iter() {
        let periph_name = &p.name.to_lowercase();
        let periph_path = periph_dir.join(periph_name).with_extension("rx");
        let mut writer = BufWriter::new(File::create(periph_path).unwrap());
        let mut ctx = Context {
            out: &mut writer,
        };        
        write_peripheral(&mut ctx, 0, p).unwrap();
    }
}
