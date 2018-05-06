use bobbin_sys::system::{System, SystemProvider};
use bobbin_sys::heap::Heap;
use bobbin_sys::tick::Tick;
use bobbin_sys::pend::{Pend, HandlePend};
use bobbin_sys::irq_dispatch::{IrqDispatcher, IrqHandler};

use {Board, Mcu};

use mcu::clock::Clocks;
use mcu::ext::clock::*;

use bobbin_hal::flash::*;
use mcu::ftfe::{FtfePeriph, FTFE};

pub type Clk = Clocks<DynamicClock<Extal50m, Extal32k>>;
pub type Dispatcher = ::bobbin_sys::irq_dispatch::IrqDispatcher<Mcu>;

impl SystemProvider for Board {
    type Mcu = Mcu;
    type Clk = Clk;

    fn init() -> Self {
        use core::ptr;
        const WDOG_STCTRLH: *mut u16 = 0x4005_2000 as *mut u16;
        const WDOG_UNLOCK: *mut u16 = 0x4005_200e as *mut u16;

        // Disable Watchdog
        unsafe {
            // Unlock Watchdog
            ptr::write_volatile(WDOG_UNLOCK, 0xc520);
            ptr::write_volatile(WDOG_UNLOCK, 0xd928);
            // Disable Watchdog
            ptr::write_volatile(WDOG_STCTRLH, 0x00d2);
        }

        Self {}
    }

    fn init_mcu() -> Self::Mcu {
        Self::Mcu::default()
    }

    fn init_clk() -> Self::Clk {
        use mcu::ext::clock::*;        

        clock_init::init();

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
        
        systick::enable_systick_internal(clk);
        Tick::take()
    }

    fn init_pend() -> Pend {
        static mut HANDLERS: [Option<*const HandlePend>; 8] = [None; 8];
        unsafe { Pend::init(HANDLERS.as_mut_ptr(), HANDLERS.len()) }
    }

    fn init_console(clk: &Self::Clk, _: &mut Heap) {
        use prelude::*;
        use mcu::uart::*;
        use mcu::pin::*;

        const UART: Uart0 = UART0;
        const UART_RX: Ptb16 = PTB16;
        const UART_TX: Ptb17 = PTB17;
        const UART_BAUD: u32 = 115_200;

        // Enable Clocks
        UART.gate_enable();
        UART_TX.port().gate_enable();
        UART_RX.port().gate_enable();

        UART_TX.connect_to(UART);
        UART_RX.connect_to(UART);

        let baud_div = clk.clock_for(UART).as_u32() / (16 * UART_BAUD);
        UART
            .set_config(|c| c.set_baud_divisor(baud_div as u16))
            .set_enabled(true);
        Console::set(Console::new(UART.as_periph(), ConsoleMode::Cooked));        
    }

    fn init_led(_: &Self::Clk, _: &mut Heap) {
        ::led::init();
    }

    fn init_btn(_: &Self::Clk, _: &mut Heap) {
        ::btn::init();
    }
}

impl GetFlash for Board {
    type Output = ::mcu::ftfe::FtfePeriph;
    fn flash(&self) -> &FtfePeriph {
        &FTFE
    }
}

pub fn init() -> System<Board> {
    System::take()
}
