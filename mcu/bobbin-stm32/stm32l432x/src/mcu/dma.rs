#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dma::*;

periph!( DMA1, Dma1, DMA1_PERIPH, DmaPeriph, DMA1_OWNED, DMA1_REF_COUNT, 0x40020000, 0x00, 0x0e);
periph!( DMA2, Dma2, DMA2_PERIPH, DmaPeriph, DMA2_OWNED, DMA2_REF_COUNT, 0x40020400, 0x01, 0x0f);

channel!(DMA1_CH1, Dma1Ch1, dma1_ch1, DMA1, Dma1, DMA1_CH1_CH, DmaCh, DMA1_PERIPH, DMA1_CH1_OWNED, DMA1_CH1_REF_COUNT, 0);
channel!(DMA1_CH2, Dma1Ch2, dma1_ch2, DMA1, Dma1, DMA1_CH2_CH, DmaCh, DMA1_PERIPH, DMA1_CH2_OWNED, DMA1_CH2_REF_COUNT, 1);
channel!(DMA1_CH3, Dma1Ch3, dma1_ch3, DMA1, Dma1, DMA1_CH3_CH, DmaCh, DMA1_PERIPH, DMA1_CH3_OWNED, DMA1_CH3_REF_COUNT, 2);
channel!(DMA1_CH4, Dma1Ch4, dma1_ch4, DMA1, Dma1, DMA1_CH4_CH, DmaCh, DMA1_PERIPH, DMA1_CH4_OWNED, DMA1_CH4_REF_COUNT, 3);
channel!(DMA1_CH5, Dma1Ch5, dma1_ch5, DMA1, Dma1, DMA1_CH5_CH, DmaCh, DMA1_PERIPH, DMA1_CH5_OWNED, DMA1_CH5_REF_COUNT, 4);
channel!(DMA1_CH6, Dma1Ch6, dma1_ch6, DMA1, Dma1, DMA1_CH6_CH, DmaCh, DMA1_PERIPH, DMA1_CH6_OWNED, DMA1_CH6_REF_COUNT, 5);
channel!(DMA1_CH7, Dma1Ch7, dma1_ch7, DMA1, Dma1, DMA1_CH7_CH, DmaCh, DMA1_PERIPH, DMA1_CH7_OWNED, DMA1_CH7_REF_COUNT, 6);
channel!(DMA2_CH1, Dma2Ch1, dma2_ch1, DMA2, Dma2, DMA2_CH1_CH, DmaCh, DMA2_PERIPH, DMA2_CH1_OWNED, DMA2_CH1_REF_COUNT, 0);
channel!(DMA2_CH2, Dma2Ch2, dma2_ch2, DMA2, Dma2, DMA2_CH2_CH, DmaCh, DMA2_PERIPH, DMA2_CH2_OWNED, DMA2_CH2_REF_COUNT, 1);
channel!(DMA2_CH3, Dma2Ch3, dma2_ch3, DMA2, Dma2, DMA2_CH3_CH, DmaCh, DMA2_PERIPH, DMA2_CH3_OWNED, DMA2_CH3_REF_COUNT, 2);
channel!(DMA2_CH4, Dma2Ch4, dma2_ch4, DMA2, Dma2, DMA2_CH4_CH, DmaCh, DMA2_PERIPH, DMA2_CH4_OWNED, DMA2_CH4_REF_COUNT, 3);
channel!(DMA2_CH5, Dma2Ch5, dma2_ch5, DMA2, Dma2, DMA2_CH5_CH, DmaCh, DMA2_PERIPH, DMA2_CH5_OWNED, DMA2_CH5_REF_COUNT, 4);
channel!(DMA2_CH6, Dma2Ch6, dma2_ch6, DMA2, Dma2, DMA2_CH6_CH, DmaCh, DMA2_PERIPH, DMA2_CH6_OWNED, DMA2_CH6_REF_COUNT, 5);
channel!(DMA2_CH7, Dma2Ch7, dma2_ch7, DMA2, Dma2, DMA2_CH7_CH, DmaCh, DMA2_PERIPH, DMA2_CH7_OWNED, DMA2_CH7_REF_COUNT, 6);
// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB1RSTR"), field: Some("DMA1RST"), description: None }
impl ::bobbin_common::gate::GateRst for Dma1 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb1rstr().dma1rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1rstr(|r| r.set_dma1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB1ENR"), field: Some("DMA1EN"), description: None }
impl ::bobbin_common::gate::GateEn for Dma1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb1enr().dma1en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1enr(|r| r.set_dma1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB1SMENR"), field: Some("DMA1SMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Dma1 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb1smenr().dma1smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1smenr(|r| r.set_dma1smen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB1RSTR"), field: Some("DMA2RST"), description: None }
impl ::bobbin_common::gate::GateRst for Dma2 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb1rstr().dma2rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1rstr(|r| r.set_dma2rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB1ENR"), field: Some("DMA2EN"), description: None }
impl ::bobbin_common::gate::GateEn for Dma2 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb1enr().dma2en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1enr(|r| r.set_dma2en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB1SMENR"), field: Some("DMA2SMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Dma2 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb1smenr().dma2smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1smenr(|r| r.set_dma2smen(value));
        self
    }
}

