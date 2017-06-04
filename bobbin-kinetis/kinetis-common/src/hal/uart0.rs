use chip::uart0::*;

pub trait Uart0Ext {
    fn enable(&self, bd: u16) -> &Self;
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
        u.set_c1(C1(0));
        //Disable TX and Receive
        u.set_c2(C2(0));
        // Set baud divider
        u.set_bdh(Bdh(0).set_sbr((baud_divider >> 8) as u8));
        u.set_bdl(Bdl(0).set_sbr(baud_divider as u8));

        u.set_c3(C3(0));                               
        // Oversampling 4x
        u.set_c4(C4(0).set_osr(0x3));        
        u.set_c5(C5(0));
        // Enable Transmit and Receive
        u.set_c2(C2(0).set_te(1).set_re(1));    
        self    
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
            Some(uart.d().rt())
        } else {
            None
        }
    }

    fn getc(&self) -> u8 {
        let uart = self;
        while uart.s1().rdrf() == 0 {}
        uart.d().rt()
        
    }

    fn putc(&self, value: u8) {            
        let uart = self;
        while uart.s1().tdre() == 0 {}
        uart.set_d(D(0).set_rt(value));
    }

    fn write(&self, data: &[u8]) {
        for i in 0..data.len() {
            self.putc(data[i]);
        }
    }        
}