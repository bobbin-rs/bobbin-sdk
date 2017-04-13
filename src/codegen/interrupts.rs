use std::io::{Write, Result};
use {Device, Interrupt};
use super::indent;


pub fn display_interrupts<W: Write>(out: &mut W, d: &Device) -> Result<()> {
    let mut interrupts: Vec<&Interrupt> = Vec::new();

    for p in d.peripherals.iter() {
        for i in p.interrupts.iter() {
            interrupts.push(i);
        }
    }

    for pg in d.peripheral_groups.iter() {
        for p in pg.peripherals.iter() {
            for i in p.interrupts.iter() {
                interrupts.push(i)
            }
        }
    }

    interrupts.sort_by_key(|i| i.value);

    try!(write!(out, "(interrupts"));
    for i in interrupts.iter() {
        try!(write!(out, "\n{}(interrupt (value {}) (name {}) (description {:?})", indent(1), i.value, i.name, i.description));
    }
    try!(write!(out, ")\n"));

    Ok(())
}