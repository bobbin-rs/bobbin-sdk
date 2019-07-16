use std::io::{Write, Result};
use std::path::{Path, PathBuf};
use clap::{ArgMatches};
use std::fs::File;
use std::collections::{HashSet, HashMap};

use {Access, Device, PeripheralGroup, Peripheral, Descriptor, Register, Cluster, Field, Interrupt, Exception, Clock};

use super::{size_type, field_getter, field_setter, field_writer, field_with, field_test, field_reg, field_ptr, field_mut, field_name, to_camel, gen_doc};

pub type SignalMap = HashMap<String, (String, String, String)>;

pub struct Config {
    pub path: PathBuf,
    pub is_root: bool,
    pub common: String,
}

impl<'a> From<&'a ArgMatches<'a>> for Config {
    fn from(matches: &'a ArgMatches) -> Config {
        Config {
            path: PathBuf::from(matches.value_of("output").expect("No output path specified")),
            is_root: matches.is_present("root"),
            common: String::from("common"),
        }
    }
}

pub fn gen_modules<W: Write>(matches: &ArgMatches, _out: &mut W, d: &Device) -> Result<()> {
    let cfg = Config::from(matches);
    let out_path = &cfg.path;
    let p_mod = if cfg.is_root {
        out_path.join("lib.rs")
    } else {
        out_path.join("mod.rs")
    };
    let mut f_mod = try!(File::create(p_mod));
    // try!(writeln!(f_mod, "#[allow(unused_imports)] use {}::*;", cfg.common));
    
    try!(gen_mod(&cfg, &mut f_mod, d, out_path));

    Ok(())
}

