#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::pit::*;

periph!( PIT, Pit, _PIT, PitPeriph, 0x40037000);



channel!(PIT_CH0, PitCh0, PIT, Pit, _PIT_CH0, PitCh, _PIT, 0);
channel!(PIT_CH1, PitCh1, PIT, Pit, _PIT_CH1, PitCh, _PIT, 1);
channel!(PIT_CH2, PitCh2, PIT, Pit, _PIT_CH2, PitCh, _PIT, 2);
channel!(PIT_CH3, PitCh3, PIT, Pit, _PIT_CH3, PitCh, _PIT, 3);

