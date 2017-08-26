#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::uart::*;

periph!(UartPeriph, UART0, Uart0, 0x4006a000);
periph!(UartPeriph, UART1, Uart1, 0x4006b000);
periph!(UartPeriph, UART2, Uart2, 0x4006c000);
periph!(UartPeriph, UART3, Uart3, 0x4006d000);
periph!(UartPeriph, UART4, Uart4, 0x400ea000);
periph!(UartPeriph, UART5, Uart5, 0x400eb000);

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


