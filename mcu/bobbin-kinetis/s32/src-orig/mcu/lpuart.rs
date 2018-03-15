#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::lpuart::*;

periph!( LPUART0, Lpuart0, LPUART0_PERIPH, LpuartPeriph, 0x4006a000, 0x1d);
periph!( LPUART1, Lpuart1, LPUART1_PERIPH, LpuartPeriph, 0x4006b000, 0x1e);
periph!( LPUART2, Lpuart2, LPUART2_PERIPH, LpuartPeriph, 0x4006c000, 0x1f);

