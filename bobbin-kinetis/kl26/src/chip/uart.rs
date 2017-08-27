#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "UART", peripherals: [Peripheral { derived_from: None, group_name: None, name: "UART1", address: 1074180096, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART1", types: [], value: 13, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART1_TX", types: ["TX"], description: None }, Signal { name: "UART1_RX", types: ["RX"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "UART2", address: 1074184192, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART2", types: [], value: 14, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART2_TX", types: ["TX"], description: None }, Signal { name: "UART2_RX", types: ["RX"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::uart::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::uart::*;

periph!( UART1, Uart1, _UART1, UartPeriph, 0x4006b000);
periph!( UART2, Uart2, _UART2, UartPeriph, 0x4006c000);

impl super::sig::Signal<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::SignalTx<super::sig::Uart1Tx> for Uart1 {}
impl super::sig::Signal<super::sig::Uart1Rx> for Uart1 {}
impl super::sig::SignalRx<super::sig::Uart1Rx> for Uart1 {}

impl super::sig::Signal<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::SignalTx<super::sig::Uart2Tx> for Uart2 {}
impl super::sig::Signal<super::sig::Uart2Rx> for Uart2 {}
impl super::sig::SignalRx<super::sig::Uart2Rx> for Uart2 {}


