#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::pit::*;

periph!( PIT, Pit, PIT_PERIPH, PitPeriph, PIT_OWNED, PIT_REF_COUNT, 0x40037000, 0x00, 0x0a);

channel!(PIT_CH0, PitCh0, pit_ch0, PIT, Pit, PIT_CH0_CH, PitCh, PIT_PERIPH, PIT_CH0_OWNED, PIT_CH0_REF_COUNT, 0);
channel!(PIT_CH1, PitCh1, pit_ch1, PIT, Pit, PIT_CH1_CH, PitCh, PIT_PERIPH, PIT_CH1_OWNED, PIT_CH1_REF_COUNT, 1);
channel!(PIT_CH2, PitCh2, pit_ch2, PIT, Pit, PIT_CH2_CH, PitCh, PIT_PERIPH, PIT_CH2_OWNED, PIT_CH2_REF_COUNT, 2);
channel!(PIT_CH3, PitCh3, pit_ch3, PIT, Pit, PIT_CH3_CH, PitCh, PIT_PERIPH, PIT_CH3_OWNED, PIT_CH3_REF_COUNT, 3);
