extern crate bobbin_chip as chip;
extern crate xml;
extern crate clap;
extern crate bobbin_svd;

use bobbin_svd::*;
use chip::writer::*;

use xml::reader::EventReader;

use std::path::Path;
use std::fs::{File, create_dir};
use std::io::{BufReader, BufWriter, Write};
use std::collections::{HashSet, HashMap};

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
    
    let output_path = output_dir.join("lib.rx");        

    let mut reader = EventReader::new(BufReader::new(File::open(input_path).unwrap()));
    let mut writer = BufWriter::new(File::create(&output_path).unwrap());
    let doc = read_document(&mut reader).unwrap();
    let mut dev = doc.device;

    // Set group name for all peripherals

    for p in dev.peripherals.iter_mut() {
        if p.group_name.is_none() {
            p.group_name = Some(p.name.clone());
        }
        // println!("{} {:?}", p.name, p.group_name);
    }


    // Check for periphs that should not be grouped together in same group
    {
        let mut pg_map: HashMap<String, u64> = HashMap::new();
        let mut pg_set: HashSet<String> = HashSet::new();

        for p in dev.peripherals.iter() {
            if let Some(ref group_name) = p.group_name {
                let sig = p.signature();
                if let Some(pg_sig) = pg_map.get(group_name) {
                    if *pg_sig != sig {
                        pg_set.insert(group_name.clone());
                    }
                }
                pg_map.insert(group_name.clone(), sig);
            }        
        }
        if !pg_set.is_empty() {
            for p in dev.peripherals.iter_mut() {
                let modify_p = if let Some(ref group_name) = p.group_name {
                    pg_set.contains(group_name)
                } else {
                    false
                };
                if modify_p {
                    p.group_name = Some(p.name.clone());
                }
            }
        }
    }

    // Set group name for derived peripherals or ones with same signature
    {
        let mut p_map: HashMap<String, String> = HashMap::new();
        let mut s_map: HashMap<u64, String> = HashMap::new();
        for p in dev.peripherals.iter() {            
            if let Some(ref group_name) = p.group_name {
                p_map.insert(p.name.clone(), group_name.clone());
                let sig = p.signature();
                if !s_map.contains_key(&sig) {
                    s_map.insert(sig, group_name.clone());
                }
            }
        }
        for p in dev.peripherals.iter_mut() {
            let sig = p.signature();
            if let Some(ref derived_from) = p.derived_from {
                if let Some(pg) = p_map.get(derived_from) {
                    p.group_name = Some(pg.clone())
                } else {
                    println!("{}: attempted to derive from {} which does not exist", p.name, derived_from);
                }
            } else if s_map.contains_key(&sig) {
                p.group_name = Some(s_map.get(&sig).unwrap().clone());
            }
        }
    }

    // Write peripheral groups

    let pg_list = {
        let mut pg_list: Vec<(String, u64)> = Vec::new();
        let mut pg_set: HashSet<String> = HashSet::new();

        let periph_dir = output_dir.join("periph");
        if !periph_dir.exists() {
            create_dir(&periph_dir).unwrap();
        }

        for p in dev.peripherals.iter() {
            if let Some(ref pg_name) = p.group_name {
                if pg_set.contains(pg_name) {
                    continue
                }
                pg_set.insert(pg_name.clone());
                pg_list.push((pg_name.clone(), p.signature()));

                let mut p = p.clone();
                p.name = String::new();
                p.address = 0;
                p.interrupts = Vec::new();


                let pg_path = periph_dir.join(pg_name.to_lowercase()).with_extension("rx");
                let mut writer = BufWriter::new(File::create(pg_path).unwrap());
                let mut ctx = Context {
                    out: &mut writer,
                };        
                write_peripheral(&mut ctx, 0, &p).unwrap();

            }
        }
        pg_list
    };

    // Write main module

    {
        let mut ctx = Context {
            out: &mut writer,
        };
        
        write_device_open(&mut ctx, 0, &dev).unwrap();
        for &(ref pg, sig) in pg_list.iter() {
            writeln!(ctx.out, "{}(peripheral-group", indent(1)).unwrap();
            writeln!(ctx.out, "{}; signature: {:016x}", indent(2), sig).unwrap();
            writeln!(ctx.out, "{}(name {})", indent(2), pg).unwrap();
            writeln!(ctx.out, "{}(prototype \"{}\")", indent(2), &format!("periph/{}.rx", pg.to_lowercase())).unwrap();

            for p in dev.peripherals.iter() {
                if p.group_name.as_ref() != Some(pg) {
                    continue;
                }
                let periph_name = &p.name;
                writeln!(ctx.out, "{}(peripheral", indent(2)).unwrap();
                writeln!(ctx.out, "{}(name {})", indent(3), periph_name).unwrap();                
                writeln!(ctx.out, "{}(address 0x{:08x})", indent(3), p.address).unwrap();
                for i in p.interrupts.iter() {
                    write_interrupt(&mut ctx, 3, i).unwrap();
                }                
                writeln!(ctx.out, "{})", indent(2)).unwrap();
            }
            writeln!(ctx.out, "{})", indent(1)).unwrap();
        }


        // for p in dev.peripherals.iter() {
        //     let periph_name = &p.name.to_lowercase();
        //     write_peripheral_include(&mut ctx, 1, &format!("periph/{}.rx", periph_name)).unwrap();
        // }
        write_device_close(&mut ctx, 0, &dev).unwrap();
    }


}
