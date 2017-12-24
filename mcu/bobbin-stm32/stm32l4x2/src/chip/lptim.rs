#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::lptim::*;

periph!( LPTIM1, Lptim1, _LPTIM1, LptimPeriph, 0x40007c00);
periph!( LPTIM2, Lptim2, _LPTIM2, LptimPeriph, 0x40009400);





pub trait IrqLptim<T> {
    fn irq_lptim(&self) -> T;
}

impl IrqLptim<super::irq::IrqLptim1> for Lptim1 {
    fn irq_lptim(&self) -> super::irq::IrqLptim1 { super::irq::IRQ_LPTIM1 }
}

impl IrqLptim<super::irq::IrqLptim2> for Lptim2 {
    fn irq_lptim(&self) -> super::irq::IrqLptim2 { super::irq::IRQ_LPTIM2 }
}

