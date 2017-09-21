#[allow(unused_imports)] use bobbin_common::*;

periph!( GPIO, Gpio, _GPIO, GpioPeriph, 0x40010000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct GpioPeriph(pub usize); 



impl GpioPeriph {
    #[doc="Get the *mut pointer for the PADREG register."]
    #[inline] pub fn padreg_mut<I: Into<bits::R13>>(&self, index: I) -> *mut Padreg { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x0 + (index << 2)) as *mut Padreg
    }

    #[doc="Get the *const pointer for the PADREG register."]
    #[inline] pub fn padreg_ptr<I: Into<bits::R13>>(&self, index: I) -> *const Padreg { 
           self.padreg_mut(index)
    }

    #[doc="Read the PADREG register."]
    #[inline] pub fn padreg<I: Into<bits::R13>>(&self, index: I) -> Padreg { 
        unsafe {
            read_volatile(self.padreg_ptr(index))
        }
    }

    #[doc="Write the PADREG register."]
    #[inline] pub fn set_padreg<I: Into<bits::R13>, F: FnOnce(Padreg) -> Padreg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.padreg_mut(index), f(Padreg(0)));
        }
        self
    }

    #[doc="Modify the PADREG register."]
    #[inline] pub fn with_padreg<I: Into<bits::R13> + Copy, F: FnOnce(Padreg) -> Padreg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.padreg_mut(index), f(self.padreg(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFG register."]
    #[inline] pub fn cfg_mut<I: Into<bits::R7>>(&self, index: I) -> *mut Cfg { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x40 + (index << 2)) as *mut Cfg
    }

    #[doc="Get the *const pointer for the CFG register."]
    #[inline] pub fn cfg_ptr<I: Into<bits::R7>>(&self, index: I) -> *const Cfg { 
           self.cfg_mut(index)
    }

    #[doc="Read the CFG register."]
    #[inline] pub fn cfg<I: Into<bits::R7>>(&self, index: I) -> Cfg { 
        unsafe {
            read_volatile(self.cfg_ptr(index))
        }
    }

    #[doc="Write the CFG register."]
    #[inline] pub fn set_cfg<I: Into<bits::R7>, F: FnOnce(Cfg) -> Cfg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(index), f(Cfg(0)));
        }
        self
    }

    #[doc="Modify the CFG register."]
    #[inline] pub fn with_cfg<I: Into<bits::R7> + Copy, F: FnOnce(Cfg) -> Cfg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(index), f(self.cfg(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the PADKEY register."]
    #[inline] pub fn padkey_mut(&self) -> *mut Padkey { 
        (self.0 + 0x60) as *mut Padkey
    }

    #[doc="Get the *const pointer for the PADKEY register."]
    #[inline] pub fn padkey_ptr(&self) -> *const Padkey { 
           self.padkey_mut()
    }

    #[doc="Read the PADKEY register."]
    #[inline] pub fn padkey(&self) -> Padkey { 
        unsafe {
            read_volatile(self.padkey_ptr())
        }
    }

    #[doc="Write the PADKEY register."]
    #[inline] pub fn set_padkey<F: FnOnce(Padkey) -> Padkey>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padkey_mut(), f(Padkey(0)));
        }
        self
    }

    #[doc="Modify the PADKEY register."]
    #[inline] pub fn with_padkey<F: FnOnce(Padkey) -> Padkey>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.padkey_mut(), f(self.padkey()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RD register."]
    #[inline] pub fn rd_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Rd { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x80 + (index << 2)) as *mut Rd
    }

    #[doc="Get the *const pointer for the RD register."]
    #[inline] pub fn rd_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Rd { 
           self.rd_mut(index)
    }

    #[doc="Read the RD register."]
    #[inline] pub fn rd<I: Into<bits::R2>>(&self, index: I) -> Rd { 
        unsafe {
            read_volatile(self.rd_ptr(index))
        }
    }

    #[doc="Write the RD register."]
    #[inline] pub fn set_rd<I: Into<bits::R2>, F: FnOnce(Rd) -> Rd>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.rd_mut(index), f(Rd(0)));
        }
        self
    }

    #[doc="Modify the RD register."]
    #[inline] pub fn with_rd<I: Into<bits::R2> + Copy, F: FnOnce(Rd) -> Rd>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.rd_mut(index), f(self.rd(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the WT register."]
    #[inline] pub fn wt_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Wt { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x88 + (index << 2)) as *mut Wt
    }

    #[doc="Get the *const pointer for the WT register."]
    #[inline] pub fn wt_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Wt { 
           self.wt_mut(index)
    }

    #[doc="Read the WT register."]
    #[inline] pub fn wt<I: Into<bits::R2>>(&self, index: I) -> Wt { 
        unsafe {
            read_volatile(self.wt_ptr(index))
        }
    }

    #[doc="Write the WT register."]
    #[inline] pub fn set_wt<I: Into<bits::R2>, F: FnOnce(Wt) -> Wt>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.wt_mut(index), f(Wt(0)));
        }
        self
    }

    #[doc="Modify the WT register."]
    #[inline] pub fn with_wt<I: Into<bits::R2> + Copy, F: FnOnce(Wt) -> Wt>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.wt_mut(index), f(self.wt(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the WTS register."]
    #[inline] pub fn wts_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Wts { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x90 + (index << 2)) as *mut Wts
    }

    #[doc="Get the *const pointer for the WTS register."]
    #[inline] pub fn wts_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Wts { 
           self.wts_mut(index)
    }

    #[doc="Read the WTS register."]
    #[inline] pub fn wts<I: Into<bits::R2>>(&self, index: I) -> Wts { 
        unsafe {
            read_volatile(self.wts_ptr(index))
        }
    }

    #[doc="Write the WTS register."]
    #[inline] pub fn set_wts<I: Into<bits::R2>, F: FnOnce(Wts) -> Wts>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.wts_mut(index), f(Wts(0)));
        }
        self
    }

    #[doc="Modify the WTS register."]
    #[inline] pub fn with_wts<I: Into<bits::R2> + Copy, F: FnOnce(Wts) -> Wts>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.wts_mut(index), f(self.wts(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the WTC register."]
    #[inline] pub fn wtc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Wtc { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x98 + (index << 2)) as *mut Wtc
    }

    #[doc="Get the *const pointer for the WTC register."]
    #[inline] pub fn wtc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Wtc { 
           self.wtc_mut(index)
    }

    #[doc="Read the WTC register."]
    #[inline] pub fn wtc<I: Into<bits::R2>>(&self, index: I) -> Wtc { 
        unsafe {
            read_volatile(self.wtc_ptr(index))
        }
    }

    #[doc="Write the WTC register."]
    #[inline] pub fn set_wtc<I: Into<bits::R2>, F: FnOnce(Wtc) -> Wtc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtc_mut(index), f(Wtc(0)));
        }
        self
    }

    #[doc="Modify the WTC register."]
    #[inline] pub fn with_wtc<I: Into<bits::R2> + Copy, F: FnOnce(Wtc) -> Wtc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.wtc_mut(index), f(self.wtc(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the EN register."]
    #[inline] pub fn en_mut<I: Into<bits::R2>>(&self, index: I) -> *mut En { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xa0 + (index << 2)) as *mut En
    }

    #[doc="Get the *const pointer for the EN register."]
    #[inline] pub fn en_ptr<I: Into<bits::R2>>(&self, index: I) -> *const En { 
           self.en_mut(index)
    }

    #[doc="Read the EN register."]
    #[inline] pub fn en<I: Into<bits::R2>>(&self, index: I) -> En { 
        unsafe {
            read_volatile(self.en_ptr(index))
        }
    }

    #[doc="Write the EN register."]
    #[inline] pub fn set_en<I: Into<bits::R2>, F: FnOnce(En) -> En>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.en_mut(index), f(En(0)));
        }
        self
    }

    #[doc="Modify the EN register."]
    #[inline] pub fn with_en<I: Into<bits::R2> + Copy, F: FnOnce(En) -> En>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.en_mut(index), f(self.en(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the ENS register."]
    #[inline] pub fn ens_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Ens { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xa8 + (index << 2)) as *mut Ens
    }

    #[doc="Get the *const pointer for the ENS register."]
    #[inline] pub fn ens_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Ens { 
           self.ens_mut(index)
    }

    #[doc="Read the ENS register."]
    #[inline] pub fn ens<I: Into<bits::R2>>(&self, index: I) -> Ens { 
        unsafe {
            read_volatile(self.ens_ptr(index))
        }
    }

    #[doc="Write the ENS register."]
    #[inline] pub fn set_ens<I: Into<bits::R2>, F: FnOnce(Ens) -> Ens>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ens_mut(index), f(Ens(0)));
        }
        self
    }

    #[doc="Modify the ENS register."]
    #[inline] pub fn with_ens<I: Into<bits::R2> + Copy, F: FnOnce(Ens) -> Ens>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ens_mut(index), f(self.ens(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the ENC register."]
    #[inline] pub fn enc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Enc { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xb4 + (index << 2)) as *mut Enc
    }

    #[doc="Get the *const pointer for the ENC register."]
    #[inline] pub fn enc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Enc { 
           self.enc_mut(index)
    }

    #[doc="Read the ENC register."]
    #[inline] pub fn enc<I: Into<bits::R2>>(&self, index: I) -> Enc { 
        unsafe {
            read_volatile(self.enc_ptr(index))
        }
    }

    #[doc="Write the ENC register."]
    #[inline] pub fn set_enc<I: Into<bits::R2>, F: FnOnce(Enc) -> Enc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.enc_mut(index), f(Enc(0)));
        }
        self
    }

    #[doc="Modify the ENC register."]
    #[inline] pub fn with_enc<I: Into<bits::R2> + Copy, F: FnOnce(Enc) -> Enc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.enc_mut(index), f(self.enc(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the STMRCAP register."]
    #[inline] pub fn stmrcap_mut(&self) -> *mut Stmrcap { 
        (self.0 + 0xbc) as *mut Stmrcap
    }

    #[doc="Get the *const pointer for the STMRCAP register."]
    #[inline] pub fn stmrcap_ptr(&self) -> *const Stmrcap { 
           self.stmrcap_mut()
    }

    #[doc="Read the STMRCAP register."]
    #[inline] pub fn stmrcap(&self) -> Stmrcap { 
        unsafe {
            read_volatile(self.stmrcap_ptr())
        }
    }

    #[doc="Write the STMRCAP register."]
    #[inline] pub fn set_stmrcap<F: FnOnce(Stmrcap) -> Stmrcap>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmrcap_mut(), f(Stmrcap(0)));
        }
        self
    }

    #[doc="Modify the STMRCAP register."]
    #[inline] pub fn with_stmrcap<F: FnOnce(Stmrcap) -> Stmrcap>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmrcap_mut(), f(self.stmrcap()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IOMIRQ register."]
    #[inline] pub fn iomirq_mut<I: Into<bits::R6>>(&self, index: I) -> *mut Iomirq { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xc0 + (index << 2)) as *mut Iomirq
    }

    #[doc="Get the *const pointer for the IOMIRQ register."]
    #[inline] pub fn iomirq_ptr<I: Into<bits::R6>>(&self, index: I) -> *const Iomirq { 
           self.iomirq_mut(index)
    }

    #[doc="Read the IOMIRQ register."]
    #[inline] pub fn iomirq<I: Into<bits::R6>>(&self, index: I) -> Iomirq { 
        unsafe {
            read_volatile(self.iomirq_ptr(index))
        }
    }

    #[doc="Write the IOMIRQ register."]
    #[inline] pub fn set_iomirq<I: Into<bits::R6>, F: FnOnce(Iomirq) -> Iomirq>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.iomirq_mut(index), f(Iomirq(0)));
        }
        self
    }

    #[doc="Modify the IOMIRQ register."]
    #[inline] pub fn with_iomirq<I: Into<bits::R6> + Copy, F: FnOnce(Iomirq) -> Iomirq>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.iomirq_mut(index), f(self.iomirq(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the LOOPBACK register."]
    #[inline] pub fn loopback_mut(&self) -> *mut Loopback { 
        (self.0 + 0xd8) as *mut Loopback
    }

    #[doc="Get the *const pointer for the LOOPBACK register."]
    #[inline] pub fn loopback_ptr(&self) -> *const Loopback { 
           self.loopback_mut()
    }

    #[doc="Read the LOOPBACK register."]
    #[inline] pub fn loopback(&self) -> Loopback { 
        unsafe {
            read_volatile(self.loopback_ptr())
        }
    }

    #[doc="Write the LOOPBACK register."]
    #[inline] pub fn set_loopback<F: FnOnce(Loopback) -> Loopback>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.loopback_mut(), f(Loopback(0)));
        }
        self
    }

    #[doc="Modify the LOOPBACK register."]
    #[inline] pub fn with_loopback<F: FnOnce(Loopback) -> Loopback>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.loopback_mut(), f(self.loopback()));
        }
        self
    }

    #[doc="Get the *mut pointer for the GPIOOBS register."]
    #[inline] pub fn gpioobs_mut(&self) -> *mut Gpioobs { 
        (self.0 + 0xdc) as *mut Gpioobs
    }

    #[doc="Get the *const pointer for the GPIOOBS register."]
    #[inline] pub fn gpioobs_ptr(&self) -> *const Gpioobs { 
           self.gpioobs_mut()
    }

    #[doc="Read the GPIOOBS register."]
    #[inline] pub fn gpioobs(&self) -> Gpioobs { 
        unsafe {
            read_volatile(self.gpioobs_ptr())
        }
    }

    #[doc="Write the GPIOOBS register."]
    #[inline] pub fn set_gpioobs<F: FnOnce(Gpioobs) -> Gpioobs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.gpioobs_mut(), f(Gpioobs(0)));
        }
        self
    }

    #[doc="Modify the GPIOOBS register."]
    #[inline] pub fn with_gpioobs<F: FnOnce(Gpioobs) -> Gpioobs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.gpioobs_mut(), f(self.gpioobs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGA register."]
    #[inline] pub fn altpadcfga_mut(&self) -> *mut Altpadcfga { 
        (self.0 + 0xe0) as *mut Altpadcfga
    }

    #[doc="Get the *const pointer for the ALTPADCFGA register."]
    #[inline] pub fn altpadcfga_ptr(&self) -> *const Altpadcfga { 
           self.altpadcfga_mut()
    }

    #[doc="Read the ALTPADCFGA register."]
    #[inline] pub fn altpadcfga(&self) -> Altpadcfga { 
        unsafe {
            read_volatile(self.altpadcfga_ptr())
        }
    }

    #[doc="Write the ALTPADCFGA register."]
    #[inline] pub fn set_altpadcfga<F: FnOnce(Altpadcfga) -> Altpadcfga>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfga_mut(), f(Altpadcfga(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGA register."]
    #[inline] pub fn with_altpadcfga<F: FnOnce(Altpadcfga) -> Altpadcfga>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfga_mut(), f(self.altpadcfga()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGB register."]
    #[inline] pub fn altpadcfgb_mut(&self) -> *mut Altpadcfgb { 
        (self.0 + 0xe4) as *mut Altpadcfgb
    }

    #[doc="Get the *const pointer for the ALTPADCFGB register."]
    #[inline] pub fn altpadcfgb_ptr(&self) -> *const Altpadcfgb { 
           self.altpadcfgb_mut()
    }

    #[doc="Read the ALTPADCFGB register."]
    #[inline] pub fn altpadcfgb(&self) -> Altpadcfgb { 
        unsafe {
            read_volatile(self.altpadcfgb_ptr())
        }
    }

    #[doc="Write the ALTPADCFGB register."]
    #[inline] pub fn set_altpadcfgb<F: FnOnce(Altpadcfgb) -> Altpadcfgb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgb_mut(), f(Altpadcfgb(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGB register."]
    #[inline] pub fn with_altpadcfgb<F: FnOnce(Altpadcfgb) -> Altpadcfgb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgb_mut(), f(self.altpadcfgb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGC register."]
    #[inline] pub fn altpadcfgc_mut(&self) -> *mut Altpadcfgc { 
        (self.0 + 0xe8) as *mut Altpadcfgc
    }

    #[doc="Get the *const pointer for the ALTPADCFGC register."]
    #[inline] pub fn altpadcfgc_ptr(&self) -> *const Altpadcfgc { 
           self.altpadcfgc_mut()
    }

    #[doc="Read the ALTPADCFGC register."]
    #[inline] pub fn altpadcfgc(&self) -> Altpadcfgc { 
        unsafe {
            read_volatile(self.altpadcfgc_ptr())
        }
    }

    #[doc="Write the ALTPADCFGC register."]
    #[inline] pub fn set_altpadcfgc<F: FnOnce(Altpadcfgc) -> Altpadcfgc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgc_mut(), f(Altpadcfgc(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGC register."]
    #[inline] pub fn with_altpadcfgc<F: FnOnce(Altpadcfgc) -> Altpadcfgc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgc_mut(), f(self.altpadcfgc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGD register."]
    #[inline] pub fn altpadcfgd_mut(&self) -> *mut Altpadcfgd { 
        (self.0 + 0xec) as *mut Altpadcfgd
    }

    #[doc="Get the *const pointer for the ALTPADCFGD register."]
    #[inline] pub fn altpadcfgd_ptr(&self) -> *const Altpadcfgd { 
           self.altpadcfgd_mut()
    }

    #[doc="Read the ALTPADCFGD register."]
    #[inline] pub fn altpadcfgd(&self) -> Altpadcfgd { 
        unsafe {
            read_volatile(self.altpadcfgd_ptr())
        }
    }

    #[doc="Write the ALTPADCFGD register."]
    #[inline] pub fn set_altpadcfgd<F: FnOnce(Altpadcfgd) -> Altpadcfgd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgd_mut(), f(Altpadcfgd(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGD register."]
    #[inline] pub fn with_altpadcfgd<F: FnOnce(Altpadcfgd) -> Altpadcfgd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgd_mut(), f(self.altpadcfgd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGE register."]
    #[inline] pub fn altpadcfge_mut(&self) -> *mut Altpadcfge { 
        (self.0 + 0xf0) as *mut Altpadcfge
    }

    #[doc="Get the *const pointer for the ALTPADCFGE register."]
    #[inline] pub fn altpadcfge_ptr(&self) -> *const Altpadcfge { 
           self.altpadcfge_mut()
    }

    #[doc="Read the ALTPADCFGE register."]
    #[inline] pub fn altpadcfge(&self) -> Altpadcfge { 
        unsafe {
            read_volatile(self.altpadcfge_ptr())
        }
    }

    #[doc="Write the ALTPADCFGE register."]
    #[inline] pub fn set_altpadcfge<F: FnOnce(Altpadcfge) -> Altpadcfge>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfge_mut(), f(Altpadcfge(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGE register."]
    #[inline] pub fn with_altpadcfge<F: FnOnce(Altpadcfge) -> Altpadcfge>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfge_mut(), f(self.altpadcfge()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGF register."]
    #[inline] pub fn altpadcfgf_mut(&self) -> *mut Altpadcfgf { 
        (self.0 + 0xf4) as *mut Altpadcfgf
    }

    #[doc="Get the *const pointer for the ALTPADCFGF register."]
    #[inline] pub fn altpadcfgf_ptr(&self) -> *const Altpadcfgf { 
           self.altpadcfgf_mut()
    }

    #[doc="Read the ALTPADCFGF register."]
    #[inline] pub fn altpadcfgf(&self) -> Altpadcfgf { 
        unsafe {
            read_volatile(self.altpadcfgf_ptr())
        }
    }

    #[doc="Write the ALTPADCFGF register."]
    #[inline] pub fn set_altpadcfgf<F: FnOnce(Altpadcfgf) -> Altpadcfgf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgf_mut(), f(Altpadcfgf(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGF register."]
    #[inline] pub fn with_altpadcfgf<F: FnOnce(Altpadcfgf) -> Altpadcfgf>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgf_mut(), f(self.altpadcfgf()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGG register."]
    #[inline] pub fn altpadcfgg_mut(&self) -> *mut Altpadcfgg { 
        (self.0 + 0xf8) as *mut Altpadcfgg
    }

    #[doc="Get the *const pointer for the ALTPADCFGG register."]
    #[inline] pub fn altpadcfgg_ptr(&self) -> *const Altpadcfgg { 
           self.altpadcfgg_mut()
    }

    #[doc="Read the ALTPADCFGG register."]
    #[inline] pub fn altpadcfgg(&self) -> Altpadcfgg { 
        unsafe {
            read_volatile(self.altpadcfgg_ptr())
        }
    }

    #[doc="Write the ALTPADCFGG register."]
    #[inline] pub fn set_altpadcfgg<F: FnOnce(Altpadcfgg) -> Altpadcfgg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgg_mut(), f(Altpadcfgg(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGG register."]
    #[inline] pub fn with_altpadcfgg<F: FnOnce(Altpadcfgg) -> Altpadcfgg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgg_mut(), f(self.altpadcfgg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGH register."]
    #[inline] pub fn altpadcfgh_mut(&self) -> *mut Altpadcfgh { 
        (self.0 + 0xfc) as *mut Altpadcfgh
    }

    #[doc="Get the *const pointer for the ALTPADCFGH register."]
    #[inline] pub fn altpadcfgh_ptr(&self) -> *const Altpadcfgh { 
           self.altpadcfgh_mut()
    }

    #[doc="Read the ALTPADCFGH register."]
    #[inline] pub fn altpadcfgh(&self) -> Altpadcfgh { 
        unsafe {
            read_volatile(self.altpadcfgh_ptr())
        }
    }

    #[doc="Write the ALTPADCFGH register."]
    #[inline] pub fn set_altpadcfgh<F: FnOnce(Altpadcfgh) -> Altpadcfgh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgh_mut(), f(Altpadcfgh(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGH register."]
    #[inline] pub fn with_altpadcfgh<F: FnOnce(Altpadcfgh) -> Altpadcfgh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgh_mut(), f(self.altpadcfgh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGI register."]
    #[inline] pub fn altpadcfgi_mut(&self) -> *mut Altpadcfgi { 
        (self.0 + 0x100) as *mut Altpadcfgi
    }

    #[doc="Get the *const pointer for the ALTPADCFGI register."]
    #[inline] pub fn altpadcfgi_ptr(&self) -> *const Altpadcfgi { 
           self.altpadcfgi_mut()
    }

    #[doc="Read the ALTPADCFGI register."]
    #[inline] pub fn altpadcfgi(&self) -> Altpadcfgi { 
        unsafe {
            read_volatile(self.altpadcfgi_ptr())
        }
    }

    #[doc="Write the ALTPADCFGI register."]
    #[inline] pub fn set_altpadcfgi<F: FnOnce(Altpadcfgi) -> Altpadcfgi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgi_mut(), f(Altpadcfgi(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGI register."]
    #[inline] pub fn with_altpadcfgi<F: FnOnce(Altpadcfgi) -> Altpadcfgi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgi_mut(), f(self.altpadcfgi()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGJ register."]
    #[inline] pub fn altpadcfgj_mut(&self) -> *mut Altpadcfgj { 
        (self.0 + 0x104) as *mut Altpadcfgj
    }

    #[doc="Get the *const pointer for the ALTPADCFGJ register."]
    #[inline] pub fn altpadcfgj_ptr(&self) -> *const Altpadcfgj { 
           self.altpadcfgj_mut()
    }

    #[doc="Read the ALTPADCFGJ register."]
    #[inline] pub fn altpadcfgj(&self) -> Altpadcfgj { 
        unsafe {
            read_volatile(self.altpadcfgj_ptr())
        }
    }

    #[doc="Write the ALTPADCFGJ register."]
    #[inline] pub fn set_altpadcfgj<F: FnOnce(Altpadcfgj) -> Altpadcfgj>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgj_mut(), f(Altpadcfgj(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGJ register."]
    #[inline] pub fn with_altpadcfgj<F: FnOnce(Altpadcfgj) -> Altpadcfgj>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgj_mut(), f(self.altpadcfgj()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGK register."]
    #[inline] pub fn altpadcfgk_mut(&self) -> *mut Altpadcfgk { 
        (self.0 + 0x108) as *mut Altpadcfgk
    }

    #[doc="Get the *const pointer for the ALTPADCFGK register."]
    #[inline] pub fn altpadcfgk_ptr(&self) -> *const Altpadcfgk { 
           self.altpadcfgk_mut()
    }

    #[doc="Read the ALTPADCFGK register."]
    #[inline] pub fn altpadcfgk(&self) -> Altpadcfgk { 
        unsafe {
            read_volatile(self.altpadcfgk_ptr())
        }
    }

    #[doc="Write the ALTPADCFGK register."]
    #[inline] pub fn set_altpadcfgk<F: FnOnce(Altpadcfgk) -> Altpadcfgk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgk_mut(), f(Altpadcfgk(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGK register."]
    #[inline] pub fn with_altpadcfgk<F: FnOnce(Altpadcfgk) -> Altpadcfgk>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgk_mut(), f(self.altpadcfgk()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGL register."]
    #[inline] pub fn altpadcfgl_mut(&self) -> *mut Altpadcfgl { 
        (self.0 + 0x10c) as *mut Altpadcfgl
    }

    #[doc="Get the *const pointer for the ALTPADCFGL register."]
    #[inline] pub fn altpadcfgl_ptr(&self) -> *const Altpadcfgl { 
           self.altpadcfgl_mut()
    }

    #[doc="Read the ALTPADCFGL register."]
    #[inline] pub fn altpadcfgl(&self) -> Altpadcfgl { 
        unsafe {
            read_volatile(self.altpadcfgl_ptr())
        }
    }

    #[doc="Write the ALTPADCFGL register."]
    #[inline] pub fn set_altpadcfgl<F: FnOnce(Altpadcfgl) -> Altpadcfgl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgl_mut(), f(Altpadcfgl(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGL register."]
    #[inline] pub fn with_altpadcfgl<F: FnOnce(Altpadcfgl) -> Altpadcfgl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgl_mut(), f(self.altpadcfgl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ALTPADCFGM register."]
    #[inline] pub fn altpadcfgm_mut(&self) -> *mut Altpadcfgm { 
        (self.0 + 0x110) as *mut Altpadcfgm
    }

    #[doc="Get the *const pointer for the ALTPADCFGM register."]
    #[inline] pub fn altpadcfgm_ptr(&self) -> *const Altpadcfgm { 
           self.altpadcfgm_mut()
    }

    #[doc="Read the ALTPADCFGM register."]
    #[inline] pub fn altpadcfgm(&self) -> Altpadcfgm { 
        unsafe {
            read_volatile(self.altpadcfgm_ptr())
        }
    }

    #[doc="Write the ALTPADCFGM register."]
    #[inline] pub fn set_altpadcfgm<F: FnOnce(Altpadcfgm) -> Altpadcfgm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgm_mut(), f(Altpadcfgm(0)));
        }
        self
    }

    #[doc="Modify the ALTPADCFGM register."]
    #[inline] pub fn with_altpadcfgm<F: FnOnce(Altpadcfgm) -> Altpadcfgm>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.altpadcfgm_mut(), f(self.altpadcfgm()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTEN register."]
    #[inline] pub fn inten_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Inten { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x200 + (index << 4)) as *mut Inten
    }

    #[doc="Get the *const pointer for the INTEN register."]
    #[inline] pub fn inten_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Inten { 
           self.inten_mut(index)
    }

    #[doc="Read the INTEN register."]
    #[inline] pub fn inten<I: Into<bits::R2>>(&self, index: I) -> Inten { 
        unsafe {
            read_volatile(self.inten_ptr(index))
        }
    }

    #[doc="Write the INTEN register."]
    #[inline] pub fn set_inten<I: Into<bits::R2>, F: FnOnce(Inten) -> Inten>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(index), f(Inten(0)));
        }
        self
    }

    #[doc="Modify the INTEN register."]
    #[inline] pub fn with_inten<I: Into<bits::R2> + Copy, F: FnOnce(Inten) -> Inten>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(index), f(self.inten(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSTAT register."]
    #[inline] pub fn intstat_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Intstat { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x204 + (index << 4)) as *mut Intstat
    }

    #[doc="Get the *const pointer for the INTSTAT register."]
    #[inline] pub fn intstat_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Intstat { 
           self.intstat_mut(index)
    }

    #[doc="Read the INTSTAT register."]
    #[inline] pub fn intstat<I: Into<bits::R2>>(&self, index: I) -> Intstat { 
        unsafe {
            read_volatile(self.intstat_ptr(index))
        }
    }

    #[doc="Write the INTSTAT register."]
    #[inline] pub fn set_intstat<I: Into<bits::R2>, F: FnOnce(Intstat) -> Intstat>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(index), f(Intstat(0)));
        }
        self
    }

    #[doc="Modify the INTSTAT register."]
    #[inline] pub fn with_intstat<I: Into<bits::R2> + Copy, F: FnOnce(Intstat) -> Intstat>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(index), f(self.intstat(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTCLR register."]
    #[inline] pub fn intclr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Intclr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x208 + (index << 4)) as *mut Intclr
    }

    #[doc="Get the *const pointer for the INTCLR register."]
    #[inline] pub fn intclr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Intclr { 
           self.intclr_mut(index)
    }

    #[doc="Read the INTCLR register."]
    #[inline] pub fn intclr<I: Into<bits::R2>>(&self, index: I) -> Intclr { 
        unsafe {
            read_volatile(self.intclr_ptr(index))
        }
    }

    #[doc="Write the INTCLR register."]
    #[inline] pub fn set_intclr<I: Into<bits::R2>, F: FnOnce(Intclr) -> Intclr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(index), f(Intclr(0)));
        }
        self
    }

    #[doc="Modify the INTCLR register."]
    #[inline] pub fn with_intclr<I: Into<bits::R2> + Copy, F: FnOnce(Intclr) -> Intclr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(index), f(self.intclr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the INT0SET register."]
    #[inline] pub fn int0set_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Int0set { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x20c + (index << 4)) as *mut Int0set
    }

    #[doc="Get the *const pointer for the INT0SET register."]
    #[inline] pub fn int0set_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Int0set { 
           self.int0set_mut(index)
    }

    #[doc="Read the INT0SET register."]
    #[inline] pub fn int0set<I: Into<bits::R2>>(&self, index: I) -> Int0set { 
        unsafe {
            read_volatile(self.int0set_ptr(index))
        }
    }

    #[doc="Write the INT0SET register."]
    #[inline] pub fn set_int0set<I: Into<bits::R2>, F: FnOnce(Int0set) -> Int0set>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.int0set_mut(index), f(Int0set(0)));
        }
        self
    }

    #[doc="Modify the INT0SET register."]
    #[inline] pub fn with_int0set<I: Into<bits::R2> + Copy, F: FnOnce(Int0set) -> Int0set>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.int0set_mut(index), f(self.int0set(index)));
        }
        self
    }

}

