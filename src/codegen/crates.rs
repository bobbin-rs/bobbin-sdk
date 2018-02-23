use Device;
use super::modules;

use std::io::{Write, Read, Result};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

pub struct Config {
    pub out_path: PathBuf,
    pub cargo_template: PathBuf,
}

pub fn gen_crate<W: Write>(cfg: Config, _out: &mut W, d: &Device) -> Result<()> {
    // Copy cargo.toml from template
    {
        let cargo_toml_src = cfg.cargo_template.join("Cargo.toml");
        let mut src = File::open(cargo_toml_src)?;
        let mut data = String::new();
        src.read_to_string(&mut data)?;    
        let data = data.replace("%name%", &d.name.to_lowercase());

        let cargo_toml_dst = cfg.out_path.join("Cargo.toml");
        let mut dst = File::create(cargo_toml_dst)?;
        dst.write(&data.as_bytes())?;
    }

    // Copy cargo.toml from template
    {
        let xargo_toml_src = cfg.cargo_template.join("Xargo.toml");
        let mut src = File::open(xargo_toml_src)?;
        let mut data = String::new();
        src.read_to_string(&mut data)?;    
        let xargo_toml_dst = cfg.out_path.join("Xargo.toml");
        let mut dst = File::create(xargo_toml_dst)?;
        dst.write(&data.as_bytes())?;
    }
    // Copy .cargo/config
    {
        let ccfg_dir = cfg.out_path.join(".cargo");
        if !ccfg_dir.exists() {
            fs::create_dir(&ccfg_dir)?;
        }
        let ccfg_src = cfg.cargo_template.join(".cargo/config");
        let mut src = File::open(ccfg_src)?;
        let mut data = String::new();
        src.read_to_string(&mut data)?;    
        let ccfg_dst = ccfg_dir.join("config");
        let mut dst = File::create(ccfg_dst)?;
        dst.write(&data.as_bytes())?;        
    }

    let src_path = cfg.out_path.join("src");
    if !src_path.exists() {
        fs::create_dir(&src_path)?;
    }

    let periph_path = src_path.join("periph/");
    if !periph_path.exists() {
        fs::create_dir(&periph_path)?;
    }

    let hal_path = src_path.join("hal/");
    if !hal_path.exists() {
        fs::create_dir(&hal_path)?;
    }

    let map_path = src_path.join("map/");
    if !map_path.exists() {
        fs::create_dir(&map_path)?;
    }    


    let mut out = File::create(src_path.clone().join("lib.rs"))?;
    writeln!(out, "#![no_std]")?;
    writeln!(out, "#![feature(global_asm, used)]")?;
    // writeln!(out, "#![cfg_attr(target_os=\"none\", feature(compiler_builtins_lib))]")?;
    // writeln!(out, "#[cfg(target_os=\"none\")] extern crate compiler_builtins;")?;
    writeln!(out, "#[macro_use] extern crate bobbin_chip_common;")?;
    writeln!(out, "#[allow(unused_imports)] use bobbin_chip_common::*;")?;    
    writeln!(out, "")?;    

    let cfg = modules::Config { 
        path: src_path.clone(), 
        is_root: false, 
        common: String::from("bobbin_chip_common"),
    };

    writeln!(out, "pub mod periph;")?;    
    let mut periph_out = File::create(periph_path.clone().join("mod.rs"))?;
    gen_periph_mod(&cfg, &mut periph_out, d, &periph_path)?;

    writeln!(out, "pub mod hal;")?;    
    let mut hal_out = File::create(hal_path.clone().join("mod.rs"))?;
    gen_hal_mod(&cfg, &mut hal_out, d, &hal_path)?;


    writeln!(out, "pub mod map;")?;    
    let mut map_out = File::create(map_path.clone().join("mod.rs"))?;
    gen_map_mod(&cfg, &mut map_out, d, &map_path)?;

    writeln!(out, "")?;    
    writeln!(out, "pub use map::*;")?;
    writeln!(out, "")?;    

    Ok(())
}

pub fn gen_cargo_toml(path: &Path) -> Result<()> {
    let mut out = File::create(path)?;

    writeln!(out, "[package]")?;

    Ok(())
    
}


