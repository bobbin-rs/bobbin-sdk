pub use kinetis_common::chip::uart::*;

pub const UART0: Uart0 = Uart0 {};
pub const UART0_REF: &Uart0 = &UART0;
pub const UART0_IMPL: UartImpl = UartImpl(0x4006a000);
pub const UART0_IMPL_REF: &UartImpl = &UART0_IMPL;

pub struct Uart0 {}
impl ::core::ops::Deref for Uart0 {
   type Target = UartImpl;
   #[inline]
   fn deref(&self) -> &UartImpl { UART0_IMPL_REF }
}

impl super::sig::Signal<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::Signal<super::sig::Uart0Rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::Uart0Rx> for Uart0 {}

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

pub const UART3: Uart3 = Uart3 {};
pub const UART3_REF: &Uart3 = &UART3;
pub const UART3_IMPL: UartImpl = UartImpl(0x4006d000);
pub const UART3_IMPL_REF: &UartImpl = &UART3_IMPL;

pub struct Uart3 {}
impl ::core::ops::Deref for Uart3 {
   type Target = UartImpl;
   #[inline]
   fn deref(&self) -> &UartImpl { UART3_IMPL_REF }
}

impl super::sig::Signal<super::sig::Uart3Tx> for Uart3 {}
impl super::sig::SignalTx<super::sig::Uart3Tx> for Uart3 {}
impl super::sig::Signal<super::sig::Uart3Rx> for Uart3 {}
impl super::sig::SignalRx<super::sig::Uart3Rx> for Uart3 {}

pub const UART4: Uart4 = Uart4 {};
pub const UART4_REF: &Uart4 = &UART4;
pub const UART4_IMPL: UartImpl = UartImpl(0x400ea000);
pub const UART4_IMPL_REF: &UartImpl = &UART4_IMPL;

pub struct Uart4 {}
impl ::core::ops::Deref for Uart4 {
   type Target = UartImpl;
   #[inline]
   fn deref(&self) -> &UartImpl { UART4_IMPL_REF }
}

impl super::sig::Signal<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::Uart4Rx> for Uart4 {}

pub const UART5: Uart5 = Uart5 {};
pub const UART5_REF: &Uart5 = &UART5;
pub const UART5_IMPL: UartImpl = UartImpl(0x400eb000);
pub const UART5_IMPL_REF: &UartImpl = &UART5_IMPL;

pub struct Uart5 {}
impl ::core::ops::Deref for Uart5 {
   type Target = UartImpl;
   #[inline]
   fn deref(&self) -> &UartImpl { UART5_IMPL_REF }
}

impl super::sig::Signal<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::Uart5Rx> for Uart5 {}


