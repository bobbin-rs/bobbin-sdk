use bobbin_sys::system::{System, SystemProvider};
use bobbin_sys::heap::Heap;
use bobbin_sys::tick::Tick;

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
        Self::Clk::default()
    }

    fn init_heap() -> Heap {
        unsafe { Heap::take().extended(4096) }
    }

    fn init_tick(clk: &Self::Clk) -> Tick {
        use mcu::ext::systick;
        
        systick::enable_systick_external(clk);
        Tick::take()
    }

    fn init_console(_: &Self::Clk, _: &mut Heap) {
        use prelude::*;
        use mcu::ext::rcc::*;
        use mcu::usart::*;
        use mcu::pin::*;

        const USART: Usart1 = USART1;
        const USART_TX: Pc4 = PC4;
        const USART_RX: Pc5 = PC5;
        const USART_CLOCK: u32 = 8_000_000; // Use HSI Clock
        const USART_BAUD: u32 = 115_200;

        USART_TX
            .port_gate_enable()
            .connect_to(USART);

        USART_RX
            .port_gate_enable()
            .connect_to(USART);

        USART
            .set_clock_source(UsartClock::Hsi)
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
