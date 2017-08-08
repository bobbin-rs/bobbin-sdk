use chip::uart0::*;

pub trait Uart0Ext {
    fn enable(&self, bd: u16) -> &Self;
    fn disable(&self) -> &Self;
    fn tx_empty(&self) -> bool;
    fn tx_complete(&self) -> bool;
    fn rx_full(&self) -> bool;
    fn idle(&self) -> bool;
    fn try_getc(&self) -> Option<u8>;
    fn getc(&self) -> u8;
    fn putc(&self, value: u8);
    fn write(&self, data: &[u8]);
}

impl<T> Uart0Ext for Periph<T> {
    fn enable(&self, baud_divider: u16) -> &Self {
        let u = self;
        u.set_c1(|r| r);
        //Disable TX and Receive
        u.set_c2(|r| r);
        // Set baud divider
        u.set_bdh(|r| r.set_sbr((baud_divider >> 8) as u8));
        u.set_bdl(|r| r.set_sbr(baud_divider as u8));

        u.set_c3(|r| r);                               
        // Oversampling 4x
        u.set_c4(|r| r.set_osr(0x3));        
        u.set_c5(|r| r);
        // Enable Transmit and Receive
        u.set_c2(|r| r.set_te(1).set_re(1));    
        self    
    }    

    fn disable(&self) -> &Self {
        self.set_c2(|r| r.set_te(0).set_re(0))
    }
    fn tx_empty(&self) -> bool {
        self.s1().tdre() != 0
    }

    fn tx_complete(&self) -> bool {
        self.s1().tc() != 0
    }

    fn rx_full(&self) -> bool {
        self.s1().rdrf() != 0
    }

    fn idle(&self) -> bool {
        self.s1().idle() != 0
    }        

    fn try_getc(&self) -> Option<u8> {
        let uart = self;
        if uart.s1().rdrf() != 0 {
            Some(uart.d().rt().into())
        } else {
            None
        }
    }

    fn getc(&self) -> u8 {
        let uart = self;
        while uart.s1().rdrf() == 0 {}
        uart.d().rt().into()        
    }

    fn putc(&self, value: u8) {            
        let uart = self;
        while uart.s1().tdre() == 0 {}
        uart.set_d(|r| r.set_rt(value));
    }

    fn write(&self, data: &[u8]) {
        for i in 0..data.len() {
            self.putc(data[i]);
        }
    }        
}