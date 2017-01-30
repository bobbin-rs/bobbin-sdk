#![allow(dead_code, unused_imports)]

extern crate chip;
extern crate xml;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use chip::*;

use std::mem;
use std::fs::File;
use std::io::BufReader;

use regex::Regex;
use xml::reader::{self, EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    StateError(String),
    XmlError(reader::Error),
}

impl From<reader::Error> for Error {
    fn from(other: reader::Error) -> Self {
        Error::XmlError(other)
    }
}

pub struct Document {
    pub device: Device,
}

fn normalize(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }
    RE.replace_all(s, " ").to_string()
}

pub fn read_bit_range(s: &str) -> Result<(u64, u64), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(\d+):(\d+)\]").unwrap();
    }
    let caps = RE.captures(s).unwrap();

    let hi = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let lo = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
    Ok((lo, hi))
}

pub fn read_unknown<R: std::io::Read>(r: &mut EventReader<R>) -> Result<(), Error> {
    let mut depth = 1;
    loop {
        match try!(r.next()) {
            XmlEvent::StartElement { .. } => depth += 1,
            XmlEvent::EndElement { .. } => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            return Ok(());
        }
    }
}

pub fn read_opt_u64<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Option<u64>, Error> {
    let text = try!(read_opt_text(r));
    if let Some(text) = text {
        if text.starts_with("0x") {
            if let Ok(v) = u64::from_str_radix(&text[2..], 16) {
                return Ok(Some(v))
            } else {
                return Err(Error::ParseError(format!("Invalid hex number: {:?}", text)))
            }
        } 
        if let Ok(v) = text.parse::<u64>() {
            Ok(Some(v))
        } else {
            Err(Error::ParseError(format!("Invalid number: {:?}", text)))
        }
    } else {
        return Ok(None);
    }
}

pub fn read_u64<R: std::io::Read>(r: &mut EventReader<R>) -> Result<u64, Error> {
    if let Some(value) = try!(read_opt_u64(r)) {
        Ok(value)
    } else {
        Err(Error::ParseError(format!("Missing number value")))
    }
}


pub fn read_opt_text<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Option<String>, Error> {
    let mut result: Option<String> = None;
    loop {
        let e = try!(r.next());
        match e {
            XmlEvent::Characters(s) => result = Some(s),
            XmlEvent::EndElement { .. } => return Ok(result),
            _ => return Err(Error::StateError(format!("Unexpected text end"))),
        }
    }
}


pub fn read_text<R: std::io::Read>(r: &mut EventReader<R>) -> Result<String, Error> {
    if let Some(text) = try!(read_opt_text(r)) {
        Ok(text)
    } else {
        return Err(Error::StateError(format!("Expected non-empty text")))
    }
}



pub fn read_description<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Option<String>, Error> {
    read_opt_text(r).map(|t| t.map(|s| normalize(s.as_ref())))
}


pub fn read_enumerated_value<R: std::io::Read>(r: &mut EventReader<R>)
                                               -> Result<EnumeratedValue, Error> {
    let mut p_value: Option<String> = None;
    let mut p_name: Option<String> = None;
    let mut p_desc: Option<String> = None;

    loop {
        let e = try!(r.next());
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "value" => p_value = try!(read_opt_text(r)),
                    "name" => p_name = try!(read_opt_text(r)),
                    "description" => p_desc = try!(read_description(r)),
                    _ => try!(read_unknown(r)),
                }
            }
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "enumeratedValue" => {
                        if p_value.is_none() {
                            return Err(Error::StateError(format!("enumerated value without value")));
                        }
                        return Ok(EnumeratedValue {
                            value: p_value.unwrap(),
                            name: p_name,
                            description: p_desc,
                        });

                    }
                    _ => return Err(Error::StateError(format!("Expected </enumeratedValue>"))),
                }
            }
            _ => {}
        }
    }
}

pub fn read_enumerated_values<R: std::io::Read>(r: &mut EventReader<R>)
                                                -> Result<Vec<EnumeratedValue>, Error> {
    let mut values: Vec<EnumeratedValue> = Vec::new();
    loop {
        let e = try!(r.next());
        // println!("read_fields: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => try!(read_unknown(r)),
                    "usage" => try!(read_unknown(r)),
                    "enumeratedValue" => values.push(try!(read_enumerated_value(r))),
                    _ => return Err(Error::StateError(format!("Expected <enumeratedValue>"))),
                }
            }
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "enumeratedValues" => return Ok(values),
                    _ => return Err(Error::StateError(format!("Expected </enumeratedValues>"))),
                }
            }
            _ => {}
        }
    }
}

