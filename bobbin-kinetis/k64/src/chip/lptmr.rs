#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::lptmr::*;

periph!( LPTMR0, Lptmr0, _LPTMR0, LptmrPeriph, 0x40040000);




pub trait IrqLptmr<T> {
    fn irq_lptmr(&self) -> T;
}

impl IrqLptmr<super::irq::IrqLptmr0> for Lptmr0 {
    fn irq_lptmr(&self) -> super::irq::IrqLptmr0 { super::irq::IRQ_LPTMR0 }
}

