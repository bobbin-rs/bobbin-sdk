#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::lpit::*;

periph!( LPIT0, Lpit0, LPIT0_PERIPH, LpitPeriph, 0x40037000, 0x0e);

channel!(LPIT0_CH0, Lpit0Ch0, LPIT0, Lpit0, LPIT0_CH0_CH, LpitCh, LPIT0_PERIPH, 0);
channel!(LPIT0_CH1, Lpit0Ch1, LPIT0, Lpit0, LPIT0_CH1_CH, LpitCh, LPIT0_PERIPH, 1);
channel!(LPIT0_CH2, Lpit0Ch2, LPIT0, Lpit0, LPIT0_CH2_CH, LpitCh, LPIT0_PERIPH, 2);
channel!(LPIT0_CH3, Lpit0Ch3, LPIT0, Lpit0, LPIT0_CH3_CH, LpitCh, LPIT0_PERIPH, 3);
