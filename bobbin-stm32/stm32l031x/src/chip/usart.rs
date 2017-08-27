#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "USART", peripherals: [Peripheral { derived_from: None, group_name: None, name: "USART2", address: 1073759232, size: None, access: None, reset_value: None, reset_mask: None, description: None, modules: [], features: [], links: [], interrupts: [Interrupt { name: "USART2", types: ["USART"], value: 28, description: Some("USART2 global interrupt through EXTI26") }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "USART2_TX", types: ["TX"], description: None }, Signal { name: "USART2_RX", types: ["RX"], description: None }, Signal { name: "USART2_CTS", types: ["CTS"], description: None }, Signal { name: "USART2_RTS", types: ["RTS"], description: None }, Signal { name: "USART2_CK", types: ["CK"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "stm32_common::chip::usart::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use stm32_common::chip::usart::*;

periph!( USART2, Usart2, _USART2, UsartPeriph, 0x40004400);

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


