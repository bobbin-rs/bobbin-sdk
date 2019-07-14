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
use mcu::nvmctrl::{NvmctrlPeriph, NVMCTRL};

pub type Clk = Clocks<DynamicClock<Osc48m, Osc32k>>;
pub type Dispatcher = ::bobbin_sys::irq_dispatch::IrqDispatcher<Mcu>;

fn init_console_helper(clk: &<Board as SystemProvider>::Clk) {
    use prelude::*;
    use mcu::pin::*;
    use mcu::sercom::*;
    use mcu::gclk;

    const SERCOM: Sercom0 = SERCOM0;
    const SERCOM_TX: Pa10 = PA10;
    const SERCOM_RX: Pa11 = PA11;
    const SERCOM_BAUD: u32 = 115200;

    SERCOM.gate_enable();
    SERCOM_RX.port().gate_enable();
    SERCOM_TX.port().gate_enable();
    // Set GCLK_GEN0 as source for SERCOM

    gclk::GCLK.set_clkctrl(|r| r
        .set_id(0x14 + 0) // ID corresponds to SERCOM
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
    type Output = ::mcu::nvmctrl::NvmctrlPeriph;
    fn flash(&self) -> &NvmctrlPeriph {
        &NVMCTRL
    }
}

pub fn init() -> System<Board> {
    System::take()
}
