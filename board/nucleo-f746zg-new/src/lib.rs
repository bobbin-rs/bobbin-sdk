#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn)]

#[cfg(target_os="none")]
#[macro_use]
pub extern crate cortex_m_rt;
pub extern crate bobbin_sys;
// pub extern crate bobbin_sys_new;
pub extern crate stm32f74x as mcu;

pub use mcu::bobbin_bits;
pub use mcu::bobbin_mcu;
pub use mcu::bobbin_hal;

#[cfg(target_os="none")]
pub use cortex_m_rt::{default_handler, exception};
pub use bobbin_sys::{system, memory, heap, tick, irq_dispatch, print, println, abort};
#[cfg(feature="logger")]
pub use bobbin_sys::logger;

#[cfg(target_os="none")]
mod lang_items;

pub mod prelude;
pub mod startup;
pub mod clock;
// pub mod tick;
pub mod console;
pub mod led;
pub mod btn;
// pub mod sys_new;

pub use startup::init;

use system::{SystemProvider};

// pub type System = system::System<Mcu, Clk>;

pub type Mcu = mcu::Stm32f74x;
pub type Clk = clock::SystemClock;
pub type Heap = heap::Heap;
pub type Tick = tick::Tick;

#[cfg(feature="logger")]
pub type Logger = logger::Logger;
pub type Dispatcher = irq_dispatch::IrqDispatcher<Mcu>;

#[cfg(target_os="none")]
default_handler!(handle_exception);

fn handle_exception() {
    use prelude::GetActiveIrq;
    let exc = Mcu::get_active_irq();
    if exc > 16 && Dispatcher::dispatch(exc.wrapping_sub(16)) {
        return
    } else {
        ::bobbin_sys::console::write(b"Unhandled Exception: 0x");
        ::bobbin_sys::console::write_u8_hex(exc);
        ::bobbin_sys::console::write(b"\r\n");
        unsafe { asm!("bkpt") };
        loop {}
    }
}

pub struct NucleoF746zg {}

pub type Board = NucleoF746zg;

impl SystemProvider for Board {
    type Mcu = mcu::Stm32f74x;
    type Clk = clock::SystemClock;

    fn init() -> Self {
        Self {}
    }

    fn init_mcu() -> Self::Mcu {
        use mcu::scb::*;
        // Enable Instruction Cache
        SCB.set_iciallu(|r| r);
        unsafe { asm!("dsb") }
        unsafe { asm!("isb") }
        SCB.with_ccr(|r| r.set_ic(1));
        Self::Mcu::default()
    }

    fn init_clk() -> Self::Clk {
        clock::init();
        Self::Clk::default()
    }

    fn init_heap() -> Heap {
        unsafe { Heap::extend(4096) }
        Heap::take()
    }

    fn init_tick(clk: &Self::Clk) -> Tick {
        use mcu::systick::SYSTICK;
        use mcu::ext::systick::SystickHz;

        let ms_hz = (clk.systick_hz() / 1000).as_u32() - 1;    
        let st = SYSTICK;
        st.set_reload_value(ms_hz);
        st.set_current_value(ms_hz);
        st.set_enabled(true);
        st.set_tick_interrupt(true);           

        Tick::take()
    }

}
exception!(SYS_TICK, Tick::incr_ticks);

