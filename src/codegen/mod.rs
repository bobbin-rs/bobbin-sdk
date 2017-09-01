pub mod constants;
pub mod modules;
pub mod registers;
pub mod interrupts;

use std::io::{Write, Result};
use {Device};
pub use self::constants::gen_constants;
pub use self::modules::gen_modules;
pub use self::registers::gen_registers;


pub fn field_getter(name: &str) -> String {    
    match name {
        "AS" | "DO" | "MOD" | "MATCH" | "TYPE" | "RESET" | "LOOP" | "IF" | "IN" | "BREAK" | "REF" => {
            format!("_{}", field_name(name))
        },
        _ => {
            if name.chars().next().unwrap().is_digit(10) {                
                format!("_{}", field_name(name))
            } else {
                String::from(name.to_lowercase())
            }
        }
    }
}

pub fn field_setter(name: &str) -> String {
    format!("set_{}", field_name(name))
}

pub fn field_with(name: &str) -> String {
    format!("with_{}", field_name(name))
}

pub fn field_test(name: &str) -> String {
    format!("test_{}", field_name(name))
}

pub fn field_ptr(name: &str) -> String {
    let name = field_name(name);
    if name.chars().next().unwrap().is_digit(10) {
        format!("_{}_ptr", name)
    } else {
        format!("{}_ptr", name)
    }
    
}

pub fn field_mut(name: &str) -> String {
    let name = field_name(name);
    if name.chars().next().unwrap().is_digit(10) {
        format!("_{}_mut", name)
    } else {
        format!("{}_mut", name)
    }
}

pub fn field_name(name: &str) -> String {
    let mut result = String::new();
    for (i,c) in name.chars().enumerate() {
        if i == 0 && c == '_' {
            continue;
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result    
}

fn to_camel(word: &str) -> String {
    let mut result = String::new();
    
    let mut last_dash = false;
    for (i, c) in word.chars().enumerate() {
        if i == 0 {
            result.push(c.to_uppercase().next().unwrap());
            continue;        
        }
        if c == '_' {
            last_dash = true;
            continue;
        }
        if last_dash {
            result.push(c.to_uppercase().next().unwrap());
            last_dash = false;
            continue
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}


pub fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

pub fn size_type(size: u64) -> &'static str {
    match size {
        8 => "u8",
        16 => "u16",
        32 => "u32",
        64 => "u64",
        _ => panic!("Unsupported size: {}", size),
    }
}


pub fn gen_groups<W: Write>(out: &mut W, d: &Device) -> Result<()> {
    let groups = d.make_groups();
    try!(write!(out, "(device {}", d.name));    

    for (_, periphs) in groups.iter() {
        if periphs.len() > 1 {
            let p0 = d.get_peripheral(&periphs[0]).unwrap();
            try!(write!(out, "\n{}(peripheral-group 0x{:016x}", indent(1), p0.signature()));
            
            for p in periphs.iter() {
                try!(write!(out, "\n{}(peripheral {})", indent(2), p));
            }
            try!(write!(out, ")"));
        } else {
            let p = d.get_peripheral(&periphs[0]).unwrap();
            try!(write!(out, "\n{}(peripheral {} 0x{:016x})", indent(1), p.name, p.signature()));
        }
    }
    Ok(())
}

pub fn gen_signatures<W: Write>(out: &mut W, d: &Device) -> Result<()> {
    try!(write!(out, "(device {}", d.name));


    for p in d.peripherals.iter() {
        let pd = if p.derived_from.is_some() {
            d.get_peripheral(p.derived_from.as_ref().unwrap()).unwrap()
        } else {
            &p
        };

        let p_name = if p.group_name.is_some() {
            format!("{}:{}", p.group_name.as_ref().unwrap(), p.name)
        } else {
            format!("{}", p.name)
        };

        try!(write!(out, "\n{}(peripheral {:16} 0x{:016x}", indent(1), p_name, pd.signature()));

        for r in pd.registers.iter() {
            try!(write!(out, "\n{}(register {:16} 0x{:016x})", indent(2), r.name, r.signature()));
        }

        try!(write!(out, ")"));
    }

    try!(write!(out, ")\n"));


    Ok(())   
}

pub fn gen_resets<W: Write>(out: &mut W, d: &Device) -> Result<()> {
    try!(write!(out, "(device {}", d.name));
    for p in d.peripherals.iter() {
        let pd = if p.derived_from.is_some() {
            d.get_peripheral(p.derived_from.as_ref().unwrap()).unwrap()
        } else {
            &p
        };

        try!(write!(out, "\n{}(peripheral {}", indent(1), p.name));

        for r in pd.registers.iter() {
            if r.reset_value.unwrap_or(0) != 0 {
                try!(write!(out, "\n{}(register {:16} (reset-value 0x{:08x})", indent(2), r.name, r.reset_value.unwrap_or(0) as u32));
                if r.reset_mask.is_some() {
                    try!(write!(out, " (reset-mask 0x{:08x}))", r.reset_mask.unwrap_or(u64::max_value()) as u32));
                }
                try!(write!(out, ")"));
                
            }
        }

        try!(write!(out, ")"));
    }
    try!(write!(out, ")\n"));


    Ok(())   
}