pub use stm32_common::chip::usart_f24::*;

pub const USART1: Usart1 = Periph(0x40011000, Usart1Id {});
pub const USART2: Usart2 = Periph(0x40004400, Usart2Id {});
pub const USART3: Usart3 = Periph(0x40004800, Usart3Id {});
pub const UART4: Uart4 = Periph(0x40004c00, Uart4Id {});
pub const UART5: Uart5 = Periph(0x40005000, Uart5Id {});
pub const USART6: Usart6 = Periph(0x40011400, Usart6Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Usart1Id {}
pub type Usart1 = Periph<Usart1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Usart2Id {}
pub type Usart2 = Periph<Usart2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Usart3Id {}
pub type Usart3 = Periph<Usart3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart4Id {}
pub type Uart4 = Periph<Uart4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart5Id {}
pub type Uart5 = Periph<Uart5Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Usart6Id {}
pub type Usart6 = Periph<Usart6Id>;

impl super::sig::Signal<super::sig::Usart1Tx> for Usart1 {}
impl super::sig::SignalTx<super::sig::Usart1Tx> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Rx> for Usart1 {}
impl super::sig::SignalRx<super::sig::Usart1Rx> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Cts> for Usart1 {}
impl super::sig::SignalCts<super::sig::Usart1Cts> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Rts> for Usart1 {}
impl super::sig::SignalRts<super::sig::Usart1Rts> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Ck> for Usart1 {}
impl super::sig::SignalCk<super::sig::Usart1Ck> for Usart1 {}

impl super::sig::Signal<super::sig::Usart2Tx> for Usart2 {}
impl super::sig::SignalTx<super::sig::Usart2Tx> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Rx> for Usart2 {}
impl super::sig::SignalRx<super::sig::Usart2Rx> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Cts> for Usart2 {}
impl super::sig::SignalCts<super::sig::Usart2Cts> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Rts> for Usart2 {}
impl super::sig::SignalRts<super::sig::Usart2Rts> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Ck> for Usart2 {}
impl super::sig::SignalCk<super::sig::Usart2Ck> for Usart2 {}

impl super::sig::Signal<super::sig::Usart3Tx> for Usart3 {}
impl super::sig::SignalTx<super::sig::Usart3Tx> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Rx> for Usart3 {}
impl super::sig::SignalRx<super::sig::Usart3Rx> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Cts> for Usart3 {}
impl super::sig::SignalCts<super::sig::Usart3Cts> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Rts> for Usart3 {}
impl super::sig::SignalRts<super::sig::Usart3Rts> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Ck> for Usart3 {}
impl super::sig::SignalCk<super::sig::Usart3Ck> for Usart3 {}

impl super::sig::Signal<super::sig::Usart4Tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::Usart4Tx> for Uart4 {}
impl super::sig::Signal<super::sig::Usart4Rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::Usart4Rx> for Uart4 {}
impl super::sig::Signal<super::sig::Usart4Cts> for Uart4 {}
impl super::sig::SignalCts<super::sig::Usart4Cts> for Uart4 {}
impl super::sig::Signal<super::sig::Usart4Rts> for Uart4 {}
impl super::sig::SignalRts<super::sig::Usart4Rts> for Uart4 {}
impl super::sig::Signal<super::sig::Usart4Ck> for Uart4 {}
impl super::sig::SignalCk<super::sig::Usart4Ck> for Uart4 {}

impl super::sig::Signal<super::sig::Usart5Tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::Usart5Tx> for Uart5 {}
impl super::sig::Signal<super::sig::Usart5Rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::Usart5Rx> for Uart5 {}
impl super::sig::Signal<super::sig::Usart5Cts> for Uart5 {}
impl super::sig::SignalCts<super::sig::Usart5Cts> for Uart5 {}
impl super::sig::Signal<super::sig::Usart5Rts> for Uart5 {}
impl super::sig::SignalRts<super::sig::Usart5Rts> for Uart5 {}
impl super::sig::Signal<super::sig::Usart5Ck> for Uart5 {}
impl super::sig::SignalCk<super::sig::Usart5Ck> for Uart5 {}

impl super::sig::Signal<super::sig::Usart6Tx> for Usart6 {}
impl super::sig::SignalTx<super::sig::Usart6Tx> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Rx> for Usart6 {}
impl super::sig::SignalRx<super::sig::Usart6Rx> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Cts> for Usart6 {}
impl super::sig::SignalCts<super::sig::Usart6Cts> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Rts> for Usart6 {}
impl super::sig::SignalRts<super::sig::Usart6Rts> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Ck> for Usart6 {}
impl super::sig::SignalCk<super::sig::Usart6Ck> for Usart6 {}

