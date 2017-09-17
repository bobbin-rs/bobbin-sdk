#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::lpuart::*;

periph!( LPUART1, Lpuart1, _LPUART1, LpuartPeriph, 0x40008000);

impl super::sig::Signal<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::SignalTx<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::SignalRx<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::SignalCts<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rts> for Lpuart1 {}
impl super::sig::SignalRts<super::sig::Lpuart1Rts> for Lpuart1 {}



pub trait IrqLpuart<T> {
    fn irq_lpuart(&self) -> T;
}

impl IrqLpuart<super::irq::IrqLpuart1> for Lpuart1 {
    fn irq_lpuart(&self) -> super::irq::IrqLpuart1 { super::irq::IRQ_LPUART1 }
}

