#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::flash::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( FLASH, Flash, FLASH_PERIPH, FlashPeriph, 0x40023c00, 0x00, 0x01);


