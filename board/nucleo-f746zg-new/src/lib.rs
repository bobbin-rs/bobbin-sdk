#![no_std]
#![feature(asm, use_extern_macros)]

extern crate panic_abort;
extern crate cortex_m_rt;
pub extern crate stm32f74x as mcu;

pub use mcu::bobbin_bits;
pub use mcu::bobbin_mcu;
pub use mcu::bobbin_hal;
pub use mcu::bobbin_sys;

pub mod prelude;
pub mod clock;

pub mod led;
pub mod btn;
pub mod sys;

pub use bobbin_sys::{print, println, abort};
pub use sys::init;

pub type Mcu = mcu::Stm32f74x;
pub type Board = NucleoF746zg;

pub struct NucleoF746zg {}

cortex_m_rt::default_handler!(handle_exception);

fn handle_exception() {
    use prelude::GetActiveIrq;
    let exc = Mcu::get_active_irq();
    if exc > 16 && sys::Dispatcher::dispatch(exc.wrapping_sub(16)) {
        return
    } else {
        ::bobbin_sys::console::write(b"Unhandled Exception: 0x");
        ::bobbin_sys::console::write_u8_hex(exc);
        ::bobbin_sys::console::write(b"\r\n");
        unsafe { asm!("bkpt") };
        loop {}
    }
}

cortex_m_rt::exception!(SYS_TICK, bobbin_sys::tick::Tick::tick);