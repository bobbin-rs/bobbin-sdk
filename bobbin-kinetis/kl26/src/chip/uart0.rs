pub use kinetis_common::chip::uart0::*;

pub const UART0: Uart0 = Periph(0x4006a000, Uart0Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart0Id {}
pub type Uart0 = Periph<Uart0Id>;

impl super::sig::Signal<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::Signal<super::sig::Uart0Rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::Uart0Rx> for Uart0 {}


