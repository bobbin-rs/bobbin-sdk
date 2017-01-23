#![allow(dead_code, unused_imports)]

extern crate xml;
extern crate regex;
#[macro_use] extern crate lazy_static;

use std::mem;
use std::fs::File;
use std::io::BufReader;

use regex::Regex;
use xml::reader::{self, EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

#[derive(Debug)]
pub enum Error {    
    ParseError(&'static str),
    StateError(&'static str),
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

pub struct Device {
    pub name: String,
    pub peripherals: Vec<Peripheral>,
    pub description: Option<String>,
}

pub struct Peripheral {
    pub name: String,
    pub address: String,
    pub interrupts: Vec<Interrupt>,
    pub registers: Vec<Register>,
    pub clusters: Vec<Cluster>,
    pub group_name: Option<String>,
    pub dim: Option<u64>,
    pub dim_index: Option<String>,
    pub dim_increment: Option<String>,    
    pub derived_from: Option<String>,
    pub description: Option<String>,
}

pub struct Interrupt {
    pub name: String,
    pub value: u64,
    pub description: Option<String>,    
}

pub struct Cluster {
    pub name: String,
    pub offset: String,
    pub registers: Vec<Register>,
    pub description: Option<String>,
}

pub struct Register {
    pub name: String,
    pub offset: String,
    pub fields: Vec<Field>,
    pub description: Option<String>,
    pub reset_value: Option<String>,
    pub dim: Option<u64>,
    pub dim_index: Option<String>,
    pub dim_increment: Option<String>,
}

pub struct Field {
    pub name: String,
    pub bits: String,
    pub description: Option<String>,
    pub access: Option<String>,
    pub enumerated_values: Vec<EnumeratedValue>,
}

pub struct EnumeratedValue {
    pub value: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub fn read_bit_range(s: &str) -> Result<(usize, usize), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(\d+):(\d+)\]").unwrap();
    }
    let caps = RE.captures(s).unwrap();
    
    let hi = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let lo = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    Ok((lo, hi))
}

pub fn read_unknown<R: std::io::Read>(r: &mut EventReader<R>) -> Result<(), Error> {
    let mut depth = 1;
    loop {
        match try!(r.next()) {
            XmlEvent::StartElement { .. }  => depth += 1,
            XmlEvent::EndElement { .. } => depth -= 1,
            _ => {},
        }
        if depth == 0 {
            return Ok(())
        }
    }
}

pub fn read_u64<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Option<u64>, Error> {
    let text = try!(read_text(r));
    if let Some(text) = text {
        if let Ok(v) = text.parse::<u64>() {
            Ok(Some(v))
        } else {
            Err(Error::ParseError("Invalid number"))
        }
    } else {
        return Ok(None)
    }
}

pub fn read_text<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Option<String>, Error> {
    let mut result: Option<String> = None;
    loop {
        let e = try!(r.next());
        //println!("read_text: {:?}", e);

        match e {
            XmlEvent::Characters(s) => result = Some(s),
            XmlEvent::EndElement {..} => {
                return Ok(result)
            },
            _ => {
                return Err(Error::StateError("Unexpected text end"))
            }
        }
    }
}

pub fn read_enumerated_value<R: std::io::Read>(r: &mut EventReader<R>) -> Result<EnumeratedValue, Error> {
    let mut p_value: Option<String> = None;
    let mut p_name: Option<String> = None;
    let mut p_desc: Option<String> = None;
    
    loop {
        let e = try!(r.next());
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "value" => p_value = try!(read_text(r)),
                    "name" => p_name = try!(read_text(r)),
                    "description" => p_desc = try!(read_text(r)),
                    _ => try!(read_unknown(r)),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "enumeratedValue" => {
                        if p_value.is_none() {
                            return Err(Error::StateError("enumerated value without value"))
                        }
                        return Ok(EnumeratedValue {
                            value: p_value.unwrap(),
                            name: p_name,
                            description: p_desc,
                        })
                        
                    },
                    _ => return Err(Error::StateError("Expected </enumeratedValue>"))
                }
            },
            _ => {},
        }
    }   
}

