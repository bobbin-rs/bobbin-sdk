pub fn init() {
    mcu_init();
    heap_init();
    ::clock::init();
    ::tick::init();
    ::delay::init();
    ::console::init();
    ::led::init();
    ::btn::init();

    #[cfg(feature="logger")]
    ::Logger::init();             
}

pub fn mcu_init() {
    use mcu::scb::*;
    // Enable Instruction Cache
    SCB.set_iciallu(|r| r);
    unsafe { asm!("dsb") }
    unsafe { asm!("isb") }
    SCB.with_ccr(|r| r.set_ic(1));
}

pub fn heap_init() {
    unsafe { (::heap::Heap {}).extend(4096) }
}