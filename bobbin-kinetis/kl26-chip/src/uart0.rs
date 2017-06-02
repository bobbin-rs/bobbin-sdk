pub use kinetis_common::chip::uart0::*;

pub const UART0: Uart0 = Uart0 {};
pub const UART0_REF: &Uart0 = &UART0;
pub const UART0_IMPL: Uart0Impl = Uart0Impl(0x4006a000);
pub const UART0_IMPL_REF: &Uart0Impl = &UART0_IMPL;

pub struct Uart0 {}
impl ::core::ops::Deref for Uart0 {
   type Target = Uart0Impl;
   #[inline]
   fn deref(&self) -> &Uart0Impl { UART0_IMPL_REF }
}

impl super::sig::Signal<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::Signal<super::sig::Uart0Rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::Uart0Rx> for Uart0 {}


