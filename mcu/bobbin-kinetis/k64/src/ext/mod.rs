pub use kinetis_common::ext::*;

pub mod clock;

use bobbin_mcu::mcu::{GetActiveIrq, IrqEnable, Sleep};
use nvic::NVIC;

impl GetActiveIrq for ::Mcu {
    fn get_active_irq() -> u8 {
        get_active_irq()
    }
}

impl IrqEnable for ::Mcu {
    fn irq_enabled(irq: u8) -> bool { NVIC.enabled(irq) }
    fn irq_enable(irq: u8) { NVIC.set_enabled(irq, true); }
    fn irq_disable(irq: u8) { NVIC.set_enabled(irq, false); }
}

impl Sleep for ::Mcu {    
    fn sleep() { sleep() }
}


// pub mod sim;
// pub mod mcg;
// pub mod mpu;
// pub mod osc;
// pub mod rcm;
// pub mod enet;
// pub mod crc;
// pub mod wdog;
// pub mod ftfe;
// pub mod dmamux;
// pub mod edma;
// pub mod ftm;
// pub mod pit;
// pub mod lptmr;
// pub mod spi;
// pub mod i2c;
// pub mod uart;
// pub mod usb;
// pub mod flexcan;
// pub mod dac;
// pub mod gpio;
// pub mod port;
// pub mod adc;
// pub mod clock;
