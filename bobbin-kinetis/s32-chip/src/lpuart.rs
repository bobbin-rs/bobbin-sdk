pub use kinetis_common::chip::lpuart::*;

pub const LPUART0: Lpuart0 = Periph(0x4006a000, Lpuart0Id {});
pub const LPUART1: Lpuart1 = Periph(0x4006b000, Lpuart1Id {});
pub const LPUART2: Lpuart2 = Periph(0x4006c000, Lpuart2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpuart0Id {}
pub type Lpuart0 = Periph<Lpuart0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpuart1Id {}
pub type Lpuart1 = Periph<Lpuart1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Lpuart2Id {}
pub type Lpuart2 = Periph<Lpuart2Id>;

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


