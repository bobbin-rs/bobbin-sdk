pub use stm32_common::chip::exti::*;

pub const EXTI: Exti = Periph(0x40010400, ExtiId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ExtiId {}
pub type Exti = Periph<ExtiId>;



pub const EXTI_LINE0: Channel<ExtiLine0Id, ExtiId> = Channel { periph: EXTI, index: 0, id: ExtiLine0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine0Id {}
pub type ExtiLine0 = Channel<ExtiLine0Id, ExtiId>;

pub const EXTI_LINE1: Channel<ExtiLine1Id, ExtiId> = Channel { periph: EXTI, index: 1, id: ExtiLine1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine1Id {}
pub type ExtiLine1 = Channel<ExtiLine1Id, ExtiId>;

pub const EXTI_LINE2: Channel<ExtiLine2Id, ExtiId> = Channel { periph: EXTI, index: 2, id: ExtiLine2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine2Id {}
pub type ExtiLine2 = Channel<ExtiLine2Id, ExtiId>;

pub const EXTI_LINE3: Channel<ExtiLine3Id, ExtiId> = Channel { periph: EXTI, index: 3, id: ExtiLine3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine3Id {}
pub type ExtiLine3 = Channel<ExtiLine3Id, ExtiId>;

pub const EXTI_LINE4: Channel<ExtiLine4Id, ExtiId> = Channel { periph: EXTI, index: 4, id: ExtiLine4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine4Id {}
pub type ExtiLine4 = Channel<ExtiLine4Id, ExtiId>;

pub const EXTI_LINE5: Channel<ExtiLine5Id, ExtiId> = Channel { periph: EXTI, index: 5, id: ExtiLine5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine5Id {}
pub type ExtiLine5 = Channel<ExtiLine5Id, ExtiId>;

pub const EXTI_LINE6: Channel<ExtiLine6Id, ExtiId> = Channel { periph: EXTI, index: 6, id: ExtiLine6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine6Id {}
pub type ExtiLine6 = Channel<ExtiLine6Id, ExtiId>;

pub const EXTI_LINE7: Channel<ExtiLine7Id, ExtiId> = Channel { periph: EXTI, index: 7, id: ExtiLine7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine7Id {}
pub type ExtiLine7 = Channel<ExtiLine7Id, ExtiId>;

pub const EXTI_LINE8: Channel<ExtiLine8Id, ExtiId> = Channel { periph: EXTI, index: 8, id: ExtiLine8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine8Id {}
pub type ExtiLine8 = Channel<ExtiLine8Id, ExtiId>;

pub const EXTI_LINE9: Channel<ExtiLine9Id, ExtiId> = Channel { periph: EXTI, index: 9, id: ExtiLine9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine9Id {}
pub type ExtiLine9 = Channel<ExtiLine9Id, ExtiId>;

pub const EXTI_LINE10: Channel<ExtiLine10Id, ExtiId> = Channel { periph: EXTI, index: 10, id: ExtiLine10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine10Id {}
pub type ExtiLine10 = Channel<ExtiLine10Id, ExtiId>;

pub const EXTI_LINE11: Channel<ExtiLine11Id, ExtiId> = Channel { periph: EXTI, index: 11, id: ExtiLine11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine11Id {}
pub type ExtiLine11 = Channel<ExtiLine11Id, ExtiId>;

pub const EXTI_LINE12: Channel<ExtiLine12Id, ExtiId> = Channel { periph: EXTI, index: 12, id: ExtiLine12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine12Id {}
pub type ExtiLine12 = Channel<ExtiLine12Id, ExtiId>;

pub const EXTI_LINE13: Channel<ExtiLine13Id, ExtiId> = Channel { periph: EXTI, index: 13, id: ExtiLine13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine13Id {}
pub type ExtiLine13 = Channel<ExtiLine13Id, ExtiId>;

pub const EXTI_LINE14: Channel<ExtiLine14Id, ExtiId> = Channel { periph: EXTI, index: 14, id: ExtiLine14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine14Id {}
pub type ExtiLine14 = Channel<ExtiLine14Id, ExtiId>;

pub const EXTI_LINE15: Channel<ExtiLine15Id, ExtiId> = Channel { periph: EXTI, index: 15, id: ExtiLine15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine15Id {}
pub type ExtiLine15 = Channel<ExtiLine15Id, ExtiId>;

pub const EXTI_LINE16: Channel<ExtiLine16Id, ExtiId> = Channel { periph: EXTI, index: 16, id: ExtiLine16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine16Id {}
pub type ExtiLine16 = Channel<ExtiLine16Id, ExtiId>;

pub const EXTI_LINE17: Channel<ExtiLine17Id, ExtiId> = Channel { periph: EXTI, index: 17, id: ExtiLine17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine17Id {}
pub type ExtiLine17 = Channel<ExtiLine17Id, ExtiId>;

pub const EXTI_LINE18: Channel<ExtiLine18Id, ExtiId> = Channel { periph: EXTI, index: 18, id: ExtiLine18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine18Id {}
pub type ExtiLine18 = Channel<ExtiLine18Id, ExtiId>;

pub const EXTI_LINE19: Channel<ExtiLine19Id, ExtiId> = Channel { periph: EXTI, index: 19, id: ExtiLine19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine19Id {}
pub type ExtiLine19 = Channel<ExtiLine19Id, ExtiId>;

pub const EXTI_LINE20: Channel<ExtiLine20Id, ExtiId> = Channel { periph: EXTI, index: 20, id: ExtiLine20Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine20Id {}
pub type ExtiLine20 = Channel<ExtiLine20Id, ExtiId>;

pub const EXTI_LINE21: Channel<ExtiLine21Id, ExtiId> = Channel { periph: EXTI, index: 21, id: ExtiLine21Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine21Id {}
pub type ExtiLine21 = Channel<ExtiLine21Id, ExtiId>;

pub const EXTI_LINE22: Channel<ExtiLine22Id, ExtiId> = Channel { periph: EXTI, index: 22, id: ExtiLine22Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine22Id {}
pub type ExtiLine22 = Channel<ExtiLine22Id, ExtiId>;

pub const EXTI_LINE23: Channel<ExtiLine23Id, ExtiId> = Channel { periph: EXTI, index: 23, id: ExtiLine23Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine23Id {}
pub type ExtiLine23 = Channel<ExtiLine23Id, ExtiId>;

pub const EXTI_LINE24: Channel<ExtiLine24Id, ExtiId> = Channel { periph: EXTI, index: 24, id: ExtiLine24Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine24Id {}
pub type ExtiLine24 = Channel<ExtiLine24Id, ExtiId>;

pub const EXTI_LINE25: Channel<ExtiLine25Id, ExtiId> = Channel { periph: EXTI, index: 25, id: ExtiLine25Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine25Id {}
pub type ExtiLine25 = Channel<ExtiLine25Id, ExtiId>;

pub const EXTI_LINE26: Channel<ExtiLine26Id, ExtiId> = Channel { periph: EXTI, index: 26, id: ExtiLine26Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine26Id {}
pub type ExtiLine26 = Channel<ExtiLine26Id, ExtiId>;

pub const EXTI_LINE28: Channel<ExtiLine28Id, ExtiId> = Channel { periph: EXTI, index: 28, id: ExtiLine28Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine28Id {}
pub type ExtiLine28 = Channel<ExtiLine28Id, ExtiId>;

pub const EXTI_LINE29: Channel<ExtiLine29Id, ExtiId> = Channel { periph: EXTI, index: 29, id: ExtiLine29Id {} }; 
#[derive(Clone, Copy, PartialEq)]
pub struct ExtiLine29Id {}
pub type ExtiLine29 = Channel<ExtiLine29Id, ExtiId>;