#[doc="Pad Configuration Register n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padreg(pub u32);
impl Padreg {
    #[doc="Pad 0 function select"]
    #[inline] pub fn padfncsel<I: Into<bits::R4>>(&self, index: I) -> bits::U3 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 3 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PADFNCSEL != 0"]
    #[inline] pub fn test_padfncsel<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.padfncsel(index) != 0
    }

    #[doc="Sets the PADFNCSEL field."]
    #[inline] pub fn set_padfncsel<I: Into<bits::R4>, V: Into<bits::U3>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        let shift: usize = 3 + (index << 3);
        self.0 &= !(0x7 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Pad n drive strength"]
    #[inline] pub fn padstrng<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PADSTRNG != 0"]
    #[inline] pub fn test_padstrng<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.padstrng(index) != 0
    }

    #[doc="Sets the PADSTRNG field."]
    #[inline] pub fn set_padstrng<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Pad n input enable"]
    #[inline] pub fn padinpen<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PADINPEN != 0"]
    #[inline] pub fn test_padinpen<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.padinpen(index) != 0
    }

    #[doc="Sets the PADINPEN field."]
    #[inline] pub fn set_padinpen<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Pad n pullup enable"]
    #[inline] pub fn padpull<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PADPULL != 0"]
    #[inline] pub fn test_padpull<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.padpull(index) != 0
    }

    #[doc="Sets the PADPULL field."]
    #[inline] pub fn set_padpull<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Padreg {
    #[inline]
    fn from(other: u32) -> Self {
         Padreg(other)
    }
}

impl ::core::fmt::Display for Padreg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padreg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.padfncsel(0) != 0 { try!(write!(f, " padfncsel[0]=0x{:x}", self.padfncsel(0)))}
        if self.padfncsel(1) != 0 { try!(write!(f, " padfncsel[1]=0x{:x}", self.padfncsel(1)))}
        if self.padfncsel(2) != 0 { try!(write!(f, " padfncsel[2]=0x{:x}", self.padfncsel(2)))}
        if self.padfncsel(3) != 0 { try!(write!(f, " padfncsel[3]=0x{:x}", self.padfncsel(3)))}
        if self.padstrng(0) != 0 { try!(write!(f, " padstrng[0]"))}
        if self.padstrng(1) != 0 { try!(write!(f, " padstrng[1]"))}
        if self.padstrng(2) != 0 { try!(write!(f, " padstrng[2]"))}
        if self.padstrng(3) != 0 { try!(write!(f, " padstrng[3]"))}
        if self.padinpen(0) != 0 { try!(write!(f, " padinpen[0]"))}
        if self.padinpen(1) != 0 { try!(write!(f, " padinpen[1]"))}
        if self.padinpen(2) != 0 { try!(write!(f, " padinpen[2]"))}
        if self.padinpen(3) != 0 { try!(write!(f, " padinpen[3]"))}
        if self.padpull(0) != 0 { try!(write!(f, " padpull[0]"))}
        if self.padpull(1) != 0 { try!(write!(f, " padpull[1]"))}
        if self.padpull(2) != 0 { try!(write!(f, " padpull[2]"))}
        if self.padpull(3) != 0 { try!(write!(f, " padpull[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Configuration Register n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc="GPIOn interrupt direction."]
    #[inline] pub fn gpiointd<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 3 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GPIOINTD != 0"]
    #[inline] pub fn test_gpiointd<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.gpiointd(index) != 0
    }

    #[doc="Sets the GPIOINTD field."]
    #[inline] pub fn set_gpiointd<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 3 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPIOn output configuration."]
    #[inline] pub fn gpiooutcfg<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if GPIOOUTCFG != 0"]
    #[inline] pub fn test_gpiooutcfg<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.gpiooutcfg(index) != 0
    }

    #[doc="Sets the GPIOOUTCFG field."]
    #[inline] pub fn set_gpiooutcfg<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 2);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPIOn input enable."]
    #[inline] pub fn gpioincfg<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIOINCFG != 0"]
    #[inline] pub fn test_gpioincfg<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.gpioincfg(index) != 0
    }

    #[doc="Sets the GPIOINCFG field."]
    #[inline] pub fn set_gpioincfg<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Cfg(other)
    }
}

