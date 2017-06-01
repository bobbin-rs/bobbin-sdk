pub use stm32_common::chip::tim_gen::*;

pub const TIM2: Tim2 = Tim2 {};
pub const TIM2_IMPL: TimGenImpl = TimGenImpl(0x40000000);
pub const TIM2_IMPL_REF: &TimGenImpl = &TIM2_IMPL;

pub struct Tim2 {}
impl ::core::ops::Deref for Tim2 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM2_IMPL_REF }
}

impl super::sig::Signal<super::sig::Tim2Ch1> for Tim2 {}
impl super::sig::SignalCh1<super::sig::Tim2Ch1> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Ch2> for Tim2 {}
impl super::sig::SignalCh2<super::sig::Tim2Ch2> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Ch3> for Tim2 {}
impl super::sig::SignalCh3<super::sig::Tim2Ch3> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Ch4> for Tim2 {}
impl super::sig::SignalCh4<super::sig::Tim2Ch4> for Tim2 {}
impl super::sig::Signal<super::sig::Tim2Etr> for Tim2 {}
impl super::sig::SignalEtr<super::sig::Tim2Etr> for Tim2 {}

pub const TIM21: Tim21 = Tim21 {};
pub const TIM21_IMPL: TimGenImpl = TimGenImpl(0x40010800);
pub const TIM21_IMPL_REF: &TimGenImpl = &TIM21_IMPL;

pub struct Tim21 {}
impl ::core::ops::Deref for Tim21 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM21_IMPL_REF }
}

impl super::sig::Signal<super::sig::Tim21Ch1> for Tim21 {}
impl super::sig::SignalCh1<super::sig::Tim21Ch1> for Tim21 {}
impl super::sig::Signal<super::sig::Tim21Ch2> for Tim21 {}
impl super::sig::SignalCh2<super::sig::Tim21Ch2> for Tim21 {}
impl super::sig::Signal<super::sig::Tim21Etr> for Tim21 {}
impl super::sig::SignalEtr<super::sig::Tim21Etr> for Tim21 {}

pub const TIM22: Tim22 = Tim22 {};
pub const TIM22_IMPL: TimGenImpl = TimGenImpl(0x40011400);
pub const TIM22_IMPL_REF: &TimGenImpl = &TIM22_IMPL;

pub struct Tim22 {}
impl ::core::ops::Deref for Tim22 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM22_IMPL_REF }
}

impl super::sig::Signal<super::sig::Tim22Ch1> for Tim22 {}
impl super::sig::SignalCh1<super::sig::Tim22Ch1> for Tim22 {}
impl super::sig::Signal<super::sig::Tim22Ch2> for Tim22 {}
impl super::sig::SignalCh2<super::sig::Tim22Ch2> for Tim22 {}
impl super::sig::Signal<super::sig::Tim22Etr> for Tim22 {}
impl super::sig::SignalEtr<super::sig::Tim22Etr> for Tim22 {}


