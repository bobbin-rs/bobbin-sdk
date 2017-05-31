pub use stm32_common::chip::tim_adv::*;

pub const TIM1: Tim1 = Tim1 {};
pub const TIM1_IMPL: TimAdvImpl = TimAdvImpl(0x40012c00);
pub const TIM1_IMPL_REF: &TimAdvImpl = &TIM1_IMPL;

pub struct Tim1 {}
impl ::core::ops::Deref for Tim1 {
   type Target = TimAdvImpl;
   #[inline]
   fn deref(&self) -> &TimAdvImpl { TIM1_IMPL_REF }
}


pub const TIM8: Tim8 = Tim8 {};
pub const TIM8_IMPL: TimAdvImpl = TimAdvImpl(0x40013400);
pub const TIM8_IMPL_REF: &TimAdvImpl = &TIM8_IMPL;

pub struct Tim8 {}
impl ::core::ops::Deref for Tim8 {
   type Target = TimAdvImpl;
   #[inline]
   fn deref(&self) -> &TimAdvImpl { TIM8_IMPL_REF }
}


pub const TIM20: Tim20 = Tim20 {};
pub const TIM20_IMPL: TimAdvImpl = TimAdvImpl(0x40015000);
pub const TIM20_IMPL_REF: &TimAdvImpl = &TIM20_IMPL;

pub struct Tim20 {}
impl ::core::ops::Deref for Tim20 {
   type Target = TimAdvImpl;
   #[inline]
   fn deref(&self) -> &TimAdvImpl { TIM20_IMPL_REF }
}



