#[allow(unused_imports)] use bobbin_common::*;

// PeripheralGroup { name: "LPSPI", peripherals: [Peripheral { derived_from: None, group_name: None, name: "LPSPI0", address: 1073922048, size: None, access: None, reset_value: None, reset_mask: None, description: Some("The LPSPI Memory Map/Register Definition can be found here."), modules: [], features: [], links: [], interrupts: [Interrupt { name: "LPSPI0", types: ["LPSPI"], value: 26, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "LPSPI0_SCK", types: ["SPI_SCK"], description: None }, Signal { name: "LPSPI0_SOUT", types: ["SPI_SOUT"], description: None }, Signal { name: "LPSPI0_SIN", types: ["SPI_SIN"], description: None }, Signal { name: "LPSPI0_PCS0", types: ["SPI_PCS0"], description: None }, Signal { name: "LPSPI0_PCS1", types: ["SPI_PCS1"], description: None }, Signal { name: "LPSPI0_PCS2", types: ["SPI_PCS2"], description: None }, Signal { name: "LPSPI0_PCS3", types: ["SPI_PCS3"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "LPSPI1", address: 1073926144, size: None, access: None, reset_value: None, reset_mask: None, description: Some("The LPSPI Memory Map/Register Definition can be found here."), modules: [], features: [], links: [], interrupts: [Interrupt { name: "LPSPI1", types: ["LPSPI"], value: 27, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "LPSPI1_SCK", types: ["SPI_SCK"], description: None }, Signal { name: "LPSPI1_SOUT", types: ["SPI_SOUT"], description: None }, Signal { name: "LPSPI1_SIN", types: ["SPI_SIN"], description: None }, Signal { name: "LPSPI1_PCS0", types: ["SPI_PCS0"], description: None }, Signal { name: "LPSPI1_PCS1", types: ["SPI_PCS1"], description: None }, Signal { name: "LPSPI1_PCS2", types: ["SPI_PCS2"], description: None }, Signal { name: "LPSPI1_PCS3", types: ["SPI_PCS3"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }, Peripheral { derived_from: None, group_name: None, name: "LPSPI2", address: 1073930240, size: None, access: None, reset_value: None, reset_mask: None, description: Some("The LPSPI Memory Map/Register Definition can be found here."), modules: [], features: [], links: [], interrupts: [Interrupt { name: "LPSPI2", types: ["LPSPI"], value: 28, description: None }], clusters: [], registers: [], descriptors: [], signals: [Signal { name: "LPSPI2_SCK", types: ["SPI_SCK"], description: None }, Signal { name: "LPSPI2_SOUT", types: ["SPI_SOUT"], description: None }, Signal { name: "LPSPI2_SIN", types: ["SPI_SIN"], description: None }, Signal { name: "LPSPI2_PCS0", types: ["SPI_PCS0"], description: None }, Signal { name: "LPSPI2_PCS1", types: ["SPI_PCS1"], description: None }, Signal { name: "LPSPI2_PCS2", types: ["SPI_PCS2"], description: None }, Signal { name: "LPSPI2_PCS3", types: ["SPI_PCS3"], description: None }], pins: [], channels: [], dim: None, dim_increment: None, dim_index: None }], prototype: None, modules: [Module { name: "kinetis_common::chip::lpspi::*", _as: None }], has_pins: false, has_channels: false, description: None }
pub use kinetis_common::chip::lpspi::*;

periph!( LPSPI0, Lpspi0, _LPSPI0, LpspiPeriph, 0x4002c000);
periph!( LPSPI1, Lpspi1, _LPSPI1, LpspiPeriph, 0x4002d000);
periph!( LPSPI2, Lpspi2, _LPSPI2, LpspiPeriph, 0x4002e000);

impl super::sig::Signal<super::sig::Lpspi0Sck> for Lpspi0 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi0Sck> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Sout> for Lpspi0 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi0Sout> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Sin> for Lpspi0 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi0Sin> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs0> for Lpspi0 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi0Pcs0> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs1> for Lpspi0 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi0Pcs1> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs2> for Lpspi0 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi0Pcs2> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs3> for Lpspi0 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi0Pcs3> for Lpspi0 {}

impl super::sig::Signal<super::sig::Lpspi1Sck> for Lpspi1 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi1Sck> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Sout> for Lpspi1 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi1Sout> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Sin> for Lpspi1 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi1Sin> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs0> for Lpspi1 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi1Pcs0> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs1> for Lpspi1 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi1Pcs1> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs2> for Lpspi1 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi1Pcs2> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs3> for Lpspi1 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi1Pcs3> for Lpspi1 {}

impl super::sig::Signal<super::sig::Lpspi2Sck> for Lpspi2 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi2Sck> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Sout> for Lpspi2 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi2Sout> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Sin> for Lpspi2 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi2Sin> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs0> for Lpspi2 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi2Pcs0> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs1> for Lpspi2 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi2Pcs1> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs2> for Lpspi2 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi2Pcs2> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs3> for Lpspi2 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi2Pcs3> for Lpspi2 {}


