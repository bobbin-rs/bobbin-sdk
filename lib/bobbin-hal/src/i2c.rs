//! Traits for using the I2C protocol.
//! 
//! TODO: Add Error results to trait

/// Abstracts basic blocking I2C reads and writes of type `T` to a device at address `addr`.
pub trait I2cTransfer<T> {
    /// A blocking write of `tx_data` to `addr` followed by a read of `rx_data.len()` bytes, stored in `rx_data`.
    fn transfer(&self, addr: T, tx_data: &[u8], rx_data: &mut [u8]) -> &Self;
    
    /// A blocking write of `tx_data` to `addr`.
    fn write(&self, addr: T, tx_data: &[u8]) -> &Self {
        self.transfer(addr, tx_data, &mut[])
    }

    /// A blocking read of `rx_data.len()` bytes from `addr`, stored in `rx_data`.
    fn read(&self, addr: T, rx_data: &mut [u8]) -> &Self {
        self.transfer(addr, &[], rx_data)
    }

    /// A blocking write of `reg` and `value` to `addr` in a single I2C transaction.
    fn write_reg(&self, addr: T, reg: u8, value: u8) -> &Self {
        self.write(addr, &[reg, value])
    }

    /// A blocking write of `reg` to `addr` followed by a blocking read of one byte from `addr`.
    fn read_reg(&self, addr: T, reg: u8) -> u8 {
        let mut buf = [0u8];
        self.transfer(addr, &[reg], &mut buf);
        buf[0]
    }
}

/// Abstracts basic blocking I2C transfers for lists of transmit and receive buffers.
pub trait I2cTransferIovecs<T> {
    fn transfer_iovecs(&self, addr: T, tx_bufs: &[&[u8]], rx_bufs: &mut[&mut [u8]]) -> &Self;
}

