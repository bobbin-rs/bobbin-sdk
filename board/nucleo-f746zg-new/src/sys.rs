use bobbin_sys::system::{System, SystemProvider};
use {Board, Mcu, Clk, Heap, Tick};

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
        ::clock::init();
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

    fn init_console(_clk: &Self::Clk) {
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
}

pub fn init() -> System<Board> {
    ::led::init();
    ::btn::init();
    System::take()
}
