pub use stm32_common::chip::tim_adv::*;

pub const TIM1: Tim1 = Tim1 {};
pub const TIM1_IMPL: TimAdvImpl = TimAdvImpl(0x40010000);
pub const TIM1_IMPL_REF: &TimAdvImpl = &TIM1_IMPL;

pub struct Tim1 {}
impl ::core::ops::Deref for Tim1 {
   type Target = TimAdvImpl;
   #[inline]
   fn deref(&self) -> &TimAdvImpl { TIM1_IMPL_REF }
}


pub const TIM8: Tim8 = Tim8 {};
pub const TIM8_IMPL: TimAdvImpl = TimAdvImpl(0x40010400);
pub const TIM8_IMPL_REF: &TimAdvImpl = &TIM8_IMPL;

pub struct Tim8 {}
impl ::core::ops::Deref for Tim8 {
   type Target = TimAdvImpl;
   #[inline]
   fn deref(&self) -> &TimAdvImpl { TIM8_IMPL_REF }
}



