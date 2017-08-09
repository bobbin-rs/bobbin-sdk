pub trait SerialTx<T: Copy> {
    fn can_tx(&self) -> bool;

    fn tx(&self, c: T) -> &Self;

    fn wait_tx(&self) -> &Self {
        while !self.can_tx() {}
        self
    }

    fn putc(&self, c: T) -> &Self {
        self.wait_tx().tx(c)
    }

    fn try_putc(&self, c: T) -> bool {
        if self.can_tx() {
            self.tx(c);
            true
        } else {
            false
        }
    }    

    fn write(&self, buf: &[T]) -> &Self {
        for b in buf.iter() {
            self.putc(*b);
        }
        self
    }    
}

pub trait SerialRx<T: Copy> {   
    fn can_rx(&self) -> bool;

    fn rx(&self) -> T;

    fn wait_rx(&self) -> &Self {
        while !self.can_rx() {}
        self
    }

    fn getc(&self) -> T {
        self.wait_rx().rx()
    }

    fn try_getc(&self) -> Option<T> {
        if self.can_rx() {
            Some(self.rx())
        } else {
            None
        }
    }        
}