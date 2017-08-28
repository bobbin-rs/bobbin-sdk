pub use bobbin_common::serial::*;
use chip::uart0::*;

impl Uart0Periph {
    pub fn enable(&self, baud_divider: u16) -> &Self {
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

    pub fn disable(&self) -> &Self {
        self.set_c2(|r| r.set_te(0).set_re(0))
    }
    pub fn tx_empty(&self) -> bool {
        self.s1().tdre() != 0
    }

    pub fn tx_complete(&self) -> bool {
        self.s1().tc() != 0
    }

    pub fn rx_full(&self) -> bool {
        self.s1().rdrf() != 0
    }

    pub fn idle(&self) -> bool {
        self.s1().idle() != 0
    }        

    // pub fn try_getc(&self) -> Option<u8> {
    //     let uart = self;
    //     if uart.s1().rdrf() != 0 {
    //         Some(uart.d().rt().into())
    //     } else {
    //         None
    //     }
    // }

    // pub fn getc(&self) -> u8 {
    //     let uart = self;
    //     while uart.s1().rdrf() == 0 {}
    //     uart.d().rt().into()        
    // }

    // pub fn putc(&self, value: u8) {            
    //     let uart = self;
    //     while uart.s1().tdre() == 0 {}
    //     uart.set_d(|r| r.set_rt(value));
    // }

    // pub fn write(&self, data: &[u8]) {
    //     for i in 0..data.len() {
    //         self.putc(data[i]);
    //     }
    // }        
}

impl SerialTx<u8> for Uart0Periph {    
    fn can_tx(&self) -> bool {
        self.tx_empty()
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_d(|r| r.set_rt(c))
    }
}

impl SerialRx<u8> for Uart0Periph {
    fn can_rx(&self) -> bool {
        self.rx_full()
    }

    fn rx(&self) -> u8 {
        self.d().rt().value()
    }
}