pub use kinetis_common::chip::uart::*;

pub const UART1: Uart1 = Periph(0x4006b000, Uart1Id {});
pub const UART2: Uart2 = Periph(0x4006c000, Uart2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart1Id {}
pub type Uart1 = Periph<Uart1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart2Id {}
pub type Uart2 = Periph<Uart2Id>;

impl super::sig::Signal<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::SignalTx<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::Signal<super::sig::Uart1Rx> for Uart1 {}
impl super::sig::SignalRx<super::sig::Uart1Rx> for Uart1 {}

impl super::sig::Signal<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::SignalTx<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::Signal<super::sig::Uart2Rx> for Uart2 {}
impl super::sig::SignalRx<super::sig::Uart2Rx> for Uart2 {}


