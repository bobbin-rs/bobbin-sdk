pub use ::stm32_common::tim_adv::*;

::bobbin_mcu::periph!( TIM1, Tim1, TIM1_PERIPH, TimAdvPeriph, TIM1_OWNED, TIM1_REF_COUNT, 0x40010000, 0x00, 0x1a);
::bobbin_mcu::periph!( TIM8, Tim8, TIM8_PERIPH, TimAdvPeriph, TIM8_OWNED, TIM8_REF_COUNT, 0x40010400, 0x01, 0x1b);

::bobbin_mcu::channel!(TIM1_CH1, Tim1Ch1, tim1_ch1, TIM1, Tim1, TIM1_CH1_CH, TimAdvCh, TIM1_PERIPH, TIM1_CH1_OWNED, TIM1_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(TIM1_CH2, Tim1Ch2, tim1_ch2, TIM1, Tim1, TIM1_CH2_CH, TimAdvCh, TIM1_PERIPH, TIM1_CH2_OWNED, TIM1_CH2_REF_COUNT, 1);
::bobbin_mcu::channel!(TIM1_CH3, Tim1Ch3, tim1_ch3, TIM1, Tim1, TIM1_CH3_CH, TimAdvCh, TIM1_PERIPH, TIM1_CH3_OWNED, TIM1_CH3_REF_COUNT, 2);
::bobbin_mcu::channel!(TIM1_CH4, Tim1Ch4, tim1_ch4, TIM1, Tim1, TIM1_CH4_CH, TimAdvCh, TIM1_PERIPH, TIM1_CH4_OWNED, TIM1_CH4_REF_COUNT, 3);
::bobbin_mcu::channel!(TIM8_CH1, Tim8Ch1, tim8_ch1, TIM8, Tim8, TIM8_CH1_CH, TimAdvCh, TIM8_PERIPH, TIM8_CH1_OWNED, TIM8_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(TIM8_CH2, Tim8Ch2, tim8_ch2, TIM8, Tim8, TIM8_CH2_CH, TimAdvCh, TIM8_PERIPH, TIM8_CH2_OWNED, TIM8_CH2_REF_COUNT, 1);
::bobbin_mcu::channel!(TIM8_CH3, Tim8Ch3, tim8_ch3, TIM8, Tim8, TIM8_CH3_CH, TimAdvCh, TIM8_PERIPH, TIM8_CH3_OWNED, TIM8_CH3_REF_COUNT, 2);
::bobbin_mcu::channel!(TIM8_CH4, Tim8Ch4, tim8_ch4, TIM8, Tim8, TIM8_CH4_CH, TimAdvCh, TIM8_PERIPH, TIM8_CH4_OWNED, TIM8_CH4_REF_COUNT, 3);
