#![allow(dead_code, unused_imports)]

extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{self, EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

#[derive(Debug)]
pub enum Error {    
    StateError(&'static str),
    XmlError(reader::Error),
}

impl From<reader::Error> for Error {
    fn from(other: reader::Error) -> Self {
        Error::XmlError(other)
    }
}

pub struct Document {
    device: Device,
}

pub struct Device {
    name: String,
    peripherals: Vec<Peripheral>,
    description: Option<String>,
}

pub struct Peripheral {
    name: String,
    address: String,
    derived_from: Option<String>,
    registers: Option<Vec<Register>>,
    description: Option<String>,
}

pub struct Register {
    name: String,
    offset: String,
    fields: Vec<Field>,
    description: Option<String>,
}

pub struct Field {
    name: String,
    bits: String,
    description: Option<String>,
    access: Option<String>,
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


pub fn read_text<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Option<String>, Error> {
    let mut result: Option<String> = None;
    loop {
        let e = try!(r.next());
        println!("read_text: {:?}", e);

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


pub fn read_field<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Field, Error> {
    let mut p_name: Option<String> = None;
    let mut p_desc: Option<String> = None;
    let mut p_access: Option<String> = None;
    let mut p_offset: Option<String> = None;
    let mut p_width: Option<String> = None;
    loop {
        let e = try!(r.next());
        println!("read_field: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => p_name = try!(read_text(r)),
                    "description" => p_desc = try!(read_text(r)),
                    "access" => p_access = try!(read_text(r)),
                    "bitOffset" => p_offset = try!(read_text(r)),
                    "bitWidth" => p_width = try!(read_text(r)),
                    _ => try!(read_unknown(r)),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "field" => return Ok(Field {
                        name: p_name.unwrap(),
                        description: p_desc,
                        access: p_access,
                        bits: format!("[{}:{}]", p_offset.unwrap(), p_width.unwrap()),
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
        println!("read_fields: {:?}", e);
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

pub fn read_register<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Register, Error> {
    let mut p_name: Option<String> = None;
    let mut p_desc: Option<String> = None;
    let mut p_offset: Option<String> = None;
    let mut p_fields: Option<Vec<Field>> = None;
    loop {
        let e = try!(r.next());
        println!("read_register: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => p_name = try!(read_text(r)),
                    "description" => p_desc = try!(read_text(r)),
                    "addressOffset" => p_offset = try!(read_text(r)),
                    "fields" => p_fields = Some(try!(read_fields(r))),
                    _ => try!(read_unknown(r)),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "register" => return Ok(Register {
                        name: p_name.unwrap(),
                        description: p_desc,
                        offset: p_offset.unwrap(),
                        fields: p_fields.unwrap(),
                    }),
                    _ => return Err(Error::StateError("expected </register>")),
                }
            },
            _ => {},
        }
    }
}


pub fn read_registers<R: std::io::Read>(r: &mut EventReader<R>) -> Result<Vec<Register>, Error> {
    let mut regs: Vec<Register> = Vec::new();
    loop {
        let e = try!(r.next());
        println!("read_registers: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "register" => regs.push(try!(read_register(r))),
                    _ => return Err(Error::StateError("Expected <register>")),
                }
            },
            XmlEvent::EndElement { name } => {
                match name.local_name.as_ref() {
                    "registers" => {
                        return Ok(regs)
                    }
                    _ => return Err(Error::StateError("Expected </registers>"))
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
    let mut p_derived_from: Option<String> = None;
    let mut p_registers: Option<Vec<Register>> = None;

    for a in attrs.iter() {
        if a.name.local_name == "derivedFrom" {
            p_derived_from = Some(a.value.clone());
        }
    }

    loop {
        let e = try!(r.next());
        println!("read_peripheral: {:?}", e);
        match e {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => p_name = try!(read_text(r)),
                    "description" => p_desc = try!(read_text(r)),
                    "baseAddress" => p_addr = try!(read_text(r)),
                    "registers" => { p_registers = Some(try!(read_registers(r))); },
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
                            derived_from: p_derived_from,
                            registers: p_registers,
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
        println!("read_peripherals: {:?}", e);
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
        println!("read_device: {:?}", e);
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
                        return Ok(Device { 
                            name: d_name.unwrap(), 
                            description: d_desc,
                            peripherals: d_periphs.unwrap()
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
        println!("read_document: {:?}", e);
        match e {
            XmlEvent::StartDocument {..} => {},
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "device" => device = Some(try!(read_device(r))),
                    _ => return Err(Error::StateError("Expected device")),
                }
            }
            XmlEvent::EndDocument => {
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