impl ::core::fmt::Display for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpiointd(0) != 0 { try!(write!(f, " gpiointd[0]"))}
        if self.gpiointd(1) != 0 { try!(write!(f, " gpiointd[1]"))}
        if self.gpiointd(2) != 0 { try!(write!(f, " gpiointd[2]"))}
        if self.gpiointd(3) != 0 { try!(write!(f, " gpiointd[3]"))}
        if self.gpiointd(4) != 0 { try!(write!(f, " gpiointd[4]"))}
        if self.gpiointd(5) != 0 { try!(write!(f, " gpiointd[5]"))}
        if self.gpiointd(6) != 0 { try!(write!(f, " gpiointd[6]"))}
        if self.gpiointd(7) != 0 { try!(write!(f, " gpiointd[7]"))}
        if self.gpiooutcfg(0) != 0 { try!(write!(f, " gpiooutcfg[0]=0x{:x}", self.gpiooutcfg(0)))}
        if self.gpiooutcfg(1) != 0 { try!(write!(f, " gpiooutcfg[1]=0x{:x}", self.gpiooutcfg(1)))}
        if self.gpiooutcfg(2) != 0 { try!(write!(f, " gpiooutcfg[2]=0x{:x}", self.gpiooutcfg(2)))}
        if self.gpiooutcfg(3) != 0 { try!(write!(f, " gpiooutcfg[3]=0x{:x}", self.gpiooutcfg(3)))}
        if self.gpiooutcfg(4) != 0 { try!(write!(f, " gpiooutcfg[4]=0x{:x}", self.gpiooutcfg(4)))}
        if self.gpiooutcfg(5) != 0 { try!(write!(f, " gpiooutcfg[5]=0x{:x}", self.gpiooutcfg(5)))}
        if self.gpiooutcfg(6) != 0 { try!(write!(f, " gpiooutcfg[6]=0x{:x}", self.gpiooutcfg(6)))}
        if self.gpiooutcfg(7) != 0 { try!(write!(f, " gpiooutcfg[7]=0x{:x}", self.gpiooutcfg(7)))}
        if self.gpioincfg(0) != 0 { try!(write!(f, " gpioincfg[0]"))}
        if self.gpioincfg(1) != 0 { try!(write!(f, " gpioincfg[1]"))}
        if self.gpioincfg(2) != 0 { try!(write!(f, " gpioincfg[2]"))}
        if self.gpioincfg(3) != 0 { try!(write!(f, " gpioincfg[3]"))}
        if self.gpioincfg(4) != 0 { try!(write!(f, " gpioincfg[4]"))}
        if self.gpioincfg(5) != 0 { try!(write!(f, " gpioincfg[5]"))}
        if self.gpioincfg(6) != 0 { try!(write!(f, " gpioincfg[6]"))}
        if self.gpioincfg(7) != 0 { try!(write!(f, " gpioincfg[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Key Register for all pad configuration registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padkey(pub u32);
impl Padkey {
    #[doc="Key register value."]
    #[inline] pub fn padkey(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PADKEY != 0"]
    #[inline] pub fn test_padkey(&self) -> bool {
        self.padkey() != 0
    }

    #[doc="Sets the PADKEY field."]
    #[inline] pub fn set_padkey<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Padkey {
    #[inline]
    fn from(other: u32) -> Self {
         Padkey(other)
    }
}

impl ::core::fmt::Display for Padkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padkey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Input Register n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rd(pub u32);
impl Rd {
    #[doc="GPIO31-0 read data."]
    #[inline] pub fn rd<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RD != 0"]
    #[inline] pub fn test_rd<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.rd(index) != 0
    }

    #[doc="Sets the RD field."]
    #[inline] pub fn set_rd<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Rd {
    #[inline]
    fn from(other: u32) -> Self {
         Rd(other)
    }
}

impl ::core::fmt::Display for Rd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rd(0) != 0 { try!(write!(f, " rd[0]"))}
        if self.rd(1) != 0 { try!(write!(f, " rd[1]"))}
        if self.rd(2) != 0 { try!(write!(f, " rd[2]"))}
        if self.rd(3) != 0 { try!(write!(f, " rd[3]"))}
        if self.rd(4) != 0 { try!(write!(f, " rd[4]"))}
        if self.rd(5) != 0 { try!(write!(f, " rd[5]"))}
        if self.rd(6) != 0 { try!(write!(f, " rd[6]"))}
        if self.rd(7) != 0 { try!(write!(f, " rd[7]"))}
        if self.rd(8) != 0 { try!(write!(f, " rd[8]"))}
        if self.rd(9) != 0 { try!(write!(f, " rd[9]"))}
        if self.rd(10) != 0 { try!(write!(f, " rd[10]"))}
        if self.rd(11) != 0 { try!(write!(f, " rd[11]"))}
        if self.rd(12) != 0 { try!(write!(f, " rd[12]"))}
        if self.rd(13) != 0 { try!(write!(f, " rd[13]"))}
        if self.rd(14) != 0 { try!(write!(f, " rd[14]"))}
        if self.rd(15) != 0 { try!(write!(f, " rd[15]"))}
        if self.rd(16) != 0 { try!(write!(f, " rd[16]"))}
        if self.rd(17) != 0 { try!(write!(f, " rd[17]"))}
        if self.rd(18) != 0 { try!(write!(f, " rd[18]"))}
        if self.rd(19) != 0 { try!(write!(f, " rd[19]"))}
        if self.rd(20) != 0 { try!(write!(f, " rd[20]"))}
        if self.rd(21) != 0 { try!(write!(f, " rd[21]"))}
        if self.rd(22) != 0 { try!(write!(f, " rd[22]"))}
        if self.rd(23) != 0 { try!(write!(f, " rd[23]"))}
        if self.rd(24) != 0 { try!(write!(f, " rd[24]"))}
        if self.rd(25) != 0 { try!(write!(f, " rd[25]"))}
        if self.rd(26) != 0 { try!(write!(f, " rd[26]"))}
        if self.rd(27) != 0 { try!(write!(f, " rd[27]"))}
        if self.rd(28) != 0 { try!(write!(f, " rd[28]"))}
        if self.rd(29) != 0 { try!(write!(f, " rd[29]"))}
        if self.rd(30) != 0 { try!(write!(f, " rd[30]"))}
        if self.rd(31) != 0 { try!(write!(f, " rd[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Output Register n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wt(pub u32);
impl Wt {
    #[doc="GPIO31-0 write data."]
    #[inline] pub fn wt<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WT != 0"]
    #[inline] pub fn test_wt<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.wt(index) != 0
    }

    #[doc="Sets the WT field."]
    #[inline] pub fn set_wt<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Wt {
    #[inline]
    fn from(other: u32) -> Self {
         Wt(other)
    }
}

impl ::core::fmt::Display for Wt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wt(0) != 0 { try!(write!(f, " wt[0]"))}
        if self.wt(1) != 0 { try!(write!(f, " wt[1]"))}
        if self.wt(2) != 0 { try!(write!(f, " wt[2]"))}
        if self.wt(3) != 0 { try!(write!(f, " wt[3]"))}
        if self.wt(4) != 0 { try!(write!(f, " wt[4]"))}
        if self.wt(5) != 0 { try!(write!(f, " wt[5]"))}
        if self.wt(6) != 0 { try!(write!(f, " wt[6]"))}
        if self.wt(7) != 0 { try!(write!(f, " wt[7]"))}
        if self.wt(8) != 0 { try!(write!(f, " wt[8]"))}
        if self.wt(9) != 0 { try!(write!(f, " wt[9]"))}
        if self.wt(10) != 0 { try!(write!(f, " wt[10]"))}
        if self.wt(11) != 0 { try!(write!(f, " wt[11]"))}
        if self.wt(12) != 0 { try!(write!(f, " wt[12]"))}
        if self.wt(13) != 0 { try!(write!(f, " wt[13]"))}
        if self.wt(14) != 0 { try!(write!(f, " wt[14]"))}
        if self.wt(15) != 0 { try!(write!(f, " wt[15]"))}
        if self.wt(16) != 0 { try!(write!(f, " wt[16]"))}
        if self.wt(17) != 0 { try!(write!(f, " wt[17]"))}
        if self.wt(18) != 0 { try!(write!(f, " wt[18]"))}
        if self.wt(19) != 0 { try!(write!(f, " wt[19]"))}
        if self.wt(20) != 0 { try!(write!(f, " wt[20]"))}
        if self.wt(21) != 0 { try!(write!(f, " wt[21]"))}
        if self.wt(22) != 0 { try!(write!(f, " wt[22]"))}
        if self.wt(23) != 0 { try!(write!(f, " wt[23]"))}
        if self.wt(24) != 0 { try!(write!(f, " wt[24]"))}
        if self.wt(25) != 0 { try!(write!(f, " wt[25]"))}
        if self.wt(26) != 0 { try!(write!(f, " wt[26]"))}
        if self.wt(27) != 0 { try!(write!(f, " wt[27]"))}
        if self.wt(28) != 0 { try!(write!(f, " wt[28]"))}
        if self.wt(29) != 0 { try!(write!(f, " wt[29]"))}
        if self.wt(30) != 0 { try!(write!(f, " wt[30]"))}
        if self.wt(31) != 0 { try!(write!(f, " wt[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Output Register n Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wts(pub u32);
impl Wts {
    #[doc="Set the GPIO31-0 write data."]
    #[inline] pub fn wts<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WTS != 0"]
    #[inline] pub fn test_wts<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.wts(index) != 0
    }

    #[doc="Sets the WTS field."]
    #[inline] pub fn set_wts<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Wts {
    #[inline]
    fn from(other: u32) -> Self {
         Wts(other)
    }
}

impl ::core::fmt::Display for Wts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wts(0) != 0 { try!(write!(f, " wts[0]"))}
        if self.wts(1) != 0 { try!(write!(f, " wts[1]"))}
        if self.wts(2) != 0 { try!(write!(f, " wts[2]"))}
        if self.wts(3) != 0 { try!(write!(f, " wts[3]"))}
        if self.wts(4) != 0 { try!(write!(f, " wts[4]"))}
        if self.wts(5) != 0 { try!(write!(f, " wts[5]"))}
        if self.wts(6) != 0 { try!(write!(f, " wts[6]"))}
        if self.wts(7) != 0 { try!(write!(f, " wts[7]"))}
        if self.wts(8) != 0 { try!(write!(f, " wts[8]"))}
        if self.wts(9) != 0 { try!(write!(f, " wts[9]"))}
        if self.wts(10) != 0 { try!(write!(f, " wts[10]"))}
        if self.wts(11) != 0 { try!(write!(f, " wts[11]"))}
        if self.wts(12) != 0 { try!(write!(f, " wts[12]"))}
        if self.wts(13) != 0 { try!(write!(f, " wts[13]"))}
        if self.wts(14) != 0 { try!(write!(f, " wts[14]"))}
        if self.wts(15) != 0 { try!(write!(f, " wts[15]"))}
        if self.wts(16) != 0 { try!(write!(f, " wts[16]"))}
        if self.wts(17) != 0 { try!(write!(f, " wts[17]"))}
        if self.wts(18) != 0 { try!(write!(f, " wts[18]"))}
        if self.wts(19) != 0 { try!(write!(f, " wts[19]"))}
        if self.wts(20) != 0 { try!(write!(f, " wts[20]"))}
        if self.wts(21) != 0 { try!(write!(f, " wts[21]"))}
        if self.wts(22) != 0 { try!(write!(f, " wts[22]"))}
        if self.wts(23) != 0 { try!(write!(f, " wts[23]"))}
        if self.wts(24) != 0 { try!(write!(f, " wts[24]"))}
        if self.wts(25) != 0 { try!(write!(f, " wts[25]"))}
        if self.wts(26) != 0 { try!(write!(f, " wts[26]"))}
        if self.wts(27) != 0 { try!(write!(f, " wts[27]"))}
        if self.wts(28) != 0 { try!(write!(f, " wts[28]"))}
        if self.wts(29) != 0 { try!(write!(f, " wts[29]"))}
        if self.wts(30) != 0 { try!(write!(f, " wts[30]"))}
        if self.wts(31) != 0 { try!(write!(f, " wts[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Output Register n Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wtc(pub u32);
impl Wtc {
    #[doc="Clear the GPIO31-0 write data."]
    #[inline] pub fn wtc<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WTC != 0"]
    #[inline] pub fn test_wtc<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.wtc(index) != 0
    }

    #[doc="Sets the WTC field."]
    #[inline] pub fn set_wtc<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Wtc {
    #[inline]
    fn from(other: u32) -> Self {
         Wtc(other)
    }
}

impl ::core::fmt::Display for Wtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wtc(0) != 0 { try!(write!(f, " wtc[0]"))}
        if self.wtc(1) != 0 { try!(write!(f, " wtc[1]"))}
        if self.wtc(2) != 0 { try!(write!(f, " wtc[2]"))}
        if self.wtc(3) != 0 { try!(write!(f, " wtc[3]"))}
        if self.wtc(4) != 0 { try!(write!(f, " wtc[4]"))}
        if self.wtc(5) != 0 { try!(write!(f, " wtc[5]"))}
        if self.wtc(6) != 0 { try!(write!(f, " wtc[6]"))}
        if self.wtc(7) != 0 { try!(write!(f, " wtc[7]"))}
        if self.wtc(8) != 0 { try!(write!(f, " wtc[8]"))}
        if self.wtc(9) != 0 { try!(write!(f, " wtc[9]"))}
        if self.wtc(10) != 0 { try!(write!(f, " wtc[10]"))}
        if self.wtc(11) != 0 { try!(write!(f, " wtc[11]"))}
        if self.wtc(12) != 0 { try!(write!(f, " wtc[12]"))}
        if self.wtc(13) != 0 { try!(write!(f, " wtc[13]"))}
        if self.wtc(14) != 0 { try!(write!(f, " wtc[14]"))}
        if self.wtc(15) != 0 { try!(write!(f, " wtc[15]"))}
        if self.wtc(16) != 0 { try!(write!(f, " wtc[16]"))}
        if self.wtc(17) != 0 { try!(write!(f, " wtc[17]"))}
        if self.wtc(18) != 0 { try!(write!(f, " wtc[18]"))}
        if self.wtc(19) != 0 { try!(write!(f, " wtc[19]"))}
        if self.wtc(20) != 0 { try!(write!(f, " wtc[20]"))}
        if self.wtc(21) != 0 { try!(write!(f, " wtc[21]"))}
        if self.wtc(22) != 0 { try!(write!(f, " wtc[22]"))}
        if self.wtc(23) != 0 { try!(write!(f, " wtc[23]"))}
        if self.wtc(24) != 0 { try!(write!(f, " wtc[24]"))}
        if self.wtc(25) != 0 { try!(write!(f, " wtc[25]"))}
        if self.wtc(26) != 0 { try!(write!(f, " wtc[26]"))}
        if self.wtc(27) != 0 { try!(write!(f, " wtc[27]"))}
        if self.wtc(28) != 0 { try!(write!(f, " wtc[28]"))}
        if self.wtc(29) != 0 { try!(write!(f, " wtc[29]"))}
        if self.wtc(30) != 0 { try!(write!(f, " wtc[30]"))}
        if self.wtc(31) != 0 { try!(write!(f, " wtc[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Enable Register n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct En(pub u32);
impl En {
    #[doc="GPIO31-0 output enables"]
    #[inline] pub fn en<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.en(index) != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for En {
    #[inline]
    fn from(other: u32) -> Self {
         En(other)
    }
}

impl ::core::fmt::Display for En {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for En {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.en(0) != 0 { try!(write!(f, " en[0]"))}
        if self.en(1) != 0 { try!(write!(f, " en[1]"))}
        if self.en(2) != 0 { try!(write!(f, " en[2]"))}
        if self.en(3) != 0 { try!(write!(f, " en[3]"))}
        if self.en(4) != 0 { try!(write!(f, " en[4]"))}
        if self.en(5) != 0 { try!(write!(f, " en[5]"))}
        if self.en(6) != 0 { try!(write!(f, " en[6]"))}
        if self.en(7) != 0 { try!(write!(f, " en[7]"))}
        if self.en(8) != 0 { try!(write!(f, " en[8]"))}
        if self.en(9) != 0 { try!(write!(f, " en[9]"))}
        if self.en(10) != 0 { try!(write!(f, " en[10]"))}
        if self.en(11) != 0 { try!(write!(f, " en[11]"))}
        if self.en(12) != 0 { try!(write!(f, " en[12]"))}
        if self.en(13) != 0 { try!(write!(f, " en[13]"))}
        if self.en(14) != 0 { try!(write!(f, " en[14]"))}
        if self.en(15) != 0 { try!(write!(f, " en[15]"))}
        if self.en(16) != 0 { try!(write!(f, " en[16]"))}
        if self.en(17) != 0 { try!(write!(f, " en[17]"))}
        if self.en(18) != 0 { try!(write!(f, " en[18]"))}
        if self.en(19) != 0 { try!(write!(f, " en[19]"))}
        if self.en(20) != 0 { try!(write!(f, " en[20]"))}
        if self.en(21) != 0 { try!(write!(f, " en[21]"))}
        if self.en(22) != 0 { try!(write!(f, " en[22]"))}
        if self.en(23) != 0 { try!(write!(f, " en[23]"))}
        if self.en(24) != 0 { try!(write!(f, " en[24]"))}
        if self.en(25) != 0 { try!(write!(f, " en[25]"))}
        if self.en(26) != 0 { try!(write!(f, " en[26]"))}
        if self.en(27) != 0 { try!(write!(f, " en[27]"))}
        if self.en(28) != 0 { try!(write!(f, " en[28]"))}
        if self.en(29) != 0 { try!(write!(f, " en[29]"))}
        if self.en(30) != 0 { try!(write!(f, " en[30]"))}
        if self.en(31) != 0 { try!(write!(f, " en[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Enable Register n Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ens(pub u32);
impl Ens {
    #[doc="Set the GPIO31-0 output enables"]
    #[inline] pub fn ens<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENS != 0"]
    #[inline] pub fn test_ens<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.ens(index) != 0
    }

    #[doc="Sets the ENS field."]
    #[inline] pub fn set_ens<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ens {
    #[inline]
    fn from(other: u32) -> Self {
         Ens(other)
    }
}

impl ::core::fmt::Display for Ens {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ens {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ens(0) != 0 { try!(write!(f, " ens[0]"))}
        if self.ens(1) != 0 { try!(write!(f, " ens[1]"))}
        if self.ens(2) != 0 { try!(write!(f, " ens[2]"))}
        if self.ens(3) != 0 { try!(write!(f, " ens[3]"))}
        if self.ens(4) != 0 { try!(write!(f, " ens[4]"))}
        if self.ens(5) != 0 { try!(write!(f, " ens[5]"))}
        if self.ens(6) != 0 { try!(write!(f, " ens[6]"))}
        if self.ens(7) != 0 { try!(write!(f, " ens[7]"))}
        if self.ens(8) != 0 { try!(write!(f, " ens[8]"))}
        if self.ens(9) != 0 { try!(write!(f, " ens[9]"))}
        if self.ens(10) != 0 { try!(write!(f, " ens[10]"))}
        if self.ens(11) != 0 { try!(write!(f, " ens[11]"))}
        if self.ens(12) != 0 { try!(write!(f, " ens[12]"))}
        if self.ens(13) != 0 { try!(write!(f, " ens[13]"))}
        if self.ens(14) != 0 { try!(write!(f, " ens[14]"))}
        if self.ens(15) != 0 { try!(write!(f, " ens[15]"))}
        if self.ens(16) != 0 { try!(write!(f, " ens[16]"))}
        if self.ens(17) != 0 { try!(write!(f, " ens[17]"))}
        if self.ens(18) != 0 { try!(write!(f, " ens[18]"))}
        if self.ens(19) != 0 { try!(write!(f, " ens[19]"))}
        if self.ens(20) != 0 { try!(write!(f, " ens[20]"))}
        if self.ens(21) != 0 { try!(write!(f, " ens[21]"))}
        if self.ens(22) != 0 { try!(write!(f, " ens[22]"))}
        if self.ens(23) != 0 { try!(write!(f, " ens[23]"))}
        if self.ens(24) != 0 { try!(write!(f, " ens[24]"))}
        if self.ens(25) != 0 { try!(write!(f, " ens[25]"))}
        if self.ens(26) != 0 { try!(write!(f, " ens[26]"))}
        if self.ens(27) != 0 { try!(write!(f, " ens[27]"))}
        if self.ens(28) != 0 { try!(write!(f, " ens[28]"))}
        if self.ens(29) != 0 { try!(write!(f, " ens[29]"))}
        if self.ens(30) != 0 { try!(write!(f, " ens[30]"))}
        if self.ens(31) != 0 { try!(write!(f, " ens[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Enable Register n Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Enc(pub u32);
impl Enc {
    #[doc="Clear the GPIO31-0 output enables"]
    #[inline] pub fn enc<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENC != 0"]
    #[inline] pub fn test_enc<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.enc(index) != 0
    }

    #[doc="Sets the ENC field."]
    #[inline] pub fn set_enc<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Enc {
    #[inline]
    fn from(other: u32) -> Self {
         Enc(other)
    }
}

impl ::core::fmt::Display for Enc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Enc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enc(0) != 0 { try!(write!(f, " enc[0]"))}
        if self.enc(1) != 0 { try!(write!(f, " enc[1]"))}
        if self.enc(2) != 0 { try!(write!(f, " enc[2]"))}
        if self.enc(3) != 0 { try!(write!(f, " enc[3]"))}
        if self.enc(4) != 0 { try!(write!(f, " enc[4]"))}
        if self.enc(5) != 0 { try!(write!(f, " enc[5]"))}
        if self.enc(6) != 0 { try!(write!(f, " enc[6]"))}
        if self.enc(7) != 0 { try!(write!(f, " enc[7]"))}
        if self.enc(8) != 0 { try!(write!(f, " enc[8]"))}
        if self.enc(9) != 0 { try!(write!(f, " enc[9]"))}
        if self.enc(10) != 0 { try!(write!(f, " enc[10]"))}
        if self.enc(11) != 0 { try!(write!(f, " enc[11]"))}
        if self.enc(12) != 0 { try!(write!(f, " enc[12]"))}
        if self.enc(13) != 0 { try!(write!(f, " enc[13]"))}
        if self.enc(14) != 0 { try!(write!(f, " enc[14]"))}
        if self.enc(15) != 0 { try!(write!(f, " enc[15]"))}
        if self.enc(16) != 0 { try!(write!(f, " enc[16]"))}
        if self.enc(17) != 0 { try!(write!(f, " enc[17]"))}
        if self.enc(18) != 0 { try!(write!(f, " enc[18]"))}
        if self.enc(19) != 0 { try!(write!(f, " enc[19]"))}
        if self.enc(20) != 0 { try!(write!(f, " enc[20]"))}
        if self.enc(21) != 0 { try!(write!(f, " enc[21]"))}
        if self.enc(22) != 0 { try!(write!(f, " enc[22]"))}
        if self.enc(23) != 0 { try!(write!(f, " enc[23]"))}
        if self.enc(24) != 0 { try!(write!(f, " enc[24]"))}
        if self.enc(25) != 0 { try!(write!(f, " enc[25]"))}
        if self.enc(26) != 0 { try!(write!(f, " enc[26]"))}
        if self.enc(27) != 0 { try!(write!(f, " enc[27]"))}
        if self.enc(28) != 0 { try!(write!(f, " enc[28]"))}
        if self.enc(29) != 0 { try!(write!(f, " enc[29]"))}
        if self.enc(30) != 0 { try!(write!(f, " enc[30]"))}
        if self.enc(31) != 0 { try!(write!(f, " enc[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Capture Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmrcap(pub u32);
impl Stmrcap {
    #[doc="STIMER Capture n Polarity."]
    #[inline] pub fn stpol<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 6 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STPOL != 0"]
    #[inline] pub fn test_stpol<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.stpol(index) != 0
    }

    #[doc="Sets the STPOL field."]
    #[inline] pub fn set_stpol<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 6 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="STIMER Capture n Select."]
    #[inline] pub fn stsel<I: Into<bits::R4>>(&self, index: I) -> bits::U6 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if STSEL != 0"]
    #[inline] pub fn test_stsel<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.stsel(index) != 0
    }

    #[doc="Sets the STSEL field."]
    #[inline] pub fn set_stsel<I: Into<bits::R4>, V: Into<bits::U6>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0x3f << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Stmrcap {
    #[inline]
    fn from(other: u32) -> Self {
         Stmrcap(other)
    }
}

impl ::core::fmt::Display for Stmrcap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stmrcap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stpol(0) != 0 { try!(write!(f, " stpol[0]"))}
        if self.stpol(1) != 0 { try!(write!(f, " stpol[1]"))}
        if self.stpol(2) != 0 { try!(write!(f, " stpol[2]"))}
        if self.stpol(3) != 0 { try!(write!(f, " stpol[3]"))}
        if self.stsel(0) != 0 { try!(write!(f, " stsel[0]=0x{:x}", self.stsel(0)))}
        if self.stsel(1) != 0 { try!(write!(f, " stsel[1]=0x{:x}", self.stsel(1)))}
        if self.stsel(2) != 0 { try!(write!(f, " stsel[2]=0x{:x}", self.stsel(2)))}
        if self.stsel(3) != 0 { try!(write!(f, " stsel[3]=0x{:x}", self.stsel(3)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IOMn Flow Control IRQ Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iomirq(pub u32);
impl Iomirq {
    #[doc="IOMSTR IRQ pad select."]
    #[inline] pub fn iomirq(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if IOMIRQ != 0"]
    #[inline] pub fn test_iomirq(&self) -> bool {
        self.iomirq() != 0
    }

    #[doc="Sets the IOMIRQ field."]
    #[inline] pub fn set_iomirq<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Iomirq {
    #[inline]
    fn from(other: u32) -> Self {
         Iomirq(other)
    }
}

impl ::core::fmt::Display for Iomirq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iomirq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iomirq() != 0 { try!(write!(f, " iomirq=0x{:x}", self.iomirq()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IOM to IOS Loopback Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Loopback(pub u32);
impl Loopback {
    #[doc="IOM to IOS loopback control."]
    #[inline] pub fn loopback(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LOOPBACK != 0"]
    #[inline] pub fn test_loopback(&self) -> bool {
        self.loopback() != 0
    }

    #[doc="Sets the LOOPBACK field."]
    #[inline] pub fn set_loopback<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Loopback {
    #[inline]
    fn from(other: u32) -> Self {
         Loopback(other)
    }
}

impl ::core::fmt::Display for Loopback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Loopback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.loopback() != 0 { try!(write!(f, " loopback=0x{:x}", self.loopback()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Observation Mode Sample register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpioobs(pub u32);
impl Gpioobs {
    #[doc="Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only"]
    #[inline] pub fn obs_data(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if OBS_DATA != 0"]
    #[inline] pub fn test_obs_data(&self) -> bool {
        self.obs_data() != 0
    }

    #[doc="Sets the OBS_DATA field."]
    #[inline] pub fn set_obs_data<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Gpioobs {
    #[inline]
    fn from(other: u32) -> Self {
         Gpioobs(other)
    }
}

impl ::core::fmt::Display for Gpioobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpioobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.obs_data() != 0 { try!(write!(f, " obs_data=0x{:x}", self.obs_data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg0 (Pads 3,2,1,0)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfga(pub u32);
impl Altpadcfga {
    #[doc="Pad 3 slew rate selection."]
    #[inline] pub fn pad3_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD3_SR != 0"]
    #[inline] pub fn test_pad3_sr(&self) -> bool {
        self.pad3_sr() != 0
    }

    #[doc="Sets the PAD3_SR field."]
    #[inline] pub fn set_pad3_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 3 high order drive strength selection. Used in conjunction with PAD3STRNG field to set the pad drive strength."]
    #[inline] pub fn pad3_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD3_DS1 != 0"]
    #[inline] pub fn test_pad3_ds1(&self) -> bool {
        self.pad3_ds1() != 0
    }

    #[doc="Sets the PAD3_DS1 field."]
    #[inline] pub fn set_pad3_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 2 slew rate selection."]
    #[inline] pub fn pad2_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD2_SR != 0"]
    #[inline] pub fn test_pad2_sr(&self) -> bool {
        self.pad2_sr() != 0
    }

    #[doc="Sets the PAD2_SR field."]
    #[inline] pub fn set_pad2_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 2 high order drive strength selection. Used in conjunction with PAD2STRNG field to set the pad drive strength."]
    #[inline] pub fn pad2_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD2_DS1 != 0"]
    #[inline] pub fn test_pad2_ds1(&self) -> bool {
        self.pad2_ds1() != 0
    }

    #[doc="Sets the PAD2_DS1 field."]
    #[inline] pub fn set_pad2_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 1 slew rate selection."]
    #[inline] pub fn pad1_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD1_SR != 0"]
    #[inline] pub fn test_pad1_sr(&self) -> bool {
        self.pad1_sr() != 0
    }

    #[doc="Sets the PAD1_SR field."]
    #[inline] pub fn set_pad1_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 1 high order drive strength selection. Used in conjunction with PAD1STRNG field to set the pad drive strength."]
    #[inline] pub fn pad1_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD1_DS1 != 0"]
    #[inline] pub fn test_pad1_ds1(&self) -> bool {
        self.pad1_ds1() != 0
    }

    #[doc="Sets the PAD1_DS1 field."]
    #[inline] pub fn set_pad1_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 0 slew rate selection."]
    #[inline] pub fn pad0_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD0_SR != 0"]
    #[inline] pub fn test_pad0_sr(&self) -> bool {
        self.pad0_sr() != 0
    }

    #[doc="Sets the PAD0_SR field."]
    #[inline] pub fn set_pad0_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 0 high order drive strength selection. Used in conjunction with PAD0STRNG field to set the pad drive strength."]
    #[inline] pub fn pad0_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD0_DS1 != 0"]
    #[inline] pub fn test_pad0_ds1(&self) -> bool {
        self.pad0_ds1() != 0
    }

    #[doc="Sets the PAD0_DS1 field."]
    #[inline] pub fn set_pad0_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfga {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfga(other)
    }
}

impl ::core::fmt::Display for Altpadcfga {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfga {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad3_sr() != 0 { try!(write!(f, " pad3_sr"))}
        if self.pad3_ds1() != 0 { try!(write!(f, " pad3_ds1"))}
        if self.pad2_sr() != 0 { try!(write!(f, " pad2_sr"))}
        if self.pad2_ds1() != 0 { try!(write!(f, " pad2_ds1"))}
        if self.pad1_sr() != 0 { try!(write!(f, " pad1_sr"))}
        if self.pad1_ds1() != 0 { try!(write!(f, " pad1_ds1"))}
        if self.pad0_sr() != 0 { try!(write!(f, " pad0_sr"))}
        if self.pad0_ds1() != 0 { try!(write!(f, " pad0_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg1 (Pads 7,6,5,4)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgb(pub u32);
impl Altpadcfgb {
    #[doc="Pad 7 slew rate selection."]
    #[inline] pub fn pad7_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD7_SR != 0"]
    #[inline] pub fn test_pad7_sr(&self) -> bool {
        self.pad7_sr() != 0
    }

    #[doc="Sets the PAD7_SR field."]
    #[inline] pub fn set_pad7_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 7 high order drive strength selection. Used in conjunction with PAD7STRNG field to set the pad drive strength."]
    #[inline] pub fn pad7_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD7_DS1 != 0"]
    #[inline] pub fn test_pad7_ds1(&self) -> bool {
        self.pad7_ds1() != 0
    }

    #[doc="Sets the PAD7_DS1 field."]
    #[inline] pub fn set_pad7_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 6 slew rate selection."]
    #[inline] pub fn pad6_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD6_SR != 0"]
    #[inline] pub fn test_pad6_sr(&self) -> bool {
        self.pad6_sr() != 0
    }

    #[doc="Sets the PAD6_SR field."]
    #[inline] pub fn set_pad6_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 6 high order drive strength selection. Used in conjunction with PAD6STRNG field to set the pad drive strength."]
    #[inline] pub fn pad6_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD6_DS1 != 0"]
    #[inline] pub fn test_pad6_ds1(&self) -> bool {
        self.pad6_ds1() != 0
    }

    #[doc="Sets the PAD6_DS1 field."]
    #[inline] pub fn set_pad6_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 5 slew rate selection."]
    #[inline] pub fn pad5_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD5_SR != 0"]
    #[inline] pub fn test_pad5_sr(&self) -> bool {
        self.pad5_sr() != 0
    }

    #[doc="Sets the PAD5_SR field."]
    #[inline] pub fn set_pad5_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 5 high order drive strength selection. Used in conjunction with PAD5STRNG field to set the pad drive strength."]
    #[inline] pub fn pad5_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD5_DS1 != 0"]
    #[inline] pub fn test_pad5_ds1(&self) -> bool {
        self.pad5_ds1() != 0
    }

    #[doc="Sets the PAD5_DS1 field."]
    #[inline] pub fn set_pad5_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 4 slew rate selection."]
    #[inline] pub fn pad4_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD4_SR != 0"]
    #[inline] pub fn test_pad4_sr(&self) -> bool {
        self.pad4_sr() != 0
    }

    #[doc="Sets the PAD4_SR field."]
    #[inline] pub fn set_pad4_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 4 high order drive strength selection. Used in conjunction with PAD4STRNG field to set the pad drive strength."]
    #[inline] pub fn pad4_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD4_DS1 != 0"]
    #[inline] pub fn test_pad4_ds1(&self) -> bool {
        self.pad4_ds1() != 0
    }

    #[doc="Sets the PAD4_DS1 field."]
    #[inline] pub fn set_pad4_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgb {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgb(other)
    }
}

impl ::core::fmt::Display for Altpadcfgb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad7_sr() != 0 { try!(write!(f, " pad7_sr"))}
        if self.pad7_ds1() != 0 { try!(write!(f, " pad7_ds1"))}
        if self.pad6_sr() != 0 { try!(write!(f, " pad6_sr"))}
        if self.pad6_ds1() != 0 { try!(write!(f, " pad6_ds1"))}
        if self.pad5_sr() != 0 { try!(write!(f, " pad5_sr"))}
        if self.pad5_ds1() != 0 { try!(write!(f, " pad5_ds1"))}
        if self.pad4_sr() != 0 { try!(write!(f, " pad4_sr"))}
        if self.pad4_ds1() != 0 { try!(write!(f, " pad4_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg2 (Pads 11,10,9,8)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgc(pub u32);
impl Altpadcfgc {
    #[doc="Pad 11 slew rate selection."]
    #[inline] pub fn pad11_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD11_SR != 0"]
    #[inline] pub fn test_pad11_sr(&self) -> bool {
        self.pad11_sr() != 0
    }

    #[doc="Sets the PAD11_SR field."]
    #[inline] pub fn set_pad11_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 11 high order drive strength selection. Used in conjunction with PAD11STRNG field to set the pad drive strength."]
    #[inline] pub fn pad11_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD11_DS1 != 0"]
    #[inline] pub fn test_pad11_ds1(&self) -> bool {
        self.pad11_ds1() != 0
    }

    #[doc="Sets the PAD11_DS1 field."]
    #[inline] pub fn set_pad11_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 10 slew rate selection."]
    #[inline] pub fn pad10_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD10_SR != 0"]
    #[inline] pub fn test_pad10_sr(&self) -> bool {
        self.pad10_sr() != 0
    }

    #[doc="Sets the PAD10_SR field."]
    #[inline] pub fn set_pad10_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 10 high order drive strength selection. Used in conjunction with PAD10STRNG field to set the pad drive strength."]
    #[inline] pub fn pad10_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD10_DS1 != 0"]
    #[inline] pub fn test_pad10_ds1(&self) -> bool {
        self.pad10_ds1() != 0
    }

    #[doc="Sets the PAD10_DS1 field."]
    #[inline] pub fn set_pad10_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 9 slew rate selection."]
    #[inline] pub fn pad9_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD9_SR != 0"]
    #[inline] pub fn test_pad9_sr(&self) -> bool {
        self.pad9_sr() != 0
    }

    #[doc="Sets the PAD9_SR field."]
    #[inline] pub fn set_pad9_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 9 high order drive strength selection. Used in conjunction with PAD9STRNG field to set the pad drive strength."]
    #[inline] pub fn pad9_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD9_DS1 != 0"]
    #[inline] pub fn test_pad9_ds1(&self) -> bool {
        self.pad9_ds1() != 0
    }

    #[doc="Sets the PAD9_DS1 field."]
    #[inline] pub fn set_pad9_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 8 slew rate selection."]
    #[inline] pub fn pad8_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD8_SR != 0"]
    #[inline] pub fn test_pad8_sr(&self) -> bool {
        self.pad8_sr() != 0
    }

    #[doc="Sets the PAD8_SR field."]
    #[inline] pub fn set_pad8_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 8 high order drive strength selection. Used in conjunction with PAD8STRNG field to set the pad drive strength."]
    #[inline] pub fn pad8_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD8_DS1 != 0"]
    #[inline] pub fn test_pad8_ds1(&self) -> bool {
        self.pad8_ds1() != 0
    }

    #[doc="Sets the PAD8_DS1 field."]
    #[inline] pub fn set_pad8_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgc {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgc(other)
    }
}

impl ::core::fmt::Display for Altpadcfgc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad11_sr() != 0 { try!(write!(f, " pad11_sr"))}
        if self.pad11_ds1() != 0 { try!(write!(f, " pad11_ds1"))}
        if self.pad10_sr() != 0 { try!(write!(f, " pad10_sr"))}
        if self.pad10_ds1() != 0 { try!(write!(f, " pad10_ds1"))}
        if self.pad9_sr() != 0 { try!(write!(f, " pad9_sr"))}
        if self.pad9_ds1() != 0 { try!(write!(f, " pad9_ds1"))}
        if self.pad8_sr() != 0 { try!(write!(f, " pad8_sr"))}
        if self.pad8_ds1() != 0 { try!(write!(f, " pad8_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg3 (Pads 15,14,13,12)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgd(pub u32);
impl Altpadcfgd {
    #[doc="Pad 15 slew rate selection."]
    #[inline] pub fn pad15_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD15_SR != 0"]
    #[inline] pub fn test_pad15_sr(&self) -> bool {
        self.pad15_sr() != 0
    }

    #[doc="Sets the PAD15_SR field."]
    #[inline] pub fn set_pad15_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
    #[inline] pub fn pad15_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD15_DS1 != 0"]
    #[inline] pub fn test_pad15_ds1(&self) -> bool {
        self.pad15_ds1() != 0
    }

    #[doc="Sets the PAD15_DS1 field."]
    #[inline] pub fn set_pad15_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 14 slew rate selection."]
    #[inline] pub fn pad14_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD14_SR != 0"]
    #[inline] pub fn test_pad14_sr(&self) -> bool {
        self.pad14_sr() != 0
    }

    #[doc="Sets the PAD14_SR field."]
    #[inline] pub fn set_pad14_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
    #[inline] pub fn pad14_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD14_DS1 != 0"]
    #[inline] pub fn test_pad14_ds1(&self) -> bool {
        self.pad14_ds1() != 0
    }

    #[doc="Sets the PAD14_DS1 field."]
    #[inline] pub fn set_pad14_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 13 slew rate selection."]
    #[inline] pub fn pad13_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD13_SR != 0"]
    #[inline] pub fn test_pad13_sr(&self) -> bool {
        self.pad13_sr() != 0
    }

    #[doc="Sets the PAD13_SR field."]
    #[inline] pub fn set_pad13_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
    #[inline] pub fn pad13_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD13_DS1 != 0"]
    #[inline] pub fn test_pad13_ds1(&self) -> bool {
        self.pad13_ds1() != 0
    }

    #[doc="Sets the PAD13_DS1 field."]
    #[inline] pub fn set_pad13_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 12 slew rate selection."]
    #[inline] pub fn pad12_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD12_SR != 0"]
    #[inline] pub fn test_pad12_sr(&self) -> bool {
        self.pad12_sr() != 0
    }

    #[doc="Sets the PAD12_SR field."]
    #[inline] pub fn set_pad12_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
    #[inline] pub fn pad12_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD12_DS1 != 0"]
    #[inline] pub fn test_pad12_ds1(&self) -> bool {
        self.pad12_ds1() != 0
    }

    #[doc="Sets the PAD12_DS1 field."]
    #[inline] pub fn set_pad12_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgd {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgd(other)
    }
}

impl ::core::fmt::Display for Altpadcfgd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad15_sr() != 0 { try!(write!(f, " pad15_sr"))}
        if self.pad15_ds1() != 0 { try!(write!(f, " pad15_ds1"))}
        if self.pad14_sr() != 0 { try!(write!(f, " pad14_sr"))}
        if self.pad14_ds1() != 0 { try!(write!(f, " pad14_ds1"))}
        if self.pad13_sr() != 0 { try!(write!(f, " pad13_sr"))}
        if self.pad13_ds1() != 0 { try!(write!(f, " pad13_ds1"))}
        if self.pad12_sr() != 0 { try!(write!(f, " pad12_sr"))}
        if self.pad12_ds1() != 0 { try!(write!(f, " pad12_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg4 (Pads 19,18,17,16)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfge(pub u32);
impl Altpadcfge {
    #[doc="Pad 19 slew rate selection."]
    #[inline] pub fn pad19_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD19_SR != 0"]
    #[inline] pub fn test_pad19_sr(&self) -> bool {
        self.pad19_sr() != 0
    }

    #[doc="Sets the PAD19_SR field."]
    #[inline] pub fn set_pad19_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
    #[inline] pub fn pad19_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD19_DS1 != 0"]
    #[inline] pub fn test_pad19_ds1(&self) -> bool {
        self.pad19_ds1() != 0
    }

    #[doc="Sets the PAD19_DS1 field."]
    #[inline] pub fn set_pad19_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 18 slew rate selection."]
    #[inline] pub fn pad18_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD18_SR != 0"]
    #[inline] pub fn test_pad18_sr(&self) -> bool {
        self.pad18_sr() != 0
    }

    #[doc="Sets the PAD18_SR field."]
    #[inline] pub fn set_pad18_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
    #[inline] pub fn pad18_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD18_DS1 != 0"]
    #[inline] pub fn test_pad18_ds1(&self) -> bool {
        self.pad18_ds1() != 0
    }

    #[doc="Sets the PAD18_DS1 field."]
    #[inline] pub fn set_pad18_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 17 slew rate selection."]
    #[inline] pub fn pad17_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD17_SR != 0"]
    #[inline] pub fn test_pad17_sr(&self) -> bool {
        self.pad17_sr() != 0
    }

    #[doc="Sets the PAD17_SR field."]
    #[inline] pub fn set_pad17_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
    #[inline] pub fn pad17_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD17_DS1 != 0"]
    #[inline] pub fn test_pad17_ds1(&self) -> bool {
        self.pad17_ds1() != 0
    }

    #[doc="Sets the PAD17_DS1 field."]
    #[inline] pub fn set_pad17_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 16 slew rate selection."]
    #[inline] pub fn pad16_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD16_SR != 0"]
    #[inline] pub fn test_pad16_sr(&self) -> bool {
        self.pad16_sr() != 0
    }

    #[doc="Sets the PAD16_SR field."]
    #[inline] pub fn set_pad16_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
    #[inline] pub fn pad16_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD16_DS1 != 0"]
    #[inline] pub fn test_pad16_ds1(&self) -> bool {
        self.pad16_ds1() != 0
    }

    #[doc="Sets the PAD16_DS1 field."]
    #[inline] pub fn set_pad16_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfge {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfge(other)
    }
}

