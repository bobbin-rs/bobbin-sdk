use bobbin_sys::system::{System, SystemProvider};
#[cfg(not(feature = "no-heap"))]
use bobbin_sys::heap::Heap;
use bobbin_sys::tick::{Tick, HandleTick};
use bobbin_sys::pend::{Pend, HandlePend};
use bobbin_sys::irq_dispatch::{IrqDispatcher, IrqHandler};

use {Board, Mcu};

use mcu::clock::Clocks;
use mcu::ext::clock::*;

use bobbin_hal::flash::*;
use mcu::flash::{FlashPeriph, FLASH};

// pub type Clk = Clocks<DynamicClock<Osc8m, Osc32k>>;
pub type Dispatcher = ::bobbin_sys::irq_dispatch::IrqDispatcher<Mcu>;

impl SystemProvider for Board {
    type Mcu = Mcu;
    type Clk = Clk;

    fn init() -> Self {
        Self {}
    }

    fn init_mcu() -> Self::Mcu {
        Self::Mcu::default()
    }

    fn init_clk() -> Self::Clk {
        Self::Clk::default()
    }

    #[cfg(not(feature = "no-heap"))]
    fn init_heap() -> Heap {
        unsafe { Heap::take().extended(4096) }
    }

    fn init_dispatcher() -> IrqDispatcher<Self::Mcu> {
        static mut HANDLERS: [Option<IrqHandler>; 8] = [None; 8];
        unsafe { IrqDispatcher::init(HANDLERS.as_mut_ptr(), HANDLERS.len()) }
    }

    fn init_tick(clk: &Self::Clk) -> Tick {
        use mcu::ext::systick;
        static mut HANDLERS: [Option<*const HandleTick>; 8] = [None; 8];
        
        systick::enable_systick_external(clk);
        unsafe { Tick::init(HANDLERS.as_mut_ptr(), HANDLERS.len()) }
    }

    #[cfg(not(feature = "no-heap"))]
    fn init_console(_: &Self::Clk, _: &mut Heap) {
    }

    #[cfg(feature = "no-heap")]
    fn init_console(_: &Self::Clk) {
    }

    #[cfg(not(feature = "no-heap"))]
    fn init_led(_: &Self::Clk, _: &mut Heap) {
        ::led::init();
    }

    #[cfg(feature = "no-heap")]
    fn init_led(_: &Self::Clk) {
        ::led::init();
    }

    #[cfg(not(feature = "no-heap"))]
    fn init_btn(_: &Self::Clk, _: &mut Heap) {
        ::btn::init();
    }

    #[cfg(feature = "no-heap")]
    fn init_btn(_: &Self::Clk) {
        ::btn::init();
    }
}

impl GetFlash for Board {
    type Output = ::mcu::flash::FlashPeriph;
    fn flash(&self) -> &FlashPeriph {
        &FLASH
    }
}

pub fn init() -> System<Board> {
    System::take()
}
