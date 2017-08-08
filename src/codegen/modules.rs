use std::io::{Write, Result};
use std::path::{Path, PathBuf};
use clap::{ArgMatches};
use std::fs::File;
use std::collections::HashSet;

use {Access, Device, PeripheralGroup, Peripheral, Descriptor, Register, Cluster, Field, Interrupt, Exception, Pin};

use super::{size_type, field_getter, field_setter, field_with, field_ptr, field_mut, field_name, to_camel};

pub struct Config {
    pub path: PathBuf,
    pub is_root: bool,
    pub bit_types: bool,
}

impl<'a> From<&'a ArgMatches<'a>> for Config {
    fn from(matches: &'a ArgMatches) -> Config {
        Config {
            path: PathBuf::from(matches.value_of("output").expect("No output path specified")),
            is_root: matches.is_present("root"),
            bit_types: matches.is_present("bit-types"),
        }
    }
}

fn gen_doc<W: Write>(cfg: &Config, out: &mut W, doc: &str) -> Result<()> {
    let doc = doc.trim();
    if doc.len() > 0 {
        try!(writeln!(out, "#[doc=\"{}\"]", doc))
    }
    Ok(())
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

    if cfg.bit_types {
        try!(writeln!(out, "#[allow(unused_imports)] use bobbin_common::bits;"));
    }

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

    // Generate Signals

    {    
        let p_name = "sig";
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(gen_signals(cfg, &mut f_mod, &d));
    }


    for p in d.peripherals.iter() {
        let p_name = p.group_name.as_ref().unwrap_or(&p.name).to_lowercase();
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
    try!(writeln!(out, "//! Exceptions"));
    try!(writeln!(out, ""));
    try!(writeln!(out, "pub type Handler = unsafe extern \"C\" fn();"));
    try!(writeln!(out, ""));

    for (i, e) in exceptions.iter().enumerate() {
        if e.name != "" {
            try!(writeln!(out, "{:40} // {}", format!("pub const EXC_{}: Exception = Exception({});", e.name.to_uppercase(), i), e.description.as_ref().unwrap()));
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

    try!(writeln!(out, "//! Interrupts"));
    try!(writeln!(out, ""));

    for _ in 0..interrupt_count {
        interrupts.push(None);
    }

    for p in d.peripherals.iter() {
        for i in p.interrupts.iter() {
            interrupts[i.value as usize] = Some(i);
        }
    }

    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            for i in p.interrupts.iter() {
                interrupts[i.value as usize] = Some(i);
            }
        }
    }

    try!(writeln!(out, "use ::core::marker::PhantomData;"));
    try!(writeln!(out, "pub type Handler = extern \"C\" fn();"));
    try!(writeln!(out, ""));


    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            for irq in p.interrupts.iter() {
                let irq_type = format!("Irq{}", to_camel(&irq.name));
                let irq_id = format!("{}Id", to_camel(&irq.name));

                let desc = irq.description.as_ref().map(|s| s.as_ref()).unwrap_or("No Description");
                try!(writeln!(out, "pub const IRQ_{}: {} = Irq({}, {} {{}});", 
                    irq.name.to_uppercase(), 
                    irq_type,
                    irq.value,
                    irq_id,                
                ));
            }
            for ch in p.channels.iter() {
                for irq in ch.interrupts.iter() {
                    let irq_type = format!("Irq{}", to_camel(&irq.name));
                    let irq_id = format!("{}Id", to_camel(&irq.name));

                    let desc = irq.description.as_ref().map(|s| s.as_ref()).unwrap_or("No Description");
                    try!(writeln!(out, "pub const IRQ_{}: {} = Irq({}, {} {{}});", 
                        irq.name.to_uppercase(), 
                        irq_type,
                        irq.value,
                        irq_id,                
                    ));
                }                
            }

        }
    }
    try!(writeln!(out, ""));

    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            for irq in p.interrupts.iter() {
                let irq_type = format!("Irq{}", to_camel(&irq.name));
                let irq_id = format!("{}Id", to_camel(&irq.name));
                try!(writeln!(out, "pub type {} = Irq<{}>;", irq_type, irq_id));
            }
            for ch in p.channels.iter() {
                for irq in ch.interrupts.iter() {
                    let irq_type = format!("Irq{}", to_camel(&irq.name));
                    let irq_id = format!("{}Id", to_camel(&irq.name));
                    try!(writeln!(out, "pub type {} = Irq<{}>;", irq_type, irq_id));
                }                
            }
        }
    }
    
    try!(writeln!(out, ""));    

    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            for irq in p.interrupts.iter() {
                let irq_id = format!("{}Id", to_camel(&irq.name));
                try!(writeln!(out, "#[doc(hidden)]"));
                try!(writeln!(out, "pub struct {} {{}} // IRQ {}", irq_id, irq.value));
            }
            for ch in p.channels.iter() {
                for irq in ch.interrupts.iter() {
                    let irq_id = format!("{}Id", to_camel(&irq.name));
                    try!(writeln!(out, "#[doc(hidden)]"));
                    try!(writeln!(out, "pub struct {} {{}} // IRQ {}", irq_id, irq.value));
                }                
            }
        }
    }
    
    try!(writeln!(out, ""));    
    
    // TODO: Assert that NVIC is disabled before setting handler to None
    try!(writeln!(out, "pub fn set_handler(index: usize, handler: Option<Handler>) {{"));
    try!(writeln!(out, "  unsafe {{ "));
    try!(writeln!(out, "     assert!(R_INTERRUPT_HANDLERS[index].is_some() != handler.is_some());"));
    try!(writeln!(out, "     R_INTERRUPT_HANDLERS[index] = handler"));
    try!(writeln!(out, "  }};"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));    

    // try!(writeln!(out, "use super::nvic::{{NVIC, Iser, Icer, Ispr, Icpr, Stir}};"));
    try!(writeln!(out, "use super::nvic::NVIC;"));

    try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));
    try!(writeln!(out, "pub struct Irq<T>(usize, T);"));
    try!(writeln!(out, "impl<T> Irq<T> {{"));
    try!(writeln!(out, "   pub fn index(&self) -> usize {{ self.0 }}"));
    try!(writeln!(out, ""));    
    try!(writeln!(out, "   pub fn is_enabled(&self) -> bool {{ NVIC.iser((self.0 >> 5)).setena((self.0 & 0b11111)) != 0 }}"));
    try!(writeln!(out, ""));
    try!(writeln!(out, "   pub fn set_enabled(&self, value: bool) {{"));
    try!(writeln!(out, "      if value {{"));
    try!(writeln!(out, "         assert!(self.handler().is_some());"));
    try!(writeln!(out, "         NVIC.set_iser((self.0 >> 5), |r| r.set_setena((self.0 & 0b11111), 1));"));
    try!(writeln!(out, "      }} else {{"));
    try!(writeln!(out, "         NVIC.set_icer((self.0 >> 5), |r| r.set_clrena((self.0 & 0b11111), 1));"));
    try!(writeln!(out, "      }}"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, ""));
    try!(writeln!(out, "   pub fn is_pending(&self) -> bool {{"));
    try!(writeln!(out, "       NVIC.ispr((self.0 >> 5)).setpend((self.0 & 0b11111)) != 0"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, ""));
    try!(writeln!(out, "   pub fn set_pending(&self, value: bool) {{"));
    try!(writeln!(out, "       if value {{"));
    try!(writeln!(out, "           NVIC.set_ispr((self.0 >> 5), |r| r.set_setpend((self.0 & 0b11111), 1));"));
    try!(writeln!(out, "       }} else {{"));
    try!(writeln!(out, "           NVIC.set_icpr((self.0 >> 5), |r| r.set_clrpend((self.0 & 0b11111), 1));"));
    try!(writeln!(out, "       }}"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, ""));
    try!(writeln!(out, "   pub fn is_active(&self) -> bool {{"));
    try!(writeln!(out, "       NVIC.iabr((self.0 >> 5)).active(self.0 & 0b11111) != 0"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, ""));
    try!(writeln!(out, "   pub fn priority(&self) -> u8 {{"));
    try!(writeln!(out, "       NVIC.ipr((self.0 >> 4)).pri(self.0 & 0b1111).into()"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, ""));
    try!(writeln!(out, "   pub fn set_priority(&self, value: u8) {{"));
    try!(writeln!(out, "       NVIC.with_ipr((self.0 >> 4), |r| r.set_pri(self.0 & 0b1111, value));"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, ""));
    try!(writeln!(out, "   pub fn trigger_interrupt(&self) {{"));
    try!(writeln!(out, "       NVIC.set_stir(|r| r.set_intid(self.0));"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, ""));

    try!(writeln!(out, "   pub fn handler(&self) -> Option<Handler> {{ unsafe {{ R_INTERRUPT_HANDLERS[self.0] }} }}"));
    try!(writeln!(out, ""));

    // TODO: Assert that NVIC is disabled before setting handler to None
    try!(writeln!(out, "   pub fn set_handler(&self, handler: Option<Handler>) {{"));
    try!(writeln!(out, "      unsafe {{ assert!(R_INTERRUPT_HANDLERS[self.0].is_some() != handler.is_some()); }};"));
    try!(writeln!(out, "      unsafe {{ R_INTERRUPT_HANDLERS[self.0] = handler }};"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    // try!(writeln!(out, "impl Irq {{"));
    // try!(writeln!(out, "   pub fn set_handler(&self, handler: Option<Handler>) {{"));
    // try!(writeln!(out, "      unsafe {{ R_INTERRUPT_HANDLERS[self.0] = handler }};"));
    // try!(writeln!(out, "   }}"));
    // try!(writeln!(out, "}}"));
    // try!(writeln!(out, ""));    

    try!(writeln!(out, "pub struct IrqHandle {{}}"));
    try!(writeln!(out, "pub struct IrqGuard<'a>(usize, PhantomData<&'a IrqHandle>);"));
    try!(writeln!(out, "impl<'a> IrqGuard<'a> {{"));
    try!(writeln!(out, "   pub fn new(index: usize) -> Self {{"));
    try!(writeln!(out, "      IrqGuard(index, PhantomData)"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, "impl<'a> Drop for IrqGuard<'a> {{"));
    try!(writeln!(out, "   fn drop(&mut self) {{"));
    try!(writeln!(out, "      set_handler(self.0, None)"));
    try!(writeln!(out, "   }}"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    try!(writeln!(out, ""));
    try!(writeln!(out, "pub trait RegisterHandler {{"));
    try!(writeln!(out, "   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a>;"));;
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    try!(writeln!(out, "pub trait HandleInterrupt {{"));
    try!(writeln!(out, "   fn handle_interrupt(&self);"));;
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            for irq in p.interrupts.iter() {
                let irq_const = irq.name.to_uppercase();
                let irq_id = format!("Irq{}", to_camel(&irq.name));
                try!(writeln!(out, "impl RegisterHandler for {} {{", irq_id));
                try!(writeln!(out, "   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {{"));;          
                try!(writeln!(out, "       static mut HANDLER: Option<usize> = None;"));
                try!(writeln!(out, "       unsafe {{ HANDLER = Some(f as *const F as usize) }}"));
                try!(writeln!(out, "       extern \"C\" fn wrapper<W: HandleInterrupt>() {{"));
                try!(writeln!(out, "          unsafe {{ (*(HANDLER.unwrap() as *const W)).handle_interrupt() }}"));
                try!(writeln!(out, "       }}"));
                try!(writeln!(out, "       set_handler({}, Some(wrapper::<F>));", irq.value));
                try!(writeln!(out, "       IrqGuard::new({})", irq.value));
                try!(writeln!(out, "   }}"));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));
            }
            for ch in p.channels.iter() {
                for irq in ch.interrupts.iter() {
                    let irq_const = irq.name.to_uppercase();
                    let irq_id = format!("Irq{}", to_camel(&irq.name));
                    try!(writeln!(out, "impl RegisterHandler for {} {{", irq_id));
                    try!(writeln!(out, "   fn register_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleInterrupt>(&self, f: &F) -> IrqGuard<'a> {{"));;          
                    try!(writeln!(out, "       static mut HANDLER: Option<usize> = None;"));
                    try!(writeln!(out, "       unsafe {{ HANDLER = Some(f as *const F as usize) }}"));
                    try!(writeln!(out, "       extern \"C\" fn wrapper<W: HandleInterrupt>() {{"));
                    try!(writeln!(out, "          unsafe {{ (*(HANDLER.unwrap() as *const W)).handle_interrupt() }}"));
                    try!(writeln!(out, "       }}"));
                    try!(writeln!(out, "       set_handler({}, Some(wrapper::<F>));", irq.value));
                    try!(writeln!(out, "       IrqGuard::new({})", irq.value));
                    try!(writeln!(out, "   }}"));
                    try!(writeln!(out, "}}"));
                    try!(writeln!(out, ""));
                }                
            }
        }
    }


    // let mut interrupt_types = HashSet::new();
    // 
    // for pg in d.peripheral_groups.iter() {
    //     for p in pg.peripherals.iter() {
    //         for irq in p.interrupts.iter() {
    //             for itype in irq.types.iter() {
    //                 if !interrupt_types.contains(&itype) {
    //                     let itype_rtrait = format!("Register{}Handler", to_camel(itype));
    //                     let itype_trait = format!("Handle{}", to_camel(itype));
    //                     let itype_meth = format!("handle_{}", itype.to_lowercase());
    //                     try!(writeln!(out, "pub trait {} {{", itype_rtrait));
    //                     try!(writeln!(out, "   fn register_{}_handler<'a, F: {}>(&self, f: &F) -> IrqGuard<'a>;", itype.to_lowercase(), itype_trait));
    //                     try!(writeln!(out, "}}"));        
    //                     try!(writeln!(out, ""));                
    //                     try!(writeln!(out, "pub trait {} {{", itype_trait));    
    //                     try!(writeln!(out, "   fn {}(&self);", itype_meth));
    //                     try!(writeln!(out, "}}"));
    //                     try!(writeln!(out, ""));
    //                     interrupt_types.insert(itype);
    //                 }
    //             }
    //         }
    //     }
    // }

    // for pg in d.peripheral_groups.iter() {
    //     for p in pg.peripherals.iter() {
    //         let p_type = format!("::{}::{}", pg.name.to_lowercase(), to_camel(&p.name));
    //         for irq in p.interrupts.iter() {
    //             for itype in irq.types.iter() {
    //                 let itype_rtrait = format!("Register{}Handler", to_camel(itype));
    //                 let itype_trait = format!("Handle{}", to_camel(itype));
    //                 let itype_meth = format!("handle_{}", itype.to_lowercase());
    //                 try!(writeln!(out, "impl {} for {} {{", itype_rtrait, p_type));
    //                 try!(writeln!(out, "   fn register_{}_handler<'a, F: {}>(&self, f: &F) -> IrqGuard<'a> {{", itype.to_lowercase(), itype_trait));
    //                 try!(writeln!(out, "       static mut HANDLER: Option<usize> = None;"));
    //                 try!(writeln!(out, "       unsafe {{ HANDLER = Some(f as *const F as usize) }}"));
    //                 try!(writeln!(out, "       extern \"C\" fn wrapper<W: {}>() {{", itype_trait));
    //                 try!(writeln!(out, "          unsafe {{ (*(HANDLER.unwrap() as *const W)).{}() }}", itype_meth));
    //                 try!(writeln!(out, "       }}"));
    //                 try!(writeln!(out, "       set_handler({}, Some(wrapper::<F>));", irq.value));
    //                 try!(writeln!(out, "       IrqGuard::new({})", irq.value));
    //                 try!(writeln!(out, "   }}"));
    //                 try!(writeln!(out, "}}"));                        
    //             }
    //         }
    //     }
    // }    


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