pub fn gen_periph_mod<W: Write>(cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {
    try!(writeln!(out, ""));
    // Preflight Checks

    // Check for duplicate module names
    // let bool precheck_ok = true;

    {
        let mut mods: HashSet<String> = HashSet::new();
        for p in d.peripherals.iter() {            
            if let Some(ref group_name) = p.group_name {
                if mods.contains(group_name) {
                    panic!("Module {} defined more than once in {}", group_name, p.name);
                }
                mods.insert(group_name.clone());
            }
        }

    }


    // Generate Imports

    for c in d.crates.iter() {
        // NOTE: crates now need to be imported from crate root
        // try!(writeln!(out, "extern crate {};", c.name));
        for m in c.modules.iter() {
            if let Some(ref use_as) = m._as {
                try!(writeln!(out, "pub use {}::{} as {};", c.name, m.name, use_as));
            } else {
                try!(writeln!(out, "pub use {}::{};", c.name, m.name));
            }
        }
        try!(writeln!(out, ""));
    }

    // Generate Exceptions
    if d.exceptions.len() > 0 {
        let p_name = "exc";
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(modules::gen_exceptions(cfg, &mut f_mod, &d.exceptions));
    }

    for p in d.peripherals.iter() {
        let p_name = p.group_name.as_ref().unwrap_or(&p.name).to_lowercase();
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(modules::gen_peripheral_impl(cfg, &mut f_mod, p));
    }

    for pg in d.peripheral_groups.iter() {
        let pg_name = pg.name.to_lowercase();
        try!(writeln!(out, "pub mod {};", pg_name));
        let p_mod = path.join(format!("{}.rs", pg_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(modules::gen_peripheral_group_impl(cfg, &mut f_mod, pg));
    }
    Ok(())
}

pub fn gen_hal_mod<W: Write>(_cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {

    for p in d.peripherals.iter() {
        let p_name = p.group_name.as_ref().unwrap_or(&p.name).to_lowercase();
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        if !p_mod.exists() {
            let mut f_mod = try!(File::create(p_mod));
            try!(write!(f_mod, "pub use periph::{}::*;", p_name));
        }
    }

    for pg in d.peripheral_groups.iter() {
        let pg_name = pg.name.to_lowercase();
        try!(writeln!(out, "pub mod {};", pg_name));
        let p_mod = path.join(format!("{}.rs", pg_name));
        if !p_mod.exists() {
            let mut f_mod = try!(File::create(p_mod));
            try!(write!(f_mod, "pub use periph::{}::*;", pg_name));
        }
    }
    Ok(())
}

pub fn gen_map_mod<W: Write>(cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {

    for p in d.peripherals.iter() {
        let p_name = p.group_name.as_ref().unwrap_or(&p.name).to_lowercase();
        writeln!(out, "pub mod {};", p_name)?;;
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        writeln!(f_mod, "use hal::{}::*;", p_name)?;
        writeln!(f_mod, "")?;
        try!(modules::gen_peripheral(cfg, &mut f_mod, p));
    }

    for pg in d.peripheral_groups.iter() {
        let pg_name = pg.name.to_lowercase();
        writeln!(out, "pub mod {};", pg_name)?;;
        let p_mod = path.join(format!("{}.rs", pg_name));
        let mut f_mod = try!(File::create(p_mod));
        writeln!(f_mod, "use hal::{}::*;", pg_name)?;
        writeln!(f_mod, "")?;
        try!(modules::gen_peripheral_group(&cfg, &mut f_mod, pg));
    }

    gen_signals_mod(&cfg, out, d, path)?;
    gen_interrupts_mod(&cfg, out, d, path)?; 

    Ok(())
}


pub fn gen_signals_mod<W: Write>(cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {
    // Generate Signals

    {    
        let p_name = "sig";
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(modules::gen_signals(cfg, &mut f_mod, &d));
    }
    
    Ok(())
}

pub fn gen_interrupts_mod<W: Write>(cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {
    // Generate Interrupts

    {
        let interrupt_count = d.interrupt_count();
        let p_name = "irq";
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(modules::gen_interrupts(&cfg, &mut f_mod, &d, interrupt_count));
    }
    
    Ok(())
}