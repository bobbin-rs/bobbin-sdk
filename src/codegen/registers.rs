use std::io::{Write, Result};
use clap::{ArgMatches};
use {Device, Register};
use super::{size_type, field_getter, field_setter, field_with, field_name, to_camel};

pub fn gen_registers<W: Write>(matches: &ArgMatches, out: &mut W, d: &Device) -> Result<()> {
    if matches.is_present("panic") {
        gen_registers_panic(matches, out, d)
    } else {
        gen_registers_nopanic(matches, out, d)
    }
}

pub fn gen_registers_panic<W: Write>(_matches: &ArgMatches, out: &mut W, d: &Device) -> Result<()> {
    assert!(d.peripherals.len() == 1, "Only one peripheral allowed");
    let p = &d.peripherals[0];
    let r_size = size_type(p.size.or(d.size).unwrap());

    try!(writeln!(out, "/// {}", d.name));
    try!(writeln!(out, ""));

    try!(writeln!(out, "pub trait ReadWrite {{"));
    try!(writeln!(out, "  fn read(&self, rw: {}) -> {};", r_size, r_size));
    try!(writeln!(out, "  fn write(&self, rw: {}, val: {});", r_size, r_size));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    for r in p.registers.iter() {
        try!(writeln!(out, "pub const REG_{}: {} = 0x{:02x};",
            r.name,
            r_size,
            r.offset,
        ));
        if let Some(dim) = r.dim {
            for i in 0..dim {
                try!(writeln!(out, "pub const REG_{}{}: {} = 0x{:02x};",
                    r.name,
                    i + 1,
                    r_size,
                    r.offset + i,
                ));
                
            }
        }
    }

    try!(writeln!(out, ""));
    
    try!(writeln!(out, "pub struct Registers<'a, R: 'a + ReadWrite> {{"));
    try!(writeln!(out, "  rw: &'a R,"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));


    try!(writeln!(out, "impl<'a, R: 'a + ReadWrite> Registers<'a, R> {{"));
    try!(writeln!(out, "  pub fn new(rw: &'a R) -> Self {{"));
    try!(writeln!(out, "    Registers {{ rw: rw }}"));
    try!(writeln!(out, "  }}"));
    try!(writeln!(out, ""));

    for r in p.registers.iter() {
        let reg_struct = to_camel(&r.name);
        let reg_getter = field_getter(&r.name);
        let reg_setter = field_setter(&r.name);
        let reg_with = field_with(&r.name);
        if let Some(dim) = r.dim {
            try!(writeln!(out, "  pub fn {}(&self, index: usize) -> {} {{", reg_getter, reg_struct));
            try!(writeln!(out, "    assert!(index < {});", dim));
            try!(writeln!(out, "    {}(self.rw.read(REG_{} + index as {}))", reg_struct, r.name, r_size));
            try!(writeln!(out, "  }}"));
            try!(writeln!(out, "  pub fn {}(&self, index: usize, value: {}) {{", reg_setter, reg_struct));
            try!(writeln!(out, "    assert!(index < {});", dim));            
            try!(writeln!(out, "    self.rw.write(REG_{} + index as {}, value.0)", r.name, r_size));
            try!(writeln!(out, "  }}"));       
            try!(writeln!(out, "  pub fn {}<F: FnOnce({}) -> {}>(&self, index: usize, f: F) {{", reg_with, reg_struct, reg_struct));
            try!(writeln!(out, "    assert!(index < {});", dim));
            try!(writeln!(out, "    let tmp = {}(self.rw.read(REG_{} + index as {}));", reg_struct, r.name, r_size));
            try!(writeln!(out, "    self.rw.write(REG_{} + index as {}, f(tmp).0)", r.name, r_size));
            try!(writeln!(out, "  }}"));          
            try!(writeln!(out, ""));            
        } else {
            try!(writeln!(out, "  pub fn {}(&self) -> {} {{", reg_getter, reg_struct));
            try!(writeln!(out, "    {}(self.rw.read(REG_{}))", reg_struct, r.name));
            try!(writeln!(out, "  }}"));
            try!(writeln!(out, "  pub fn {}(&self, value: {}) {{", reg_setter, reg_struct));
            try!(writeln!(out, "    self.rw.write(REG_{}, value.0)", r.name));
            try!(writeln!(out, "  }}"));       
            try!(writeln!(out, "  pub fn {}<F: FnOnce({}) -> {}>(&self, f: F) {{", reg_with, reg_struct, reg_struct));
            try!(writeln!(out, "    let tmp = {}(self.rw.read(REG_{}));", reg_struct, r.name));
            try!(writeln!(out, "    self.rw.write(REG_{}, f(tmp).0)", r.name));
            try!(writeln!(out, "  }}"));          
            try!(writeln!(out, ""));
        }
    }
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    for r in p.registers.iter() {
        try!(gen_register(out, r, r_size));
    }
    Ok(())
}

