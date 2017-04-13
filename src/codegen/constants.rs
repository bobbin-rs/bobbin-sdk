use std::io::{Write, Result};
use {Device, Peripheral, PeripheralGroup, Cluster, Register, Field};
use super::size_type;


pub fn gen_constants<W: Write>(out: &mut W, d: &Device) -> Result<()> {
    gen_device_constants(out, d)
}

pub fn gen_device_constants<W: Write>(out: &mut W, d: &Device) -> Result<()> {
    for pg in d.peripheral_groups.iter() {
        try!(gen_peripheral_group_constants(out, pg, d.size.or(Some(0x20))))
    }

    for p in d.peripherals.iter() {
        if p.derived_from.is_some() {            
            let mut tmp = d.get_peripheral(p.derived_from.as_ref().unwrap()).unwrap().clone();
            tmp.name = p.name.clone();
            tmp.address = p.address;
            try!(gen_peripheral_constants(out, &tmp, d.size.or(Some(0x20))))
        } else {
            try!(gen_peripheral_constants(out, p, d.size.or(Some(0x20))))
        };
        
    }
    Ok(())
}


pub fn gen_peripheral_group_constants<W: Write>(out: &mut W, pg: &PeripheralGroup, size: Option<u64>) -> Result<()> {
    for p in pg.peripherals.iter() {
        if p.derived_from.is_some() {
            let mut tmp = pg.get_peripheral(p.derived_from.as_ref().unwrap()).unwrap().clone();
            tmp.name = p.name.clone();
            tmp.address = p.address;
            try!(gen_peripheral_constants(out, &tmp, size))
        } else {
            try!(gen_peripheral_constants(out, p, size))
        }
        
    }
    Ok(())
}


pub fn gen_peripheral_constants<W: Write>(out: &mut W, p: &Peripheral, size: Option<u64>) -> Result<()> {
    let p_addr = p.address as u32;
    let size = p.size.or(size);


    try!(write!(out, "pub const {:32}: u32 = 0x{:08x};\n", p.name, p_addr));
    for c in p.clusters.iter() {
        try!(gen_cluster_constants(out, c, &p.name, p_addr, size));
    }    
    for r in p.registers.iter() {
        try!(gen_register_constants(out, r, &p.name, p_addr, size));
    }    
    Ok(())
}

pub fn gen_cluster_constants<W: Write>(out: &mut W, c: &Cluster, name: &str, addr: u32, size: Option<u64>) -> Result<()> {   
    let size = c.size.or(size);

    if let Some(_dim) = c.dim {
        
        try!(write!(out, "// CLUSTER ARRAY \n"));

        for (offset, c_name) in c.iter_dim() {
            let c_name = format!("{}_{}", name, c_name.replace("[","").replace("]",""));
            let c_addr = addr + offset as u32;
            for c in c.clusters.iter() {
                try!(gen_cluster_constants(out, c, &c_name, c_addr, size));
            }    
            for r in c.registers.iter() {
                try!(gen_register_constants(out, r, &c_name, c_addr, size));
            }                
        }

    } else {
        let c_name = format!("{}_{}", name, c.name);
        try!(write!(out, "// CLUSTER \npub const {:32}: u32 = 0x{:08x};\n", c_name, addr));
        for c in c.clusters.iter() {
            try!(gen_cluster_constants(out, c, &c_name, addr + c.offset as u32, size));
        }    
        for r in c.registers.iter() {
            try!(gen_register_constants(out, r, &c_name, addr + c.offset as u32, size));
        }    
    }
    Ok(())
}


pub fn gen_register_constants<W: Write>(out: &mut W, r: &Register, name: &str, addr: u32, size: Option<u64>) -> Result<()> {
    let r_size = match size {
        Some(size) => size_type(size),
        None => panic!("No size defined for register {}", r.name),
    };            
    if let Some(_dim) = r.dim {
        for (offset, name) in r.iter_dim() {
            let r_name = format!("{}_{}", name, r.name.replace("[","").replace("]",""));
            let r_offset = addr + offset as u32;            
            try!(write!(out, "pub const   {:30}: *const {:3} = 0x{:08x}; \n", r_name, r_size, r_offset));
            for f in r.fields.iter() {
               try!(gen_field_constants(out, f, &r_name))
            }
        }
        
        
        // let r_name = format!("{}_{}", name, r.name.replace("%s","x").replace("[","").replace("]",""));
        // try!(write!(out, "pub const   {:30}: [*const {:3}; {}] = [\n", r_name, r_size, dim));
        
        // for (offset, _name) in r.iter_dim() {
        //     let r_offset = addr + offset as u32;
        //     try!(write!(out, "   0x{:08x},\n", r_offset));
        // }
        // try!(write!(out, "];\n"));
        // for f in r.fields.iter() {
        //     try!(gen_field_constants(out, f, &r_name))
        //}
    } else {
        let r_name = format!("{}_{}", name, r.name);
        let r_offset = addr + r.offset as u32;

        try!(write!(out, "pub const   {:30}: *const {:3} = 0x{:08x};\n", r_name, r_size, r_offset));
        for f in r.fields.iter() {
            //try!(gen_field_constants(out, f, &r_name))
        }
    }    
    Ok(())
}

pub fn gen_field_constants<W: Write>(out: &mut W, f: &Field, name: &str) -> Result<()> {
    let f_name = format!("{}_{}", name, f.name);
    //let f_name_mask = write!(out, "    {}_{}_{}_MASK", p.name, r.name, f.name);
    let f_name_shift = format!("{}_SHIFT", f_name);
    let f_offset = f.bit_offset as u32;
    let f_size = f.bit_width as u32;
    let f_lo = f_offset;
    let f_hi = (f_offset + f_size) - 1;
    //print!("f_offset: {}, f_size: {}", f_offset, f_size);
    let f_mask = if f_size == 32 {
        u32::max_value()
    } else {
        ((1 << f_size) - 1)
    };
    let f_shift_mask = f_mask << f_offset;
    let bits = if f_size > 1 {
        format!("[{}:{}]", f_hi, f_lo)
    } else {
        format!("[{}]", f_lo)
    };
    try!(write!(out, "pub const     {:28}:u32 =     0x{:08x}; // {}\n", f_name, f_shift_mask, bits));
    //try!(write!(out, "pub const {:32}:u32 =     0x{:08x};\n", f_name_mask, f_mask));
    if f_size > 1 {
        try!(write!(out, "pub const     {:28}:u32 =       {:8};\n", f_name_shift, f_lo));
    }
    Ok(())
}
