#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::flexcan::*;

periph!( CAN0, Can0, CAN0_PERIPH, FlexcanPeriph, CAN0_OWNED, CAN0_REF_COUNT, 0x40024000, 0x00, 0x10);
periph!( CAN1, Can1, CAN1_PERIPH, FlexcanPeriph, CAN1_OWNED, CAN1_REF_COUNT, 0x40025000, 0x01, 0x11);
periph!( CAN2, Can2, CAN2_PERIPH, FlexcanPeriph, CAN2_OWNED, CAN2_REF_COUNT, 0x4002b000, 0x02, 0x12);

