#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::tim_adv::*;

periph!( TIM1, Tim1, TIM1_PERIPH, TimAdvPeriph, TIM1_OWNED, TIM1_REF_COUNT, 0x40012c00, 0x00, 0x2b);
periph!( TIM8, Tim8, TIM8_PERIPH, TimAdvPeriph, TIM8_OWNED, TIM8_REF_COUNT, 0x40013400, 0x01, 0x2c);
periph!( TIM20, Tim20, TIM20_PERIPH, TimAdvPeriph, TIM20_OWNED, TIM20_REF_COUNT, 0x40015000, 0x02, 0x2d);

channel!(TIM1_CH1, Tim1Ch1, tim1_ch1, TIM1, Tim1, TIM1_CH1_CH, TimAdvCh, TIM1_PERIPH, TIM1_CH1_OWNED, TIM1_CH1_REF_COUNT, 0);
channel!(TIM1_CH2, Tim1Ch2, tim1_ch2, TIM1, Tim1, TIM1_CH2_CH, TimAdvCh, TIM1_PERIPH, TIM1_CH2_OWNED, TIM1_CH2_REF_COUNT, 1);
channel!(TIM1_CH3, Tim1Ch3, tim1_ch3, TIM1, Tim1, TIM1_CH3_CH, TimAdvCh, TIM1_PERIPH, TIM1_CH3_OWNED, TIM1_CH3_REF_COUNT, 2);
channel!(TIM1_CH4, Tim1Ch4, tim1_ch4, TIM1, Tim1, TIM1_CH4_CH, TimAdvCh, TIM1_PERIPH, TIM1_CH4_OWNED, TIM1_CH4_REF_COUNT, 3);
channel!(TIM8_CH1, Tim8Ch1, tim8_ch1, TIM8, Tim8, TIM8_CH1_CH, TimAdvCh, TIM8_PERIPH, TIM8_CH1_OWNED, TIM8_CH1_REF_COUNT, 0);
channel!(TIM8_CH2, Tim8Ch2, tim8_ch2, TIM8, Tim8, TIM8_CH2_CH, TimAdvCh, TIM8_PERIPH, TIM8_CH2_OWNED, TIM8_CH2_REF_COUNT, 1);
channel!(TIM8_CH3, Tim8Ch3, tim8_ch3, TIM8, Tim8, TIM8_CH3_CH, TimAdvCh, TIM8_PERIPH, TIM8_CH3_OWNED, TIM8_CH3_REF_COUNT, 2);
channel!(TIM8_CH4, Tim8Ch4, tim8_ch4, TIM8, Tim8, TIM8_CH4_CH, TimAdvCh, TIM8_PERIPH, TIM8_CH4_OWNED, TIM8_CH4_REF_COUNT, 3);
channel!(TIM20_CH1, Tim20Ch1, tim20_ch1, TIM20, Tim20, TIM20_CH1_CH, TimAdvCh, TIM20_PERIPH, TIM20_CH1_OWNED, TIM20_CH1_REF_COUNT, 0);
channel!(TIM20_CH2, Tim20Ch2, tim20_ch2, TIM20, Tim20, TIM20_CH2_CH, TimAdvCh, TIM20_PERIPH, TIM20_CH2_OWNED, TIM20_CH2_REF_COUNT, 1);
channel!(TIM20_CH3, Tim20Ch3, tim20_ch3, TIM20, Tim20, TIM20_CH3_CH, TimAdvCh, TIM20_PERIPH, TIM20_CH3_OWNED, TIM20_CH3_REF_COUNT, 2);
channel!(TIM20_CH4, Tim20Ch4, tim20_ch4, TIM20, Tim20, TIM20_CH4_CH, TimAdvCh, TIM20_PERIPH, TIM20_CH4_OWNED, TIM20_CH4_REF_COUNT, 3);
