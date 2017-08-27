#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::pit::*;

periph!(_PIT, PitPeriph, PIT, Pit, 0x40037000);



channel!(PIT_CH0, PitCh0, PIT, Pit, 0);
channel!(PIT_CH1, PitCh1, PIT, Pit, 1);
channel!(PIT_CH2, PitCh2, PIT, Pit, 2);
channel!(PIT_CH3, PitCh3, PIT, Pit, 3);
