#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::lpuart::*;

periph!( LPUART0, Lpuart0, LPUART0_PERIPH, LpuartPeriph, LPUART0_OWNED, LPUART0_REF_COUNT, 0x4006a000, 0x00, 0x1d);
periph!( LPUART1, Lpuart1, LPUART1_PERIPH, LpuartPeriph, LPUART1_OWNED, LPUART1_REF_COUNT, 0x4006b000, 0x01, 0x1e);
periph!( LPUART2, Lpuart2, LPUART2_PERIPH, LpuartPeriph, LPUART2_OWNED, LPUART2_REF_COUNT, 0x4006c000, 0x02, 0x1f);

