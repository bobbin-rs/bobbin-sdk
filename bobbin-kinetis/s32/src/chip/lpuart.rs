#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::lpuart::*;

periph!( LPUART0, Lpuart0, _LPUART0, LpuartPeriph, 0x4006a000);
periph!( LPUART1, Lpuart1, _LPUART1, LpuartPeriph, 0x4006b000);
periph!( LPUART2, Lpuart2, _LPUART2, LpuartPeriph, 0x4006c000);

impl super::sig::Signal<super::sig::Lpuart0Tx> for Lpuart0 {}
impl super::sig::SignalTx<super::sig::Lpuart0Tx> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Rx> for Lpuart0 {}
impl super::sig::SignalRx<super::sig::Lpuart0Rx> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Cts> for Lpuart0 {}
impl super::sig::SignalCts<super::sig::Lpuart0Cts> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Rts> for Lpuart0 {}
impl super::sig::SignalRts<super::sig::Lpuart0Rts> for Lpuart0 {}

impl super::sig::Signal<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::SignalTx<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::SignalRx<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::SignalCts<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rts> for Lpuart1 {}
impl super::sig::SignalRts<super::sig::Lpuart1Rts> for Lpuart1 {}

impl super::sig::Signal<super::sig::Lpuart2Tx> for Lpuart2 {}
impl super::sig::SignalTx<super::sig::Lpuart2Tx> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Rx> for Lpuart2 {}
impl super::sig::SignalRx<super::sig::Lpuart2Rx> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Cts> for Lpuart2 {}
impl super::sig::SignalCts<super::sig::Lpuart2Cts> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Rts> for Lpuart2 {}
impl super::sig::SignalRts<super::sig::Lpuart2Rts> for Lpuart2 {}



pub trait IrqLpuartRxtx<T> {
   fn irq_lpuart_rxtx(&self) -> T;
}

impl IrqLpuartRxtx<super::irq::IrqLpuart0Rxtx> for Lpuart0 {
   fn irq_lpuart_rxtx(&self) -> super::irq::IrqLpuart0Rxtx { super::irq::IRQ_LPUART0_RXTX }
}

impl IrqLpuartRxtx<super::irq::IrqLpuart1Rxtx> for Lpuart1 {
   fn irq_lpuart_rxtx(&self) -> super::irq::IrqLpuart1Rxtx { super::irq::IRQ_LPUART1_RXTX }
}

impl IrqLpuartRxtx<super::irq::IrqLpuart2Rxtx> for Lpuart2 {
   fn irq_lpuart_rxtx(&self) -> super::irq::IrqLpuart2Rxtx { super::irq::IRQ_LPUART2_RXTX }
}

