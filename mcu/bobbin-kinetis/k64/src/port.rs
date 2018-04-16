pub use kinetis_common::port::*;

::bobbin_mcu::periph!( PORTA, Porta, PORTA_PERIPH, PortPeriph, PORTA_OWNED, PORTA_REF_COUNT, 0x40049000, 0x00, 0x24);
::bobbin_mcu::periph!( PORTB, Portb, PORTB_PERIPH, PortPeriph, PORTB_OWNED, PORTB_REF_COUNT, 0x4004a000, 0x01, 0x25);
::bobbin_mcu::periph!( PORTC, Portc, PORTC_PERIPH, PortPeriph, PORTC_OWNED, PORTC_REF_COUNT, 0x4004b000, 0x02, 0x26);
::bobbin_mcu::periph!( PORTD, Portd, PORTD_PERIPH, PortPeriph, PORTD_OWNED, PORTD_REF_COUNT, 0x4004c000, 0x03, 0x27);
::bobbin_mcu::periph!( PORTE, Porte, PORTE_PERIPH, PortPeriph, PORTE_OWNED, PORTE_REF_COUNT, 0x4004d000, 0x04, 0x28);

pub trait LinkGpio<T> {
    fn gpio(&self) -> T;
}

impl LinkGpio<super::gpio::Gpioa > for Porta {
    #[inline] fn gpio(&self) -> super::gpio::Gpioa  { super::gpio::GPIOA }
}

impl LinkGpio<super::gpio::Gpiob > for Portb {
    #[inline] fn gpio(&self) -> super::gpio::Gpiob  { super::gpio::GPIOB }
}

impl LinkGpio<super::gpio::Gpioc > for Portc {
    #[inline] fn gpio(&self) -> super::gpio::Gpioc  { super::gpio::GPIOC }
}

impl LinkGpio<super::gpio::Gpiod > for Portd {
    #[inline] fn gpio(&self) -> super::gpio::Gpiod  { super::gpio::GPIOD }
}

impl LinkGpio<super::gpio::Gpioe > for Porte {
    #[inline] fn gpio(&self) -> super::gpio::Gpioe  { super::gpio::GPIOE }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC5"), field: Some("PORTA"), description: None }
impl ::bobbin_mcu::gate::GateEn for Porta {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc5().porta() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_porta(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC5"), field: Some("PORTB"), description: None }
impl ::bobbin_mcu::gate::GateEn for Portb {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc5().portb() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_portb(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC5"), field: Some("PORTC"), description: None }
impl ::bobbin_mcu::gate::GateEn for Portc {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc5().portc() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_portc(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC5"), field: Some("PORTD"), description: None }
impl ::bobbin_mcu::gate::GateEn for Portd {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc5().portd() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_portd(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC5"), field: Some("PORTE"), description: None }
impl ::bobbin_mcu::gate::GateEn for Porte {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc5().porte() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_porte(value));
        self
    }
}

pub struct PortPin { pub port: PortPeriph, pub index: usize }

