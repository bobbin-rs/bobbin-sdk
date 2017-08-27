#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "USART_F24", peripherals: [Peripheral { derived_from: None, group_name: None, name: "USART1", address: 1073811456, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "USART1_IRQ", types: [], value: 37, description: Some("USART1 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "USART1_TX", types: ["TX"], description: None }, Signal { name: "USART1_RX", types: ["RX"], description: None }, Signal { name: "USART1_CTS", types: ["CTS"], description: None }, Signal { name: "USART1_RTS", types: ["RTS"], description: None }, Signal { name: "USART1_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "USART2", address: 1073759232, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "USART2_IRQ", types: [], value: 38, description: Some("USART2 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "USART2_TX", types: ["TX"], description: None }, Signal { name: "USART2_RX", types: ["RX"], description: None }, Signal { name: "USART2_CTS", types: ["CTS"], description: None }, Signal { name: "USART2_RTS", types: ["RTS"], description: None }, Signal { name: "USART2_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "USART3", address: 1073760256, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "USART3_IRQ", types: [], value: 39, description: Some("USART3 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "USART3_TX", types: ["TX"], description: None }, Signal { name: "USART3_RX", types: ["RX"], description: None }, Signal { name: "USART3_CTS", types: ["CTS"], description: None }, Signal { name: "USART3_RTS", types: ["RTS"], description: None }, Signal { name: "USART3_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "UART4", address: 1073761280, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Universal synchronous asynchronous receiver transmitter"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART4_IRQ", types: [], value: 52, description: Some("UART4 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "USART4_TX", types: ["TX"], description: None }, Signal { name: "USART4_RX", types: ["RX"], description: None }, Signal { name: "USART4_CTS", types: ["CTS"], description: None }, Signal { name: "USART4_RTS", types: ["RTS"], description: None }, Signal { name: "USART4_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "UART5", address: 1073762304, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART5_IRQ", types: [], value: 53, description: Some("UART5 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "USART5_TX", types: ["TX"], description: None }, Signal { name: "USART5_RX", types: ["RX"], description: None }, Signal { name: "USART5_CTS", types: ["CTS"], description: None }, Signal { name: "USART5_RTS", types: ["RTS"], description: None }, Signal { name: "USART5_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "USART6", address: 1073812480, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "USART6_IRQ", types: [], value: 71, description: Some("USART6 global interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "USART6_TX", types: ["TX"], description: None }, Signal { name: "USART6_RX", types: ["RX"], description: None }, Signal { name: "USART6_CTS", types: ["CTS"], description: None }, Signal { name: "USART6_RTS", types: ["RTS"], description: None }, Signal { name: "USART6_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::usart_f24::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::usart_f24::*;

periph!( USART1, Usart1, _USART1, UsartF24Periph, 0x40011000);
periph!( USART2, Usart2, _USART2, UsartF24Periph, 0x40004400);
periph!( USART3, Usart3, _USART3, UsartF24Periph, 0x40004800);
periph!( UART4, Uart4, _UART4, UsartF24Periph, 0x40004c00);
periph!( UART5, Uart5, _UART5, UsartF24Periph, 0x40005000);
periph!( USART6, Usart6, _USART6, UsartF24Periph, 0x40011400);

impl super::sig::Signal<super::sig::Usart1Tx> for Usart1 {}
impl super::sig::SignalTx<super::sig::Usart1Tx> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Rx> for Usart1 {}
impl super::sig::SignalRx<super::sig::Usart1Rx> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Cts> for Usart1 {}
impl super::sig::SignalCts<super::sig::Usart1Cts> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Rts> for Usart1 {}
impl super::sig::SignalRts<super::sig::Usart1Rts> for Usart1 {}
impl super::sig::Signal<super::sig::Usart1Ck> for Usart1 {}
impl super::sig::SignalCk<super::sig::Usart1Ck> for Usart1 {}

impl super::sig::Signal<super::sig::Usart2Tx> for Usart2 {}
impl super::sig::SignalTx<super::sig::Usart2Tx> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Rx> for Usart2 {}
impl super::sig::SignalRx<super::sig::Usart2Rx> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Cts> for Usart2 {}
impl super::sig::SignalCts<super::sig::Usart2Cts> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Rts> for Usart2 {}
impl super::sig::SignalRts<super::sig::Usart2Rts> for Usart2 {}
impl super::sig::Signal<super::sig::Usart2Ck> for Usart2 {}
impl super::sig::SignalCk<super::sig::Usart2Ck> for Usart2 {}

impl super::sig::Signal<super::sig::Usart3Tx> for Usart3 {}
impl super::sig::SignalTx<super::sig::Usart3Tx> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Rx> for Usart3 {}
impl super::sig::SignalRx<super::sig::Usart3Rx> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Cts> for Usart3 {}
impl super::sig::SignalCts<super::sig::Usart3Cts> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Rts> for Usart3 {}
impl super::sig::SignalRts<super::sig::Usart3Rts> for Usart3 {}
impl super::sig::Signal<super::sig::Usart3Ck> for Usart3 {}
impl super::sig::SignalCk<super::sig::Usart3Ck> for Usart3 {}

impl super::sig::Signal<super::sig::Usart4Tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::Usart4Tx> for Uart4 {}
impl super::sig::Signal<super::sig::Usart4Rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::Usart4Rx> for Uart4 {}
impl super::sig::Signal<super::sig::Usart4Cts> for Uart4 {}
impl super::sig::SignalCts<super::sig::Usart4Cts> for Uart4 {}
impl super::sig::Signal<super::sig::Usart4Rts> for Uart4 {}
impl super::sig::SignalRts<super::sig::Usart4Rts> for Uart4 {}
impl super::sig::Signal<super::sig::Usart4Ck> for Uart4 {}
impl super::sig::SignalCk<super::sig::Usart4Ck> for Uart4 {}

impl super::sig::Signal<super::sig::Usart5Tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::Usart5Tx> for Uart5 {}
impl super::sig::Signal<super::sig::Usart5Rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::Usart5Rx> for Uart5 {}
impl super::sig::Signal<super::sig::Usart5Cts> for Uart5 {}
impl super::sig::SignalCts<super::sig::Usart5Cts> for Uart5 {}
impl super::sig::Signal<super::sig::Usart5Rts> for Uart5 {}
impl super::sig::SignalRts<super::sig::Usart5Rts> for Uart5 {}
impl super::sig::Signal<super::sig::Usart5Ck> for Uart5 {}
impl super::sig::SignalCk<super::sig::Usart5Ck> for Uart5 {}

impl super::sig::Signal<super::sig::Usart6Tx> for Usart6 {}
impl super::sig::SignalTx<super::sig::Usart6Tx> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Rx> for Usart6 {}
impl super::sig::SignalRx<super::sig::Usart6Rx> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Cts> for Usart6 {}
impl super::sig::SignalCts<super::sig::Usart6Cts> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Rts> for Usart6 {}
impl super::sig::SignalRts<super::sig::Usart6Rts> for Usart6 {}
impl super::sig::Signal<super::sig::Usart6Ck> for Usart6 {}
impl super::sig::SignalCk<super::sig::Usart6Ck> for Usart6 {}