pub fn read_enumerated_values<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Vec<EnumeratedValue>, Error> {
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
                    _ => return Err(Error::StateError("Expected <enumeratedValue>")),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "enumeratedValues" => {
                        return Ok(values)
                    }
                    _ => return Err(Error::StateError("Expected </enumeratedValues>"))
                }
            },
            _ => {},
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
                    "name" => p_name = try!(read_text(r)),
                    "description" => p_desc = try!(read_text(r)),
                    "access" => p_access = try!(read_text(r)),
                    "bitOffset" => p_offset = try!(read_u64(r)),
                    "bitWidth" => p_width = try!(read_u64(r)),
                    "bitRange" => p_range = try!(read_text(r)),
                    "lsb" => p_lsb = try!(read_u64(r)),
                    "msb" => p_msb = try!(read_u64(r)),
                    "enumeratedValues" => p_enumerated_values.append(&mut try!(read_enumerated_values(r))),
                    _ => try!(read_unknown(r)),
                }
            },
            XmlEvent::EndElement { name } => {
                let mut bits = String::new();      
                if let Some(ref lsb) = p_lsb {
                    if let Some(ref msb) = p_msb {
                        bits = format!("({} {})", lsb, 1 + msb - lsb);
                    } else {
                        return Err(Error::StateError("No msb specified"));
                    }
                }
                if let Some(ref p_range) = p_range {
                    let (mut lo, mut hi) = try!(read_bit_range(p_range));
                    if lo > hi {
                        mem::swap(&mut lo, &mut hi)
                    }
                    if hi != lo {
                        bits = format!("({} {})", lo, 1 + hi - lo);
                    } else {
                        bits = format!("{}", lo);
                    }
                }
                if let Some(p_offset) = p_offset {
                    if let Some(p_width) = p_width {
                        if p_width > 1 {
                            bits = format!("({} {})", p_offset, p_width);
                        } else {
                            bits = format!("{}", p_offset)
                        }
                    } else {
                        bits = format!("{}", p_offset)
                    }                     
                }                
                if bits.len() == 0 {
                    return Err(Error::StateError("No field width specified"));
                }
                match name.local_name.as_ref() {
                    "field" => return Ok(Field {
                        name: p_name.unwrap(),
                        description: p_desc,
                        access: p_access,
                        bits: bits,
                        enumerated_values: p_enumerated_values,
                    }),
                    _ => return Err(Error::StateError("expected </field>")),
                }
            },
            _ => {},
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
                    _ => return Err(Error::StateError("Expected <field>")),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "fields" => {
                        return Ok(fields)
                    }
                    _ => return Err(Error::StateError("Expected </fields>"))
                }
            },
            _ => {},
        }
    }
}

pub fn read_cluster<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Cluster, Error> {
    let mut p_name: Option<String> = None;
    let mut p_desc: Option<String> = None;
    let mut p_offset: Option<String> = None;
    let mut p_registers: Vec<Register> = Vec::new();

    loop {
        let e = try!(r.next());
        // println!("read_register: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => p_name = try!(read_text(r)),
                    "description" => p_desc = try!(read_text(r)),
                    "addressOffset" => p_offset = try!(read_text(r)),
                    "register" => p_registers.push(try!(read_register(r))),
                    _ => try!(read_unknown(r)),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "cluster" => {
                        if p_name.is_none() {
                            return Err(Error::StateError("Cluster missing name"))
                        }
                        if p_offset.is_none() {
                            return Err(Error::StateError("Cluster missing offset"))
                        }
                        return Ok(Cluster {
                            name: p_name.unwrap(),
                            offset: p_offset.unwrap(),
                            description: p_desc,
                            registers: p_registers,
                        })
                    },
                    _ => return Err(Error::StateError("expected </cluster>")),
                }
            },
            _ => {},
        }
    }
}


pub fn read_register<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Register, Error> {
    let mut p_name: Option<String> = None;
    let mut p_desc: Option<String> = None;
    let mut p_offset: Option<String> = None;
    let mut p_reset_value: Option<String> = None;
    let mut p_fields: Vec<Field> = Vec::new();
    let mut dim: Option<u64> = None;
    let mut dim_increment: Option<String> = None;
    let mut dim_index: Option<String> = None;
    loop {
        let e = try!(r.next());
        // println!("read_register: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => p_name = try!(read_text(r)),
                    "description" => p_desc = try!(read_text(r)),
                    "addressOffset" => p_offset = try!(read_text(r)),
                    "resetValue" => p_reset_value = try!(read_text(r)),
                    "dim" => dim = try!(read_u64(r)),
                    "dimIncrement" => dim_increment = try!(read_text(r)),
                    "dimIndex" => dim_index = try!(read_text(r)),
                    "fields" => p_fields = try!(read_fields(r)),
                    _ => try!(read_unknown(r)),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "register" => {
                        if p_name.is_none() {
                            return Err(Error::StateError("Register missing name"))
                        }
                        if p_offset.is_none() {
                            return Err(Error::StateError("Register missing offset"))
                        }
                        return Ok(Register {
                            name: p_name.unwrap(),
                            offset: p_offset.unwrap(),
                            reset_value: p_reset_value,
                            description: p_desc,                                                        
                            dim: dim,
                            dim_index: dim_index,
                            dim_increment: dim_increment,
                            fields: p_fields,
                        })
                    },
                    _ => return Err(Error::StateError("expected </register>")),
                }
            },
            _ => {},
        }
    }
}


pub fn read_registers<R: std::io::Read>(r: &mut EventReader<R>) -> Result<(Vec<Register>, Vec<Cluster>), Error> {
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
                    _ => return Err(Error::StateError("Expected <register> or <cluster>")),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "registers" => {
                        return Ok((regs, clusters))
                    }
                    _ => return Err(Error::StateError("Expected </registers>"))
                }
            },
            _ => {},
        }
    }
}

