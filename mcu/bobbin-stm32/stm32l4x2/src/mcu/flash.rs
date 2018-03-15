#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::flash::*;

periph!( FLASH, Flash, FLASH_PERIPH, FlashPeriph, 0x40022000, 0x0f);

