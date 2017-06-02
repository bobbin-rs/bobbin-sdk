pub use kinetis_common::chip::lpspi::*;

pub const LPSPI0: Lpspi0 = Lpspi0 {};
pub const LPSPI0_REF: &Lpspi0 = &LPSPI0;
pub const LPSPI0_IMPL: LpspiImpl = LpspiImpl(0x4002c000);
pub const LPSPI0_IMPL_REF: &LpspiImpl = &LPSPI0_IMPL;

pub struct Lpspi0 {}
impl ::core::ops::Deref for Lpspi0 {
   type Target = LpspiImpl;
   #[inline]
   fn deref(&self) -> &LpspiImpl { LPSPI0_IMPL_REF }
}


pub const LPSPI1: Lpspi1 = Lpspi1 {};
pub const LPSPI1_REF: &Lpspi1 = &LPSPI1;
pub const LPSPI1_IMPL: LpspiImpl = LpspiImpl(0x4002d000);
pub const LPSPI1_IMPL_REF: &LpspiImpl = &LPSPI1_IMPL;

pub struct Lpspi1 {}
impl ::core::ops::Deref for Lpspi1 {
   type Target = LpspiImpl;
   #[inline]
   fn deref(&self) -> &LpspiImpl { LPSPI1_IMPL_REF }
}


pub const LPSPI2: Lpspi2 = Lpspi2 {};
pub const LPSPI2_REF: &Lpspi2 = &LPSPI2;
pub const LPSPI2_IMPL: LpspiImpl = LpspiImpl(0x4002e000);
pub const LPSPI2_IMPL_REF: &LpspiImpl = &LPSPI2_IMPL;

pub struct Lpspi2 {}
impl ::core::ops::Deref for Lpspi2 {
   type Target = LpspiImpl;
   #[inline]
   fn deref(&self) -> &LpspiImpl { LPSPI2_IMPL_REF }
}



