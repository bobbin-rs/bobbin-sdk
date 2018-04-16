#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::port::*;

periph!( PORTA, Porta, PORTA_PERIPH, PortPeriph, PORTA_OWNED, PORTA_REF_COUNT, 0x40049000, 0x00, 0x10);
periph!( PORTB, Portb, PORTB_PERIPH, PortPeriph, PORTB_OWNED, PORTB_REF_COUNT, 0x4004a000, 0x01, 0x11);
periph!( PORTC, Portc, PORTC_PERIPH, PortPeriph, PORTC_OWNED, PORTC_REF_COUNT, 0x4004b000, 0x02, 0x12);
periph!( PORTD, Portd, PORTD_PERIPH, PortPeriph, PORTD_OWNED, PORTD_REF_COUNT, 0x4004c000, 0x03, 0x13);
periph!( PORTE, Porte, PORTE_PERIPH, PortPeriph, PORTE_OWNED, PORTE_REF_COUNT, 0x4004d000, 0x04, 0x14);

