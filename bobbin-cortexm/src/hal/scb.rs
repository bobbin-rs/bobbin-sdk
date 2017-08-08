pub use chip::scb::*;

// VTOR

pub fn tbloff() -> u32 {
    SCB.vtor().tbloff().into()
}

pub fn set_tbloff(value: u32) {
    SCB.set_vtor(|r| r.set_tbloff(value));
}
