pub use stm32_common::chip::tim_gen::*;

pub const TIM2: Tim2 = Tim2 {};
pub const TIM2_IMPL: TimGenImpl = TimGenImpl(0x40000000);
pub const TIM2_IMPL_REF: &TimGenImpl = &TIM2_IMPL;

pub struct Tim2 {}

impl ::core::ops::Deref for Tim2 {
   type Target = TimGenImpl;
   fn deref(&self) -> &TimGenImpl { TIM2_IMPL_REF }
}

pub const TIM21: Tim21 = Tim21 {};
pub const TIM21_IMPL: TimGenImpl = TimGenImpl(0x40010800);
pub const TIM21_IMPL_REF: &TimGenImpl = &TIM21_IMPL;

pub struct Tim21 {}

impl ::core::ops::Deref for Tim21 {
   type Target = TimGenImpl;
   fn deref(&self) -> &TimGenImpl { TIM21_IMPL_REF }
}

pub const TIM22: Tim22 = Tim22 {};
pub const TIM22_IMPL: TimGenImpl = TimGenImpl(0x40011400);
pub const TIM22_IMPL_REF: &TimGenImpl = &TIM22_IMPL;

pub struct Tim22 {}

impl ::core::ops::Deref for Tim22 {
   type Target = TimGenImpl;
   fn deref(&self) -> &TimGenImpl { TIM22_IMPL_REF }
}