impl ::core::fmt::Display for Altpadcfge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad19_sr() != 0 { try!(write!(f, " pad19_sr"))}
        if self.pad19_ds1() != 0 { try!(write!(f, " pad19_ds1"))}
        if self.pad18_sr() != 0 { try!(write!(f, " pad18_sr"))}
        if self.pad18_ds1() != 0 { try!(write!(f, " pad18_ds1"))}
        if self.pad17_sr() != 0 { try!(write!(f, " pad17_sr"))}
        if self.pad17_ds1() != 0 { try!(write!(f, " pad17_ds1"))}
        if self.pad16_sr() != 0 { try!(write!(f, " pad16_sr"))}
        if self.pad16_ds1() != 0 { try!(write!(f, " pad16_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg5 (Pads 23,22,21,20)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgf(pub u32);
impl Altpadcfgf {
    #[doc="Pad 23 slew rate selection."]
    #[inline] pub fn pad23_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD23_SR != 0"]
    #[inline] pub fn test_pad23_sr(&self) -> bool {
        self.pad23_sr() != 0
    }

    #[doc="Sets the PAD23_SR field."]
    #[inline] pub fn set_pad23_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
    #[inline] pub fn pad23_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD23_DS1 != 0"]
    #[inline] pub fn test_pad23_ds1(&self) -> bool {
        self.pad23_ds1() != 0
    }

    #[doc="Sets the PAD23_DS1 field."]
    #[inline] pub fn set_pad23_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 22 slew rate selection."]
    #[inline] pub fn pad22_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD22_SR != 0"]
    #[inline] pub fn test_pad22_sr(&self) -> bool {
        self.pad22_sr() != 0
    }

    #[doc="Sets the PAD22_SR field."]
    #[inline] pub fn set_pad22_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
    #[inline] pub fn pad22_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD22_DS1 != 0"]
    #[inline] pub fn test_pad22_ds1(&self) -> bool {
        self.pad22_ds1() != 0
    }

    #[doc="Sets the PAD22_DS1 field."]
    #[inline] pub fn set_pad22_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 21 slew rate selection."]
    #[inline] pub fn pad21_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD21_SR != 0"]
    #[inline] pub fn test_pad21_sr(&self) -> bool {
        self.pad21_sr() != 0
    }

    #[doc="Sets the PAD21_SR field."]
    #[inline] pub fn set_pad21_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
    #[inline] pub fn pad21_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD21_DS1 != 0"]
    #[inline] pub fn test_pad21_ds1(&self) -> bool {
        self.pad21_ds1() != 0
    }

    #[doc="Sets the PAD21_DS1 field."]
    #[inline] pub fn set_pad21_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 20 slew rate selection."]
    #[inline] pub fn pad20_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD20_SR != 0"]
    #[inline] pub fn test_pad20_sr(&self) -> bool {
        self.pad20_sr() != 0
    }

    #[doc="Sets the PAD20_SR field."]
    #[inline] pub fn set_pad20_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
    #[inline] pub fn pad20_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD20_DS1 != 0"]
    #[inline] pub fn test_pad20_ds1(&self) -> bool {
        self.pad20_ds1() != 0
    }

    #[doc="Sets the PAD20_DS1 field."]
    #[inline] pub fn set_pad20_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgf {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgf(other)
    }
}

