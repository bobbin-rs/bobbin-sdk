#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::lptim::*;

periph!(LptimPeriph, LPTIM, Lptim, 0x40007c00);



