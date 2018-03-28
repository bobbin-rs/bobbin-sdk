use Peripheral;
use super::{to_camel, size_type};
use std::io::{Write, Read, Result};
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};

// use std::collections::HashMap;

pub struct Config {
    pub out_path: PathBuf,
    pub cargo_template: PathBuf,
}

pub fn gen_periph<W: Write>(cfg: Config, _out: &mut W, p: &Peripheral) -> Result<()> {    
    let p_name = p.name.to_lowercase();
    let p_type = to_camel(&p_name);
    let r_size = size_type(p.size.unwrap_or(32));

    let tmpl_path = &cfg.cargo_template;
    let out_path = &cfg.out_path;
    mkdir(out_path)?;

    copy_file_with(&tmpl_path.join("Cargo.toml"), &out_path.join("Cargo.toml"), |s| {
        s.replace("%name%", &p_name)
    })?;
    
    copy_file(&tmpl_path.join("Makefile"), &out_path.join("Makefile"))?;

    // let src_dir = tmpl_path.join("src");
    let dst_dir = out_path.join("src");
    mkdir(&dst_dir)?;

    copy_file_with(&tmpl_path.join("src/lib.rs"), &out_path.join("src/lib.rs"), |s| {
        s.replace("%name%", &p_name)        
    })?;


    let mut out = OpenOptions::new().append(true).open(out_path.join("src/lib.rs"))?;

    {
        try!(writeln!(out, "pub trait ReadWrite {{"));
        try!(writeln!(out, "  fn read(&self, rw: {}) -> {};", r_size, r_size));
        try!(writeln!(out, "  fn write(&self, rw: {}, val: {});", r_size, r_size));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));
        try!(writeln!(out, "pub trait TryReadWrite {{"));
        try!(writeln!(out, "  type Error;"));
        try!(writeln!(out, "  fn try_read(&self, rw: {}) -> Result<{}, Self::Error>;", r_size, r_size));
        try!(writeln!(out, "  fn try_write(&self, rw: {}, val: {}) -> Result<(), Self::Error>;", r_size, r_size));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));        
        try!(writeln!(out, "pub struct {} {{}}", p_type));
    }

    Ok(())
}

pub fn mkdir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir(&path)?;
    }
    Ok(())
}
pub fn copy_file(src_path: &Path, dst_path: &Path) -> Result<()> {
    copy_file_with(src_path, dst_path, |s| s)
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
