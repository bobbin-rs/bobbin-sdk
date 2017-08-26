#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::iwdg::*;

periph!(IwdgPeriph, IWDG, Iwdg, 0x40003000);



