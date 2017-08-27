#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "LPUART", peripherals: [Peripheral { derived_from: None, group_name: None, name: "LPUART0", address: 1074176000, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Universal Asynchronous Receiver/Transmitter"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "LPUART0_RxTx", types: ["LPUART_RXTX"], value: 31, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "LPUART0_TX", types: ["TX"], description: None }, Signal { name: "LPUART0_RX", types: ["RX"], description: None }, Signal { name: "LPUART0_CTS", types: ["CTS"], description: None }, Signal { name: "LPUART0_RTS", types: ["RTS"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "LPUART1", address: 1074180096, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Universal Asynchronous Receiver/Transmitter"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "LPUART1_RxTx", types: ["LPUART_RXTX"], value: 33, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "LPUART1_TX", types: ["TX"], description: None }, Signal { name: "LPUART1_RX", types: ["RX"], description: None }, Signal { name: "LPUART1_CTS", types: ["CTS"], description: None }, Signal { name: "LPUART1_RTS", types: ["RTS"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "LPUART2", address: 1074184192, size: None, access: None, reset_value: None, reset_mask: None, description: Some("Universal Asynchronous Receiver/Transmitter"), modules: [], features: [], links: [], interrupts: [Interrupt { name: "LPUART2_RxTx", types: ["LPUART_RXTX"], value: 35, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "LPUART2_TX", types: ["TX"], description: None }, Signal { name: "LPUART2_RX", types: ["RX"], description: None }, Signal { name: "LPUART2_CTS", types: ["CTS"], description: None }, Signal { name: "LPUART2_RTS", types: ["RTS"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::lpuart::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::lpuart::*;

periph!( LPUART0, Lpuart0, _LPUART0, LpuartPeriph, 0x4006a000);
periph!( LPUART1, Lpuart1, _LPUART1, LpuartPeriph, 0x4006b000);
periph!( LPUART2, Lpuart2, _LPUART2, LpuartPeriph, 0x4006c000);

impl super::sig::Signal<super::sig::Lpuart0Tx> for Lpuart0 {}
impl super::sig::SignalTx<super::sig::Lpuart0Tx> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Rx> for Lpuart0 {}
impl super::sig::SignalRx<super::sig::Lpuart0Rx> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Cts> for Lpuart0 {}
impl super::sig::SignalCts<super::sig::Lpuart0Cts> for Lpuart0 {}
impl super::sig::Signal<super::sig::Lpuart0Rts> for Lpuart0 {}
impl super::sig::SignalRts<super::sig::Lpuart0Rts> for Lpuart0 {}

impl super::sig::Signal<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::SignalTx<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::SignalRx<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::SignalCts<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rts> for Lpuart1 {}
impl super::sig::SignalRts<super::sig::Lpuart1Rts> for Lpuart1 {}

impl super::sig::Signal<super::sig::Lpuart2Tx> for Lpuart2 {}
impl super::sig::SignalTx<super::sig::Lpuart2Tx> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Rx> for Lpuart2 {}
impl super::sig::SignalRx<super::sig::Lpuart2Rx> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Cts> for Lpuart2 {}
impl super::sig::SignalCts<super::sig::Lpuart2Cts> for Lpuart2 {}
impl super::sig::Signal<super::sig::Lpuart2Rts> for Lpuart2 {}
impl super::sig::SignalRts<super::sig::Lpuart2Rts> for Lpuart2 {}


