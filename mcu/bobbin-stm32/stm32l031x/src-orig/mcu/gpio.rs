#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x50000000, 0x0f);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x50000400, 0x10);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x50000800, 0x11);
periph!( GPIOH, Gpioh, GPIOH_PERIPH, GpioPeriph, 0x50001c00, 0x12);