pub fn gen_signals<W: Write>(cfg: &Config, out: &mut W, d: &Device) -> Result<()> {
    let mut signals = HashSet::new();
    let mut signal_types = HashSet::new();

    try!(writeln!(out, "//! Signals"));
    try!(writeln!(out, ""));


    try!(writeln!(out, "pub trait Signal<T> {{}}"));
    try!(writeln!(out, ""));

    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            for s in p.signals.iter() {                
                for st in s.types.iter() {
                    let st_type = to_camel(&st);
                    if !signal_types.contains(&st_type) {
                        try!(writeln!(out, "pub trait {} {{}}", st_type));
                        try!(writeln!(out, "pub trait Signal{}<T> {{}}", st_type));
                    }
                    signal_types.insert(st_type);                    
                }                
            }
            for ch in p.channels.iter() {
                for s in ch.signals.iter() {                
                    for st in s.types.iter() {
                        let st_type = to_camel(&st);
                        if !signal_types.contains(&st_type) {
                            try!(writeln!(out, "pub trait {} {{}}", st_type));
                            try!(writeln!(out, "pub trait Signal{}<T> {{}}", st_type));
                        }
                        signal_types.insert(st_type);                    
                    }                
                }                
            }
        }
    }
    try!(writeln!(out, ""));

    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            let p_type = to_camel(&p.name);
            for s in p.signals.iter() {
                let s_const = s.name.to_uppercase();
                let s_type = to_camel(&s.name);
                try!(writeln!(out, "pub const {}: {} = {} {{}};", s_const, s_type, s_type));
                try!(writeln!(out, "pub struct {} {{}}", s_type));
                for st in s.types.iter() {
                    let st_type = to_camel(&st);
                    try!(writeln!(out, "impl {} for {} {{}}", st_type, s_type));
                }                
                try!(writeln!(out, ""));
                signals.insert(s_type);
            }
            for ch in p.channels.iter() {
                let ch_type = to_camel(&ch.name);
                for s in ch.signals.iter() {
                    let s_const = s.name.to_uppercase();
                    let s_type = to_camel(&s.name);
                    try!(writeln!(out, "pub const {}: {} = {} {{}};", s_const, s_type, s_type));
                    try!(writeln!(out, "pub struct {} {{}}", s_type));
                    for st in s.types.iter() {
                        let st_type = to_camel(&st);
                        try!(writeln!(out, "impl {} for {} {{}}", st_type, s_type));
                    }                
                    try!(writeln!(out, ""));
                    signals.insert(s_type);
                }                
            }
        }
    }
    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            for pin in p.pins.iter() {
                for af in pin.altfns.iter() {
                    let s_const = af.signal.to_uppercase();
                    let s_type = to_camel(&af.signal);
                    if !signals.contains(&s_type) {
                        try!(writeln!(out, "pub const {}: {} = {} {{}};", s_const, s_type, s_type));
                        try!(writeln!(out, "pub struct {} {{}}", s_type));
                        try!(writeln!(out, ""));
                        signals.insert(s_type);
                    }
                }
            }
        }
    }    

    Ok(())
}

