use memory::Memory;
use heap::Heap;
use console::{Console, console_ref};

#[cfg(feature="logger")]
use logger::Logger;

use core::cell::UnsafeCell;

static mut SYSTEM_DATA: UnsafeCell<SystemData> = UnsafeCell::new(SystemData { locked: false });

struct SystemData {
    locked: bool,
}

pub struct Config {
}

impl Default for Config {
    fn default() -> Config {
        Config {}
    }
}

#[must_use]
pub struct System<MCU, CLK, DIS> 
where
    MCU: Default,
    CLK: Default,
    DIS: Default,
{
    mcu: MCU,
    memory: Memory,
    heap: Heap,
    clock: CLK,
    #[cfg(feature="logger")]
    logger: Logger,
    dispatcher: DIS,
}

impl<MCU, CLK, DIS> System<MCU, CLK, DIS> 
where
    MCU: Default,
    CLK: Default,
    DIS: Default,
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
            dispatcher: DIS::default(),
        }
    }

    fn data() -> &'static mut SystemData {
        unsafe { &mut *SYSTEM_DATA.get() }
    }

    #[inline]
    fn enable_interrupts() {
        // unsafe { asm!("cpsie i") }
    }

    #[inline]
    fn disable_interrupts() {
        // unsafe { asm!("cpsid i") }
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

    pub fn dispatcher(&self) -> &DIS {
        &self.dispatcher
    }

    pub fn dispatcher_mut(&mut self) -> &mut DIS {
        &mut self.dispatcher
    }

    pub fn run<T, F: FnOnce(&Self) -> T>(&mut self, f: F) -> T {
        Self::enable_interrupts();
        let ret = f(self);
        Self::disable_interrupts();
        ret
    }
}

impl<MCU, CLK, DIS> Drop for System<MCU, CLK, DIS> 
where
    MCU: Default,
    CLK: Default,
    DIS: Default,
{
    fn drop(&mut self) {
        Self::unlock();
        Self::enable_interrupts()
    }
}
