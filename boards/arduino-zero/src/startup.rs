use {Board, System, Mcu, Clk, Heap};

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