#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::flash::*;

periph!( FLASH, Flash, FLASH_PERIPH, FlashPeriph, 0x40022000, 0x00, 0x0f);

