#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::port::*;

periph!( PORTA, Porta, PORTA_PERIPH, PortPeriph, 0x40049000, 0x21);
periph!( PORTB, Portb, PORTB_PERIPH, PortPeriph, 0x4004a000, 0x22);
periph!( PORTC, Portc, PORTC_PERIPH, PortPeriph, 0x4004b000, 0x23);
periph!( PORTD, Portd, PORTD_PERIPH, PortPeriph, 0x4004c000, 0x24);
periph!( PORTE, Porte, PORTE_PERIPH, PortPeriph, 0x4004d000, 0x25);

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
impl ::bobbin_common::gate::GateEn for Porta {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc5().porta() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_porta(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC5"), field: Some("PORTB"), description: None }
impl ::bobbin_common::gate::GateEn for Portb {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc5().portb() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_portb(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC5"), field: Some("PORTC"), description: None }
impl ::bobbin_common::gate::GateEn for Portc {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc5().portc() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_portc(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC5"), field: Some("PORTD"), description: None }
impl ::bobbin_common::gate::GateEn for Portd {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc5().portd() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_portd(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC5"), field: Some("PORTE"), description: None }
impl ::bobbin_common::gate::GateEn for Porte {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc5().porte() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_porte(value));
        self
    }
}

