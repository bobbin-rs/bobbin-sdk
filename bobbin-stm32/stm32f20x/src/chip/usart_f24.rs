#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::usart_f24::*;

periph!( USART1, Usart1, _USART1, UsartF24Periph, 0x40011000);
periph!( USART2, Usart2, _USART2, UsartF24Periph, 0x40004400);
periph!( USART3, Usart3, _USART3, UsartF24Periph, 0x40004800);
periph!( UART4, Uart4, _UART4, UsartF24Periph, 0x40004c00);
periph!( UART5, Uart5, _UART5, UsartF24Periph, 0x40005000);
periph!( USART6, Usart6, _USART6, UsartF24Periph, 0x40011400);

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


