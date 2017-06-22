pub use stm32_common::chip::lpuart::*;

pub const LPUART1: Lpuart1 = Periph(0x40004800, Lpuart1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lpuart1Id {}
pub type Lpuart1 = Periph<Lpuart1Id>;

impl super::sig::Signal<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::SignalTx<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::SignalRx<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::SignalCts<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rts> for Lpuart1 {}
impl super::sig::SignalRts<super::sig::Lpuart1Rts> for Lpuart1 {}


