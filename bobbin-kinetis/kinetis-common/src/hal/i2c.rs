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

    pub fn with_tx<F: FnOnce(&Self)>(&self, f: F) -> &Self {
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

    pub fn restart(&self) -> &Self {
        self.with_c1(|r| r.set_tx(1).set_rsta(1));
        self                     
    }

    pub fn wait_transfer(&self) -> &Self {
        while self.s().iicif() == 0 {}
        self.with_s(|r| r.set_iicif(1));
        self
    }

    pub fn write(&self, addr: u8, bytes: &[u8]) -> &Self {
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

    pub fn data(&self) -> u8 {
        self.d().data().value()
    }

    pub fn set_data(&self, d: u8) {
        self.set_d(|_| D(0).set_data(d));
    }

    pub fn set_tx(&self, value: bool) {
        self.with_c1(|r| r.set_tx(value));
    }

    pub fn set_txak(&self, value: bool) {
        self.with_c1(|r| r.set_txak(value));
    }    
}

// impl I2cTransfer<u8> for I2cPeriph {
//     fn transfer(&self, addr: u8, tx_data: &[u8], rx_data: &mut [u8]) -> &Self {
//         self.write(addr, tx_data);
//         self.read(addr, rx_data);
//         self
//     }

//     fn write(&self, addr: u8, bytes: &[u8]) -> &Self {
//         let i2c = self;
//         // Wait while Busy
//         while self.s().busy() != 0 {}
//         // Send Start
//         self.with_c1(|r| r.set_tx(1).set_mst(1));                        
//         // Send Slave Address
//         self.set_d(|_| D(0).set_data(addr << 1));
//         // wait for interrupt
//         while self.s().iicif() == 0 {}
//         self.with_s(|r| r.set_iicif(1));

//         for i in 0..bytes.len() {                
//             // Send Byte
//             self.set_d(|_| D(0).set_data(bytes[i]));
//             // wait for interrupt
//             while self.s().iicif() == 0 {}
//             self.with_s(|r| r.set_iicif(1));
//         }
        
//         self.with_c1(|r| r.set_mst(0).set_tx(0));
//         self
//     }

//     fn read(&self, addr: u8, bytes: &mut [u8]) -> &Self {
//         let i2c = self;
//         // Wait while Busy
//         while self.s().busy() != 0 {}
//         // Send Start
//         self.with_c1(|r| r.set_tx(1).set_mst(1));                        
//         // Send Slave Address
//         self.set_d(|_| D(0).set_data((addr << 1) | 1));
//         // wait for interrupt
//         while self.s().iicif() == 0 {}
//         self.with_s(|r| r.set_iicif(1));

//         // Enter Receive Mode with ACK
//         self.with_c1(|r| r.set_tx(0).set_txak(0));

//         // Receive dummy byte
//         let _ = self.d().data();
//         // wait for interrupt
//         while self.s().iicif() == 0 {}
//         self.with_s(|r| r.set_iicif(1));

//         let len = bytes.len();

//         for i in 0..len-2 {
//             self.with_c1(|r| r.set_txak(0));
//             bytes[i] = self.d().data().value();
//             // wait for interrupt
//             while self.s().iicif() == 0 {}
//             self.with_s(|r| r.set_iicif(1));
//         }
//         // Set NACK
//         self.with_c1(|r| r.set_txak(1));

//         bytes[len-2] = self.d().data().value();
//         // wait for interrupt
//         while self.s().iicif() == 0 {}
//         self.with_s(|r| r.set_iicif(1));
//         // send stop
//         self.with_c1(|r| r.set_mst(0));
//         bytes[len-1] = self.d().data().value();
//         self
//     }
// }


