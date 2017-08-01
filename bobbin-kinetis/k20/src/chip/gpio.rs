pub use kinetis_common::chip::gpio::*;

pub trait LinkPort<T> {
   fn port(&self) -> T;
}

pub const GPIOA: Gpioa = Periph(0x400ff000, GpioaId {});
pub const GPIOB: Gpiob = Periph(0x400ff040, GpiobId {});
pub const GPIOC: Gpioc = Periph(0x400ff080, GpiocId {});
pub const GPIOD: Gpiod = Periph(0x400ff0c0, GpiodId {});
pub const GPIOE: Gpioe = Periph(0x400ff100, GpioeId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpioaId {}
pub type Gpioa = Periph<GpioaId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiobId {}
pub type Gpiob = Periph<GpiobId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiocId {}
pub type Gpioc = Periph<GpiocId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpiodId {}
pub type Gpiod = Periph<GpiodId>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct GpioeId {}
pub type Gpioe = Periph<GpioeId>;

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



