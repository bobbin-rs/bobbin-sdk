#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::lpuart::*;

periph!( LPUART1, Lpuart1, LPUART1_PERIPH, LpuartPeriph, 0x40004800, 0x14);

