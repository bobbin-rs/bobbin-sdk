pub use kinetis_common::chip::lpit::*;

pub const LPIT0: Lpit0 = Lpit0 {};
pub const LPIT0_REF: &Lpit0 = &LPIT0;
pub const LPIT0_IMPL: LpitImpl = LpitImpl(0x40037000);
pub const LPIT0_IMPL_REF: &LpitImpl = &LPIT0_IMPL;

pub struct Lpit0 {}
impl ::core::ops::Deref for Lpit0 {
   type Target = LpitImpl;
   #[inline]
   fn deref(&self) -> &LpitImpl { LPIT0_IMPL_REF }
}



