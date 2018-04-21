pub fn init() {
    mcu_init();
    heap_init();
    ::clock::init();
    ::tick::init();
    ::delay::init();
    ::console::init();
    ::led::init();
    ::btn::init();   
}

pub fn mcu_init() {
}

pub fn heap_init() {
    unsafe { (::heap::Heap {}).extend(4096) }
}