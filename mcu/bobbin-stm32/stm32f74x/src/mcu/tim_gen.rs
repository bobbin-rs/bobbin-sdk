#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::tim_gen::*;

periph!( TIM2, Tim2, TIM2_PERIPH, TimGenPeriph, 0x40000000, 0x00, 0x1e);
periph!( TIM3, Tim3, TIM3_PERIPH, TimGenPeriph, 0x40000400, 0x01, 0x1f);
periph!( TIM4, Tim4, TIM4_PERIPH, TimGenPeriph, 0x40000800, 0x02, 0x20);
periph!( TIM5, Tim5, TIM5_PERIPH, TimGenPeriph, 0x40000c00, 0x03, 0x21);
periph!( TIM9, Tim9, TIM9_PERIPH, TimGenPeriph, 0x40014000, 0x04, 0x22);
periph!( TIM10, Tim10, TIM10_PERIPH, TimGenPeriph, 0x40014400, 0x05, 0x23);
periph!( TIM11, Tim11, TIM11_PERIPH, TimGenPeriph, 0x40014800, 0x06, 0x24);
periph!( TIM12, Tim12, TIM12_PERIPH, TimGenPeriph, 0x40001800, 0x07, 0x25);
periph!( TIM13, Tim13, TIM13_PERIPH, TimGenPeriph, 0x40001c00, 0x08, 0x26);
periph!( TIM14, Tim14, TIM14_PERIPH, TimGenPeriph, 0x40002000, 0x09, 0x27);

channel!(TIM2_CH1, Tim2Ch1, TIM2, Tim2, TIM2_CH1_CH, TimGenCh, TIM2_PERIPH, 0);
channel!(TIM2_CH2, Tim2Ch2, TIM2, Tim2, TIM2_CH2_CH, TimGenCh, TIM2_PERIPH, 1);
channel!(TIM2_CH3, Tim2Ch3, TIM2, Tim2, TIM2_CH3_CH, TimGenCh, TIM2_PERIPH, 2);
channel!(TIM2_CH4, Tim2Ch4, TIM2, Tim2, TIM2_CH4_CH, TimGenCh, TIM2_PERIPH, 3);
channel!(TIM3_CH1, Tim3Ch1, TIM3, Tim3, TIM3_CH1_CH, TimGenCh, TIM3_PERIPH, 0);
channel!(TIM3_CH2, Tim3Ch2, TIM3, Tim3, TIM3_CH2_CH, TimGenCh, TIM3_PERIPH, 1);
channel!(TIM3_CH3, Tim3Ch3, TIM3, Tim3, TIM3_CH3_CH, TimGenCh, TIM3_PERIPH, 2);
channel!(TIM3_CH4, Tim3Ch4, TIM3, Tim3, TIM3_CH4_CH, TimGenCh, TIM3_PERIPH, 3);
channel!(TIM4_CH1, Tim4Ch1, TIM4, Tim4, TIM4_CH1_CH, TimGenCh, TIM4_PERIPH, 0);
channel!(TIM4_CH2, Tim4Ch2, TIM4, Tim4, TIM4_CH2_CH, TimGenCh, TIM4_PERIPH, 1);
channel!(TIM4_CH3, Tim4Ch3, TIM4, Tim4, TIM4_CH3_CH, TimGenCh, TIM4_PERIPH, 2);
channel!(TIM4_CH4, Tim4Ch4, TIM4, Tim4, TIM4_CH4_CH, TimGenCh, TIM4_PERIPH, 3);
channel!(TIM5_CH1, Tim5Ch1, TIM5, Tim5, TIM5_CH1_CH, TimGenCh, TIM5_PERIPH, 0);
channel!(TIM5_CH2, Tim5Ch2, TIM5, Tim5, TIM5_CH2_CH, TimGenCh, TIM5_PERIPH, 1);
channel!(TIM5_CH3, Tim5Ch3, TIM5, Tim5, TIM5_CH3_CH, TimGenCh, TIM5_PERIPH, 2);
channel!(TIM5_CH4, Tim5Ch4, TIM5, Tim5, TIM5_CH4_CH, TimGenCh, TIM5_PERIPH, 3);
channel!(TIM9_CH1, Tim9Ch1, TIM9, Tim9, TIM9_CH1_CH, TimGenCh, TIM9_PERIPH, 0);
channel!(TIM9_CH2, Tim9Ch2, TIM9, Tim9, TIM9_CH2_CH, TimGenCh, TIM9_PERIPH, 1);
channel!(TIM10_CH1, Tim10Ch1, TIM10, Tim10, TIM10_CH1_CH, TimGenCh, TIM10_PERIPH, 0);
channel!(TIM11_CH1, Tim11Ch1, TIM11, Tim11, TIM11_CH1_CH, TimGenCh, TIM11_PERIPH, 0);
channel!(TIM12_CH1, Tim12Ch1, TIM12, Tim12, TIM12_CH1_CH, TimGenCh, TIM12_PERIPH, 0);
channel!(TIM12_CH2, Tim12Ch2, TIM12, Tim12, TIM12_CH2_CH, TimGenCh, TIM12_PERIPH, 1);
channel!(TIM13_CH1, Tim13Ch1, TIM13, Tim13, TIM13_CH1_CH, TimGenCh, TIM13_PERIPH, 0);
channel!(TIM14_CH1, Tim14Ch1, TIM14, Tim14, TIM14_CH1_CH, TimGenCh, TIM14_PERIPH, 0);
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

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("TIM2LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Tim2 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1lpenr().tim2lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_tim2lpen(value));
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

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("TIM3LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Tim3 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1lpenr().tim3lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_tim3lpen(value));
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

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("TIM4LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Tim4 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1lpenr().tim4lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_tim4lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("TIM5RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim5 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().tim5rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_tim5rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("TIM5EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim5 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().tim5en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_tim5en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("TIM5LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Tim5 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1lpenr().tim5lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_tim5lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("TIM9RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim9 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().tim9rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_tim9rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("TIM9EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim9 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().tim9en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_tim9en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2LPENR"), field: Some("TIM9LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Tim9 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb2lpenr().tim9lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2lpenr(|r| r.set_tim9lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("TIM10RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim10 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().tim10rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_tim10rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("TIM10EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim10 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().tim10en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_tim10en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2LPENR"), field: Some("TIM10LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Tim10 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb2lpenr().tim10lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2lpenr(|r| r.set_tim10lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("TIM11RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim11 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().tim11rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_tim11rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("TIM11EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim11 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().tim11en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_tim11en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2LPENR"), field: Some("TIM11LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Tim11 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb2lpenr().tim11lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2lpenr(|r| r.set_tim11lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("TIM12RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim12 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().tim12rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_tim12rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("TIM12EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim12 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().tim12en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_tim12en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("TIM12LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Tim12 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1lpenr().tim12lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_tim12lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("TIM13RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim13 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().tim13rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_tim13rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("TIM13EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim13 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().tim13en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_tim13en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("TIM13LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Tim13 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1lpenr().tim13lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_tim13lpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("TIM14RST"), description: None }
impl ::bobbin_common::gate::GateRst for Tim14 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().tim14rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_tim14rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("TIM14EN"), description: None }
impl ::bobbin_common::gate::GateEn for Tim14 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().tim14en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_tim14en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("TIM14LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Tim14 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1lpenr().tim14lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_tim14lpen(value));
        self
    }
}

