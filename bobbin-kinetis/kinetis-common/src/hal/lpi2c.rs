pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::i2c::*;

pub use ::chip::lpi2c::*;

impl Lpi2cPeriph {
    pub fn reset(&self) -> &Self {
        self.with_mcr(|r| r.set_rst(1));
        self.with_mcr(|r| r.set_rst(0));
        self
    }

    pub fn reset_receive_fifo(&self) -> &Self {
        self.with_mcr(|r| r.set_rrf(true))
    }

    pub fn reset_transmit_fifo(&self) -> &Self {
        self.with_mcr(|r| r.set_rtf(true))
    }

    pub fn bus_busy(&self) -> bool {
        self.msr().test_bbf()
    }

    pub fn master_busy(&self) -> bool {
        self.msr().test_mbf()
    }

    pub fn rdf(&self) -> bool {
        self.msr().test_rdf()
    }

    pub fn tdf(&self) -> bool {
        self.msr().test_tdf()
    }

    pub fn cmd_transmit(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b000).set_data(value))
    }

    pub fn cmd_receive(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b001).set_data(value))
    }

    pub fn cmd_stop(&self) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b010))
    }

    pub fn cmd_discard(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b011).set_data(value))
    }

    pub fn cmd_start(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b100).set_data(value))
    }

    pub fn cmd_start_nack(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b101).set_data(value))
    }

    pub fn cmd_start_hs(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b110).set_data(value))
    }
    
    pub fn cmd_start_hs_nack(&self, value: u8) -> &Self {
        self.set_mtdr(|r| r.set_cmd(0b111).set_data(value))
    }    
    
    pub fn rx_empty(&self) -> bool {
        self.mfsr().rxcount() == 0
    }

    pub fn receive(&self) -> u8 {
        self.mrdr().data().value()
    }

    pub fn wait_rdf(&self) -> &Self {
        while !self.msr().test_rdf() {}
        self
    }

    pub fn wait_tdf(&self) -> &Self {
        while !self.msr().test_tdf() {}
        self
    }
}

impl Enabled for Lpi2cPeriph {
    fn enabled(&self) -> bool {
        self.mcr().test_men()
    }

    fn set_enabled(&self, value: bool) -> &Self {
        self.with_mcr(|r| r.set_men(value))
    }
}

impl I2cTransfer<u8> for Lpi2cPeriph {
    fn write(&self, addr: u8, tx_data: &[u8]) -> &Self {        
        self.wait_tdf().cmd_start(addr << 1);
        for b in tx_data.iter() {
            self.wait_tdf().cmd_transmit(*b);
        }
        self.wait_tdf().cmd_stop();
        self
    }

    fn read(&self, addr: u8, rx_data: &mut [u8]) -> &Self {
        self.wait_tdf().cmd_start(addr << 1 | 1);
        self.wait_tdf().cmd_receive(rx_data.len() as u8 - 1);
        self.wait_tdf().cmd_stop();
        for i in 0..rx_data.len() {
            while self.rx_empty() {}
            rx_data[i] = self.receive();
        }
        self
    }

    fn transfer(&self, addr: u8, tx_data: &[u8], rx_data: &mut [u8]) -> &Self {
        self.wait_tdf().cmd_start(addr << 1);
        for b in tx_data.iter() {
            self.wait_tdf().cmd_transmit(*b);
        }
        self.wait_tdf().cmd_start(addr << 1 | 1);
        self.wait_tdf().cmd_receive(rx_data.len() as u8 - 1);
        self.wait_tdf().cmd_stop();
        for i in 0..rx_data.len() {
            while self.rx_empty() {}
            rx_data[i] = self.receive();
        }        
        self
    }
    
}