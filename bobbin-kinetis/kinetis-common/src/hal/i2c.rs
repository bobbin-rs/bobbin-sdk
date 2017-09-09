pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::i2c::*;

pub use ::chip::i2c::*;

impl I2cPeriph {
    pub fn init(&self, mult: u8, icr: u8) -> &Self {
        self.set_f(|_| F(0).set_mult(mult).set_icr(icr));
        self.with_c1(|r| r.set_iicen(1));
        self
    }

    fn data(&self) -> u8 {
        self.d().data().value()
    }

    fn set_data(&self, d: u8) {
        self.set_d(|_| D(0).set_data(d));
    }

    fn set_tx(&self, value: bool) {
        self.with_c1(|r| r.set_tx(value));
    }

    fn set_txak(&self, value: bool) {
        self.with_c1(|r| r.set_txak(value));
    }    

    fn with_tx<F: FnOnce(&Self) -> &Self>(&self, f: F) -> &Self {
        // Wait while Busy
        while self.s().busy() != 0 {}
        // Send Start
        self.with_c1(|r| r.set_mst(1).set_tx(1));
        f(&self);
        // Send Stop
        self.with_c1(|r| r.set_mst(0).set_tx(0));        
        // Wait while Busy
        while self.s().busy() != 0 {}
        self
    }

    fn restart(&self) -> &Self {
        self.with_c1(|r| r.set_tx(1).set_rsta(1));
        self                     
    }

    fn wait_transfer(&self) -> &Self {
        while self.s().iicif() == 0 {}
        self.with_s(|r| r.set_iicif(1));
        self
    }

    fn write(&self, addr: u8, bytes: &[u8]) -> &Self {
        self.set_data(addr << 1);
        self.wait_transfer();
        let mut n = bytes.len();
        let mut i = 0;
        while n > 0 {        
            self.set_data(bytes[i]);
            self.wait_transfer();
            i += 1;
            n -= 1;
        }
        self
    }

    pub fn read(&self, addr: u8, bytes: &mut [u8]) -> &Self {
        self.set_data(addr << 1 | 1);
        self.wait_transfer();
        self.set_tx(false);
        if bytes.len() == 1 {
            self.set_txak(true);
        } else {
            self.set_txak(false);
        }
        
        let _ = self.data();        
        let mut n = bytes.len();
        let mut i = 0;
        loop {
            self.wait_transfer();
            match n {
                1 => {
                    self.set_tx(true);
                    bytes[i] = self.data();
                    return self
                },
                2 => {
                    self.set_txak(true);
                    bytes[i] = self.data();
                    i += 1;
                },
                _ => {
                    bytes[i] = self.data();
                    i += 1;
                }
            }
            n -= 1;
        }
    }

    pub fn reg_write(&self, addr: u8, reg: u8, value: u8) -> &Self {
        let cmd = [reg, value];
        self.with_tx(|i| i.write(addr, &cmd))
    }

    pub fn reg_read(&self, addr: u8, reg: u8) -> u8 {
        let cmd = [reg];
        let mut buf = [0u8];
        self.with_tx(|i| i.write(addr, &cmd).restart().read(addr, &mut buf));
        buf[0]
    }
}

impl I2cTransfer<u8> for I2cPeriph {
    fn transfer(&self, addr: u8, tx_data: &[u8], rx_data: &mut [u8]) -> &Self {
        if tx_data.len() == 0 {            
            self.write(addr, tx_data)
        } else if rx_data.len() == 0 {
            self.write(addr, rx_data)
        } else {
            self.with_tx(|i| i.write(addr, tx_data).restart().read(addr, rx_data))
        }
    }

    fn write(&self, addr: u8, bytes: &[u8]) -> &Self {
        self.with_tx(|i| i.write(addr, bytes))
    }

    fn read(&self, addr: u8, bytes: &mut [u8]) -> &Self {
        self.with_tx(|i| i.read(addr, bytes))
    }
}


