#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::wwdg::*;

periph!( WWDG, Wwdg, WWDG_PERIPH, WwdgPeriph, WWDG_OWNED, WWDG_REF_COUNT, 0x40002c00, 0x00, 0x06);

