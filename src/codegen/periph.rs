use {Peripheral, Register};
use super::{to_camel, size_type, field_name, field_getter, field_setter, field_with};
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
            let reg_struct = format!("reg::{}", to_camel(&r.name));
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
            let reg_struct = format!("reg::{}", to_camel(&r.name));
            let reg_getter = field_getter(&r.name);
            let reg_setter = field_setter(&r.name);
            let reg_with = field_with(&r.name);
            if let Some(dim) = r.dim {
                writeln!(out, "    pub fn try_{}(&self, index: usize) -> Result<{}, RW::Error> {{", reg_getter, reg_struct)?;
                writeln!(out, "        assert!(index < {});", dim)?;
                writeln!(out, "        Ok({}(self.try_read(REG_{} + index as {})?))", reg_struct, r.name, r_size)?;
                writeln!(out, "    }}")?;
                writeln!(out, "    pub fn try_{}(&self, index: usize, value: {}) -> Result<(), RW::Error> {{", reg_setter, reg_struct)?;
                writeln!(out, "        assert!(index < {});", dim)?;            
                writeln!(out, "        self.try_write(REG_{} + index as {}, value.0)", r.name, r_size)?;
                writeln!(out, "    }}")?;       
                writeln!(out, "    pub fn try_{}<F: FnOnce({}) -> {}>(&self, index: usize, f: F) -> Result<(), RW::Error> {{", reg_with, reg_struct, reg_struct)?;
                writeln!(out, "        assert!(index < {});", dim)?;
                writeln!(out, "        let tmp = {}(self.try_read(REG_{} + index as {})?);", reg_struct, r.name, r_size)?;
                writeln!(out, "        self.try_write(REG_{} + index as {}, f(tmp).0)", r.name, r_size)?;
                writeln!(out, "    }}")?;          
                writeln!(out, "")?;            
            } else {
                writeln!(out, "    pub fn try_{}(&self) -> Result<{}, RW::Error> {{", reg_getter, reg_struct)?;
                writeln!(out, "        Ok({}(self.try_read(REG_{})?))", reg_struct, r.name)?;
                writeln!(out, "    }}")?;
                writeln!(out, "    pub fn try_{}(&self, value: {}) -> Result<(), RW::Error> {{", reg_setter, reg_struct)?;
                writeln!(out, "        self.try_write(REG_{}, value.0)", r.name)?;
                writeln!(out, "    }}")?;       
                writeln!(out, "    pub fn try_{}<F: FnOnce({}) -> {}>(&self, f: F) -> Result<(), RW::Error> {{", reg_with, reg_struct, reg_struct)?;
                writeln!(out, "        let tmp = {}(self.try_read(REG_{})?);", reg_struct, r.name)?;
                writeln!(out, "        self.try_write(REG_{}, f(tmp).0)", r.name)?;
                writeln!(out, "    }}")?;          
                writeln!(out, "")?;
            }
        }
        writeln!(out, "}}")?;
        writeln!(out, "")?;                

        writeln!(out, "pub mod reg {{")?;
        writeln!(out, "")?;                
        for r in p.registers.iter() {
            gen_register(&mut out, r, r_size)?;
        }        
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


