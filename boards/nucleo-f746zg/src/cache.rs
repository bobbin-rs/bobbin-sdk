use mcu::scb::*;

pub fn init() {
    // Enable Instruction Cache
    SCB.set_iciallu(|r| r);
    unsafe { asm!("dsb") }
    unsafe { asm!("isb") }
    SCB.with_ccr(|r| r.set_ic(1));
}