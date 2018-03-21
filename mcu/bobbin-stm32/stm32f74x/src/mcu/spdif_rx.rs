#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::spdif_rx::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( SPDIF_RX, SpdifRx, SPDIF_RX_PERIPH, SpdifRxPeriph, 0x40004000, 0x00, 0x0d);