impl ::core::fmt::Display for Altpadcfgf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad23_sr() != 0 { try!(write!(f, " pad23_sr"))}
        if self.pad23_ds1() != 0 { try!(write!(f, " pad23_ds1"))}
        if self.pad22_sr() != 0 { try!(write!(f, " pad22_sr"))}
        if self.pad22_ds1() != 0 { try!(write!(f, " pad22_ds1"))}
        if self.pad21_sr() != 0 { try!(write!(f, " pad21_sr"))}
        if self.pad21_ds1() != 0 { try!(write!(f, " pad21_ds1"))}
        if self.pad20_sr() != 0 { try!(write!(f, " pad20_sr"))}
        if self.pad20_ds1() != 0 { try!(write!(f, " pad20_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg6 (Pads 27,26,25,24)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgg(pub u32);
impl Altpadcfgg {
    #[doc="Pad 27 slew rate selection."]
    #[inline] pub fn pad27_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD27_SR != 0"]
    #[inline] pub fn test_pad27_sr(&self) -> bool {
        self.pad27_sr() != 0
    }

    #[doc="Sets the PAD27_SR field."]
    #[inline] pub fn set_pad27_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
    #[inline] pub fn pad27_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD27_DS1 != 0"]
    #[inline] pub fn test_pad27_ds1(&self) -> bool {
        self.pad27_ds1() != 0
    }

    #[doc="Sets the PAD27_DS1 field."]
    #[inline] pub fn set_pad27_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 26 slew rate selection."]
    #[inline] pub fn pad26_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD26_SR != 0"]
    #[inline] pub fn test_pad26_sr(&self) -> bool {
        self.pad26_sr() != 0
    }

    #[doc="Sets the PAD26_SR field."]
    #[inline] pub fn set_pad26_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
    #[inline] pub fn pad26_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD26_DS1 != 0"]
    #[inline] pub fn test_pad26_ds1(&self) -> bool {
        self.pad26_ds1() != 0
    }

    #[doc="Sets the PAD26_DS1 field."]
    #[inline] pub fn set_pad26_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 25 slew rate selection."]
    #[inline] pub fn pad25_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD25_SR != 0"]
    #[inline] pub fn test_pad25_sr(&self) -> bool {
        self.pad25_sr() != 0
    }

    #[doc="Sets the PAD25_SR field."]
    #[inline] pub fn set_pad25_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
    #[inline] pub fn pad25_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD25_DS1 != 0"]
    #[inline] pub fn test_pad25_ds1(&self) -> bool {
        self.pad25_ds1() != 0
    }

    #[doc="Sets the PAD25_DS1 field."]
    #[inline] pub fn set_pad25_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 24 slew rate selection."]
    #[inline] pub fn pad24_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD24_SR != 0"]
    #[inline] pub fn test_pad24_sr(&self) -> bool {
        self.pad24_sr() != 0
    }

    #[doc="Sets the PAD24_SR field."]
    #[inline] pub fn set_pad24_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
    #[inline] pub fn pad24_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD24_DS1 != 0"]
    #[inline] pub fn test_pad24_ds1(&self) -> bool {
        self.pad24_ds1() != 0
    }

    #[doc="Sets the PAD24_DS1 field."]
    #[inline] pub fn set_pad24_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgg {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgg(other)
    }
}

impl ::core::fmt::Display for Altpadcfgg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad27_sr() != 0 { try!(write!(f, " pad27_sr"))}
        if self.pad27_ds1() != 0 { try!(write!(f, " pad27_ds1"))}
        if self.pad26_sr() != 0 { try!(write!(f, " pad26_sr"))}
        if self.pad26_ds1() != 0 { try!(write!(f, " pad26_ds1"))}
        if self.pad25_sr() != 0 { try!(write!(f, " pad25_sr"))}
        if self.pad25_ds1() != 0 { try!(write!(f, " pad25_ds1"))}
        if self.pad24_sr() != 0 { try!(write!(f, " pad24_sr"))}
        if self.pad24_ds1() != 0 { try!(write!(f, " pad24_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg7 (Pads 31,30,29,28)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgh(pub u32);
impl Altpadcfgh {
    #[doc="Pad 31 slew rate selection."]
    #[inline] pub fn pad31_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD31_SR != 0"]
    #[inline] pub fn test_pad31_sr(&self) -> bool {
        self.pad31_sr() != 0
    }

    #[doc="Sets the PAD31_SR field."]
    #[inline] pub fn set_pad31_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 31 high order drive strength selection. Used in conjunction with PAD31STRNG field to set the pad drive strength."]
    #[inline] pub fn pad31_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD31_DS1 != 0"]
    #[inline] pub fn test_pad31_ds1(&self) -> bool {
        self.pad31_ds1() != 0
    }

    #[doc="Sets the PAD31_DS1 field."]
    #[inline] pub fn set_pad31_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 30 slew rate selection."]
    #[inline] pub fn pad30_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD30_SR != 0"]
    #[inline] pub fn test_pad30_sr(&self) -> bool {
        self.pad30_sr() != 0
    }

    #[doc="Sets the PAD30_SR field."]
    #[inline] pub fn set_pad30_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 30 high order drive strength selection. Used in conjunction with PAD30STRNG field to set the pad drive strength."]
    #[inline] pub fn pad30_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD30_DS1 != 0"]
    #[inline] pub fn test_pad30_ds1(&self) -> bool {
        self.pad30_ds1() != 0
    }

    #[doc="Sets the PAD30_DS1 field."]
    #[inline] pub fn set_pad30_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 29 slew rate selection."]
    #[inline] pub fn pad29_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD29_SR != 0"]
    #[inline] pub fn test_pad29_sr(&self) -> bool {
        self.pad29_sr() != 0
    }

    #[doc="Sets the PAD29_SR field."]
    #[inline] pub fn set_pad29_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 29 high order drive strength selection. Used in conjunction with PAD29STRNG field to set the pad drive strength."]
    #[inline] pub fn pad29_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD29_DS1 != 0"]
    #[inline] pub fn test_pad29_ds1(&self) -> bool {
        self.pad29_ds1() != 0
    }

    #[doc="Sets the PAD29_DS1 field."]
    #[inline] pub fn set_pad29_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 28 slew rate selection."]
    #[inline] pub fn pad28_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD28_SR != 0"]
    #[inline] pub fn test_pad28_sr(&self) -> bool {
        self.pad28_sr() != 0
    }

    #[doc="Sets the PAD28_SR field."]
    #[inline] pub fn set_pad28_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 28 high order drive strength selection. Used in conjunction with PAD28STRNG field to set the pad drive strength."]
    #[inline] pub fn pad28_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD28_DS1 != 0"]
    #[inline] pub fn test_pad28_ds1(&self) -> bool {
        self.pad28_ds1() != 0
    }

    #[doc="Sets the PAD28_DS1 field."]
    #[inline] pub fn set_pad28_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgh {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgh(other)
    }
}

impl ::core::fmt::Display for Altpadcfgh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad31_sr() != 0 { try!(write!(f, " pad31_sr"))}
        if self.pad31_ds1() != 0 { try!(write!(f, " pad31_ds1"))}
        if self.pad30_sr() != 0 { try!(write!(f, " pad30_sr"))}
        if self.pad30_ds1() != 0 { try!(write!(f, " pad30_ds1"))}
        if self.pad29_sr() != 0 { try!(write!(f, " pad29_sr"))}
        if self.pad29_ds1() != 0 { try!(write!(f, " pad29_ds1"))}
        if self.pad28_sr() != 0 { try!(write!(f, " pad28_sr"))}
        if self.pad28_ds1() != 0 { try!(write!(f, " pad28_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg8 (Pads 35,34,33,32)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgi(pub u32);
impl Altpadcfgi {
    #[doc="Pad 35 slew rate selection."]
    #[inline] pub fn pad35_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD35_SR != 0"]
    #[inline] pub fn test_pad35_sr(&self) -> bool {
        self.pad35_sr() != 0
    }

    #[doc="Sets the PAD35_SR field."]
    #[inline] pub fn set_pad35_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 35 high order drive strength selection. Used in conjunction with PAD35STRNG field to set the pad drive strength."]
    #[inline] pub fn pad35_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD35_DS1 != 0"]
    #[inline] pub fn test_pad35_ds1(&self) -> bool {
        self.pad35_ds1() != 0
    }

    #[doc="Sets the PAD35_DS1 field."]
    #[inline] pub fn set_pad35_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 34 slew rate selection."]
    #[inline] pub fn pad34_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD34_SR != 0"]
    #[inline] pub fn test_pad34_sr(&self) -> bool {
        self.pad34_sr() != 0
    }

    #[doc="Sets the PAD34_SR field."]
    #[inline] pub fn set_pad34_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 34 high order drive strength selection. Used in conjunction with PAD34STRNG field to set the pad drive strength."]
    #[inline] pub fn pad34_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD34_DS1 != 0"]
    #[inline] pub fn test_pad34_ds1(&self) -> bool {
        self.pad34_ds1() != 0
    }

    #[doc="Sets the PAD34_DS1 field."]
    #[inline] pub fn set_pad34_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 33 slew rate selection."]
    #[inline] pub fn pad33_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD33_SR != 0"]
    #[inline] pub fn test_pad33_sr(&self) -> bool {
        self.pad33_sr() != 0
    }

    #[doc="Sets the PAD33_SR field."]
    #[inline] pub fn set_pad33_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 33 high order drive strength selection. Used in conjunction with PAD33STRNG field to set the pad drive strength."]
    #[inline] pub fn pad33_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD33_DS1 != 0"]
    #[inline] pub fn test_pad33_ds1(&self) -> bool {
        self.pad33_ds1() != 0
    }

    #[doc="Sets the PAD33_DS1 field."]
    #[inline] pub fn set_pad33_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 32 slew rate selection."]
    #[inline] pub fn pad32_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD32_SR != 0"]
    #[inline] pub fn test_pad32_sr(&self) -> bool {
        self.pad32_sr() != 0
    }

    #[doc="Sets the PAD32_SR field."]
    #[inline] pub fn set_pad32_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 32 high order drive strength selection. Used in conjunction with PAD32STRNG field to set the pad drive strength."]
    #[inline] pub fn pad32_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD32_DS1 != 0"]
    #[inline] pub fn test_pad32_ds1(&self) -> bool {
        self.pad32_ds1() != 0
    }

    #[doc="Sets the PAD32_DS1 field."]
    #[inline] pub fn set_pad32_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgi {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgi(other)
    }
}

