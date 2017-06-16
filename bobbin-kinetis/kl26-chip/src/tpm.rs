pub use kinetis_common::chip::tpm::*;

pub const TPM0: Tpm0 = Periph(0x40038000, Tpm0Id {});
pub const TPM1: Tpm1 = Periph(0x40039000, Tpm1Id {});
pub const TPM2: Tpm2 = Periph(0x4003a000, Tpm2Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tpm0Id {}
pub type Tpm0 = Periph<Tpm0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tpm1Id {}
pub type Tpm1 = Periph<Tpm1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tpm2Id {}
pub type Tpm2 = Periph<Tpm2Id>;

impl super::sig::Signal<super::sig::Tpm0Ch0> for Tpm0Ch0 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch0> for Tpm0Ch0 {}
impl super::sig::Signal<super::sig::Tpm0Ch1> for Tpm0Ch1 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch1> for Tpm0Ch1 {}
impl super::sig::Signal<super::sig::Tpm0Ch2> for Tpm0Ch2 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch2> for Tpm0Ch2 {}
impl super::sig::Signal<super::sig::Tpm0Ch3> for Tpm0Ch3 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch3> for Tpm0Ch3 {}
impl super::sig::Signal<super::sig::Tpm0Ch4> for Tpm0Ch4 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch4> for Tpm0Ch4 {}
impl super::sig::Signal<super::sig::Tpm0Ch5> for Tpm0Ch5 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch5> for Tpm0Ch5 {}

impl super::sig::Signal<super::sig::Tpm1Ch0> for Tpm1Ch0 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch0> for Tpm1Ch0 {}
impl super::sig::Signal<super::sig::Tpm1Ch1> for Tpm1Ch1 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch1> for Tpm1Ch1 {}
impl super::sig::Signal<super::sig::Tpm1Ch2> for Tpm1Ch2 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch2> for Tpm1Ch2 {}
impl super::sig::Signal<super::sig::Tpm1Ch3> for Tpm1Ch3 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch3> for Tpm1Ch3 {}
impl super::sig::Signal<super::sig::Tpm1Ch4> for Tpm1Ch4 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch4> for Tpm1Ch4 {}
impl super::sig::Signal<super::sig::Tpm1Ch5> for Tpm1Ch5 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch5> for Tpm1Ch5 {}

impl super::sig::Signal<super::sig::Tpm2Ch0> for Tpm2Ch0 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch0> for Tpm2Ch0 {}
impl super::sig::Signal<super::sig::Tpm2Ch1> for Tpm2Ch1 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch1> for Tpm2Ch1 {}
impl super::sig::Signal<super::sig::Tpm2Ch2> for Tpm2Ch2 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch2> for Tpm2Ch2 {}
impl super::sig::Signal<super::sig::Tpm2Ch3> for Tpm2Ch3 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch3> for Tpm2Ch3 {}
impl super::sig::Signal<super::sig::Tpm2Ch4> for Tpm2Ch4 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch4> for Tpm2Ch4 {}
impl super::sig::Signal<super::sig::Tpm2Ch5> for Tpm2Ch5 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch5> for Tpm2Ch5 {}


pub const TPM0_CH0: Channel<Tpm0Ch0Id, Tpm0Id> = Channel { periph: TPM0, index: 0, id: Tpm0Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm0Ch0Id {}
pub type Tpm0Ch0 = Channel<Tpm0Ch0Id, Tpm0Id>;

pub const TPM0_CH1: Channel<Tpm0Ch1Id, Tpm0Id> = Channel { periph: TPM0, index: 1, id: Tpm0Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm0Ch1Id {}
pub type Tpm0Ch1 = Channel<Tpm0Ch1Id, Tpm0Id>;

pub const TPM0_CH2: Channel<Tpm0Ch2Id, Tpm0Id> = Channel { periph: TPM0, index: 2, id: Tpm0Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm0Ch2Id {}
pub type Tpm0Ch2 = Channel<Tpm0Ch2Id, Tpm0Id>;

pub const TPM0_CH3: Channel<Tpm0Ch3Id, Tpm0Id> = Channel { periph: TPM0, index: 3, id: Tpm0Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm0Ch3Id {}
pub type Tpm0Ch3 = Channel<Tpm0Ch3Id, Tpm0Id>;

pub const TPM0_CH4: Channel<Tpm0Ch4Id, Tpm0Id> = Channel { periph: TPM0, index: 4, id: Tpm0Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm0Ch4Id {}
pub type Tpm0Ch4 = Channel<Tpm0Ch4Id, Tpm0Id>;

pub const TPM0_CH5: Channel<Tpm0Ch5Id, Tpm0Id> = Channel { periph: TPM0, index: 5, id: Tpm0Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm0Ch5Id {}
pub type Tpm0Ch5 = Channel<Tpm0Ch5Id, Tpm0Id>;

pub const TPM1_CH0: Channel<Tpm1Ch0Id, Tpm1Id> = Channel { periph: TPM1, index: 0, id: Tpm1Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm1Ch0Id {}
pub type Tpm1Ch0 = Channel<Tpm1Ch0Id, Tpm1Id>;

pub const TPM1_CH1: Channel<Tpm1Ch1Id, Tpm1Id> = Channel { periph: TPM1, index: 1, id: Tpm1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm1Ch1Id {}
pub type Tpm1Ch1 = Channel<Tpm1Ch1Id, Tpm1Id>;

pub const TPM1_CH2: Channel<Tpm1Ch2Id, Tpm1Id> = Channel { periph: TPM1, index: 2, id: Tpm1Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm1Ch2Id {}
pub type Tpm1Ch2 = Channel<Tpm1Ch2Id, Tpm1Id>;

pub const TPM1_CH3: Channel<Tpm1Ch3Id, Tpm1Id> = Channel { periph: TPM1, index: 3, id: Tpm1Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm1Ch3Id {}
pub type Tpm1Ch3 = Channel<Tpm1Ch3Id, Tpm1Id>;

pub const TPM1_CH4: Channel<Tpm1Ch4Id, Tpm1Id> = Channel { periph: TPM1, index: 4, id: Tpm1Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm1Ch4Id {}
pub type Tpm1Ch4 = Channel<Tpm1Ch4Id, Tpm1Id>;

pub const TPM1_CH5: Channel<Tpm1Ch5Id, Tpm1Id> = Channel { periph: TPM1, index: 5, id: Tpm1Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm1Ch5Id {}
pub type Tpm1Ch5 = Channel<Tpm1Ch5Id, Tpm1Id>;

pub const TPM2_CH0: Channel<Tpm2Ch0Id, Tpm2Id> = Channel { periph: TPM2, index: 0, id: Tpm2Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm2Ch0Id {}
pub type Tpm2Ch0 = Channel<Tpm2Ch0Id, Tpm2Id>;

pub const TPM2_CH1: Channel<Tpm2Ch1Id, Tpm2Id> = Channel { periph: TPM2, index: 1, id: Tpm2Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm2Ch1Id {}
pub type Tpm2Ch1 = Channel<Tpm2Ch1Id, Tpm2Id>;

pub const TPM2_CH2: Channel<Tpm2Ch2Id, Tpm2Id> = Channel { periph: TPM2, index: 2, id: Tpm2Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm2Ch2Id {}
pub type Tpm2Ch2 = Channel<Tpm2Ch2Id, Tpm2Id>;

pub const TPM2_CH3: Channel<Tpm2Ch3Id, Tpm2Id> = Channel { periph: TPM2, index: 3, id: Tpm2Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm2Ch3Id {}
pub type Tpm2Ch3 = Channel<Tpm2Ch3Id, Tpm2Id>;

pub const TPM2_CH4: Channel<Tpm2Ch4Id, Tpm2Id> = Channel { periph: TPM2, index: 4, id: Tpm2Ch4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm2Ch4Id {}
pub type Tpm2Ch4 = Channel<Tpm2Ch4Id, Tpm2Id>;

pub const TPM2_CH5: Channel<Tpm2Ch5Id, Tpm2Id> = Channel { periph: TPM2, index: 5, id: Tpm2Ch5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct Tpm2Ch5Id {}
pub type Tpm2Ch5 = Channel<Tpm2Ch5Id, Tpm2Id>;

