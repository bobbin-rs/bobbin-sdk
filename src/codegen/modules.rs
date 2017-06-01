use std::io::{Write, Result};
use std::path::{Path, PathBuf};
use clap::{ArgMatches};
use std::fs::File;
use std::collections::HashSet;

use {Access, Device, PeripheralGroup, Peripheral, Register, Cluster, Field, Interrupt, Exception, Pin};

use super::{size_type, field_getter, field_setter, field_with, field_ptr, field_mut, field_name, to_camel};

pub struct Config {
    pub path: PathBuf,
    pub is_root: bool,
}

impl<'a> From<&'a ArgMatches<'a>> for Config {
    fn from(matches: &'a ArgMatches) -> Config {
        Config {
            path: PathBuf::from(matches.value_of("output").expect("No output path specified")),
            is_root: matches.is_present("root"),
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

pub fn gen_signals<W: Write>(cfg: &Config, out: &mut W, d: &Device) -> Result<()> {
    let mut signals = HashSet::new();
    let mut signal_types = HashSet::new();

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
        for l in p.links.iter() {
            let l_trait = format!("{}Link", to_camel(&l.name));
            let l_getter = field_getter(&l.name);
            let pg_mod = l.peripheral_group.to_lowercase();
            let l_type = format!("::core::ops::Deref<Target=super::{}::{}Impl>", pg_mod, to_camel(&l.peripheral_group));

            if !link_traits.contains(&l_trait) {
                try!(writeln!(out, "pub trait {} {{", l_trait));
                try!(writeln!(out, "   fn {}(&self) -> &{};", l_getter, l_type));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));
                link_traits.insert(l_trait);
            }
        }
    }

    let p_impl_type = format!("{}Impl", to_camel(&pg.name));

    for p in pg.peripherals.iter() {
        if p.features.len() > 0 {
            try!(writeln!(out, "#[cfg(any("));
            for f in p.features.iter() {
                try!(writeln!(out, "feature=\"{}\",", f));
            }
            try!(writeln!(out, "))]"));
        }
        let p_name = p.name.to_uppercase();
        let p_type = to_camel(&p.name);
        try!(writeln!(out, "pub const {}: {} = {} {{}};", p_name, p_type, p_type));
        try!(writeln!(out, "pub const {}_REF: &{} = &{};", p_name, p_type, p_name));
        try!(writeln!(out, "pub const {}_IMPL: {} = {}(0x{:08x});", p_name, p_impl_type, p_impl_type, p.address));
        try!(writeln!(out, "pub const {}_IMPL_REF: &{} = &{}_IMPL;", p_name, p_impl_type, p_name));
        try!(writeln!(out, ""));
        
        try!(writeln!(out, "pub struct {} {{}}", p_type));
        try!(writeln!(out, "impl ::core::ops::Deref for {} {{", p_type));
        try!(writeln!(out, "   type Target = {};", p_impl_type));
        try!(writeln!(out, "   #[inline]")); 
        try!(writeln!(out, "   fn deref(&self) -> &{} {{ {}_IMPL_REF }}", p_impl_type, p_name));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));


        // Generate Links

        for l in p.links.iter() {
            let l_trait = format!("{}Link", to_camel(&l.name));
            let l_getter = field_getter(&l.name);
            let pg_mod = l.peripheral_group.to_lowercase();
            let l_type = format!("::core::ops::Deref<Target=super::{}::{}Impl>", pg_mod, to_camel(&l.peripheral_group));
            let p_const = l.peripheral.to_uppercase();            
                  
            try!(writeln!(out, "impl {} for {} {{", l_trait, p_type));
            try!(writeln!(out, "   fn {}(&self) -> &{} {{ super::{}::{}_REF }}", l_getter, l_type, pg_mod, p_const));
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
    
    if pg.modules.len() == 0 {
        try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));
        try!(writeln!(out, "pub struct {}(pub u32);", p_impl_type));    
    }
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

    if pg.has_pins {
        // Generate Pin Impl

        try!(writeln!(out, "pub struct PinImpl {{"));
        try!(writeln!(out, "  pub port: {},", p_impl_type));
        try!(writeln!(out, "  pub index: usize,"));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));

        // Generate Pin Trait

        try!(writeln!(out, "pub trait Pin<T> {{"));
        try!(writeln!(out, "   fn port(&self) -> T;"));
        try!(writeln!(out, "   fn index(&self) -> usize;"));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));

        // Generate AltFn Trait

        try!(writeln!(out, "pub trait AltFn<T> {{"));
        try!(writeln!(out, "   fn alt_fn(&self) -> usize;"));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));
    }

    let mut has_pins = false;
    for p in pg.peripherals.iter() {
        if p.pins.len() > 0 {
            has_pins = true;
        }
    }

    if has_pins {       
        for p in pg.peripherals.iter() {
            let p_name = p.name.to_uppercase();
            let p_type = to_camel(&p.name);
            for pin in p.pins.iter() {
                let pin_name = pin.name.to_uppercase();
                let pin_type = to_camel(&pin.name);
                try!(writeln!(out, "pub const {}: {} = {} {{}}; ", pin_name, pin_type, pin_type));
                try!(writeln!(out, "pub const {}_IMPL: PinImpl = PinImpl {{ port: {}_IMPL, index: {} }};", pin_name, p_name, pin.index.unwrap()));
                try!(writeln!(out, "pub const {}_IMPL_REF: &PinImpl = &{}_IMPL;", pin_name, pin_name));
                try!(writeln!(out, ""));

                try!(writeln!(out, "impl ::core::ops::Deref for {} {{", pin_type));
                try!(writeln!(out, "   type Target = PinImpl;"));
                try!(writeln!(out, "   #[inline]")); 
                try!(writeln!(out, "   fn deref(&self) -> &PinImpl {{ {}_IMPL_REF }}", pin_name));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));

                try!(writeln!(out, "#[derive(Clone, Copy, PartialEq)]"));
                try!(writeln!(out, "pub struct {} {{}}", pin_type));
                try!(writeln!(out, ""));
                try!(writeln!(out, "impl Pin<{}> for {} {{", p_type, pin_type));
                try!(writeln!(out, "   #[inline]")); 
                try!(writeln!(out, "   fn port(&self) -> {} {{ {} }}", p_type, p_name));
                try!(writeln!(out, "   #[inline]")); 
                try!(writeln!(out, "   fn index(&self) -> usize {{ {} }}", pin.index.unwrap()));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));                
                for af in pin.altfns.iter() {
                    let s_type = to_camel(&af.signal);
                    try!(writeln!(out, "impl AltFn<super::sig::{}> for {} {{", s_type, pin_type));
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


    Ok(())
}

