pub use ::stm32_common::iwdg::*;

::bobbin_mcu::periph!( IWDG, Iwdg, IWDG_PERIPH, IwdgPeriph, IWDG_OWNED, IWDG_REF_COUNT, 0x40003000, 0x00, 0x18);