pub fn gen_peripheral_group<W: Write>(cfg: &Config, out: &mut W, pg: &PeripheralGroup) -> Result<()> {
    let mut link_traits = HashSet::new();

    if let Some(ref desc) = pg.description {
        let desc = desc.trim();
        if desc.len() > 0 {
            try!(writeln!(out, "//! {}", desc));
        }
    }       

    if cfg.bit_types {
        try!(writeln!(out, "#[allow(unused_imports)] use bobbin_common::bits;"));
    }

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

    // let mut count: usize = 0;

    // Generate Link Traits
    for p in pg.peripherals.iter() {
        let pg_type = to_camel(&pg.name);
        let p_type = to_camel(&p.name);
        for l in p.links.iter() {
            let l_trait = format!("Link{}<T>", to_camel(&l.name));
            let l_getter = field_getter(&l.name);
            let pg_mod = l.peripheral_group.to_lowercase();
            //let l_type = format!("::core::ops::Deref<Target=super::{}::{}Impl>", pg_mod, to_camel(&l.peripheral_group));

            if !link_traits.contains(&l_trait) {
                try!(writeln!(out, "pub trait {} {{", l_trait));
                try!(writeln!(out, "   fn {}(&self) -> T;", l_getter));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));
                link_traits.insert(l_trait);
            }

            // let l_trait = format!("{}Link", to_camel(&l.name));
            // let l_getter = field_getter(&l.name);
            // let pg_mod = l.peripheral_group.to_lowercase();
            // //let l_type = format!("::core::ops::Deref<Target=super::{}::{}Impl>", pg_mod, to_camel(&l.peripheral_group));
            // let l_type = format!("{}<{}>", pg_type, p_type);

            // if !link_traits.contains(&l_trait) {
            //     try!(writeln!(out, "pub trait {} {{", l_trait));
            //     try!(writeln!(out, "   fn {}(&self) -> &{};", l_getter, l_type));
            //     try!(writeln!(out, "}}"));
            //     try!(writeln!(out, ""));
            //     link_traits.insert(l_trait);
            // }
        }
    }

    let pg_type = "Periph";
    let p_impl_type = format!("{}<T>", pg_type);
    let mut p_count = 0;
    for p in pg.peripherals.iter() {
        if p.features.len() > 0 {
            try!(writeln!(out, "#[cfg(any("));
            for f in p.features.iter() {
                try!(writeln!(out, "feature=\"{}\",", f));
            }
            try!(writeln!(out, "))]"));
        }
        p_count += 1;
        let p_name = p.name.to_uppercase();
        let p_type = to_camel(&p.name);
        let p_id = format!("{}Id", p_type);
        try!(writeln!(out, "pub const {}: {} = {}(0x{:08x}, {} {{}});", p_name, p_type, pg_type, p.address, p_id));
        
    }
    try!(writeln!(out, ""));

    // if p_count == 0 {
    if pg.modules.len() == 0 {
        try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));
        try!(gen_doc(cfg, out, &format!("{} Peripheral", pg.name.to_uppercase())));
        try!(writeln!(out, "pub struct {}<T>(pub u32, pub T); ", pg_type));
        try!(writeln!(out, ""));        
    }

    for p in pg.peripherals.iter() {
        let p_type = to_camel(&p.name);
        let p_id = format!("{}Id", p_type);
        try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));
        try!(writeln!(out, "#[doc(hidden)]"));
        try!(writeln!(out, "pub struct {} {{}}", p_id));
        try!(writeln!(out, "pub type {} = Periph<{}>;", p_type, p_id));
    }
    try!(writeln!(out, ""));

    for p in pg.peripherals.iter() {
        let p_type = to_camel(&p.name);
        let p_id = format!("{}Id", p_type);
        let p_name = p.name.to_uppercase();
        // Generate Links

        for l in p.links.iter() {
            let l_trait = format!("Link{}", to_camel(&l.name));
            let l_getter = field_getter(&l.name);
            let pg_mod = l.peripheral_group.to_lowercase();
            let l_type = format!("super::{}::Periph<super::{}::{}Id>", pg_mod, pg_mod, to_camel(&l.peripheral));
            //let l_type = format!("::core::ops::Deref<Target=super::{}::{}Impl>", pg_mod, to_camel(&l.peripheral_group));

            let p_const = l.peripheral.to_uppercase();            
                  
            try!(writeln!(out, "impl {}<{}> for {} {{", l_trait, l_type, p_type));
            try!(writeln!(out, "   fn {}(&self) -> {} {{ super::{}::{} }}", l_getter, l_type, pg_mod, l.peripheral.to_uppercase()));
            try!(writeln!(out, "}}"));
            try!(writeln!(out, ""));
        }


        // Generate Signals

        for s in p.signals.iter() {
            let s_type = to_camel(&s.name);
            try!(writeln!(out, "impl super::sig::Signal<super::sig::{}> for {} {{}}", s_type, p_type));
            for st in s.types.iter() {
                let st_type = to_camel(&st);
                try!(writeln!(out, "impl super::sig::Signal{}<super::sig::{}> for {} {{}}", st_type, s_type, p_type));
            }
        }
        for ch in p.channels.iter() {
            let ch_type = to_camel(&ch.name);
            for s in ch.signals.iter() {
                let s_type = to_camel(&s.name);
                try!(writeln!(out, "impl super::sig::Signal<super::sig::{}> for {} {{}}", s_type, ch_type));
                for st in s.types.iter() {
                    let st_type = to_camel(&st);
                    try!(writeln!(out, "impl super::sig::Signal{}<super::sig::{}> for {} {{}}", st_type, s_type, ch_type));
                }
            }            
        }
        try!(writeln!(out, ""));
        // count += 1;
    }    
    try!(writeln!(out, ""));

    // Generate peripheral list

    // if pg.peripherals.len() > 0 {
    //     try!(writeln!(out, "pub const {}: [{}; {}] = [", pg.name.to_uppercase(), p_type, count));
    //     for p in pg.peripherals.iter() {
    //         let p_name = p.name.to_uppercase();
    //         try!(writeln!(out, "   {},", p_name));
    //         count += 1;
    //     }    
    //     try!(writeln!(out, "];"));
    //     try!(writeln!(out, ""));
    // }
    
    // if pg.modules.len() == 0 {
    //     try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));
    //     try!(writeln!(out, "pub struct {}(pub u32);", p_impl_type));    
    // }
    let p0 = if let Some(ref p0) = pg.prototype {
        p0
    } else {
        &pg.peripherals[0]
    };

    if p0.registers.len() > 0 {
        try!(gen_registers(cfg, out, &p_impl_type, &p0.registers[..], p0.size.or(Some(32)), p0.access.or(Some(Access::ReadWrite))));
    }
    if p0.clusters.len() > 0 {
        try!(gen_clusters(cfg, out, &p_impl_type, &p0.clusters[..], p0.size.or(Some(32)), p0.access.or(Some(Access::ReadWrite))));
    }

    if p0.descriptors.len() > 0 {
        for desc in p0.descriptors.iter() {
            try!(gen_descriptor(cfg, out, &p_impl_type, &desc));
        }
    }

    let mut pin_count = 0;
    for p in pg.peripherals.iter() {
        pin_count += p.pins.len();
    }

    //if pg.has_pins && pin_count == 0 {
    //if pg.has_pins && (pg.modules.len() > 0 || pin_count > 0) {
    if pg.has_pins {
        // Generate Pin Impl
        try!(gen_doc(cfg, out, &format!("{} Pin", pg.name.to_uppercase())));
        try!(writeln!(out, "pub struct Pin<P, T> {{ pub port: {}<T>, pub index: usize, pub id: P }}",pg_type));
        try!(writeln!(out, ""));

        try!(writeln!(out, "impl<P,T> Pin<P,T> {{"));
        try!(writeln!(out, "   #[inline] pub fn port(&self) -> &Periph<T> {{ &self.port }}"));
        try!(writeln!(out, "   #[inline] pub fn index(&self) -> usize {{ self.index }}"));
        try!(writeln!(out, "}}"));

        // Generate AltFn Trait

        try!(writeln!(out, "pub trait AltFn<T> {{"));
        try!(writeln!(out, "   fn alt_fn(&self) -> usize;"));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));
    }


    if pin_count > 0 {       
        for p in pg.peripherals.iter() {
            let p_name = p.name.to_uppercase();
            let p_type = to_camel(&p.name);
            let p_id = format!("{}Id", p_type);
            for pin in p.pins.iter() {
                let pin_name = pin.name.to_uppercase();                
                let pin_type = to_camel(&pin.name);
                let pin_id = format!("{}Id", pin_type);
                try!(writeln!(out, "pub const {}: Pin<{}, {}> = Pin {{ port: {}, index: {}, id: {} {{}} }}; ", 
                    pin_name, pin_id, p_id, p_name, pin.index.unwrap(), pin_id));                

                try!(writeln!(out, "#[derive(Clone, Copy, PartialEq)]"));
                try!(writeln!(out, "#[doc(hidden)]"));
                try!(writeln!(out, "pub struct {} {{}}", pin_id));
                try!(writeln!(out, "pub type {} = Pin<{}, {}>;", pin_type, pin_id, p_id));
                for af in pin.altfns.iter() {
                    let s_type = to_camel(&af.signal);
                    try!(writeln!(out, "impl AltFn<super::sig::{}> for {} {{", s_type, pin_id));
                    try!(writeln!(out, "   #[inline] fn alt_fn(&self) -> usize {{ {} }}", af.index));
                    try!(writeln!(out, "}}"));
                    try!(writeln!(out, ""));
                }
            }        
        }    

        // // Generate AltFn Traits

        // for p in pg.peripherals.iter() {
        //     let p_name = p.name.to_uppercase();
        //     for pin in p.pins.iter() {
        //         for af in pin.altfns.iter() {
        //             let af_trait = format!("Af{}", to_camel(&af.signal));
        //             let af_fn = format!("af_{}", af.signal.to_lowercase());

        //             if !traits.contains(&af_trait) {
        //                 try!(writeln!(out, "pub trait {} {{", af_trait));
        //                 try!(writeln!(out, "   fn {}(&self) -> usize;", af_fn));
        //                 try!(writeln!(out, "}}"));
        //                 try!(writeln!(out, ""));   
        //                 traits.insert(af_trait);
        //             }           
        //         }
        //     }
        // }        
    }

    // Generate Peripheral Group Channels

    if pg.has_channels {
        // Generate Channel Impl

        try!(writeln!(out, "#[derive(Clone, Copy, PartialEq)]"));
        try!(gen_doc(cfg, out, &format!("{} Channel", pg.name.to_uppercase())));        
        try!(writeln!(out, "pub struct Channel<P, T> {{ pub periph: Periph<T>, pub index: usize, pub id: P }}"));
        try!(writeln!(out, ""));

        try!(writeln!(out, "impl<P,T> Channel<P,T> {{"));
        try!(writeln!(out, "   #[inline] pub fn periph(&self) -> &Periph<T> {{ &self.periph }}"));
        try!(writeln!(out, "   #[inline] pub fn index(&self) -> usize {{ self.index }}"));
        try!(writeln!(out, "}}"));        
        try!(writeln!(out, ""));
    }

    for p in pg.peripherals.iter() {
        let p_name = p.name.to_uppercase();
        let p_type = to_camel(&p.name);
        let p_id = format!("{}Id", p_type);
        for ch in p.channels.iter() {
            let ch_name = ch.name.to_uppercase();                
            let ch_type = to_camel(&ch.name);
            let ch_id = format!("{}Id", ch_type);
            try!(writeln!(out, "pub const {}: Channel<{}, {}> = Channel {{ periph: {}, index: {}, id: {} {{}} }}; ", 
                ch_name, ch_id, p_id, p_name, ch.index.unwrap(), ch_id));

            try!(writeln!(out, "#[derive(Clone, Copy, PartialEq)]"));
            try!(writeln!(out, "#[doc(hidden)]"));            
            try!(writeln!(out, "pub struct {} {{}}", ch_id));
            try!(writeln!(out, "pub type {} = Channel<{}, {}>;", ch_type, ch_id, p_id));
            try!(writeln!(out, ""));
        }        
    }


    // Generate Peripheral Group Interrupts

    let mut interrupt_types = HashSet::new();

    for p in pg.peripherals.iter() {
        for irq in p.interrupts.iter() {
            for itype in irq.types.iter() {
                if !interrupt_types.contains(&itype) {
                    let itype_itrait = format!("Irq{}", to_camel(itype));
                    let itype_rtrait = format!("Register{}Handler", to_camel(itype));
                    let itype_trait = format!("Handle{}", to_camel(itype));
                    let itype_meth = format!("handle_{}", itype.to_lowercase());
                    try!(writeln!(out, "pub trait {}<T> {{", itype_itrait));
                    try!(writeln!(out, "   fn irq_{}(&self) -> super::irq::Irq<T>;", itype.to_lowercase()));
                    try!(writeln!(out, "}}"));        
                    try!(writeln!(out, ""));                
                    try!(writeln!(out, "pub trait {} {{", itype_rtrait));
                    try!(writeln!(out, "   fn register_{}_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + {}>(&self, f: &F) -> super::irq::IrqGuard<'a>;", itype.to_lowercase(), itype_trait));
                    try!(writeln!(out, "}}"));        
                    try!(writeln!(out, ""));                
                    try!(writeln!(out, "pub trait {} {{", itype_trait));    
                    try!(writeln!(out, "   fn {}(&self);", itype_meth));
                    try!(writeln!(out, "}}"));
                    try!(writeln!(out, ""));
                    interrupt_types.insert(itype);
                }
            }
        }
        for ch in p.channels.iter() {
            for irq in ch.interrupts.iter() {
                for itype in irq.types.iter() {
                    if !interrupt_types.contains(&itype) {
                        let itype_itrait = format!("Irq{}", to_camel(itype));
                        let itype_rtrait = format!("Register{}Handler", to_camel(itype));
                        let itype_trait = format!("Handle{}", to_camel(itype));
                        let itype_meth = format!("handle_{}", itype.to_lowercase());
                        try!(writeln!(out, "pub trait {}<T> {{", itype_itrait));
                        try!(writeln!(out, "   fn irq_{}(&self) -> super::irq::Irq<T>;", itype.to_lowercase()));
                        try!(writeln!(out, "}}"));        
                        try!(writeln!(out, ""));                
                        try!(writeln!(out, "pub trait {} {{", itype_rtrait));
                        try!(writeln!(out, "   fn register_{}_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + {}>(&self, f: &F) -> super::irq::IrqGuard<'a>;", itype.to_lowercase(), itype_trait));
                        try!(writeln!(out, "}}"));        
                        try!(writeln!(out, ""));                
                        try!(writeln!(out, "pub trait {} {{", itype_trait));    
                        try!(writeln!(out, "   fn {}(&self);", itype_meth));
                        try!(writeln!(out, "}}"));
                        try!(writeln!(out, ""));
                        interrupt_types.insert(itype);
                    }
                }
            }
        }
    }

    for p in pg.peripherals.iter() {
        let p_type = to_camel(&p.name);            

        for irq in p.interrupts.iter() {
            let irq_type = format!("super::irq::Irq{}", to_camel(&irq.name));
                        
            for itype in irq.types.iter() {
                let itype_itrait = format!("Irq{}<super::irq::{}Id>", to_camel(itype), to_camel(&irq.name));
                let itype_rtrait = format!("Register{}Handler", to_camel(itype));
                let itype_trait = format!("Handle{}", to_camel(itype));
                let itype_meth = format!("handle_{}", itype.to_lowercase());
                try!(writeln!(out, "impl {} for {} {{", itype_itrait, p_type));
                try!(writeln!(out, "   fn irq_{}(&self) -> {} {{ super::irq::IRQ_{} }}", itype.to_lowercase(), irq_type, irq.name.to_uppercase()));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));

                try!(writeln!(out, "impl {} for {} {{", itype_rtrait, p_type));
                try!(writeln!(out, "   fn register_{}_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + {}>(&self, f: &F) -> super::irq::IrqGuard<'a> {{", itype.to_lowercase(), itype_trait));
                try!(writeln!(out, "       static mut HANDLER: Option<usize> = None;"));
                try!(writeln!(out, "       unsafe {{ HANDLER = Some(f as *const F as usize) }}"));
                try!(writeln!(out, "       extern \"C\" fn wrapper<W: {}>() {{", itype_trait));
                try!(writeln!(out, "          unsafe {{ (*(HANDLER.unwrap() as *const W)).{}() }}", itype_meth));
                try!(writeln!(out, "       }}"));
                try!(writeln!(out, "       super::irq::set_handler({}, Some(wrapper::<F>));", irq.value));
                try!(writeln!(out, "       super::irq::IrqGuard::new({})", irq.value));
                try!(writeln!(out, "   }}"));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));
            }
        }
        for ch in p.channels.iter() {
            let ch_type = to_camel(&ch.name);
            for irq in ch.interrupts.iter() {
                let irq_type = format!("super::irq::Irq{}", to_camel(&irq.name));
                            
                for itype in irq.types.iter() {
                    let itype_itrait = format!("Irq{}<super::irq::{}Id>", to_camel(itype), to_camel(&irq.name));
                    let itype_rtrait = format!("Register{}Handler", to_camel(itype));
                    let itype_trait = format!("Handle{}", to_camel(itype));
                    let itype_meth = format!("handle_{}", itype.to_lowercase());
                    try!(writeln!(out, "impl {} for {} {{", itype_itrait, ch_type));
                    try!(writeln!(out, "   fn irq_{}(&self) -> {} {{ super::irq::IRQ_{} }}", itype.to_lowercase(), irq_type, irq.name.to_uppercase()));
                    try!(writeln!(out, "}}"));
                    try!(writeln!(out, ""));

                    try!(writeln!(out, "impl {} for {} {{", itype_rtrait, ch_type));
                    try!(writeln!(out, "   fn register_{}_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + {}>(&self, f: &F) -> super::irq::IrqGuard<'a> {{", itype.to_lowercase(), itype_trait));
                    try!(writeln!(out, "       static mut HANDLER: Option<usize> = None;"));
                    try!(writeln!(out, "       unsafe {{ HANDLER = Some(f as *const F as usize) }}"));
                    try!(writeln!(out, "       extern \"C\" fn wrapper<W: {}>() {{", itype_trait));
                    try!(writeln!(out, "          unsafe {{ (*(HANDLER.unwrap() as *const W)).{}() }}", itype_meth));
                    try!(writeln!(out, "       }}"));
                    try!(writeln!(out, "       super::irq::set_handler({}, Some(wrapper::<F>));", irq.value));
                    try!(writeln!(out, "       super::irq::IrqGuard::new({})", irq.value));
                    try!(writeln!(out, "   }}"));
                    try!(writeln!(out, "}}"));
                    try!(writeln!(out, ""));
                }
            }            
        }
    }



    Ok(())
}

