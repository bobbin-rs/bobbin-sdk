use bobbin_sys::system::{System, SystemProvider};
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
        enable_pll_hse_bypass_mode(8, 360, 0b00, 7);

        Self::Clk::default()
    }

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

    fn init_console(_: &Self::Clk, _: &mut Heap) {
        use prelude::*;
        use mcu::usart::*;
        use mcu::pin::*;
        use mcu::bobbin_mcu::*;
        use mcu::bobbin_mcu::prelude::*;
        use mcu::bobbin_hal::prelude::*;

        const USART: Usart1 = USART1;
        const USART_TX: Pa9 = PA9;
        const USART_RX: Pa10 = PA10;
        const USART_CLOCK: u32 = 90_000_000; // Use HSI Clock
        const USART_BAUD: u32 = 115_200;

        USART_TX
            .port_gate_enable()
            .connect_to(USART);

        USART_RX
            .port_gate_enable()
            .connect_to(USART);

        USART
            .gate_enable()
            .set_config(|c| c.set_baud_clock(USART_BAUD, USART_CLOCK))
            .enable();

        Console::set(Console::new(USART.as_periph(), ConsoleMode::Cooked));
    }

    fn init_led(_: &Self::Clk, _: &mut Heap) {
        ::led::init();
    }

    fn init_btn(_: &Self::Clk, _: &mut Heap) {
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
