#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "USART", peripherals: [Peripheral { derived_from: None, group_name: None, name: "USART1", address: 1073821696, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Universal synchronous asynchronous receiver transmitter"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "USART1_EXTI25", types: [], value: 37, description: Some("USART1 global interrupt and EXTI Line 25 interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "USART1_TX", types: ["TX"], description: None }, Signal { name: "USART1_RX", types: ["RX"], description: None }, Signal { name: "USART1_CTS", types: ["CTS"], description: None }, Signal { name: "USART1_RTS", types: ["RTS"], description: None }, Signal { name: "USART1_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "USART2", address: 1073759232, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "USART2_EXTI26", types: [], value: 38, description: Some("USART2 global interrupt and EXTI Line 26 interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "USART2_TX", types: ["TX"], description: None }, Signal { name: "USART2_RX", types: ["RX"], description: None }, Signal { name: "USART2_CTS", types: ["CTS"], description: None }, Signal { name: "USART2_RTS", types: ["RTS"], description: None }, Signal { name: "USART2_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "USART3", address: 1073760256, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "USART3_EXTI28", types: [], value: 39, description: Some("USART3 global interrupt and EXTI Line 28 interrupt") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "USART3_TX", types: ["TX"], description: None }, Signal { name: "USART3_RX", types: ["RX"], description: None }, Signal { name: "USART3_CTS", types: ["CTS"], description: None }, Signal { name: "USART3_RTS", types: ["RTS"], description: None }, Signal { name: "USART3_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "UART4", address: 1073761280, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART4_EXTI34", types: [], value: 52, description: Some("UART4 global and EXTI Line 34 interrupts") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART4_TX", types: ["TX"], description: None }, Signal { name: "UART4_RX", types: ["RX"], description: None }, Signal { name: "UART4_CTS", types: ["CTS"], description: None }, Signal { name: "UART4_RTS", types: ["RTS"], description: None }, Signal { name: "UART4_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "UART5", address: 1073762304, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART5_EXTI35", types: [], value: 53, description: Some("UART5 global and EXTI Line 35 interrupts") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART5_TX", types: ["TX"], description: None }, Signal { name: "UART5_RX", types: ["RX"], description: None }, Signal { name: "UART5_CTS", types: ["CTS"], description: None }, Signal { name: "UART5_RTS", types: ["RTS"], description: None }, Signal { name: "UART5_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::usart::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::usart::*;

periph!( USART1, Usart1, _USART1, UsartPeriph, 0x40013800);
periph!( USART2, Usart2, _USART2, UsartPeriph, 0x40004400);
periph!( USART3, Usart3, _USART3, UsartPeriph, 0x40004800);
periph!( UART4, Uart4, _UART4, UsartPeriph, 0x40004c00);
periph!( UART5, Uart5, _UART5, UsartPeriph, 0x40005000);

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

impl super::sig::Signal<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Cts> for Uart4 {}
impl super::sig::SignalCts<super::sig::Uart4Cts> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rts> for Uart4 {}
impl super::sig::SignalRts<super::sig::Uart4Rts> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Ck> for Uart4 {}
impl super::sig::SignalCk<super::sig::Uart4Ck> for Uart4 {}

impl super::sig::Signal<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Cts> for Uart5 {}
impl super::sig::SignalCts<super::sig::Uart5Cts> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rts> for Uart5 {}
impl super::sig::SignalRts<super::sig::Uart5Rts> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Ck> for Uart5 {}
impl super::sig::SignalCk<super::sig::Uart5Ck> for Uart5 {}


