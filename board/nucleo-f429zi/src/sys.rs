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

pub type Clk = Clocks<DynamicClock<Osc8m, Osc32k>>;
pub type Dispatcher = ::bobbin_sys::irq_dispatch::IrqDispatcher<Mcu>;

fn init_console_helper(clk: &<Board as SystemProvider>::Clk) {
    use prelude::*;
    use mcu::usart::*;
    use mcu::pin::*;


    const USART: Usart3 = USART3;
    const USART_TX: Pd8 = PD8;
    const USART_RX: Pd9 = PD9;
    const USART_BAUD: u32 = 115_200;

    let usart_clk = clk.clock_for(USART).as_u32();

    USART_TX
        .port_gate_enable()
        .connect_to(USART);

    USART_RX
        .port_gate_enable()
        .connect_to(USART);

    USART
        .gate_enable()
        .set_config(|c| c.set_baud_clock(USART_BAUD, usart_clk))
        .enable();

    Console::set(Console::new(USART.as_periph(), ConsoleMode::Cooked));
}

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
        // 8 MHz External Clock
        // M = 8
        // N = 336
        // P = 2
        // Q = 7

        enable_pll_hse_bypass_mode(8, 336, 0b00, 7);
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

    fn init_pend() -> Pend {
        static mut HANDLERS: [Option<*const HandlePend>; 8] = [None; 8];
        unsafe { Pend::init(HANDLERS.as_mut_ptr(), HANDLERS.len()) }
    }

    #[cfg(not(feature = "no-heap"))]
    fn init_console(clk: &Self::Clk, _: &mut Heap) {
        init_console_helper(clk);
    }

    #[cfg(feature = "no-heap")]
    fn init_console(clk: &Self::Clk) {
        init_console_helper(clk);
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
