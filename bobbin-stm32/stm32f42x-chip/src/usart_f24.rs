pub use stm32_common::chip::usart_f24::*;

pub const USART1: Usart1 = Usart1 {};
pub const USART1_IMPL: UsartF24Impl = UsartF24Impl(0x40011000);
pub const USART1_IMPL_REF: &UsartF24Impl = &USART1_IMPL;

pub struct Usart1 {}
impl ::core::ops::Deref for Usart1 {
   type Target = UsartF24Impl;
   #[inline]
   fn deref(&self) -> &UsartF24Impl { USART1_IMPL_REF }
}

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

pub const USART2: Usart2 = Usart2 {};
pub const USART2_IMPL: UsartF24Impl = UsartF24Impl(0x40004400);
pub const USART2_IMPL_REF: &UsartF24Impl = &USART2_IMPL;

pub struct Usart2 {}
impl ::core::ops::Deref for Usart2 {
   type Target = UsartF24Impl;
   #[inline]
   fn deref(&self) -> &UsartF24Impl { USART2_IMPL_REF }
}

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

pub const USART3: Usart3 = Usart3 {};
pub const USART3_IMPL: UsartF24Impl = UsartF24Impl(0x40004800);
pub const USART3_IMPL_REF: &UsartF24Impl = &USART3_IMPL;

pub struct Usart3 {}
impl ::core::ops::Deref for Usart3 {
   type Target = UsartF24Impl;
   #[inline]
   fn deref(&self) -> &UsartF24Impl { USART3_IMPL_REF }
}

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

pub const USART6: Usart6 = Usart6 {};
pub const USART6_IMPL: UsartF24Impl = UsartF24Impl(0x40011400);
pub const USART6_IMPL_REF: &UsartF24Impl = &USART6_IMPL;

pub struct Usart6 {}
impl ::core::ops::Deref for Usart6 {
   type Target = UsartF24Impl;
   #[inline]
   fn deref(&self) -> &UsartF24Impl { USART6_IMPL_REF }
}

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

pub const UART7: Uart7 = Uart7 {};
pub const UART7_IMPL: UsartF24Impl = UsartF24Impl(0x40007800);
pub const UART7_IMPL_REF: &UsartF24Impl = &UART7_IMPL;

pub struct Uart7 {}
impl ::core::ops::Deref for Uart7 {
   type Target = UsartF24Impl;
   #[inline]
   fn deref(&self) -> &UsartF24Impl { UART7_IMPL_REF }
}

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

pub const UART8: Uart8 = Uart8 {};
pub const UART8_IMPL: UsartF24Impl = UsartF24Impl(0x40007c00);
pub const UART8_IMPL_REF: &UsartF24Impl = &UART8_IMPL;

pub struct Uart8 {}
impl ::core::ops::Deref for Uart8 {
   type Target = UsartF24Impl;
   #[inline]
   fn deref(&self) -> &UsartF24Impl { UART8_IMPL_REF }
}

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


