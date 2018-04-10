#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, GPIOA_OWNED, GPIOA_REF_COUNT, 0x48000000, 0x00, 0x09);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, GPIOB_OWNED, GPIOB_REF_COUNT, 0x48000400, 0x01, 0x0a);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, GPIOC_OWNED, GPIOC_REF_COUNT, 0x48000800, 0x02, 0x0b);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, GPIOD_OWNED, GPIOD_REF_COUNT, 0x48000c00, 0x03, 0x0c);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, GPIOE_OWNED, GPIOE_REF_COUNT, 0x48001000, 0x04, 0x0d);
periph!( GPIOF, Gpiof, GPIOF_PERIPH, GpioPeriph, GPIOF_OWNED, GPIOF_REF_COUNT, 0x48001400, 0x05, 0x0e);
periph!( GPIOG, Gpiog, GPIOG_PERIPH, GpioPeriph, GPIOG_OWNED, GPIOG_REF_COUNT, 0x48001800, 0x06, 0x0f);
periph!( GPIOH, Gpioh, GPIOH_PERIPH, GpioPeriph, GPIOH_OWNED, GPIOH_REF_COUNT, 0x48001c00, 0x07, 0x10);

