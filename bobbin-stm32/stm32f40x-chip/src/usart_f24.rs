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

pub const UART4: Uart4 = Uart4 {};
pub const UART4_IMPL: UsartF24Impl = UsartF24Impl(0x40004c00);
pub const UART4_IMPL_REF: &UsartF24Impl = &UART4_IMPL;

pub struct Uart4 {}
impl ::core::ops::Deref for Uart4 {
   type Target = UsartF24Impl;
   #[inline]
   fn deref(&self) -> &UsartF24Impl { UART4_IMPL_REF }
}

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

pub const UART5: Uart5 = Uart5 {};
pub const UART5_IMPL: UsartF24Impl = UsartF24Impl(0x40005000);
pub const UART5_IMPL_REF: &UsartF24Impl = &UART5_IMPL;

pub struct Uart5 {}
impl ::core::ops::Deref for Uart5 {
   type Target = UsartF24Impl;
   #[inline]
   fn deref(&self) -> &UsartF24Impl { UART5_IMPL_REF }
}

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


