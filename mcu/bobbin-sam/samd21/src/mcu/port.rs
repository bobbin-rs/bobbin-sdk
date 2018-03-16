#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::port::*;

// //! I/O Pin Controller

periph!( PORTA, Porta, PORTA_PERIPH, PortPeriph, 0x41004400, 0x0e);
periph!( PORTB, Portb, PORTB_PERIPH, PortPeriph, 0x41004480, 0x0f);

