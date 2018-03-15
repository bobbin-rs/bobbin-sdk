#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::flexcan::*;

periph!( CAN0, Can0, CAN0_PERIPH, FlexcanPeriph, 0x40024000, 0x10);
periph!( CAN1, Can1, CAN1_PERIPH, FlexcanPeriph, 0x40025000, 0x11);
periph!( CAN2, Can2, CAN2_PERIPH, FlexcanPeriph, 0x4002b000, 0x12);

