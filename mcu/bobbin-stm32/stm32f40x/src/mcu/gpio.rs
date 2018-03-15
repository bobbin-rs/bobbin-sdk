#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x40020000, 0x2a);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x40020400, 0x2b);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x40020800, 0x2c);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x40020c00, 0x2d);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x40021000, 0x2e);
periph!( GPIOF, Gpiof, GPIOF_PERIPH, GpioPeriph, 0x40021400, 0x2f);
periph!( GPIOG, Gpiog, GPIOG_PERIPH, GpioPeriph, 0x40021800, 0x30);
periph!( GPIOH, Gpioh, GPIOH_PERIPH, GpioPeriph, 0x40021c00, 0x31);
periph!( GPIOI, Gpioi, GPIOI_PERIPH, GpioPeriph, 0x40022000, 0x32);

