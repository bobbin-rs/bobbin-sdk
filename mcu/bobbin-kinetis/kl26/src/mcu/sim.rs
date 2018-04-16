#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::sim::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( SIM, Sim, SIM_PERIPH, SimPeriph, SIM_OWNED, SIM_REF_COUNT, 0x40047000, 0x00, 0x02);


