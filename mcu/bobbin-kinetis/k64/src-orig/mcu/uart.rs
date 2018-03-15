#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::uart::*;

periph!( UART0, Uart0, UART0_PERIPH, UartPeriph, 0x4006a000, 0x14);
periph!( UART1, Uart1, UART1_PERIPH, UartPeriph, 0x4006b000, 0x15);
periph!( UART2, Uart2, UART2_PERIPH, UartPeriph, 0x4006c000, 0x16);
periph!( UART3, Uart3, UART3_PERIPH, UartPeriph, 0x4006d000, 0x17);
periph!( UART4, Uart4, UART4_PERIPH, UartPeriph, 0x400ea000, 0x18);
periph!( UART5, Uart5, UART5_PERIPH, UartPeriph, 0x400eb000, 0x19);

