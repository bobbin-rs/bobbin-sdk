#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::lptim::*;

periph!( LPTIM1, Lptim1, _LPTIM1, LptimPeriph, 0x40007c00);
periph!( LPTIM2, Lptim2, _LPTIM2, LptimPeriph, 0x40009400);





