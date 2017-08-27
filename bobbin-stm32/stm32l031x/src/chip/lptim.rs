#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::lptim::*;

periph!( LPTIM, Lptim, _LPTIM, LptimPeriph, 0x40007c00);



