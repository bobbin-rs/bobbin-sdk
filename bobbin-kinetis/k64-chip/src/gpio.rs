pub use kinetis_common::chip::gpio::*;

pub trait PortLink {
   fn port(&self) -> &::core::ops::Deref<Target=super::port::PortImpl>;
}

pub const GPIOA: Gpioa = Gpioa {};
pub const GPIOA_REF: &Gpioa = &GPIOA;
pub const GPIOA_IMPL: GpioImpl = GpioImpl(0x400ff000);
pub const GPIOA_IMPL_REF: &GpioImpl = &GPIOA_IMPL;

pub struct Gpioa {}
impl ::core::ops::Deref for Gpioa {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOA_IMPL_REF }
}

impl PortLink for Gpioa {
   fn port(&self) -> &::core::ops::Deref<Target=super::port::PortImpl> { super::port::PORTA_REF }
}


pub const GPIOB: Gpiob = Gpiob {};
pub const GPIOB_REF: &Gpiob = &GPIOB;
pub const GPIOB_IMPL: GpioImpl = GpioImpl(0x400ff040);
pub const GPIOB_IMPL_REF: &GpioImpl = &GPIOB_IMPL;

pub struct Gpiob {}
impl ::core::ops::Deref for Gpiob {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOB_IMPL_REF }
}

impl PortLink for Gpiob {
   fn port(&self) -> &::core::ops::Deref<Target=super::port::PortImpl> { super::port::PORTB_REF }
}


pub const GPIOC: Gpioc = Gpioc {};
pub const GPIOC_REF: &Gpioc = &GPIOC;
pub const GPIOC_IMPL: GpioImpl = GpioImpl(0x400ff080);
pub const GPIOC_IMPL_REF: &GpioImpl = &GPIOC_IMPL;

pub struct Gpioc {}
impl ::core::ops::Deref for Gpioc {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOC_IMPL_REF }
}

impl PortLink for Gpioc {
   fn port(&self) -> &::core::ops::Deref<Target=super::port::PortImpl> { super::port::PORTC_REF }
}


pub const GPIOD: Gpiod = Gpiod {};
pub const GPIOD_REF: &Gpiod = &GPIOD;
pub const GPIOD_IMPL: GpioImpl = GpioImpl(0x400ff0c0);
pub const GPIOD_IMPL_REF: &GpioImpl = &GPIOD_IMPL;

pub struct Gpiod {}
impl ::core::ops::Deref for Gpiod {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOD_IMPL_REF }
}

impl PortLink for Gpiod {
   fn port(&self) -> &::core::ops::Deref<Target=super::port::PortImpl> { super::port::PORTD_REF }
}


pub const GPIOE: Gpioe = Gpioe {};
pub const GPIOE_REF: &Gpioe = &GPIOE;
pub const GPIOE_IMPL: GpioImpl = GpioImpl(0x400ff100);
pub const GPIOE_IMPL_REF: &GpioImpl = &GPIOE_IMPL;

pub struct Gpioe {}
impl ::core::ops::Deref for Gpioe {
   type Target = GpioImpl;
   #[inline]
   fn deref(&self) -> &GpioImpl { GPIOE_IMPL_REF }
}

impl PortLink for Gpioe {
   fn port(&self) -> &::core::ops::Deref<Target=super::port::PortImpl> { super::port::PORTE_REF }
}



