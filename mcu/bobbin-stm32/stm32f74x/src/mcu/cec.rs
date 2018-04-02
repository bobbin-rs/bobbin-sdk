#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::cec::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( CEC, Cec, CEC_PERIPH, CecPeriph, CEC_OWNED, CEC_REF_COUNT, 0x40006c00, 0x00, 0x0b);


