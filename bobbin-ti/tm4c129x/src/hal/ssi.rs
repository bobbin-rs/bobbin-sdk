pub use bobbin_common::spi::*;
pub use super::sysctl::SysctlEnabled;
pub use ::chip::ssi::*;

impl SsiPeriph {
    pub fn init(&self) -> &Self {
        // BR = SysClk / (CPSDVSR * (1 + SCR))
        // with SysClk = 120Mhz and desired BR of 1Mhz
        // 1Mhz = 120Mhz / (CPSDVSR * (1 + SCR))
        // 120 = (CPSDVSR * (1 + SCR))
        // CPSDVSR = 30
        // SCR = 39

        // // Wait for SSE=0
        // while self.cr0().sse() != 0 {}

        self.with_cr1(|r| r.set_sse(0));
        
        // Master Mode
        self.set_cr1(|_| Cr1(0));

        self.set_cpsr(|_| Cpsr(0).set_cpsdvsr(30));

        self.set_cr0(|_| Cr0(0)
            .set_scr(39)
            .set_sph(0)
            .set_spo(0)
            .set_frf(0)
            .set_dss(0x7)
        );
        self.with_cr1(|r| r.set_sse(1));
        self
    }

    pub fn write(&self, bytes: &[u8]) -> &Self {
        for i in 0..bytes.len() {
            while self.sr().tnf() == 0 {}
            self.set_dr(|_| Dr(0).set_data(bytes[i]));
        }
        while self.sr().bsy() != 0 {}
        self
    }    

    pub fn read(&self, bytes: &mut [u8]) -> &Self {
        // Flush FIFO
        while self.sr().rne() != 0 {
            let _ = self.dr().data();
        }
        
        for i in 0..bytes.len() {
            while self.sr().tnf() == 0 {}
            self.set_dr(|_| Dr(0).set_data(0xff));
            while self.sr().rne() == 0 {}
            bytes[i] = self.dr().data().into();
        }
        self
    }

    pub fn transfer(&self, bytes_out: &[u8], bytes_in: &mut [u8]) -> &Self {
        // Flush FIFO
        while self.sr().rne() != 0 {
            let _ = self.dr().data();
        }

        for i in 0..bytes_out.len() {
            while self.sr().tnf() == 0 {}
            self.set_dr(|_| Dr(0).set_data(bytes_out[i]));
            while self.sr().rne() == 0 {}
            bytes_in[i] = self.dr().data().into();
        }
        self
    }    
}

