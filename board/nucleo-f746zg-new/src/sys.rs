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
        use mcu::scb::*;
        // Enable Instruction Cache
        SCB.set_iciallu(|r| r);
        unsafe { asm!("dsb") }
        unsafe { asm!("isb") }
        SCB.with_ccr(|r| r.set_ic(1));
        Self::Mcu::default()
    }

    fn init_clk() -> Self::Clk {
        // 8 MHz External Clock
        // VCO = 432MHz
        // PLL = 216MHz
        // PLLQ = 48MHz
        // SYSCLK = 216MHz
        // AHB = 216MHz
        // APB1 = 54MHz
        // APB2 = 108MHz 

        enable_pll_hse_bypass_mode(8, 432, 0b00, 9);
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
        use mcu::ext::rcc::DedicatedClock;
        use mcu::usart::*;
        use mcu::pin::*;


        const USART: Usart3 = USART3;
        const USART_TX: Pd8 = PD8;
        const USART_RX: Pd9 = PD9;
        const USART_CLOCK: u32 = 16_000_000; // Use HSI Clock
        const USART_BAUD: u32 = 115_200;

        USART_TX
            .port_gate_enable()
            .connect_to(USART);

        USART_RX
            .port_gate_enable()
            .connect_to(USART);

        USART
            .set_clock_source(DedicatedClock::Hsi)
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
