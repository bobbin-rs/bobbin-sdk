pub use stm32_common::chip::usart::*;

pub const USART1: Usart1 = Usart1 {};
pub const USART1_IMPL: UsartImpl = UsartImpl(0x40013800);
pub const USART1_IMPL_REF: &UsartImpl = &USART1_IMPL;

pub struct Usart1 {}
impl ::core::ops::Deref for Usart1 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { USART1_IMPL_REF }
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
pub const USART2_IMPL: UsartImpl = UsartImpl(0x40004400);
pub const USART2_IMPL_REF: &UsartImpl = &USART2_IMPL;

pub struct Usart2 {}
impl ::core::ops::Deref for Usart2 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { USART2_IMPL_REF }
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
pub const USART3_IMPL: UsartImpl = UsartImpl(0x40004800);
pub const USART3_IMPL_REF: &UsartImpl = &USART3_IMPL;

pub struct Usart3 {}
impl ::core::ops::Deref for Usart3 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { USART3_IMPL_REF }
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

pub const UART4: Uart4 = Uart4 {};
pub const UART4_IMPL: UsartImpl = UsartImpl(0x40004c00);
pub const UART4_IMPL_REF: &UsartImpl = &UART4_IMPL;

pub struct Uart4 {}
impl ::core::ops::Deref for Uart4 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { UART4_IMPL_REF }
}

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

pub const UART5: Uart5 = Uart5 {};
pub const UART5_IMPL: UsartImpl = UsartImpl(0x40005000);
pub const UART5_IMPL_REF: &UsartImpl = &UART5_IMPL;

pub struct Uart5 {}
impl ::core::ops::Deref for Uart5 {
   type Target = UsartImpl;
   #[inline]
   fn deref(&self) -> &UsartImpl { UART5_IMPL_REF }
}

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


