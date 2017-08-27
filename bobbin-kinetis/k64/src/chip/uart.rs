#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "UART", peripherals: [Peripheral { derived_from: None, group_name: None, name: "UART0", address: 1074176000, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Serial Communication Interface"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART0_LON", types: ["UART_LON"], value: 30, description: None }, Interrupt { name: "UART0_RX_TX", types: ["UART_RX_TX"], value: 31, description: None }, Interrupt { name: "UART0_ERR", types: ["UART_ERR"], value: 32, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART0_TX", types: ["TX"], description: None }, Signal { name: "UART0_RX", types: ["RX"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "UART1", address: 1074180096, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Serial Communication Interface"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART1_RX_TX", types: ["UART_RX_TX"], value: 33, description: None }, Interrupt { name: "UART1_ERR", types: ["UART_ERR"], value: 34, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART1_TX", types: ["TX"], description: None }, Signal { name: "UART1_RX", types: ["RX"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "UART2", address: 1074184192, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Serial Communication Interface"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART2_RX_TX", types: ["UART_RX_TX"], value: 35, description: None }, Interrupt { name: "UART2_ERR", types: ["UART_ERR"], value: 36, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART2_TX", types: ["TX"], description: None }, Signal { name: "UART2_RX", types: ["RX"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "UART3", address: 1074188288, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Serial Communication Interface"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART3_RX_TX", types: ["UART_RX_TX"], value: 37, description: None }, Interrupt { name: "UART3_ERR", types: ["UART_ERR"], value: 38, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART3_TX", types: ["TX"], description: None }, Signal { name: "UART3_RX", types: ["RX"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "UART4", address: 1074700288, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Serial Communication Interface"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART4_RX_TX", types: ["UART_RX_TX"], value: 66, description: None }, Interrupt { name: "UART4_ERR", types: ["UART_ERR"], value: 67, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART4_TX", types: ["TX"], description: None }, Signal { name: "UART4_RX", types: ["RX"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "UART5", address: 1074704384, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Serial Communication Interface"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART5_RX_TX", types: ["UART_RX_TX"], value: 68, description: None }, Interrupt { name: "UART5_ERR", types: ["UART_ERR"], value: 69, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART5_TX", types: ["TX"], description: None }, Signal { name: "UART5_RX", types: ["RX"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::uart::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::uart::*;

periph!( UART0, Uart0, _UART0, UartPeriph, 0x4006a000);
periph!( UART1, Uart1, _UART1, UartPeriph, 0x4006b000);
periph!( UART2, Uart2, _UART2, UartPeriph, 0x4006c000);
periph!( UART3, Uart3, _UART3, UartPeriph, 0x4006d000);
periph!( UART4, Uart4, _UART4, UartPeriph, 0x400ea000);
periph!( UART5, Uart5, _UART5, UartPeriph, 0x400eb000);

impl super::sig::Signal<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::Signal<super::sig::Uart0Rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::Uart0Rx> for Uart0 {}

impl super::sig::Signal<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::SignalTx<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::Signal<super::sig::Uart1Rx> for Uart1 {}
impl super::sig::SignalRx<super::sig::Uart1Rx> for Uart1 {}

impl super::sig::Signal<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::SignalTx<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::Signal<super::sig::Uart2Rx> for Uart2 {}
impl super::sig::SignalRx<super::sig::Uart2Rx> for Uart2 {}

impl super::sig::Signal<super::sig::Uart3Tx> for Uart3 {}
impl super::sig::SignalTx<super::sig::Uart3Tx> for Uart3 {}
impl super::sig::Signal<super::sig::Uart3Rx> for Uart3 {}
impl super::sig::SignalRx<super::sig::Uart3Rx> for Uart3 {}

impl super::sig::Signal<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::Uart4Tx> for Uart4 {}
impl super::sig::Signal<super::sig::Uart4Rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::Uart4Rx> for Uart4 {}

impl super::sig::Signal<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::Uart5Tx> for Uart5 {}
impl super::sig::Signal<super::sig::Uart5Rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::Uart5Rx> for Uart5 {}


