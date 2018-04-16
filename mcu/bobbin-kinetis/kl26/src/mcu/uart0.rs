#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::uart0::*;

periph!( UART0, Uart0, UART0_PERIPH, Uart0Periph, UART0_OWNED, UART0_REF_COUNT, 0x4006a000, 0x00, 0x1a);

