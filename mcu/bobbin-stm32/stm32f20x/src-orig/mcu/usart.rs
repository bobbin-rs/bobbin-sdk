#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::usart::*;

periph!( USART1, Usart1, USART1_PERIPH, UsartPeriph, 0x40011000, 0x35);
periph!( USART2, Usart2, USART2_PERIPH, UsartPeriph, 0x40004400, 0x36);
periph!( USART3, Usart3, USART3_PERIPH, UsartPeriph, 0x40004800, 0x37);
periph!( UART4, Uart4, UART4_PERIPH, UsartPeriph, 0x40004c00, 0x38);
periph!( UART5, Uart5, UART5_PERIPH, UsartPeriph, 0x40005000, 0x39);
periph!( USART6, Usart6, USART6_PERIPH, UsartPeriph, 0x40011400, 0x3a);

