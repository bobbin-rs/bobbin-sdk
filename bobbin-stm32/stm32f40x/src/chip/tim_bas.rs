#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_bas::*;

periph!( TIM6, Tim6, _TIM6, TimBasPeriph, 0x40001000);
periph!( TIM7, Tim7, _TIM7, TimBasPeriph, 0x40001400);





pub trait IrqTim<T> {
    fn irq_tim(&self) -> T;
}

impl IrqTim<super::irq::IrqTim6> for Tim6 {
    fn irq_tim(&self) -> super::irq::IrqTim6 { super::irq::IRQ_TIM6 }
}

impl IrqTim<super::irq::IrqTim7> for Tim7 {
    fn irq_tim(&self) -> super::irq::IrqTim7 { super::irq::IRQ_TIM7 }
}

