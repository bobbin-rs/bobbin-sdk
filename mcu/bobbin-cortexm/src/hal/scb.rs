//! Extends the chip::scb module.

pub use chip::scb::*;

// VTOR

/// Returns bits [29:7] of the offset of the vector table base.
pub fn tbloff() -> u32 {
    SCB.vtor().tbloff().into()
}

/// Sets bits [29:7] of the offset of the vector table base.
pub fn set_tbloff(value: u32) {
    SCB.set_vtor(|r| r.set_tbloff(value));
}
