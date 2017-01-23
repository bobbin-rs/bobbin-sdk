extern crate xml;
extern crate clap;
extern crate svd2chip;
extern crate regex;
#[macro_use] extern crate lazy_static;

use svd2chip::*;

use xml::reader::{EventReader};
use regex::Regex;

use std::fs::File;
use std::io::{BufReader, Write};

use clap::{Arg, App};

pub struct Config<'a, W: 'a + Write> {
    out: &'a mut W,
    compact: bool
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let matches = App::new("svd2chip")
        .arg(Arg::with_name("compact")
            .short("c")
            .long("compact"))
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

    let mut cfg = Config {
        out: &mut out,
        compact: matches.is_present("compact")
    };

    write_device(&mut cfg, 0, &dev).unwrap();
}

fn normalize(s: &str) -> std::borrow::Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }
    RE.replace_all(s, " ")
}


fn write_device<W: Write>(cfg: &mut Config<W>, depth: usize, d: &Device) -> std::io::Result<()> {   
    if cfg.compact {
        try!(write!(&mut cfg.out, "{}(device", indent(depth)));
        try!(write!(&mut cfg.out, " {}", d.name));
        if let Some(ref desc) = d.description {        
            try!(write!(&mut cfg.out, " {:?}", normalize(desc)));
        }
        for p in d.peripherals.iter() {
            try!(write_peripheral(cfg, depth + 1, p));
        }
        try!(writeln!(&mut cfg.out, ")"));
    } else {
        try!(writeln!(&mut cfg.out, "{}(device", indent(depth)));
        try!(writeln!(&mut cfg.out, "{}(name {})", indent(depth + 1), d.name));
        if let Some(ref desc) = d.description {        
            try!(writeln!(&mut cfg.out, "{}(description {:?})", indent(depth + 1), normalize(desc)));
        }
        for p in d.peripherals.iter() {
            try!(write_peripheral(cfg, depth + 1, p));
        }
        try!(writeln!(&mut cfg.out, ")"));
    }
    Ok(())
}

fn write_peripheral<W: Write>(cfg: &mut Config<W>, depth: usize, d: &Peripheral) -> std::io::Result<()> {    
    if cfg.compact {
        try!(write!(&mut cfg.out, "\n{}(peripheral", indent(depth)));
        if let Some(ref group_name) = d.group_name {
            if group_name != &d.name {
                try!(write!(&mut cfg.out, " ({} {})", group_name, d.name));
            } else {
                try!(write!(&mut cfg.out, " {}", d.name));
            }
        } else {
            try!(write!(&mut cfg.out, " {}", d.name));
        }
        try!(write!(&mut cfg.out, " {}", d.address.to_lowercase()));
        if let Some(ref derived_from) = d.derived_from {
            try!(write!(&mut cfg.out, " (derived-from {})", derived_from))
        }        
        if let Some(ref desc) = d.description {        
            try!(write!(&mut cfg.out, " {:?}", normalize(&desc)));
        }
        for p in d.interrupts.iter() {
            try!(write_interrupt(cfg, depth + 1, p));
        }
        for p in d.clusters.iter() {
            try!(write_cluster(cfg, depth + 1, p));
        }
        for p in d.registers.iter() {
            try!(write_register(cfg, depth + 1, p));
        }

        try!(write!(&mut cfg.out, ")"));
    } else {
        try!(writeln!(&mut cfg.out, "{}(peripheral", indent(depth)));
        try!(writeln!(&mut cfg.out, "{}(name {})", indent(depth + 1), d.name));
        try!(writeln!(&mut cfg.out, "{}(address {})", indent(depth + 1), d.address.to_lowercase()));
        if let Some(ref group_name) = d.group_name {
            try!(writeln!(&mut cfg.out, "{}(group {})", indent(depth + 1), group_name));
        }        
        if let Some(ref derived_from) = d.derived_from {
            try!(writeln!(&mut cfg.out, "{}(derived-from {})", indent(depth + 1), derived_from))
        }
        if let Some(ref desc) = d.description {        
            try!(writeln!(&mut cfg.out, "{}(description {:?})", indent(depth + 1), normalize(desc)));
        }
        for p in d.clusters.iter() {
            try!(write_cluster(cfg, depth + 1, p));
        }        
        for p in d.registers.iter() {
            try!(write_register(cfg, depth + 1, p));
        }
        try!(writeln!(&mut cfg.out, "{})", indent(depth)));
    }
    Ok(())
}

