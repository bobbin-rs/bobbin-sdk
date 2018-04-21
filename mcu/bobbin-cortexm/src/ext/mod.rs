pub mod nvic;
pub mod systick;

#[inline]
pub fn get_active_irq() -> u8 {
    ::scb::SCB.icsr().vectactive().value()
}

#[inline]
pub fn sleep() {
    unsafe { asm!("
        cpsid i
        wfi
        cpsie i
    ")}
}
