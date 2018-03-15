#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::lpspi::*;

periph!( LPSPI0, Lpspi0, LPSPI0_PERIPH, LpspiPeriph, 0x4002c000, 0x21);
periph!( LPSPI1, Lpspi1, LPSPI1_PERIPH, LpspiPeriph, 0x4002d000, 0x22);
periph!( LPSPI2, Lpspi2, LPSPI2_PERIPH, LpspiPeriph, 0x4002e000, 0x23);