pub fn gen_registers_nopanic<W: Write>(_matches: &ArgMatches, out: &mut W, d: &Device) -> Result<()> {
    assert!(d.peripherals.len() == 1, "Only one peripheral allowed");
    let p = &d.peripherals[0];
    let r_size = size_type(p.size.or(d.size).unwrap());

    try!(writeln!(out, "/// {}", d.name));
    try!(writeln!(out, ""));
    //try!(writeln!(out, "use embed_common::ReadWrite;"));


    try!(writeln!(out, "use ::core::marker::PhantomData;"));
    try!(writeln!(out, ""));

    try!(writeln!(out, "pub trait ReadWrite<E> {{"));
    try!(writeln!(out, "  fn read(&self, rw: {}) -> Result<{}, E>;", r_size, r_size));
    try!(writeln!(out, "  fn write(&self, rw: {}, val: {}) -> Result<(), E>;", r_size, r_size));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    for r in p.registers.iter() {
        try!(writeln!(out, "pub const REG_{}: {} = 0x{:02x};",
            r.name,
            r_size,
            r.offset,
        ));
        if let Some(dim) = r.dim {
            for i in 0..dim {
                try!(writeln!(out, "pub const REG_{}{}: {} = 0x{:02x};",
                    r.name,
                    i + 1,
                    r_size,
                    r.offset + i,
                ));
                
            }
        }
    }

    try!(writeln!(out, ""));
    
    try!(writeln!(out, "pub struct Registers<'a, R: 'a + ReadWrite<E>, E> {{"));
    try!(writeln!(out, "  rw: &'a R,"));
    try!(writeln!(out, "  _phantomdata: PhantomData<E>,"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));




    try!(writeln!(out, "impl<'a, R: 'a + ReadWrite<E>, E> Registers<'a, R, E> {{"));
    try!(writeln!(out, "  pub fn new(rw: &'a R) -> Self {{"));
    try!(writeln!(out, "     Registers {{ rw: rw, _phantomdata: PhantomData }}"));
    try!(writeln!(out, "  }}"));
    try!(writeln!(out, ""));

    for r in p.registers.iter() {
        let reg_struct = to_camel(&r.name);
        let reg_getter = field_getter(&r.name);
        let reg_setter = field_setter(&r.name);
        let reg_with = field_with(&r.name);
        if let Some(dim) = r.dim {
            try!(writeln!(out, "  pub fn {}(&self, index: usize) -> Result<{}, E> {{", reg_getter, reg_struct));
            try!(writeln!(out, "    assert!(index < {});", dim));
            try!(writeln!(out, "    self.rw.read(REG_{} + index as {}).map({})", r.name, r_size, reg_struct));
            try!(writeln!(out, "  }}"));
            try!(writeln!(out, "  pub fn {}(&self, index: usize, value: {}) -> Result<(), E> {{", reg_setter, reg_struct));
            try!(writeln!(out, "    assert!(index < {});", dim));            
            try!(writeln!(out, "    self.rw.write(REG_{} + index as {}, value.0)", r.name, r_size));
            try!(writeln!(out, "  }}"));       
            try!(writeln!(out, "  pub fn {}<F: FnOnce({}) -> {}>(&self, index: usize, f: F) -> Result<(), E> {{", reg_with, reg_struct, reg_struct));
            try!(writeln!(out, "    assert!(index < {});", dim));
            try!(writeln!(out, "    let tmp = try!(self.rw.read(REG_{} + index as {}).map({}));", r.name, r_size, reg_struct));
            try!(writeln!(out, "    self.rw.write(REG_{} + index as {}, f(tmp).0)", r.name, r_size));
            try!(writeln!(out, "  }}"));          
            try!(writeln!(out, ""));            
        } else {
            try!(writeln!(out, "  pub fn {}(&self) -> Result<{}, E> {{", reg_getter, reg_struct));
            try!(writeln!(out, "    self.rw.read(REG_{}).map({})", r.name, reg_struct));
            try!(writeln!(out, "  }}"));
            try!(writeln!(out, "  pub fn {}(&self, value: {}) -> Result<(), E> {{", reg_setter, reg_struct));
            try!(writeln!(out, "    self.rw.write(REG_{}, value.0)", r.name));
            try!(writeln!(out, "  }}"));       
            try!(writeln!(out, "  pub fn {}<F: FnOnce({}) -> {}>(&self, f: F) -> Result<(), E> {{", reg_with, reg_struct, reg_struct));
            try!(writeln!(out, "    let tmp = try!(self.rw.read(REG_{}).map({}));", r.name, reg_struct));
            try!(writeln!(out, "    self.rw.write(REG_{}, f(tmp).0)", r.name));
            try!(writeln!(out, "  }}"));          
            try!(writeln!(out, ""));
        }
    }
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    for r in p.registers.iter() {
        try!(gen_register(out, r, r_size));
    }
    Ok(())
}

