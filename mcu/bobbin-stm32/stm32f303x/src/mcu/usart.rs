#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::usart::*;

periph!( USART1, Usart1, USART1_PERIPH, UsartPeriph, USART1_OWNED, USART1_REF_COUNT, 0x40013800, 0x00, 0x11);
periph!( USART2, Usart2, USART2_PERIPH, UsartPeriph, USART2_OWNED, USART2_REF_COUNT, 0x40004400, 0x01, 0x12);
periph!( USART3, Usart3, USART3_PERIPH, UsartPeriph, USART3_OWNED, USART3_REF_COUNT, 0x40004800, 0x02, 0x13);
periph!( UART4, Uart4, UART4_PERIPH, UsartPeriph, UART4_OWNED, UART4_REF_COUNT, 0x40004c00, 0x03, 0x14);
periph!( UART5, Uart5, UART5_PERIPH, UsartPeriph, UART5_OWNED, UART5_REF_COUNT, 0x40005000, 0x04, 0x15);

