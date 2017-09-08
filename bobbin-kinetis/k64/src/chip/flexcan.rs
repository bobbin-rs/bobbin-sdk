#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::flexcan::*;

periph!( CAN0, Can0, _CAN0, FlexcanPeriph, 0x40024000);




