use std::io::{Write, Result};
use std::path::{Path, PathBuf};
use clap::{ArgMatches};
use std::fs::File;

use {Access, Device, PeripheralGroup, Peripheral, Register, Cluster, Field, Interrupt, Exception, Pin};

use super::{size_type, field_getter, field_setter, field_with, field_name, to_camel};

pub struct Config {
    pub path: PathBuf,
    pub is_root: bool,
}

impl<'a> From<&'a ArgMatches<'a>> for Config {
    fn from(matches: &'a ArgMatches) -> Config {
        Config {
            path: PathBuf::from(matches.value_of("output").expect("No output path specified")),
            is_root: matches.is_present("root")
        }
    }
}

pub fn gen_modules<W: Write>(matches: &ArgMatches, out: &mut W, d: &Device) -> Result<()> {
    let cfg = Config::from(matches);
    let out_path = &cfg.path;
    let p_mod = if cfg.is_root {
        out_path.join("lib.rs")
    } else {
        out_path.join("mod.rs")
    };
    let mut f_mod = try!(File::create(p_mod));
    try!(gen_mod(&cfg, &mut f_mod, d, out_path));

    Ok(())
}

pub fn gen_mod<W: Write>(cfg: &Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {

    // Only add module import if not generating cortex-core

    let p_mod = if cfg.is_root { 
        try!(writeln!(out, "#![no_std]"));
    };

    // Generate Imports

    for c in d.crates.iter() {
        try!(writeln!(out, "extern crate {};", c.name));
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
        try!(gen_exceptions(cfg, &mut f_mod, &d.exceptions));
    }

    // Generate Interrupts
    if let Some(interrupt_count) = d.interrupt_count {    
        let p_name = "irq";
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(gen_interrupts(cfg, &mut f_mod, &d, interrupt_count));
    }

    // // Generate Pins
    // {
    //     let p_name = "pins";
    //     try!(writeln!(out, "pub mod {};", p_name));
    //     let p_mod = path.join(format!("{}.rs", p_name));
    //     let mut f_mod = try!(File::create(p_mod));
    //     try!(gen_pins(cfg, &mut f_mod, &d));
    // }

    for p in d.peripherals.iter() {
        let p_name = p.group_name.as_ref().expect("Expected group name").to_lowercase();
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(gen_peripheral(cfg, &mut f_mod, p));
    }

    for pg in d.peripheral_groups.iter() {
        let pg_name = pg.name.to_lowercase();
        try!(writeln!(out, "pub mod {};", pg_name));
        let p_mod = path.join(format!("{}.rs", pg_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(gen_peripheral_group(cfg, &mut f_mod, pg));
    }
    Ok(())
}

pub fn gen_exceptions<W: Write>(cfg: &Config, out: &mut W, exceptions: &Vec<Exception>) -> Result<()> {
    try!(writeln!(out, "pub type Handler = unsafe extern \"C\" fn();"));
    try!(writeln!(out, ""));

    for (i, e) in exceptions.iter().enumerate() {
        if e.name != "" {
            try!(write!(out, "{:40} // {}\n", format!("pub const EXC_{}: Exception = Exception({});", e.name.to_uppercase(), i), e.description.as_ref().unwrap()));
        }
    }
    try!(writeln!(out, ""));
    
    try!(writeln!(out, "pub fn set_handler(exc: Exception, handler: Option<Handler>) {{"));
    try!(writeln!(out, "  unsafe {{ R_EXCEPTION_HANDLERS[exc.0] = handler }};"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));
    try!(writeln!(out, "pub struct Exception(pub usize);"));
    try!(writeln!(out, ""));
    
    try!(writeln!(out, "impl Exception {{"));
    try!(writeln!(out, "   pub fn set_handler(&self, handler: Option<Handler>) {{"));
    try!(writeln!(out, "      unsafe {{ R_EXCEPTION_HANDLERS[self.0] = handler }};"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    try!(writeln!(out,"#[link_section = \".vector.exceptions\"]"));
    try!(writeln!(out,"#[no_mangle]"));
    try!(writeln!(out,"pub static EXCEPTION_HANDLERS: [Option<Handler>; {}] = [", exceptions.len()));
    for e in exceptions.iter() {
        if e.name != "" {
            try!(writeln!(out, "   {:30} // {}", format!("Some(_{}),", e.name), e.description.as_ref().unwrap()));
        } else {
            try!(writeln!(out, "   None,"));
        }        
    }
    try!(writeln!(out,"];"));
    try!(writeln!(out,""));
   
    

    try!(writeln!(out,"#[link_section = \".bss.r_exceptions\"]"));
    try!(writeln!(out,"#[no_mangle]"));
    try!(writeln!(out,"pub static mut R_EXCEPTION_HANDLERS: [Option<Handler>; {}] = [None; {}];", exceptions.len(), exceptions.len()));
    try!(writeln!(out,""));    

    try!(writeln!(out, "extern \"C\" {{"));
    for e in exceptions.iter() {
        if e.name != "" {
            try!(writeln!(out, "   {:30} // {}", format!("pub fn _{}();", e.name), e.description.as_ref().unwrap()));
        }      
    }
    try!(writeln!(out,"}}"));    
    Ok(())
}
pub fn gen_interrupts<W: Write>(cfg: &Config, out: &mut W, d: &Device, interrupt_count: u64) -> Result<()> {
    let mut interrupts: Vec<Option<&Interrupt>> = Vec::with_capacity(interrupt_count as usize);

    for _ in 0..interrupt_count {
        interrupts.push(None);
    }

    for p in d.peripherals.iter() {
        for i in p.interrupts.iter() {
            interrupts[i.value as usize] = Some(i)
        }
    }

    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            for i in p.interrupts.iter() {
                interrupts[i.value as usize] = Some(i)
            }
        }
    }

    try!(writeln!(out, "pub type Handler = unsafe extern \"C\" fn();"));
    try!(writeln!(out, ""));

    for irq in interrupts.iter() {
        if let &Some(irq) = irq { 
            let desc = irq.description.as_ref().map(|s| s.as_ref()).unwrap_or("No Description");
            try!(writeln!(out, "{:40} // IRQ {}: {}", format!("pub const IRQ_{}: Irq = Irq({});", irq.name.to_uppercase(), irq.value), irq.value, desc));
        }
    }
    try!(writeln!(out, ""));

    try!(writeln!(out, "pub fn set_handler(irq: Irq, handler: Option<Handler>) {{"));
    try!(writeln!(out, "  unsafe {{ R_INTERRUPT_HANDLERS[irq.0] = handler }};"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));    

    try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));
    try!(writeln!(out, "pub struct Irq(pub usize);"));
    try!(writeln!(out, ""));

    try!(writeln!(out, "impl Irq {{"));
    try!(writeln!(out, "   pub fn set_handler(&self, handler: Option<Handler>) {{"));
    try!(writeln!(out, "      unsafe {{ R_INTERRUPT_HANDLERS[self.0] = handler }};"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));    

    try!(writeln!(out,"#[link_section = \".vector.interrupts\"]"));
    try!(writeln!(out,"#[no_mangle]"));
    try!(writeln!(out,"pub static mut INTERRUPT_HANDLERS: [Option<Handler>; {}] = [", interrupts.len()));

    for irq in interrupts.iter() {
        if let &Some(irq) = irq { 
            let desc = irq.description.as_ref().map(|s| s.as_ref()).unwrap_or("No Description");
            // try!(writeln!(out, "   {:30} // {}", format!("Some(_{}),", irq.name.to_lowercase()), desc));
            try!(writeln!(out, "   {:30} // IRQ {}: {}", format!("None,"), irq.value, desc));
        } else {
            try!(writeln!(out, "   None,"));
        }
    }

    try!(writeln!(out,"];"));
    try!(writeln!(out,""));
    
    try!(writeln!(out,"#[link_section = \".bss.r_interrupts\"]"));
    try!(writeln!(out,"#[no_mangle]"));
    try!(writeln!(out,"pub static mut R_INTERRUPT_HANDLERS: [Option<Handler>; {}] = [None; {}];",
         interrupts.len(), interrupts.len()));
    try!(writeln!(out,""));    

    // try!(writeln!(out, "extern \"C\" {{"));
    // for irq in interrupts.iter() {
    //     if let &Some(irq) = irq {
    //         let desc = irq.description.as_ref().map(|s| s.as_ref()).unwrap_or("No Description");
    //         try!(writeln!(out, "   {:30} // {}", format!("pub fn _{}();", irq.name.to_lowercase()), desc));
    //     }      
    // }
    // try!(writeln!(out,"}}"));    
    Ok(())
}

pub fn gen_pins<W: Write>(cfg: &Config, out: &mut W, p_type: &str, p_name: &str, pins: &[Pin]) -> Result<()> {

    for pin in pins.iter() {
        try!(writeln!(out, "pub const {}: Pin = ({}, {});", 
            pin.name, p_name, pin.index.unwrap()));

        for af in pin.altfns.iter() {
            let name = format!("{}_{}", pin.name, af.signal);
            try!(writeln!(out, "pub const {}: PinFn = ({}, {}, {});", 
                name, p_name, pin.index.unwrap(), af.index));
        }
        try!(writeln!(out,""));
    }

    Ok(())
}


pub fn gen_peripheral_group<W: Write>(cfg: &Config, out: &mut W, pg: &PeripheralGroup) -> Result<()> {
    if pg.modules.len() > 0 {
        for m in pg.modules.iter() {
            if let Some(ref use_as) = m._as {
                try!(writeln!(out, "pub use {} as {};", m.name, use_as));
            } else {
                try!(writeln!(out, "pub use {};", m.name));
            }
        }
        try!(writeln!(out, ""));
    }

    let mut count: usize = 0;

    let p_type = to_camel(&pg.name);

    for p in pg.peripherals.iter() {
        let p_name = p.name.to_uppercase();
        try!(write!(out, "pub const {}: {} = {}(0x{:08x});\n", p_name, p_type, p_type, p.address));            
        count += 1;
    }    
    try!(write!(out, "\n"));

    try!(writeln!(out, "pub const {}: [{}; {}] = [", pg.name.to_uppercase(), p_type, count));
    for p in pg.peripherals.iter() {
        let p_name = p.name.to_uppercase();
        try!(writeln!(out, "   {},", p_name));
        count += 1;
    }    
    try!(writeln!(out, "];"));
    try!(writeln!(out, ""));

    if pg.modules.len() == 0 {
        try!(write!(out, "#[derive(Clone, Copy, PartialEq, Eq)]\n"));
        try!(write!(out, "pub struct {}(pub u32);\n\n", p_type));    
    }
    let p0 = if let Some(ref p0) = pg.prototype {
        p0
    } else {
        &pg.peripherals[0]
    };

    if p0.registers.len() > 0 {
        try!(gen_registers(cfg, out, &p_type, &p0.registers[..], p0.size.or(Some(32)), p0.access.or(Some(Access::ReadWrite))));
    }
    if p0.clusters.len() > 0 {
        try!(gen_clusters(cfg, out, &p_type, &p0.clusters[..], p0.size.or(Some(32)), p0.access.or(Some(Access::ReadWrite))));
    }

    if p0.pins.len() > 0 {
        let mut pin_count: usize = 0;

        try!(writeln!(out, "pub type Pin = ({}, usize);", p_type));
        try!(writeln!(out, "pub type PinFn = ({}, usize, usize);", p_type));
        try!(writeln!(out, ""));
        
        for p in pg.peripherals.iter() {
            let p_name = p.name.to_uppercase();

            if p.pins.len() > 0 {
                try!(gen_pins(cfg, out, &p_type, &p_name, &p.pins[..]));
                pin_count += p.pins.len();
            }
        }    

        try!(writeln!(out, "pub const PIN: [(&'static str, Pin); {}] = [", pin_count));

        for p in pg.peripherals.iter() {
            for pin in p.pins.iter() {
                try!(writeln!(out,"  (\"{}\", {}),", pin.name, pin.name));          
            }
        }
        try!(writeln!(out, "];"));
        try!(writeln!(out,""));          
    }


    Ok(())
}

pub fn gen_peripheral<W: Write>(cfg: &Config, out: &mut W, p: &Peripheral) -> Result<()> {
    let p_type = to_camel(&p.group_name.as_ref().unwrap());

    if let Some(dim) = p.dim {
        for (offset, name) in p.iter_dim() {
            let p_name = name.replace("[","").replace("]","");            
            try!(write!(out, "pub const {}: {} = {}(0x{:08x});\n", p_name, p_type, p_type, offset));    
        }
        try!(write!(out, "\n"));

        // let p_arr = p.group_name.as_ref().unwrap().to_uppercase();
        // try!(write!(out, "pub const {}: [{}; {}] = [\n", p_arr, p_type, dim));
        // for (offset, name) in p.iter_dim() {
        //     let p_name = name.replace("[","").replace("]","");            
        //     try!(write!(out, "   {},\n", p_name));
        // }
        // try!(write!(out, "];\n\n"));

    } else {
        try!(write!(out, "pub const {}: {} = {}(0x{:08x});\n", p.name, p_type, p_type, p.address));    
    }
    try!(write!(out, "\n"));

    try!(write!(out, "#[derive(Clone, Copy, PartialEq, Eq)]\n"));
    try!(write!(out, "pub struct {}(pub u32);\n\n", p_type));    

    if p.registers.len() > 0 {
        try!(gen_registers(cfg, out, &p_type, &p.registers[..], p.size.or(Some(32)), p.access.or(Some(Access::ReadWrite))));
    }

    if p.clusters.len() > 0 {
        try!(gen_clusters(cfg, out, &p_type, &p.clusters[..], p.size.or(Some(32)), p.access.or(Some(Access::ReadWrite))));
    }

    Ok(())
}

pub fn gen_peripheral_enum<W: Write>(cfg: &Config, out: &mut W, p: &Peripheral) -> Result<()> {
    let p_name = to_camel(&p.name);
    try!(write!(out, "pub enum {} {{\n", p_name));
    try!(write!(out, "  {} = 0x{:08x}, \n", p_name, p.address));
    try!(write!(out, "}}\n"));
    try!(write!(out, "\n"));

    try!(gen_registers(cfg, out, &p_name, &p.registers[..], p.size.or(Some(32)), p.access.or(Some(Access::ReadWrite))));

    Ok(())
}

pub fn gen_clusters<W: Write>(cfg: &Config, out: &mut W, p_type: &str, clusters: &[Cluster], size: Option<u64>, access: Option<Access>) -> Result<()> {
    try!(write!(out, "impl {} {{\n", p_type));

    for c in clusters.iter() {
        let c_type = format!("{}", to_camel(&c.name));
        let mod_name = c.name.to_lowercase();
        try!(write!(out, "   pub fn {}(&self) -> {}::{} {{\n", mod_name, mod_name, c_type));
        try!(write!(out, "      {}::{}(self.0 + 0x{:x})\n", mod_name, c_type, c.offset));
        try!(write!(out, "   }}\n\n"));
    }
    try!(write!(out, "}}\n\n"));

    for c in clusters.iter() {        
        let c_type = format!("{}", to_camel(&c.name));
        let mod_name = c.name.to_lowercase();        
        try!(write!(out, "pub mod {} {{\n", mod_name));
        try!(write!(out, "   #[derive(Clone, Copy, PartialEq, Eq)]\n"));
        try!(write!(out, "   pub struct {}(pub u32);\n\n", c_type));
        try!(gen_registers(cfg, out, &c_type, &c.registers[..], c.size.or(size), c.access.or(access)));
        try!(write!(out, "}}\n"));
        try!(write!(out, "// End of {}\n\n", mod_name));
    }

    Ok(())
}

pub fn gen_registers<W: Write>(cfg: &Config, out: &mut W, p_type: &str, regs: &[Register], size: Option<u64>, access: Option<Access>) -> Result<()> {
    try!(write!(out, "impl {} {{\n", p_type));
    
    for r in regs.iter() {  
        let r_type = format!("{}", to_camel(&r.name));
        let r_getter = field_getter(&r.name);
        let r_setter = field_setter(&r.name);
        let r_with = field_with(&r.name);
        let r_size = size_type(r.size.or(size).unwrap());
        let r_access = r.access.or(access).unwrap();
        let r_offset = r.offset;        

        let r_typevar = if r.name.to_lowercase() == "f" {
            "_F"
        } else {
            "F"
        };

        if let Some(dim) = r.dim {
            let r_incr = r.dim_increment.unwrap();
            let r_shift = match r_incr {
                1 => format!("(index)"),
                2 => format!("(index << 1)"),
                4 => format!("(index << 2)"),
                8 => format!("(index << 3)"),
                16 => format!("(index << 4)"),
                _ => format!("(index * {})", r_incr),
            };        
            if r_access.is_readable() {
                try!(write!(out, "  pub unsafe fn {}(&self, index: usize) -> {} {{ \n", r_getter, r_type));
                try!(write!(out, "     assert!(index < {});\n", dim));
                try!(write!(out, "     {}(::core::ptr::read_volatile(((self.0 as usize) + 0x{:x} + {}) as *const {}))\n", r_type, r_offset, r_shift, r_size));
                try!(write!(out, "  }}\n"));
            }
            if r_access.is_writable() {
                try!(write!(out, "  pub unsafe fn {}(&mut self, index: usize, value: {}) {{\n", r_setter, r_type));
                try!(write!(out, "     assert!(index < {});\n", dim));
                try!(write!(out, "     ::core::ptr::write_volatile(((self.0 as usize) + 0x{:x} + {}) as *mut {}, value.0);\n", r_offset, r_shift, r_size)); 
                try!(write!(out, "  }}\n"));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(write!(out, "  pub unsafe fn {}<{}: FnOnce({}) -> {}>(&mut self, index: usize, f: {}) {{\n", r_with, r_typevar, r_type, r_type, r_typevar));
                try!(write!(out, "     let tmp = self.{}(index);\n", r_getter));
                try!(write!(out, "     self.{}(index, f(tmp))\n", r_setter));
                try!(write!(out, "  }}\n"));            
            }            
        } else {
            if r_access.is_readable() {
                try!(write!(out, "  pub unsafe fn {}(&self) -> {} {{ \n", r_getter, r_type));
                try!(write!(out, "     {}(::core::ptr::read_volatile(((self.0 as usize) + 0x{:x}) as *const {}))\n", r_type, r_offset, r_size));
                try!(write!(out, "  }}\n"));
            }
            if r_access.is_writable() {
                try!(write!(out, "  pub unsafe fn {}(&mut self, value: {}) {{\n", r_setter, r_type));
                try!(write!(out, "     ::core::ptr::write_volatile(((self.0 as usize) + 0x{:x}) as *mut {}, value.0);\n", r_offset, r_size));                    
                try!(write!(out, "  }}\n"));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(write!(out, "  pub unsafe fn {}<{}: FnOnce({}) -> {}>(&mut self, f: {}) {{\n", r_with, r_typevar, r_type, r_type, r_typevar));
                try!(write!(out, "     let tmp = self.{}();\n", r_getter));
                try!(write!(out, "     self.{}(f(tmp))\n", r_setter));
                try!(write!(out, "  }}\n"));            
            }
        }
        try!(write!(out, "\n"));
    }    
    try!(write!(out, "}}\n"));
    try!(write!(out, "\n"));

    for r in regs.iter() {  
        let r_type = format!("{}", to_camel(&r.name));
        let r_size = size_type(r.size.or(size).unwrap());
        try!(write!(out, "#[derive(PartialEq, Eq)]\n"));
        try!(write!(out, "pub struct {}(pub {});\n\n", r_type, r_size));
        try!(write!(out, "impl {} {{\n", r_type));        
        for f in r.fields.iter() {
            try!(gen_field(cfg, out, &f, &r_size, f.access.or(access)))
        }
        try!(write!(out, "}}\n\n"));

        try!(write!(out, "impl ::core::fmt::Display for {} {{\n", r_type));
        try!(write!(out, "   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{\n"));
        try!(write!(out, "       self.0.fmt(f)\n"));
        try!(write!(out, "   }}\n"));
        try!(write!(out, "}}\n\n"));        

        try!(write!(out, "impl ::core::fmt::Debug for {} {{\n", r_type));
        try!(write!(out, "   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{\n"));        
        try!(write!(out, "      try!(write!(f, \"[0x{{:08x}}\", self.0));\n"));
        for f in r.fields.iter() {
            let f_name = field_name(&f.name);
            let f_getter = field_getter(&f.name);

            if let Some(dim) = f.dim {
                for i in 0..dim {
                    match f.bit_width {
                        1 => {
                            try!(write!(out, "      if self.{}({}) != 0 {{ try!(write!(f, \" {}[{}]\"))}}\n", f_getter, i, f_getter, i));
                        },
                        32 => {},
                        _ => {
                            try!(write!(out, "      if self.{}({}) != 0 {{ try!(write!(f, \" {}[{}]=0x{{:x}}\", self.{}({})))}}\n", f_getter, i, f_name, i, f_getter, i));
                        }
                    }                    
                }
            } else {
                match f.bit_width {
                    1 => {
                        try!(write!(out, "      if self.{}() != 0 {{ try!(write!(f, \" {}\"))}}\n", f_getter, f_getter));
                    },
                    32 => {},
                    _ => {
                        try!(write!(out, "      if self.{}() != 0 {{ try!(write!(f, \" {}=0x{{:x}}\", self.{}()))}}\n", f_getter, f_name, f_getter));
                    }
                }
            }
            
        }
        try!(write!(out, "      try!(write!(f, \"]\"));\n"));
        try!(write!(out, "      Ok(())\n"));
        try!(write!(out, "   }}\n"));
        try!(write!(out, "}}\n\n"));        
    }
    Ok(())
}

pub fn gen_field<W: Write>(cfg: &Config, out: &mut W, f: &Field, size: &str, access: Option<Access>) -> Result<()> {
    let f_access = f.access.or(access).unwrap();
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
    let f_shift_mask = f_mask << f_offset;   
    let f_bits = if f_width > 1 {
        format!("[{}:{}]", f_hi, f_lo)
    } else {
        format!("[{}]", f_lo)
    };    

    if let Some(dim) = f.dim {
        let f_incr = f.dim_increment.unwrap();
        let f_getter = field_getter(&f.name.replace("%s","x"));
        let f_setter = field_setter(&f.name.replace("%s","x"));

        try!(write!(out, "  pub fn {}(&self, index: usize) -> {} {{\n", f_getter, size));
        try!(write!(out, "     assert!(index < {});\n", dim));
        match f_incr {
            1 => {
                try!(write!(out, "     let shift: usize = {} + index;\n", f_offset));
            },
            2 => {
                try!(write!(out, "     let shift: usize = {} + (index << 1);\n", f_offset));
            },
            4 => {
                try!(write!(out, "     let shift: usize = {} + (index << 2);\n", f_offset));
            },
            8 => {
                try!(write!(out, "     let shift: usize = {} + (index << 3);\n", f_offset));
            },
            _ => {
                try!(write!(out, "     let shift: usize = {} + (index * {});\n", f_offset, f_incr));                
            }
        }
        try!(write!(out, "     ((self.0 as {}) >> shift) & 0x{:x} // {}\n", size, f_mask, f_bits));
        try!(write!(out, "  }}\n"));    

        try!(write!(out, "  pub fn {}(mut self, index: usize, value: {}) -> Self {{\n", f_setter, size));
        try!(write!(out, "     assert!(index < {});\n", dim));
        try!(write!(out, "     assert!((value & !0x{:x}) == 0);\n", f_mask));
        match f_incr {
            1 => {
                try!(write!(out, "     let shift: usize = {} + index;\n", f_offset));
            },
            2 => {
                try!(write!(out, "     let shift: usize = {} + (index << 1);\n", f_offset));
            },
            4 => {
                try!(write!(out, "     let shift: usize = {} + (index << 2);\n", f_offset));
            },
            8 => {
                try!(write!(out, "     let shift: usize = {} + (index << 3);\n", f_offset));
            },
            _ => {
                try!(write!(out, "     let shift: usize = {} + (index * {});\n", f_offset, f_incr));                
            }
        }        
        try!(write!(out, "     self.0 &= !(0x{:x} << shift);\n", f_mask));
        try!(write!(out, "     self.0 |= value << shift;\n"));
        try!(write!(out, "     self\n"));
        try!(write!(out, "  }}\n"));    
        try!(write!(out, "\n"));
    } else {

        try!(write!(out, "  pub fn {}(&self) -> {} {{\n", f_getter, size));
        try!(write!(out, "     ((self.0 as {}) >> {}) & 0x{:x} // {}\n", size, f_offset, f_mask, f_bits));
        try!(write!(out, "  }}\n"));    

        try!(write!(out, "  pub fn {}(mut self, value: {}) -> Self {{\n", f_setter, size));
        try!(write!(out, "     assert!((value & !0x{:x}) == 0);\n", f_mask));
        try!(write!(out, "     self.0 &= !(0x{:x} << {});\n", f_mask, f_offset));
        try!(write!(out, "     self.0 |= value << {};\n", f_offset));
        try!(write!(out, "     self\n"));
        try!(write!(out, "  }}\n"));    
        try!(write!(out, "\n"));
            
    }
    Ok(())
}

#[cfg(xtest)]
mod tests {
    extern crate core;

    #[derive(Debug, Clone, Copy)]
    pub enum Gpio {
        GpioA = 0x10000000,
        GpioB = 0x10000100,
    }

    impl Gpio {
        pub unsafe fn moder(&self) -> ModeR {
            ModeR(core::ptr::read_volatile(((*self as usize) + 0) as *const u32))
        }
    }

    pub struct ModeR(pub u32);

    impl ModeR {
        pub fn moder15(&self) -> u32 {
            (self.0 >> 30) & 0b11
        }

        pub fn with_moder15<F: FnOnce(u32) -> u32>(&mut self, f: F) {
            let tmp = self.moder15();
            self.set_moder15(f(tmp));
        }

        pub fn set_moder15(&mut self, value: u32) {
            assert!((value & !0b11) == 0);
            self.0 &= !(0b11 << 30);
            self.0 |= value << 30;
        }
    }
}