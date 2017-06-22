pub use stm32_common::chip::rng::*;

pub const RNG: Rng = Periph(0x40025000, RngId {});

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RngId {}
pub type Rng = Periph<RngId>;



