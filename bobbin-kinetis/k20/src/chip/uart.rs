pub use kinetis_common::chip::uart::*;

pub const UART0: Uart0 = Periph(0x4006a000, Uart0Id {});
pub const UART1: Uart1 = Periph(0x4006b000, Uart1Id {});
pub const UART2: Uart2 = Periph(0x4006c000, Uart2Id {});
pub const UART3: Uart3 = Periph(0x4006d000, Uart3Id {});
pub const UART4: Uart4 = Periph(0x400ea000, Uart4Id {});
pub const UART5: Uart5 = Periph(0x400eb000, Uart5Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart0Id {}
pub type Uart0 = Periph<Uart0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart1Id {}
pub type Uart1 = Periph<Uart1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart2Id {}
pub type Uart2 = Periph<Uart2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart3Id {}
pub type Uart3 = Periph<Uart3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart4Id {}
pub type Uart4 = Periph<Uart4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Uart5Id {}
pub type Uart5 = Periph<Uart5Id>;

impl super::sig::Signal<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::Signal<super::sig::Uart0Rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::Uart0Rx> for Uart0 {}

impl super::sig::Signal<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::SignalTx<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::Signal<super::sig::Uart1Rx> for Uart1 {}
impl super::sig::SignalRx<super::sig::Uart1Rx> for Uart1 {}

impl super::sig::Signal<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::SignalTx<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::Signal<super::sig::Uart2Rx> for Uart2 {}
impl super::sig::SignalRx<super::sig::Uart2Rx> for Uart2 {}

impl super::sig::Signal<super::sig::Uart3Tx> for Uart3 {}
impl super::sig::SignalTx<super::sig::Uart3Tx> for Uart3 {}
impl super::sig::Signal<super::sig::Uart3Rx> for Uart3 {}
impl super::sig::SignalRx<super::sig::Uart3Rx> for Uart3 {}

impl super::sig::Signal<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::Uart4Rx> for Uart4 {}

impl super::sig::Signal<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::Uart5Rx> for Uart5 {}


