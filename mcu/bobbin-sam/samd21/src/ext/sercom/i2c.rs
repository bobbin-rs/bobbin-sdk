pub use bobbin_common::hal::i2c::*;
pub use ::periph::sercom::*;

use gclk;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TxMode {
    Write = 0,
    Read = 1,
}

impl SercomPeriph {
    pub fn index(&self) -> usize {
        match self.0 {
            0x42000800 => 0,
            0x42000c00 => 1,
            0x42001000 => 2,
            0x42001400 => 3,
            0x42001800 => 4,
            0x42001c00 => 5,
            _ => unimplemented!(),
        }
    }

    pub fn init_i2c(&self, baud: u32) {
        let s = self.i2cm();

        // Set GCLK_GEN0 as source for SERCOM

        gclk::GCLK.set_clkctrl(|_| gclk::Clkctrl(0)
            .set_id(0x14 + self.index())
            .set_gen(0x0)
            .set_clken(1)
        );

        // Wait for synchronization
        while gclk::GCLK.status().syncbusy() != 0 {}

        // Disable the SERCOM module
        s.with_ctrla(|r| r.set_enable(0));

        // Wait for synchronization
        while s.syncbusy().swrst() != 0 {}
        
        // Software Reset
        s.with_ctrla(|r| r.set_swrst(1));
    
        // Wait for synchronization
        while s.ctrla().swrst() != 0 {}

        // Wait for synchronization
        while s.syncbusy().swrst() != 0 || s.syncbusy().enable() != 0 {}

        // Set master mode
        s.set_ctrla(|_| i2cm::Ctrla(0).set_mode(0x5));
        // Set master mode and enable SCL Clock Stretch mode (stretch after ACK bit)
        //s.set_ctrla(i2c::Ctrla(0).set_mode(0x5).set_sdahold(0x3).set_sclsm(1));

        // Enable Smart mode
        //s.set_ctrlb(i2c::Ctrlb(0).set_smen(1));

        // set baud rate
        // SystemCoreClock / ( 2 * baudrate) - 1
        // 48Mhz / (2 * baudrate) - 1
        // baudrate = 100_000
        // baud = 240

        s.set_baud(|_| i2cm::Baud(0).set_baud(baud));
    }

    pub fn enable_i2c(&self) {
        let s = self.i2cm();
        // Enable the I²C master mode
        s.with_ctrla(|r| r.set_enable(1));

        // Wait until not busy
        while s.syncbusy().enable() != 0 {}

        // Set bus idle mode
        s.with_status(|r| r.set_busstate(0x1));

        // Wait until not busy
        while s.syncbusy().sysop() != 0 {}        
    }

    pub fn set_nack(&self) {
        self.i2cm().with_ctrlb(|r| r.set_ackact(1));
    }

    pub fn set_ack(&self) {
        self.i2cm().with_ctrlb(|r| r.set_ackact(0));
    }

    pub fn set_cmd(&self, cmd: u8) {
        self.i2cm().with_ctrlb(|r| r.set_cmd(cmd));
        while self.i2cm().syncbusy().sysop() != 0 {}
    }

    pub fn start_tx(&self, addr: u8, rw: TxMode) -> bool {
        let s = self.i2cm();
        let addr = (addr << 1) | rw as u8;
        //let addr = addr | rw as u8;
        while !self.bus_idle() && !self.bus_owner() {}
        s.with_addr(|r| r.set_addr(addr));
        match rw {
            TxMode::Write => {
                // wait until master busy flag clear
                while s.intflag().mb() == 0 {}
            },
            TxMode::Read => {
                // wait until slave busy flag clear
                while s.intflag().sb() == 0 {
                    // If the slave NACKS the address, the MB bit will be set.
                    // In that case, send a stop condition and return 0.
                    if s.intflag().mb() != 0 {
                        s.with_ctrlb(|r| r.set_cmd(3)); // stop condition
                        return false
                    }
                }
            },            
        }
        s.status().rxnack() == 0
    }

    pub fn send_data(&self, data: u8) -> bool {
        let s = self.i2cm();
        s.set_data(|_| i2cm::Data(data));
        // Wait transmission successful
        while s.intflag().mb() == 0 {
            // If a bus error occurs, the MB bit may never be set.
            // Check the bus error bit and bail if it's set.
            if s.status().buserr() != 0 {
                return false;
            }
        }
        s.status().rxnack() == 0
    }

    pub fn read_data(&self) -> u8 {
        let s = self.i2cm();
        // wait for receive
        while s.intflag().sb() == 0 {}
        s.data().data().value() as u8
    }


    pub fn bus_idle(&self) -> bool {
       self.i2cm().status().busstate() == 0x1
    }

    pub fn bus_owner(&self) -> bool {
        self.i2cm().status().busstate() == 0x2
    }
}

impl I2cTransfer<u8> for SercomPeriph {
    fn write(&self, addr: u8, write_buf: &[u8]) -> &Self {
        while !self.bus_idle() {}
        self.set_ack();
        if !self.start_tx(addr, TxMode::Write) {
            self.set_cmd(0x3); // Stop
            panic!("error starting tx");
        }
        for b in write_buf.iter() {
            if !self.send_data(*b) {
                self.set_cmd(0x3); // Stop
                panic!("error sending data");
            }        
        }
        self.set_cmd(0x3); // Stop    
        self
    }

    fn read(&self, addr: u8, read_buf: &mut[u8]) -> &Self {
        while !self.bus_idle() {}
        self.set_ack();
        if !self.start_tx(addr, TxMode::Read) {
            panic!("error starting tx");
        }              

        let len = read_buf.len();

        for i in 0..len-1 {
            read_buf[i]= self.read_data();
            self.set_ack();
            self.set_cmd(0x2); // Acknowledge
        }
        read_buf[len-1]= self.read_data();
        self.set_nack();
        self.set_cmd(0x3); // Stop
        self
    }

    fn transfer(&self, addr: u8, write_buf: &[u8], read_buf: &mut [u8]) -> &Self {        
        let read_len = read_buf.len();
        while !self.bus_idle() {}
        self.set_ack();
        if !self.start_tx(addr, TxMode::Write) {
            panic!("error starting tx");
        }
        for b in write_buf.iter() {
            if !self.send_data(*b) {
                self.set_cmd(0x3); // Stop
                panic!("error writing data");
            }                
        }
        while !self.i2cm().intflag().mb() == 0 {}

        if read_len > 0 {
            self.set_cmd(0x1); // Repeated Start

            if !self.start_tx(addr, TxMode::Read) {
                panic!("error starting tx");
            }              

            let len = read_buf.len();

            for i in 0..len-1 {
                read_buf[i]= self.read_data();
                self.set_ack();
                self.set_cmd(0x2); // Acknowledge
            }
            read_buf[len-1] = self.read_data();        
            self.set_nack();
            while !self.i2cm().intflag().sb() == 0 {}
        }
        self.set_cmd(0x3); // Stop
        self
    }    
}
