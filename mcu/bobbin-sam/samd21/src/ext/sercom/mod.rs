use sercom::*;
use bobbin_hal::configure::*;
use bobbin_hal::enabled::*;
use bobbin_hal::serial::*;
// pub use super::pm::PmEnabled;

use bobbin_bits::*;

pub mod i2c;
pub mod spi;

// pub use self::i2c::*;
// pub use self::spi::*;


// NOTE: Before usage, power up and set clocks

// pm::set_sercom_enabled(sercom, true);

// // Set GCLK_GEN0 as source for SERCOM

// gclk::GCLK.set_clkctrl(gclk::Clkctrl(0)
//     .set_id(0x14 + sercom_index(sercom))
//     .set_gen(0x0)
//     .set_clken(1)
// );
// // Wait for synchronization
// while gclk::GCLK.status().syncbusy() != 0 {}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Mode {
    UsartExternal = 0x0,
    UsartInternal = 0x1,
    SpiSlave = 0x2,
    SpiMaster = 0x3,
    I2cSlave = 0x4,
    I2cMaster = 0x5,
}

pub struct Config {
    ctrla: usart::Ctrla,
    ctrlb: usart::Ctrlb,
    baud: usart::Baud,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ctrla: usart::Ctrla(0).set_dord(0x1),
            ctrlb: usart::Ctrlb(0).set_rxen(0x1).set_txen(0x1).set_chsize(0x0),
            baud: usart::Baud(0),
        }
    }
}

impl Config {
    pub fn set_mode_usart_ext(self) -> Self {
        self.set_mode(Mode::UsartExternal)
    }

    pub fn set_mode_usart_int(self) -> Self {
        self.set_mode(Mode::UsartInternal)
    }

    pub fn set_mode(mut self, mode: Mode) -> Self {
        self.ctrla = self.ctrla.set_mode(mode as u8);
        self
    }

    pub fn set_baud_clock(mut self, baud: u32, clock: u32) -> Self {
        let baud_times_8 = clock * 8 / (16 * baud);
        let baud_fp = baud_times_8 % 8;
        let baud = baud_times_8 / 8;
        let baud_val = (baud as u16) | (baud_fp as u16) << 13;
        self.baud = usart::Baud(baud_val);
        self.ctrla = self.ctrla.set_sampr(1);
        self
    }



    pub fn set_baud(mut self, value: u16) -> Self {
        self.baud = usart::Baud(value);
        self
    }

    pub fn set_txpo<V: Into<U2>>(mut self, value: V) -> Self {
        self.ctrla = self.ctrla.set_txpo(value.into());
        self
    }

    pub fn set_rxpo<V: Into<U2>>(mut self, value: V) -> Self {
        self.ctrla = self.ctrla.set_rxpo(value.into());
        self
    }
}

impl Configure<Config> for SercomPeriph {
    fn config(&self) -> Config {
        let s = self.usart();
        Config {
            ctrla: s.ctrla(),
            ctrlb: s.ctrlb(),
            baud: s.baud(),
        }
    }

    fn configure(&self, cfg: Config) -> &Self {
        let s = self.usart();

        // Reset peripheral
        s.set_ctrla(|r| r.set_swrst(0x1));

        // Wait for reset
        while s.ctrla().swrst() != 0 {}

        // Update CTRLA
        s.set_ctrla(|_| cfg.ctrla);

        // Update CTRLB
        s.set_ctrlb(|_| cfg.ctrlb);

        // Wait for synchronization
        while s.syncbusy().ctrlb() != 0 {}

        // Update BAUD
        s.set_baud(|_| cfg.baud);

        self
    }
}

impl Enabled for SercomPeriph {
    fn enabled(&self) -> bool {
        self.usart().ctrla().test_enable()
    }

    fn set_enabled(&self, value: bool) -> &Self {
        self.usart().with_ctrla(|r| r.set_enable(value));
        self
    }        
}

impl SerialTx<u8> for SercomPeriph {    
    fn can_tx(&self) -> bool {
        self.usart().intflag().dre() != 0
    }

    fn tx(&self, c: u8) -> &Self {
        self.usart().set_data(|r| r.set_data(c));
        self
    }
}

impl SerialRx<u8> for SercomPeriph {
    fn can_rx(&self) -> bool {
        self.usart().intflag().rxc() != 0
    }

    fn rx(&self) -> u8 {
        self.usart().data().data().value() as u8
    }
}


impl SerialTxIrq for SercomPeriph {
    fn tx_irq(&self) -> bool {
        self.usart().intenset().test_dre()
    }

    fn set_tx_irq(&self, value: bool) -> &Self {
        if value {
            self.usart().set_intenset(|r| r.set_dre(1));
        } else {
            self.usart().set_intenclr(|r| r.set_dre(1));
        }
        self
    }
}

impl SerialRxIrq for SercomPeriph {
    fn rx_irq(&self) -> bool {
        self.usart().intenset().test_rxc()
    }

    fn set_rx_irq(&self, value: bool) -> &Self {
        if value {
            self.usart().set_intenset(|r| r.set_rxc(1));
        } else {
            self.usart().set_intenclr(|r| r.set_rxc(1));
        }
        self
    }
}
