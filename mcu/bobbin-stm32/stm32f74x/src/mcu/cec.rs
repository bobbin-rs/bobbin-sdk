#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::cec::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( CEC, Cec, CEC_PERIPH, CecPeriph, 0x40006c00, 0x0c);


