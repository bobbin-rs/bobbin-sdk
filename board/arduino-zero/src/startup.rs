use {Board, System, Mcu, Clk, Heap};
use bobbin_hal::flash::*;
use mcu::nvmctrl::{NvmctrlPeriph, NVMCTRL};

pub fn init() -> Board {
    mcu_init();
    heap_init();
    
    ::clock::init();
    ::tick::init();
    
    ::console::init();
    ::led::init();
    ::btn::init();

    #[cfg(feature="logger")]
    ::Logger::init();             

    let mcu = Mcu {};
    let clk = Clk::default();

    Board {
        system: System::take(mcu, clk),
    }
}

pub fn mcu_init() {
}

pub fn heap_init() {
    unsafe { Heap::extend(4096) }
}

impl GetFlash for Board {
    type Output = ::mcu::nvmctrl::NvmctrlPeriph;
    fn flash(&self) -> &NvmctrlPeriph {
        &NVMCTRL
    }
}