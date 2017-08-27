#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "UART0", peripherals: [Peripheral { derived_from: None, group_name: None, name: "UART0", address: 1074176000, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Universal Asynchronous Receiver/Transmitter"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "UART0", types: [], value: 12, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "UART0_TX", types: ["TX"], description: None }, Signal { name: "UART0_RX", types: ["RX"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::uart0::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::uart0::*;

periph!( UART0, Uart0, _UART0, Uart0Periph, 0x4006a000);

impl super::sig::Signal<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::Uart0Tx> for Uart0 {}
impl super::sig::Signal<super::sig::Uart0Rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::Uart0Rx> for Uart0 {}


