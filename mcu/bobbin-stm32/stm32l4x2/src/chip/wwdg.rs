#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::wwdg::*;

periph!( WWDG, Wwdg, _WWDG, WwdgPeriph, 0x40002c00);