impl ::core::fmt::Display for Altpadcfgi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad35_sr() != 0 { try!(write!(f, " pad35_sr"))}
        if self.pad35_ds1() != 0 { try!(write!(f, " pad35_ds1"))}
        if self.pad34_sr() != 0 { try!(write!(f, " pad34_sr"))}
        if self.pad34_ds1() != 0 { try!(write!(f, " pad34_ds1"))}
        if self.pad33_sr() != 0 { try!(write!(f, " pad33_sr"))}
        if self.pad33_ds1() != 0 { try!(write!(f, " pad33_ds1"))}
        if self.pad32_sr() != 0 { try!(write!(f, " pad32_sr"))}
        if self.pad32_ds1() != 0 { try!(write!(f, " pad32_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg9 (Pads 39,38,37,36)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgj(pub u32);
impl Altpadcfgj {
    #[doc="Pad 39 slew rate selection."]
    #[inline] pub fn pad39_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD39_SR != 0"]
    #[inline] pub fn test_pad39_sr(&self) -> bool {
        self.pad39_sr() != 0
    }

    #[doc="Sets the PAD39_SR field."]
    #[inline] pub fn set_pad39_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
    #[inline] pub fn pad39_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD39_DS1 != 0"]
    #[inline] pub fn test_pad39_ds1(&self) -> bool {
        self.pad39_ds1() != 0
    }

    #[doc="Sets the PAD39_DS1 field."]
    #[inline] pub fn set_pad39_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 38 slew rate selection."]
    #[inline] pub fn pad38_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD38_SR != 0"]
    #[inline] pub fn test_pad38_sr(&self) -> bool {
        self.pad38_sr() != 0
    }

    #[doc="Sets the PAD38_SR field."]
    #[inline] pub fn set_pad38_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
    #[inline] pub fn pad38_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD38_DS1 != 0"]
    #[inline] pub fn test_pad38_ds1(&self) -> bool {
        self.pad38_ds1() != 0
    }

    #[doc="Sets the PAD38_DS1 field."]
    #[inline] pub fn set_pad38_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 37 slew rate selection."]
    #[inline] pub fn pad37_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD37_SR != 0"]
    #[inline] pub fn test_pad37_sr(&self) -> bool {
        self.pad37_sr() != 0
    }

    #[doc="Sets the PAD37_SR field."]
    #[inline] pub fn set_pad37_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
    #[inline] pub fn pad37_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD37_DS1 != 0"]
    #[inline] pub fn test_pad37_ds1(&self) -> bool {
        self.pad37_ds1() != 0
    }

    #[doc="Sets the PAD37_DS1 field."]
    #[inline] pub fn set_pad37_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 36 slew rate selection."]
    #[inline] pub fn pad36_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD36_SR != 0"]
    #[inline] pub fn test_pad36_sr(&self) -> bool {
        self.pad36_sr() != 0
    }

    #[doc="Sets the PAD36_SR field."]
    #[inline] pub fn set_pad36_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
    #[inline] pub fn pad36_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD36_DS1 != 0"]
    #[inline] pub fn test_pad36_ds1(&self) -> bool {
        self.pad36_ds1() != 0
    }

    #[doc="Sets the PAD36_DS1 field."]
    #[inline] pub fn set_pad36_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgj {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgj(other)
    }
}

impl ::core::fmt::Display for Altpadcfgj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad39_sr() != 0 { try!(write!(f, " pad39_sr"))}
        if self.pad39_ds1() != 0 { try!(write!(f, " pad39_ds1"))}
        if self.pad38_sr() != 0 { try!(write!(f, " pad38_sr"))}
        if self.pad38_ds1() != 0 { try!(write!(f, " pad38_ds1"))}
        if self.pad37_sr() != 0 { try!(write!(f, " pad37_sr"))}
        if self.pad37_ds1() != 0 { try!(write!(f, " pad37_ds1"))}
        if self.pad36_sr() != 0 { try!(write!(f, " pad36_sr"))}
        if self.pad36_ds1() != 0 { try!(write!(f, " pad36_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg10 (Pads 43,42,41,40)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgk(pub u32);
impl Altpadcfgk {
    #[doc="Pad 43 slew rate selection."]
    #[inline] pub fn pad43_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD43_SR != 0"]
    #[inline] pub fn test_pad43_sr(&self) -> bool {
        self.pad43_sr() != 0
    }

    #[doc="Sets the PAD43_SR field."]
    #[inline] pub fn set_pad43_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 43 high order drive strength selection. Used in conjunction with PAD43STRNG field to set the pad drive strength."]
    #[inline] pub fn pad43_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD43_DS1 != 0"]
    #[inline] pub fn test_pad43_ds1(&self) -> bool {
        self.pad43_ds1() != 0
    }

    #[doc="Sets the PAD43_DS1 field."]
    #[inline] pub fn set_pad43_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 42 slew rate selection."]
    #[inline] pub fn pad42_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD42_SR != 0"]
    #[inline] pub fn test_pad42_sr(&self) -> bool {
        self.pad42_sr() != 0
    }

    #[doc="Sets the PAD42_SR field."]
    #[inline] pub fn set_pad42_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 42 high order drive strength selection. Used in conjunction with PAD42STRNG field to set the pad drive strength."]
    #[inline] pub fn pad42_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD42_DS1 != 0"]
    #[inline] pub fn test_pad42_ds1(&self) -> bool {
        self.pad42_ds1() != 0
    }

    #[doc="Sets the PAD42_DS1 field."]
    #[inline] pub fn set_pad42_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 41 slew rate selection."]
    #[inline] pub fn pad41_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD41_SR != 0"]
    #[inline] pub fn test_pad41_sr(&self) -> bool {
        self.pad41_sr() != 0
    }

    #[doc="Sets the PAD41_SR field."]
    #[inline] pub fn set_pad41_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 41 high order drive strength selection. Used in conjunction with PAD41STRNG field to set the pad drive strength."]
    #[inline] pub fn pad41_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD41_DS1 != 0"]
    #[inline] pub fn test_pad41_ds1(&self) -> bool {
        self.pad41_ds1() != 0
    }

    #[doc="Sets the PAD41_DS1 field."]
    #[inline] pub fn set_pad41_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 40 slew rate selection."]
    #[inline] pub fn pad40_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD40_SR != 0"]
    #[inline] pub fn test_pad40_sr(&self) -> bool {
        self.pad40_sr() != 0
    }

    #[doc="Sets the PAD40_SR field."]
    #[inline] pub fn set_pad40_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 40 high order drive strength selection. Used in conjunction with PAD40STRNG field to set the pad drive strength."]
    #[inline] pub fn pad40_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD40_DS1 != 0"]
    #[inline] pub fn test_pad40_ds1(&self) -> bool {
        self.pad40_ds1() != 0
    }

    #[doc="Sets the PAD40_DS1 field."]
    #[inline] pub fn set_pad40_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgk {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgk(other)
    }
}

impl ::core::fmt::Display for Altpadcfgk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad43_sr() != 0 { try!(write!(f, " pad43_sr"))}
        if self.pad43_ds1() != 0 { try!(write!(f, " pad43_ds1"))}
        if self.pad42_sr() != 0 { try!(write!(f, " pad42_sr"))}
        if self.pad42_ds1() != 0 { try!(write!(f, " pad42_ds1"))}
        if self.pad41_sr() != 0 { try!(write!(f, " pad41_sr"))}
        if self.pad41_ds1() != 0 { try!(write!(f, " pad41_ds1"))}
        if self.pad40_sr() != 0 { try!(write!(f, " pad40_sr"))}
        if self.pad40_ds1() != 0 { try!(write!(f, " pad40_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg11 (Pads 47,46,45,44)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgl(pub u32);
impl Altpadcfgl {
    #[doc="Pad 47 slew rate selection."]
    #[inline] pub fn pad47_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PAD47_SR != 0"]
    #[inline] pub fn test_pad47_sr(&self) -> bool {
        self.pad47_sr() != 0
    }

    #[doc="Sets the PAD47_SR field."]
    #[inline] pub fn set_pad47_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pad 47 high order drive strength selection. Used in conjunction with PAD47STRNG field to set the pad drive strength."]
    #[inline] pub fn pad47_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PAD47_DS1 != 0"]
    #[inline] pub fn test_pad47_ds1(&self) -> bool {
        self.pad47_ds1() != 0
    }

    #[doc="Sets the PAD47_DS1 field."]
    #[inline] pub fn set_pad47_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pad 46 slew rate selection."]
    #[inline] pub fn pad46_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PAD46_SR != 0"]
    #[inline] pub fn test_pad46_sr(&self) -> bool {
        self.pad46_sr() != 0
    }

    #[doc="Sets the PAD46_SR field."]
    #[inline] pub fn set_pad46_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pad 46 high order drive strength selection. Used in conjunction with PAD46STRNG field to set the pad drive strength."]
    #[inline] pub fn pad46_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PAD46_DS1 != 0"]
    #[inline] pub fn test_pad46_ds1(&self) -> bool {
        self.pad46_ds1() != 0
    }

    #[doc="Sets the PAD46_DS1 field."]
    #[inline] pub fn set_pad46_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pad 45 slew rate selection."]
    #[inline] pub fn pad45_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD45_SR != 0"]
    #[inline] pub fn test_pad45_sr(&self) -> bool {
        self.pad45_sr() != 0
    }

    #[doc="Sets the PAD45_SR field."]
    #[inline] pub fn set_pad45_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 45 high order drive strength selection. Used in conjunction with PAD45STRNG field to set the pad drive strength."]
    #[inline] pub fn pad45_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD45_DS1 != 0"]
    #[inline] pub fn test_pad45_ds1(&self) -> bool {
        self.pad45_ds1() != 0
    }

    #[doc="Sets the PAD45_DS1 field."]
    #[inline] pub fn set_pad45_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 44 slew rate selection."]
    #[inline] pub fn pad44_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD44_SR != 0"]
    #[inline] pub fn test_pad44_sr(&self) -> bool {
        self.pad44_sr() != 0
    }

    #[doc="Sets the PAD44_SR field."]
    #[inline] pub fn set_pad44_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 44 high order drive strength selection. Used in conjunction with PAD44STRNG field to set the pad drive strength."]
    #[inline] pub fn pad44_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD44_DS1 != 0"]
    #[inline] pub fn test_pad44_ds1(&self) -> bool {
        self.pad44_ds1() != 0
    }

    #[doc="Sets the PAD44_DS1 field."]
    #[inline] pub fn set_pad44_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgl {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgl(other)
    }
}

impl ::core::fmt::Display for Altpadcfgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad47_sr() != 0 { try!(write!(f, " pad47_sr"))}
        if self.pad47_ds1() != 0 { try!(write!(f, " pad47_ds1"))}
        if self.pad46_sr() != 0 { try!(write!(f, " pad46_sr"))}
        if self.pad46_ds1() != 0 { try!(write!(f, " pad46_ds1"))}
        if self.pad45_sr() != 0 { try!(write!(f, " pad45_sr"))}
        if self.pad45_ds1() != 0 { try!(write!(f, " pad45_ds1"))}
        if self.pad44_sr() != 0 { try!(write!(f, " pad44_sr"))}
        if self.pad44_ds1() != 0 { try!(write!(f, " pad44_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Alternate Pad Configuration reg12 (Pads 49,48)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Altpadcfgm(pub u32);
impl Altpadcfgm {
    #[doc="Pad 49 slew rate selection."]
    #[inline] pub fn pad49_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PAD49_SR != 0"]
    #[inline] pub fn test_pad49_sr(&self) -> bool {
        self.pad49_sr() != 0
    }

    #[doc="Sets the PAD49_SR field."]
    #[inline] pub fn set_pad49_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pad 49 high order drive strength selection. Used in conjunction with PAD49STRNG field to set the pad drive strength."]
    #[inline] pub fn pad49_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PAD49_DS1 != 0"]
    #[inline] pub fn test_pad49_ds1(&self) -> bool {
        self.pad49_ds1() != 0
    }

    #[doc="Sets the PAD49_DS1 field."]
    #[inline] pub fn set_pad49_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pad 48 slew rate selection."]
    #[inline] pub fn pad48_sr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PAD48_SR != 0"]
    #[inline] pub fn test_pad48_sr(&self) -> bool {
        self.pad48_sr() != 0
    }

    #[doc="Sets the PAD48_SR field."]
    #[inline] pub fn set_pad48_sr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pad 48 high order drive strength selection. Used in conjunction with PAD48STRNG field to set the pad drive strength."]
    #[inline] pub fn pad48_ds1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PAD48_DS1 != 0"]
    #[inline] pub fn test_pad48_ds1(&self) -> bool {
        self.pad48_ds1() != 0
    }

    #[doc="Sets the PAD48_DS1 field."]
    #[inline] pub fn set_pad48_ds1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Altpadcfgm {
    #[inline]
    fn from(other: u32) -> Self {
         Altpadcfgm(other)
    }
}

