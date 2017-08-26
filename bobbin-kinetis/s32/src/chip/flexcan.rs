#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::flexcan::*;

periph!(FlexcanPeriph, CAN0, Can0, 0x40024000);
periph!(FlexcanPeriph, CAN1, Can1, 0x40025000);
periph!(FlexcanPeriph, CAN2, Can2, 0x4002b000);





