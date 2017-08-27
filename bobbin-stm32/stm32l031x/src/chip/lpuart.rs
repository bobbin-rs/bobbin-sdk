#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "LPUART", peripherals: [Peripheral { derived_from: None, group_name: None, name: "LPUART1", address: 1073760256, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "LPUART1", types: ["LPUART"], value: 29, description: Some("LPUART1 global interrupt through EXTI28") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "LPUART1_TX", types: ["TX"], description: None }, Signal { name: "LPUART1_RX", types: ["RX"], description: None }, Signal { name: "LPUART1_CTS", types: ["CTS"], description: None }, Signal { name: "LPUART1_RTS", types: ["RTS"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::lpuart::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::lpuart::*;

periph!( LPUART1, Lpuart1, _LPUART1, LpuartPeriph, 0x40004800);

impl super::sig::Signal<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::SignalTx<super::sig::Lpuart1Tx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::SignalRx<super::sig::Lpuart1Rx> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::SignalCts<super::sig::Lpuart1Cts> for Lpuart1 {}
impl super::sig::Signal<super::sig::Lpuart1Rts> for Lpuart1 {}
impl super::sig::SignalRts<super::sig::Lpuart1Rts> for Lpuart1 {}


