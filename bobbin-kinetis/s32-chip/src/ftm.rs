pub use kinetis_common::chip::ftm::*;

pub const FTM0: Ftm0 = Periph(0x40038000, Ftm0Id {});
pub const FTM1: Ftm1 = Periph(0x40039000, Ftm1Id {});
pub const FTM2: Ftm2 = Periph(0x4003a000, Ftm2Id {});
pub const FTM3: Ftm3 = Periph(0x40026000, Ftm3Id {});
pub const FTM4: Ftm4 = Periph(0x4006e000, Ftm4Id {});
pub const FTM5: Ftm5 = Periph(0x4006f000, Ftm5Id {});
pub const FTM6: Ftm6 = Periph(0x40070000, Ftm6Id {});
pub const FTM7: Ftm7 = Periph(0x40071000, Ftm7Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ftm0Id {}
pub type Ftm0 = Periph<Ftm0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ftm1Id {}
pub type Ftm1 = Periph<Ftm1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ftm2Id {}
pub type Ftm2 = Periph<Ftm2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ftm3Id {}
pub type Ftm3 = Periph<Ftm3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ftm4Id {}
pub type Ftm4 = Periph<Ftm4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ftm5Id {}
pub type Ftm5 = Periph<Ftm5Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ftm6Id {}
pub type Ftm6 = Periph<Ftm6Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ftm7Id {}
pub type Ftm7 = Periph<Ftm7Id>;










