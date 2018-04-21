use {Peripheral};
use super::{to_camel, size_type, field_getter, field_setter, field_with, gen_crate_doc};
use super::modules::gen_register_types;
use std::io::{Write, Read, Result};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

// use std::collections::HashMap;

pub struct Config {
    pub out_path: PathBuf,
    pub cargo_template: PathBuf,
}

pub fn gen_periph<W: Write>(cfg: Config, _out: &mut W, p: &Peripheral) -> Result<()> {    
    let p_name = p.name.to_lowercase();
    let p_author = p.author.clone().unwrap_or(String::from("-- NO AUTHOR --"));
    let p_version = p.version.clone().unwrap_or(String::from("0.1"));
    let p_type = to_camel(&p_name);
    let r_size = size_type(p.size.unwrap_or(32));

    let tmpl_path = &cfg.cargo_template;
    let out_path = &cfg.out_path;
    mkdir(out_path)?;

    copy_file_with(&tmpl_path.join("Cargo.toml"), &out_path.join("Cargo.toml"), |s| {
        s.replace("%name%", &p_name).replace("%author%", &p_author).replace("%version%", &p_version)
    })?;

    copy_file(&tmpl_path.join("Makefile"), &out_path.join("Makefile"))?;

    // let src_dir = tmpl_path.join("src");
    let dst_dir = out_path.join("src");
    mkdir(&dst_dir)?;

    let mut out = File::create(out_path.join("src/lib.rs"))?;
    if let Some(ref doc) = p.description {
        gen_crate_doc(&mut out, doc)?;
    }
    append_file(&mut out, &tmpl_path.join("src/lib.rs"))?;
    {
        writeln!(out, "")?;
        writeln!(out, "pub type Addr = {};", r_size)?;
        writeln!(out, "pub type Value = {};", r_size)?;
        writeln!(out, "")?;    

        writeln!(out, "pub mod addr {{")?;
        writeln!(out, "    use super::Addr;")?;
        for r in p.registers.iter() {
            try!(writeln!(out, "    pub const REG_{}: Addr = 0x{:02x};",
                r.name,
                r.offset,
            ));
            if let Some(dim) = r.dim {
                for i in 0..dim {
                    try!(writeln!(out, "    pub const REG_{}{}: Addr = 0x{:02x};",
                        r.name,
                        i + 1,
                        r.offset + i,
                    ));
                    
                }
            }
        }
        writeln!(out, "}}")?;    
        writeln!(out, "")?;

        writeln!(out, "pub struct {}<RW> {{ rw: RW }}", p_type)?;
        writeln!(out, "")?;

        writeln!(out, "pub trait ReadWrite {{")?;
        writeln!(out, "    fn read(&self, addr: Addr) -> Value;")?;
        writeln!(out, "    fn write(&self, addr: Addr, val: Value);")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;

        writeln!(out, "pub trait TryReadWrite {{")?;
        writeln!(out, "    type Error;")?;
        writeln!(out, "    fn try_read(&self, addr: Addr) -> Result<Value, Self::Error>;")?;
        writeln!(out, "    fn try_write(&self, addr: Addr, val: Value) -> Result<(), Self::Error>;")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;

        writeln!(out, "impl<RW: ReadWrite> ReadWrite for {}<RW> {{", p_type)?;
        writeln!(out, "    fn read(&self, addr: Addr) -> Value {{ self.rw.read(addr) }}")?;
        writeln!(out, "    fn write(&self, addr: Addr, val: Value) {{ self.rw.write(addr, val) }}")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;

        writeln!(out, "impl<RW: TryReadWrite> TryReadWrite for {}<RW> {{", p_type)?;
        writeln!(out, "    type Error = RW::Error;")?;
        writeln!(out, "    fn try_read(&self, addr: Addr) -> Result<Value, Self::Error> {{ self.rw.try_read(addr) }}")?;
        writeln!(out, "    fn try_write(&self, addr: Addr, val: Value) -> Result<(), Self::Error> {{ self.rw.try_write(addr, val) }}")?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;

        writeln!(out, "impl<RW> {}<RW> {{", p_type)?;
        writeln!(out, "    pub fn new(rw: RW) -> Self {{ {} {{ rw }} }}", p_type)?;
        writeln!(out, "}}")?;
        writeln!(out, "")?;
        

        writeln!(out, "impl<RW: ReadWrite> {}<RW> {{", p_type)?;
        for r in p.registers.iter() {
            let reg_struct = format!("types::{}", to_camel(&r.name));
            let reg_getter = field_getter(&r.name);
            let reg_setter = field_setter(&r.name);
            let reg_with = field_with(&r.name);
            if let Some(dim) = r.dim {
                writeln!(out, "    pub fn {}(&self, index: usize) -> {} {{", reg_getter, reg_struct)?;
                writeln!(out, "        assert!(index < {});", dim)?;
                writeln!(out, "        {}(self.read(addr::REG_{} + index as {}))", reg_struct, r.name, r_size)?;
                writeln!(out, "    }}")?;
                writeln!(out, "    pub fn {}(&self, index: usize, value: {}) {{", reg_setter, reg_struct)?;
                writeln!(out, "        assert!(index < {});", dim)?;            
                writeln!(out, "        self.write(addr::REG_{} + index as {}, value.0)", r.name, r_size)?;
                writeln!(out, "    }}")?;       
                writeln!(out, "    pub fn {}<F: FnOnce({}) -> {}>(&self, index: usize, f: F) {{", reg_with, reg_struct, reg_struct)?;
                writeln!(out, "        assert!(index < {});", dim)?;
                writeln!(out, "        let tmp = {}(self.read(addr::REG_{} + index as {}));", reg_struct, r.name, r_size)?;
                writeln!(out, "        self.write(addr::REG_{} + index as {}, f(tmp).0)", r.name, r_size)?;
                writeln!(out, "    }}")?;          
                writeln!(out, "")?;            
            } else {
                writeln!(out, "    pub fn {}(&self) -> {} {{", reg_getter, reg_struct)?;
                writeln!(out, "        {}(self.read(addr::REG_{}))", reg_struct, r.name)?;
                writeln!(out, "    }}")?;
                writeln!(out, "    pub fn {}(&self, value: {}) {{", reg_setter, reg_struct)?;
                writeln!(out, "        self.write(addr::REG_{}, value.0)", r.name)?;
                writeln!(out, "    }}")?;       
                writeln!(out, "    pub fn {}<F: FnOnce({}) -> {}>(&self, f: F) {{", reg_with, reg_struct, reg_struct)?;
                writeln!(out, "        let tmp = {}(self.read(addr::REG_{}));", reg_struct, r.name)?;
                writeln!(out, "        self.write(addr::REG_{}, f(tmp).0)", r.name)?;
                writeln!(out, "    }}")?;          
                writeln!(out, "")?;
            }
        }
        writeln!(out, "}}")?;
        writeln!(out, "")?;        

        writeln!(out, "impl<RW: TryReadWrite> {}<RW> {{", p_type)?;
        for r in p.registers.iter() {
            let reg_struct = format!("types::{}", to_camel(&r.name));
            let reg_getter = field_getter(&r.name);
            let reg_setter = field_setter(&r.name);
            let reg_with = field_with(&r.name);
            if let Some(dim) = r.dim {
                writeln!(out, "    pub fn try_{}(&self, index: usize) -> Result<{}, RW::Error> {{", reg_getter, reg_struct)?;
                writeln!(out, "        assert!(index < {});", dim)?;
                writeln!(out, "        Ok({}(self.try_read(addr::REG_{} + index as {})?))", reg_struct, r.name, r_size)?;
                writeln!(out, "    }}")?;
                writeln!(out, "    pub fn try_{}(&self, index: usize, value: {}) -> Result<(), RW::Error> {{", reg_setter, reg_struct)?;
                writeln!(out, "        assert!(index < {});", dim)?;            
                writeln!(out, "        self.try_write(addr::REG_{} + index as {}, value.0)", r.name, r_size)?;
                writeln!(out, "    }}")?;       
                writeln!(out, "    pub fn try_{}<F: FnOnce({}) -> {}>(&self, index: usize, f: F) -> Result<(), RW::Error> {{", reg_with, reg_struct, reg_struct)?;
                writeln!(out, "        assert!(index < {});", dim)?;
                writeln!(out, "        let tmp = {}(self.try_read(addr::REG_{} + index as {})?);", reg_struct, r.name, r_size)?;
                writeln!(out, "        self.try_write(addr::REG_{} + index as {}, f(tmp).0)", r.name, r_size)?;
                writeln!(out, "    }}")?;          
                writeln!(out, "")?;            
            } else {
                writeln!(out, "    pub fn try_{}(&self) -> Result<{}, RW::Error> {{", reg_getter, reg_struct)?;
                writeln!(out, "        Ok({}(self.try_read(addr::REG_{})?))", reg_struct, r.name)?;
                writeln!(out, "    }}")?;
                writeln!(out, "    pub fn try_{}(&self, value: {}) -> Result<(), RW::Error> {{", reg_setter, reg_struct)?;
                writeln!(out, "        self.try_write(addr::REG_{}, value.0)", r.name)?;
                writeln!(out, "    }}")?;       
                writeln!(out, "    pub fn try_{}<F: FnOnce({}) -> {}>(&self, f: F) -> Result<(), RW::Error> {{", reg_with, reg_struct, reg_struct)?;
                writeln!(out, "        let tmp = {}(self.try_read(addr::REG_{})?);", reg_struct, r.name)?;
                writeln!(out, "        self.try_write(addr::REG_{}, f(tmp).0)", r.name)?;
                writeln!(out, "    }}")?;          
                writeln!(out, "")?;
            }
        }
        writeln!(out, "}}")?;
        writeln!(out, "")?;                

        writeln!(out, "pub mod types {{")?;
        writeln!(out, "")?;                
        gen_register_types(&mut out, &p.registers, p.size, p.access)?;
        // for r in p.registers.iter() {
        //     gen_register(&mut out, r, r_size)?;
        // }        
        writeln!(out, "")?;                
        writeln!(out, "}}")?;
        writeln!(out, "")?;                

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

pub fn append_file<W: Write>(out: &mut W, src_path: &Path) -> Result<()> {
    let mut src = File::open(src_path)?;
    let mut data = String::new();
    src.read_to_string(&mut data)?;    
    out.write(&data.as_bytes())?;    
    Ok(())
}