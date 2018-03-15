#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x48000000, 0x15);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x48000400, 0x16);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x48000800, 0x17);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x48000c00, 0x18);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x48001000, 0x19);
periph!( GPIOF, Gpiof, GPIOF_PERIPH, GpioPeriph, 0x48001400, 0x1a);
periph!( GPIOG, Gpiog, GPIOG_PERIPH, GpioPeriph, 0x48001800, 0x1b);
periph!( GPIOH, Gpioh, GPIOH_PERIPH, GpioPeriph, 0x48001c00, 0x1c);

