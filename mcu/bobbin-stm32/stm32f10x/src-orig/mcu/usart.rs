#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::usart::*;

periph!( USART1, Usart1, USART1_PERIPH, UsartPeriph, 0x40013800, 0x1f);
periph!( USART2, Usart2, USART2_PERIPH, UsartPeriph, 0x40004400, 0x20);
periph!( USART3, Usart3, USART3_PERIPH, UsartPeriph, 0x40004800, 0x21);