pub fn gen_mod<W: Write>(cfg: &Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {
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
        let c_name = c.name.replace("-","_");
        try!(writeln!(out, "extern crate {};", c_name));
        for m in c.modules.iter() {
            if let Some(ref use_as) = m._as {
                try!(writeln!(out, "pub use {}::{} as {};", c_name, m.name, use_as));
            } else {
                try!(writeln!(out, "pub use {}::{};", c_name, m.name));
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

    let mut ord = 0;

    for p in d.peripherals.iter() {
        let p_name = p.group_name.as_ref().unwrap_or(&p.name).to_lowercase();
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(gen_peripheral(cfg, &mut f_mod, d, p, ord));
        try!(gen_peripheral_impl(cfg, &mut f_mod, p));
        ord += 1;
    }

    for pg in d.peripheral_groups.iter() {
        let pg_name = pg.name.to_lowercase();
        try!(writeln!(out, "pub mod {};", pg_name));
        let p_mod = path.join(format!("{}.rs", pg_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(gen_peripheral_group(cfg, &mut f_mod, d, pg, &mut ord));
        try!(gen_peripheral_group_impl(cfg, &mut f_mod, pg));
        ord += 1;
    }
    Ok(())
}

pub fn gen_exceptions<W: Write>(_cfg: &Config, out: &mut W, exceptions: &Vec<Exception>) -> Result<()> {
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
    try!(writeln!(out, "    pub fn set_handler(&self, handler: Option<Handler>) {{"));
    try!(writeln!(out, "        unsafe {{ R_EXCEPTION_HANDLERS[self.0] = handler }};"));
    try!(writeln!(out, "    }}"));
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    try!(writeln!(out,"#[cfg_attr(target_os=\"none\", link_section=\".vector_table.exceptions\")]"));
    try!(writeln!(out,"#[no_mangle]"));
    try!(writeln!(out,"pub static EXCEPTIONS: [Option<Handler>; {}] = [", exceptions.len()));
    for e in exceptions.iter() {
        if e.name != "" {
            try!(writeln!(out, "    {:30} // {}", format!("Some(_{}),", e.name), e.description.as_ref().unwrap()));
        } else {
            try!(writeln!(out, "    None,"));
        }        
    }
    try!(writeln!(out,"];"));
    try!(writeln!(out,""));
   
    
    try!(writeln!(out,"#[cfg_attr(target_os=\"none\", link_section=\".bss.r_exceptions\")]"));
    try!(writeln!(out,"#[no_mangle]"));
    try!(writeln!(out,"pub static mut R_EXCEPTION_HANDLERS: [Option<Handler>; {}] = [None; {}];", exceptions.len(), exceptions.len()));
    try!(writeln!(out,""));    


    try!(writeln!(out, "extern \"C\" {{"));
    for e in exceptions.iter() {
        if e.name != "" {
            try!(writeln!(out, "    {:30} // {}", format!("pub fn _{}();", e.name), e.description.as_ref().unwrap()));
        }      
    }
    try!(writeln!(out,"}}"));    
    Ok(())
}
pub fn gen_interrupts<W: Write>(_cfg: &Config, out: &mut W, d: &Device, interrupt_count: u64) -> Result<()> {
    let mut interrupts: Vec<Option<&Interrupt>> = Vec::with_capacity(interrupt_count as usize);

    writeln!(out, "//! Interrupts")?;
    writeln!(out, "//!")?;
    writeln!(out, "//! The interrupt vector table is populated by `cortex-m-rt` crate with `DefaultHandler`s.")?;
    writeln!(out, "//! Board support crates should implement their `DefaultHandler` and pass an IRQ number to the IRQ dispatcher.")?;
    writeln!(out, "")?;

    writeln!(out, "extern crate cortex_m_rt;")?;
    writeln!(out, "extern crate cortex_m;")?;
    writeln!(out, "")?;

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

    for i in 0..interrupt_count {
        writeln!(out, "::bobbin_mcu::irq_number!(IRQ_{}, Irq{}, {});", i, i, i)?;
    }

    writeln!(out, "")?;

    // Generate Defined Interrupt Types

    let mut interrupt_types = HashSet::new();

    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            for irq in p.interrupts.iter() {
                for itype in irq.types.iter() {
                    if !interrupt_types.contains(&itype) {
                        writeln!(out, "::bobbin_mcu::irq_type!(IRQ_{}, Irq{});",
                            itype.to_uppercase(),
                            to_camel(&itype),
                        )?;
                        interrupt_types.insert(itype);
                    }
                }
            }

            for ch in p.channels.iter() {
                for irq in ch.interrupts.iter() {
                    for itype in irq.types.iter() {
                        if !interrupt_types.contains(&itype) {
                            writeln!(out, "::bobbin_mcu::irq_type!(IRQ_{}, Irq{});",
                                itype.to_uppercase(),
                                to_camel(&itype),
                            )?;
                            interrupt_types.insert(itype);
                        }
                    }
                }
            }
        }    
    }


    for p in d.peripherals.iter() {
        for irq in p.interrupts.iter() {
            for itype in irq.types.iter() {
                if !interrupt_types.contains(&itype) {               
                    writeln!(out, "::bobbin_mcu::irq_type!(IRQ_{}, Irq{});",
                        itype.to_uppercase(),
                        to_camel(&itype),
                    )?;
                    interrupt_types.insert(itype);
                }
            }
        }
        for ch in p.channels.iter() {
            for irq in ch.interrupts.iter() {
                for itype in irq.types.iter() {
                    if !interrupt_types.contains(&itype) {
                        writeln!(out, "::bobbin_mcu::irq_type!(IRQ_{}, Irq{});",
                            itype.to_uppercase(),
                            to_camel(&itype),
                        )?;
                        interrupt_types.insert(itype);
                    }
                }
            }
        }
    }

    for pg in d.peripheral_groups.iter() {
        let mut use_local_irq_type = false;
        let pg_mod = pg.name.to_lowercase();
        for p in pg.peripherals.iter() {
            let mut p_first = true;
            for irq in p.interrupts.iter() {
                if p_first {
                    writeln!(out, "::bobbin_mcu::irq!(::{}::{}, {}, Irq{});", pg_mod, to_camel(&p.name), "::bobbin_mcu::irq::IrqMain", irq.value)?;
                }
                if irq.types.len() == 0 {
                    writeln!(out, "::bobbin_mcu::irq!(::{}::{}, Irq{}, Irq{});", pg_mod, to_camel(&p.name), to_camel(&pg.name), irq.value)?;
                    use_local_irq_type = true;
                }
                p_first = false;
            }
            for ch in p.channels.iter() {
                let mut ch_first = true;
                for irq in ch.interrupts.iter() {
                    if ch_first {
                        writeln!(out, "::bobbin_mcu::irq!(::{}::{}, {}, Irq{});",  pg_mod, to_camel(&ch.name), "::bobbin_mcu::irq::IrqMain", irq.value)?;
                    }
                    if irq.types.len() == 0 {
                        writeln!(out, "::bobbin_mcu::irq!(::{}::{}, Irq{}, Irq{});",  pg_mod, to_camel(&ch.name), to_camel(&pg.name), irq.value)?;
                        use_local_irq_type = true;
                    }
                    for itype in irq.types.iter() {
                        writeln!(out, "::bobbin_mcu::irq!(::{}::{}, Irq{}, Irq{});",  pg_mod, to_camel(&ch.name), to_camel(&itype), irq.value)?;
                    }
                    ch_first = false;                    
                }
            }
        }
        if use_local_irq_type {
            writeln!(out, "::bobbin_mcu::irq_type!(IRQ_{}, Irq{});",
                pg.name.to_uppercase(),
                to_camel(&pg.name),
            )?;
        }
    }  

    for p in d.peripherals.iter() {
        let p_mod = p.group_name.as_ref().unwrap().to_lowercase();
        let mut use_local_irq_type = false;    
        let mut p_first = true;
        for irq in p.interrupts.iter() {
            if p_first {
                writeln!(out, "::bobbin_mcu::irq!(::{}::{}, {}, Irq{});", p_mod, to_camel(&p.name), "::bobbin_mcu::irq::IrqMain", irq.value)?;
            }
            // NOTE: why is this loop different?
            for itype in irq.types.iter() {
                if irq.types.len() == 0 {
                    writeln!(out, "::bobbin_mcu::irq!(::{}::{}, Irq{}, Irq{});", p_mod, to_camel(&p.name), to_camel(&p.name), irq.value)?;
                    use_local_irq_type = true;
                }            
                writeln!(out, "::bobbin_mcu::irq!(::{}::{}, {}, Irq{});", p_mod, to_camel(&p.name), to_camel(&itype), irq.value)?;
            }
            p_first = false;
        }
        for ch in p.channels.iter() {
            let mut ch_first = true;            
            for irq in ch.interrupts.iter() {
                if ch_first {
                    writeln!(out, "::bobbin_mcu::irq!(::{}::{}, {}, Irq{});", p_mod, to_camel(&ch.name), "::bobbin_mcu::irq::IrqMain", irq.value)?;
                }
                if irq.types.len() == 0 {
                    writeln!(out, "::bobbin_mcu::irq!(::{}::{}, Irq{}, Irq{});", p_mod, to_camel(&ch.name), to_camel(&p.name), irq.value)?;
                    use_local_irq_type = true;
                }            
                for itype in irq.types.iter() {
                    writeln!(out, "::bobbin_mcu::irq!(::{}::{}, Irq{}, Irq{});", p_mod, to_camel(&ch.name), to_camel(&itype), irq.value)?;
                }
                ch_first = false;
            }
        }
        if use_local_irq_type {
            writeln!(out, "::bobbin_mcu::irq_type!(IRQ_{}, Irq{});",
                p.name.to_uppercase(),
                to_camel(&p.name),
            )?;
        } 
    }   
    Ok(())
}
pub fn gen_pins<W: Write>(_cfg: &Config, out: &mut W, d: &Device, signals: &SignalMap) -> Result<()> {
    let mut mod_set = HashSet::new();

    for pg in &d.peripheral_groups {
        let pg_mod = pg.name.to_lowercase();
        for p in &pg.peripherals {
            let p_name = p.name.to_uppercase();
            let p_type = to_camel(&p.name);
            let base_type = format!("{}Pin", to_camel(&pg.name));
            
            for pin in p.pins.iter() {
                if !mod_set.contains(&pg_mod) {
                    try!(writeln!(out, "pub use super::{}::*;", pg_mod));
                    try!(writeln!(out, ""));
                    mod_set.insert(pg_mod.clone());
                }

                let id = pin.name.to_uppercase();                
                let ty = to_camel(&pin.name);
                let meth = pin.name.to_lowercase();
                let owned = format!("{}_OWNED", id);
                let ref_count = format!("{}_REF_COUNT", id);
                let base_name = format!("{}_PIN", id);
                let base_port = format!("{}_PERIPH", p_name);
                let pin_index = pin.index.unwrap();

                try!(writeln!(out, "::bobbin_mcu::pin!({id}, {ty}, {meth}, {port_id}, {port_type}, {base_id}, {base_type}, {base_port}, {owned}, {ref_count}, {index});",
                    id=id,
                    ty=ty,
                    meth=meth,
                    port_id=p_name,
                    port_type=p_type,
                    base_id=base_name,
                    base_type=base_type,
                    base_port=base_port,
                    owned=owned,
                    ref_count=ref_count,
                    index=pin_index,
                ));

                for af in pin.altfns.iter() {
                    let sig = to_camel(&af.signal);
                    if let Some(&(ref src_mod, ref src_type, ref sig_type)) = signals.get(&sig) {
                        let af_index = format!("::bobbin_bits::U4::B{:04b}", af.index);
                        try!(writeln!(out, "   ::bobbin_mcu::pin_source!({}, super::{}::{}, super::sig::{}, {});",
                            ty,
                            src_mod,
                            src_type,
                            sig_type,
                            af_index,
                        ));
                    } else {
                        // println!("Error: Signal {} has not been defined.", sig);
                    }                
                }                
            }
        }
    }    
    Ok(())
}

pub fn gen_signals<W: Write>(_cfg: &Config, out: &mut W, d: &Device) -> Result<SignalMap> {
    let mut signal_types = HashSet::new();
    let mut signals = HashMap::new();

    // Collect signals and generate signal types

    for pg in &d.peripheral_groups {
        let pg_mod = pg.name.to_lowercase();
        for p in &pg.peripherals {
            for s in p.signals.iter() {
                let s_type = to_camel(&s.name);
                for st in s.types.iter() {
                    let st_type = format!("Sig{}", to_camel(&st));
                    if !signal_types.contains(&st_type) {
                        try!(writeln!(out, "::bobbin_mcu::signal_type!({}, {});", &st, st_type));
                        signal_types.insert(st_type.clone());
                    }
                    let key = s_type.clone();
                    let value = (pg_mod.clone(), to_camel(&p.name), st_type.clone());
                    // println!("   {} => {:?}", key, value);
                    signals.insert(key, value);
                }
            }

            for ch in p.channels.iter() {
                let ch_type = to_camel(&ch.name);
                for s in ch.signals.iter() {
                    let s_type = to_camel(&s.name);
                    for st in s.types.iter() {
                        let st_type = format!("Sig{}", to_camel(&st));
                        if !signal_types.contains(&st_type) {
                            try!(writeln!(out, "::bobbin_mcu::signal_type!({}, {});", &st, st_type));                            
                            signal_types.insert(st_type.clone());
                        }                 
                        let key = s_type.clone();
                        let value = (pg_mod.clone(), ch_type.clone(), st_type.clone());
                        // println!("  {} => {:?}", key, value);
                        signals.insert(key, value);
                    }
                }          
            }
        }
    }

    try!(writeln!(out, ""));

    // Generate periph_signals and channel_signals

    for pg in &d.peripheral_groups {
        try!(writeln!(out, "// {}", pg.name));
        let pg_mod = pg.name.to_lowercase();
        for p in &pg.peripherals {
            for s in p.signals.iter() {
                for st in s.types.iter() {
                    let st_type = format!("Sig{}", to_camel(&st));                    
                    try!(writeln!(out, "::bobbin_mcu::periph_signal!(super::{}::{}, {});", pg_mod, to_camel(&p.name), st_type));
                }
            }

            for ch in p.channels.iter() {
                for s in ch.signals.iter() {
                    for st in s.types.iter() {
                        let st_type = format!("Sig{}", to_camel(&st));                    
                        try!(writeln!(out, "::bobbin_mcu::channel_signal!(super::{}::{}, {});", pg_mod, to_camel(&ch.name), st_type));
                    }
                }          
            }
        }
        try!(writeln!(out, ""));
    }    
    // Generate pin_source maps

    for pg in &d.peripheral_groups {
        for p in &pg.peripherals {
            for pin in p.pins.iter() {
                let pin_type = to_camel(&pin.name);
                // let pin_index = pin.index.unwrap();
                for af in pin.altfns.iter() {
                    let sig = &af.signal;
                    if let Some(&(ref src_mod, ref src_type, ref sig_type)) = signals.get(sig) {
                        let af_index = format!("::bobbin_bits::U4::B{:04b}", af.index);
                        try!(writeln!(out, "::bobbin_mcu::pin_source!({}, super::{}::{}, {}, {})",
                            pin_type,
                            src_mod,
                            src_type,
                            sig_type,
                            af_index,
                        ));
                    } else {
                        // println!("Error: Signal {} has not been defined.", sig);
                    }                
                }
                try!(writeln!(out, ""));
            }        
        }
    }       

    Ok(signals)
}


pub fn gen_peripheral_group<W: Write>(cfg: &Config, out: &mut W, d: &Device, pg: &PeripheralGroup, ord: &mut usize) -> Result<()> {
    let pg_name = if let Some(ref prototype) = pg.prototype {
        if let Some(ref name) = prototype.group_name {
            format!("{}", name)
        } else {
            format!("{}", pg.name)
        }
    } else {
        format!("{}", pg.name)
    };

    let pg_type = format!("{}Periph", to_camel(&pg_name));
    // let pin_type = format!("{}Pin", to_camel(&pg_name));
    // let ch_type = format!("{}Ch", to_camel(&pg_name));
    
    let mut link_traits = HashSet::new();
    let mut signal_types = HashSet::new();
    let mut signals = HashMap::new();

    // TODO: Description must be added at top of crate

    // if let Some(ref desc) = pg.description {
    //     let desc = desc.trim();
    //     if desc.len() > 0 {
    //         try!(writeln!(out, "// //! {}", desc));
    //         try!(writeln!(out, ""));
    //     }
    // }       


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

    for (i, p) in pg.peripherals.iter().enumerate() {
        if p.features.len() > 0 {
            try!(writeln!(out, "#[cfg(any("));
            for f in p.features.iter() {
                try!(writeln!(out, "feature=\"{}\",", f));
            }
            try!(writeln!(out, "))]"));
        }
        let p_name = p.name.to_uppercase();
        let p_const = format!("{}_PERIPH", p_name);
        let p_type = to_camel(&p.name);
        let p_owned = format!("{}_OWNED", p_name);
        let p_ref_count = format!("{}_REF_COUNT", p_name);
        try!(writeln!(out, "::bobbin_mcu::periph!( {p_name}, {p_type}, {p_const}, {pg_type}, {p_owned}, {p_ref_count}, 0x{p_addr:08x}, 0x{p_index:02x}, 0x{p_ord:02x});", 
            p_const=p_const, pg_type=pg_type, p_name=p_name, p_type=p_type, p_owned=p_owned, p_ref_count=p_ref_count, p_addr=p.address, p_index=i, p_ord=ord));
        *ord += 1;
    }
    try!(writeln!(out, ""));
    

    // Generate Link Traits
    for p in pg.peripherals.iter() {
        // Peripheral Links
        for l in p.links.iter() {
            let l_trait = format!("Link{}<T>", to_camel(&l.name));
            let l_getter = field_getter(&l.name);

            if !link_traits.contains(&l_trait) {
                try!(writeln!(out, "pub trait {} {{", l_trait));
                try!(writeln!(out, "    fn {}(&self) -> T;", l_getter));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));
                link_traits.insert(l_trait);
            }
        }

        for pin in p.pins.iter() {
            for l in pin.links.iter() {
                let l_trait = format!("Link{}<T>", to_camel(&l.name));
                let l_getter = field_getter(&l.name);

                if !link_traits.contains(&l_trait) {
                    try!(writeln!(out, "pub trait {} {{", l_trait));
                    try!(writeln!(out, "    fn {}(&self) -> T;", l_getter));
                    try!(writeln!(out, "}}"));
                    try!(writeln!(out, ""));
                    link_traits.insert(l_trait);
                }                
            }
        }
    }

    for p in pg.peripherals.iter() {
        let p_type = to_camel(&p.name);

        // Generate Peripheral Links

        for l in p.links.iter() {
            let l_trait = format!("Link{}", to_camel(&l.name));
            let l_getter = field_getter(&l.name);
            let pg_mod = l.peripheral_group.to_lowercase();
            let l_type = format!("super::{}::{} ", pg_mod, to_camel(&l.peripheral));
            let p_const = l.peripheral.to_uppercase();            
                  
            try!(writeln!(out, "impl {}<{}> for {} {{", l_trait, l_type, p_type));
            try!(writeln!(out, "    #[inline] fn {}(&self) -> {} {{ super::{}::{} }}", l_getter, l_type, pg_mod, p_const));
            try!(writeln!(out, "}}"));
            try!(writeln!(out, ""));
        }

        // Generate Pin Links

        for pin in p.pins.iter() {
            let pin_type = to_camel(&pin.name);
            
            for l in pin.links.iter() {
                let l_trait = format!("Link{}", to_camel(&l.name));
                let l_getter = field_getter(&l.name);
                let pg_mod = l.peripheral_group.to_lowercase();
                let l_type = format!("super::{}::{} ", pg_mod, to_camel(&l.pin));
                let pin_const = l.pin.to_uppercase();            
                    
                try!(writeln!(out, "impl {}<{}> for {} {{", l_trait, l_type, pin_type));
                try!(writeln!(out, "    #[inline] fn {}(&self) -> {} {{ super::{}::{} }}", l_getter, l_type, pg_mod, pin_const));
                try!(writeln!(out, "}}"));
                try!(writeln!(out, ""));
            }            
        }


        // Generate Signals

        for s in p.signals.iter() {
            let s_type = to_camel(&s.name);

            for st in s.types.iter() {
                let st_type = to_camel(&st);
                if !signal_types.contains(&st_type) {
                    signal_types.insert(st_type.clone());
                }
                let key = s_type.clone();
                let value = (to_camel(&p.name), st_type.clone());
                signals.insert(key, value);
            }
        }

        for ch in p.channels.iter() {
            let ch_type = to_camel(&ch.name);

            for s in ch.signals.iter() {
                let s_type = to_camel(&s.name);
                for st in s.types.iter() {
                    let st_type = to_camel(&st);
                    if !signal_types.contains(&st_type) {
                        signal_types.insert(st_type.clone());
                    }                 
                    let key = s_type.clone();
                    let value = (ch_type.clone(), st_type.clone());
                    signals.insert(key, value);
                }
            }            
        }
    }    

    // Generate Peripheral Group Channels

    for p in pg.peripherals.iter() {
        let p_name = p.name.to_uppercase();
        let p_type = to_camel(&p.name);

        for ch in p.channels.iter() {
            let id = ch.name.to_uppercase();                
            let ty = to_camel(&ch.name);
            let meth = ch.name.to_lowercase();
            let base_name = format!("{}_CH", id);
            let base_type = format!("{}Ch", to_camel(&pg.name));
            let base_periph = format!("{}_PERIPH", p_name);
            let owned = format!("{}_OWNED", id);
            let ref_count = format!("{}_REF_COUNT", id);
            
            try!(writeln!(out, "::bobbin_mcu::channel!({id}, {ty}, {meth}, {port_id}, {port_type}, {base_id}, {base_type}, {base_periph}, {owned}, {ref_count}, {index});",
                id=id,
                ty=ty,
                meth=meth,
                port_id=p_name,
                port_type=p_type,
                base_id=base_name,
                base_type=base_type,
                base_periph=base_periph,
                owned=owned,
                ref_count=ref_count,
                index=ch.index.unwrap(),
            ));            
        }        
    }

    // Generate Peripheral Group Clocks

    for p in pg.peripherals.iter() {
        gen_peripheral_clocks(cfg, out, d, p)?;
    }


    Ok(())
}
pub fn gen_peripheral_group_impl<W: Write>(_cfg: &Config, out: &mut W, pg: &PeripheralGroup) -> Result<()> {
    let pg_name = if let Some(ref prototype) = pg.prototype {
        if let Some(ref name) = prototype.group_name {
            format!("{}", name)
        } else {
            format!("{}", pg.name)
        }
    } else {
        format!("{}", pg.name)
    };
    let pg_type = format!("{}Periph", to_camel(&pg_name));
    let ch_type = format!("{}Ch", to_camel(&pg_name));


    // try!(writeln!(out, "#[allow(unused_imports)] use {}::*;", cfg.common));
    // try!(writeln!(out, ""));

    // Generate Periphal Group Impl

    if pg.modules.len() == 0 {
        try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));
        try!(gen_doc(out, 0, &format!("{} Peripheral", pg.name.to_uppercase())));
        try!(writeln!(out, "pub struct {}(pub usize); ", pg_type));
        try!(writeln!(out, ""));        

        let mut pin_count = 0;
        for p in pg.peripherals.iter() {
            pin_count += p.pins.len();
        }

        if pg.has_pins || pin_count > 0 {
            let pin_type = format!("{}Pin", to_camel(&pg_name));
            try!(writeln!(out, "pub struct {} {{ pub port: {}, pub index: usize }}", pin_type, pg_type));
            try!(writeln!(out, ""));
        }

        if pg.has_channels {
            try!(writeln!(out, "pub struct {} {{ pub periph: {}, pub index: usize }}", ch_type, pg_type));
            try!(writeln!(out, ""));
        }        
    }



    let p0 = if let Some(ref p0) = pg.prototype {
        p0
    } else {
        &pg.peripherals[0]
    };

    if p0.registers.len() > 0 {
        try!(gen_registers(out, &pg_type, &p0.registers[..], p0.size.or(Some(32)), p0.access.or(Some(Access::ReadWrite))));
    }
    if p0.clusters.len() > 0 {
        try!(gen_clusters(out, &pg_type, &p0.clusters[..], p0.size.or(Some(32)), p0.access.or(Some(Access::ReadWrite))));
    }

    if p0.descriptors.len() > 0 {
        for desc in p0.descriptors.iter() {
            try!(gen_descriptor(out, &pg_type, &desc));
        }
    }

    Ok(())
}

