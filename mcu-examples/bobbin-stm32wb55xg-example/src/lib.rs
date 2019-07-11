#![no_std]

use core::panic::PanicInfo;
use cortex_m::interrupt;
pub use stm32wb55xx as mcu;
pub use stm32wb55xx::bobbin_sys::{print, println};

use mcu::ext::clock;

pub fn init() {
    clock::clock_init::init_pll();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();
    println!("{}", info);
    loop {}
}
