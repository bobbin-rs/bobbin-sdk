use Device;
use super::modules;

use std::io::{Write, Read, Result};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

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

        // Generate Variants

        writeln!(dst, "")?;

        for v in d.variants.iter() {
            writeln!(dst, "{} = []", v.name)?;
        }

    }

    // Copy Xargo.toml from template
    {
        let xargo_toml_src = cfg.cargo_template.join("Xargo.toml");
        if xargo_toml_src.exists() {
            let mut src = File::open(xargo_toml_src)?;
            let mut data = String::new();
            src.read_to_string(&mut data)?;    
            let xargo_toml_dst = cfg.out_path.join("Xargo.toml");
            let mut dst = File::create(xargo_toml_dst)?;
            dst.write(&data.as_bytes())?;
        }
    }

    // Copy .cargo/config
    {
        let ccfg_src = cfg.cargo_template.join(".cargo/config");
        if ccfg_src.exists() {
            let ccfg_dir = cfg.out_path.join(".cargo");
            if !ccfg_dir.exists() {
                fs::create_dir(&ccfg_dir)?;
            }
            let mut src = File::open(ccfg_src)?;
            let mut data = String::new();
            src.read_to_string(&mut data)?;    
            let ccfg_dst = ccfg_dir.join("config");
            let mut dst = File::create(ccfg_dst)?;
            dst.write(&data.as_bytes())?;        
        }
    }

    // Copy build.rs from template
    {
        let build_src = cfg.cargo_template.join("build.rs");
        if build_src.exists() {
            let mut src = File::open(build_src)?;
            let mut data = String::new();
            src.read_to_string(&mut data)?;    
            let build_dst = cfg.out_path.join("build.rs");
            let mut dst = File::create(build_dst)?;
            dst.write(&data.as_bytes())?;
        }
    }

    // Copy Makefile from template
    {
        let make_src = cfg.cargo_template.join("Makefile");
        if make_src.exists() {
            let mut src = File::open(make_src)?;
            let mut data = String::new();
            src.read_to_string(&mut data)?;    
            let make_dst = cfg.out_path.join("Makefile");
            let mut dst = File::create(make_dst)?;
            dst.write(&data.as_bytes())?;
        }
    }

    let src_path = cfg.out_path.join("src");
    if !src_path.exists() {
        fs::create_dir(&src_path)?;
    }


    // Copy src/lib.rs
    {
        let lib_src = cfg.cargo_template.join("src/lib.rs");
        let mut src = File::open(lib_src)?;
        let mut data = String::new();
        src.read_to_string(&mut data)?;    
        let lib_dst = src_path.join("lib.rs");
        let mut out = File::create(lib_dst)?;
        out.write(&data.as_bytes())?;

        let mut wrote_crate = false;
        for c in d.crates.iter() {
            let c_name = c.name.replace("-","_");
            if c_name != "" {
                if let Some(ref use_as) = c._as {
                    try!(writeln!(out, "pub extern crate {} as {};", c_name, use_as));
                } else {
                    try!(writeln!(out, "pub extern crate {};", c_name));
                }                            
            }
            let c_name = if let Some(ref use_as) = c._as {
                use_as
            } else {
                &c_name
            };
            for m in c.modules.iter() {
                let m_name = if c_name != "" { format!("{}::{}", c_name, m.name) } else { format!("{}", m.name) };
                if let Some(ref use_as) = m._as {
                    try!(writeln!(out, "pub use {} as {};", m_name, use_as));
                } else {
                    try!(writeln!(out, "pub use {};", m_name));
                }
            }
            wrote_crate = true;
        }         
        if wrote_crate {
            try!(writeln!(out, ""));
        }

        let cfg = modules::Config { 
            path: src_path.clone(), 
            is_root: false, 
            common: String::from("::bobbin_common"),
        };

        // let periph_path = src_path.join("periph/");
        // if !periph_path.exists() {
        //     fs::create_dir(&periph_path)?;
        // }

        // writeln!(out, "pub mod periph;")?;    
        // let mut periph_out = File::create(periph_path.clone().join("mod.rs"))?;
        // gen_periph_mod(&cfg, &mut periph_out, d, &periph_path)?;

        writeln!(out, "pub mod ext;")?;    

        let ext_path = src_path.join("ext/");
        if !ext_path.exists() {
            fs::create_dir(&ext_path)?;
        }

        let ext_out_path = ext_path.clone().join("mod.rs");
        if !ext_out_path.exists() {
            let _ = File::create(ext_out_path)?;
        }
            // gen_ext_mod(&cfg, &mut ext_out, d, &ext_path)?;

        // writeln!(out, "pub mod mcu;")?;
        // writeln!(out, "")?;        

        // let mcu_path = src_path.join("mcu/");
        // if !mcu_path.exists() {
        //     fs::create_dir(&mcu_path)?;
        // }    
        // let mut mcu_out = File::create(mcu_path.clone().join("mod.rs"))?;
        gen_mcu_mod(&cfg, &mut out, d, &src_path)?;
        // // writeln!(out, "pub use mcu::*;")?;
        writeln!(out, "")?;

    }
    Ok(())
}

