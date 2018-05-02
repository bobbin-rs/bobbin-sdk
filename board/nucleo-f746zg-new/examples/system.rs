#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;

use board::bobbin_sys_new::ConsoleProvider;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::sys_init();
    sys.run(|sys| loop {
        sys.con().write(b"Hello, World\r\n");
        for _ in 0..5_000_000 {
            unsafe { asm!("nop")}
        }
    })
    
}