pub fn read_interrupt<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Interrupt, Error> {
    let mut p_name: Option<String> = None;
    let mut p_desc: Option<String> = None;
    let mut p_value: Option<u64> = None;
    loop {
        let e = try!(r.next());
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => p_name = try!(read_text(r)),
                    "description" => p_desc = try!(read_text(r)),
                    "value" => p_value = try!(read_u64(r)),
                    _ => try!(read_unknown(r)),
                }            
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "interrupt" => {
                        if p_name.is_none() {
                            return Err(Error::StateError("Interrupt missing name"))
                        }
                        if p_value.is_none() {
                            return Err(Error::StateError("Interrupt missing value"))
                        }
                        return Ok(Interrupt {
                            name: p_name.unwrap(),
                            value: p_value.unwrap(),
                            description: p_desc,
                        })
                    },
                    _ => return Err(Error::StateError("expected </interrupt>")),
                }
            },
            _ => {},
        }
    }
}


pub fn read_peripheral<R: std::io::Read>(r: &mut EventReader<R>, attrs: &[OwnedAttribute]) -> Result<Peripheral, Error> {
    let mut p_name: Option<String> = None;
    let mut p_desc: Option<String> = None;
    let mut p_addr: Option<String> = None;
    let mut p_group_name: Option<String> = None;  
    let mut dim: Option<u64> = None;
    let mut dim_increment: Option<String> = None;
    let mut dim_index: Option<String> = None;      
    let mut p_derived_from: Option<String> = None;
    let mut p_interrupts: Vec<Interrupt> = Vec::new();
    let mut p_registers: Vec<Register> = Vec::new();
    let mut p_clusters: Vec<Cluster> = Vec::new();

    for a in attrs.iter() {
        if a.name.local_name == "derivedFrom" {
            p_derived_from = Some(a.value.clone());
        }
    }

    loop {
        let e = try!(r.next());
        // println!("read_peripheral: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => p_name = try!(read_text(r)),
                    "description" => p_desc = try!(read_text(r)),
                    "baseAddress" => p_addr = try!(read_text(r)),
                    "groupName" => p_group_name = try!(read_text(r)),
                    "dim" => dim = try!(read_u64(r)),
                    "dimIncrement" => dim_increment = try!(read_text(r)),
                    "dimIndex" => dim_index = try!(read_text(r)),
                    "interrupt" => p_interrupts.push(try!(read_interrupt(r))),
                    "registers" => { 
                        let (regs, clusters) = try!(read_registers(r)); 
                        p_registers = regs;
                        p_clusters = clusters;
                    },
                    _ => try!(read_unknown(r)),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "peripheral" => {
                        if p_name.is_none() {
                            return Err(Error::StateError("Peripheral missing name"))
                        }
                        if p_addr.is_none() {
                            return Err(Error::StateError("Peripheral missing address"))
                        }
                        return Ok(Peripheral {
                            name: p_name.unwrap(),
                            address: p_addr.unwrap(),
                            description: p_desc,
                            group_name: p_group_name,
                            dim: dim,
                            dim_index: dim_index,
                            dim_increment: dim_increment,                            
                            derived_from: p_derived_from,
                            interrupts: p_interrupts,
                            registers: p_registers,
                            clusters: p_clusters,
                        })
                    },
                    _ => return Err(Error::StateError("expected </peripheral>")),
                }
            },
            _ => {},
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
                    _ => return Err(Error::StateError("Expected <peripheral>")),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "peripherals" => {
                        return Ok(periphs)
                    }
                    _ => return Err(Error::StateError("Expected </peripherals>"))
                }
            },
            _ => {},
        }
    }
}

pub fn read_device<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Device, Error> {
    let mut d_name: Option<String> = None;
    let mut d_desc: Option<String> = None;
    let mut d_periphs: Option<Vec<Peripheral>> = None;
    loop {
        let e = try!(r.next());
        // println!("read_device: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. }  => {
                match name.local_name.as_ref() {
                    "name" => { d_name = try!(read_text(r)); },
                    "description" => { d_desc = try!(read_text(r)); },
                    "peripherals" => { d_periphs = Some(try!(read_peripherals(r))); },                    
                    _ => try!(read_unknown(r)),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "device" => {
                        if d_name.is_none() {
                            return Err(Error::StateError("No name found for device"))
                        }
                        if d_periphs.is_none() {
                            return Err(Error::StateError("No peripherals found for device"))
                        }
                        return Ok(Device { 
                            name: d_name.unwrap(), 
                            peripherals: d_periphs.unwrap(),
                            description: d_desc,
                        })
                    },
                    _ => return Err(Error::StateError("Expected </device>"))
                }
            },
            _ => {},
        }
    }

}

pub fn read_document<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Document, Error> {
    let mut device: Option<Device> = None;
    loop {
        let e = try!(r.next());
        // println!("read_document: {:?}", e);
        match e {
            XmlEvent::StartDocument {..} => {},
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "device" => device = Some(try!(read_device(r))),
                    _ => return Err(Error::StateError("Expected device")),
                }
            }
            XmlEvent::EndDocument => {
                if device.is_none() {
                    return Err(Error::StateError("No device found in document"))
                }
                return Ok(Document { device: device.unwrap()})
            },
            _ => {},
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
        //assert_eq!(p.description.as_ref(), Some("Random Number Generator"));
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
