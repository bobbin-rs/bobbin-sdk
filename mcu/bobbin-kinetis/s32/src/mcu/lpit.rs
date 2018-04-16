#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::lpit::*;

periph!( LPIT0, Lpit0, LPIT0_PERIPH, LpitPeriph, LPIT0_OWNED, LPIT0_REF_COUNT, 0x40037000, 0x00, 0x0e);

channel!(LPIT0_CH0, Lpit0Ch0, lpit0_ch0, LPIT0, Lpit0, LPIT0_CH0_CH, LpitCh, LPIT0_PERIPH, LPIT0_CH0_OWNED, LPIT0_CH0_REF_COUNT, 0);
channel!(LPIT0_CH1, Lpit0Ch1, lpit0_ch1, LPIT0, Lpit0, LPIT0_CH1_CH, LpitCh, LPIT0_PERIPH, LPIT0_CH1_OWNED, LPIT0_CH1_REF_COUNT, 1);
channel!(LPIT0_CH2, Lpit0Ch2, lpit0_ch2, LPIT0, Lpit0, LPIT0_CH2_CH, LpitCh, LPIT0_PERIPH, LPIT0_CH2_OWNED, LPIT0_CH2_REF_COUNT, 2);
channel!(LPIT0_CH3, Lpit0Ch3, lpit0_ch3, LPIT0, Lpit0, LPIT0_CH3_CH, LpitCh, LPIT0_PERIPH, LPIT0_CH3_OWNED, LPIT0_CH3_REF_COUNT, 3);
