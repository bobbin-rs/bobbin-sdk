use core::ptr;

pub const WDOG_STCTRLH: *mut u16 = 0x4005_2000 as *mut u16;
pub const WDOG_UNLOCK: *mut u16 = 0x4005_200e as *mut u16;

pub fn init() {
    // Disable Watchdog
    unsafe {
        // Unlock Watchdog
        ptr::write_volatile(WDOG_UNLOCK, 0xc520);
        ptr::write_volatile(WDOG_UNLOCK, 0xd928);
        // Disable Watchdog
        ptr::write_volatile(WDOG_STCTRLH, 0x00d2);
    }

}