pub fn gen_peripheral<W: Write>(cfg: &Config, out: &mut W, p: &Peripheral) -> Result<()> {
    let p_type = to_camel(&p.group_name.as_ref().unwrap());

    if let Some(ref desc) = p.description {
        let desc = desc.trim();
        if desc.len() > 0 {
            try!(writeln!(out, "//! {}", desc));
        }
    }       

    if cfg.bit_types {
        try!(writeln!(out, "#[allow(unused_imports)] use bobbin_common::bits;"));
    }

    if let Some(dim) = p.dim {
        for (offset, name) in p.iter_dim() {
            let p_name = name.replace("[","").replace("]","");            
            try!(writeln!(out, "pub const {}: {} = {}(0x{:08x});", p_name, p_type, p_type, offset));    
        }
        try!(writeln!(out, ""));

        // let p_arr = p.group_name.as_ref().unwrap().to_uppercase();
        // try!(writeln!(out, "pub const {}: [{}; {}] = [", p_arr, p_type, dim));
        // for (offset, name) in p.iter_dim() {
        //     let p_name = name.replace("[","").replace("]","");            
        //     try!(writeln!(out, "   {},", p_name));
        // }
        // try!(writeln!(out, "];"));

    } else {
        try!(writeln!(out, "pub const {}: {} = {}(0x{:08x});", p.name, p_type, p_type, p.address));    
    }
    try!(writeln!(out, ""));
    
    if let Some(ref desc) = p.description {
        try!(gen_doc(cfg, out, desc));
    }
    
    try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));    
    try!(writeln!(out, "pub struct {}(pub u32);", p_type));    

    if p.registers.len() > 0 {
        try!(gen_registers(cfg, out, &p_type, &p.registers[..], p.size.or(Some(32)), p.access.or(Some(Access::ReadWrite))));
    }

    if p.clusters.len() > 0 {
        try!(gen_clusters(cfg, out, &p_type, &p.clusters[..], p.size.or(Some(32)), p.access.or(Some(Access::ReadWrite))));
    }

    // Generate Links

    let p_size = size_type(p.size.or(Some(32)).unwrap());
    for link in p.links.iter() {
        try!(writeln!(out, "pub trait {} {{", to_camel(&link.name)));
        try!(writeln!(out, "   fn {}(&self) -> {};", field_getter(&link.name), p_size));
        try!(writeln!(out, "   fn {}(&self, value: {});", field_setter(&link.name), p_size));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));
        try!(writeln!(out, "impl {} {{", to_camel(&p.name)));
        try!(writeln!(out, "   #[inline] pub fn {}<P: {}>(&self, p: &P) -> {} {{", field_getter(&link.name), to_camel(&link.name), p_size));
        try!(writeln!(out, "      p.{}()", field_getter(&link.name)));
        try!(writeln!(out, "   }}"));
        try!(writeln!(out, "   #[inline] pub fn {}<P: {}>(&self, p: &P, value: {}) {{", field_setter(&link.name), to_camel(&link.name), p_size));
        try!(writeln!(out, "      p.{}(value)", field_setter(&link.name)));
        try!(writeln!(out, "   }}"));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));
    }


    // Generate Links

    for r in p.registers.iter() {
        for f in r.fields.iter() {
            for link in f.links.iter() {
                // FIXME: Should not be using peripheral size as return value
                let pg_mod = link.peripheral_group.to_lowercase();
                //let l_type = format!("super::{}::{}<super::{}::{}>", pg_mod, to_camel(&link.peripheral_group), pg_mod, to_camel(&link.peripheral));
                let l_type = format!("super::{}::{}", pg_mod, to_camel(&link.peripheral));
                try!(writeln!(out, "impl {} for {} {{", to_camel(&link.name), l_type));
                try!(writeln!(out, "   #[inline] fn {}(&self) -> {} {{ {}.{}().{}().into() }}", field_getter(&link.name),  p_size, p.name, r.name.to_lowercase(), field_getter(&f.name)));                
                try!(writeln!(out, "   #[inline] fn {}(&self, value: {}) {{ {}.{}(|r| r.{}(value)); }}", field_setter(&link.name),  p_size, p.name, field_with(&r.name), field_setter(&f.name)));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));
            }
        }
    }

    // Generate Signals

    for s in p.signals.iter() {
        let s_type = to_camel(&s.name);
        try!(writeln!(out, "impl super::sig::Signal<super::sig::{}> for {} {{}}", s_type, p_type));
        for st in s.types.iter() {
            let st_type = to_camel(&st);
            try!(writeln!(out, "impl super::sig::Signal{}<super::sig::{}> for {} {{}}", st_type, s_type, p_type));
        }
    }
    for ch in p.channels.iter() {
        for s in ch.signals.iter() {
            let s_type = to_camel(&s.name);
            try!(writeln!(out, "impl super::sig::Signal<super::sig::{}> for {} {{}}", s_type, p_type));
            for st in s.types.iter() {
                let st_type = to_camel(&st);
                try!(writeln!(out, "impl super::sig::Signal{}<super::sig::{}> for {} {{}}", st_type, s_type, p_type));
            }
        }        
    }
    try!(writeln!(out, ""));


    Ok(())
}

