pub use stm32_common::chip::tim_gen::*;

pub const TIM2: Tim2 = Periph(0x40000000, Tim2Id {});
pub const TIM3: Tim3 = Periph(0x40000400, Tim3Id {});
pub const TIM4: Tim4 = Periph(0x40000800, Tim4Id {});
pub const TIM5: Tim5 = Periph(0x40000c00, Tim5Id {});
pub const TIM9: Tim9 = Periph(0x40014000, Tim9Id {});
pub const TIM10: Tim10 = Periph(0x40014400, Tim10Id {});
pub const TIM11: Tim11 = Periph(0x40014800, Tim11Id {});
pub const TIM12: Tim12 = Periph(0x40001800, Tim12Id {});
pub const TIM13: Tim13 = Periph(0x40001c00, Tim13Id {});
pub const TIM14: Tim14 = Periph(0x40002000, Tim14Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim2Id {}
pub type Tim2 = Periph<Tim2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim3Id {}
pub type Tim3 = Periph<Tim3Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim4Id {}
pub type Tim4 = Periph<Tim4Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim5Id {}
pub type Tim5 = Periph<Tim5Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim9Id {}
pub type Tim9 = Periph<Tim9Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim10Id {}
pub type Tim10 = Periph<Tim10Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim11Id {}
pub type Tim11 = Periph<Tim11Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim12Id {}
pub type Tim12 = Periph<Tim12Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim13Id {}
pub type Tim13 = Periph<Tim13Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tim14Id {}
pub type Tim14 = Periph<Tim14Id>;