pub fn gen_peripheral<W: Write>(cfg: &Config, out: &mut W, d: &Device, p: &Peripheral, ord: usize) -> Result<()> {
    let p_const = format!("{}_PERIPH", p.name);
    let p_owned = format!("{}_OWNED", p.name);
    let p_ref_count = format!("{}_REF_COUNT", p.name);
    let p_type = to_camel(&p.group_name.as_ref().unwrap());
    let pg_type = format!("{}Periph", to_camel(&p.group_name.as_ref().unwrap()));

    
    // try!(writeln!(out, "#[allow(unused_imports)] use {}::*;", cfg.common));
    // try!(writeln!(out, ""));

    if p.modules.len() > 0 {
        for m in p.modules.iter() {
            if let Some(ref use_as) = m._as {
                try!(writeln!(out, "pub use {} as {};", m.name, use_as));
            } else {
                try!(writeln!(out, "pub use {};", m.name));
            }
        }
    } 
    try!(writeln!(out, ""));

    if let Some(_) = p.dim {
        unimplemented!()
    } else {
        try!(writeln!(out, "::bobbin_mcu::periph!( {p_name}, {p_type}, {p_const}, {pg_type}, {p_owned}, {p_ref_count}, 0x{p_addr:08x}, 0x{p_index:02x}, 0x{p_ord:02x});", 
            p_const=p_const, pg_type=pg_type, p_name=p.name, p_type=p_type, p_owned=p_owned, p_ref_count=p_ref_count, p_addr=p.address, p_index=0, p_ord=ord));
        try!(writeln!(out, ""));
    }
    
    // // Generate Links

    // let gen_link_traits = false;

    // if gen_link_traits {
    //     let p_size = size_type(p.size.or(Some(32)).unwrap());
    //     for link in p.links.iter() {
    //         try!(writeln!(out, "pub trait {} {{", to_camel(&link.name)));
    //         try!(writeln!(out, "    fn {}(&self) -> {};", field_getter(&link.name), p_size));
    //         try!(writeln!(out, "    fn {}(&self, value: {});", field_setter(&link.name), p_size));
    //         try!(writeln!(out, "}}"));
    //         try!(writeln!(out, ""));
    //         try!(writeln!(out, "impl {} {{", to_camel(&p.name)));
    //         try!(writeln!(out, "    #[inline] pub fn {}<P: {}>(&self, p: &P) -> {} {{", field_getter(&link.name), to_camel(&link.name), p_size));
    //         try!(writeln!(out, "        p.{}()", field_getter(&link.name)));
    //         try!(writeln!(out, "    }}"));
    //         try!(writeln!(out, "    #[inline] pub fn {}<P: {}>(&self, p: &P, value: {}) {{", field_setter(&link.name), to_camel(&link.name), p_size));
    //         try!(writeln!(out, "        p.{}(value)", field_setter(&link.name)));
    //         try!(writeln!(out, "    }}"));
    //         try!(writeln!(out, "}}"));
    //         try!(writeln!(out, ""));
    //     }
    // }

    // {
    //     // Generate Links

    //     for r in p.registers.iter() {
    //         for f in r.fields.iter() {
    //             let f_width = f.bit_width;
    //             let field_type = format!("::bobbin_bits::U{}", f_width);
    //             for link in f.links.iter() {
    //                 // FIXME: Should not be using peripheral size as return value
    //                 let pg_mod = link.peripheral_group.to_lowercase();
    //                 let l_type = format!("super::{}::{}", pg_mod, to_camel(&link.peripheral));
    //                 try!(writeln!(out, "impl {} for {} {{", to_camel(&link.name), l_type));
    //                 try!(writeln!(out, "    #[inline] fn {}(&self) -> {} {{ {}.{}().{}() }}", field_getter(&link.name), field_type, p.name, r.name.to_lowercase(), field_getter(&f.name)));                
    //                 try!(writeln!(out, "    #[inline] fn {}<V: Into<{}>>(&self, value: V) {{ {}.{}(|r| r.{}(value)); }}", field_setter(&link.name),  field_type, p.name, field_with(&r.name), field_setter(&f.name)));
    //                 try!(writeln!(out, "}}"));
    //                 try!(writeln!(out, ""));
    //             }
    //         }
    //     }
    // }
    // Generate Signals

    for s in p.signals.iter() {
        let s_type = to_camel(&s.name);
        try!(writeln!(out, "impl super::sig::Signal<super::sig::{}> for {} {{}}", s_type, p_type));
        for st in s.types.iter() {
            let st_type = to_camel(&st);
            try!(writeln!(out, "impl super::sig::Signal{}<super::sig::{}> for {} {{}}", st_type, s_type, p_type));
        }
    }

    // Generate Channels

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
    gen_peripheral_clocks(cfg, out, d, p)?;
    
    Ok(())    
}

