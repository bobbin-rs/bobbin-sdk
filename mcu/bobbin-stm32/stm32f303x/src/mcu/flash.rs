#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::flash::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( FLASH, Flash, FLASH_PERIPH, FlashPeriph, FLASH_OWNED, FLASH_REF_COUNT, 0x40022000, 0x00, 0x02);


