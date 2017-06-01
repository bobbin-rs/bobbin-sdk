pub use kinetis_common::chip::pit::*;

pub const PIT: Pit = Pit {};
pub const PIT_REF: &Pit = &PIT;
pub const PIT_IMPL: PitImpl = PitImpl(0x40037000);
pub const PIT_IMPL_REF: &PitImpl = &PIT_IMPL;

pub struct Pit {}
impl ::core::ops::Deref for Pit {
   type Target = PitImpl;
   #[inline]
   fn deref(&self) -> &PitImpl { PIT_IMPL_REF }
}



