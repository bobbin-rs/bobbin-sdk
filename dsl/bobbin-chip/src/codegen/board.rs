use Board;
// use super::modules;

use std::io::{Write, Read, Result};
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};

// use std::collections::HashMap;

pub struct Config {
    pub out_path: PathBuf,
    pub cargo_template: PathBuf,
}

pub fn gen_board<W: Write>(cfg: Config, _out: &mut W, b: &Board) -> Result<()> {
    // Generate Board Type and Traits
    let board_ty = super::to_camel(&b.name.replace("-","_"));
    let mcu_ty = super::to_camel(&b.mcu);

    let tmpl_path = &cfg.cargo_template;
    let out_path = &cfg.out_path;
    mkdir(out_path)?;

    // .cargo/config

    mkdir(&out_path.join(".cargo"))?;
    copy_file_with(&tmpl_path.join(".cargo/config"), &out_path.join(".cargo/config"), |s| {
        s.replace("%target%", &b.target)
    })?;

    copy_file(&tmpl_path.join("Makefile"), &out_path.join("Makefile"))?;
    copy_file(&tmpl_path.join("Xargo.toml"), &out_path.join("Xargo.toml"))?;

    copy_file_with(&tmpl_path.join("Cargo.toml"), &out_path.join("Cargo.toml"), |s| {
        s.replace("%name%", &b.name)
    })?;

    let mut imports = String::new();

    {
        let mut out = OpenOptions::new().append(true).open(out_path.join("Cargo.toml"))?;
        writeln!(out, "")?;
        for c in b.crates.iter() {
            let c_name = c.name.replace("-","_");

            imports.push_str(&format!("pub extern crate {}", c_name));
            if let Some(ref c_as) = c._as {
                imports.push_str(&format!(" as {}", c_as));
            }
            imports.push_str(";\n");

            writeln!(out, "[dependencies.{}]", c.name)?;
            if let Some(ref path) = c.path {
                writeln!(out, "path = {:?}", path)?;
            }
            if let Some(ref version) = c.version {
                writeln!(out, "version = {:?}", version)?;
            }
            if c.features.len() > 0 {
                write!(out, "features = [")?;
                for (i, f) in c.features.iter().enumerate() {
                    if i != 0 { write!(out, ", ")? }
                    write!(out, "{:?}", f)?;

                }
                write!(out, "]")?;
                writeln!(out, "")?;
            }
            if let Some(ref default_features) = c.default_features {
                writeln!(out, "default-features = {}", default_features)?;
            }
            writeln!(out, "")?;            
        }
    }

    let src_dir = tmpl_path.join("src");
    let dst_dir = out_path.join("src");
    mkdir(&dst_dir)?;
    // copy_file_with(&tmpl_path.join("src/lib.rs"), &out_path.join("src/lib.rs"), |s| {
    //     s.replace("%name%", &b.name)        
    // })?;

    for entry in fs::read_dir(&src_dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name();
        if let Some(name) = name {
            let src = src_dir.join(name);
            let dst = dst_dir.join(name);
            if !dst.exists() || name == "lib.rs" {
                let board_name = b.name.replace("-","_");
                copy_file_with(&src, &dst, |s| s
                    .replace("%imports%", &imports)
                    .replace("%board%", &b.name)
                    .replace("%board_name%", &board_name)
                    .replace("%board_type%", &board_ty)
                    .replace("%mcu_type%", &mcu_ty)
                )?;
            }
        }
    }

    // {


    //     let mut out = OpenOptions::new().append(true).open(out_path.join("src/lib.rs"))?;
    //     writeln!(out, "")?;
    //     writeln!(out, "#[derive(Debug, Default)]")?;
    //     writeln!(out, "pub struct {} {{", board_ty)?;
    //     writeln!(out, "   system: System,")?;
    //     writeln!(out, "}}")?;
    //     writeln!(out, "")?;
    //     writeln!(out, "impl bobbin_sys::board::Board for {} {{", board_ty)?;
    //     writeln!(out, "   type System = System;")?;
    //     writeln!(out, "   fn id(&self) -> &'static str {{ {:?} }}", b.name)?;
    //     writeln!(out, "   fn sys(&self) -> &Self::System {{ &self.system }}")?;
    //     writeln!(out, "   fn sys_mut(&self) -> &mut Self::System {{ &mut self.system }}")?;
    //     writeln!(out, "}}")?;
    //     writeln!(out, "")?;
    //     writeln!(out, "")?;
        
    //     writeln!(out, "pub type Board = {};", board_ty)?;
    // }

    Ok(())
}

pub fn mkdir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir(&path)?;
    }
    Ok(())
}
pub fn copy_file(src_path: &Path, dst_path: &Path) -> Result<()> {
    if src_path.exists() {
        copy_file_with(src_path, dst_path, |s| s)
    } else {
        Ok(())
    }
}
pub fn copy_file_with<F: FnOnce(String) -> String>(src_path: &Path, dst_path: &Path, f: F) -> Result<()> {
    let mut src = File::open(src_path)?;
    let mut dst = File::create(dst_path)?;
    let mut data = String::new();
    src.read_to_string(&mut data)?;    
    let data = f(data);
    dst.write(&data.as_bytes())?;    
    Ok(())
}