#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::wwdg::*;

periph!(WwdgPeriph, WWDG, Wwdg, 0x40002c00);



