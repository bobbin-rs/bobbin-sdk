pub use ::stm32_common::tim_bas::*;

::bobbin_mcu::periph!( TIM6, Tim6, TIM6_PERIPH, TimBasPeriph, TIM6_OWNED, TIM6_REF_COUNT, 0x40001000, 0x00, 0x0e);
::bobbin_mcu::periph!( TIM7, Tim7, TIM7_PERIPH, TimBasPeriph, TIM7_OWNED, TIM7_REF_COUNT, 0x40001400, 0x01, 0x0f);

