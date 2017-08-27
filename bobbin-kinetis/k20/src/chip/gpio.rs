#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::gpio::*;

pub trait LinkPort<T> {
   fn port(&self) -> T;
}

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x400ff000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x400ff040);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x400ff080);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x400ff0c0);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x400ff100);

impl LinkPort<super::port::Periph<super::port::PortaId>> for Gpioa {
   fn port(&self) -> super::port::Periph<super::port::PortaId> { super::port::PORTA }
}


impl LinkPort<super::port::Periph<super::port::PortbId>> for Gpiob {
   fn port(&self) -> super::port::Periph<super::port::PortbId> { super::port::PORTB }
}


impl LinkPort<super::port::Periph<super::port::PortcId>> for Gpioc {
   fn port(&self) -> super::port::Periph<super::port::PortcId> { super::port::PORTC }
}


impl LinkPort<super::port::Periph<super::port::PortdId>> for Gpiod {
   fn port(&self) -> super::port::Periph<super::port::PortdId> { super::port::PORTD }
}


impl LinkPort<super::port::Periph<super::port::PorteId>> for Gpioe {
   fn port(&self) -> super::port::Periph<super::port::PorteId> { super::port::PORTE }
}