pub fn read_field<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Field, Error> {
    let mut p_name: Option<String> = None;
    let mut p_desc: Option<String> = None;
    let mut p_access: Option<String> = None;
    let mut p_offset: Option<u64> = None;
    let mut p_width: Option<u64> = None;
    let mut p_range: Option<String> = None;
    let mut p_lsb: Option<u64> = None;
    let mut p_msb: Option<u64> = None;
    let mut p_enumerated_values: Vec<EnumeratedValue> = Vec::new();

    loop {
        let e = try!(r.next());
        // println!("read_field: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => p_name = try!(read_opt_text(r)),
                    "description" => p_desc = try!(read_description(r)),
                    "access" => p_access = try!(read_opt_text(r)),
                    "bitOffset" => p_offset = try!(read_opt_u64(r)),
                    "bitWidth" => p_width = try!(read_opt_u64(r)),
                    "bitRange" => p_range = try!(read_opt_text(r)),
                    "lsb" => p_lsb = try!(read_opt_u64(r)),
                    "msb" => p_msb = try!(read_opt_u64(r)),
                    "enumeratedValues" => {
                        p_enumerated_values.append(&mut try!(read_enumerated_values(r)))
                    }
                    _ => try!(read_unknown(r)),
                }
            }
            XmlEvent::EndElement { name } => {
                let bit_offset: u64;
                let bit_width: u64;
                if let Some(lsb) = p_lsb {
                    if let Some(msb) = p_msb {
                        bit_offset = lsb;
                        bit_width = 1 + msb - lsb;
                    } else {
                        return Err(Error::StateError(format!("No msb specified")));
                    }
                } else if let Some(p_range) = p_range {
                    let (mut lo, mut hi) = try!(read_bit_range(&p_range));
                    if lo > hi {
                        mem::swap(&mut lo, &mut hi)
                    }
                    bit_offset = lo;
                    bit_width = 1 + hi - lo;
                } else if let Some(p_offset) = p_offset {
                    bit_offset = p_offset;
                    bit_width = if let Some(p_width) = p_width {
                        p_width
                    } else {
                        1
                    };
                } else {
                    return Err(Error::StateError(format!("No field width specified")));
                }
                match name.local_name.as_ref() {
                    "field" => {
                        return Ok(Field {
                            name: p_name.unwrap(),
                            description: p_desc,
                            access: p_access.map(|a| Access::from(a.as_ref())),
                            offset: bit_offset,
                            size: bit_width,
                            enumerated_values: p_enumerated_values,
                        })
                    }
                    _ => return Err(Error::StateError(format!("expected </field>"))),
                }
            }
            _ => {}
        }
    }
}

pub fn read_fields<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Vec<Field>, Error> {
    let mut fields: Vec<Field> = Vec::new();
    loop {
        let e = try!(r.next());
        // println!("read_fields: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "field" => fields.push(try!(read_field(r))),
                    _ => return Err(Error::StateError(format!("Expected <field>"))),
                }
            }
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "fields" => return Ok(fields),
                    _ => return Err(Error::StateError(format!("Expected </fields>"))),
                }
            }
            _ => {}
        }
    }
}

