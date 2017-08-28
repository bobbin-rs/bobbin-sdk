#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::usart_f24::*;

periph!( USART1, Usart1, _USART1, UsartPeriph, 0x40011000);
periph!( USART2, Usart2, _USART2, UsartPeriph, 0x40004400);
periph!( USART3, Usart3, _USART3, UsartPeriph, 0x40004800);
periph!( UART4, Uart4, _UART4, UsartPeriph, 0x40004c00);
periph!( UART5, Uart5, _UART5, UsartPeriph, 0x40005000);
periph!( USART6, Usart6, _USART6, UsartPeriph, 0x40011400);
periph!( UART7, Uart7, _UART7, UsartPeriph, 0x40007800);
periph!( UART8, Uart8, _UART8, UsartPeriph, 0x40007c00);

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

impl super::sig::Signal<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Cts> for Uart4 {}
impl super::sig::SignalCts<super::sig::Uart4Cts> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rts> for Uart4 {}
impl super::sig::SignalRts<super::sig::Uart4Rts> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Ck> for Uart4 {}
impl super::sig::SignalCk<super::sig::Uart4Ck> for Uart4 {}

impl super::sig::Signal<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Cts> for Uart5 {}
impl super::sig::SignalCts<super::sig::Uart5Cts> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rts> for Uart5 {}
impl super::sig::SignalRts<super::sig::Uart5Rts> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Ck> for Uart5 {}
impl super::sig::SignalCk<super::sig::Uart5Ck> for Uart5 {}

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


