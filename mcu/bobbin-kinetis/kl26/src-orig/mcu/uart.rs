#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::uart::*;

periph!( UART1, Uart1, UART1_PERIPH, UartPeriph, 0x4006b000, 0x1b);
periph!( UART2, Uart2, UART2_PERIPH, UartPeriph, 0x4006c000, 0x1c);

