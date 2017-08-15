#[allow(unused_imports)] use bobbin_common::bits;
pub use stm32_common::chip::adc_f124::*;

pub const ADC1: Adc1 = Periph(0x40012000, Adc1Id {});
pub const ADC2: Adc2 = Periph(0x40012100, Adc2Id {});
pub const ADC3: Adc3 = Periph(0x40012200, Adc3Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc1Id {}
pub type Adc1 = Periph<Adc1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc2Id {}
pub type Adc2 = Periph<Adc2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc3Id {}
pub type Adc3 = Periph<Adc3Id>;





pub trait IrqAdc<T> {
   fn irq_adc(&self) -> super::irq::Irq<T>;
}

pub trait RegisterAdcHandler {
   fn register_adc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleAdc>(&self, f: &F) -> super::irq::IrqGuard<'a>;
}

pub trait HandleAdc {
   fn handle_adc(&self);
}

impl IrqAdc<super::irq::Adc1Id> for Adc1 {
   fn irq_adc(&self) -> super::irq::IrqAdc1 { super::irq::IRQ_ADC1 }
}

impl RegisterAdcHandler for Adc1 {
   fn register_adc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleAdc>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleAdc>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_adc() }
       }
       super::irq::set_handler(18, Some(wrapper::<F>));
       super::irq::IrqGuard::new(18)
   }
}

impl IrqAdc<super::irq::Adc2Id> for Adc2 {
   fn irq_adc(&self) -> super::irq::IrqAdc2 { super::irq::IRQ_ADC2 }
}

impl RegisterAdcHandler for Adc2 {
   fn register_adc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleAdc>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleAdc>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_adc() }
       }
       super::irq::set_handler(18, Some(wrapper::<F>));
       super::irq::IrqGuard::new(18)
   }
}

impl IrqAdc<super::irq::Adc3Id> for Adc3 {
   fn irq_adc(&self) -> super::irq::IrqAdc3 { super::irq::IRQ_ADC3 }
}

impl RegisterAdcHandler for Adc3 {
   fn register_adc_handler<'a, F: ::core::marker::Sync + ::core::marker::Send + HandleAdc>(&self, f: &F) -> super::irq::IrqGuard<'a> {
       static mut HANDLER: Option<usize> = None;
       unsafe { HANDLER = Some(f as *const F as usize) }
       extern "C" fn wrapper<W: HandleAdc>() {
          unsafe { (*(HANDLER.unwrap() as *const W)).handle_adc() }
       }
       super::irq::set_handler(18, Some(wrapper::<F>));
       super::irq::IrqGuard::new(18)
   }
}

