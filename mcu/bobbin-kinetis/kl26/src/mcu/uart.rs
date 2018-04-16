#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::uart::*;

periph!( UART1, Uart1, UART1_PERIPH, UartPeriph, UART1_OWNED, UART1_REF_COUNT, 0x4006b000, 0x00, 0x1b);
periph!( UART2, Uart2, UART2_PERIPH, UartPeriph, UART2_OWNED, UART2_REF_COUNT, 0x4006c000, 0x01, 0x1c);

