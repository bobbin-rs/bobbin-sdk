#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::tim_gen::*;

periph!( TIM2, Tim2, TIM2_PERIPH, TimGenPeriph, 0x40000000, 0x0d);
periph!( TIM3, Tim3, TIM3_PERIPH, TimGenPeriph, 0x40000400, 0x0e);
periph!( TIM4, Tim4, TIM4_PERIPH, TimGenPeriph, 0x40000800, 0x0f);

channel!(TIM2_CH1, Tim2Ch1, TIM2, Tim2, TIM2_CH1_CH, TimGenCh, TIM2_PERIPH, 0);
channel!(TIM2_CH2, Tim2Ch2, TIM2, Tim2, TIM2_CH2_CH, TimGenCh, TIM2_PERIPH, 1);
channel!(TIM2_CH3, Tim2Ch3, TIM2, Tim2, TIM2_CH3_CH, TimGenCh, TIM2_PERIPH, 2);
channel!(TIM2_CH4, Tim2Ch4, TIM2, Tim2, TIM2_CH4_CH, TimGenCh, TIM2_PERIPH, 3);
channel!(TIM3_CH1, Tim3Ch1, TIM3, Tim3, TIM3_CH1_CH, TimGenCh, TIM3_PERIPH, 0);
channel!(TIM3_CH2, Tim3Ch2, TIM3, Tim3, TIM3_CH2_CH, TimGenCh, TIM3_PERIPH, 1);
channel!(TIM3_CH3, Tim3Ch3, TIM3, Tim3, TIM3_CH3_CH, TimGenCh, TIM3_PERIPH, 2);
channel!(TIM3_CH4, Tim3Ch4, TIM3, Tim3, TIM3_CH4_CH, TimGenCh, TIM3_PERIPH, 3);
channel!(TIM4_CH1, Tim4Ch1, TIM4, Tim4, TIM4_CH1_CH, TimGenCh, TIM4_PERIPH, 0);
channel!(TIM4_CH2, Tim4Ch2, TIM4, Tim4, TIM4_CH2_CH, TimGenCh, TIM4_PERIPH, 1);
channel!(TIM4_CH3, Tim4Ch3, TIM4, Tim4, TIM4_CH3_CH, TimGenCh, TIM4_PERIPH, 2);
channel!(TIM4_CH4, Tim4Ch4, TIM4, Tim4, TIM4_CH4_CH, TimGenCh, TIM4_PERIPH, 3);
