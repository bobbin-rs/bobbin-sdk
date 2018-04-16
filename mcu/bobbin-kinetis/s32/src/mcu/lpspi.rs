#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::lpspi::*;

periph!( LPSPI0, Lpspi0, LPSPI0_PERIPH, LpspiPeriph, LPSPI0_OWNED, LPSPI0_REF_COUNT, 0x4002c000, 0x00, 0x21);
periph!( LPSPI1, Lpspi1, LPSPI1_PERIPH, LpspiPeriph, LPSPI1_OWNED, LPSPI1_REF_COUNT, 0x4002d000, 0x01, 0x22);
periph!( LPSPI2, Lpspi2, LPSPI2_PERIPH, LpspiPeriph, LPSPI2_OWNED, LPSPI2_REF_COUNT, 0x4002e000, 0x02, 0x23);

