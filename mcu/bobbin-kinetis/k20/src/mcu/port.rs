#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::port::*;

periph!( PORTA, Porta, PORTA_PERIPH, PortPeriph, 0x40049000, 0x1a);
periph!( PORTB, Portb, PORTB_PERIPH, PortPeriph, 0x4004a000, 0x1b);
periph!( PORTC, Portc, PORTC_PERIPH, PortPeriph, 0x4004b000, 0x1c);
periph!( PORTD, Portd, PORTD_PERIPH, PortPeriph, 0x4004c000, 0x1d);
periph!( PORTE, Porte, PORTE_PERIPH, PortPeriph, 0x4004d000, 0x1e);