pub fn gen_peripheral<W: Write>(cfg: &Config, out: &mut W, p: &Peripheral) -> Result<()> {
    let p_type = to_camel(&p.group_name.as_ref().unwrap());

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
        try!(writeln!(out, "   #[inline]")); 
        try!(writeln!(out, "   pub fn {}<P: {}>(&self, p: &P) -> {} {{", field_getter(&link.name), to_camel(&link.name), p_size));
        try!(writeln!(out, "      p.{}()", field_getter(&link.name)));
        try!(writeln!(out, "   }}"));
        try!(writeln!(out, "   #[inline]")); 
        try!(writeln!(out, "   pub fn {}<P: {}>(&self, p: &P, value: {}) {{", field_setter(&link.name), to_camel(&link.name), p_size));
        try!(writeln!(out, "      p.{}(value)", field_setter(&link.name)));
        try!(writeln!(out, "   }}"));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));
    }


    // Generate Signals

    for r in p.registers.iter() {
        for f in r.fields.iter() {
            for link in f.links.iter() {
                let l_type = format!("super::{}::{}", link.peripheral_group.to_lowercase(), to_camel(&link.peripheral));
                try!(writeln!(out, "impl {} for {} {{", to_camel(&link.name), l_type));
                try!(writeln!(out, "   #[inline]")); 
                try!(writeln!(out, "   fn {}(&self) -> {} {{ {}.{}().{}() }}", field_getter(&link.name),  p_size, p.name, r.name.to_lowercase(), field_getter(&f.name)));
                try!(writeln!(out, "   #[inline]")); 
                try!(writeln!(out, "   fn {}(&self, value: {}) {{ {}.{}(|r| r.{}(value)); }}", field_setter(&link.name),  p_size, p.name, field_with(&r.name), field_setter(&f.name)));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));
            }
        }
    }

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
    try!(writeln!(out, "impl {} {{", p_type));

    for c in clusters.iter() {
        let c_type = format!("{}", to_camel(&c.name));
        let mod_name = c.name.to_lowercase();
        try!(writeln!(out, "   #[inline]")); 
        try!(writeln!(out, "   pub fn {}(&self) -> {}::{} {{", mod_name, mod_name, c_type));
        try!(writeln!(out, "      {}::{}(self.0 + 0x{:x})", mod_name, c_type, c.offset));
        try!(writeln!(out, "   }}"));
    }
    try!(writeln!(out, "}}"));

    for c in clusters.iter() {        
        let c_type = format!("{}", to_camel(&c.name));
        let mod_name = c.name.to_lowercase();        
        try!(writeln!(out, "pub mod {} {{", mod_name));
        try!(writeln!(out, "   #[derive(Clone, Copy, PartialEq, Eq)]"));
        try!(writeln!(out, "   pub struct {}(pub u32);", c_type));
        try!(gen_registers(cfg, out, &c_type, &c.registers[..], c.size.or(size), c.access.or(access)));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, "// End of {}", mod_name));
    }

    Ok(())
}

