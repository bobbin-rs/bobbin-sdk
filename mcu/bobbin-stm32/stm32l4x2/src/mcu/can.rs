#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::can::*;

periph!( CAN1, Can1, CAN1_PERIPH, CanPeriph, 0x40006400, 0x00, 0x32);

