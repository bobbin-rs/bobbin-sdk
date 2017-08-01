pub use chip::uart::*;
pub use super::sysctl::SysctlEnabled;

pub trait UartExt {
    fn configure(&self, baud_hz: u32, sysclk_hz: u32) -> &Self;
    fn enable(&self) -> &Self;
    fn disable(&self) -> &Self;
    fn try_getc(&self) -> Option<u8>;
    fn putc(&self, c: u8);
    fn write(&self, buf: &[u8]);
    fn data(&self) -> u8;
    fn set_data(&self, value: u8) -> &Self;
    fn rxfe(&self) -> bool;
    fn txff(&self) -> bool;
}

impl<T> UartExt for Periph<T> {
    fn configure(&self, baud_hz: u32, sysclk_hz: u32) -> &Self {
        let baud_div = ((8 * sysclk_hz) / baud_hz) + 1;
        let baud_int = baud_div / 64;
        let baud_frac = baud_div % 64;

        self.with_ctl(|r| r.set_uarten(0));
        self.with_ibrd(|r| r.set_divint(baud_int));
        self.with_fbrd(|r| r.set_divfrac(baud_frac));
        self.with_lcrh(|r| r.set_wlen(0x3).set_fen(1));
        self.with_ctl(|r| r.set_hse(1).set_rxe(1).set_txe(1));        
        self
    }

    fn enable(&self) -> &Self {
        self.with_ctl(|r| r.set_uarten(1))        
    }

    fn disable(&self) -> &Self {
        self.with_ctl(|r| r.set_uarten(0))
    }

    fn try_getc(&self) -> Option<u8> {
        if !self.rxfe() {
            Some(self.data())
        } else {
            None
        }        
    }

    fn putc(&self, c: u8) {
        while self.txff() {}
        self.set_data(c);
    }

    fn write(&self, buf: &[u8]) {
        for b in buf.iter() {
            self.putc(*b)
        }
    }

    fn data(&self) -> u8 {
        self.dr().data() as u8
    }

    fn set_data(&self, value: u8) -> &Self {  
        self.set_dr(Dr(0).set_data(value as u32))
    }
    
    fn rxfe(&self) -> bool {
        self.fr().rxfe() != 0
    }

    fn txff(&self) -> bool {
        self.fr().txff() != 0
    }        
}