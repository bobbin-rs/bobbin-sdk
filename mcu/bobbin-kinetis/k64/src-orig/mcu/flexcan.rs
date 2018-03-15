#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::flexcan::*;

periph!( CAN0, Can0, CAN0_PERIPH, FlexcanPeriph, 0x40024000, 0x1b);

