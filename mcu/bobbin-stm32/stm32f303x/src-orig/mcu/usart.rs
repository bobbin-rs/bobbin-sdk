#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::usart::*;

periph!( USART1, Usart1, USART1_PERIPH, UsartPeriph, 0x40013800, 0x11);
periph!( USART2, Usart2, USART2_PERIPH, UsartPeriph, 0x40004400, 0x12);
periph!( USART3, Usart3, USART3_PERIPH, UsartPeriph, 0x40004800, 0x13);
periph!( UART4, Uart4, UART4_PERIPH, UsartPeriph, 0x40004c00, 0x14);
periph!( UART5, Uart5, UART5_PERIPH, UsartPeriph, 0x40005000, 0x15);