pub fn gen_registers<W: Write>(cfg: &Config, out: &mut W, p_type: &str, regs: &[Register], size: Option<u64>, access: Option<Access>) -> Result<()> {
    try!(writeln!(out, "impl {} {{", p_type));
    
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
            try!(writeln!(out, "  #[inline]"));
            try!(writeln!(out, "  pub fn {}(&self, index: usize) -> *const {} {{ ", r_ptr, r_size));
            try!(writeln!(out, "     assert!(index < {});", dim));
            try!(writeln!(out, "     ((self.0 as usize) + 0x{:x} + {}) as *const {}", r_offset, r_shift, r_size));
            try!(writeln!(out, "  }}"));

            try!(writeln!(out, "  #[inline]"));
            try!(writeln!(out, "  pub fn {}(&self, index: usize) -> *mut {} {{ ", r_mut, r_size));
            try!(writeln!(out, "     assert!(index < {});", dim));
            try!(writeln!(out, "     ((self.0 as usize) + 0x{:x} + {}) as *mut {}", r_offset, r_shift, r_size));
            try!(writeln!(out, "  }}"));


            if r_access.is_readable() {
                try!(writeln!(out, "  #[inline]"));
                try!(writeln!(out, "  pub fn {}(&self, index: usize) -> {} {{ ", r_getter, r_type));
                try!(writeln!(out, "     assert!(index < {});", dim));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "        {}(::core::ptr::read_volatile(((self.0 as usize) + 0x{:x} + {}) as *const {}))", r_type, r_offset, r_shift, r_size));
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "  }}"));
            }
            if r_access.is_writable() {
                try!(writeln!(out, "  #[inline]"));
                try!(writeln!(out, "  pub fn {}(&self, index: usize, value: {}) -> &{} {{", r_setter, r_type, p_type));
                try!(writeln!(out, "     assert!(index < {});", dim));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "       ::core::ptr::write_volatile(((self.0 as usize) + 0x{:x} + {}) as *mut {}, value.0);", r_offset, r_shift, r_size)); 
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "     self"));
                try!(writeln!(out, "  }}"));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(writeln!(out, "  #[inline]"));
                try!(writeln!(out, "  pub fn {}<{}: FnOnce({}) -> {}>(&self, index: usize, f: {}) -> &{} {{", r_with, r_typevar, r_type, r_type, r_typevar, p_type));
                try!(writeln!(out, "     let tmp = self.{}(index);", r_getter));
                try!(writeln!(out, "     self.{}(index, f(tmp))", r_setter));
                try!(writeln!(out, "  }}"));            
            }            
        } else {
            try!(writeln!(out, "  #[inline]"));
            try!(writeln!(out, "  pub fn {}(&self) -> *const {} {{ ", r_ptr, r_size));
            try!(writeln!(out, "     ((self.0 as usize) + 0x{:x}) as *const {}", r_offset, r_size));
            try!(writeln!(out, "  }}"));
            try!(writeln!(out, "  #[inline]"));
            try!(writeln!(out, "  pub fn {}(&self) -> *mut {} {{ ", r_mut, r_size));
            try!(writeln!(out, "     ((self.0 as usize) + 0x{:x}) as *mut {}", r_offset, r_size));
            try!(writeln!(out, "  }}"));
            
            if r_access.is_readable() {
                try!(writeln!(out, "  #[inline]"));
                try!(writeln!(out, "  pub fn {}(&self) -> {} {{ ", r_getter, r_type));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "       {}(::core::ptr::read_volatile(((self.0 as usize) + 0x{:x}) as *const {}))", r_type, r_offset, r_size));
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "  }}"));
            }
            if r_access.is_writable() {
                try!(writeln!(out, "  #[inline]"));
                try!(writeln!(out, "  pub fn {}(&self, value: {}) -> &{} {{", r_setter, r_type, p_type));
                try!(writeln!(out, "     unsafe {{"));
                try!(writeln!(out, "       ::core::ptr::write_volatile(((self.0 as usize) + 0x{:x}) as *mut {}, value.0);", r_offset, r_size));                    
                try!(writeln!(out, "     }}"));
                try!(writeln!(out, "     self"));
                try!(writeln!(out, "  }}"));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(writeln!(out, "  #[inline]"));
                try!(writeln!(out, "  pub fn {}<{}: FnOnce({}) -> {}>(&self, f: {}) -> &{} {{", r_with, r_typevar, r_type, r_type, r_typevar, p_type));
                try!(writeln!(out, "     let tmp = self.{}();", r_getter));
                try!(writeln!(out, "     self.{}(f(tmp))", r_setter));
                try!(writeln!(out, "  }}"));            
            }
        }
        try!(writeln!(out, ""));
    }    
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    for r in regs.iter() {  
        let r_type = format!("{}", to_camel(&r.name));
        let r_size = size_type(r.size.or(size).unwrap());
        try!(writeln!(out, "#[derive(PartialEq, Eq)]"));
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

    if let Some(dim) = f.dim {
        let f_incr = f.dim_increment.unwrap();
        let f_getter = field_getter(&f.name.replace("%s","x"));
        let f_setter = field_setter(&f.name.replace("%s","x"));
        try!(writeln!(out, "  #[inline]")); 
        try!(writeln!(out, "  pub fn {}(&self, index: usize) -> {} {{", f_getter, size));
        try!(writeln!(out, "     assert!(index < {});", dim));
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
        try!(writeln!(out, "     ((self.0 as {}) >> shift) & 0x{:x} // {}", size, f_mask, f_bits));
        try!(writeln!(out, "  }}"));    

        try!(writeln!(out, "  #[inline]")); 
        try!(writeln!(out, "  pub fn {}(mut self, index: usize, value: {}) -> Self {{", f_setter, size));
        try!(writeln!(out, "     assert!(index < {});", dim));
        try!(writeln!(out, "     assert!((value & !0x{:x}) == 0);", f_mask));
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
        try!(writeln!(out, "  #[inline]")); 
        try!(writeln!(out, "  pub fn {}(&self) -> {} {{", f_getter, size));
        try!(writeln!(out, "     ((self.0 as {}) >> {}) & 0x{:x} // {}", size, f_offset, f_mask, f_bits));
        try!(writeln!(out, "  }}"));    

        try!(writeln!(out, "  #[inline]")); 
        try!(writeln!(out, "  pub fn {}(mut self, value: {}) -> Self {{", f_setter, size));
        try!(writeln!(out, "     assert!((value & !0x{:x}) == 0);", f_mask));
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