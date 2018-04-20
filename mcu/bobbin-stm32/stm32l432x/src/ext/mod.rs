pub use stm32_common::ext::*;
use bobbin_mcu::mcu::{GetActiveIrq, IrqEnable, Sleep};
use nvic::NVIC;

pub mod rcc;
pub mod clock;

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

// pub mod flash;
// pub mod pwr;
// pub mod rcc;
// pub mod syscfg;
// pub mod c_adc;
// pub mod dac;
// pub mod rng;
// pub mod iwdg;
// pub mod wwdg;
// pub mod crc;
// pub mod rtc;
// pub mod lptim;
// pub mod exti;
// pub mod dma;
// pub mod i2c;
// pub mod tim_adv;
// pub mod tim_bas;
// pub mod tim_gen;
// pub mod gpio;
// pub mod usart;
// pub mod lpuart;
// pub mod spi;
// pub mod adc;
// pub mod clock;
