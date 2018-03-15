#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x400ff000, 0x18);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x400ff040, 0x19);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x400ff080, 0x1a);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x400ff0c0, 0x1b);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x400ff100, 0x1c);