pub fn read_cluster<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Cluster, Error> {
    let mut p_name: Option<String> = None;
    let mut p_desc: Option<String> = None;
    let mut p_size: Option<u64> = None;
    let mut p_access: Option<String> = None;
    let mut p_reset_value: Option<u64> = None;
    let mut p_reset_mask: Option<u64> = None;
    
    let mut p_offset: Option<u64> = None;
    let mut p_registers: Vec<Register> = Vec::new();
    let mut dim: Option<u64> = None;
    let mut dim_increment: Option<u64> = None;
    let mut dim_index: Option<String> = None;

    loop {
        let e = try!(r.next());
        // println!("read_register: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => p_name = try!(read_opt_text(r)),
                    "description" => p_desc = try!(read_description(r)),
                    "size" => p_size = try!(read_opt_u64(r)),
                    "access" => p_access = try!(read_opt_text(r)),                    
                    "resetValue" => p_reset_value = try!(read_opt_u64(r)),
                    "resetMask" => p_reset_mask = try!(read_opt_u64(r)),                    
                    "addressOffset" => p_offset = try!(read_opt_u64(r)),
                    "dim" => dim = try!(read_opt_u64(r)),
                    "dimIncrement" => dim_increment = try!(read_opt_u64(r)),
                    "dimIndex" => dim_index = try!(read_opt_text(r)),                    
                    "register" => p_registers.push(try!(read_register(r))),
                    _ => try!(read_unknown(r)),
                }
            }
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "cluster" => {
                        if p_name.is_none() {
                            return Err(Error::StateError(format!("Cluster missing name")));
                        }
                        if p_offset.is_none() {
                            return Err(Error::StateError(format!("Cluster missing offset")));
                        }
                        return Ok(Cluster {
                            name: p_name.unwrap(),
                            offset: p_offset.unwrap(),
                            size: p_size,
                            access: p_access.map(|a| Access::from(a.as_ref())),
                            reset_value: p_reset_value,
                            reset_mask: p_reset_mask,
                            description: p_desc,
                            dim: dim,
                            dim_index: dim_index,
                            dim_increment: dim_increment,
                            clusters: Vec::new(),
                            registers: p_registers,
                        });
                    }
                    _ => return Err(Error::StateError(format!("expected </cluster>"))),
                }
            }
            _ => {}
        }
    }
}


pub fn read_register<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Register, Error> {
    let mut reg = Register::default();
    loop {
        let e = try!(r.next());
        // println!("read_register: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => reg.name = try!(read_text(r)),
                    "description" => reg.description = try!(read_description(r)),
                    "addressOffset" => reg.offset = try!(read_u64(r)),
                    "size" => reg.size = try!(read_opt_u64(r)),
                    "access" => reg.access = try!(read_opt_text(r)).map(Access::from),
                    "resetValue" => reg.reset_value = try!(read_opt_u64(r)),
                    "resetMask" => reg.reset_mask = try!(read_opt_u64(r)),
                    "dim" => reg.dim = try!(read_opt_u64(r)),
                    "dimIncrement" => reg.dim_increment = try!(read_opt_u64(r)),
                    "dimIndex" => reg.dim_index = try!(read_opt_text(r)),
                    "fields" => reg.fields = try!(read_fields(r)),
                    _ => try!(read_unknown(r)),
                }
            }
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "register" => return Ok(reg),
                    _ => return Err(Error::StateError(format!("expected </register>"))),
                }
            }
            _ => {}
        }
    }
}


pub fn read_registers<R: std::io::Read>(r: &mut EventReader<R>)
                                        -> Result<(Vec<Register>, Vec<Cluster>), Error> {
    let mut regs: Vec<Register> = Vec::new();
    let mut clusters: Vec<Cluster> = Vec::new();
    loop {
        let e = try!(r.next());
        // println!("read_registers: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "register" => regs.push(try!(read_register(r))),
                    "cluster" => clusters.push(try!(read_cluster(r))),
                    _ => return Err(Error::StateError(format!("Expected <register> or <cluster>"))),
                }
            }
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "registers" => return Ok((regs, clusters)),
                    _ => return Err(Error::StateError(format!("Expected </registers>"))),
                }
            }
            _ => {}
        }
    }
}

pub fn read_interrupt<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Interrupt, Error> {
    let mut i = Interrupt::default();
    loop {
        let e = try!(r.next());
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => i.name = try!(read_text(r)),
                    "description" => i.description = try!(read_description(r)),
                    "value" => i.value = try!(read_u64(r)),
                    _ => try!(read_unknown(r)),
                }
            }
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "interrupt" => return Ok(i),
                    _ => return Err(Error::StateError(format!("expected </interrupt>"))),
                }
            }
            _ => {}
        }
    }
}


