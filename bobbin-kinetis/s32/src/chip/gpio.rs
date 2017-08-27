#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "GPIO", peripherals: [Peripheral { derived_from: None, group_name: None, name: "GPIOA", address: 1074786304, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General Purpose Input/Output"), modules: [], features: [], links: [Link { name: "PORT", peripheral_group: "PORT", peripheral: "PORTA", channel: "" }], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOB", address: 1074786368, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General Purpose Input/Output"), modules: [], features: [], links: [Link { name: "PORT", peripheral_group: "PORT", peripheral: "PORTB", channel: "" }], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOC", address: 1074786432, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General Purpose Input/Output"), modules: [], features: [], links: [Link { name: "PORT", peripheral_group: "PORT", peripheral: "PORTC", channel: "" }], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOD", address: 1074786496, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General Purpose Input/Output"), modules: [], features: [], links: [Link { name: "PORT", peripheral_group: "PORT", peripheral: "PORTD", channel: "" }], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "GPIOE", address: 1074786560, size: None, access: None, reset_value: None, reset_mask: None, description: Some("General Purpose Input/Output"), modules: [], features: [], links: [Link { name: "PORT", peripheral_group: "PORT", peripheral: "PORTE", channel: "" }], interrupts: [], clusters: [], registers: [], descriptors: [], signals: [], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::gpio::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::gpio::*;

pub trait LinkPort<T> {
   fn port(&self) -> T;
}

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x400ff000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x400ff040);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x400ff080);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x400ff0c0);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x400ff100);

impl LinkPort<super::port::Periph<super::port::PortaId>> for Gpioa {
   fn port(&self) -> super::port::Periph<super::port::PortaId> { super::port::PORTA }
}


impl LinkPort<super::port::Periph<super::port::PortbId>> for Gpiob {
   fn port(&self) -> super::port::Periph<super::port::PortbId> { super::port::PORTB }
}


impl LinkPort<super::port::Periph<super::port::PortcId>> for Gpioc {
   fn port(&self) -> super::port::Periph<super::port::PortcId> { super::port::PORTC }
}


impl LinkPort<super::port::Periph<super::port::PortdId>> for Gpiod {
   fn port(&self) -> super::port::Periph<super::port::PortdId> { super::port::PORTD }
}


impl LinkPort<super::port::Periph<super::port::PorteId>> for Gpioe {
   fn port(&self) -> super::port::Periph<super::port::PorteId> { super::port::PORTE }
}



