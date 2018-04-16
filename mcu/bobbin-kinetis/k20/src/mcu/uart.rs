#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::uart::*;

periph!( UART0, Uart0, UART0_PERIPH, UartPeriph, UART0_OWNED, UART0_REF_COUNT, 0x4006a000, 0x00, 0x0f);
periph!( UART1, Uart1, UART1_PERIPH, UartPeriph, UART1_OWNED, UART1_REF_COUNT, 0x4006b000, 0x01, 0x10);
periph!( UART2, Uart2, UART2_PERIPH, UartPeriph, UART2_OWNED, UART2_REF_COUNT, 0x4006c000, 0x02, 0x11);
periph!( UART3, Uart3, UART3_PERIPH, UartPeriph, UART3_OWNED, UART3_REF_COUNT, 0x4006d000, 0x03, 0x12);
periph!( UART4, Uart4, UART4_PERIPH, UartPeriph, UART4_OWNED, UART4_REF_COUNT, 0x400ea000, 0x04, 0x13);
periph!( UART5, Uart5, UART5_PERIPH, UartPeriph, UART5_OWNED, UART5_REF_COUNT, 0x400eb000, 0x05, 0x14);

