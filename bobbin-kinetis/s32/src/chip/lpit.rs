#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::lpit::*;

periph!(LpitPeriph, LPIT0, Lpit0, 0x40037000);



channel!(LPIT0_CH0, Lpit0Ch0, LPIT0, Lpit0, 0);
channel!(LPIT0_CH1, Lpit0Ch1, LPIT0, Lpit0, 1);
channel!(LPIT0_CH2, Lpit0Ch2, LPIT0, Lpit0, 2);
channel!(LPIT0_CH3, Lpit0Ch3, LPIT0, Lpit0, 3);
