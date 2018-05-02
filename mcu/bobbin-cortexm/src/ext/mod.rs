pub mod nvic;
pub mod systick;

use scb::SCB;

#[inline]
pub fn get_active_irq() -> u8 {
    SCB.icsr().vectactive().value()
}

pub fn enable_instruction_cache() {
    // Enable Instruction Cache
    SCB.set_iciallu(|r| r);
    unsafe { asm!("dsb") }
    unsafe { asm!("isb") }
    SCB.with_ccr(|r| r.set_ic(1));    
}

#[inline]
pub fn sleep() {
    #[cfg(target_os="none")]
    unsafe { asm!("
        cpsid i
        wfi
        cpsie i
    ")}
}
