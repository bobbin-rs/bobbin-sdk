
pub trait I2cTransfer<T> {
    fn transfer(&self, addr: T, tx_data: &[u8], rx_data: &mut [u8]) -> &Self;
    
    fn write(&self, addr: T, tx_data: &[u8]) -> &Self {
        self.transfer(addr, tx_data, &mut[])
    }

    fn read(&self, addr: T, rx_data: &mut [u8]) -> &Self {
        self.transfer(addr, &[], rx_data)
    }

    fn write_reg(&self, addr: T, reg: u8, value: u8) -> &Self {
        self.write(addr, &[reg, value])
    }

    fn read_reg(&self, addr: T, reg: u8) -> u8 {
        let mut buf = [0u8];
        self.transfer(addr, &[reg], &mut buf);
        buf[0]
    }
}