#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::tim_gen::*;

periph!( TIM2, Tim2, TIM2_PERIPH, TimGenPeriph, 0x40000000, 0x0c);
periph!( TIM21, Tim21, TIM21_PERIPH, TimGenPeriph, 0x40010800, 0x0d);
periph!( TIM22, Tim22, TIM22_PERIPH, TimGenPeriph, 0x40011400, 0x0e);

channel!(TIM2_CH1, Tim2Ch1, TIM2, Tim2, TIM2_CH1_CH, TimGenCh, TIM2_PERIPH, 0);
channel!(TIM2_CH2, Tim2Ch2, TIM2, Tim2, TIM2_CH2_CH, TimGenCh, TIM2_PERIPH, 1);
channel!(TIM2_CH3, Tim2Ch3, TIM2, Tim2, TIM2_CH3_CH, TimGenCh, TIM2_PERIPH, 2);
channel!(TIM2_CH4, Tim2Ch4, TIM2, Tim2, TIM2_CH4_CH, TimGenCh, TIM2_PERIPH, 3);
channel!(TIM21_CH1, Tim21Ch1, TIM21, Tim21, TIM21_CH1_CH, TimGenCh, TIM21_PERIPH, 0);
channel!(TIM21_CH2, Tim21Ch2, TIM21, Tim21, TIM21_CH2_CH, TimGenCh, TIM21_PERIPH, 1);
channel!(TIM22_CH1, Tim22Ch1, TIM22, Tim22, TIM22_CH1_CH, TimGenCh, TIM22_PERIPH, 0);
channel!(TIM22_CH2, Tim22Ch2, TIM22, Tim22, TIM22_CH2_CH, TimGenCh, TIM22_PERIPH, 1);
