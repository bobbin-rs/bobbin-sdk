pub use stm32_common::chip::tim_bas::*;

pub const TIM6: Tim6 = Tim6 {};
pub const TIM6_IMPL: TimBasImpl = TimBasImpl(0x40001000);
pub const TIM6_IMPL_REF: &TimBasImpl = &TIM6_IMPL;

pub struct Tim6 {}
impl ::core::ops::Deref for Tim6 {
   type Target = TimBasImpl;
   #[inline]
   fn deref(&self) -> &TimBasImpl { TIM6_IMPL_REF }
}


pub const TIM7: Tim7 = Tim7 {};
pub const TIM7_IMPL: TimBasImpl = TimBasImpl(0x40001400);
pub const TIM7_IMPL_REF: &TimBasImpl = &TIM7_IMPL;

pub struct Tim7 {}
impl ::core::ops::Deref for Tim7 {
   type Target = TimBasImpl;
   #[inline]
   fn deref(&self) -> &TimBasImpl { TIM7_IMPL_REF }
}