impl ::core::fmt::Display for Altpadcfgm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Altpadcfgm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pad49_sr() != 0 { try!(write!(f, " pad49_sr"))}
        if self.pad49_ds1() != 0 { try!(write!(f, " pad49_ds1"))}
        if self.pad48_sr() != 0 { try!(write!(f, " pad48_sr"))}
        if self.pad48_ds1() != 0 { try!(write!(f, " pad48_ds1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers n: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="GPIOn interrupt."]
    #[inline] pub fn gpio<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO != 0"]
    #[inline] pub fn test_gpio<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.gpio(index) != 0
    }

    #[doc="Sets the GPIO field."]
    #[inline] pub fn set_gpio<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Inten {
    #[inline]
    fn from(other: u32) -> Self {
         Inten(other)
    }
}

impl ::core::fmt::Display for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio(0) != 0 { try!(write!(f, " gpio[0]"))}
        if self.gpio(1) != 0 { try!(write!(f, " gpio[1]"))}
        if self.gpio(2) != 0 { try!(write!(f, " gpio[2]"))}
        if self.gpio(3) != 0 { try!(write!(f, " gpio[3]"))}
        if self.gpio(4) != 0 { try!(write!(f, " gpio[4]"))}
        if self.gpio(5) != 0 { try!(write!(f, " gpio[5]"))}
        if self.gpio(6) != 0 { try!(write!(f, " gpio[6]"))}
        if self.gpio(7) != 0 { try!(write!(f, " gpio[7]"))}
        if self.gpio(8) != 0 { try!(write!(f, " gpio[8]"))}
        if self.gpio(9) != 0 { try!(write!(f, " gpio[9]"))}
        if self.gpio(10) != 0 { try!(write!(f, " gpio[10]"))}
        if self.gpio(11) != 0 { try!(write!(f, " gpio[11]"))}
        if self.gpio(12) != 0 { try!(write!(f, " gpio[12]"))}
        if self.gpio(13) != 0 { try!(write!(f, " gpio[13]"))}
        if self.gpio(14) != 0 { try!(write!(f, " gpio[14]"))}
        if self.gpio(15) != 0 { try!(write!(f, " gpio[15]"))}
        if self.gpio(16) != 0 { try!(write!(f, " gpio[16]"))}
        if self.gpio(17) != 0 { try!(write!(f, " gpio[17]"))}
        if self.gpio(18) != 0 { try!(write!(f, " gpio[18]"))}
        if self.gpio(19) != 0 { try!(write!(f, " gpio[19]"))}
        if self.gpio(20) != 0 { try!(write!(f, " gpio[20]"))}
        if self.gpio(21) != 0 { try!(write!(f, " gpio[21]"))}
        if self.gpio(22) != 0 { try!(write!(f, " gpio[22]"))}
        if self.gpio(23) != 0 { try!(write!(f, " gpio[23]"))}
        if self.gpio(24) != 0 { try!(write!(f, " gpio[24]"))}
        if self.gpio(25) != 0 { try!(write!(f, " gpio[25]"))}
        if self.gpio(26) != 0 { try!(write!(f, " gpio[26]"))}
        if self.gpio(27) != 0 { try!(write!(f, " gpio[27]"))}
        if self.gpio(28) != 0 { try!(write!(f, " gpio[28]"))}
        if self.gpio(29) != 0 { try!(write!(f, " gpio[29]"))}
        if self.gpio(30) != 0 { try!(write!(f, " gpio[30]"))}
        if self.gpio(31) != 0 { try!(write!(f, " gpio[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers n: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="GPIOn interrupt."]
    #[inline] pub fn gpio<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO != 0"]
    #[inline] pub fn test_gpio<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.gpio(index) != 0
    }

    #[doc="Sets the GPIO field."]
    #[inline] pub fn set_gpio<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Intstat {
    #[inline]
    fn from(other: u32) -> Self {
         Intstat(other)
    }
}

impl ::core::fmt::Display for Intstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio(0) != 0 { try!(write!(f, " gpio[0]"))}
        if self.gpio(1) != 0 { try!(write!(f, " gpio[1]"))}
        if self.gpio(2) != 0 { try!(write!(f, " gpio[2]"))}
        if self.gpio(3) != 0 { try!(write!(f, " gpio[3]"))}
        if self.gpio(4) != 0 { try!(write!(f, " gpio[4]"))}
        if self.gpio(5) != 0 { try!(write!(f, " gpio[5]"))}
        if self.gpio(6) != 0 { try!(write!(f, " gpio[6]"))}
        if self.gpio(7) != 0 { try!(write!(f, " gpio[7]"))}
        if self.gpio(8) != 0 { try!(write!(f, " gpio[8]"))}
        if self.gpio(9) != 0 { try!(write!(f, " gpio[9]"))}
        if self.gpio(10) != 0 { try!(write!(f, " gpio[10]"))}
        if self.gpio(11) != 0 { try!(write!(f, " gpio[11]"))}
        if self.gpio(12) != 0 { try!(write!(f, " gpio[12]"))}
        if self.gpio(13) != 0 { try!(write!(f, " gpio[13]"))}
        if self.gpio(14) != 0 { try!(write!(f, " gpio[14]"))}
        if self.gpio(15) != 0 { try!(write!(f, " gpio[15]"))}
        if self.gpio(16) != 0 { try!(write!(f, " gpio[16]"))}
        if self.gpio(17) != 0 { try!(write!(f, " gpio[17]"))}
        if self.gpio(18) != 0 { try!(write!(f, " gpio[18]"))}
        if self.gpio(19) != 0 { try!(write!(f, " gpio[19]"))}
        if self.gpio(20) != 0 { try!(write!(f, " gpio[20]"))}
        if self.gpio(21) != 0 { try!(write!(f, " gpio[21]"))}
        if self.gpio(22) != 0 { try!(write!(f, " gpio[22]"))}
        if self.gpio(23) != 0 { try!(write!(f, " gpio[23]"))}
        if self.gpio(24) != 0 { try!(write!(f, " gpio[24]"))}
        if self.gpio(25) != 0 { try!(write!(f, " gpio[25]"))}
        if self.gpio(26) != 0 { try!(write!(f, " gpio[26]"))}
        if self.gpio(27) != 0 { try!(write!(f, " gpio[27]"))}
        if self.gpio(28) != 0 { try!(write!(f, " gpio[28]"))}
        if self.gpio(29) != 0 { try!(write!(f, " gpio[29]"))}
        if self.gpio(30) != 0 { try!(write!(f, " gpio[30]"))}
        if self.gpio(31) != 0 { try!(write!(f, " gpio[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers n: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="GPIOn interrupt."]
    #[inline] pub fn gpio<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO != 0"]
    #[inline] pub fn test_gpio<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.gpio(index) != 0
    }

    #[doc="Sets the GPIO field."]
    #[inline] pub fn set_gpio<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Intclr {
    #[inline]
    fn from(other: u32) -> Self {
         Intclr(other)
    }
}

impl ::core::fmt::Display for Intclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio(0) != 0 { try!(write!(f, " gpio[0]"))}
        if self.gpio(1) != 0 { try!(write!(f, " gpio[1]"))}
        if self.gpio(2) != 0 { try!(write!(f, " gpio[2]"))}
        if self.gpio(3) != 0 { try!(write!(f, " gpio[3]"))}
        if self.gpio(4) != 0 { try!(write!(f, " gpio[4]"))}
        if self.gpio(5) != 0 { try!(write!(f, " gpio[5]"))}
        if self.gpio(6) != 0 { try!(write!(f, " gpio[6]"))}
        if self.gpio(7) != 0 { try!(write!(f, " gpio[7]"))}
        if self.gpio(8) != 0 { try!(write!(f, " gpio[8]"))}
        if self.gpio(9) != 0 { try!(write!(f, " gpio[9]"))}
        if self.gpio(10) != 0 { try!(write!(f, " gpio[10]"))}
        if self.gpio(11) != 0 { try!(write!(f, " gpio[11]"))}
        if self.gpio(12) != 0 { try!(write!(f, " gpio[12]"))}
        if self.gpio(13) != 0 { try!(write!(f, " gpio[13]"))}
        if self.gpio(14) != 0 { try!(write!(f, " gpio[14]"))}
        if self.gpio(15) != 0 { try!(write!(f, " gpio[15]"))}
        if self.gpio(16) != 0 { try!(write!(f, " gpio[16]"))}
        if self.gpio(17) != 0 { try!(write!(f, " gpio[17]"))}
        if self.gpio(18) != 0 { try!(write!(f, " gpio[18]"))}
        if self.gpio(19) != 0 { try!(write!(f, " gpio[19]"))}
        if self.gpio(20) != 0 { try!(write!(f, " gpio[20]"))}
        if self.gpio(21) != 0 { try!(write!(f, " gpio[21]"))}
        if self.gpio(22) != 0 { try!(write!(f, " gpio[22]"))}
        if self.gpio(23) != 0 { try!(write!(f, " gpio[23]"))}
        if self.gpio(24) != 0 { try!(write!(f, " gpio[24]"))}
        if self.gpio(25) != 0 { try!(write!(f, " gpio[25]"))}
        if self.gpio(26) != 0 { try!(write!(f, " gpio[26]"))}
        if self.gpio(27) != 0 { try!(write!(f, " gpio[27]"))}
        if self.gpio(28) != 0 { try!(write!(f, " gpio[28]"))}
        if self.gpio(29) != 0 { try!(write!(f, " gpio[29]"))}
        if self.gpio(30) != 0 { try!(write!(f, " gpio[30]"))}
        if self.gpio(31) != 0 { try!(write!(f, " gpio[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO Interrupt Registers 31-0: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Int0set(pub u32);
impl Int0set {
    #[doc="GPIOn interrupt."]
    #[inline] pub fn gpio<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GPIO != 0"]
    #[inline] pub fn test_gpio<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.gpio(index) != 0
    }

    #[doc="Sets the GPIO field."]
    #[inline] pub fn set_gpio<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Int0set {
    #[inline]
    fn from(other: u32) -> Self {
         Int0set(other)
    }
}

impl ::core::fmt::Display for Int0set {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Int0set {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpio(0) != 0 { try!(write!(f, " gpio[0]"))}
        if self.gpio(1) != 0 { try!(write!(f, " gpio[1]"))}
        if self.gpio(2) != 0 { try!(write!(f, " gpio[2]"))}
        if self.gpio(3) != 0 { try!(write!(f, " gpio[3]"))}
        if self.gpio(4) != 0 { try!(write!(f, " gpio[4]"))}
        if self.gpio(5) != 0 { try!(write!(f, " gpio[5]"))}
        if self.gpio(6) != 0 { try!(write!(f, " gpio[6]"))}
        if self.gpio(7) != 0 { try!(write!(f, " gpio[7]"))}
        if self.gpio(8) != 0 { try!(write!(f, " gpio[8]"))}
        if self.gpio(9) != 0 { try!(write!(f, " gpio[9]"))}
        if self.gpio(10) != 0 { try!(write!(f, " gpio[10]"))}
        if self.gpio(11) != 0 { try!(write!(f, " gpio[11]"))}
        if self.gpio(12) != 0 { try!(write!(f, " gpio[12]"))}
        if self.gpio(13) != 0 { try!(write!(f, " gpio[13]"))}
        if self.gpio(14) != 0 { try!(write!(f, " gpio[14]"))}
        if self.gpio(15) != 0 { try!(write!(f, " gpio[15]"))}
        if self.gpio(16) != 0 { try!(write!(f, " gpio[16]"))}
        if self.gpio(17) != 0 { try!(write!(f, " gpio[17]"))}
        if self.gpio(18) != 0 { try!(write!(f, " gpio[18]"))}
        if self.gpio(19) != 0 { try!(write!(f, " gpio[19]"))}
        if self.gpio(20) != 0 { try!(write!(f, " gpio[20]"))}
        if self.gpio(21) != 0 { try!(write!(f, " gpio[21]"))}
        if self.gpio(22) != 0 { try!(write!(f, " gpio[22]"))}
        if self.gpio(23) != 0 { try!(write!(f, " gpio[23]"))}
        if self.gpio(24) != 0 { try!(write!(f, " gpio[24]"))}
        if self.gpio(25) != 0 { try!(write!(f, " gpio[25]"))}
        if self.gpio(26) != 0 { try!(write!(f, " gpio[26]"))}
        if self.gpio(27) != 0 { try!(write!(f, " gpio[27]"))}
        if self.gpio(28) != 0 { try!(write!(f, " gpio[28]"))}
        if self.gpio(29) != 0 { try!(write!(f, " gpio[29]"))}
        if self.gpio(30) != 0 { try!(write!(f, " gpio[30]"))}
        if self.gpio(31) != 0 { try!(write!(f, " gpio[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub struct GpioPin { pub port: GpioPeriph, pub index: usize }
pin!(GP0, Gp0, GPIO, Gpio, _GP0, GpioPin, _GPIO, 0);
    alt_fn!(Gp0, super::sig::Slscl, 0);
    alt_fn!(Gp0, super::sig::Slsck, 1);
    alt_fn!(Gp0, super::sig::Clkout, 2);
    alt_fn!(Gp0, super::sig::Gpio0, 3);
    alt_fn!(Gp0, super::sig::Mxscklb, 4);
    alt_fn!(Gp0, super::sig::M2sck, 5);
    alt_fn!(Gp0, super::sig::Mxscllb, 6);
    alt_fn!(Gp0, super::sig::M2scl, 7);

pin!(GP1, Gp1, GPIO, Gpio, _GP1, GpioPin, _GPIO, 1);
    alt_fn!(Gp1, super::sig::Slsda, 0);
    alt_fn!(Gp1, super::sig::Slmiso, 1);
    alt_fn!(Gp1, super::sig::Uart0tx, 2);
    alt_fn!(Gp1, super::sig::Gpio1, 3);
    alt_fn!(Gp1, super::sig::Mxmisolb, 4);
    alt_fn!(Gp1, super::sig::M2mis0, 5);
    alt_fn!(Gp1, super::sig::Mxsdalb, 6);
    alt_fn!(Gp1, super::sig::M2sda, 7);

pin!(GP2, Gp2, GPIO, Gpio, _GP2, GpioPin, _GPIO, 2);
    alt_fn!(Gp2, super::sig::Slwir3, 0);
    alt_fn!(Gp2, super::sig::Slmosi, 1);
    alt_fn!(Gp2, super::sig::Uart0rx, 2);
    alt_fn!(Gp2, super::sig::Gpio2, 3);
    alt_fn!(Gp2, super::sig::Mxmosilb, 4);
    alt_fn!(Gp2, super::sig::M2mosi, 5);
    alt_fn!(Gp2, super::sig::Mxwir3lb, 6);
    alt_fn!(Gp2, super::sig::M2wir3, 7);

pin!(GP3, Gp3, GPIO, Gpio, _GP3, GpioPin, _GPIO, 3);
    alt_fn!(Gp3, super::sig::Ua0rts, 0);
    alt_fn!(Gp3, super::sig::Slnce, 1);
    alt_fn!(Gp3, super::sig::M1nce4, 2);
    alt_fn!(Gp3, super::sig::Gpio3, 3);
    alt_fn!(Gp3, super::sig::Mxncelb, 4);
    alt_fn!(Gp3, super::sig::M2nce0, 5);
    alt_fn!(Gp3, super::sig::Trig1, 6);
    alt_fn!(Gp3, super::sig::I2sWclk, 7);

pin!(GP4, Gp4, GPIO, Gpio, _GP4, GpioPin, _GPIO, 4);
    alt_fn!(Gp4, super::sig::Ua0cts, 0);
    alt_fn!(Gp4, super::sig::Slint, 1);
    alt_fn!(Gp4, super::sig::M0nce5, 2);
    alt_fn!(Gp4, super::sig::Gpio4, 3);
    alt_fn!(Gp4, super::sig::Slintgp, 4);
    alt_fn!(Gp4, super::sig::M2nce5, 5);
    alt_fn!(Gp4, super::sig::Clkout, 6);
    alt_fn!(Gp4, super::sig::Xt32khz, 7);

pin!(GP5, Gp5, GPIO, Gpio, _GP5, GpioPin, _GPIO, 5);
    alt_fn!(Gp5, super::sig::M0scl, 0);
    alt_fn!(Gp5, super::sig::M0sck, 1);
    alt_fn!(Gp5, super::sig::Ua0rts, 2);
    alt_fn!(Gp5, super::sig::Gpio5, 3);
    alt_fn!(Gp5, super::sig::M0scklb, 4);
    alt_fn!(Gp5, super::sig::M0scllb, 6);
    alt_fn!(Gp5, super::sig::M1nce2, 7);

pin!(GP6, Gp6, GPIO, Gpio, _GP6, GpioPin, _GPIO, 6);
    alt_fn!(Gp6, super::sig::M0sda, 0);
    alt_fn!(Gp6, super::sig::M0miso, 1);
    alt_fn!(Gp6, super::sig::Ua0cts, 2);
    alt_fn!(Gp6, super::sig::Gpio6, 3);
    alt_fn!(Gp6, super::sig::Slmisolb, 4);
    alt_fn!(Gp6, super::sig::M1nce0, 5);
    alt_fn!(Gp6, super::sig::Slsdalb, 6);
    alt_fn!(Gp6, super::sig::I2sDat, 7);

pin!(GP7, Gp7, GPIO, Gpio, _GP7, GpioPin, _GPIO, 7);
    alt_fn!(Gp7, super::sig::M0wir3, 0);
    alt_fn!(Gp7, super::sig::M0mosi, 1);
    alt_fn!(Gp7, super::sig::Clkout, 2);
    alt_fn!(Gp7, super::sig::Gpio7, 3);
    alt_fn!(Gp7, super::sig::Trig0, 4);
    alt_fn!(Gp7, super::sig::Uart0tx, 5);
    alt_fn!(Gp7, super::sig::Slwir3lb, 6);
    alt_fn!(Gp7, super::sig::M1nce1, 7);

pin!(GP8, Gp8, GPIO, Gpio, _GP8, GpioPin, _GPIO, 8);
    alt_fn!(Gp8, super::sig::M1scl, 0);
    alt_fn!(Gp8, super::sig::M1sck, 1);
    alt_fn!(Gp8, super::sig::M0nce4, 2);
    alt_fn!(Gp8, super::sig::Gpio8, 3);
    alt_fn!(Gp8, super::sig::M2nce4, 4);
    alt_fn!(Gp8, super::sig::M1scklb, 5);
    alt_fn!(Gp8, super::sig::Uart1tx, 6);
    alt_fn!(Gp8, super::sig::M1scllb, 7);

pin!(GP9, Gp9, GPIO, Gpio, _GP9, GpioPin, _GPIO, 9);
    alt_fn!(Gp9, super::sig::M1sda, 0);
    alt_fn!(Gp9, super::sig::M1miso, 1);
    alt_fn!(Gp9, super::sig::M0nce5, 2);
    alt_fn!(Gp9, super::sig::Gpio9, 3);
    alt_fn!(Gp9, super::sig::M4nce5, 4);
    alt_fn!(Gp9, super::sig::Slmisolb, 5);
    alt_fn!(Gp9, super::sig::Uart1rx, 6);
    alt_fn!(Gp9, super::sig::Slsdalb, 7);

pin!(GP10, Gp10, GPIO, Gpio, _GP10, GpioPin, _GPIO, 10);
    alt_fn!(Gp10, super::sig::M1wir3, 0);
    alt_fn!(Gp10, super::sig::M1mosi, 1);
    alt_fn!(Gp10, super::sig::M0nce6, 2);
    alt_fn!(Gp10, super::sig::Gpio10, 3);
    alt_fn!(Gp10, super::sig::M2nce6, 4);
    alt_fn!(Gp10, super::sig::Ua1rts, 5);
    alt_fn!(Gp10, super::sig::M4nce4, 6);
    alt_fn!(Gp10, super::sig::Slwir3lb, 7);

pin!(GP11, Gp11, GPIO, Gpio, _GP11, GpioPin, _GPIO, 11);
    alt_fn!(Gp11, super::sig::Adcse2, 0);
    alt_fn!(Gp11, super::sig::M0nce0, 1);
    alt_fn!(Gp11, super::sig::Clkout, 2);
    alt_fn!(Gp11, super::sig::Gpio11, 3);
    alt_fn!(Gp11, super::sig::M2nce7, 4);
    alt_fn!(Gp11, super::sig::Ua1cts, 5);
    alt_fn!(Gp11, super::sig::Uart0rx, 6);
    alt_fn!(Gp11, super::sig::PdmData, 7);

pin!(GP12, Gp12, GPIO, Gpio, _GP12, GpioPin, _GPIO, 12);
    alt_fn!(Gp12, super::sig::Adcd0nse9, 0);
    alt_fn!(Gp12, super::sig::M1nce0, 1);
    alt_fn!(Gp12, super::sig::Tcta0, 2);
    alt_fn!(Gp12, super::sig::Gpio12, 3);
    alt_fn!(Gp12, super::sig::Clkout, 4);
    alt_fn!(Gp12, super::sig::PdmClk, 5);
    alt_fn!(Gp12, super::sig::Ua0cts, 6);
    alt_fn!(Gp12, super::sig::Uart1tx, 7);

pin!(GP13, Gp13, GPIO, Gpio, _GP13, GpioPin, _GPIO, 13);
    alt_fn!(Gp13, super::sig::Adcd0pse8, 0);
    alt_fn!(Gp13, super::sig::M1nce1, 1);
    alt_fn!(Gp13, super::sig::Tctb0, 2);
    alt_fn!(Gp13, super::sig::Gpio13, 3);
    alt_fn!(Gp13, super::sig::M2nce3, 4);
    alt_fn!(Gp13, super::sig::Ua0rts, 6);
    alt_fn!(Gp13, super::sig::Uart1rx, 7);

pin!(GP14, Gp14, GPIO, Gpio, _GP14, GpioPin, _GPIO, 14);
    alt_fn!(Gp14, super::sig::Adcd1p, 0);
    alt_fn!(Gp14, super::sig::M1nce2, 1);
    alt_fn!(Gp14, super::sig::Uart1tx, 2);
    alt_fn!(Gp14, super::sig::Gpio14, 3);
    alt_fn!(Gp14, super::sig::M2nce1, 4);
    alt_fn!(Gp14, super::sig::Swdck, 6);
    alt_fn!(Gp14, super::sig::Xt32khz, 7);

pin!(GP15, Gp15, GPIO, Gpio, _GP15, GpioPin, _GPIO, 15);
    alt_fn!(Gp15, super::sig::Adcd1n, 0);
    alt_fn!(Gp15, super::sig::M1nce3, 1);
    alt_fn!(Gp15, super::sig::Uart1rx, 2);
    alt_fn!(Gp15, super::sig::Gpio15, 3);
    alt_fn!(Gp15, super::sig::M2nce2, 4);
    alt_fn!(Gp15, super::sig::Swdio, 6);
    alt_fn!(Gp15, super::sig::Swo, 7);

pin!(GP16, Gp16, GPIO, Gpio, _GP16, GpioPin, _GPIO, 16);
    alt_fn!(Gp16, super::sig::Adcse0, 0);
    alt_fn!(Gp16, super::sig::M0nce4, 1);
    alt_fn!(Gp16, super::sig::Trig0, 2);
    alt_fn!(Gp16, super::sig::Gpio16, 3);
    alt_fn!(Gp16, super::sig::M2nce3, 4);
    alt_fn!(Gp16, super::sig::Cmpin0, 5);
    alt_fn!(Gp16, super::sig::Uart0tx, 6);
    alt_fn!(Gp16, super::sig::Ua1rts, 7);

pin!(GP17, Gp17, GPIO, Gpio, _GP17, GpioPin, _GPIO, 17);
    alt_fn!(Gp17, super::sig::Cmprf1, 0);
    alt_fn!(Gp17, super::sig::M0nce1, 1);
    alt_fn!(Gp17, super::sig::Trig1, 2);
    alt_fn!(Gp17, super::sig::Gpio17, 3);
    alt_fn!(Gp17, super::sig::M4nce3, 4);
    alt_fn!(Gp17, super::sig::Uart0rx, 6);
    alt_fn!(Gp17, super::sig::Ua1cts, 7);

pin!(GP18, Gp18, GPIO, Gpio, _GP18, GpioPin, _GPIO, 18);
    alt_fn!(Gp18, super::sig::Cmpin1, 0);
    alt_fn!(Gp18, super::sig::M0nce2, 1);
    alt_fn!(Gp18, super::sig::Tcta1, 2);
    alt_fn!(Gp18, super::sig::Gpio18, 3);
    alt_fn!(Gp18, super::sig::M4nce1, 4);
    alt_fn!(Gp18, super::sig::Uart1tx, 6);
    alt_fn!(Gp18, super::sig::Xt32khz, 7);

pin!(GP19, Gp19, GPIO, Gpio, _GP19, GpioPin, _GPIO, 19);
    alt_fn!(Gp19, super::sig::Cmprf0, 0);
    alt_fn!(Gp19, super::sig::M0nce3, 1);
    alt_fn!(Gp19, super::sig::Tctb1, 2);
    alt_fn!(Gp19, super::sig::Gpio19, 3);
    alt_fn!(Gp19, super::sig::Tcta1, 4);
    alt_fn!(Gp19, super::sig::Uart1rx, 6);
    alt_fn!(Gp19, super::sig::I2sBclk, 7);

pin!(GP20, Gp20, GPIO, Gpio, _GP20, GpioPin, _GPIO, 20);
    alt_fn!(Gp20, super::sig::Swdck, 0);
    alt_fn!(Gp20, super::sig::M1nce5, 1);
    alt_fn!(Gp20, super::sig::Tcta2, 2);
    alt_fn!(Gp20, super::sig::Gpio20, 3);
    alt_fn!(Gp20, super::sig::Uart0tx, 4);
    alt_fn!(Gp20, super::sig::Uart1tx, 5);

pin!(GP21, Gp21, GPIO, Gpio, _GP21, GpioPin, _GPIO, 21);
    alt_fn!(Gp21, super::sig::Swdio, 0);
    alt_fn!(Gp21, super::sig::M1nce6, 1);
    alt_fn!(Gp21, super::sig::Tctb2, 2);
    alt_fn!(Gp21, super::sig::Gpio21, 3);
    alt_fn!(Gp21, super::sig::Uart0rx, 4);
    alt_fn!(Gp21, super::sig::Uart1rx, 5);

pin!(GP22, Gp22, GPIO, Gpio, _GP22, GpioPin, _GPIO, 22);
    alt_fn!(Gp22, super::sig::Uart0tx, 0);
    alt_fn!(Gp22, super::sig::M1nce7, 1);
    alt_fn!(Gp22, super::sig::Tcta3, 2);
    alt_fn!(Gp22, super::sig::Gpio22, 3);
    alt_fn!(Gp22, super::sig::PdmClk, 4);
    alt_fn!(Gp22, super::sig::Tctb1, 6);
    alt_fn!(Gp22, super::sig::Swo, 7);

pin!(GP23, Gp23, GPIO, Gpio, _GP23, GpioPin, _GPIO, 23);
    alt_fn!(Gp23, super::sig::Uart0rx, 0);
    alt_fn!(Gp23, super::sig::M0nce0, 1);
    alt_fn!(Gp23, super::sig::Tctb3, 2);
    alt_fn!(Gp23, super::sig::Gpio23, 3);
    alt_fn!(Gp23, super::sig::PdmData, 4);
    alt_fn!(Gp23, super::sig::Cmpout, 5);
    alt_fn!(Gp23, super::sig::Tctb1, 6);

pin!(GP24, Gp24, GPIO, Gpio, _GP24, GpioPin, _GPIO, 24);
    alt_fn!(Gp24, super::sig::M2nce1, 0);
    alt_fn!(Gp24, super::sig::M0nce1, 1);
    alt_fn!(Gp24, super::sig::Clkout, 2);
    alt_fn!(Gp24, super::sig::Gpio24, 3);
    alt_fn!(Gp24, super::sig::M5nce0, 4);
    alt_fn!(Gp24, super::sig::Tcta1, 5);
    alt_fn!(Gp24, super::sig::I2sBclk, 6);
    alt_fn!(Gp24, super::sig::Swo, 7);

pin!(GP25, Gp25, GPIO, Gpio, _GP25, GpioPin, _GPIO, 25);
    alt_fn!(Gp25, super::sig::M0nce2, 1);
    alt_fn!(Gp25, super::sig::Tcta0, 2);
    alt_fn!(Gp25, super::sig::Gpio25, 3);
    alt_fn!(Gp25, super::sig::M2sda, 4);
    alt_fn!(Gp25, super::sig::M2miso, 5);
    alt_fn!(Gp25, super::sig::Slmisolb, 6);
    alt_fn!(Gp25, super::sig::Slsdalb, 7);

pin!(GP26, Gp26, GPIO, Gpio, _GP26, GpioPin, _GPIO, 26);
    alt_fn!(Gp26, super::sig::M0nce3, 1);
    alt_fn!(Gp26, super::sig::Tctb0, 2);
    alt_fn!(Gp26, super::sig::Gpio26, 3);
    alt_fn!(Gp26, super::sig::M2nce0, 4);
    alt_fn!(Gp26, super::sig::Tcta1, 5);
    alt_fn!(Gp26, super::sig::M5nce1, 6);
    alt_fn!(Gp26, super::sig::M3nce0, 7);

pin!(GP27, Gp27, GPIO, Gpio, _GP27, GpioPin, _GPIO, 27);
    alt_fn!(Gp27, super::sig::M1nce4, 1);
    alt_fn!(Gp27, super::sig::Tcta1, 2);
    alt_fn!(Gp27, super::sig::Gpio27, 3);
    alt_fn!(Gp27, super::sig::M2scl, 4);
    alt_fn!(Gp27, super::sig::M2sck, 5);
    alt_fn!(Gp27, super::sig::M2scklb, 6);
    alt_fn!(Gp27, super::sig::M2scllb, 7);

pin!(GP28, Gp28, GPIO, Gpio, _GP28, GpioPin, _GPIO, 28);
    alt_fn!(Gp28, super::sig::I2sWclk, 0);
    alt_fn!(Gp28, super::sig::M1nce5, 1);
    alt_fn!(Gp28, super::sig::Tctb1, 2);
    alt_fn!(Gp28, super::sig::Gpio28, 3);
    alt_fn!(Gp28, super::sig::M2wir3, 4);
    alt_fn!(Gp28, super::sig::M2mosi, 5);
    alt_fn!(Gp28, super::sig::M5nce3, 6);
    alt_fn!(Gp28, super::sig::Slwir3lb, 7);

pin!(GP29, Gp29, GPIO, Gpio, _GP29, GpioPin, _GPIO, 29);
    alt_fn!(Gp29, super::sig::Adcse1, 0);
    alt_fn!(Gp29, super::sig::M1nce6, 1);
    alt_fn!(Gp29, super::sig::Tcta2, 2);
    alt_fn!(Gp29, super::sig::Gpio29, 3);
    alt_fn!(Gp29, super::sig::Ua0cts, 4);
    alt_fn!(Gp29, super::sig::Ua1cts, 5);
    alt_fn!(Gp29, super::sig::M4nce0, 6);
    alt_fn!(Gp29, super::sig::PdmData, 7);

pin!(GP30, Gp30, GPIO, Gpio, _GP30, GpioPin, _GPIO, 30);
    alt_fn!(Gp30, super::sig::M1nce7, 1);
    alt_fn!(Gp30, super::sig::Tctb2, 2);
    alt_fn!(Gp30, super::sig::Gpio30, 3);
    alt_fn!(Gp30, super::sig::Uart0tx, 4);
    alt_fn!(Gp30, super::sig::Ua1rts, 5);
    alt_fn!(Gp30, super::sig::I2sDat, 7);

pin!(GP31, Gp31, GPIO, Gpio, _GP31, GpioPin, _GPIO, 31);
    alt_fn!(Gp31, super::sig::Adcse3, 0);
    alt_fn!(Gp31, super::sig::M0nce4, 1);
    alt_fn!(Gp31, super::sig::Tcta3, 2);
    alt_fn!(Gp31, super::sig::Gpio31, 3);
    alt_fn!(Gp31, super::sig::Uart0rx, 4);
    alt_fn!(Gp31, super::sig::Tctb1, 5);

pin!(GP32, Gp32, GPIO, Gpio, _GP32, GpioPin, _GPIO, 32);
    alt_fn!(Gp32, super::sig::Adcse4, 0);
    alt_fn!(Gp32, super::sig::M0nce5, 1);
    alt_fn!(Gp32, super::sig::Tctb3, 2);
    alt_fn!(Gp32, super::sig::Gpio32, 3);
    alt_fn!(Gp32, super::sig::Tctb1, 5);

pin!(GP33, Gp33, GPIO, Gpio, _GP33, GpioPin, _GPIO, 33);
    alt_fn!(Gp33, super::sig::Adcse5, 0);
    alt_fn!(Gp33, super::sig::M0nce6, 1);
    alt_fn!(Gp33, super::sig::Xt32khz, 2);
    alt_fn!(Gp33, super::sig::Gpio33, 3);
    alt_fn!(Gp33, super::sig::M3nce7, 5);
    alt_fn!(Gp33, super::sig::Tctb1, 6);
    alt_fn!(Gp33, super::sig::Swo, 7);

pin!(GP34, Gp34, GPIO, Gpio, _GP34, GpioPin, _GPIO, 34);
    alt_fn!(Gp34, super::sig::Adcse6, 0);
    alt_fn!(Gp34, super::sig::M0nce7, 1);
    alt_fn!(Gp34, super::sig::M2nce3, 2);
    alt_fn!(Gp34, super::sig::Gpio34, 3);
    alt_fn!(Gp34, super::sig::Cmprf2, 4);
    alt_fn!(Gp34, super::sig::M3nce1, 5);
    alt_fn!(Gp34, super::sig::M4nce0, 6);
    alt_fn!(Gp34, super::sig::M5nce2, 7);

pin!(GP35, Gp35, GPIO, Gpio, _GP35, GpioPin, _GPIO, 35);
    alt_fn!(Gp35, super::sig::Adcse7, 0);
    alt_fn!(Gp35, super::sig::M1nce0, 1);
    alt_fn!(Gp35, super::sig::Uart1tx, 2);
    alt_fn!(Gp35, super::sig::Gpio35, 3);
    alt_fn!(Gp35, super::sig::M4nce6, 4);
    alt_fn!(Gp35, super::sig::Tcta1, 5);
    alt_fn!(Gp35, super::sig::Ua0rts, 6);
    alt_fn!(Gp35, super::sig::M3nce2, 7);

pin!(GP36, Gp36, GPIO, Gpio, _GP36, GpioPin, _GPIO, 36);
    alt_fn!(Gp36, super::sig::Trig1, 0);
    alt_fn!(Gp36, super::sig::M1nce1, 1);
    alt_fn!(Gp36, super::sig::Uart1rx, 2);
    alt_fn!(Gp36, super::sig::Gpio36, 3);
    alt_fn!(Gp36, super::sig::Xt32khz, 4);
    alt_fn!(Gp36, super::sig::M2nce0, 5);
    alt_fn!(Gp36, super::sig::Ua0cts, 6);
    alt_fn!(Gp36, super::sig::M3nce3, 7);

pin!(GP37, Gp37, GPIO, Gpio, _GP37, GpioPin, _GPIO, 37);
    alt_fn!(Gp37, super::sig::Trig2, 0);
    alt_fn!(Gp37, super::sig::M1nce2, 1);
    alt_fn!(Gp37, super::sig::Ua0rts, 2);
    alt_fn!(Gp37, super::sig::Gpio37, 3);
    alt_fn!(Gp37, super::sig::M3nce4, 4);
    alt_fn!(Gp37, super::sig::M4nce1, 5);
    alt_fn!(Gp37, super::sig::PdmClk, 6);
    alt_fn!(Gp37, super::sig::Tcta1, 7);

pin!(GP38, Gp38, GPIO, Gpio, _GP38, GpioPin, _GPIO, 38);
    alt_fn!(Gp38, super::sig::Trig3, 0);
    alt_fn!(Gp38, super::sig::M1nce3, 1);
    alt_fn!(Gp38, super::sig::Ua0cts, 2);
    alt_fn!(Gp38, super::sig::Gpio38, 3);
    alt_fn!(Gp38, super::sig::M3wir3, 4);
    alt_fn!(Gp38, super::sig::M3mosi, 5);
    alt_fn!(Gp38, super::sig::M4nce7, 6);
    alt_fn!(Gp38, super::sig::Slwir3lb, 7);

pin!(GP39, Gp39, GPIO, Gpio, _GP39, GpioPin, _GPIO, 39);
    alt_fn!(Gp39, super::sig::Uart0tx, 0);
    alt_fn!(Gp39, super::sig::Uart1tx, 1);
    alt_fn!(Gp39, super::sig::Clkout, 2);
    alt_fn!(Gp39, super::sig::Gpio39, 3);
    alt_fn!(Gp39, super::sig::M4scl, 4);
    alt_fn!(Gp39, super::sig::M4sck, 5);
    alt_fn!(Gp39, super::sig::M4scklb, 6);
    alt_fn!(Gp39, super::sig::M4scllb, 7);

pin!(GP40, Gp40, GPIO, Gpio, _GP40, GpioPin, _GPIO, 40);
    alt_fn!(Gp40, super::sig::Uart0rx, 0);
    alt_fn!(Gp40, super::sig::Uart1rx, 1);
    alt_fn!(Gp40, super::sig::Trig0, 2);
    alt_fn!(Gp40, super::sig::Gpio40, 3);
    alt_fn!(Gp40, super::sig::M4sda, 4);
    alt_fn!(Gp40, super::sig::M4miso, 5);
    alt_fn!(Gp40, super::sig::Slmisolb, 6);
    alt_fn!(Gp40, super::sig::Slsdalb, 7);

pin!(GP41, Gp41, GPIO, Gpio, _GP41, GpioPin, _GPIO, 41);
    alt_fn!(Gp41, super::sig::M2nce1, 0);
    alt_fn!(Gp41, super::sig::Clkout, 1);
    alt_fn!(Gp41, super::sig::Swo, 2);
    alt_fn!(Gp41, super::sig::Gpio41, 3);
    alt_fn!(Gp41, super::sig::M3nce5, 4);
    alt_fn!(Gp41, super::sig::M5nce7, 5);
    alt_fn!(Gp41, super::sig::M4nce2, 6);
    alt_fn!(Gp41, super::sig::Ua0rts, 7);

pin!(GP42, Gp42, GPIO, Gpio, _GP42, GpioPin, _GPIO, 42);
    alt_fn!(Gp42, super::sig::M2nce2, 0);
    alt_fn!(Gp42, super::sig::M0nce0, 1);
    alt_fn!(Gp42, super::sig::Tcta0, 2);
    alt_fn!(Gp42, super::sig::Gpio42, 3);
    alt_fn!(Gp42, super::sig::M3scl, 4);
    alt_fn!(Gp42, super::sig::M3sck, 5);
    alt_fn!(Gp42, super::sig::M3scklb, 6);
    alt_fn!(Gp42, super::sig::M3scllb, 7);

pin!(GP43, Gp43, GPIO, Gpio, _GP43, GpioPin, _GPIO, 43);
    alt_fn!(Gp43, super::sig::M2nce4, 0);
    alt_fn!(Gp43, super::sig::M0nce1, 1);
    alt_fn!(Gp43, super::sig::Tctb0, 2);
    alt_fn!(Gp43, super::sig::Gpio43, 3);
    alt_fn!(Gp43, super::sig::M3sda, 4);
    alt_fn!(Gp43, super::sig::M3miso, 5);
    alt_fn!(Gp43, super::sig::Slmisolb, 6);
    alt_fn!(Gp43, super::sig::Slsdalb, 7);

pin!(GP44, Gp44, GPIO, Gpio, _GP44, GpioPin, _GPIO, 44);
    alt_fn!(Gp44, super::sig::Ua1rts, 0);
    alt_fn!(Gp44, super::sig::M0nce2, 1);
    alt_fn!(Gp44, super::sig::Tcta1, 2);
    alt_fn!(Gp44, super::sig::Gpio44, 3);
    alt_fn!(Gp44, super::sig::M4wir3, 4);
    alt_fn!(Gp44, super::sig::M4mosi, 5);
    alt_fn!(Gp44, super::sig::M5nce6, 6);
    alt_fn!(Gp44, super::sig::Slwir3lb, 7);

pin!(GP45, Gp45, GPIO, Gpio, _GP45, GpioPin, _GPIO, 45);
    alt_fn!(Gp45, super::sig::Ua1cts, 0);
    alt_fn!(Gp45, super::sig::M0nce3, 1);
    alt_fn!(Gp45, super::sig::Tctb1, 2);
    alt_fn!(Gp45, super::sig::Gpio45, 3);
    alt_fn!(Gp45, super::sig::M4nce3, 4);
    alt_fn!(Gp45, super::sig::M3nce6, 5);
    alt_fn!(Gp45, super::sig::M5nce5, 6);
    alt_fn!(Gp45, super::sig::Swo, 7);

pin!(GP46, Gp46, GPIO, Gpio, _GP46, GpioPin, _GPIO, 46);
    alt_fn!(Gp46, super::sig::Xt32khz, 0);
    alt_fn!(Gp46, super::sig::M0nce4, 1);
    alt_fn!(Gp46, super::sig::Tcta2, 2);
    alt_fn!(Gp46, super::sig::Gpio46, 3);
    alt_fn!(Gp46, super::sig::Tcta1, 4);
    alt_fn!(Gp46, super::sig::M5nce4, 5);
    alt_fn!(Gp46, super::sig::M4nce4, 6);
    alt_fn!(Gp46, super::sig::Swo, 7);

pin!(GP47, Gp47, GPIO, Gpio, _GP47, GpioPin, _GPIO, 47);
    alt_fn!(Gp47, super::sig::M2nce5, 0);
    alt_fn!(Gp47, super::sig::M0nce5, 1);
    alt_fn!(Gp47, super::sig::Tctb2, 2);
    alt_fn!(Gp47, super::sig::Gpio47, 3);
    alt_fn!(Gp47, super::sig::M5wir3, 4);
    alt_fn!(Gp47, super::sig::M5mosi, 5);
    alt_fn!(Gp47, super::sig::M4nce5, 6);
    alt_fn!(Gp47, super::sig::Slwir3lb, 7);

pin!(GP48, Gp48, GPIO, Gpio, _GP48, GpioPin, _GPIO, 48);
    alt_fn!(Gp48, super::sig::M2nce6, 0);
    alt_fn!(Gp48, super::sig::M0nce6, 1);
    alt_fn!(Gp48, super::sig::Tcta3, 2);
    alt_fn!(Gp48, super::sig::Gpio48, 3);
    alt_fn!(Gp48, super::sig::M5scl, 4);
    alt_fn!(Gp48, super::sig::M5sck, 5);
    alt_fn!(Gp48, super::sig::M5scklb, 6);
    alt_fn!(Gp48, super::sig::M5scllb, 7);

pin!(GP49, Gp49, GPIO, Gpio, _GP49, GpioPin, _GPIO, 49);
    alt_fn!(Gp49, super::sig::M2nce7, 0);
    alt_fn!(Gp49, super::sig::M0nce7, 1);
    alt_fn!(Gp49, super::sig::Tctb3, 2);
    alt_fn!(Gp49, super::sig::Gpio49, 3);
    alt_fn!(Gp49, super::sig::M5sda, 4);
    alt_fn!(Gp49, super::sig::M5miso, 5);
    alt_fn!(Gp49, super::sig::Slmisolb, 6);
    alt_fn!(Gp49, super::sig::Slsdalb, 7);