pub fn gen_peripheral_enum<W: Write>(cfg: &Config, out: &mut W, p: &Peripheral) -> Result<()> {
    let p_name = to_camel(&p.name);
    try!(writeln!(out, "pub enum {} {{", p_name));
    try!(writeln!(out, "  {} = 0x{:08x}, ", p_name, p.address));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    try!(gen_registers(cfg, out, &p_name, &p.registers[..], p.size.or(Some(32)), p.access.or(Some(Access::ReadWrite))));

    Ok(())
}

pub fn gen_clusters<W: Write>(cfg: &Config, out: &mut W, p_type: &str, clusters: &[Cluster], size: Option<u64>, access: Option<Access>) -> Result<()> {
    if p_type.contains("<T>") {
        try!(writeln!(out, "impl<T> {} {{", p_type));
    } else {
        try!(writeln!(out, "impl {} {{", p_type));
    }

    for c in clusters.iter() {
        let c_type = format!("{}", to_camel(&c.name));
        let mod_name = c.name.to_lowercase();
        if let Some(ref desc) = c.description {
            try!(gen_doc(cfg, out, &format!("Get {} Peripheral", desc)));
        }               
        try!(writeln!(out, "   #[inline] pub fn {}(&self) -> {}::{} {{", mod_name, mod_name, c_type));
        try!(writeln!(out, "      {}::{}(self.0 + 0x{:x})", mod_name, c_type, c.offset));
        try!(writeln!(out, "   }}"));
    }
    try!(writeln!(out, "}}"));

    for c in clusters.iter() {        
        let c_type = format!("{}", to_camel(&c.name));
        let mod_name = c.name.to_lowercase();        
        if let Some(ref desc) = c.description {
            try!(gen_doc(cfg, out, &format!("{} Cluster", desc)));
        }                
        try!(writeln!(out, "pub mod {} {{", mod_name));
        try!(writeln!(out, "   #[derive(Clone, Copy, PartialEq, Eq)]"));
        if let Some(ref desc) = c.description {
            try!(gen_doc(cfg, out, &format!("{} Peripheral", desc)));
        }                
        try!(writeln!(out, "   pub struct {}(pub u32);", c_type));
        try!(gen_registers(cfg, out, &c_type, &c.registers[..], c.size.or(size), c.access.or(access)));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, "// End of {}", mod_name));
    }

    Ok(())
}

