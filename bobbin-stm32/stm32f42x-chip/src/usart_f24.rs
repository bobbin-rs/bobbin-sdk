pub use stm32_common::chip::usart_f24::*;

pub const USART1: Usart1 = Periph(0x40011000, Usart1Id {});
pub const USART2: Usart2 = Periph(0x40004400, Usart2Id {});
pub const USART3: Usart3 = Periph(0x40004800, Usart3Id {});
pub const USART6: Usart6 = Periph(0x40011400, Usart6Id {});
pub const UART7: Uart7 = Periph(0x40007800, Uart7Id {});
pub const UART8: Uart8 = Periph(0x40007c00, Uart8Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Usart1Id {}
pub type Usart1 = Periph<Usart1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Usart2Id {}
pub type Usart2 = Periph<Usart2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Usart3Id {}
pub type Usart3 = Periph<Usart3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Usart6Id {}
pub type Usart6 = Periph<Usart6Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Uart7Id {}
pub type Uart7 = Periph<Uart7Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Uart8Id {}
pub type Uart8 = Periph<Uart8Id>;

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

impl super::sig::Signal<super::sig::Uart7Tx> for Uart7 {}
impl super::sig::SignalTx<super::sig::Uart7Tx> for Uart7 {}
impl super::sig::Signal<super::sig::Uart7Rx> for Uart7 {}
impl super::sig::SignalRx<super::sig::Uart7Rx> for Uart7 {}
impl super::sig::Signal<super::sig::Uart7Cts> for Uart7 {}
impl super::sig::SignalCts<super::sig::Uart7Cts> for Uart7 {}
impl super::sig::Signal<super::sig::Uart7Rts> for Uart7 {}
impl super::sig::SignalRts<super::sig::Uart7Rts> for Uart7 {}
impl super::sig::Signal<super::sig::Uart7Ck> for Uart7 {}
impl super::sig::SignalCk<super::sig::Uart7Ck> for Uart7 {}

impl super::sig::Signal<super::sig::Uart8Tx> for Uart8 {}
impl super::sig::SignalTx<super::sig::Uart8Tx> for Uart8 {}
impl super::sig::Signal<super::sig::Uart8Rx> for Uart8 {}
impl super::sig::SignalRx<super::sig::Uart8Rx> for Uart8 {}
impl super::sig::Signal<super::sig::Uart8Cts> for Uart8 {}
impl super::sig::SignalCts<super::sig::Uart8Cts> for Uart8 {}
impl super::sig::Signal<super::sig::Uart8Rts> for Uart8 {}
impl super::sig::SignalRts<super::sig::Uart8Rts> for Uart8 {}
impl super::sig::Signal<super::sig::Uart8Ck> for Uart8 {}
impl super::sig::SignalCk<super::sig::Uart8Ck> for Uart8 {}


