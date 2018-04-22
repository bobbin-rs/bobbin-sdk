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

/// Access rights for the associated entity. Examples:
/// 
/// ```
/// (access read-only)
/// (access write-only)
/// (access read-write)
/// (access write-once)
/// (access read-write-once)
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Access {
    /// May only be read.
    ReadOnly,
    /// May only be written.
    WriteOnly,
    /// May be read or written.
    ReadWrite,
    /// May be written once.
    WriteOnce,
    /// May be read and written once.
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

/// The top-level item contained in a file.
#[derive(Debug)]
pub enum TopLevel {
    Board(Board),
    Device(Device),
    Peripheral(Peripheral),
}

/// A system containing a MCU and external components.
#[derive(Debug, Default)]
pub struct Board {
    pub name: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub target: String,
    pub mcu: String,
    pub crates: Vec<Crate>,
    pub clocks: Vec<Clock>,
}

/// A device with peripherals.
#[derive(Debug, Default)]
pub struct Device {
    pub vendor: Option<String>,
    pub vendor_id: Option<String>,
    pub name: String,
    pub size: Option<u64>,
    pub access: Option<Access>,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub documentation: Option<String>,    
    pub interrupt_count: Option<u64>,
    pub exceptions: Vec<Exception>,
    pub peripheral_groups: Vec<PeripheralGroup>,
    pub peripherals: Vec<Peripheral>,
    pub crates: Vec<Crate>,
    pub regions: Vec<Region>,
    pub signals: Vec<Signal>,
    pub clocks: Option<Clocks>,
    pub variants: Vec<Variant>,
}

impl Device {
    pub fn interrupt_count(&self) -> u64 {
        if let Some(interrupt_count) = self.interrupt_count {
            interrupt_count
        } else {
            let mut max_irq = 0;
            for p in self.peripherals.iter() {
                for i in p.interrupts.iter() {
                    if i.value > max_irq {
                        max_irq = i.value
                    }
                }
            }
            for pg in self.peripheral_groups.iter() {
                for p in pg.peripherals.iter() {
                    for i in p.interrupts.iter() {
                        if i.value > max_irq {
                            max_irq = i.value
                        }
                    }
                }
            }                                
            // Round to mod 16 to account for missing IRQs and to align code.
            max_irq = (max_irq + 15) & 0xf0;
            max_irq
        }
    }

}

/// A variation of a MCU that may have different memory or flash capacity, peripherals,
/// or other application-visible differences.
#[derive(Debug, Clone, Default)] 
pub struct Variant {
    pub name: String,
    pub link: Option<String>,
    pub description: Option<String>,
}

// #[derive(Debug, Default)] 
// pub struct Connection {
//     pub device_a: String,
//     pub signal_a: String,
//     pub device_b: String,
//     pub signal_b: String,
// }

// #[derive(Debug, Default)] 
// pub struct Path {
//     pub path_elements: Vec<PathElement>,
// }

// #[derive(Debug, Default)] 
// pub struct PathElement {
//     pub device: String,
//     pub signal: String,
// }

/// A Rust crate to reference. Will result in an `extern crate` statement during code generation.
#[derive(Debug, Default)] 
pub struct Crate {
    /// This crate's name.
    pub name: String, 
    /// An optional `as` clause for renaming the crate.
    pub _as: Option<String>, 
    /// A list of modules to import from the crate.
    pub path: Option<String>,
    pub modules: Vec<Module>, 
    pub version: Option<String>,
    pub features: Vec<String>,
    pub default_features: Option<String>,
}

/// A Rust module to import. Will result in a `use` statement during code generation.
#[derive(Debug, Clone, Default)] 
pub struct Module {
    /// This module's name.
    pub name: String,
    /// An optional `as` clause for renaming the module.
    pub _as: Option<String>,
}

/// A group of peripherals sharing the same prototype and defined together within a Rust module.
#[derive(Debug, Default)]
pub struct PeripheralGroup {
    /// This peripheral group's name.
    pub name: String,
    /// Peripherals that belong to this group.
    pub peripherals: Vec<Peripheral>,
    /// Optionally, the prototype to use for peripherals in this group.
    pub prototype: Option<Peripheral>,
    /// Rust modules to import into this group's module.
    pub modules: Vec<Module>,
    /// True if peripherals in this group has associated Pins.
    pub has_pins: bool,
    /// The list of interrupt types associated with this peripheral group.
    pub interrupt_types: Vec<InterruptType>,    
    /// True if peripherals in this group have associated Channels.
    pub has_channels: bool,
    /// Text describing this peripheral group.
    pub description: Option<String>,
}

/// An MCU peripheral, including associated registers, interrupts, signals, pins, and channels.
#[derive(Debug, Clone, Default)]
pub struct Peripheral {
    /// If specified, the name of the peripheral to use as the prototype.
    pub derived_from: Option<String>,
    /// If specified, the name of the group that this peripheral belongs to.
    pub group_name: Option<String>,
    /// The name of the peripheral. Will be used as an identifier during code generation.
    pub name: String,
    /// The base address of the peripheral.
    pub address: u64,
    /// The default size of the peripheral's registers, in bits. Inherits the device's default register size if not specified.
    pub size: Option<u64>,
    /// The default access rights for this peripheral's registers. Inherits the devices's default register access rights if not specified.
    pub access: Option<Access>,
    /// The default reset value for this peripheral's registers. Inherits the devices's default register reset value if not specified.
    pub reset_value: Option<u64>,
    /// The default reset mask for this peripheral's registers. Inherits the devices's default register reset mask if not specified.
    pub reset_mask: Option<u64>,    
    /// The author of this periheral
    pub author: Option<String>,
    /// The version of this periheral
    pub version: Option<String>,
    /// Text describing this peripheral.
    pub description: Option<String>,
    /// Documentation for this peripheral.
    pub documentation: Option<String>,    
    /// The list of Rust modules to import for this peripheral. Currently not used in code generation.
    pub modules: Vec<Module>,
    /// The list of Rust config features associated with this peripheral. Will result in a `#[cfg(any(feature_name))]` during code generation.
    pub features: Vec<String>,
    /// The list of inter-peripheral links associated with this peripheral.
    pub links: Vec<Link>,
    /// The list of address blocks associated with this peripheral.
    pub address_blocks: Vec<AddressBlock>,
    /// The list of interrupt types associated with this peripheral.
    pub interrupt_types: Vec<InterruptType>,
    /// The list of interrupts associated with this peripheral.
    pub interrupts: Vec<Interrupt>,
    /// The list of register clusters associated with this peripheral.
    pub clusters: Vec<Cluster>,
    /// The list of registers associated with this peripheral.
    pub registers: Vec<Register>,
    /// The list of descriptors associated with this peripheral.
    pub descriptors: Vec<Descriptor>,
    /// The list of signals associated with this peripheral.
    pub signals: Vec<Signal>,
    /// The list of pins associated with this peripheral.
    pub pins: Vec<Pin>,
    /// The list of channels associated with this peripheral.
    pub channels: Vec<Channel>,

    /// If specified, the number of instances of this peripheral.
    pub dim: Option<u64>,
    /// If specified, the number of bytes to skip for each instance following the first.
    pub dim_increment: Option<u64>,
    /// Not currently used.
    pub dim_index: Option<String>,

    pub clocks: Vec<Clock>,
}

/// An address range assigned to a peripheral. Example:
/// 
/// ```
/// (address-block
///    (offset 0)
///    (size 0x400)
///    (usage registers)
/// )
/// ```
#[derive(Debug, Clone, Default)]
pub struct AddressBlock {
    /// The start of the address range in bytes relative to the peripheral base address.
    pub offset: u64,
    /// The size of the address range in bytes.
    pub size: u64,
    /// The usage of the address range - may be 'registers', 'buffer' or 'reserved'.
    pub usage: String,
}

/// An in-memory data structure used by a peripheral.
#[derive(Debug, Clone, Default)]
pub struct Descriptor {
    /// This descriptor's symbolic name.
    pub name: String,
    /// The size of the descriptor, in bytes.
    pub size: Option<u64>,
    /// The list of registers in the descriptor.
    pub registers: Vec<Register>,
    /// Text describing the descriptor.
    pub description: Option<String>,
    /// Documentation for this descriptor.
    pub documentation: Option<String>,        
}

/// A device interrupt.
#[derive(Debug, Clone, Default)]
pub struct Interrupt {
    /// The interrupt's symbolic name.
    pub name: String,        
    pub types: Vec<String>,
    /// The IRQ number of this interrupt.
    pub value: u64,
    /// Text describing this interrupt.
    pub description: Option<String>,
    /// Documentation for this interrupt.
    pub documentation: Option<String>,        
}

/// A device interrupt type.
#[derive(Debug, Clone, Default)]
pub struct InterruptType {
    /// The interrupt's symbolic name.
    pub name: String,        
    pub description: Option<String>,
}

/// A logical input or output from a peripheral, channel, or pin.
#[derive(Debug, Clone, Default)]
pub struct Signal {
    /// The signal's symbolic name.
    pub name: String,
    pub types: Vec<String>,
    /// Text describing this signal.
    pub description: Option<String>,
}

/// A device exception
#[derive(Debug, Clone, Default)]
pub struct Exception {    
    /// The name of this exception
    pub name: String,
    /// Text describing this exception.
    pub description: Option<String>,
}

/// A sequence of neighboring registers within a peripheral.
#[derive(Debug, Clone, Default)]
pub struct Cluster {
    /// The symbolic name of the cluster.
    pub name: String,
    /// The address offset in bytes relative to the peripheral base address.
    pub offset: u64,
    /// The size in bytes of this cluster.
    pub size: Option<u64>,
    /// The default access rights for this cluster. Inherits the parent's default access rights if not specified.
    pub access: Option<Access>,
    /// The default reset value for this cluster. Inherits the parent's default reset value if not specified.
    pub reset_value: Option<u64>,
    /// The default reset mask for this cluster. Inherits the parent's default reset mask if not specified.
    pub reset_mask: Option<u64>,
    /// Text describing this cluster.
    pub description: Option<String>,
    /// Documentation for this cluster.
    pub documentation: Option<String>,        

    /// Clusters contained to this cluster.
    pub clusters: Vec<Cluster>,
    /// Registers contained by this cluster.
    pub registers: Vec<Register>,

    /// If specified, the number of cluster instances.
    pub dim: Option<u64>,
    /// If specified, the number of bytes to skip for each instance following the first.
    pub dim_increment: Option<u64>,
    /// Not used.
    pub dim_index: Option<String>,
}

/// A memory-mapped register.
#[derive(Debug, Clone, Default)]
pub struct Register {
    /// The symbolic name of the register.
    pub name: String,
    /// The offset of the register in bytes relative to its parent.
    pub offset: u64,
    /// The size of the register in bits. Inherits the parent's default register size if not specified.
    pub size: Option<u64>,
    /// The access rights of this register. Inherits the parent's default access rights if not specified.
    pub access: Option<Access>,
    /// The reset value of this register. Inherits the parent's default reset value if not specified.
    pub reset_value: Option<u64>,
    /// The reset mask of this register. Inherits the parent's default reset mask if not specified.
    pub reset_mask: Option<u64>,
    /// Text describing the register.
    pub description: Option<String>,
    /// Documentation for this register.
    pub documentation: Option<String>,            
    /// Fields within this register.
    pub fields: Vec<Field>,

    /// If specified, the number of instances of this register.
    pub dim: Option<u64>,
    /// If specified, the number of bytes to skip for each instance following the first.
    pub dim_increment: Option<u64>,
    /// Not used.
    pub dim_index: Option<String>,
}

/// A field within a register.
#[derive(Debug, Clone, Default)]
pub struct Field {
    /// The symbolic name of the field.
    pub name: String,
    /// The offset of the first bit of the field within the register in bits, with the LSB as zero.
    pub bit_offset: u64,
    /// The width of the field in bits.
    pub bit_width: u64,
    /// The access rights of this field. Inherits the register's default access rights if not specified.
    pub access: Option<Access>,
    /// Text describing this field.
    pub description: Option<String>,
    /// Documentation for this field.
    pub documentation: Option<String>,            
    /// A list of valid values for this field.
    pub enumerated_values: Vec<EnumeratedValue>,
    /// A list of intra-device links for this field.
    pub links: Vec<Link>,

    /// If specified, the number of instances of this field.
    pub dim: Option<u64>,
    /// If specified, the number of bits to skip for each instance of this field following the first.
    pub dim_increment: Option<u64>,
    /// Not used.
    pub dim_index: Option<String>,
}

/// An intra-device link.
#[derive(Debug, Clone, Default)]
pub struct Link {
    /// The symbolic name of the link.
    pub name: String,
    /// If not empty, the peripheral group that the entity links to.
    pub peripheral_group: String,
    /// If not empty, the peripheral that the entity links to.
    pub peripheral: String,
    /// If not empty, the channel that the entity links to.
    pub channel: String,
    /// If not empty, the pin that the entity links to.
    pub pin: String,
}

/// An explicitly specified valid value for a field.
#[derive(Debug, Clone, Default)]
pub struct EnumeratedValue {
    pub value: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// A range of address space.
#[derive(Debug, Clone, Default)]
pub struct Region {
    pub name: String,
    pub rtype: String,
    pub offset: u64,
    pub size: u64,
    pub description: Option<String>,
}

/// A logical pin of a MCU.
#[derive(Debug, Clone, Default)]
pub struct Pin {
    /// The symbolic name of the pin.
    pub name: String,
    /// The index of the pin.
    pub index: Option<u64>,
    /// Text describing the pin.
    pub description: Option<String>,
    /// A list of Alternate Functions associated with the pin.
    pub altfns: Vec<AltFn>,
    /// A list of Links associated with the pin.
    pub links: Vec<Link>,
}

/// An Alternate Function for a pin. Example:
/// 
/// ```
/// (pin (name PA0) (index 0)
///    (altfn 1 TIM2_CH1)
///    (altfn 2 TIM5_CH1)
///    (altfn 3 TIM8_ETR)
///    (altfn 7 USART2_CTS)
///    (altfn 8 UART4_TX)
/// )
/// ```
#[derive(Debug, Clone, Default)]
pub struct AltFn {
    /// The index of the alternate function.
    pub index: u64,
    /// The signal associated with the alternate function.
    pub signal: String,
    /// Text describing the alternate function.
    pub description: Option<String>,
}

/// A peripheral channel. Example:
/// 
/// ```
/// (channel
///     (name TIM2_CH1)
///     (index 0)
///     (signal (name TIM2_CH1) (type TIM))
/// )
/// ```
#[derive(Debug, Clone, Default)]
pub struct Channel {
    /// The symbolic name of the channel.
    pub name: String,
    /// The index of the channel.
    pub index: Option<u64>,
    /// Text describing the channel.
    pub description: Option<String>,
    /// Signals associated with this channel.
    pub signals: Vec<Signal>,
    /// Interrupts associated with this channel.
    pub interrupts: Vec<Interrupt>,
}

/// A collection of clock inputs, sources and outputs.
/// 
/// ```
/// (clocks
///     (input (name OSC) (min 4000000) (max 26000000))
///     (input (name OSC32) (min 32768) (max 32768))
///
///     (source (name HSI16) (speed 16000000))
///     (source (name HSE) (input (name OSC)))
///     (source (name MSI))
///     (source (name LSI) (speed 32000))
///     (source (name LSE) (input (name OSC32)))
///     (output (name PLLCLK))
///     (output (name PLL48CLK))
///     (output (name SYSCLK) (max 216000000))
///     (output (name HCLK))
///     (output (name SYSTICK))
///     (output (name PCLK1))
///     (output (name PCLK2))
///     (output (name TIM_PCLK1))
///     (output (name TIM_PCLK2))
///     ...
/// )
/// ```
#[derive(Debug, Clone, Default)]
pub struct Clocks {
    pub inputs: Vec<Clock>,
    pub sources: Vec<Clock>,
    pub outputs: Vec<Clock>,
}


/// An internal or external clock. Example:
/// 
/// ```
/// (clock
///    (name HSI)
///    (speed 16_000_000)
///    (description "High Speed Internal Clock - 16MHz")
/// )
/// ```
#[derive(Debug, Clone, Default)]
pub struct Clock {
    /// The symbolic name of the clock.
    pub name: String,
    pub clock_type: String,
    pub min: Option<u64>,
    pub max: Option<u64>,
    pub speed: Option<u64>,
    pub inputs: Vec<Clock>,
    pub gates: Vec<Gate>,
    /// Text describing the clock.
    pub description: Option<String>,
}

impl Clock {
    pub fn const_id(&self) -> String {
        self.name.to_uppercase()
    }

    pub fn type_id(&self) -> String {
        codegen::to_camel(&self.name)
    }


    pub fn trait_method(&self) -> String {
        self.name.to_lowercase()
    }
}

/// A clock gate.
/// 
/// A clock gate controls whether a clock is connected to the corresponding peripheral.
/// 
///  Example:
/// 
/// ```
/// (clock
///     (input (name PCLK1))
///     (gate (type RST) (periph RCC) (register APB1RSTR1) (field DAC1RST))
///     (gate (type EN) (periph RCC) (register APB1ENR1) (field DAC1EN))
///     (gate (type SLEEP_EN) (periph RCC) (register APB1SMENR1) (field DAC1SMEN))
/// )     
/// ```
#[derive(Debug, Clone, Default)]
pub struct Gate {
    pub name: Option<String>,
    pub gate_type: Option<String>,
    pub periph: Option<String>,
    pub register: Option<String>,
    pub field: Option<String>,
    pub description: Option<String>,
}

impl Device {
    /// Returns the peripheral with the specified name or None if not found.
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
    /// Returns the peripheral with the specified name or None if not found.
    pub fn get_peripheral(&self, name: &str) -> Option<&Peripheral> {
        self.peripherals.iter().find(|p| p.name == name)
    }    
}

impl Peripheral {
    /// Returns a 64 bit signature describing the registers and fields within the peripheral.
    pub fn signature(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }

    /// Constructs the hash used to generate the peripheral's signature.
    pub fn hash(&self, h: &mut Hasher) {
        for c in self.clusters.iter() {
            c.hash(h)
        }
        for r in self.registers.iter() {
            r.hash(h)
        }
    }

    /// Returns an interator for the peripheral instances.
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

impl AddressBlock {
    /// Returns a 64 bit signature describing the address block.
    pub fn signature(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }    

    /// Constructs the hash used for the signature.
    pub fn hash(&self, h: &mut Hasher) {
        h.write_u64(self.offset);
        h.write_u64(self.size);
        h.write(self.usage.as_bytes());
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

#[doc(hidden)]
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