pub fn gen_peripheral_impl<W: Write>(_cfg: &Config, out: &mut W, p: &Peripheral) -> Result<()> {
    let p_type = format!("{}Periph", to_camel(&p.group_name.as_ref().unwrap()));
    let ch_type = format!("{}Ch", to_camel(&p.group_name.as_ref().unwrap()));


    // TODO: Description must be added at top of file
    // if let Some(ref desc) = p.description {
    //     let desc = desc.trim();
    //     if desc.len() > 0 {
    //         try!(writeln!(out, "// //! {}", desc));
    //         try!(writeln!(out, ""));
    //     }
    // }

    // try!(writeln!(out, "#[allow(unused_imports)] use {}::*;", cfg.common));
    // try!(writeln!(out, ""));

    if let Some(ref desc) = p.description {
        try!(gen_doc(out, 0, desc));
    }
    
    try!(writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq)]"));    
    try!(writeln!(out, "pub struct {}(pub usize);", p_type));    
    if p.channels.len() > 0 {
        try!(writeln!(out, "pub struct {} {{ pub periph: {}, pub index: usize);", ch_type, p_type));
    }

    if p.registers.len() > 0 {
        try!(gen_registers(out, &p_type, &p.registers[..], p.size.or(Some(32)), p.access.or(Some(Access::ReadWrite))));
    }

    if p.clusters.len() > 0 {
        try!(gen_clusters(out, &p_type, &p.clusters[..], p.size.or(Some(32)), p.access.or(Some(Access::ReadWrite))));
    }

    Ok(())
}

