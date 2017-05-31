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


pub const TIM3: Tim3 = Tim3 {};
pub const TIM3_IMPL: TimGenImpl = TimGenImpl(0x40000400);
pub const TIM3_IMPL_REF: &TimGenImpl = &TIM3_IMPL;

pub struct Tim3 {}
impl ::core::ops::Deref for Tim3 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM3_IMPL_REF }
}


pub const TIM4: Tim4 = Tim4 {};
pub const TIM4_IMPL: TimGenImpl = TimGenImpl(0x40000800);
pub const TIM4_IMPL_REF: &TimGenImpl = &TIM4_IMPL;

pub struct Tim4 {}
impl ::core::ops::Deref for Tim4 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM4_IMPL_REF }
}


pub const TIM15: Tim15 = Tim15 {};
pub const TIM15_IMPL: TimGenImpl = TimGenImpl(0x40014000);
pub const TIM15_IMPL_REF: &TimGenImpl = &TIM15_IMPL;

pub struct Tim15 {}
impl ::core::ops::Deref for Tim15 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM15_IMPL_REF }
}


pub const TIM16: Tim16 = Tim16 {};
pub const TIM16_IMPL: TimGenImpl = TimGenImpl(0x40014400);
pub const TIM16_IMPL_REF: &TimGenImpl = &TIM16_IMPL;

pub struct Tim16 {}
impl ::core::ops::Deref for Tim16 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM16_IMPL_REF }
}


pub const TIM17: Tim17 = Tim17 {};
pub const TIM17_IMPL: TimGenImpl = TimGenImpl(0x40014800);
pub const TIM17_IMPL_REF: &TimGenImpl = &TIM17_IMPL;

pub struct Tim17 {}
impl ::core::ops::Deref for Tim17 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM17_IMPL_REF }
}



