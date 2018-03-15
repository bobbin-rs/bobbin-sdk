#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::wwdg::*;

periph!( WWDG, Wwdg, WWDG_PERIPH, WwdgPeriph, 0x40002c00, 0x0b);

