use ::core::ptr;

pub fn uid() -> [u32; 4] {
    unsafe {
        [
            ptr::read_volatile(0x0080A00C as *const u32),
            ptr::read_volatile(0x0080A040 as *const u32),
            ptr::read_volatile(0x0080A044 as *const u32),
            ptr::read_volatile(0x0080A048 as *const u32),
        ]
    }
}