pub fn gen_clusters<W: Write>(out: &mut W, p_type: &str, clusters: &[Cluster], size: Option<u64>, access: Option<Access>) -> Result<()> {
    try!(writeln!(out, "impl {} {{", p_type));

    for c in clusters.iter() {
        let c_type = format!("{}", to_camel(&c.name));
        let mod_name = c.name.to_lowercase();
        if let Some(ref desc) = c.description {
            try!(gen_doc(out, 4, &format!("Get {} Peripheral", desc)));
        }               
        try!(writeln!(out, "    #[inline] pub fn {}(&self) -> {}::{} {{", mod_name, mod_name, c_type));
        try!(writeln!(out, "        {}::{}(self.0 + 0x{:x})", mod_name, c_type, c.offset));
        try!(writeln!(out, "    }}"));
    }
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));

    // Find the right place to generate the main periph trait if not already created

    for c in clusters.iter() {        
        let c_type = to_camel(&c.name);
        let mod_name = c.name.to_lowercase();        
        if let Some(ref desc) = c.description {
            try!(gen_doc(out, 0, &format!("{} Cluster", desc)));
        }                
        try!(writeln!(out, "pub mod {} {{", mod_name));
        // try!(writeln!(out, "    #[allow(unused_imports)] use {}::*;", cfg.common));
        try!(writeln!(out, "    #[allow(unused_imports)] use super::*;"));
        
        try!(writeln!(out, "    #[derive(Clone, Copy, PartialEq, Eq)]"));
        if let Some(ref desc) = c.description {
            try!(gen_doc(out, 4, &format!("{} Peripheral", desc)));
        }                
        try!(writeln!(out, "    pub struct {}(pub usize);", c_type));
        try!(gen_registers(out, &c_type, &c.registers[..], c.size.or(size), c.access.or(access)));
        try!(writeln!(out, "}}"));
        try!(writeln!(out, "// End of {}", mod_name));
        try!(writeln!(out, ""));
    }

    Ok(())
}

