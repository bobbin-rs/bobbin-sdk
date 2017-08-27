#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::iwdg::*;

periph!( IWDG, Iwdg, _IWDG, IwdgPeriph, 0x40003000);



