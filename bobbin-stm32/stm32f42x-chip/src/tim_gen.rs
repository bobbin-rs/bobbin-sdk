pub use stm32_common::chip::tim_gen::*;

pub const TIM2: TimGen = TimGen(0x40000000);
pub const TIM3: TimGen = TimGen(0x40000400);
pub const TIM4: TimGen = TimGen(0x40000800);
pub const TIM5: TimGen = TimGen(0x40000c00);
pub const TIM9: TimGen = TimGen(0x40014000);
pub const TIM10: TimGen = TimGen(0x40014400);
pub const TIM11: TimGen = TimGen(0x40014800);
pub const TIM12: TimGen = TimGen(0x40001800);
pub const TIM13: TimGen = TimGen(0x40001c00);
pub const TIM14: TimGen = TimGen(0x40002000);

