#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::spdif_rx::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( SPDIF_RX, SpdifRx, SPDIF_RX_PERIPH, SpdifRxPeriph, SPDIF_RX_OWNED, SPDIF_RX_REF_COUNT, 0x40004000, 0x00, 0x0c);


