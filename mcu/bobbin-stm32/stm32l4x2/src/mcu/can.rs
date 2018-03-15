#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::can::*;

periph!( CAN1, Can1, CAN1_PERIPH, CanPeriph, 0x40006400, 0x32);