pub fn gen_register<W: Write>(out: &mut W, r: &Register, size: &'static str) -> Result<()> {
    let reg_struct = to_camel(&r.name);    

    try!(writeln!(out, "pub struct {}(pub {});", reg_struct, size));
    try!(writeln!(out, ""));
    try!(writeln!(out, "impl {} {{", reg_struct));

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
        try!(writeln!(out, "  pub fn {}(&self) -> {} {{", f_getter, size));
        try!(writeln!(out, "     ((self.0 as {}) >> {}) & 0x{:x} // {}", size, f_offset, f_mask, f_bits));
        try!(writeln!(out, "  }}"));    
        try!(writeln!(out, ""));

        try!(writeln!(out, "  pub fn {}(mut self, value: {}) -> Self {{", f_setter, size));
        try!(writeln!(out, "     assert!((value & !0x{:x}) == 0);", f_mask));
        try!(writeln!(out, "     self.0 &= !(0x{:x} << {});", f_mask, f_offset));
        try!(writeln!(out, "     self.0 |= value << {};", f_offset));
        try!(writeln!(out, "     self"));
        try!(writeln!(out, "  }}"));    
        try!(writeln!(out, ""));
    }
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    try!(writeln!(out, "impl ::core::fmt::Display for {} {{", reg_struct));
    try!(writeln!(out, "   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{"));
    try!(writeln!(out, "       self.0.fmt(f)"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, "}}"));        
    try!(writeln!(out, ""));


    try!(writeln!(out, "impl ::core::fmt::Debug for {} {{", reg_struct));
    try!(writeln!(out, "   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{"));
    match size {
        "u8" => try!(writeln!(out, "      try!(write!(f, \"[0x{{:02x}}\", self.0));")),
        "u16" => try!(writeln!(out, "      try!(write!(f, \"[0x{{:04x}}\", self.0));")),
        "u32" => try!(writeln!(out, "      try!(write!(f, \"[0x{{:08x}}\", self.0));")),
        s @ _ => panic!("invalid size: {}", s),
    }        
    
    for f in r.fields.iter() {
        let f_name = field_name(&f.name);
        let f_getter = field_getter(&f.name);

        if let Some(dim) = f.dim {
            for i in 0..dim {
                match f.bit_width {
                    1 => {
                        try!(writeln!(out, "      if self.{}({}) != 0 {{ try!(write!(f, \" {}[{}]\"))}}", f_getter, i, f_getter, i));
                    },
                    32 => {},
                    _ => {
                        try!(writeln!(out, "      if self.{}({}) != 0 {{ try!(write!(f, \" {}[{}]=0x{{:x}}\", self.{}({})))}}", f_getter, i, f_name, i, f_getter, i));
                    }
                }                    
            }
        } else {
            match f.bit_width {
                1 => {
                    try!(writeln!(out, "      if self.{}() != 0 {{ try!(write!(f, \" {}\"))}}", f_getter, f_getter));
                },
                32 => {},
                _ => {
                    try!(writeln!(out, "      if self.{}() != 0 {{ try!(write!(f, \" {}=0x{{:x}}\", self.{}()))}}", f_getter, f_name, f_getter));
                }
            }
        }
        
    }
    try!(writeln!(out, "      try!(write!(f, \"]\"));"));
    try!(writeln!(out, "      Ok(())"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, "}}"));        
    try!(writeln!(out, ""));    

    Ok(())
}