pub fn gen_descriptor<W: Write>(out: &mut W, _p_type: &str, desc: &Descriptor) -> Result<()> {
    let d_type = to_camel(&desc.name);
    let d_size = desc.size.expect("Descriptor size is required");

    try!(writeln!(out, ""));

    if let Some(ref desc) = desc.description {
        try!(gen_doc(out, 0, desc));
    }
    try!(writeln!(out, "#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]"));
    try!(writeln!(out, "pub struct {}(pub [u8; {}]);", d_type, d_size));
    try!(writeln!(out, ""));

    try!(writeln!(out, "impl {} {{", d_type));


    for r in desc.registers.iter() {
        let r_type = format!("{}", to_camel(&r.name));
        // let r_ptr = field_ptr(&r.name);
        // let r_mut = field_mut(&r.name);
        let r_getter = field_getter(&r.name);
        let r_setter = field_setter(&r.name);
        let r_with = field_with(&r.name);
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
                1...32 => format!("::bobbin_bits::R{}", dim),
                64 => format!("::bobbin_bits::U6"),
                128 => format!("::bobbin_bits::U7"),
                256 => format!("::bobbin_bits::U8"),
                _ => panic!("Unsupported dim value for {}: {}", r.name, dim),
            };


            if r_access.is_readable() {
                try!(gen_doc(out, 0, &format!("Read the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> {} {{ ", r_getter, i_type, r_type));
                try!(writeln!(out, "      let index: usize = index.into().value() as usize;"));
                try!(writeln!(out, "      unsafe {{"));
                try!(writeln!(out, "          ::core::ptr::read_volatile(self.0.as_ptr().offset(0x{:x} + {}) as *const {})", r_offset, r_shift, r_type));
                try!(writeln!(out, "      }}"));
                try!(writeln!(out, "  }}"));
                try!(writeln!(out, ""));
            }
            if r_access.is_writable() {
                try!(gen_doc(out, 0, &format!("Set the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}>, {}: FnOnce({}) -> {}>(&self, index: I, f: {}) -> &mut Self {{", r_setter, i_type, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "      let index: usize = index.into().value() as usize;"));
                try!(writeln!(out, "      unsafe {{"));
                try!(writeln!(out, "          ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x{:x} + {}) as *mut {}, f({}(0)));", r_offset, r_shift, r_type, r_type)); 
                try!(writeln!(out, "      }}"));
                try!(writeln!(out, "      self"));
                try!(writeln!(out, "  }}"));
                try!(writeln!(out, ""));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(gen_doc(out, 0, &format!("Modify the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "  #[inline] pub fn {}<I: Into<{}> + Copy, {}: FnOnce({}) -> {}>(&mut self, index: usize, f: {}) -> &mut Self {{", r_with, i_type, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "      let index: usize = index.into().value() as usize;"));                                
                try!(writeln!(out, "      unsafe {{"));
                try!(writeln!(out, "          ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x{:x} + {}) as *mut {}, f(self.{}(index)));", r_offset, r_shift, r_type, r_getter)); 
                try!(writeln!(out, "      }}"));
                try!(writeln!(out, "      self"));
                try!(writeln!(out, "  }}"));            
                try!(writeln!(out, ""));
            }            
        } else {
            // try!(writeln!(out, "  #[inline] pub fn {}(&self) -> *const {} {{ ", r_ptr, r_size));
            // try!(writeln!(out, "      (&self.0 as *const {}).offset({})", r_size, r_offset));
            // try!(writeln!(out, "  }}"));
            // try!(writeln!(out, "  #[inline] pub fn {}(&mut self) -> *mut {} {{ ", r_mut, r_size));
            // try!(writeln!(out, "      (&mut self.0 as *mut {}).offset({})", r_size, r_offset));
            // try!(writeln!(out, "  }}"));
            
            if r_access.is_readable() {
                try!(gen_doc(out, 0, &format!("Read the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}(&self) -> {} {{ ", r_getter, r_type));
                try!(writeln!(out, "        unsafe {{"));
                try!(writeln!(out, "            ::core::ptr::read_volatile(self.0.as_ptr().offset(0x{:x}) as *const {})", r_offset, r_type));
                try!(writeln!(out, "        }}"));
                try!(writeln!(out, "    }}"));
                try!(writeln!(out, ""));
            }
            if r_access.is_writable() {
                try!(gen_doc(out, 0, &format!("Write the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}<{}: FnOnce({}) -> {}>(&mut self, f: {}) -> &mut Self {{", r_setter, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "        unsafe {{"));
                try!(writeln!(out, "            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x{:x}) as *mut {}, f({}(0)));", r_offset, r_type, r_type));                    
                try!(writeln!(out, "        }}"));
                try!(writeln!(out, "        self"));
                try!(writeln!(out, "  }}"));
                try!(writeln!(out, ""));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(gen_doc(out, 0, &format!("Modfy the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}<{}: FnOnce({}) -> {}>(&mut self, f: {}) -> &mut Self {{", r_with, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "        unsafe {{"));
                try!(writeln!(out, "            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x{:x}) as *mut {}, f(self.{}()));", r_offset, r_type, r_getter));                    
                try!(writeln!(out, "        }}"));
                try!(writeln!(out, "      self"));
                try!(writeln!(out, "    }}"));            
                try!(writeln!(out, ""));
            }
        }
        try!(writeln!(out, ""));        
    }
    try!(writeln!(out, "}}"));

    try!(gen_register_types(out, &desc.registers, None, Some(Access::ReadWrite)));

    Ok(())
}

pub fn gen_registers<W: Write>(out: &mut W, p_type: &str, regs: &[Register], size: Option<u64>, access: Option<Access>) -> Result<()> {
    try!(gen_register_methods(out, p_type, regs, size, access));
    try!(gen_register_types(out, regs, size, access));
    Ok(())
}

pub fn gen_register_methods<W: Write>(out: &mut W, p_type: &str, regs: &[Register], _size: Option<u64>, access: Option<Access>) -> Result<()> {
    try!(writeln!(out, "impl {} {{", p_type));        

    for r in regs.iter() {  
        let r_type = format!("{}", to_camel(&r.name));
        let r_reg = field_reg(&r.name);
        let r_ptr = field_ptr(&r.name);
        let r_mut = field_mut(&r.name);
        let r_getter = field_getter(&r.name);
        let r_setter = field_setter(&r.name);
        let r_writer = field_writer(&r.name);
        let r_with = field_with(&r.name);
        let r_access = r.access.or(access).unwrap();
        let r_offset = r.offset;        

        let r_typevar = if r.name.to_lowercase() == "f" {
            "_F"
        } else {
            "F"
        };

        if let Some(dim) = r.dim {
            let r_incr = r.dim_increment.unwrap();
            // let r_shift = match r_incr {
            //     1 => format!("(index)"),
            //     2 => format!("(index << 1)"),
            //     4 => format!("(index << 2)"),
            //     8 => format!("(index << 3)"),
            //     16 => format!("(index << 4)"),
            //     _ => format!("(index * {})", r_incr),
            // };  
            let i_type = match dim {
                1...32 => format!("::bobbin_bits::R{}", dim),
                64 => format!("::bobbin_bits::U6"),
                128 => format!("::bobbin_bits::U7"),
                256 => format!("::bobbin_bits::U8"),
                _ => panic!("Unsupported dim value for {}: {}", r.name, dim),
            };

            try!(gen_doc(out, 4, &format!("Get the {} Register.", r.name.to_uppercase())));
            try!(writeln!(out, "    #[inline] pub fn {}(&self) -> ::bobbin_mcu::register::RegisterArray<{}, {}> {{ ", r_reg, r_type, i_type));
            try!(writeln!(out, "        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut {}, 0x{:x}, 0x{:x})", r_type, r_offset, r_incr));
            try!(writeln!(out, "    }}"));
            try!(writeln!(out, ""));


            try!(gen_doc(out, 4, &format!("Get the *mut pointer for the {} register.", r.name.to_uppercase())));
            try!(writeln!(out, "    #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> *mut {} {{ ", r_mut, i_type, r_type));
            try!(writeln!(out, "        self.{}().ptr(index.into())", r_reg));
            try!(writeln!(out, "    }}"));
            try!(writeln!(out, ""));

            try!(gen_doc(out, 4, &format!("Get the *const pointer for the {} register.", r.name.to_uppercase())));
            try!(writeln!(out, "    #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> *const {} {{ ", r_ptr, i_type, r_type));
            try!(writeln!(out, "        self.{}().ptr(index.into())", r_reg));
            try!(writeln!(out, "    }}"));
            try!(writeln!(out, ""));

            if r_access.is_readable() {
                try!(gen_doc(out, 4, &format!("Read the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> {} {{ ", r_getter, i_type, r_type));
                try!(writeln!(out, "        self.{}().read(index.into())", r_reg));
                try!(writeln!(out, "    }}"));
                try!(writeln!(out, ""));
            }
            if r_access.is_writable() {
                try!(gen_doc(out, 4, &format!("Write the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}<I: Into<{}>>(&self, index: I, value: {}) -> &Self {{", r_writer, i_type, r_type));
                try!(writeln!(out, "        self.{}().write(index.into(), value);", r_reg));
                try!(writeln!(out, "        self"));
                try!(writeln!(out, "    }}"));
                try!(writeln!(out, ""));                
                try!(gen_doc(out, 4, &format!("Set the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}<I: Into<{}>, {}: FnOnce({}) -> {}>(&self, index: I, f: {}) -> &Self {{", r_setter, i_type, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "        self.{}().set(index.into(), f);", r_reg));
                try!(writeln!(out, "        self"));
                try!(writeln!(out, "    }}"));
                try!(writeln!(out, ""));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(gen_doc(out, 4, &format!("Modify the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}<I: Into<{}> + Copy, {}: FnOnce({}) -> {}>(&self, index: I, f: {}) -> &Self {{", r_with, i_type, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "        self.{}().with(index.into(), f);", r_reg));
                try!(writeln!(out, "        self"));
                try!(writeln!(out, "    }}"));       
                try!(writeln!(out, "")); 
            }            
        } else {
            try!(gen_doc(out, 4, &format!("Get the {} Register.", r.name.to_uppercase())));
            try!(writeln!(out, "    #[inline] pub fn {}(&self) -> ::bobbin_mcu::register::Register<{}> {{ ", r_reg, r_type));
            try!(writeln!(out, "        ::bobbin_mcu::register::Register::new(self.0 as *mut {}, 0x{:x})",  r_type, r_offset));
            try!(writeln!(out, "    }}"));
            try!(writeln!(out, ""));
            
            try!(gen_doc(out, 4, &format!("Get the *mut pointer for the {} register.", r.name.to_uppercase())));
            try!(writeln!(out, "    #[inline] pub fn {}(&self) -> *mut {} {{ ", r_mut, r_type));
            try!(writeln!(out, "        self.{}().ptr()", r_reg));
            try!(writeln!(out, "    }}"));
            try!(writeln!(out, ""));
            
            try!(gen_doc(out, 4, &format!("Get the *const pointer for the {} register.", r.name.to_uppercase())));
            try!(writeln!(out, "    #[inline] pub fn {}(&self) -> *const {} {{ ", r_ptr, r_type));
            try!(writeln!(out, "        self.{}().ptr()", r_reg));
            try!(writeln!(out, "    }}"));
            try!(writeln!(out, "")); 

            if r_access.is_readable() {
                try!(gen_doc(out, 4, &format!("Read the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}(&self) -> {} {{ ", r_getter, r_type));
                try!(writeln!(out, "        self.{}().read()", r_reg));
                try!(writeln!(out, "    }}"));
                try!(writeln!(out, ""));
            }
            if r_access.is_writable() {
                try!(gen_doc(out, 4, &format!("Write the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}(&self, value: {}) -> &Self {{ ", r_writer, r_type));
                try!(writeln!(out, "        self.{}().write(value);", r_reg));
                try!(writeln!(out, "        self"));
                try!(writeln!(out, "    }}"));
                try!(writeln!(out, ""));

                try!(gen_doc(out, 4, &format!("Set the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}<{}: FnOnce({}) -> {}>(&self, f: {}) -> &Self {{", r_setter, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "        self.{}().set(f);", r_reg));
                try!(writeln!(out, "        self"));
                try!(writeln!(out, "    }}"));                
                try!(writeln!(out, ""));
            }
            if r_access.is_readable() && r_access.is_writable() {
                try!(gen_doc(out, 4, &format!("Modify the {} register.", r.name.to_uppercase())));
                try!(writeln!(out, "    #[inline] pub fn {}<{}: FnOnce({}) -> {}>(&self, f: {}) -> &Self {{", r_with, r_typevar, r_type, r_type, r_typevar));
                try!(writeln!(out, "        self.{}().with(f);", r_reg));
                try!(writeln!(out, "        self"));
                try!(writeln!(out, "    }}"));               
                try!(writeln!(out, ""));
            }
        }
    }    
    try!(writeln!(out, "}}"));
    try!(writeln!(out, ""));
    Ok(())
}

pub fn gen_register_types<W: Write>(out: &mut W, regs: &[Register], size: Option<u64>, access: Option<Access>) -> Result<()> {
    for r in regs.iter() {  
        let r_type = format!("{}", to_camel(&r.name));
        let r_size = size_type(r.size.or(size).unwrap());

        if let Some(ref desc) = r.description {
            try!(gen_doc(out, 0, desc));
        }        
        try!(writeln!(out, "#[derive(Default, Clone, Copy, PartialEq, Eq)]"));
        try!(writeln!(out, "pub struct {}(pub {});", r_type, r_size));
        try!(writeln!(out, "impl {} {{", r_type));        
        for f in r.fields.iter() {
            try!(gen_field(out, &f, &r_size, f.access.or(access)))
        }
        try!(writeln!(out, "}}"));
        try!(writeln!(out, ""));

        try!(writeln!(out, "impl From<{}> for {} {{", r_size, r_type));
        try!(writeln!(out, "    #[inline]"));
        try!(writeln!(out, "    fn from(other: {}) -> Self {{", r_size));
        try!(writeln!(out, "         {}(other)", r_type));
        try!(writeln!(out, "    }}"));
        try!(writeln!(out, "}}"));        
        try!(writeln!(out, ""));

        try!(writeln!(out, "impl ::core::fmt::Display for {} {{", r_type));
        try!(writeln!(out, "    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{"));
        try!(writeln!(out, "         self.0.fmt(f)"));
        try!(writeln!(out, "    }}"));
        try!(writeln!(out, "}}"));        
        try!(writeln!(out, ""));

        try!(writeln!(out, "impl ::core::fmt::Debug for {} {{", r_type));
        try!(writeln!(out, "    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{"));        
        try!(writeln!(out, "        try!(write!(f, \"[0x{{:08x}}\", self.0));"));
        for f in r.fields.iter() {
            let f_name = field_name(&f.name);
            let f_getter = field_getter(&f.name);

            if let Some(dim) = f.dim {
                for i in 0..dim {
                    match f.bit_width {
                        1 => {
                            try!(writeln!(out, "        if self.{}({}) != 0 {{ try!(write!(f, \" {}[{}]\"))}}", f_getter, i, f_getter, i));
                        },
                        32 => {},
                        _ => {
                            try!(writeln!(out, "        if self.{}({}) != 0 {{ try!(write!(f, \" {}[{}]=0x{{:x}}\", self.{}({})))}}", f_getter, i, f_name, i, f_getter, i));
                        }
                    }                    
                }
            } else {
                match f.bit_width {
                    1 => {
                        try!(writeln!(out, "        if self.{}() != 0 {{ try!(write!(f, \" {}\"))}}", f_getter, f_getter));
                    },
                    32 => {},
                    _ => {
                        try!(writeln!(out, "        if self.{}() != 0 {{ try!(write!(f, \" {}=0x{{:x}}\", self.{}()))}}", f_getter, f_name, f_getter));
                    }
                }
            }
            
        }
        try!(writeln!(out, "        try!(write!(f, \"]\"));"));
        try!(writeln!(out, "        Ok(())"));
        try!(writeln!(out, "    }}"));
        try!(writeln!(out, "}}"));        
        try!(writeln!(out, ""));

    }
    Ok(())
}

pub fn gen_field<W: Write>(out: &mut W, f: &Field, size: &str, _access: Option<Access>) -> Result<()> {
    let f_getter = field_getter(&f.name);
    let f_setter = field_setter(&f.name);
    let f_tester = field_test(&f.name);
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
    let field_type = format!("::bobbin_bits::U{}", f_width);

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
        let i_type = format!("::bobbin_bits::R{}", dim);

        if let Some(ref desc) = f.description {
            try!(gen_doc(out, 4, desc));
        } else {
            try!(gen_doc(out, 4, &format!("Gets the {} field.", f.name)));            
        }
        try!(writeln!(out, "    #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> {} {{", f_getter, i_type, field_type));
        try!(writeln!(out, "        let index: usize = index.into().value() as usize;"));
        match f_incr {
            1 => {
                try!(writeln!(out, "        let shift: usize = {} + index;", f_offset));
            },
            2 => {
                try!(writeln!(out, "        let shift: usize = {} + (index << 1);", f_offset));
            },
            4 => {
                try!(writeln!(out, "        let shift: usize = {} + (index << 2);", f_offset));
            },
            8 => {
                try!(writeln!(out, "        let shift: usize = {} + (index << 3);", f_offset));
            },
            _ => {
                try!(writeln!(out, "        let shift: usize = {} + (index * {});", f_offset, f_incr));                
            }
        }
        try!(writeln!(out, "        unsafe {{ ::core::mem::transmute(((self.0 >> shift) & 0x{:x}) as {}) }} // {}", f_mask, min_size, f_bits));
        try!(writeln!(out, "    }}"));    
        try!(writeln!(out, ""));    

        try!(gen_doc(out, 4, &format!("Returns true if {} != 0", f.name)));
        try!(writeln!(out, "    #[inline] pub fn {}<I: Into<{}>>(&self, index: I) -> bool{{", f_tester, i_type));
        try!(writeln!(out, "        self.{}(index) != 0", f_getter));    
        try!(writeln!(out, "    }}"));    
        try!(writeln!(out, ""));    

        try!(gen_doc(out, 4, &format!("Sets the {} field.", f.name)));
        try!(writeln!(out, "    #[inline] pub fn {}<I: Into<{}>, V: Into<{}>>(mut self, index: I, value: V) -> Self {{", f_setter, i_type, field_type));
        try!(writeln!(out, "        let index: usize = index.into().value() as usize;"));            
        try!(writeln!(out, "        let value: {} = value.into();", field_type));            
        try!(writeln!(out, "        let value: {} = value.into();", size));
        match f_incr {
            1 => {
                try!(writeln!(out, "        let shift: usize = {} + index;", f_offset));
            },
            2 => {
                try!(writeln!(out, "        let shift: usize = {} + (index << 1);", f_offset));
            },
            4 => {
                try!(writeln!(out, "        let shift: usize = {} + (index << 2);", f_offset));
            },
            8 => {
                try!(writeln!(out, "        let shift: usize = {} + (index << 3);", f_offset));
            },
            _ => {
                try!(writeln!(out, "        let shift: usize = {} + (index * {});", f_offset, f_incr));                
            }
        }        
        try!(writeln!(out, "        self.0 &= !(0x{:x} << shift);", f_mask));
        try!(writeln!(out, "        self.0 |= value << shift;"));
        try!(writeln!(out, "        self"));
        try!(writeln!(out, "    }}"));    
        try!(writeln!(out, ""));        
    } else {
        // Field Getter

        if let Some(ref desc) = f.description {
            try!(gen_doc(out, 4, desc));
        } else {
            try!(gen_doc(out, 4, &format!("Gets the {} field.", f.name)));            
        }
        try!(writeln!(out, "    #[inline] pub fn {}(&self) -> {} {{", f_getter, field_type));
        try!(writeln!(out, "        unsafe {{ ::core::mem::transmute(((self.0 >> {}) & 0x{:x}) as {}) }} // {}", f_offset, f_mask, min_size, f_bits));
        try!(writeln!(out, "    }}"));    
        try!(writeln!(out, ""));    

        // Field Tester

        try!(gen_doc(out, 4, &format!("Returns true if {} != 0", f.name)));
        try!(writeln!(out, "    #[inline] pub fn {}(&self) -> bool {{", f_tester));
        try!(writeln!(out, "        self.{}() != 0", f_getter));
        try!(writeln!(out, "    }}"));    
        try!(writeln!(out, ""));    

        // Field Setter

        try!(gen_doc(out, 4, &format!("Sets the {} field.", f.name)));
        try!(writeln!(out, "    #[inline] pub fn {}<V: Into<{}>>(mut self, value: V) -> Self {{", f_setter, field_type));
        try!(writeln!(out, "        let value: {} = value.into();", field_type));
        try!(writeln!(out, "        let value: {} = value.into();", size));
        try!(writeln!(out, "        self.0 &= !(0x{:x} << {});", f_mask, f_offset));
        try!(writeln!(out, "        self.0 |= value << {};", f_offset));
        try!(writeln!(out, "        self"));
        try!(writeln!(out, "    }}"));    
        try!(writeln!(out, ""));
            
    }


    Ok(())
}

pub fn gen_clocks<W: Write>(_cfg: &Config, out: &mut W, d: &Device, _path: &Path) -> Result<()> {
    // pub trait ClockTree { ... }

    let clocks = if let Some(ref clocks) = d.clocks {
        clocks
    } else {
        return Ok(())
    };
    // writeln!(out, "pub use ::bobbin_common::*;")?;
    writeln!(out, "use ::bobbin_mcu::clock::{{Clock, ClockFor}};")?;    
    writeln!(out, "use ::bobbin_mcu::hz::Hz;")?;    
    writeln!(out, "")?;
    writeln!(out, "#[derive(Default)]")?;
    writeln!(out, "pub struct Clocks<CP: ClockProvider> {{ provider: CP }}")?;
    writeln!(out, "")?;
    writeln!(out, "impl<CP: ClockProvider> ::core::ops::Deref for Clocks<CP> {{")?;
    writeln!(out, "    type Target = CP;")?;
    writeln!(out, "    fn deref(&self) -> &CP {{ &self.provider }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "")?;
    {
        // Define Global Clocks
        writeln!(out, "")?;
        writeln!(out, "// Define Global Clocks")?;
        writeln!(out, "")?;

        for clock in &clocks.sources {
            if let Some(speed) = clock.speed {
                // writeln!(out, "pub const {}_HZ: Hz = Hz::from_num({});", clock.const_id(), speed)?;
                writeln!(out, "#[derive(Default)]")?;
                writeln!(out, "pub struct {} {{}}", clock.type_id())?;
                writeln!(out, "impl Clock for {} {{ fn hz() -> Hz {{ Hz::from_num({}) }} }}", clock.type_id(), speed)?;
                writeln!(out, "")?;
            }
        }        

        writeln!(out, "")?;
    }   

    writeln!(out, "pub trait ClockProvider : Default {{")?;
    for clock in &clocks.inputs {
        writeln!(out, "    type {}: Clock;", clock.type_id())?;
    }
    for clock in &clocks.inputs {
        writeln!(out, "    fn {}(&self) -> Hz {{ Self::{}::hz() }}", clock.trait_method(), clock.type_id())?;
    }
    for clock in &clocks.sources {
        if let Some(speed) = clock.speed {
            writeln!(out, "    fn {}(&self) -> Hz {{ Hz::from_num({}) }}", clock.trait_method(), speed)?;
        } else {
            for input in &clock.inputs {
                writeln!(out, "    fn {}(&self) -> Hz {{ self.{}() }}", clock.trait_method(), input.trait_method())?;
            }            
        }
    }
    for clock in &clocks.outputs {
        writeln!(out, "    fn {}(&self) -> Hz {{ unimplemented!() }}", clock.trait_method())?;
    }    
    writeln!(out, "}}")?;
    writeln!(out, "")?;

    {
        for p in &d.peripherals {
            let p_mod = if let Some(ref group_name) = p.group_name {
                group_name.to_lowercase()
            } else {
                panic!("No group name specified for {}", p.name);
            };
            let p_type = to_camel(&p.name);
            for clock in &p.clocks {
                for input in &clock.inputs {
                    if input.name.len() > 0 {
                        // let i_type = to_camel(&input.name);
                        writeln!(out, "impl<CP> ClockFor<::{}::{}> for Clocks<CP> where CP: ClockProvider {{", p_mod, p_type)?;
                        writeln!(out, "    fn clock_for(&self, _: ::{}::{}) -> Hz {{ self.{}() }}", p_mod, p_type, input.trait_method())?;
                        writeln!(out, "}}")?;
                        writeln!(out, "")?;
                    }
                }
            }
        }

        // impl Clock<T> for Peripheral Groups

        for pg in &d.peripheral_groups {
            let p_mod = pg.name.to_lowercase();
            for p in &pg.peripherals {
                let p_type = to_camel(&p.name);
                for clock in &p.clocks {
                    for input in &clock.inputs {
                        if input.name.len() > 0 {
                            writeln!(out, "impl<CP> ClockFor<::{}::{}> for Clocks<CP> where CP: ClockProvider {{", p_mod, p_type)?;
                            writeln!(out, "    fn clock_for(&self, _: ::{}::{}) -> Hz {{ self.{}() }}", p_mod, p_type, input.trait_method())?;
                            writeln!(out, "}}")?;
                            writeln!(out, "")?;
                        }
                    }
                }
            }
        }
    }    
    Ok(())
}

pub fn gen_peripheral_clocks<W: Write>(_cfg: &Config, out: &mut W, d: &Device, p: &Peripheral) -> Result<()> {
    for ref clock in p.clocks.iter() {
        gen_peripheral_clock_gates(_cfg, out, d, p, clock)?;
    }
    Ok(())
}

pub fn gen_peripheral_clock_gates<W: Write>(_cfg: &Config, out: &mut W, d: &Device, p: &Peripheral, c: &Clock) -> Result<()> {
    for ref gate in c.gates.iter() {
        if let &Some(ref gate_type) = &gate.gate_type {
            let p_type = to_camel(&p.name);
            let g_type = format!("::bobbin_mcu::gate::Gate{}", to_camel(gate_type));
            let g_name = format!("gate_{}", gate_type.to_lowercase());


            let p_name = if let Some(ref p_name) = gate.periph {
                p_name.to_uppercase()
            } else {
                panic!("No register name specified for gate {} in {}", gate_type, p.name);
            };
            let r_name = if let Some(ref r_name) = gate.register {
                r_name.to_lowercase()
            } else {
                panic!("No register name specified for gate {} in {}", gate_type, p.name);
            };
            let f_name = if let Some(ref f_name) = gate.field {
                f_name.to_lowercase()
            } else {
                panic!("No field name specified for gate {} in {}", gate_type, p.name);
            };

            let pg_name = if let Some(ref p) = d.get_peripheral(&p_name) {
                if let Some(ref group_name) = p.group_name {
                    format!("::{}", group_name.to_lowercase())
                } else {
                    panic!("No group name specified for peripheral {}", p.name);
                }
            } else {
                panic!("Could not find peripheral {} for gate {} in {}", r_name, gate_type, p.name);
            };



            writeln!(out, "// {:?}", gate)?;
            writeln!(out, "impl {} for {} {{", g_type, p_type)?;
            writeln!(out, "    #[inline]",)?;
            writeln!(out, "    fn {}(&self) -> ::bobbin_bits::U1 {{ {}::{}.{}().{}() }}", g_name, pg_name, p_name, r_name, f_name)?;
            writeln!(out, "    #[inline]",)?;
            writeln!(out, "    fn set_{}<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {{", g_name)?;
            writeln!(out, "        {}::{}.with_{}(|r| r.set_{}(value));",  pg_name, p_name, r_name, f_name)?;
            writeln!(out, "        self")?;            
            writeln!(out, "    }}")?;            
            writeln!(out, "}}")?;            
            writeln!(out, "")?;
        }
    }
    Ok(())
}    
