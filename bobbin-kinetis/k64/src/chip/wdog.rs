#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::wdog::*;

periph!( WDOG, Wdog, _WDOG, WdogPeriph, 0x40052000);




pub trait IrqWdog<T> {
    fn irq_wdog(&self) -> T;
}

impl IrqWdog<super::irq::IrqWdog> for Wdog {
    fn irq_wdog(&self) -> super::irq::IrqWdog { super::irq::IRQ_WDOG }
}

