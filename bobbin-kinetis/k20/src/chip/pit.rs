#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::pit::*;

periph!( PIT, Pit, _PIT, PitPeriph, 0x40037000);



