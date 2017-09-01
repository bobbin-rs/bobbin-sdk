#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::wwdg::*;

periph!( WWDG, Wwdg, _WWDG, WwdgPeriph, 0x40002c00);




pub trait IrqWwdg<T> {
    fn irq_wwdg(&self) -> T;
}

impl IrqWwdg<super::irq::IrqWwdg> for Wwdg {
    fn irq_wwdg(&self) -> super::irq::IrqWwdg { super::irq::IRQ_WWDG }
}

