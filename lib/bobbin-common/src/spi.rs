pub trait SpiBusy {
    fn busy(&self) -> bool;
    fn wait_not_busy(&self) -> &Self {
        while self.busy() {}
        self
    }
}

pub trait SpiOutputEnabled {
    fn output_enabled(&self) -> bool;
    fn set_output_enabled(&self, value: bool) -> &Self;
}

pub trait SpiCanTx {
    fn can_tx(&self) -> bool;
    fn wait_can_tx(&self) -> &Self {
        while !self.can_tx() {}
        self
    }
}

pub trait SpiCanRx {
    fn can_rx(&self) -> bool;
    fn wait_can_rx(&self) -> &Self {
        while !self.can_rx() {}
        self
    }    
}

pub trait SpiTx<T: Copy> {
    fn tx(&self, c: T) -> &Self;
}

pub trait SpiRx<T: Copy> {   
    fn rx(&self) -> T;
}

pub trait SpiRead {
    fn read(&self, rx: &mut [u8]);
}

pub trait SpiWrite {
    fn write(&self, tx: &[u8]);
}

pub trait SpiTransfer : SpiRead + SpiWrite {
    fn transfer(&self, tx: &[u8], rx: &mut [u8]) {
        self.write(tx);
        self.read(rx);
    }
}