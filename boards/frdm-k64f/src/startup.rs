use core::ptr;

pub const WDOG_STCTRLH: *mut u16 = 0x4005_2000 as *mut u16;
pub const WDOG_UNLOCK: *mut u16 = 0x4005_200e as *mut u16;

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
    // Disable Watchdog
    unsafe {
        // Unlock Watchdog
        ptr::write_volatile(WDOG_UNLOCK, 0xc520);
        ptr::write_volatile(WDOG_UNLOCK, 0xd928);
        // Disable Watchdog
        ptr::write_volatile(WDOG_STCTRLH, 0x00d2);
    }
}

pub fn heap_init() {
    unsafe { (::heap::Heap {}).extend(4096) }
}