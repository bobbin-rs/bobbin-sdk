use std;
use std::fmt::{Debug, Display, LowerHex};
use std::io::Write;
use {Device, Peripheral, Interrupt, Cluster, Register, Field, EnumeratedValue};

pub struct Context<'a, W: 'a + Write> {
    pub out: &'a mut W,
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

pub fn write_attr_str<W: Write, T: Debug>(ctx: &mut Context<W>, depth: usize, name: &str, attr: &T) -> std::io::Result<()> {    
    writeln!(&mut ctx.out, "{}({} {:?})", indent(depth), name, attr)
}

pub fn write_attr_sym<W: Write, T: Display>(ctx: &mut Context<W>, depth: usize, name: &str, attr: &T) -> std::io::Result<()> {    
    writeln!(&mut ctx.out, "{}({} {})", indent(depth), name, attr)
}

pub fn write_attr_hex<W: Write, T: LowerHex>(ctx: &mut Context<W>, depth: usize, name: &str, attr: &T) -> std::io::Result<()> {    
    writeln!(&mut ctx.out, "{}({} 0x{:x})", indent(depth), name, attr)
}

pub fn write_opt_attr_str<W: Write, T: Debug>(ctx: &mut Context<W>, depth: usize, name: &str, attr: &Option<T>) -> std::io::Result<()> {
    if let &Some(ref attr) = attr {
        try!(writeln!(&mut ctx.out, "{}({} {:?})", indent(depth), name, attr));
    }    
    Ok(())
}

pub fn write_opt_attr_sym<W: Write, T: Display>(ctx: &mut Context<W>, depth: usize, name: &str, attr: &Option<T>) -> std::io::Result<()> {
    if let &Some(ref attr) = attr {
        try!(writeln!(&mut ctx.out, "{}({} {})", indent(depth), name, attr));
    }    
    Ok(())
}

pub fn write_opt_attr_hex<W: Write, T: LowerHex>(ctx: &mut Context<W>, depth: usize, name: &str, attr: &Option<T>) -> std::io::Result<()> {
    if let &Some(ref attr) = attr {
        try!(writeln!(&mut ctx.out, "{}({} 0x{:x})", indent(depth), name, attr));
    }    
    Ok(())
}

pub fn write_device<W: Write>(ctx: &mut Context<W>, depth: usize, d: &Device) -> std::io::Result<()> {
    write_device_open(ctx, depth, d)?;
    for p in d.peripherals.iter() {
        write_peripheral(ctx, depth + 1, p)?;
    }
    write_device_close(ctx, depth, d)?;
    Ok(())
}

pub fn write_device_open<W: Write>(ctx: &mut Context<W>, depth: usize, d: &Device) -> std::io::Result<()> {
    try!(writeln!(&mut ctx.out, "{}(device", indent(depth)));
    try!(write_attr_sym(ctx, depth + 1, "name", &d.name));
    try!(write_opt_attr_str(ctx, depth + 1, "vendor", &d.vendor));
    try!(write_opt_attr_str(ctx, depth + 1, "vendor-id", &d.vendor_id));
    try!(write_opt_attr_hex(ctx, depth + 1, "size", &d.size));
    try!(write_opt_attr_sym(ctx, depth + 1, "access", &d.access));
    try!(write_opt_attr_sym(ctx, depth + 1, "interrupt-count", &d.interrupt_count));
    try!(write_opt_attr_str(ctx, depth + 1, "description", &d.description));
    Ok(())
}
pub fn write_device_close<W: Write>(ctx: &mut Context<W>, depth: usize, _d: &Device) -> std::io::Result<()> {
    try!(writeln!(&mut ctx.out, "{})", indent(depth)));
    Ok(())    
}

pub fn write_peripheral_include<W: Write>(ctx: &mut Context<W>,
                              depth: usize,
                              path: &str)
                              -> std::io::Result<()> {
    try!(writeln!(&mut ctx.out, "{}(peripheral (include \"{}\"))", indent(depth), path));
    Ok(())
}

pub fn write_peripheral<W: Write>(ctx: &mut Context<W>,
                              depth: usize,
                              d: &Peripheral)
                              -> std::io::Result<()> {
    try!(writeln!(&mut ctx.out, "{}(peripheral", indent(depth)));
    try!(write_opt_attr_sym(ctx, depth + 1, "derived-from", &d.derived_from));
    try!(write_opt_attr_sym(ctx, depth + 1, "group-name", &d.group_name));
    if d.dim.is_some() {
        try!(write_attr_str(ctx, depth + 1, "name", &d.name));
    } else {
        try!(write_attr_sym(ctx, depth + 1, "name", &d.name));
    }
    
    try!(write_attr_hex(ctx, depth + 1, "address", &d.address));
    try!(write_opt_attr_hex(ctx, depth + 1, "size", &d.size));
    try!(write_opt_attr_sym(ctx, depth + 1, "access", &d.access));
    try!(write_opt_attr_hex(ctx, depth + 1, "reset-value", &d.reset_value));
    try!(write_opt_attr_hex(ctx, depth + 1, "reset-mask", &d.reset_mask));   
    try!(write_opt_attr_str(ctx, depth + 1, "description", &d.description)); 

    try!(write_opt_attr_sym(ctx, depth + 1, "dim", &d.dim));
    try!(write_opt_attr_sym(ctx, depth + 1, "dim-increment", &d.dim_increment));
    try!(write_opt_attr_str(ctx, depth + 1, "dim-index", &d.dim_index));
    
    for p in d.interrupts.iter() {
        try!(write_interrupt(ctx, depth + 1, p));
    }
    for p in d.clusters.iter() {
        try!(write_cluster(ctx, depth + 1, p));
    }
    for p in d.registers.iter() {
        try!(write_register(ctx, depth + 1, p));
    }

    try!(writeln!(&mut ctx.out, "{})", indent(depth)));
    Ok(())    
}

fn write_interrupt<W: Write>(ctx: &mut Context<W>,
                             depth: usize,
                             d: &Interrupt)
                             -> std::io::Result<()> {
    try!(writeln!(&mut ctx.out, "{}(interrupt", indent(depth)));
    try!(write_attr_sym(ctx, depth + 1, "name", &d.name));
    try!(write_attr_sym(ctx, depth + 1, "value", &d.value));
    try!(write_opt_attr_str(ctx, depth + 1, "description", &d.description));
    try!(writeln!(&mut ctx.out, "{})", indent(depth)));
    Ok(())
}



fn write_cluster<W: Write>(ctx: &mut Context<W>, depth: usize, d: &Cluster) -> std::io::Result<()> {
    try!(writeln!(&mut ctx.out, "{}(cluster", indent(depth)));
    if d.dim.is_some() {
        try!(write_attr_str(ctx, depth + 1, "name", &d.name));
    } else {
        try!(write_attr_sym(ctx, depth + 1, "name", &d.name));
    }
    try!(write_attr_hex(ctx, depth + 1, "offset", &d.offset));
    try!(write_opt_attr_hex(ctx, depth + 1, "size", &d.size));
    try!(write_opt_attr_sym(ctx, depth + 1, "access", &d.access));
    try!(write_opt_attr_hex(ctx, depth + 1, "reset-value", &d.reset_value));
    try!(write_opt_attr_hex(ctx, depth + 1, "reset-mask", &d.reset_mask));   
    try!(write_opt_attr_str(ctx, depth + 1, "description", &d.description)); 

    try!(write_opt_attr_sym(ctx, depth + 1, "dim", &d.dim));
    try!(write_opt_attr_sym(ctx, depth + 1, "dim-increment", &d.dim_increment));
    try!(write_opt_attr_str(ctx, depth + 1, "dim-index", &d.dim_index));

    for p in d.clusters.iter() {
        try!(write_cluster(ctx, depth + 1, p));
    }
    for p in d.registers.iter() {
        try!(write_register(ctx, depth + 1, p));
    }
    try!(writeln!(&mut ctx.out, "{})", indent(depth)));
    Ok(())
}

fn write_register<W: Write>(ctx: &mut Context<W>,
                            depth: usize,
                            d: &Register)
                            -> std::io::Result<()> {
    try!(writeln!(&mut ctx.out, "{}(register", indent(depth)));                                
    if d.dim.is_some() {
        try!(write_attr_str(ctx, depth + 1, "name", &d.name));
    } else {
        try!(write_attr_sym(ctx, depth + 1, "name", &d.name));
    }    
    try!(write_attr_hex(ctx, depth + 1, "offset", &d.offset));
    try!(write_opt_attr_hex(ctx, depth + 1, "size", &d.size));
    try!(write_opt_attr_sym(ctx, depth + 1, "access", &d.access));
    try!(write_opt_attr_hex(ctx, depth + 1, "reset-value", &d.reset_value));
    try!(write_opt_attr_hex(ctx, depth + 1, "reset-mask", &d.reset_mask));   
    try!(write_opt_attr_str(ctx, depth + 1, "description", &d.description)); 

    try!(write_opt_attr_sym(ctx, depth + 1, "dim", &d.dim));
    try!(write_opt_attr_sym(ctx, depth + 1, "dim-increment", &d.dim_increment));
    try!(write_opt_attr_str(ctx, depth + 1, "dim-index", &d.dim_index));

    for p in d.fields.iter() {
        try!(write_field(ctx, depth + 1, p));
    }

    try!(writeln!(&mut ctx.out, "{})", indent(depth)));    
    Ok(())
}

fn write_field<W: Write>(ctx: &mut Context<W>, depth: usize, d: &Field) -> std::io::Result<()> {

    try!(writeln!(&mut ctx.out, "{}(field", indent(depth)));
    try!(write_attr_sym(ctx, depth + 1, "name", &d.name));
    try!(write_attr_sym(ctx, depth + 1, "bit-offset", &d.bit_offset));
    try!(write_attr_sym(ctx, depth + 1, "bit-width", &d.bit_width));    
    try!(write_opt_attr_sym(ctx, depth + 1, "access", &d.access));
    try!(write_opt_attr_str(ctx, depth + 1, "description", &d.description)); 
    
    for v in d.enumerated_values.iter() {
        try!(write_enumerated_value(ctx, depth + 1, v));
    }
    
    try!(writeln!(&mut ctx.out, "{})", indent(depth)));
    Ok(())
}

fn write_enumerated_value<W: Write>(ctx: &mut Context<W>,
                                    depth: usize,
                                    d: &EnumeratedValue)
                                    -> std::io::Result<()> {
    try!(writeln!(&mut ctx.out, "{}(value", indent(depth)));
    try!(write_attr_str(ctx, depth + 1, "value", &d.value));    
    try!(write_opt_attr_str(ctx, depth + 1, "name", &d.name));
    try!(write_opt_attr_str(ctx, depth + 1, "description", &d.description)); 
    try!(writeln!(&mut ctx.out, "{})", indent(depth)));
    Ok(())
}
