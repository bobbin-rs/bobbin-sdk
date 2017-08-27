#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::flexcan::*;

periph!(_CAN0, FlexcanPeriph, CAN0, Can0, 0x40024000);
periph!(_CAN1, FlexcanPeriph, CAN1, Can1, 0x40025000);
periph!(_CAN2, FlexcanPeriph, CAN2, Can2, 0x4002b000);





