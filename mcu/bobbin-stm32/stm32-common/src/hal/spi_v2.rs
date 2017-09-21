pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::spi::*;
pub use ::chip::spi_v2::*;

use bobbin_common::bits::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataSize {
    Bits4 = 0b0011,
    Bits5 = 0b0100,
    Bits6 = 0b0101,
    Bits7 = 0b0110,
    Bits8 = 0b0111,
    Bits9 = 0b1000,
    Bits10 = 0b1001,
    Bits11 = 0b1010,
    Bits12 = 0b1011,
    Bits13 = 0b1100,
    Bits14 = 0b1101,
    Bits15 = 0b1110,
    Bits16 = 0b1111,
}

#[derive(Debug, Default)]
pub struct Config {    
    pub cr1: Cr1,
    pub cr2: Cr2,
}

impl Config {
    pub fn set_data_size(mut self, value: DataSize) -> Self {
        self.cr2 = self.cr2.set_ds(value as u8);
        self
    }

    pub fn baud_divider(self) -> U3 {
        self.cr1.br()
    }

    // Divide by 2^(n+1)
    pub fn set_baud_divider(mut self, value: U3) -> Self {
        self.cr1 = self.cr1.set_br(value);
        self
    }

    pub fn master(self) -> bool {
        self.cr1.test_mstr()
    }

    pub fn set_master(mut self, value: bool) -> Self {
        self.cr1 = self.cr1.set_mstr(value);
        self
    }

    pub fn cpol(self) -> bool {
        self.cr1.test_cpol()
    }

    pub fn set_cpol(mut self, value: bool) -> Self {
        self.cr1 = self.cr1.set_cpol(value);
        self
    }

    pub fn cpha(self) -> bool {
        self.cr1.test_cpha()
    }

    pub fn set_cpha(mut self, value: bool) -> Self {
        self.cr1 = self.cr1.set_cpha(value);
        self
    }    
}

impl Configure<Config> for SpiPeriph {
    fn config(&self) -> Config {
        Config {
            cr1: self.cr1(),
            cr2: self.cr2(),
        }
    }

    fn configure(&self, cfg: Config) -> &Self {
        self
            .set_cr1(|_| cfg.cr1)
            .set_cr2(|_| cfg.cr2);
        self
    }
}

impl Enabled for SpiPeriph {
    fn enabled(&self) -> bool {
        self.cr1().test_spe()
    }
    
    fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr1(|r| r.set_spe(value));
        self
    }
}

impl SpiOutputEnabled for SpiPeriph {
    fn output_enabled(&self) -> bool {
        self.cr2().test_ssoe()
    }
    
    fn set_output_enabled(&self, value: bool) -> &Self {
        self.with_cr2(|r| r.set_ssoe(value));
        self
    }
}

impl SpiBusy for SpiPeriph {
    fn busy(&self) -> bool {
        self.sr().test_bsy()
    }
}

impl SpiCanTx for SpiPeriph {
    fn can_tx(&self) -> bool {
        self.sr().test_txe()
    }
}

impl SpiTx<u8> for SpiPeriph {
    fn tx(&self, value: u8) -> &Self {
        self.set_dr8(|r| r.set_dr8(value));
        self
    }
}

impl SpiTx<u16> for SpiPeriph {
    fn tx(&self, value: u16) -> &Self {
        self.dr().set_dr(value);
        self
    }
}

impl SpiCanRx for SpiPeriph {
    fn can_rx(&self) -> bool {
        self.sr().test_rxne()
    }
}

impl SpiRx<u8> for SpiPeriph {
    fn rx(&self) -> u8 {
        self.dr8().dr8().value() as u8
    }
}

impl SpiRx<u16> for SpiPeriph {
    fn rx(&self) -> u16 {
        self.dr().dr().into()
    }
}