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
        // Main System Clock = 80 MHz
        // APB2 = 80 MHz
        // APB1 = 80 MHz
        // AHB = 80 MHz
        // 
        // HSI @ 16 MHz
        // VCO @ 160 MHz (M = 1, N = 10)
        // PLL @ 80 Mhz (R = 2)
        // FLASH: 4 wait states
                
        clock_init::init_pll();
        Self::Clk::default()
    }

    fn init_heap() -> Heap {
        unsafe { Heap::extend(4096) }
        Heap::take()
    }

    fn init_tick(clk: &Self::Clk) -> Tick {
        use mcu::systick::SYSTICK;
        use mcu::ext::systick::SystickHz;

        let ms_hz = (clk.systick_hz() / 1000).as_u32() - 1;    
        let st = SYSTICK;
        st.set_reload_value(ms_hz);
        st.set_current_value(ms_hz);
        st.set_enabled(true);
        st.set_tick_interrupt(true);           

        Tick::take()
    }

    fn init_console(_: &Self::Clk, _: &mut Heap) {    
        use prelude::*;
        use mcu::ext::rcc::UsartClock;           
        use mcu::usart::*;
        use mcu::pin::*;

        const USART: Usart2 = USART2;
        const USART_TX: Pa2 = PA2;
        const USART_RX: Pa15 = PA15;
        const USART_CLOCK: u32 = 16_000_000; // Use HSI Clock
        const USART_BAUD: u32 = 115_200;
        USART_TX
            .port_gate_enable()
            .connect_to(USART);

        USART_RX
            .port_gate_enable()
            .connect_to(USART);

        USART
            .set_clock_source(UsartClock::Hsi16)
            .gate_enable()
            .set_config(|c| c.set_baud(USART_BAUD, USART_CLOCK))
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
