use Device;
use super::modules;

use std::io::{Write, Read, Result};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

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


    let src_path = cfg.out_path.join("src");
    if !src_path.exists() {
        fs::create_dir(&src_path)?;
    }

    let cfg = modules::Config { 
        path: src_path.clone(), 
        is_root: true, 
        common: String::from("bobbin_chip_common"),
    };
    let mut out = File::create(src_path.clone().join("lib.rs"))?;
    writeln!(out, "#![no_std]")?;
    writeln!(out, "#![cfg_attr(target_os=\"none\", feature(compiler_builtins_lib))]")?;
    writeln!(out, "#[cfg(target_os=\"none\")] extern crate compiler_builtins;")?;
    writeln!(out, "#[macro_use] extern crate bobbin_chip_common;")?;
    writeln!(out, "#[allow(unused_imports)] use bobbin_chip_common::*;")?;    
    modules::gen_mod(&cfg, &mut out, d, &src_path)?;

    Ok(())
}

pub fn gen_cargo_toml(path: &Path) -> Result<()> {
    let mut out = File::create(path)?;

    writeln!(out, "[package]")?;

    Ok(())
    
}