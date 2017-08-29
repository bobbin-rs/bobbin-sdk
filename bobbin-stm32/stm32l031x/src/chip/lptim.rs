#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::lptim::*;

periph!( LPTIM, Lptim, _LPTIM, LptimPeriph, 0x40007c00);




pub trait IrqLptim<T> {
   fn irq_lptim(&self) -> T;
}

impl IrqLptim<super::irq::IrqLptim1> for Lptim {
   fn irq_lptim(&self) -> super::irq::IrqLptim1 { super::irq::IRQ_LPTIM1 }
}

