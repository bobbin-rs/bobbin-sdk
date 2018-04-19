use heap::Heap;
use tick::Tick;

struct SystemToken;
static mut SYSTEM_TOKEN: Option<SystemToken> = Some(SystemToken);

pub struct System<MCU, CLK> {
    mcu: MCU,
    clk: CLK,
    heap: Heap,
    tick: Tick,
    _private: ()
}

impl<MCU, CLK> System<MCU, CLK> {
    pub fn take(mcu: MCU, clk: CLK) -> Self {
        unsafe { while let None = SYSTEM_TOKEN.take() {} }
        System {
            mcu,
            clk,
            heap: Heap::take(),
            tick: Tick::take(),
            _private: (),
        }
    }

    pub fn release(system: Self) -> (MCU, CLK) {
        let System { mcu, clk, heap, tick, _private } = system;
        Tick::release(tick);
        Heap::release(heap);
        unsafe { SYSTEM_TOKEN = Some(SystemToken) }
        (mcu, clk)
    }

    pub fn mcu(&self) -> &MCU {
        &self.mcu
    }

    pub fn mcu_mut(&mut self) -> &mut MCU {
        &mut self.mcu
    }

    pub fn clk(&self) -> &CLK {
        &self.clk
    }

    pub fn clk_mut(&mut self) -> &mut CLK {
        &mut self.clk
    }

    pub fn heap(&self) -> &Heap {
        &self.heap
    }

    pub fn heap_mut(&mut self) -> &mut Heap {
        &mut self.heap
    }

    pub fn tick(&self) -> &Tick {
        &self.tick
    }

    pub fn tick_mut(&mut self) -> &mut Tick {
        &mut self.tick
    }

}