pub fn gen_cargo_toml(path: &Path) -> Result<()> {
    let mut out = File::create(path)?;

    writeln!(out, "[package]")?;

    Ok(())
    
}


pub fn gen_periph_mod<W: Write>(cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {
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

    // for c in d.crates.iter() {
    //     // NOTE: crates now need to be imported from crate root
    //     // try!(writeln!(out, "extern crate {};", c.name));
    //     for m in c.modules.iter() {
    //         if let Some(ref use_as) = m._as {
    //             try!(writeln!(out, "pub use {}::{} as {};", c.name, m.name, use_as));
    //         } else {
    //             try!(writeln!(out, "pub use {}::{};", c.name, m.name));
    //         }
    //     }
    //     try!(writeln!(out, ""));
    // }

    // Generate Exceptions
    if d.exceptions.len() > 0 {
        let p_name = "exc";
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(modules::gen_exceptions(cfg, &mut f_mod, &d.exceptions));
    }

    for p in d.peripherals.iter() {
        if p.modules.len() > 0 { continue }
        let p_name = p.group_name.as_ref().unwrap_or(&p.name).to_lowercase();
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(modules::gen_peripheral_impl(cfg, &mut f_mod, p));
    }

    for pg in d.peripheral_groups.iter() {
        if pg.modules.len() > 0 { continue }
        let pg_name = pg.name.to_lowercase();
        try!(writeln!(out, "pub mod {};", pg_name));
        let p_mod = path.join(format!("{}.rs", pg_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(modules::gen_peripheral_group_impl(cfg, &mut f_mod, pg));
    }
    Ok(())
}

pub fn gen_ext_mod<W: Write>(_cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {

    for p in d.peripherals.iter() {
        if p.modules.len() > 0 { continue }        
        let p_name = p.group_name.as_ref().unwrap_or(&p.name).to_lowercase();
        try!(writeln!(out, "pub mod {};", p_name));
        let p_dir = path.join(&p_name);
        let p_mod = path.join(format!("{}.rs", p_name));
        if !p_dir.exists() && !p_mod.exists() {
            let mut f_mod = try!(File::create(p_mod));
            try!(writeln!(f_mod, "pub use periph::{}::*;", p_name));
        }
    }

    for pg in d.peripheral_groups.iter() {
        let pg_name = pg.name.to_lowercase();
        try!(writeln!(out, "pub mod {};", pg_name));
        let p_dir = path.join(&pg_name);
        let p_mod = path.join(format!("{}.rs", pg_name));
        if !p_dir.exists() && !p_mod.exists() {
            let mut f_mod = try!(File::create(p_mod));
            if pg.modules.len() > 0 {
                for m in &pg.modules {
                    if let Some(ref use_as) = m._as {
                        try!(writeln!(f_mod, "pub use {} as {};", m.name, use_as));
                    } else {
                        try!(writeln!(f_mod, "pub use {};", m.name));
                    }
                }
            } else {
                try!(writeln!(f_mod, "pub use periph::{}::*;", pg_name));
            }
        }
    }

    try!(writeln!(out, "pub mod clock;"));
    Ok(())
}

pub fn gen_mcu_mod<W: Write>(cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {
    let mut ord = 0;

    let mcu_type = super::to_camel(&d.name);

    // writeln!(out, "pub use ::bobbin_common::mcu::*;")?;
    // writeln!(out, "use ::bobbin_common::owned::*;")?;
    // writeln!(out, "")?;

    for p in d.peripherals.iter() {
        let p_name = p.group_name.as_ref().unwrap_or(&p.name).to_lowercase();
        writeln!(out, "pub mod {};", p_name)?;
        // writeln!(p_out, "pub use mcu::{};", p_name)?;
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        // writeln!(f_mod, "#[allow(unused_imports)] use ::bobbin_common::*;")?;
        // writeln!(f_mod, "#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;")?;
        // writeln!(f_mod, "pub use ext::{}::*;", p_name)?;
        // writeln!(f_mod, "")?;
        try!(modules::gen_peripheral(cfg, &mut f_mod, d, p, ord));
        try!(modules::gen_peripheral_impl(cfg, &mut f_mod, p));        
        ord += 1;
    }

    for pg in d.peripheral_groups.iter() {
        let pg_name = pg.name.to_lowercase();
        writeln!(out, "pub mod {};", pg_name)?;
        // writeln!(p_out, "pub use mcu::{};", pg_name)?;
        let p_mod = path.join(format!("{}.rs", pg_name));
        let mut f_mod = try!(File::create(p_mod));
        // writeln!(f_mod, "#[allow(unused_imports)] use ::bobbin_common::*;")?;
        // writeln!(f_mod, "#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;")?;
        // writeln!(f_mod, "pub use ext::{}::*;", pg_name)?;
        // writeln!(f_mod, "")?;
        try!(modules::gen_peripheral_group(&cfg, &mut f_mod, d, pg, &mut ord));        
        try!(modules::gen_peripheral_group_impl(cfg, &mut f_mod, pg));
    }

    let signals = gen_signals_mod(&cfg, out, d, path)?;
    gen_pins_mod(&cfg, out, d, path, &signals)?;

    if d.interrupt_count.is_none() {
        return Ok(());
    } 

    // NOTE: Temporarily disabling interrupt module generation 
    //
    // gen_interrupts_mod(&cfg, out, d, path)?; 


    if d.clocks.is_some() {
        let clk_path = path.join("clock.rs");
        writeln!(out, "pub mod clock;")?;
        let mut clk_out = File::create(&clk_path)?;
        gen_clocks_mod(&cfg,  &mut clk_out, d, &clk_path)?;
        writeln!(out, "")?;
    }
    // Generate MCU and peripheral accessors

    // writeln!(p_out, "pub use mcu::{};", mcu_type)?;

    writeln!(out, "")?;   
    writeln!(out, "#[derive(Debug, Default)]")?;
    writeln!(out, "pub struct {} {{}}", mcu_type)?;
    writeln!(out, "")?;   

    writeln!(out, "impl ::bobbin_mcu::mcu::Mcu for {} {{", mcu_type)?;
    writeln!(out, "    fn id(&self) -> &'static str {{ {:?} }}", d.name)?;
    writeln!(out, "}}")?;
    writeln!(out, "")?;

    writeln!(out, "pub type Mcu = {};", mcu_type)?;

    // Don't generate accessors for now        

    // writeln!(out, "impl {} {{", mcu_type)?;
    // for p in d.peripherals.iter() {
    //     let pg_mod = p.group_name.as_ref().unwrap_or(&p.name).to_lowercase();
    //     let p_name = p.name.to_lowercase();
    //     let p_type = super::to_camel(&pg_mod);
    //     // let p_id = p.name.to_uppercase();
    //     writeln!(out, "    pub fn {}(&self) -> Option<Owned<{}::{}>> {{ {}::{}::acquire() }}", p_name, pg_mod, p_type, pg_mod, p_type)?;

    //     for pin in p.pins.iter() {
    //         let pin_meth = pin.name.to_lowercase();
    //         let pin_type = super::to_camel(&pin.name);
    //         writeln!(out, "        pub fn {}(&self) -> Option<Owned<pin::{}>> {{ pin::{}::acquire() }}", pin_meth, pin_type, pin_type)?;
    //     }

    //     for ch in p.channels.iter() {
    //         let ch_meth = ch.name.to_lowercase();
    //         let ch_type = super::to_camel(&ch.name);
    //         writeln!(out, "        pub fn {}(&self) -> Option<Owned<{}::{}>> {{ {}::{}::acquire() }}", ch_meth, pg_mod, ch_type, pg_mod, ch_type)?;
    //     }                        
    // }

    // for pg in d.peripheral_groups.iter() {
    //     let pg_mod = pg.name.to_lowercase();
    //     for p in pg.peripherals.iter() {
    //         let p_name = p.name.to_lowercase();
    //         let p_type = super::to_camel(&p.name);
    //         //let p_id = p.name.to_uppercase();
    //         writeln!(out, "    pub fn {}(&self) -> Option<Owned<{}::{}>> {{ {}::{}::acquire() }}", p_name, pg_mod, p_type, pg_mod, p_type)?;

    //         for pin in p.pins.iter() {
    //             let pin_meth = pin.name.to_lowercase();
    //             let pin_type = super::to_camel(&pin.name);
    //             writeln!(out, "        pub fn {}(&self) -> Option<Owned<pin::{}>> {{ pin::{}::acquire() }}", pin_meth, pin_type, pin_type)?;
    //         }

    //         for ch in p.channels.iter() {
    //             let ch_meth = ch.name.to_lowercase();
    //             let ch_type = super::to_camel(&ch.name);
    //             writeln!(out, "        pub fn {}(&self) -> Option<Owned<{}::{}>> {{ {}::{}::acquire() }}", ch_meth, pg_mod, ch_type, pg_mod, ch_type)?;
    //         }                        
    //     }
    // }

    // writeln!(out, "}}")?;
    // writeln!(out, "")?;
    
    // for p in d.peripherals.iter() {
    //     let pg_mod = p.group_name.as_ref().unwrap_or(&p.name).to_lowercase();
    //     let p_type = super::to_camel(&pg_mod);
    //     let p_id = p.name.to_uppercase();
    //     let p_periph_type = format!("{}Periph", p_type);
    //     let p_periph_id = format!("{}_PERIPH", p_id);
    //     writeln!(out, "impl Get<{}::{}> for {} {{", pg_mod, p_type, mcu_type)?;
    //     writeln!(out, "    fn get(&self) -> {}::{} {{ {}::{} }}", pg_mod, p_type, pg_mod, p_id)?;
    //     writeln!(out, "}}")?;    
    //     writeln!(out, "")?;
    //     writeln!(out, "impl GetPeriph<{}::{}> for {} {{", pg_mod, p_periph_type, mcu_type)?;
    //     writeln!(out, "    fn get_periph(&self) -> {}::{} {{ {}::{} }}", pg_mod, p_periph_type, pg_mod, p_periph_id)?;
    //     writeln!(out, "}}")?;    
    //     writeln!(out, "")?;
    //     writeln!(out, "impl GetPeriphInstance<{}::{}> for {} {{", pg_mod, p_periph_type, mcu_type)?;
    //     writeln!(out, "    fn get_periph_instance(&self, index: usize) -> Option<{}::{}> {{", pg_mod, p_periph_type)?;
    //     writeln!(out, "        match index {{ ")?;
    //     writeln!(out, "            0 => Some({}::{}),", pg_mod, p_periph_id)?;
    //     writeln!(out, "            _ => None,")?;
    //     writeln!(out, "        }}")?;
    //     writeln!(out, "    }}")?;
    //     writeln!(out, "    fn get_periph_instance_count(&self) -> usize {{ 1 }}")?;
    //     writeln!(out, "}}")?;
    //     writeln!(out, "")?;              
    // }

    // for pg in d.peripheral_groups.iter() {
    //     let pg_mod = pg.name.to_lowercase();
    //     let pg_len = pg.peripherals.len();
    //     let p_periph_type = format!("{}Periph", super::to_camel(&pg.name));
    //     for p in pg.peripherals.iter() {
    //         let p_type = super::to_camel(&p.name);
    //         let p_id = p.name.to_uppercase();
    //         let p_periph_id = format!("{}_PERIPH", p_id);            
    //         writeln!(out, "impl Get<{}::{}> for {} {{", pg_mod, p_type, mcu_type)?;
    //         writeln!(out, "    fn get(&self) -> {}::{} {{ {}::{} }}", pg_mod, p_type, pg_mod, p_id)?;            
    //         writeln!(out, "}}")?;    
    //         writeln!(out, "")?;
    //         if pg_len == 1 {
    //             writeln!(out, "impl GetPeriph<{}::{}> for {} {{", pg_mod, p_periph_type, mcu_type)?;
    //             writeln!(out, "    fn get_periph(&self) -> {}::{} {{ {}::{} }}", pg_mod, p_periph_type, pg_mod, p_periph_id)?;
    //             writeln!(out, "}}")?;    
    //             writeln!(out, "")?;                   
    //         }
    //     }
    //     writeln!(out, "impl GetPeriphInstance<{}::{}> for {} {{", pg_mod, p_periph_type, mcu_type)?;
    //     writeln!(out, "    fn get_periph_instance(&self, index: usize) -> Option<{}::{}> {{", pg_mod, p_periph_type)?;
    //     writeln!(out, "        match index {{")?;
    //     for (i, p) in pg.peripherals.iter().enumerate() {
    //         let p_id = p.name.to_uppercase();
    //         let p_periph_id = format!("{}_PERIPH", p_id);
    //         writeln!(out, "            {} => Some({}::{}),", i, pg_mod, p_periph_id)?;
    //     }
    //     writeln!(out, "            _ => None,")?;
    //     writeln!(out, "        }}")?;
    //     writeln!(out, "    }}")?;
    //     writeln!(out, "    fn get_periph_instance_count(&self) -> usize {{ {} }}", pg_len)?;
    //     writeln!(out, "}}")?;    
    //     writeln!(out, "")?;                
    // }


    Ok(())
}

// pub fn gen_mcu_mod<W: Write>(_cfg: &modules::Config, _p_out: &mut W, out: &mut W, _d: &Device, _path: &Path) -> Result<()> {
//     writeln!(out, "pub struct Mcu {{}}")?;
//     writeln!(out, "pub const MCU: Mcu = Mcu {{}};")?;
//     writeln!(out, "")?;
//     Ok(())
// }

pub fn gen_signals_mod<W: Write>(cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<modules::SignalMap> {
    // Generate Signals

    {    
        let p_name = "sig";
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        // try!(writeln!(f_mod, "#[allow(unused_imports)] pub use ::bobbin_common::*;"));
        // try!(writeln!(f_mod, ""));
        modules::gen_signals(cfg, &mut f_mod, &d)
    }    
}


pub fn gen_pins_mod<W: Write>(cfg: &modules::Config, out: &mut W, d: &Device, path: &Path, signals: &modules::SignalMap) -> Result<()> {
    // Generate Pins

    {    
        let p_name = "pin";
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        // try!(writeln!(f_mod, "#[allow(unused_imports)] use ::bobbin_common::*;"));
        // try!(writeln!(f_mod, "pub use ::bobbin_common::pin::*;"));
        // try!(writeln!(f_mod, "pub use ::bobbin_common::gate::*;"));
        // try!(writeln!(f_mod, ""));
        try!(modules::gen_pins(cfg, &mut f_mod, &d, signals));
    }
    
    Ok(())
}

pub fn gen_interrupts_mod<W: Write>(cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {
    // Generate Interrupts

    {
        let interrupt_count = d.interrupt_count();
        let p_name = "irq";
        try!(writeln!(out, "pub mod {};", p_name));
        let p_mod = path.join(format!("{}.rs", p_name));
        let mut f_mod = try!(File::create(p_mod));
        try!(modules::gen_interrupts(&cfg, &mut f_mod, &d, interrupt_count));
    }
    
    Ok(())
}


pub fn gen_clocks_mod<W: Write>(cfg: &modules::Config, out: &mut W, d: &Device, path: &Path) -> Result<()> {
    // let defn_name = "tree_defn";
    // let defn_path = path.join("tree_defn.rs");

    // let impl_name = "tree_impl";
    // let impl_path = path.join("tree_impl.rs");
    // if !impl_path.exists() {
    //     let _= File::create(impl_path);
    // }

    // writeln!(out, "pub mod {};", defn_name)?;
    // writeln!(out, "pub use {}::*;", defn_name)?;
    // writeln!(out, "pub mod {};", impl_name)?;
    // writeln!(out, "pub use {}::*;", impl_name)?;
    // writeln!(out, "")?;
    // let mut out = try!(File::create(defn_path));
    try!(modules::gen_clocks(&cfg, out, &d, path));
    Ok(())
}