pub use stm32_common::chip::lpusart::*;

pub const LPUSART1: Lpusart1 = Periph(0x40004800, Lpusart1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lpusart1Id {}
pub type Lpusart1 = Periph<Lpusart1Id>;

impl super::sig::Signal<super::sig::Lpuart1Tx> for Lpusart1 {}
impl super::sig::SignalTx<super::sig::Lpuart1Tx> for Lpusart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rx> for Lpusart1 {}
impl super::sig::SignalRx<super::sig::Lpuart1Rx> for Lpusart1 {}
impl super::sig::Signal<super::sig::Lpuart1Cts> for Lpusart1 {}
impl super::sig::SignalCts<super::sig::Lpuart1Cts> for Lpusart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rts> for Lpusart1 {}
impl super::sig::SignalRts<super::sig::Lpuart1Rts> for Lpusart1 {}