pub fn gen_descriptor<W: Write>(cfg: &Config, out: &mut W, p_type: &str, desc: &Descriptor) -> Result<()> {
    let d_type = to_camel(&desc.name);
    let d_size = desc.size.expect("Descriptor size is required");

    try!(writeln!(out, ""));

    if let Some(ref desc) = desc.description {
        try!(gen_doc(cfg, out, desc));
    }
    try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));
    try!(writeln!(out, "pub struct {}(pub [u8; {}]);", d_type, d_size));
    try!(writeln!(out, ""));

    try!(writeln!(out, "impl {} {{", d_type));


    for r in desc.registers.iter() {
        let r_type = format!("{}", to_camel(&r.name));
        let r_ptr = field_ptr(&r.name);
        let r_mut = field_mut(&r.name);
        let r_getter = field_getter(&r.name);
        let r_setter = field_setter(&r.name);
        let r_with = field_with(&r.name);
        let r_size = size_type(r.size.unwrap());
        let r_access = r.access.or(Some(Access::ReadWrite)).unwrap();
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
            let i_type = match dim {
                1...32 => format!("bits::R{}", dim),
                64 => format!("bits::U6"),
                128 => format!("bits::U7"),
                256 => format!("bits::U8"),
                _ => panic!("Unsupported dim value for {}: {}", r.name, dim),
            };


            if r_access.is_readable() {
                try!(gen_doc(cfg, out, &format!("Read the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> {} {{ ", r_getter, i_type, r_type));
                try!(writeln!(out, "     let index: {} = index.into();", i_type));
                try!(writeln!(out, "     let index: usize = index.value() as usize;"));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "        {}(::core::ptr::read_volatile(self.0.as_ptr().offset(0x{:x} + {}) as *const {}))", r_type, r_offset, r_shift, r_size));
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "  }}"));
            }
            if r_access.is_writable() {
                try!(gen_doc(cfg, out, &format!("Write the {} register.", r.name.to_uppercase())));
                // try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>>(&mut self, index: I, value: {}) -> &mut Self {{", r_setter, i_type, r_type));
                try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>, {}: FnOnce({}) -> {}>(&self, index: I, f: {}) -> &Self {{", r_setter, i_type, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "     let index: {} = index.into();", i_type));
                try!(writeln!(out, "     let index: usize = index.value() as usize;"));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "        ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x{:x} + {}) as *mut {}, value.0);", r_offset, r_shift, r_size)); 
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "     self"));
                try!(writeln!(out, "  }}"));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(gen_doc(cfg, out, &format!("Modify the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}> + Copy, {}: FnOnce({}) -> {}>(&mut self, index: usize, f: {}) -> &mut Self {{", r_with, i_type, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "     let index: {} = index.into();", i_type));
                try!(writeln!(out, "     let index: usize = index.value() as usize;"));                                
                try!(writeln!(out, "     let tmp = self.{}(index);", r_getter));
                try!(writeln!(out, "     let value = f(tmp);"));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "        ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x{:x} + {}) as *mut {}, value.0);", r_offset, r_shift, r_size)); 
                try!(writeln!(out, "     }}"));

                try!(writeln!(out, "  }}"));            
            }            
        } else {
            // try!(writeln!(out, "  #[inline] pub fn {}(&self) -> *const {} {{ ", r_ptr, r_size));
            // try!(writeln!(out, "     (&self.0 as *const {}).offset({})", r_size, r_offset));
            // try!(writeln!(out, "  }}"));
            // try!(writeln!(out, "  #[inline] pub fn {}(&mut self) -> *mut {} {{ ", r_mut, r_size));
            // try!(writeln!(out, "     (&mut self.0 as *mut {}).offset({})", r_size, r_offset));
            // try!(writeln!(out, "  }}"));
            
            if r_access.is_readable() {
                try!(gen_doc(cfg, out, &format!("Read the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "   #[inline] pub fn {}(&self) -> {} {{ ", r_getter, r_type));
                try!(writeln!(out, "      unsafe {{"));
                try!(writeln!(out, "         {}(::core::ptr::read_volatile(self.0.as_ptr().offset(0x{:x}) as *const {}))", r_type, r_offset, r_size));
                try!(writeln!(out, "      }}"));
                try!(writeln!(out, "   }}"));
            }
            if r_access.is_writable() {
                try!(gen_doc(cfg, out, &format!("Write the {} register.", r.name.to_uppercase())));
                // try!(writeln!(out, "   #[inline] pub fn {}(&mut self, value: {}) -> &mut Self {{", r_setter, r_type));
                try!(writeln!(out, "   #[inline] pub fn {}<{}: FnOnce({}) -> {}>(&self, f: {}) -> &Self {{", r_setter, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "      let value = f({}(0));", r_type));
                try!(writeln!(out, "      unsafe {{"));
                try!(writeln!(out, "         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x{:x}) as *mut {}, value.0);", r_offset, r_size));                    
                try!(writeln!(out, "      }}"));
                try!(writeln!(out, "      self"));
                try!(writeln!(out, "  }}"));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(gen_doc(cfg, out, &format!("Modfy the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "   #[inline] pub fn {}<{}: FnOnce({}) -> {}>(&mut self, f: {}) -> &mut Self {{", r_with, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "      let tmp = self.{}();", r_getter));
                try!(writeln!(out, "      let value = f({}(0));", r_type));
                try!(writeln!(out, "      unsafe {{"));
                try!(writeln!(out, "         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x{:x}) as *mut {}, value.0);", r_offset, r_size));                    
                try!(writeln!(out, "      }}"));
                try!(writeln!(out, "   }}"));            
            }
        }
        try!(writeln!(out, ""));        
    }
    try!(writeln!(out, "}}"));

    try!(gen_register_types(cfg, out, &desc.registers, None, Some(Access::ReadWrite)));

    Ok(())
}

pub fn gen_registers<W: Write>(cfg: &Config, out: &mut W, p_type: &str, regs: &[Register], size: Option<u64>, access: Option<Access>) -> Result<()> {
    try!(gen_register_methods(cfg, out, p_type, regs, size, access));
    try!(gen_register_types(cfg, out, regs, size, access));
    Ok(())
}

pub fn gen_register_methods<W: Write>(cfg: &Config, out: &mut W, p_type: &str, regs: &[Register], size: Option<u64>, access: Option<Access>) -> Result<()> {
    if p_type.contains("<T>") {
        try!(writeln!(out, "impl<T> {} {{", p_type));
    } else {
        try!(writeln!(out, "impl {} {{", p_type));
    }
        
    for r in regs.iter() {  
        let r_type = format!("{}", to_camel(&r.name));
        let r_ptr = field_ptr(&r.name);
        let r_mut = field_mut(&r.name);
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
            let i_type = match dim {
                1...32 => format!("bits::R{}", dim),
                64 => format!("bits::U6"),
                128 => format!("bits::U7"),
                256 => format!("bits::U8"),
                _ => panic!("Unsupported dim value for {}: {}", r.name, dim),
            };

            try!(gen_doc(cfg, out, &format!("Get the *const pointer for the {} register.", r.name.to_uppercase())));
            try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> *const {} {{ ", r_ptr, i_type, r_size));
            try!(writeln!(out, "     let index: {} = index.into();", i_type));
            try!(writeln!(out, "     let index: usize = index.value() as usize;"));
            try!(writeln!(out, "     ((self.0 as usize) + 0x{:x} + {}) as *const {}", r_offset, r_shift, r_size));
            try!(writeln!(out, "  }}"));

            try!(gen_doc(cfg, out, &format!("Get the *mut pointer for the {} register.", r.name.to_uppercase())));
            try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> *mut {} {{ ", r_mut, i_type, r_size));
            try!(writeln!(out, "     let index: {} = index.into();", i_type));
            try!(writeln!(out, "     let index: usize = index.value() as usize;"));
            try!(writeln!(out, "     ((self.0 as usize) + 0x{:x} + {}) as *mut {}", r_offset, r_shift, r_size));
            try!(writeln!(out, "  }}"));


            if r_access.is_readable() {
                try!(gen_doc(cfg, out, &format!("Read the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> {} {{ ", r_getter, i_type, r_type));
                try!(writeln!(out, "     let index: {} = index.into();", i_type));
                try!(writeln!(out, "     let index: usize = index.value() as usize;"));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "        {}(::core::ptr::read_volatile(((self.0 as usize) + 0x{:x} + {}) as *const {}))", r_type, r_offset, r_shift, r_size));
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "  }}"));
            }
            if r_access.is_writable() {
                try!(gen_doc(cfg, out, &format!("Write the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>, {}: FnOnce({}) -> {}>(&self, index: I, f: {}) -> &Self {{", r_setter, i_type, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "     let index: {} = index.into();", i_type));
                try!(writeln!(out, "     let index: usize = index.value() as usize;"));
                try!(writeln!(out, "     let value = f({}(0));", r_type));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "        ::core::ptr::write_volatile(((self.0 as usize) + 0x{:x} + {}) as *mut {}, value.0);", r_offset, r_shift, r_size)); 
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "     self"));
                try!(writeln!(out, "  }}"));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(gen_doc(cfg, out, &format!("Modify the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}> + Copy, {}: FnOnce({}) -> {}>(&self, index: I, f: {}) -> &Self {{", r_with, i_type, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "     let index: {} = index.into();", i_type));
                try!(writeln!(out, "     let index: usize = index.value() as usize;"));                
                try!(writeln!(out, "     let tmp = self.{}(index);", r_getter));
                try!(writeln!(out, "     let value = f(tmp);"));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "        ::core::ptr::write_volatile(((self.0 as usize) + 0x{:x} + {}) as *mut {}, value.0);", r_offset, r_shift, r_size)); 
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "     self"));
                try!(writeln!(out, "  }}"));            
            }            
        } else {
            try!(gen_doc(cfg, out, &format!("Get the *const pointer for the {} register.", r.name.to_uppercase())));
            try!(writeln!(out, "  #[inline] pub fn {}(&self) -> *const {} {{ ", r_ptr, r_size));
            try!(writeln!(out, "     ((self.0 as usize) + 0x{:x}) as *const {}", r_offset, r_size));
            try!(writeln!(out, "  }}"));
            try!(gen_doc(cfg, out, &format!("Get the *mut pointer for the {} register.", r.name.to_uppercase())));
            try!(writeln!(out, "  #[inline] pub fn {}(&self) -> *mut {} {{ ", r_mut, r_size));
            try!(writeln!(out, "     ((self.0 as usize) + 0x{:x}) as *mut {}", r_offset, r_size));
            try!(writeln!(out, "  }}"));
            
            if r_access.is_readable() {
                try!(gen_doc(cfg, out, &format!("Read the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}(&self) -> {} {{ ", r_getter, r_type));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "        {}(::core::ptr::read_volatile(((self.0 as usize) + 0x{:x}) as *const {}))", r_type, r_offset, r_size));
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "  }}"));
            }
            if r_access.is_writable() {
                try!(gen_doc(cfg, out, &format!("Write the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}<{}: FnOnce({}) -> {}>(&self, f: {}) -> &Self {{", r_setter, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "     let value = f({}(0));", r_type));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "        ::core::ptr::write_volatile(((self.0 as usize) + 0x{:x}) as *mut {}, value.0);", r_offset, r_size));                    
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "     self"));
                try!(writeln!(out, "  }}"));                
                // try!(gen_doc(cfg, out, &format!("Write the {} register.", r.name.to_uppercase())));
                // try!(writeln!(out, "  #[inline] pub fn {}(&self, value: {}) -> &Self {{", r_setter, r_type));
                // try!(writeln!(out, "     unsafe {{"));
                // try!(writeln!(out, "        ::core::ptr::write_volatile(((self.0 as usize) + 0x{:x}) as *mut {}, value.0);", r_offset, r_size));                    
                // try!(writeln!(out, "     }}"));
                // try!(writeln!(out, "     self"));
                // try!(writeln!(out, "  }}"));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(gen_doc(cfg, out, &format!("Modify the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}<{}: FnOnce({}) -> {}>(&self, f: {}) -> &Self {{", r_with, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "     let tmp = self.{}();", r_getter));
                try!(writeln!(out, "     let value = f(tmp);"));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "        ::core::ptr::write_volatile(((self.0 as usize) + 0x{:x}) as *mut {}, value.0);", r_offset, r_size));                    
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "     self"));
                try!(writeln!(out, "  }}"));            
                // try!(gen_doc(cfg, out, &format!("Modify the {} register.", r.name.to_uppercase())));
                // try!(writeln!(out, "  #[inline] pub fn {}<{}: FnOnce({}) -> {}>(&self, f: {}) -> &Self {{", r_with, r_typevar, r_type, r_type, r_typevar));
                // try!(writeln!(out, "     let tmp = self.{}();", r_getter));
                // try!(writeln!(out, "     self.{}(f(tmp))", r_setter));
                // try!(writeln!(out, "  }}"));            
            }
        }
        try!(writeln!(out, ""));
    }    
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));
    Ok(())
}

