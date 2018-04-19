use {Board, System, Mcu, Clk, Heap};
use bobbin_sys::irq_dispatch::IrqDispatcher;

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

    IrqDispatcher::set_enable_disable(&::mcu::nvic::NVIC);

    Board {
        system: System::take(mcu, clk),
    }
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