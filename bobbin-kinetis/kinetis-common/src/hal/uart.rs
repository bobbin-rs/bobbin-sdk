pub use bobbin_common::configure::*;
pub use bobbin_common::serial::*;

use bobbin_common::bits::*;
use chip::uart::*;

#[derive(Default)]
pub struct Config {
    bdh: Bdh,
    bdl: Bdl,
    c1: C1,
    c2: C2,
    c3: C3,
    c4: C4,
    c5: C5,
}

impl Config {
    pub fn set_baud_divisor(&mut self, value: U13) -> &Self {
        let baud_divisor: u16 = value.into();
        self.bdh.set_sbr((baud_divisor >> 8) as u8);
        self.bdl.set_sbr(baud_divisor as u8);
        self
    }
}

impl Configure<Config> for UartPeriph {
    fn config(&self) -> Config {
        Config {
            bdh: self.bdh(),
            bdl: self.bdl(),
            c1: self.c1(),
            c2: self.c2(),
            c3: self.c3(),
            c4: self.c4(),
            c5: self.c5(),
        }
    }
    fn configure(&self, cfg: Config) -> &Self {
        self
            .set_bdh(|_| cfg.bdh)
            .set_bdl(|_| cfg.bdl)
            .set_c1(|_| cfg.c1)
            .set_c2(|_| cfg.c2)
            .set_c3(|_| cfg.c3)
            .set_c4(|_| cfg.c4)
            .set_c5(|_| cfg.c5)
    }
}

impl UartPeriph {
    pub fn enable(&self, baud_divider: u16) -> &Self {
        let u = self;
        u.set_c1(|r| r);
        //Disable TX and Receive
        u.set_c2(|r| r);
        // Set baud divider
        u.set_bdh(|r| r.set_sbr((baud_divider >> 8) as u8));
        u.set_bdl(|r| r.set_sbr(baud_divider as u8));
        
        u.set_c3(|r| r);                       
        u.set_c4(|r| r);
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

    // pub fn write(&self, data: &[u8]) -> usize {
    //     for i in 0..data.len() {
    //         self.putc(data[i]);
    //     }
    //     data.len()
    // }        
}

impl SerialTx<u8> for UartPeriph {    
    fn can_tx(&self) -> bool {
        self.tx_empty()
    }

    fn tx(&self, c: u8) -> &Self {
        self.set_d(|r| r.set_rt(c))
    }
}

impl SerialRx<u8> for UartPeriph {
    fn can_rx(&self) -> bool {
        self.rx_full()
    }

    fn rx(&self) -> u8 {
        self.d().rt().value()
    }
}