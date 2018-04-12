use ::common::memory::Memory;
use ::common::heap::Heap;
use ::mcu::dispatch::{Dispatcher, ExcHandlers8};

use core::cell::UnsafeCell;

static mut SYSTEM_DATA: UnsafeCell<SystemData> = UnsafeCell::new(SystemData::new(false));

struct SystemData {
    locked: bool,
}

impl SystemData {
    const fn new(locked: bool) -> Self {
        Self { locked: locked }
    }
}

pub struct Config {
}

impl Default for Config {
    fn default() -> Config {
        Config {}
    }
}

#[must_use]
pub struct System {
    memory: Memory,
    heap: Heap,
    dispatcher: Dispatcher<ExcHandlers8>,
}

impl System {
    pub fn init() -> Self {
        Self::init_with_config(Config::default())
    }

    pub fn init_with_config(_cfg: Config) -> Self {
        Self::disable_interrupts();
        Self::lock();

        ::cache::init();
        ::clock::init();
        ::console::init();
        ::led::init();
        ::btn::init();
        ::delay::init();        
        
        System {
            memory: Memory {},
            heap: Heap {},
            dispatcher: unsafe { Dispatcher::new() },
        }
    }

    fn data() -> &'static mut SystemData {
        unsafe { &mut *SYSTEM_DATA.get() }
    }

    #[inline]
    fn enable_interrupts() {
        unsafe { asm!("cpsie i") }
    }

    #[inline]
    fn disable_interrupts() {
        unsafe { asm!("cpsid i") }
    }

    fn locked() -> bool {
        Self::data().locked    
    }

    fn lock() {
        while Self::locked() {}
        Self::data().locked = true;
    }

    fn unlock() {
        Self::data().locked = false;
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn heap(&self) -> &Heap {
        &self.heap
    }

    pub fn heap_mut(&mut self) -> &mut Heap {
        &mut self.heap
    }

    pub fn dispatcher(&self) -> &Dispatcher<ExcHandlers8> {
        &self.dispatcher
    }

    pub fn dispatcher_mut(&mut self) -> &mut Dispatcher<ExcHandlers8> {
        &mut self.dispatcher
    }

    pub fn run<T, F: FnOnce(&mut Self) -> T>(&mut self, f: F) -> T {
        Self::enable_interrupts();
        let ret = f(self);
        Self::disable_interrupts();
        ret
    }
}

impl Drop for System {
    fn drop(&mut self) {
        Self::unlock();
        Self::enable_interrupts()
    }
}