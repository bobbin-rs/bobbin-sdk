use ::common::memory::Memory;
use ::common::heap::Heap;
use ::common::console::{Console, with_console};

use ::mcu::Stm32f74x as Mcu;
use ::mcu::dispatch::{Dispatcher, ExcHandlers8};

use clock::SystemClocks;

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
    mcu: Mcu,
    memory: Memory,
    heap: Heap,
    clocks: SystemClocks,
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
            mcu: Mcu {},
            memory: Memory {},
            heap: Heap {},
            clocks: SystemClocks::default(),
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

    pub fn mcu(&self) -> &Mcu {
        &self.mcu
    }

    pub fn mcu_mut(&mut self) -> &mut Mcu {
        &mut self.mcu
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

    pub fn clocks(&self) -> &SystemClocks {
        &self.clocks
    }

    pub fn dispatcher(&self) -> &Dispatcher<ExcHandlers8> {
        &self.dispatcher
    }

    pub fn dispatcher_mut(&mut self) -> &mut Dispatcher<ExcHandlers8> {
        &mut self.dispatcher
    }

    pub fn with_console<F: FnOnce(&mut Console)>(&self, f: F) {
        with_console(f)
    }

    pub fn run<T, F: FnOnce(&Self) -> T>(&mut self, f: F) -> T {
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