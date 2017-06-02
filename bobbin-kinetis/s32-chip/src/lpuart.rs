pub use kinetis_common::chip::lpuart::*;

pub const LPUART0: Lpuart0 = Lpuart0 {};
pub const LPUART0_REF: &Lpuart0 = &LPUART0;
pub const LPUART0_IMPL: LpuartImpl = LpuartImpl(0x4006a000);
pub const LPUART0_IMPL_REF: &LpuartImpl = &LPUART0_IMPL;

pub struct Lpuart0 {}
impl ::core::ops::Deref for Lpuart0 {
   type Target = LpuartImpl;
   #[inline]
   fn deref(&self) -> &LpuartImpl { LPUART0_IMPL_REF }
}

impl super::sig::Signal<super::sig::Lpuart0Tx> for Lpuart0 {}
impl super::sig::SignalTx<super::sig::Lpuart0Tx> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Rx> for Lpuart0 {}
impl super::sig::SignalRx<super::sig::Lpuart0Rx> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Cts> for Lpuart0 {}
impl super::sig::SignalCts<super::sig::Lpuart0Cts> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Rts> for Lpuart0 {}
impl super::sig::SignalRts<super::sig::Lpuart0Rts> for Lpuart0 {}

pub const LPUART1: Lpuart1 = Lpuart1 {};
pub const LPUART1_REF: &Lpuart1 = &LPUART1;
pub const LPUART1_IMPL: LpuartImpl = LpuartImpl(0x4006b000);
pub const LPUART1_IMPL_REF: &LpuartImpl = &LPUART1_IMPL;

pub struct Lpuart1 {}
impl ::core::ops::Deref for Lpuart1 {
   type Target = LpuartImpl;
   #[inline]
   fn deref(&self) -> &LpuartImpl { LPUART1_IMPL_REF }
}

impl super::sig::Signal<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::SignalTx<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::SignalRx<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::SignalCts<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rts> for Lpuart1 {}
impl super::sig::SignalRts<super::sig::Lpuart1Rts> for Lpuart1 {}

pub const LPUART2: Lpuart2 = Lpuart2 {};
pub const LPUART2_REF: &Lpuart2 = &LPUART2;
pub const LPUART2_IMPL: LpuartImpl = LpuartImpl(0x4006c000);
pub const LPUART2_IMPL_REF: &LpuartImpl = &LPUART2_IMPL;

pub struct Lpuart2 {}
impl ::core::ops::Deref for Lpuart2 {
   type Target = LpuartImpl;
   #[inline]
   fn deref(&self) -> &LpuartImpl { LPUART2_IMPL_REF }
}

impl super::sig::Signal<super::sig::Lpuart2Tx> for Lpuart2 {}
impl super::sig::SignalTx<super::sig::Lpuart2Tx> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Rx> for Lpuart2 {}
impl super::sig::SignalRx<super::sig::Lpuart2Rx> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Cts> for Lpuart2 {}
impl super::sig::SignalCts<super::sig::Lpuart2Cts> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Rts> for Lpuart2 {}
impl super::sig::SignalRts<super::sig::Lpuart2Rts> for Lpuart2 {}


