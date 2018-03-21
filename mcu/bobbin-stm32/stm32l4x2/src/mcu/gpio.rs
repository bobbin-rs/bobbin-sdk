#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x48000000, 0x00, 0x15);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x48000400, 0x01, 0x16);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x48000800, 0x02, 0x17);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x48000c00, 0x03, 0x18);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x48001000, 0x04, 0x19);
periph!( GPIOF, Gpiof, GPIOF_PERIPH, GpioPeriph, 0x48001400, 0x05, 0x1a);
periph!( GPIOG, Gpiog, GPIOG_PERIPH, GpioPeriph, 0x48001800, 0x06, 0x1b);
periph!( GPIOH, Gpioh, GPIOH_PERIPH, GpioPeriph, 0x48001c00, 0x07, 0x1c);

