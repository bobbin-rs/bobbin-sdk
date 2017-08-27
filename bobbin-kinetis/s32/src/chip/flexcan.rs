#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::flexcan::*;

periph!( CAN0, Can0, _CAN0, FlexcanPeriph, 0x40024000);
periph!( CAN1, Can1, _CAN1, FlexcanPeriph, 0x40025000);
periph!( CAN2, Can2, _CAN2, FlexcanPeriph, 0x4002b000);