fn write_interrupt<W: Write>(cfg: &mut Config<W>, depth: usize, d: &Interrupt) -> std::io::Result<()> {
    if cfg.compact {
        try!(write!(&mut cfg.out, "\n{}(interrupt", indent(depth)));
        try!(write!(&mut cfg.out, " {}", d.name));
        try!(write!(&mut cfg.out, " {}", d.value));
        if let Some(ref desc) = d.description {        
            try!(write!(&mut cfg.out, " {:?}", normalize(desc)));
        }        
        try!(write!(&mut cfg.out, ")"));
    } else {
        try!(writeln!(&mut cfg.out, "{}(interrupt", indent(depth)));
        try!(writeln!(&mut cfg.out, "{}(name {})", indent(depth + 1), d.name));
        try!(writeln!(&mut cfg.out, "{}(value {})", indent(depth + 1), d.value));
        if let Some(ref desc) = d.description {        
            try!(writeln!(&mut cfg.out, "{}(description {:?})", indent(depth + 1), normalize(desc)));
        }
        try!(writeln!(&mut cfg.out, "{})", indent(depth)));
    }
    Ok(())
}



fn write_cluster<W: Write>(cfg: &mut Config<W>, depth: usize, d: &Cluster) -> std::io::Result<()> {
    if cfg.compact {
        try!(write!(&mut cfg.out, "\n{}(cluster", indent(depth)));
        try!(write!(&mut cfg.out, " {}", d.name));
        try!(write!(&mut cfg.out, " {}", d.offset.to_lowercase()));
        if let Some(ref desc) = d.description {        
            try!(write!(&mut cfg.out, " {:?}", normalize(desc)));
        }        
        for p in d.registers.iter() {
            try!(write_register(cfg, depth + 1, p));
        }
        try!(write!(&mut cfg.out, ")"));
    } else {
        try!(writeln!(&mut cfg.out, "{}(cluster", indent(depth)));
        try!(writeln!(&mut cfg.out, "{}(name {})", indent(depth + 1), d.name));
        try!(writeln!(&mut cfg.out, "{}(offset {})", indent(depth + 1), d.offset.to_lowercase()));
        if let Some(ref desc) = d.description {        
            try!(writeln!(&mut cfg.out, "{}(description {:?})", indent(depth + 1), normalize(desc)));
        }
        for p in d.registers.iter() {
            try!(write_register(cfg, depth + 1, p));
        }
        try!(writeln!(&mut cfg.out, "{})", indent(depth)));
    }
    Ok(())
}

fn write_register<W: Write>(cfg: &mut Config<W>, depth: usize, d: &Register) -> std::io::Result<()> {
    if cfg.compact {
        
        if d.dim.is_some() {
            try!(write!(&mut cfg.out, "\n{}(register-array", indent(depth)));
            try!(write!(&mut cfg.out, " {:?}", d.name));            
        } else {
            try!(write!(&mut cfg.out, "\n{}(register", indent(depth)));
            try!(write!(&mut cfg.out, " {}", d.name));
        }        
        try!(write!(&mut cfg.out, " {}", d.offset.to_lowercase()));
        if let Some(ref dim) = d.dim {
            try!(write!(&mut cfg.out, " (dim {})", dim));
        }
        if let Some(ref dim_increment) = d.dim_increment {
            try!(write!(&mut cfg.out, " (dim_increment {})", dim_increment));
        }
        if let Some(ref dim_index) = d.dim_index {
            try!(write!(&mut cfg.out, " (dim_index {})", dim_index));
        }
        if let Some(ref reset_value) = d.reset_value {
            if reset_value != "0x00000000" && reset_value != "0" {
                try!(write!(&mut cfg.out, " (reset {})", reset_value));
            }            
        }
        if let Some(ref desc) = d.description {
            try!(write!(&mut cfg.out, " {:?}", normalize(desc)));
        }        
        for p in d.fields.iter() {
            try!(write_field(cfg, depth + 1, p));
        }
        try!(write!(&mut cfg.out, ")"));
    } else {
        try!(writeln!(&mut cfg.out, "{}(register", indent(depth)));
        try!(writeln!(&mut cfg.out, "{}(name {})", indent(depth + 1), d.name));
        try!(writeln!(&mut cfg.out, "{}(offset {})", indent(depth + 1), d.offset.to_lowercase()));
        if let Some(ref dim) = d.dim {
            try!(write!(&mut cfg.out, "{}(dim {})", indent(depth + 1), dim));
        }
        if let Some(ref dim_increment) = d.dim_increment {
            try!(write!(&mut cfg.out, "{}(dim_increment {})", indent(depth + 1), dim_increment));
        }        
        if let Some(ref dim_index) = d.dim_index {
            try!(write!(&mut cfg.out, "{}(dim_index {})", indent(depth + 1), dim_index));
        }
        if let Some(ref reset_value) = d.reset_value {
            if reset_value != "0x00000000" {
                try!(write!(&mut cfg.out, "{}(reset {})", indent(depth + 1), reset_value));
            }
        }        
        if let Some(ref desc) = d.description {        
            try!(writeln!(&mut cfg.out, "{}(description {:?})", indent(depth + 1), normalize(desc)));
        }
        for p in d.fields.iter() {
            try!(write_field(cfg, depth + 1, p));
        }
        try!(writeln!(&mut cfg.out, "{})", indent(depth)));
    }
    Ok(())
}

