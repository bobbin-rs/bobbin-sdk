#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::tim_gen::*;

periph!( TIM2, Tim2, TIM2_PERIPH, TimGenPeriph, TIM2_OWNED, TIM2_REF_COUNT, 0x40000000, 0x00, 0x25);
periph!( TIM3, Tim3, TIM3_PERIPH, TimGenPeriph, TIM3_OWNED, TIM3_REF_COUNT, 0x40000400, 0x01, 0x26);
periph!( TIM4, Tim4, TIM4_PERIPH, TimGenPeriph, TIM4_OWNED, TIM4_REF_COUNT, 0x40000800, 0x02, 0x27);
periph!( TIM15, Tim15, TIM15_PERIPH, TimGenPeriph, TIM15_OWNED, TIM15_REF_COUNT, 0x40014000, 0x03, 0x28);
periph!( TIM16, Tim16, TIM16_PERIPH, TimGenPeriph, TIM16_OWNED, TIM16_REF_COUNT, 0x40014400, 0x04, 0x29);
periph!( TIM17, Tim17, TIM17_PERIPH, TimGenPeriph, TIM17_OWNED, TIM17_REF_COUNT, 0x40014800, 0x05, 0x2a);

channel!(TIM2_CH1, Tim2Ch1, tim2_ch1, TIM2, Tim2, TIM2_CH1_CH, TimGenCh, TIM2_PERIPH, TIM2_CH1_OWNED, TIM2_CH1_REF_COUNT, 0);
channel!(TIM2_CH2, Tim2Ch2, tim2_ch2, TIM2, Tim2, TIM2_CH2_CH, TimGenCh, TIM2_PERIPH, TIM2_CH2_OWNED, TIM2_CH2_REF_COUNT, 1);
channel!(TIM2_CH3, Tim2Ch3, tim2_ch3, TIM2, Tim2, TIM2_CH3_CH, TimGenCh, TIM2_PERIPH, TIM2_CH3_OWNED, TIM2_CH3_REF_COUNT, 2);
channel!(TIM2_CH4, Tim2Ch4, tim2_ch4, TIM2, Tim2, TIM2_CH4_CH, TimGenCh, TIM2_PERIPH, TIM2_CH4_OWNED, TIM2_CH4_REF_COUNT, 3);
channel!(TIM3_CH1, Tim3Ch1, tim3_ch1, TIM3, Tim3, TIM3_CH1_CH, TimGenCh, TIM3_PERIPH, TIM3_CH1_OWNED, TIM3_CH1_REF_COUNT, 0);
channel!(TIM3_CH2, Tim3Ch2, tim3_ch2, TIM3, Tim3, TIM3_CH2_CH, TimGenCh, TIM3_PERIPH, TIM3_CH2_OWNED, TIM3_CH2_REF_COUNT, 1);
channel!(TIM3_CH3, Tim3Ch3, tim3_ch3, TIM3, Tim3, TIM3_CH3_CH, TimGenCh, TIM3_PERIPH, TIM3_CH3_OWNED, TIM3_CH3_REF_COUNT, 2);
channel!(TIM3_CH4, Tim3Ch4, tim3_ch4, TIM3, Tim3, TIM3_CH4_CH, TimGenCh, TIM3_PERIPH, TIM3_CH4_OWNED, TIM3_CH4_REF_COUNT, 3);
channel!(TIM4_CH1, Tim4Ch1, tim4_ch1, TIM4, Tim4, TIM4_CH1_CH, TimGenCh, TIM4_PERIPH, TIM4_CH1_OWNED, TIM4_CH1_REF_COUNT, 0);
channel!(TIM4_CH2, Tim4Ch2, tim4_ch2, TIM4, Tim4, TIM4_CH2_CH, TimGenCh, TIM4_PERIPH, TIM4_CH2_OWNED, TIM4_CH2_REF_COUNT, 1);
channel!(TIM4_CH3, Tim4Ch3, tim4_ch3, TIM4, Tim4, TIM4_CH3_CH, TimGenCh, TIM4_PERIPH, TIM4_CH3_OWNED, TIM4_CH3_REF_COUNT, 2);
channel!(TIM4_CH4, Tim4Ch4, tim4_ch4, TIM4, Tim4, TIM4_CH4_CH, TimGenCh, TIM4_PERIPH, TIM4_CH4_OWNED, TIM4_CH4_REF_COUNT, 3);
channel!(TIM15_CH1, Tim15Ch1, tim15_ch1, TIM15, Tim15, TIM15_CH1_CH, TimGenCh, TIM15_PERIPH, TIM15_CH1_OWNED, TIM15_CH1_REF_COUNT, 0);
channel!(TIM15_CH2, Tim15Ch2, tim15_ch2, TIM15, Tim15, TIM15_CH2_CH, TimGenCh, TIM15_PERIPH, TIM15_CH2_OWNED, TIM15_CH2_REF_COUNT, 1);
channel!(TIM16_CH1, Tim16Ch1, tim16_ch1, TIM16, Tim16, TIM16_CH1_CH, TimGenCh, TIM16_PERIPH, TIM16_CH1_OWNED, TIM16_CH1_REF_COUNT, 0);
channel!(TIM17_CH1, Tim17Ch1, tim17_ch1, TIM17, Tim17, TIM17_CH1_CH, TimGenCh, TIM17_PERIPH, TIM17_CH1_OWNED, TIM17_CH1_REF_COUNT, 0);
// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("TIM2RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim2 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().tim2rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_tim2rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("TIM2EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim2 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().tim2en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_tim2en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("TIM3RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim3 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().tim3rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_tim3rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("TIM3EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim3 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().tim3en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_tim3en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("TIM4RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim4 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().tim4rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_tim4rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("TIM4EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim4 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().tim4en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_tim4en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("TIM15RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim15 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().tim15rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_tim15rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("TIM15EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim15 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().tim15en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_tim15en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("TIM16RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim16 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().tim16rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_tim16rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("TIM16EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim16 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().tim16en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_tim16en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("TIM17RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim17 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().tim17rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_tim17rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("TIM17EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim17 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().tim17en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_tim17en(value));
        self
    }
}

