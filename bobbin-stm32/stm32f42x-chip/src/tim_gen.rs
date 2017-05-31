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


pub const TIM5: Tim5 = Tim5 {};
pub const TIM5_IMPL: TimGenImpl = TimGenImpl(0x40000c00);
pub const TIM5_IMPL_REF: &TimGenImpl = &TIM5_IMPL;

pub struct Tim5 {}
impl ::core::ops::Deref for Tim5 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM5_IMPL_REF }
}


pub const TIM9: Tim9 = Tim9 {};
pub const TIM9_IMPL: TimGenImpl = TimGenImpl(0x40014000);
pub const TIM9_IMPL_REF: &TimGenImpl = &TIM9_IMPL;

pub struct Tim9 {}
impl ::core::ops::Deref for Tim9 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM9_IMPL_REF }
}


pub const TIM10: Tim10 = Tim10 {};
pub const TIM10_IMPL: TimGenImpl = TimGenImpl(0x40014400);
pub const TIM10_IMPL_REF: &TimGenImpl = &TIM10_IMPL;

pub struct Tim10 {}
impl ::core::ops::Deref for Tim10 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM10_IMPL_REF }
}


pub const TIM11: Tim11 = Tim11 {};
pub const TIM11_IMPL: TimGenImpl = TimGenImpl(0x40014800);
pub const TIM11_IMPL_REF: &TimGenImpl = &TIM11_IMPL;

pub struct Tim11 {}
impl ::core::ops::Deref for Tim11 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM11_IMPL_REF }
}


pub const TIM12: Tim12 = Tim12 {};
pub const TIM12_IMPL: TimGenImpl = TimGenImpl(0x40001800);
pub const TIM12_IMPL_REF: &TimGenImpl = &TIM12_IMPL;

pub struct Tim12 {}
impl ::core::ops::Deref for Tim12 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM12_IMPL_REF }
}


pub const TIM13: Tim13 = Tim13 {};
pub const TIM13_IMPL: TimGenImpl = TimGenImpl(0x40001c00);
pub const TIM13_IMPL_REF: &TimGenImpl = &TIM13_IMPL;

pub struct Tim13 {}
impl ::core::ops::Deref for Tim13 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM13_IMPL_REF }
}


pub const TIM14: Tim14 = Tim14 {};
pub const TIM14_IMPL: TimGenImpl = TimGenImpl(0x40002000);
pub const TIM14_IMPL_REF: &TimGenImpl = &TIM14_IMPL;

pub struct Tim14 {}
impl ::core::ops::Deref for Tim14 {
   type Target = TimGenImpl;
   #[inline]
   fn deref(&self) -> &TimGenImpl { TIM14_IMPL_REF }
}



