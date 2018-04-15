use bobbin_sys::memory::Memory;
use bobbin_sys::heap::Heap;
use bobbin_sys::console::{Console, console_ref};

#[cfg(feature="logger")]
use bobbin_sys::logger::Logger;

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
pub struct System<MCU, CLK> 
where
    MCU: Default,
    CLK: Default,
{
    mcu: MCU,
    memory: Memory,
    heap: Heap,
    clock: CLK,
    #[cfg(feature="logger")]
    logger: Logger,
    dispatcher: Dispatcher<ExcHandlers8>,
}

impl<MCU, CLK> System<MCU, CLK> 
where
    MCU: Default,
    CLK: Default,
{

    pub fn init<F: FnOnce()>(f: F) -> Self {
        Self::disable_interrupts();
        Self::lock();

        f();

        System {
            mcu: MCU::default(),
            memory: Memory {},
            heap: Heap {},
            clock: CLK::default(),
            #[cfg(feature="logger")]
            logger: Logger::default(),
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

    pub fn mcu(&self) -> &MCU {
        &self.mcu
    }

    pub fn mcu_mut(&mut self) -> &mut MCU {
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

    pub fn clock(&self) -> &CLK {
        &self.clock
    }

    pub fn console(&self) -> Option<&Console> {
        console_ref()
    }

    #[cfg(feature="logger")]
    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn dispatcher(&self) -> &Dispatcher<ExcHandlers8> {
        &self.dispatcher
    }

    pub fn dispatcher_mut(&mut self) -> &mut Dispatcher<ExcHandlers8> {
        &mut self.dispatcher
    }

    pub fn run<T, F: FnOnce(&Self) -> T>(&mut self, f: F) -> T {
        Self::enable_interrupts();
        let ret = f(self);
        Self::disable_interrupts();
        ret
    }
}

impl<MCU, CLK> Drop for System<MCU, CLK> 
where
    MCU: Default,
    CLK: Default,
{
    fn drop(&mut self) {
        Self::unlock();
        Self::enable_interrupts()
    }
}
