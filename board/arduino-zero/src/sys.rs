use bobbin_sys::system::{System, SystemProvider};
use bobbin_sys::heap::Heap;
use bobbin_sys::tick::Tick;

use {Board, Mcu};

use mcu::clock::Clocks;
use mcu::ext::clock::*;

use bobbin_hal::flash::*;
use mcu::nvmctrl::{NvmctrlPeriph, NVMCTRL};

pub type Clk = Clocks<DynamicClock<Osc48m, Osc32k>>;
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
        use mcu::ext::clock::*;
        run_48mhz();
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

    fn init_console(clk: &Self::Clk, _: &mut Heap) {
        use prelude::*;
        use mcu::pin::*;
        use mcu::sercom::*;
        use mcu::gclk;
                
        const SERCOM: Sercom5 = SERCOM5;
        const SERCOM_TX: Pb22 = PB22;
        const SERCOM_RX: Pb23 = PB23;
        const SERCOM_BAUD: u32 = 115200;

        SERCOM.gate_enable();
        SERCOM_RX.port().gate_enable();
        SERCOM_TX.port().gate_enable();
        // Set GCLK_GEN0 as source for SERCOM

        gclk::GCLK.set_clkctrl(|r| r
            .set_id(0x14 + 5) // ID corresponds to SERCOM
            .set_gen(0x0)
            .set_clken(1)
        );
        // Wait for synchronization
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Set Pin Configuration
        SERCOM_TX.connect_to(SERCOM);
        SERCOM_RX.connect_to(SERCOM);

        SERCOM
            .set_config(|c| c
                .set_mode_usart_int()
                .set_baud_clock(SERCOM_BAUD, clk.clock_for(SERCOM).as_u32())
                .set_txpo(1)
                .set_rxpo(3)
            )        
            .set_enabled(true);
        Console::set(Console::new(SERCOM.as_periph(), ConsoleMode::Cooked));
    }

    fn init_led(_: &Self::Clk, _: &mut Heap) {
        ::led::init();
    }

    fn init_btn(_: &Self::Clk, _: &mut Heap) {
        ::btn::init();
    }
}

impl GetFlash for Board {
    type Output = ::mcu::nvmctrl::NvmctrlPeriph;
    fn flash(&self) -> &NvmctrlPeriph {
        &NVMCTRL
    }
}

pub fn init() -> System<Board> {
    System::take()
}
