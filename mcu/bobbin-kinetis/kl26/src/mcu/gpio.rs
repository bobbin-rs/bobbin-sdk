#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x400ff000, 0x15);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x400ff040, 0x16);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x400ff080, 0x17);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x400ff0c0, 0x18);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x400ff100, 0x19);