fn write_field<W: Write>(cfg: &mut Config<W>, depth: usize, d: &Field) -> std::io::Result<()> {
    if cfg.compact {        
        try!(write!(&mut cfg.out, "\n{}(field", indent(depth)));
        try!(write!(&mut cfg.out, " {}", d.name));
        try!(write!(&mut cfg.out, " {}", d.bits));
        if let Some(ref access) = d.access {
            if access != "read-write" {
                try!(write!(&mut cfg.out, " {}", access));
            }            
        }
        if let Some(ref desc) = d.description {        
            if desc != &d.name {
                try!(write!(&mut cfg.out, " {:?}", normalize(desc)));
            }            
        }
        for v in d.enumerated_values.iter() {
            try!(write_enumerated_value(cfg, depth + 1, v));
        }
        try!(write!(&mut cfg.out, ")"));            
    } else {
        try!(writeln!(&mut cfg.out, "{}(field", indent(depth)));
        try!(writeln!(&mut cfg.out, "{}(name {})", indent(depth + 1), d.name));
        try!(writeln!(&mut cfg.out, "{}(bits {})", indent(depth + 1), d.bits));
        if let Some(ref access) = d.access {
            try!(writeln!(&mut cfg.out, "{}(access {})", indent(depth + 1), access));
        }
        if let Some(ref desc) = d.description {        
            try!(writeln!(&mut cfg.out, "{}(description {:?})", indent(depth + 1), normalize(desc)));
        }
        try!(writeln!(&mut cfg.out, "{})", indent(depth)));            
    }

    Ok(())
}

fn write_enumerated_value<W: Write>(cfg: &mut Config<W>, depth: usize, d: &EnumeratedValue) -> std::io::Result<()> {
    if cfg.compact {
        try!(write!(&mut cfg.out, "\n{}(value {}", indent(depth), d.value.replace("#","0b")));
        // if let Some(ref name) = d.name {
        //     try!(write!(&mut cfg.out, " {}", name));
        // }
        if let Some(ref desc) = d.description {
            try!(write!(&mut cfg.out, " {:?}", desc));
        }        
        try!(write!(&mut cfg.out, ")"));
    } else {
        try!(writeln!(&mut cfg.out, "{}(value", indent(depth)));
        try!(writeln!(&mut cfg.out, "{}(value {})", indent(depth + 1), d.value.replace("#","0b")));
        // if let Some(ref name) = d.name {
        //     try!(writeln!(&mut cfg.out, "{}(name {})", indent(depth + 1), name));
        // }
        if let Some(ref desc) = d.description {
            try!(writeln!(&mut cfg.out, "{}(desc {})", indent(depth + 1), desc));
        }
        try!(writeln!(&mut cfg.out, "{})", indent(depth)));            
    }
    Ok(())
}
