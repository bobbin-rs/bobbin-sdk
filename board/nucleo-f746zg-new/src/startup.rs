use {Board, Heap};
use bobbin_sys::system::System;
use bobbin_hal::flash::*;
use mcu::flash::{FlashPeriph, FLASH};

pub fn init() -> System<Board> {
    // mcu_init();
    // heap_init();
    
    // ::clock::init();
    // ::tick::init();
    
    ::console::init();
    ::led::init();
    ::btn::init();

    #[cfg(feature="logger")]
    ::Logger::init();             

    // let brd = Board {};
    // let mcu = Mcu {};
    // let clk = Clk::default();
    System::take()
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
    unsafe { Heap::extend(4096) }
}

impl GetFlash for Board {
    type Output = ::mcu::flash::FlashPeriph;
    fn flash(&self) -> &FlashPeriph {
        &FLASH
    }
}