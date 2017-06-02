pub use kinetis_common::chip::uart::*;

pub const UART1: Uart1 = Uart1 {};
pub const UART1_REF: &Uart1 = &UART1;
pub const UART1_IMPL: UartImpl = UartImpl(0x4006b000);
pub const UART1_IMPL_REF: &UartImpl = &UART1_IMPL;

pub struct Uart1 {}
impl ::core::ops::Deref for Uart1 {
   type Target = UartImpl;
   #[inline]
   fn deref(&self) -> &UartImpl { UART1_IMPL_REF }
}

impl super::sig::Signal<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::SignalTx<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::Signal<super::sig::Uart1Rx> for Uart1 {}
impl super::sig::SignalRx<super::sig::Uart1Rx> for Uart1 {}

pub const UART2: Uart2 = Uart2 {};
pub const UART2_REF: &Uart2 = &UART2;
pub const UART2_IMPL: UartImpl = UartImpl(0x4006c000);
pub const UART2_IMPL_REF: &UartImpl = &UART2_IMPL;

pub struct Uart2 {}
impl ::core::ops::Deref for Uart2 {
   type Target = UartImpl;
   #[inline]
   fn deref(&self) -> &UartImpl { UART2_IMPL_REF }
}

impl super::sig::Signal<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::SignalTx<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::Signal<super::sig::Uart2Rx> for Uart2 {}
impl super::sig::SignalRx<super::sig::Uart2Rx> for Uart2 {}