pub fn read_peripheral<R: std::io::Read>(r: &mut EventReader<R>,
                                         attrs: &[OwnedAttribute])
                                         -> Result<Peripheral, Error> {
    let mut p = Peripheral::default();

    for a in attrs.iter() {
        if a.name.local_name == "derivedFrom" {
            p.derived_from = Some(a.value.clone());
        }
    }

    loop {
        let e = try!(r.next());
        // println!("read_peripheral: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => p.name = try!(read_text(r)),
                    "description" => p.description = try!(read_description(r)),
                    "baseAddress" => p.address = try!(read_u64(r)),
                    "groupName" => p.group_name = try!(read_opt_text(r)),
                    "dim" => p.dim = try!(read_opt_u64(r)),
                    "dimIncrement" => p.dim_increment = try!(read_opt_u64(r)),
                    "dimIndex" => p.dim_index = try!(read_opt_text(r)),
                    "size" => p.size = try!(read_opt_u64(r)),
                    "access" => p.access = try!(read_opt_text(r)).map(|s| Access::from(s.as_ref())),
                    "resetValue" => p.reset_value = try!(read_opt_u64(r)),
                    "resetMask" => p.reset_mask = try!(read_opt_u64(r)),                    
                    "interrupt" => p.interrupts.push(try!(read_interrupt(r))),
                    "registers" => {
                        let (regs, clusters) = try!(read_registers(r));
                        p.registers = regs;
                        p.clusters = clusters;
                    }
                    _ => try!(read_unknown(r)),
                }
            }
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "peripheral" => return Ok(p),                    
                    _ => return Err(Error::StateError(format!("expected </peripheral>"))),
                }
            }
            _ => {}
        }
    }
}

pub fn read_peripherals<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Vec<Peripheral>, Error> {
    let mut periphs: Vec<Peripheral> = Vec::new();
    loop {
        let e = try!(r.next());
        // println!("read_peripherals: {:?}", e);
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {
                match name.local_name.as_ref() {
                    "peripheral" => periphs.push(try!(read_peripheral(r, &attributes))),
                    _ => return Err(Error::StateError(format!("Expected <peripheral>"))),
                }
            }
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "peripherals" => return Ok(periphs),
                    _ => return Err(Error::StateError(format!("Expected </peripherals>"))),
                }
            }
            _ => {}
        }
    }
}

pub fn read_device<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Device, Error> {
    let mut d = Device::default();
    loop {
        let e = try!(r.next());
        // println!("read_device: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "vendor" => d.vendor = try!(read_opt_text(r)),
                    "vendor_id" => d.vendor_id = try!(read_opt_text(r)),
                    "name" => d.name = try!(read_text(r)),
                    "size" => d.size = try!(read_opt_u64(r)),
                    "access" => d.access = try!(read_opt_text(r)).map(|s| Access::from(s.as_ref())),
                    "description" => d.description = try!(read_description(r)),
                    "peripherals" => d.peripherals = try!(read_peripherals(r)),
                    _ => try!(read_unknown(r)),
                }
            }
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "device" => {
                        return Ok(d)
                    },
                    _ => return Err(Error::StateError(format!("Expected </device>"))),
                }
            }
            _ => {}
        }
    }

}

pub fn read_document<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Document, Error> {
    let mut device: Option<Device> = None;
    loop {
        let e = try!(r.next());
        // println!("read_document: {:?}", e);
        match e {
            XmlEvent::StartDocument { .. } => {}
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "device" => device = Some(try!(read_device(r))),
                    _ => return Err(Error::StateError(format!("Expected device"))),
                }
            }
            XmlEvent::EndDocument => {
                if device.is_none() {
                    return Err(Error::StateError(format!("No device found in document")));
                }
                return Ok(Document { device: device.unwrap() });
            }
            _ => {}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_xml() {
        let data = "
        <device>
            <name>STM32F40x</name>
            <peripherals>
                <peripheral>
                    <name>RNG</name>
                    <description>Random Number Generator</description>
                    <baseAddress>0x50060800</baseAddress>
                    <registers>
                        <register>
                            <name>CR</name>
                            <description>Control Register</description>
                            <addressOffset>0x0</addressOffset>
                            <fields>
                                <field>
                                    <name>IE</name>
                                    <description>Interrupt Enable</description>
                                    <bitOffset>3</bitOffset>
                                    <bitWidth>1</bitWidth>
                                </field>
                            </fields>
                        </register>
                    </registers>
                </peripheral>
            </peripherals>
        </device>";
        let mut reader = EventReader::new(Cursor::new(data));
        let d = read_document(&mut reader).unwrap();
        assert_eq!(d.device.name, "STM32F40x");
        assert_eq!(d.device.description, None);
        let periphs = d.device.peripherals;
        let p = &periphs[0];
        assert_eq!(p.name, "RNG");
        // assert_eq!(p.description.as_ref(), Some("Random Number Generator"));
    }

    #[test]
    fn test_svd() {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open("svd/STMicro/STM32F40x.svd").unwrap();
        let file = BufReader::new(file);
        let mut reader = EventReader::new(file);
        let _d = read_document(&mut reader).unwrap();

    }
}
