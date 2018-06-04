pub use ::stm32_common::gpio::*;

::bobbin_mcu::periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, GPIOA_OWNED, GPIOA_REF_COUNT, 0x40020000, 0x00, 0x2a);
::bobbin_mcu::periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, GPIOB_OWNED, GPIOB_REF_COUNT, 0x40020400, 0x01, 0x2b);
::bobbin_mcu::periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, GPIOC_OWNED, GPIOC_REF_COUNT, 0x40020800, 0x02, 0x2c);
::bobbin_mcu::periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, GPIOD_OWNED, GPIOD_REF_COUNT, 0x40020c00, 0x03, 0x2d);
::bobbin_mcu::periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, GPIOE_OWNED, GPIOE_REF_COUNT, 0x40021000, 0x04, 0x2e);
::bobbin_mcu::periph!( GPIOF, Gpiof, GPIOF_PERIPH, GpioPeriph, GPIOF_OWNED, GPIOF_REF_COUNT, 0x40021400, 0x05, 0x2f);
::bobbin_mcu::periph!( GPIOG, Gpiog, GPIOG_PERIPH, GpioPeriph, GPIOG_OWNED, GPIOG_REF_COUNT, 0x40021800, 0x06, 0x30);
::bobbin_mcu::periph!( GPIOH, Gpioh, GPIOH_PERIPH, GpioPeriph, GPIOH_OWNED, GPIOH_REF_COUNT, 0x40021c00, 0x07, 0x31);
::bobbin_mcu::periph!( GPIOI, Gpioi, GPIOI_PERIPH, GpioPeriph, GPIOI_OWNED, GPIOI_REF_COUNT, 0x40022000, 0x08, 0x32);

