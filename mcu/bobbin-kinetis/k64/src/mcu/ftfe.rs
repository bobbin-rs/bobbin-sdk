#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::ftfe::*;

periph!( FTFE, Ftfe, FTFE_PERIPH, FtfePeriph, FTFE_OWNED, FTFE_REF_COUNT, 0x40020000, 0x00, 0x08);

