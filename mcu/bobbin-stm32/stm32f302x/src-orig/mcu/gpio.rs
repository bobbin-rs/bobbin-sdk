#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x48000000, 0x09);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x48000400, 0x0a);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x48000800, 0x0b);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x48000c00, 0x0c);
periph!( GPIOF, Gpiof, GPIOF_PERIPH, GpioPeriph, 0x48001400, 0x0d);

