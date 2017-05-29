pub use stm32_common::chip::tim_gen::*;

pub const TIM2: TimGen = TimGen(0x40000000);
pub const TIM3: TimGen = TimGen(0x40000400);
pub const TIM4: TimGen = TimGen(0x40000800);
pub const TIM15: TimGen = TimGen(0x40014000);
pub const TIM16: TimGen = TimGen(0x40014400);
pub const TIM17: TimGen = TimGen(0x40014800);

