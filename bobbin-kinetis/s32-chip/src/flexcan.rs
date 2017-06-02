pub use kinetis_common::chip::flexcan::*;

pub const CAN0: Can0 = Can0 {};
pub const CAN0_REF: &Can0 = &CAN0;
pub const CAN0_IMPL: FlexcanImpl = FlexcanImpl(0x40024000);
pub const CAN0_IMPL_REF: &FlexcanImpl = &CAN0_IMPL;

pub struct Can0 {}
impl ::core::ops::Deref for Can0 {
   type Target = FlexcanImpl;
   #[inline]
   fn deref(&self) -> &FlexcanImpl { CAN0_IMPL_REF }
}


pub const CAN1: Can1 = Can1 {};
pub const CAN1_REF: &Can1 = &CAN1;
pub const CAN1_IMPL: FlexcanImpl = FlexcanImpl(0x40025000);
pub const CAN1_IMPL_REF: &FlexcanImpl = &CAN1_IMPL;

pub struct Can1 {}
impl ::core::ops::Deref for Can1 {
   type Target = FlexcanImpl;
   #[inline]
   fn deref(&self) -> &FlexcanImpl { CAN1_IMPL_REF }
}


pub const CAN2: Can2 = Can2 {};
pub const CAN2_REF: &Can2 = &CAN2;
pub const CAN2_IMPL: FlexcanImpl = FlexcanImpl(0x4002b000);
pub const CAN2_IMPL_REF: &FlexcanImpl = &CAN2_IMPL;

pub struct Can2 {}
impl ::core::ops::Deref for Can2 {
   type Target = FlexcanImpl;
   #[inline]
   fn deref(&self) -> &FlexcanImpl { CAN2_IMPL_REF }
}



