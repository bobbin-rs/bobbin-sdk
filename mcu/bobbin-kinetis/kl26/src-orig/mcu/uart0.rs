#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::uart0::*;

periph!( UART0, Uart0, UART0_PERIPH, Uart0Periph, 0x4006a000, 0x1a);