pub fn gen_register<W: Write>(out: &mut W, r: &Register, size: &'static str) -> Result<()> {
    let reg_struct = to_camel(&r.name);    

    if let Some(ref description) = r.description {
        gen_doc(out, 4, description)?;
    }
    writeln!(out, "    #[derive(PartialEq, Eq, Clone, Copy)]")?;
    writeln!(out, "")?;
    writeln!(out, "    pub struct {}({});", reg_struct, size)?;
    writeln!(out, "")?;

    writeln!(out, "    impl From<{}> for {} {{", size, reg_struct)?;
    writeln!(out, "        fn from(other: {}) -> Self {{ {}(other) }}", size, reg_struct)?;
    writeln!(out, "    }}")?;    
    writeln!(out, "")?;

    writeln!(out, "    impl From<{}> for {} {{", reg_struct, size)?;
    writeln!(out, "        fn from(other: {}) -> Self {{ other.0 }}", reg_struct)?;
    writeln!(out, "    }}")?;
    writeln!(out, "")?;


    writeln!(out, "    impl {} {{", reg_struct)?;
    writeln!(out, "        pub fn value(&self) -> {} {{ self.0 }}", size)?;
    writeln!(out, "")?;

    for f in r.fields.iter() {
        assert!(f.dim.is_none(), "{}: Field arrays are not currently implemented", f.name);
        let f_getter = field_getter(&f.name);
        let f_setter = field_setter(&f.name);
        let f_offset = f.bit_offset;
        let f_width = f.bit_width;
        let f_lo = f_offset;
        let f_hi = (f_offset + f_width) - 1;    
        let f_mask = if f_width == 64 {
            u64::max_value()
        } else {
            ((1 << f_width) - 1)
        };
        let f_bits = if f_width > 1 {
            format!("[{}:{}]", f_hi, f_lo)
        } else {
            format!("[{}]", f_lo)
        };  
        if let Some(ref description) = f.description {
            gen_doc(out, 8, description)?;
        }                  
        writeln!(out, "        pub fn {}(&self) -> {} {{", f_getter, size)?;
        writeln!(out, "            ((self.0 as {}) >> {}) & 0x{:x} // {}", size, f_offset, f_mask, f_bits)?;
        writeln!(out, "        }}")?;    
        writeln!(out, "    ")?;

        writeln!(out, "        pub fn {}(mut self, value: {}) -> Self {{", f_setter, size)?;
        writeln!(out, "            assert!((value & !0x{:x}) == 0);", f_mask)?;
        writeln!(out, "            self.0 &= !(0x{:x} << {});", f_mask, f_offset)?;
        writeln!(out, "            self.0 |= value << {};", f_offset)?;
        writeln!(out, "            self")?;
        writeln!(out, "        }}")?;    
        writeln!(out, "    ")?;
    }
    writeln!(out, "    }}")?;
    writeln!(out, "")?;

    writeln!(out, "    impl ::core::fmt::Display for {} {{", reg_struct)?;
    writeln!(out, "        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{")?;
    writeln!(out, "            self.0.fmt(f)")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;        
    writeln!(out, "")?;


    writeln!(out, "    impl ::core::fmt::Debug for {} {{", reg_struct)?;
    writeln!(out, "        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{")?;
    match size {
        "u8" => writeln!(out, "            write!(f, \"[0x{{:02x}}\", self.0)?;")?,
        "u16" => writeln!(out, "            write!(f, \"[0x{{:04x}}\", self.0)?;")?,
        "u32" => writeln!(out, "            write!(f, \"[0x{{:08x}}\", self.0)?;")?,
        s @ _ => panic!("invalid size: {}", s),
    }        
    
    for f in r.fields.iter() {
        let f_name = field_name(&f.name);
        let f_getter = field_getter(&f.name);

        if let Some(dim) = f.dim {
            for i in 0..dim {
                match f.bit_width {
                    1 => {
                        writeln!(out, "            if self.{}({}) != 0 {{ write!(f, \" {}[{}]\")? }}", f_getter, i, f_getter, i)?;
                    },
                    32 => {},
                    _ => {
                        writeln!(out, "            if self.{}({}) != 0 {{ write!(f, \" {}[{}]=0x{{:x}}\", self.{}({})? }}", f_getter, i, f_name, i, f_getter, i)?;
                    }
                }                    
            }
        } else {
            match f.bit_width {
                1 => {
                    writeln!(out, "            if self.{}() != 0 {{ write!(f, \" {}\")? }}", f_getter, f_getter)?;
                },
                32 => {},
                _ => {
                    writeln!(out, "            if self.{}() != 0 {{ write!(f, \" {}=0x{{:x}}\", self.{}())? }}", f_getter, f_name, f_getter)?;
                }
            }
        }
        
    }
    writeln!(out, "            write!(f, \"]\")?;")?;
    writeln!(out, "            Ok(())")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;        
    writeln!(out, "")?;    

    Ok(())
}


fn gen_doc<W: Write>(out: &mut W, indent: usize, doc: &str) -> Result<()> {
    let doc = doc.trim();
    if doc.len() > 0 {
        try!(writeln!(out, "{:indent$}#[doc=\"{}\"]", "", doc, indent=indent))
    }
    Ok(())
}