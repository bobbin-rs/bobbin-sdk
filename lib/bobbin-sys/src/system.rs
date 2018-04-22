//! A global singleton for accessing system services.

use bobbin_mcu::mcu::Mcu;

use heap::Heap;
use tick::Tick;
use irq_dispatch::{IrqDispatcher, IrqHandler};
use console::Console;

struct SystemToken;
static mut SYSTEM_TOKEN: Option<SystemToken> = Some(SystemToken);

/// A global singleton that provides access to system services such as the MCU, Clock, Heap, Tick,
/// and the Interrupt Dispatcher.
pub struct System<MCU: Mcu, CLK> {
    mcu: MCU,
    clk: CLK,
    heap: Heap,
    tick: Tick,
    dispatcher: IrqDispatcher<MCU>,
    _private: ()
}

impl<MCU: Mcu, CLK> System<MCU, CLK> {
    /// Initializes and returns the global system singleton. This function will also initialize and
    /// acquire the global singletons used by System.
    pub fn take(mcu: MCU, clk: CLK) -> Self {
        unsafe { while let None = SYSTEM_TOKEN.take() {} }
        let mut heap = Heap::take();
        let tick = Tick::take();
        let irq_handlers: &mut [Option<IrqHandler>] = if let Ok(s) = heap.try_slice(None, 8) {
            s
        } else {
            &mut []
        };
        let dispatcher = IrqDispatcher::init(irq_handlers.as_mut_ptr(), irq_handlers.len());
        System {
            mcu,
            clk,
            heap,
            tick,
            dispatcher,
            _private: (),
        }
    }

    /// Releases the global system singleton as well as the global singletons used by System.
    pub fn release(system: Self) -> (MCU, CLK) {
        let System { mcu, clk, heap, tick, dispatcher, _private } = system;
        Tick::release(tick);
        Heap::release(heap);
        IrqDispatcher::release(dispatcher);
        unsafe { SYSTEM_TOKEN = Some(SystemToken) }
        (mcu, clk)
    }

    /// Returns a shared reference to the global MCU singleton.
    pub fn mcu(&self) -> &MCU {
        &self.mcu
    }

    /// Returns a mutable reference to the global MCU singleton.
    pub fn mcu_mut(&mut self) -> &mut MCU {
        &mut self.mcu
    }

    /// Returns a shared reference to the global Clock singleton.
    pub fn clk(&self) -> &CLK {
        &self.clk
    }

    /// Returns a mutable reference to the global Clock singleton.
    pub fn clk_mut(&mut self) -> &mut CLK {
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

    /// Returns a shared reference to the global Interrupt Dispatcher.
    pub fn dispatcher(&self) -> &IrqDispatcher<MCU> {
        &self.dispatcher
    }

    /// Returns a mutable reference to the global Interrupt Dispatcher.
    pub fn dispatcher_mut(&mut self) -> &mut IrqDispatcher<MCU> {
        &mut self.dispatcher
    }

    /// Returns a shared reference to the global Console.
    pub fn console(&self) -> Option<&'static Console<'static>> {
        Console::borrow()
    }

}