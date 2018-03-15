#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::uart::*;

periph!( UART1, Uart1, _UART1, UartPeriph, 0x4006b000);
periph!( UART2, Uart2, _UART2, UartPeriph, 0x4006c000);

impl super::sig::Signal<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::SignalTx<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::Signal<super::sig::Uart1Rx> for Uart1 {}
impl super::sig::SignalRx<super::sig::Uart1Rx> for Uart1 {}

impl super::sig::Signal<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::SignalTx<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::Signal<super::sig::Uart2Rx> for Uart2 {}
impl super::sig::SignalRx<super::sig::Uart2Rx> for Uart2 {}



