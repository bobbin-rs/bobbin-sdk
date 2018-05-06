//! A global singleton for accessing system services.

use core::ops::{Deref, DerefMut};

use bobbin_mcu::mcu::Mcu;

use heap::Heap;
use tick::Tick;
use pend::Pend;
use irq_dispatch::IrqDispatcher;
use console::Console;

struct SystemToken;
static mut SYSTEM_TOKEN: Option<SystemToken> = Some(SystemToken);

pub trait SystemProvider {
    type Mcu: Mcu;
    type Clk;

    fn init() -> Self;
    fn init_mcu() -> Self::Mcu;
    fn init_clk() -> Self::Clk;
    fn init_heap() -> Heap;
    fn init_dispatcher() -> IrqDispatcher<Self::Mcu>;
    fn init_pend() -> Pend;
    fn init_tick(&Self::Clk) -> Tick;    
    fn init_console(&Self::Clk, &mut Heap) {}
    fn init_led(&Self::Clk, &mut Heap) {}
    fn init_btn(&Self::Clk, &mut Heap) {}
}


/// A global singleton that provides access to system services such as the MCU, Clock, Heap, Tick,
/// and the Interrupt Dispatcher.
pub struct System<S: SystemProvider> {
    provider: S,
    mcu: S::Mcu,
    clk: S::Clk,
    heap: Heap,
    tick: Tick,
    pend: Pend,
    dispatcher: IrqDispatcher<S::Mcu>,
    _private: ()
}

impl<S: SystemProvider> System<S> {
    /// Initializes and returns the global system singleton. This function will also initialize and
    /// acquire the global singletons used by System.
    pub fn take() -> Self {
        let provider = S::init();
        unsafe { asm!("cpsid i "); }
        unsafe { while let None = SYSTEM_TOKEN.take() {} }        
        let mcu = S::init_mcu();
        let clk = S::init_clk();
        let mut heap = S::init_heap();
        let dispatcher = S::init_dispatcher();
        let tick = S::init_tick(&clk);
        let pend = S::init_pend();

        S::init_console(&clk, &mut heap);
        S::init_led(&clk, &mut heap);
        S::init_btn(&clk, &mut heap);

        System {
            provider,
            mcu,
            clk,
            heap,
            tick,
            pend,
            dispatcher,
            _private: (),
        }
    }

    /// Releases the global system singleton as well as the global singletons used by System.
    pub fn release(system: Self) {
        let System { provider, mcu, clk, heap, tick, pend, dispatcher, _private } = system;
        let _ = provider;
        let _ = mcu;
        let _ = clk;
        Tick::release(tick);
        Pend::release(pend);
        Heap::release(heap);        
        IrqDispatcher::release(dispatcher);
        unsafe { SYSTEM_TOKEN = Some(SystemToken) }
    }

    /// Returns a shared reference to the global MCU singleton.
    pub fn mcu(&self) -> &S::Mcu {
        &self.mcu
    }

    /// Returns a mutable reference to the global MCU singleton.
    pub fn mcu_mut(&mut self) -> &mut S::Mcu {
        &mut self.mcu
    }

    /// Returns a shared reference to the global Clock singleton.
    pub fn clk(&self) -> &S::Clk {
        &self.clk
    }

    /// Returns a mutable reference to the global Clock singleton.
    pub fn clk_mut(&mut self) -> &mut S::Clk {
        &mut self.clk
    }

    /// Returns a shared reference to the global Heap singleton.
    pub fn heap(&self) -> &Heap {
        &self.heap
    }

    /// Returns a mutable reference to the global Heap singleton.
    pub fn heap_mut(&mut self) -> &mut Heap {
        &mut self.heap
    }

    /// Returns a shared reference to the global Tick singleton.
    pub fn tick(&self) -> &Tick {
        &self.tick
    }

    /// Returns a mutable reference to the global Tick singleton.
    pub fn tick_mut(&mut self) -> &mut Tick {
        &mut self.tick
    }

    /// Returns a shared reference to the global Pend singleton.
    pub fn pend(&self) -> &Pend {
        &self.pend
    }

    /// Returns a mutable reference to the global Pend singleton.
    pub fn pend_mut(&mut self) -> &mut Pend {
        &mut self.pend
    }

    /// Returns a shared reference to the global Interrupt Dispatcher.
    pub fn dispatcher(&self) -> &IrqDispatcher<S::Mcu> {
        &self.dispatcher
    }

    /// Returns a mutable reference to the global Interrupt Dispatcher.
    pub fn dispatcher_mut(&mut self) -> &mut IrqDispatcher<S::Mcu> {
        &mut self.dispatcher
    }

    /// Returns a shared reference to the global Console.
    pub fn console(&self) -> &'static Console<'static> {
        if let Some(console) = Console::borrow() {
            console
        } else {
            loop {}
        }
    }

    pub fn run<T, F: FnOnce(&Self) -> T>(&mut self, f: F) -> T {
        unsafe { asm!("cpsie i"); }
        let ret = f(&*self);
        unsafe { asm!("cpsid i"); }
        ret
    }
}

impl<S: SystemProvider> Deref for System<S> {
    type Target = S;
    fn deref(&self) -> &Self::Target {
        &self.provider
    }
}
impl<S: SystemProvider> DerefMut for System<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.provider
    }
}