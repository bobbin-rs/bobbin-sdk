pub use ::stm32_common::tim_gen::*;

::bobbin_mcu::periph!( TIM2, Tim2, TIM2_PERIPH, TimGenPeriph, TIM2_OWNED, TIM2_REF_COUNT, 0x40000000, 0x00, 0x17);
::bobbin_mcu::periph!( TIM3, Tim3, TIM3_PERIPH, TimGenPeriph, TIM3_OWNED, TIM3_REF_COUNT, 0x40000400, 0x01, 0x18);
::bobbin_mcu::periph!( TIM4, Tim4, TIM4_PERIPH, TimGenPeriph, TIM4_OWNED, TIM4_REF_COUNT, 0x40000800, 0x02, 0x19);
::bobbin_mcu::periph!( TIM5, Tim5, TIM5_PERIPH, TimGenPeriph, TIM5_OWNED, TIM5_REF_COUNT, 0x40000c00, 0x03, 0x1a);
::bobbin_mcu::periph!( TIM15, Tim15, TIM15_PERIPH, TimGenPeriph, TIM15_OWNED, TIM15_REF_COUNT, 0x40014000, 0x04, 0x1b);
::bobbin_mcu::periph!( TIM16, Tim16, TIM16_PERIPH, TimGenPeriph, TIM16_OWNED, TIM16_REF_COUNT, 0x40014400, 0x05, 0x1c);

::bobbin_mcu::channel!(TIM2_CH1, Tim2Ch1, tim2_ch1, TIM2, Tim2, TIM2_CH1_CH, TimGenCh, TIM2_PERIPH, TIM2_CH1_OWNED, TIM2_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(TIM2_CH2, Tim2Ch2, tim2_ch2, TIM2, Tim2, TIM2_CH2_CH, TimGenCh, TIM2_PERIPH, TIM2_CH2_OWNED, TIM2_CH2_REF_COUNT, 1);
::bobbin_mcu::channel!(TIM2_CH3, Tim2Ch3, tim2_ch3, TIM2, Tim2, TIM2_CH3_CH, TimGenCh, TIM2_PERIPH, TIM2_CH3_OWNED, TIM2_CH3_REF_COUNT, 2);
::bobbin_mcu::channel!(TIM2_CH4, Tim2Ch4, tim2_ch4, TIM2, Tim2, TIM2_CH4_CH, TimGenCh, TIM2_PERIPH, TIM2_CH4_OWNED, TIM2_CH4_REF_COUNT, 3);
::bobbin_mcu::channel!(TIM3_CH1, Tim3Ch1, tim3_ch1, TIM3, Tim3, TIM3_CH1_CH, TimGenCh, TIM3_PERIPH, TIM3_CH1_OWNED, TIM3_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(TIM3_CH2, Tim3Ch2, tim3_ch2, TIM3, Tim3, TIM3_CH2_CH, TimGenCh, TIM3_PERIPH, TIM3_CH2_OWNED, TIM3_CH2_REF_COUNT, 1);
::bobbin_mcu::channel!(TIM3_CH3, Tim3Ch3, tim3_ch3, TIM3, Tim3, TIM3_CH3_CH, TimGenCh, TIM3_PERIPH, TIM3_CH3_OWNED, TIM3_CH3_REF_COUNT, 2);
::bobbin_mcu::channel!(TIM3_CH4, Tim3Ch4, tim3_ch4, TIM3, Tim3, TIM3_CH4_CH, TimGenCh, TIM3_PERIPH, TIM3_CH4_OWNED, TIM3_CH4_REF_COUNT, 3);
::bobbin_mcu::channel!(TIM4_CH1, Tim4Ch1, tim4_ch1, TIM4, Tim4, TIM4_CH1_CH, TimGenCh, TIM4_PERIPH, TIM4_CH1_OWNED, TIM4_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(TIM4_CH2, Tim4Ch2, tim4_ch2, TIM4, Tim4, TIM4_CH2_CH, TimGenCh, TIM4_PERIPH, TIM4_CH2_OWNED, TIM4_CH2_REF_COUNT, 1);
::bobbin_mcu::channel!(TIM4_CH3, Tim4Ch3, tim4_ch3, TIM4, Tim4, TIM4_CH3_CH, TimGenCh, TIM4_PERIPH, TIM4_CH3_OWNED, TIM4_CH3_REF_COUNT, 2);
::bobbin_mcu::channel!(TIM4_CH4, Tim4Ch4, tim4_ch4, TIM4, Tim4, TIM4_CH4_CH, TimGenCh, TIM4_PERIPH, TIM4_CH4_OWNED, TIM4_CH4_REF_COUNT, 3);
::bobbin_mcu::channel!(TIM5_CH1, Tim5Ch1, tim5_ch1, TIM5, Tim5, TIM5_CH1_CH, TimGenCh, TIM5_PERIPH, TIM5_CH1_OWNED, TIM5_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(TIM5_CH2, Tim5Ch2, tim5_ch2, TIM5, Tim5, TIM5_CH2_CH, TimGenCh, TIM5_PERIPH, TIM5_CH2_OWNED, TIM5_CH2_REF_COUNT, 1);
::bobbin_mcu::channel!(TIM5_CH3, Tim5Ch3, tim5_ch3, TIM5, Tim5, TIM5_CH3_CH, TimGenCh, TIM5_PERIPH, TIM5_CH3_OWNED, TIM5_CH3_REF_COUNT, 2);
::bobbin_mcu::channel!(TIM5_CH4, Tim5Ch4, tim5_ch4, TIM5, Tim5, TIM5_CH4_CH, TimGenCh, TIM5_PERIPH, TIM5_CH4_OWNED, TIM5_CH4_REF_COUNT, 3);
::bobbin_mcu::channel!(TIM15_CH1, Tim15Ch1, tim15_ch1, TIM15, Tim15, TIM15_CH1_CH, TimGenCh, TIM15_PERIPH, TIM15_CH1_OWNED, TIM15_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(TIM15_CH2, Tim15Ch2, tim15_ch2, TIM15, Tim15, TIM15_CH2_CH, TimGenCh, TIM15_PERIPH, TIM15_CH2_OWNED, TIM15_CH2_REF_COUNT, 1);
::bobbin_mcu::channel!(TIM16_CH1, Tim16Ch1, tim16_ch1, TIM16, Tim16, TIM16_CH1_CH, TimGenCh, TIM16_PERIPH, TIM16_CH1_OWNED, TIM16_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(TIM16_CH2, Tim16Ch2, tim16_ch2, TIM16, Tim16, TIM16_CH2_CH, TimGenCh, TIM16_PERIPH, TIM16_CH2_OWNED, TIM16_CH2_REF_COUNT, 1);
// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR1"), field: Some("TIM2RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Tim2 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr1().tim2rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr1(|r| r.set_tim2rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR1"), field: Some("TIM2EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Tim2 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr1().tim2en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr1(|r| r.set_tim2en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("TIM2SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Tim2 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1smenr1().tim2smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_tim2smen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("TIM15RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Tim15 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2rstr().tim15rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_tim15rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("TIM15EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Tim15 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2enr().tim15en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_tim15en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2SMENR"), field: Some("TIM15SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Tim15 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2smenr().tim15smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2smenr(|r| r.set_tim15smen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("TIM16RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Tim16 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2rstr().tim16rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_tim16rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("TIM16EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Tim16 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2enr().tim16en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_tim16en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2SMENR"), field: Some("TIM16SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Tim16 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2smenr().tim16smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2smenr(|r| r.set_tim16smen(value));
        self
    }
}

