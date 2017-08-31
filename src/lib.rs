extern crate bobbin_sexp as sexp;
extern crate bobbin_sexp_tokenizer as sexp_tokenizer;
extern crate clap;

use std::fmt;
use std::collections::hash_map::{HashMap, DefaultHasher};
use std::hash::{Hasher};

pub mod reader;
pub mod writer;
pub mod codegen;
pub mod builder;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    WriteOnce,
    ReadWriteOnce,
}

impl Access {
    pub fn is_readable(&self) -> bool {
        match self {
            &Access::WriteOnly => false,
            _ => true,
        }
    }
    pub fn is_writable(&self) -> bool {
        match self {
            &Access::ReadOnly => false,
            _ => true,
        }
    }
}

impl Default for Access {
    fn default() -> Self {
        Access::ReadWrite
    }
}

impl fmt::Display for Access {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Access::*;
        match self {
            &ReadOnly => write!(f, "read-only"),
            &WriteOnly => write!(f, "write-only"),
            &ReadWrite => write!(f, "read-write"),
            &WriteOnce => write!(f, "write-once"),
            &ReadWriteOnce => write!(f, "read-writeOnce"),
        }
    }
}

impl<'a> From<&'a str> for Access {
    fn from(other: &str) -> Self {
        match other {
            "read-only" => Access::ReadOnly,
            "write-only" => Access::WriteOnly,
            "read-write" => Access::ReadWrite,
            "write-once" => Access::WriteOnce,
            "read-writeOnce" => Access::ReadWriteOnce,
            _ => panic!("Unexpected access type: {:?}", other),
        }
    }
}

impl From<String> for Access {
    fn from(other: String) -> Self {
        Access::from(other.as_ref())
    }
}
#[derive(Debug)]
pub enum TopLevel {
    Board(Board),
    Device(Device),
    Peripheral(Peripheral),
}

#[derive(Debug, Default)]
pub struct Board {
    pub name: String,
    pub description: Option<String>,
    pub devices: Vec<Device>,
    pub connections: Vec<Connection>,
    pub paths: Vec<Path>,
    pub clocks: Vec<Clock>,
}

#[derive(Debug, Default)]
pub struct Device {
    pub vendor: Option<String>,
    pub vendor_id: Option<String>,
    pub name: String,
    pub size: Option<u64>,
    pub access: Option<Access>,
    pub description: Option<String>,
    pub interrupt_count: Option<u64>,
    pub exceptions: Vec<Exception>,
    pub peripheral_groups: Vec<PeripheralGroup>,
    pub peripherals: Vec<Peripheral>,
    pub crates: Vec<Crate>,
    pub regions: Vec<Region>,
    pub signals: Vec<Signal>,
    pub clocks: Vec<Clock>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, Default)] 
pub struct Variant {
    pub name: String,
    pub link: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Default)] 
pub struct Connection {
    pub device_a: String,
    pub signal_a: String,
    pub device_b: String,
    pub signal_b: String,
}

#[derive(Debug, Default)] 
pub struct Path {
    pub path_elements: Vec<PathElement>,
}

#[derive(Debug, Default)] 
pub struct PathElement {
    pub device: String,
    pub signal: String,
}


#[derive(Debug, Default)] 
pub struct Crate {
    pub name: String,
    pub modules: Vec<Module>,
}


#[derive(Debug, Clone, Default)] 
pub struct Module {
    pub name: String,
    pub _as: Option<String>,
}

