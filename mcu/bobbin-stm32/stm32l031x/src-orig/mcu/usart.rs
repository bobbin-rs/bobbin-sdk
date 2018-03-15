#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::usart::*;

periph!( USART2, Usart2, USART2_PERIPH, UsartPeriph, 0x40004400, 0x13);

