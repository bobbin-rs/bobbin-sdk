#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::uart0::*;

periph!( UART0, Uart0, _UART0, Uart0Periph, 0x4006a000);

impl super::sig::Signal<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::Signal<super::sig::Uart0Rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::Uart0Rx> for Uart0 {}


