#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x400ff000, 0x1c);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x400ff040, 0x1d);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x400ff080, 0x1e);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x400ff0c0, 0x1f);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x400ff100, 0x20);

