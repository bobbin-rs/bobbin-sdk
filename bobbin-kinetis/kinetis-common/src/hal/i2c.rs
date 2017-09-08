pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::i2c::*;

pub use ::chip::i2c::*;

impl I2cTransfer<u8> for I2cPeriph {
    fn transfer(&self, addr: u8, tx_data: &[u8], rx_data: &mut [u8]) -> &Self {
        self.write(addr, tx_data);
        self.read(addr, rx_data);
        self
    }

    fn write(&self, addr: u8, bytes: &[u8]) -> &Self {
        let i2c = self;
        // Wait while Busy
        while i2c.s().busy() != 0 {}
        // Send Start
        i2c.with_c1(|r| r.set_tx(1).set_mst(1));                        
        // Send Slave Address
        i2c.set_d(|_| D(0).set_data(addr << 1));
        // wait for interrupt
        while i2c.s().iicif() == 0 {}
        i2c.with_s(|r| r.set_iicif(1));

        for i in 0..bytes.len() {                
            // Send Byte
            i2c.set_d(|_| D(0).set_data(bytes[i]));
            // wait for interrupt
            while i2c.s().iicif() == 0 {}
            i2c.with_s(|r| r.set_iicif(1));
        }
        
        i2c.with_c1(|r| r.set_mst(0).set_tx(0));
        self
    }

    fn read(&self, addr: u8, bytes: &mut [u8]) -> &Self {
        let i2c = self;
        // Wait while Busy
        while i2c.s().busy() != 0 {}
        // Send Start
        i2c.with_c1(|r| r.set_tx(1).set_mst(1));                        
        // Send Slave Address
        i2c.set_d(|_| D(0).set_data((addr << 1) | 1));
        // wait for interrupt
        while i2c.s().iicif() == 0 {}
        i2c.with_s(|r| r.set_iicif(1));

        // Enter Receive Mode with ACK
        i2c.with_c1(|r| r.set_tx(0).set_txak(0));

        // Receive dummy byte
        let _ = i2c.d().data();
        // wait for interrupt
        while i2c.s().iicif() == 0 {}
        i2c.with_s(|r| r.set_iicif(1));

        let len = bytes.len();

        for i in 0..len-2 {
            i2c.with_c1(|r| r.set_txak(0));
            bytes[i] = i2c.d().data().value();
            // wait for interrupt
            while i2c.s().iicif() == 0 {}
            i2c.with_s(|r| r.set_iicif(1));
        }
        // Set NACK
        i2c.with_c1(|r| r.set_txak(1));

        bytes[len-2] = i2c.d().data().value();
        // wait for interrupt
        while i2c.s().iicif() == 0 {}
        i2c.with_s(|r| r.set_iicif(1));
        // send stop
        i2c.with_c1(|r| r.set_mst(0));
        bytes[len-1] = i2c.d().data().value();
        self
    }
}