pub fn gen_register_types<W: Write>(cfg: &Config, out: &mut W, regs: &[Register], size: Option<u64>, access: Option<Access>) -> Result<()> {
    for r in regs.iter() {  
        let r_type = format!("{}", to_camel(&r.name));
        let r_size = size_type(r.size.or(size).unwrap());

        if let Some(ref desc) = r.description {
            try!(gen_doc(cfg, out, desc));
        }        
        try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));
        try!(writeln!(out, "pub struct {}(pub {});", r_type, r_size));
        try!(writeln!(out, "impl {} {{", r_type));        
        for f in r.fields.iter() {
            try!(gen_field(cfg, out, &f, &r_size, f.access.or(access)))
        }
        try!(writeln!(out, "}}"));

        try!(writeln!(out, "impl ::core::fmt::Display for {} {{", r_type));
        try!(writeln!(out, "   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{"));
        try!(writeln!(out, "       self.0.fmt(f)"));
        try!(writeln!(out, "   }}"));
        try!(writeln!(out, "}}"));        

        try!(writeln!(out, "impl ::core::fmt::Debug for {} {{", r_type));
        try!(writeln!(out, "   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{"));        
        try!(writeln!(out, "      try!(write!(f, \"[0x{{:08x}}\", self.0));"));
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
    let field_type = if cfg.bit_types {
        format!("bits::U{}", f_width)
    } else {
        format!("{}", size)
    };

    let min_size = if f_width <= 8 {
        "u8"
    } else if f_width <= 16 {
        "u16"
    } else {
        "u32"
    };

    if let Some(dim) = f.dim {
        let f_incr = f.dim_increment.unwrap();
        let f_getter = field_getter(&f.name.replace("%s","x"));
        let f_setter = field_setter(&f.name.replace("%s","x"));
        let i_type = format!("bits::R{}", dim);

        if let Some(ref desc) = f.description {
            try!(gen_doc(cfg, out, desc));
        }
        try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> {} {{", f_getter, i_type, field_type));
        // try!(writeln!(out, "     assert!(index < {});", dim));
        try!(writeln!(out, "     let index: {} = index.into();", i_type));
        try!(writeln!(out, "     let index: usize = index.value();"));
        match f_incr {
            1 => {
                try!(writeln!(out, "     let shift: usize = {} + index;", f_offset));
            },
            2 => {
                try!(writeln!(out, "     let shift: usize = {} + (index << 1);", f_offset));
            },
            4 => {
                try!(writeln!(out, "     let shift: usize = {} + (index << 2);", f_offset));
            },
            8 => {
                try!(writeln!(out, "     let shift: usize = {} + (index << 3);", f_offset));
            },
            _ => {
                try!(writeln!(out, "     let shift: usize = {} + (index * {});", f_offset, f_incr));                
            }
        }
        if cfg.bit_types {
            try!(writeln!(out, "     unsafe {{ ::core::mem::transmute(((self.0 >> shift) & 0x{:x}) as {}) }} // {}", f_mask, min_size, f_bits));
        } else {
            try!(writeln!(out, "     ((self.0 as {}) >> shift) & 0x{:x} // {}", size, f_mask, f_bits));            
        }
        try!(writeln!(out, "  }}"));    

        if let Some(ref desc) = f.description {
            try!(gen_doc(cfg, out, desc));
        }
        if cfg.bit_types {
            try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>, V: Into<{}>>(mut self, index: I, value: V) -> Self {{", f_setter, i_type, field_type));
            try!(writeln!(out, "     let index: {} = index.into();", i_type));
            try!(writeln!(out, "     let index: usize = index.value();"));            
            try!(writeln!(out, "     let value: {} = value.into();", field_type));            
            try!(writeln!(out, "     let value: {} = value.into();", size));
            // try!(writeln!(out, "     assert!(index < {});", dim));
        } else {
            try!(writeln!(out, "  #[inline] pub fn {}(mut self, index: usize, value: {}) -> Self {{", f_setter, field_type));
            try!(writeln!(out, "     let value: {} = value.into();", size));
            try!(writeln!(out, "     assert!(index < {});", dim));
            try!(writeln!(out, "     assert!((value & !0x{:x}) == 0);", f_mask));
        }
        match f_incr {
            1 => {
                try!(writeln!(out, "     let shift: usize = {} + index;", f_offset));
            },
            2 => {
                try!(writeln!(out, "     let shift: usize = {} + (index << 1);", f_offset));
            },
            4 => {
                try!(writeln!(out, "     let shift: usize = {} + (index << 2);", f_offset));
            },
            8 => {
                try!(writeln!(out, "     let shift: usize = {} + (index << 3);", f_offset));
            },
            _ => {
                try!(writeln!(out, "     let shift: usize = {} + (index * {});", f_offset, f_incr));                
            }
        }        
        try!(writeln!(out, "     self.0 &= !(0x{:x} << shift);", f_mask));
        try!(writeln!(out, "     self.0 |= value << shift;"));
        try!(writeln!(out, "     self"));
        try!(writeln!(out, "  }}"));    
        try!(writeln!(out, ""));
    } else {
        if let Some(ref desc) = f.description {
            try!(gen_doc(cfg, out, desc));
        }
        try!(writeln!(out, "  #[inline] pub fn {}(&self) -> {} {{", f_getter, field_type));
        if cfg.bit_types {
            try!(writeln!(out, "     unsafe {{ ::core::mem::transmute(((self.0 >> {}) & 0x{:x}) as {}) }} // {}", f_offset, f_mask, min_size, f_bits));
        } else {
            try!(writeln!(out, "     ((self.0 as {}) >> {}) & 0x{:x} // {}", size, f_offset, f_mask, f_bits));
        }
        try!(writeln!(out, "  }}"));    

        if let Some(ref desc) = f.description {
            try!(gen_doc(cfg, out, desc));
        }


        if cfg.bit_types {
            try!(writeln!(out, "  #[inline] pub fn {}<V: Into<{}>>(mut self, value: V) -> Self {{", f_setter, field_type));
            try!(writeln!(out, "     let value: {} = value.into();", field_type));
            try!(writeln!(out, "     let value: {} = value.into();", size));
        } else {
            try!(writeln!(out, "  #[inline] pub fn {}(mut self, value: {}) -> Self {{", f_setter, field_type));
            try!(writeln!(out, "     let value: {} = value.into();", size));
            try!(writeln!(out, "     assert!((value & !0x{:x}) == 0);", f_mask));
        }
        try!(writeln!(out, "     self.0 &= !(0x{:x} << {});", f_mask, f_offset));
        try!(writeln!(out, "     self.0 |= value << {};", f_offset));
        try!(writeln!(out, "     self"));
        try!(writeln!(out, "  }}"));    
        try!(writeln!(out, ""));
            
    }


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