#[derive(Debug, Default)]
pub struct PeripheralGroup {
    pub name: String,
    pub peripherals: Vec<Peripheral>,
    pub prototype: Option<Peripheral>,
    pub modules: Vec<Module>,
    pub has_pins: bool,
    pub has_channels: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Peripheral {
    pub derived_from: Option<String>,
    pub group_name: Option<String>,
    pub name: String,
    pub address: u64,
    pub size: Option<u64>,
    pub access: Option<Access>,
    pub reset_value: Option<u64>,
    pub reset_mask: Option<u64>,    
    pub description: Option<String>,
    pub modules: Vec<Module>,
    pub features: Vec<String>,
    pub links: Vec<Link>,

    pub interrupts: Vec<Interrupt>,
    pub clusters: Vec<Cluster>,
    pub registers: Vec<Register>,
    pub descriptors: Vec<Descriptor>,
    pub signals: Vec<Signal>,
    pub pins: Vec<Pin>,
    pub channels: Vec<Channel>,

    pub dim: Option<u64>,
    pub dim_increment: Option<u64>,
    pub dim_index: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Descriptor {
    pub name: String,
    pub size: Option<u64>,
    pub registers: Vec<Register>,
    pub description: Option<String>,    
}

#[derive(Debug, Clone, Default)]
pub struct Interrupt {
    pub name: String,
    pub types: Vec<String>,
    pub value: u64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Signal {
    pub name: String,
    pub types: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Exception {    
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Cluster {
    pub name: String,
    pub offset: u64,
    pub size: Option<u64>,
    pub access: Option<Access>,
    pub reset_value: Option<u64>,
    pub reset_mask: Option<u64>,
    pub description: Option<String>,

    pub clusters: Vec<Cluster>,
    pub registers: Vec<Register>,

    pub dim: Option<u64>,
    pub dim_increment: Option<u64>,
    pub dim_index: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Register {
    pub name: String,
    pub offset: u64,
    pub size: Option<u64>,
    pub access: Option<Access>,
    pub reset_value: Option<u64>,
    pub reset_mask: Option<u64>,
    pub description: Option<String>,
    
    pub fields: Vec<Field>,

    pub dim: Option<u64>,
    pub dim_increment: Option<u64>,
    pub dim_index: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Field {
    pub name: String,
    pub bit_offset: u64,
    pub bit_width: u64,
    pub access: Option<Access>,
    pub description: Option<String>,
    pub enumerated_values: Vec<EnumeratedValue>,
    pub links: Vec<Link>,

    pub dim: Option<u64>,
    pub dim_increment: Option<u64>,
    pub dim_index: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Link {
    pub name: String,
    pub peripheral_group: String,
    pub peripheral: String,
    pub channel: String,
    pub pin: String,
}

#[derive(Debug, Clone, Default)]
pub struct EnumeratedValue {
    pub value: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Region {
    pub name: String,
    pub rtype: String,
    pub offset: u64,
    pub size: u64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Pin {
    pub name: String,
    pub index: Option<u64>,
    pub description: Option<String>,
    pub altfns: Vec<AltFn>,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, Default)]
pub struct AltFn {
    pub index: u64,
    pub signal: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Channel {
    pub name: String,
    pub index: Option<u64>,
    pub description: Option<String>,
    pub signals: Vec<Signal>,
    pub interrupts: Vec<Interrupt>,
}

#[derive(Debug, Clone, Default)]
pub struct Clock {
    pub name: String,
    pub speed: Option<u64>,
    pub description: Option<String>,
}


impl Device {
    pub fn get_peripheral(&self, name: &str) -> Option<&Peripheral> {
        if let Some(p) = self.peripherals.iter().find(|p| p.name == name) {
            return Some(p)
        }
        for pg in self.peripheral_groups.iter() {
            if let Some(p) = pg.get_peripheral(name) {
                return Some(p)
            }
        }
        None
    }

    pub fn make_groups(&self) -> HashMap<u64, Vec<String>> {
        let mut groups: HashMap<u64, Vec<String>> = HashMap::new();
        for p in self.peripherals.iter() {
            let h = if p.derived_from.is_some() {
                self.get_peripheral(p.derived_from.as_ref().unwrap()).unwrap().signature()
            } else {
                p.signature()
            };            
            

            let v = groups.entry(h).or_insert_with(|| Vec::new());
            v.push(p.name.clone());
        }
        groups
    }

}

impl PeripheralGroup {
    pub fn get_peripheral(&self, name: &str) -> Option<&Peripheral> {
        self.peripherals.iter().find(|p| p.name == name)
    }    
}

impl Peripheral {    
    pub fn signature(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }

    pub fn hash(&self, h: &mut Hasher) {
        for c in self.clusters.iter() {
            c.hash(h)
        }
        for r in self.registers.iter() {
            r.hash(h)
        }
    }

    pub fn iter_dim(&self) -> DimIter {
        if let Some(dim) = self.dim {
            DimIter { 
                index: 0, 
                name: &self.name,
                base: self.address,
                dim: dim, 
                dim_increment: self.dim_increment.unwrap() 
            }
        } else {
            DimIter { 
                index: 0, 
                name: &self.name,
                base: self.address,
                dim: 1, 
                dim_increment: 0,
            }            
        }
    }    
}

impl Cluster {
    pub fn signature(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }    

    pub fn hash(&self, h: &mut Hasher) {
        for f in self.clusters.iter() {
            f.hash(h)
        }
        for f in self.registers.iter() {
            f.hash(h)
        }
    }

    pub fn iter_dim(&self) -> DimIter {
        if let Some(dim) = self.dim {
            DimIter { 
                index: 0, 
                name: &self.name,
                base: self.offset,
                dim: dim, 
                dim_increment: self.dim_increment.unwrap() 
            }
        } else {
            DimIter { 
                index: 0, 
                name: &self.name,
                base: self.offset,
                dim: 1, 
                dim_increment: 0,
            }            
        }
    }    
}

impl Register {
    pub fn signature(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }

    pub fn hash(&self, h: &mut Hasher) {
        for f in self.fields.iter() {
            f.hash(h)
        }
    }

    pub fn iter_dim(&self) -> DimIter {
        if let Some(dim) = self.dim {
            DimIter { 
                index: 0, 
                name: &self.name,
                base: self.offset,
                dim: dim, 
                dim_increment: self.dim_increment.unwrap() 
            }
        } else {
            DimIter { 
                index: 0, 
                name: &self.name,
                base: self.offset,
                dim: 1, 
                dim_increment: 0,
            }            
        }
    }

}

impl Field {
    pub fn signature(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }

    pub fn hash(&self, h: &mut Hasher) {
        h.write(&self.name.as_bytes());
        h.write_u64(self.bit_offset);
        h.write_u64(self.bit_width);
    }

    pub fn iter_dim(&self) -> DimIter {
        if let Some(dim) = self.dim {
            DimIter { 
                index: 0, 
                name: &self.name,
                base: self.bit_offset,
                dim: dim, 
                dim_increment: self.dim_increment.unwrap() 
            }
        } else {
            DimIter { 
                index: 0, 
                name: &self.name,
                base: self.bit_offset,
                dim: 1, 
                dim_increment: 0,
            }            
        }
    }      
}

pub struct DimIter<'a> {
    index: u64,
    name: &'a str,
    base: u64,
    dim: u64,
    dim_increment: u64,
}

impl<'a> Iterator for DimIter<'a> {
    type Item = (u64, String);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.dim {
            return None
        }
        let name = self.name.clone().replace("%s", &format!("{}", self.index));

        let r = (self.base + (self.dim_increment * self.index), name);
        self.index += 1;
        return Some(r)
    }
}

pub fn group_name(periphs: &[Peripheral]) -> Option<&str> {
    for p in periphs.iter() {
        if let Some(ref group_name) = p.group_name {
            return Some(group_name)
        }
    }
    None
}