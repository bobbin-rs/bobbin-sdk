pub use ::stm32_common::crc_24::*;

::bobbin_mcu::periph!( CRC, Crc, CRC_PERIPH, CrcPeriph, CRC_OWNED, CRC_REF_COUNT, 0x40023000, 0x00, 0x0c);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB1RSTR"), field: Some("CRCRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Crc {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb1rstr().crcrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1rstr(|r| r.set_crcrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB1ENR"), field: Some("CRCEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Crc {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb1enr().crcen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1enr(|r| r.set_crcen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB1LPENR"), field: Some("CRCLPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Crc {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb1lpenr().crclpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1lpenr(|r| r.set_crclpen(value));
        self
    }
}

