#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::ftfa::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( FTFA, Ftfa, FTFA_PERIPH, FtfaPeriph, FTFA_OWNED, FTFA_REF_COUNT, 0x00000400, 0x00, 0x00);


