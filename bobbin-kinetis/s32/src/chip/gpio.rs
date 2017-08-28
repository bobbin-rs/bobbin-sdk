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

impl LinkPort<super::port::Porta > for Gpioa {
   fn port(&self) -> super::port::Porta  { super::port::PORTA }
}


impl LinkPort<super::port::Portb > for Gpiob {
   fn port(&self) -> super::port::Portb  { super::port::PORTB }
}


impl LinkPort<super::port::Portc > for Gpioc {
   fn port(&self) -> super::port::Portc  { super::port::PORTC }
}


impl LinkPort<super::port::Portd > for Gpiod {
   fn port(&self) -> super::port::Portd  { super::port::PORTD }
}


impl LinkPort<super::port::Porte > for Gpioe {
   fn port(&self) -> super::port::Porte  { super::port::PORTE }
}



