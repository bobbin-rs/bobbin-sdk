#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x40010800, 0x18);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x40010c00, 0x19);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x40011000, 0x1a);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x40011400, 0x1b);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x40011800, 0x1c);
periph!( GPIOF, Gpiof, GPIOF_PERIPH, GpioPeriph, 0x40011c00, 0x1d);
periph!( GPIOG, Gpiog, GPIOG_PERIPH, GpioPeriph, 0x40012000, 0x1e);

