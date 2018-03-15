#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::tim_adv::*;

periph!( TIM1, Tim1, TIM1_PERIPH, TimAdvPeriph, 0x40010000, 0x1a);
periph!( TIM8, Tim8, TIM8_PERIPH, TimAdvPeriph, 0x40010400, 0x1b);

channel!(TIM1_CH1, Tim1Ch1, TIM1, Tim1, TIM1_CH1_CH, TimAdvCh, TIM1_PERIPH, 0);
channel!(TIM1_CH2, Tim1Ch2, TIM1, Tim1, TIM1_CH2_CH, TimAdvCh, TIM1_PERIPH, 1);
channel!(TIM1_CH3, Tim1Ch3, TIM1, Tim1, TIM1_CH3_CH, TimAdvCh, TIM1_PERIPH, 2);
channel!(TIM1_CH4, Tim1Ch4, TIM1, Tim1, TIM1_CH4_CH, TimAdvCh, TIM1_PERIPH, 3);
channel!(TIM8_CH1, Tim8Ch1, TIM8, Tim8, TIM8_CH1_CH, TimAdvCh, TIM8_PERIPH, 0);
channel!(TIM8_CH2, Tim8Ch2, TIM8, Tim8, TIM8_CH2_CH, TimAdvCh, TIM8_PERIPH, 1);
channel!(TIM8_CH3, Tim8Ch3, TIM8, Tim8, TIM8_CH3_CH, TimAdvCh, TIM8_PERIPH, 2);
channel!(TIM8_CH4, Tim8Ch4, TIM8, Tim8, TIM8_CH4_CH, TimAdvCh, TIM8_PERIPH, 3);
