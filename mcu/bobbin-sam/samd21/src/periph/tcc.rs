#[allow(unused_imports)] use ::bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TCC Peripheral"]
pub struct TccPeriph(pub usize); 

pub struct TccCh { pub periph: TccPeriph, pub index: usize }

impl TccPeriph {
    #[doc="Get the *mut pointer for the CC register."]
    #[inline] pub fn cc_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Cc { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x44 + (index << 2)) as *mut Cc
    }

    #[doc="Get the *const pointer for the CC register."]
    #[inline] pub fn cc_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Cc { 
           self.cc_mut(index)
    }

    #[doc="Read the CC register."]
    #[inline] pub fn cc<I: Into<bits::R4>>(&self, index: I) -> Cc { 
        unsafe {
            read_volatile(self.cc_ptr(index))
        }
    }

    #[doc="Write the CC register."]
    #[inline] pub fn set_cc<I: Into<bits::R4>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(index), f(Cc(0)));
        }
        self
    }

    #[doc="Modify the CC register."]
    #[inline] pub fn with_cc<I: Into<bits::R4> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(index), f(self.cc(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CCB register."]
    #[inline] pub fn ccb_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Ccb { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x70 + (index << 2)) as *mut Ccb
    }

    #[doc="Get the *const pointer for the CCB register."]
    #[inline] pub fn ccb_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Ccb { 
           self.ccb_mut(index)
    }

    #[doc="Read the CCB register."]
    #[inline] pub fn ccb<I: Into<bits::R4>>(&self, index: I) -> Ccb { 
        unsafe {
            read_volatile(self.ccb_ptr(index))
        }
    }

    #[doc="Write the CCB register."]
    #[inline] pub fn set_ccb<I: Into<bits::R4>, F: FnOnce(Ccb) -> Ccb>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccb_mut(index), f(Ccb(0)));
        }
        self
    }

    #[doc="Modify the CCB register."]
    #[inline] pub fn with_ccb<I: Into<bits::R4> + Copy, F: FnOnce(Ccb) -> Ccb>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccb_mut(index), f(self.ccb(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the COUNT register."]
    #[inline] pub fn count_mut(&self) -> *mut Count { 
        (self.0 + 0x34) as *mut Count
    }

    #[doc="Get the *const pointer for the COUNT register."]
    #[inline] pub fn count_ptr(&self) -> *const Count { 
           self.count_mut()
    }

    #[doc="Read the COUNT register."]
    #[inline] pub fn count(&self) -> Count { 
        unsafe {
            read_volatile(self.count_ptr())
        }
    }

    #[doc="Write the COUNT register."]
    #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.count_mut(), f(Count(0)));
        }
        self
    }

    #[doc="Modify the COUNT register."]
    #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.count_mut(), f(self.count()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLA register."]
    #[inline] pub fn ctrla_mut(&self) -> *mut Ctrla { 
        (self.0 + 0x0) as *mut Ctrla
    }

    #[doc="Get the *const pointer for the CTRLA register."]
    #[inline] pub fn ctrla_ptr(&self) -> *const Ctrla { 
           self.ctrla_mut()
    }

    #[doc="Read the CTRLA register."]
    #[inline] pub fn ctrla(&self) -> Ctrla { 
        unsafe {
            read_volatile(self.ctrla_ptr())
        }
    }

    #[doc="Write the CTRLA register."]
    #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrla_mut(), f(Ctrla(0)));
        }
        self
    }

    #[doc="Modify the CTRLA register."]
    #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrla_mut(), f(self.ctrla()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr_mut(&self) -> *mut Ctrlbclr { 
        (self.0 + 0x4) as *mut Ctrlbclr
    }

    #[doc="Get the *const pointer for the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr_ptr(&self) -> *const Ctrlbclr { 
           self.ctrlbclr_mut()
    }

    #[doc="Read the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
        unsafe {
            read_volatile(self.ctrlbclr_ptr())
        }
    }

    #[doc="Write the CTRLBCLR register."]
    #[inline] pub fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbclr_mut(), f(Ctrlbclr(0)));
        }
        self
    }

    #[doc="Modify the CTRLBCLR register."]
    #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbclr_mut(), f(self.ctrlbclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLBSET register."]
    #[inline] pub fn ctrlbset_mut(&self) -> *mut Ctrlbset { 
        (self.0 + 0x5) as *mut Ctrlbset
    }

    #[doc="Get the *const pointer for the CTRLBSET register."]
    #[inline] pub fn ctrlbset_ptr(&self) -> *const Ctrlbset { 
           self.ctrlbset_mut()
    }

    #[doc="Read the CTRLBSET register."]
    #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
        unsafe {
            read_volatile(self.ctrlbset_ptr())
        }
    }

    #[doc="Write the CTRLBSET register."]
    #[inline] pub fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbset_mut(), f(Ctrlbset(0)));
        }
        self
    }

    #[doc="Modify the CTRLBSET register."]
    #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbset_mut(), f(self.ctrlbset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_mut(&self) -> *mut Dbgctrl { 
        (self.0 + 0x1e) as *mut Dbgctrl
    }

    #[doc="Get the *const pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_ptr(&self) -> *const Dbgctrl { 
           self.dbgctrl_mut()
    }

    #[doc="Read the DBGCTRL register."]
    #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
        unsafe {
            read_volatile(self.dbgctrl_ptr())
        }
    }

    #[doc="Write the DBGCTRL register."]
    #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgctrl_mut(), f(Dbgctrl(0)));
        }
        self
    }

    #[doc="Modify the DBGCTRL register."]
    #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgctrl_mut(), f(self.dbgctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DRVCTRL register."]
    #[inline] pub fn drvctrl_mut(&self) -> *mut Drvctrl { 
        (self.0 + 0x18) as *mut Drvctrl
    }

    #[doc="Get the *const pointer for the DRVCTRL register."]
    #[inline] pub fn drvctrl_ptr(&self) -> *const Drvctrl { 
           self.drvctrl_mut()
    }

    #[doc="Read the DRVCTRL register."]
    #[inline] pub fn drvctrl(&self) -> Drvctrl { 
        unsafe {
            read_volatile(self.drvctrl_ptr())
        }
    }

    #[doc="Write the DRVCTRL register."]
    #[inline] pub fn set_drvctrl<F: FnOnce(Drvctrl) -> Drvctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.drvctrl_mut(), f(Drvctrl(0)));
        }
        self
    }

    #[doc="Modify the DRVCTRL register."]
    #[inline] pub fn with_drvctrl<F: FnOnce(Drvctrl) -> Drvctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.drvctrl_mut(), f(self.drvctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        (self.0 + 0x20) as *mut Evctrl
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
           self.evctrl_mut()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        unsafe {
            read_volatile(self.evctrl_ptr())
        }
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.evctrl_mut(), f(Evctrl(0)));
        }
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.evctrl_mut(), f(self.evctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FCTRLA register."]
    #[inline] pub fn fctrla_mut(&self) -> *mut Fctrla { 
        (self.0 + 0xc) as *mut Fctrla
    }

    #[doc="Get the *const pointer for the FCTRLA register."]
    #[inline] pub fn fctrla_ptr(&self) -> *const Fctrla { 
           self.fctrla_mut()
    }

    #[doc="Read the FCTRLA register."]
    #[inline] pub fn fctrla(&self) -> Fctrla { 
        unsafe {
            read_volatile(self.fctrla_ptr())
        }
    }

    #[doc="Write the FCTRLA register."]
    #[inline] pub fn set_fctrla<F: FnOnce(Fctrla) -> Fctrla>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fctrla_mut(), f(Fctrla(0)));
        }
        self
    }

    #[doc="Modify the FCTRLA register."]
    #[inline] pub fn with_fctrla<F: FnOnce(Fctrla) -> Fctrla>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fctrla_mut(), f(self.fctrla()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FCTRLB register."]
    #[inline] pub fn fctrlb_mut(&self) -> *mut Fctrlb { 
        (self.0 + 0x10) as *mut Fctrlb
    }

    #[doc="Get the *const pointer for the FCTRLB register."]
    #[inline] pub fn fctrlb_ptr(&self) -> *const Fctrlb { 
           self.fctrlb_mut()
    }

    #[doc="Read the FCTRLB register."]
    #[inline] pub fn fctrlb(&self) -> Fctrlb { 
        unsafe {
            read_volatile(self.fctrlb_ptr())
        }
    }

    #[doc="Write the FCTRLB register."]
    #[inline] pub fn set_fctrlb<F: FnOnce(Fctrlb) -> Fctrlb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fctrlb_mut(), f(Fctrlb(0)));
        }
        self
    }

    #[doc="Modify the FCTRLB register."]
    #[inline] pub fn with_fctrlb<F: FnOnce(Fctrlb) -> Fctrlb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fctrlb_mut(), f(self.fctrlb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        (self.0 + 0x24) as *mut Intenclr
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
           self.intenclr_mut()
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        unsafe {
            read_volatile(self.intenclr_ptr())
        }
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenclr_mut(), f(Intenclr(0)));
        }
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenclr_mut(), f(self.intenclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        (self.0 + 0x28) as *mut Intenset
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
           self.intenset_mut()
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        unsafe {
            read_volatile(self.intenset_ptr())
        }
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenset_mut(), f(Intenset(0)));
        }
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenset_mut(), f(self.intenset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        (self.0 + 0x2c) as *mut Intflag
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
           self.intflag_mut()
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        unsafe {
            read_volatile(self.intflag_ptr())
        }
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intflag_mut(), f(Intflag(0)));
        }
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intflag_mut(), f(self.intflag()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PATT register."]
    #[inline] pub fn patt_mut(&self) -> *mut Patt { 
        (self.0 + 0x38) as *mut Patt
    }

    #[doc="Get the *const pointer for the PATT register."]
    #[inline] pub fn patt_ptr(&self) -> *const Patt { 
           self.patt_mut()
    }

    #[doc="Read the PATT register."]
    #[inline] pub fn patt(&self) -> Patt { 
        unsafe {
            read_volatile(self.patt_ptr())
        }
    }

    #[doc="Write the PATT register."]
    #[inline] pub fn set_patt<F: FnOnce(Patt) -> Patt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.patt_mut(), f(Patt(0)));
        }
        self
    }

    #[doc="Modify the PATT register."]
    #[inline] pub fn with_patt<F: FnOnce(Patt) -> Patt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.patt_mut(), f(self.patt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PATTB register."]
    #[inline] pub fn pattb_mut(&self) -> *mut Pattb { 
        (self.0 + 0x64) as *mut Pattb
    }

    #[doc="Get the *const pointer for the PATTB register."]
    #[inline] pub fn pattb_ptr(&self) -> *const Pattb { 
           self.pattb_mut()
    }

    #[doc="Read the PATTB register."]
    #[inline] pub fn pattb(&self) -> Pattb { 
        unsafe {
            read_volatile(self.pattb_ptr())
        }
    }

    #[doc="Write the PATTB register."]
    #[inline] pub fn set_pattb<F: FnOnce(Pattb) -> Pattb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pattb_mut(), f(Pattb(0)));
        }
        self
    }

    #[doc="Modify the PATTB register."]
    #[inline] pub fn with_pattb<F: FnOnce(Pattb) -> Pattb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pattb_mut(), f(self.pattb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PER register."]
    #[inline] pub fn per_mut(&self) -> *mut Per { 
        (self.0 + 0x40) as *mut Per
    }

    #[doc="Get the *const pointer for the PER register."]
    #[inline] pub fn per_ptr(&self) -> *const Per { 
           self.per_mut()
    }

    #[doc="Read the PER register."]
    #[inline] pub fn per(&self) -> Per { 
        unsafe {
            read_volatile(self.per_ptr())
        }
    }

    #[doc="Write the PER register."]
    #[inline] pub fn set_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.per_mut(), f(Per(0)));
        }
        self
    }

    #[doc="Modify the PER register."]
    #[inline] pub fn with_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.per_mut(), f(self.per()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PERB register."]
    #[inline] pub fn perb_mut(&self) -> *mut Perb { 
        (self.0 + 0x6c) as *mut Perb
    }

    #[doc="Get the *const pointer for the PERB register."]
    #[inline] pub fn perb_ptr(&self) -> *const Perb { 
           self.perb_mut()
    }

    #[doc="Read the PERB register."]
    #[inline] pub fn perb(&self) -> Perb { 
        unsafe {
            read_volatile(self.perb_ptr())
        }
    }

    #[doc="Write the PERB register."]
    #[inline] pub fn set_perb<F: FnOnce(Perb) -> Perb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.perb_mut(), f(Perb(0)));
        }
        self
    }

    #[doc="Modify the PERB register."]
    #[inline] pub fn with_perb<F: FnOnce(Perb) -> Perb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.perb_mut(), f(self.perb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0x30) as *mut Status
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
           self.status_mut()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            read_volatile(self.status_ptr())
        }
    }

    #[doc="Write the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.status_mut(), f(Status(0)));
        }
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.status_mut(), f(self.status()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_mut(&self) -> *mut Syncbusy { 
        (self.0 + 0x8) as *mut Syncbusy
    }

    #[doc="Get the *const pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_ptr(&self) -> *const Syncbusy { 
           self.syncbusy_mut()
    }

    #[doc="Read the SYNCBUSY register."]
    #[inline] pub fn syncbusy(&self) -> Syncbusy { 
        unsafe {
            read_volatile(self.syncbusy_ptr())
        }
    }

    #[doc="Get the *mut pointer for the WAVE register."]
    #[inline] pub fn wave_mut(&self) -> *mut Wave { 
        (self.0 + 0x3c) as *mut Wave
    }

    #[doc="Get the *const pointer for the WAVE register."]
    #[inline] pub fn wave_ptr(&self) -> *const Wave { 
           self.wave_mut()
    }

    #[doc="Read the WAVE register."]
    #[inline] pub fn wave(&self) -> Wave { 
        unsafe {
            read_volatile(self.wave_ptr())
        }
    }

    #[doc="Write the WAVE register."]
    #[inline] pub fn set_wave<F: FnOnce(Wave) -> Wave>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wave_mut(), f(Wave(0)));
        }
        self
    }

    #[doc="Modify the WAVE register."]
    #[inline] pub fn with_wave<F: FnOnce(Wave) -> Wave>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wave_mut(), f(self.wave()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WAVEB register."]
    #[inline] pub fn waveb_mut(&self) -> *mut Waveb { 
        (self.0 + 0x68) as *mut Waveb
    }

    #[doc="Get the *const pointer for the WAVEB register."]
    #[inline] pub fn waveb_ptr(&self) -> *const Waveb { 
           self.waveb_mut()
    }

    #[doc="Read the WAVEB register."]
    #[inline] pub fn waveb(&self) -> Waveb { 
        unsafe {
            read_volatile(self.waveb_ptr())
        }
    }

    #[doc="Write the WAVEB register."]
    #[inline] pub fn set_waveb<F: FnOnce(Waveb) -> Waveb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.waveb_mut(), f(Waveb(0)));
        }
        self
    }

    #[doc="Modify the WAVEB register."]
    #[inline] pub fn with_waveb<F: FnOnce(Waveb) -> Waveb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.waveb_mut(), f(self.waveb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WEXCTRL register."]
    #[inline] pub fn wexctrl_mut(&self) -> *mut Wexctrl { 
        (self.0 + 0x14) as *mut Wexctrl
    }

    #[doc="Get the *const pointer for the WEXCTRL register."]
    #[inline] pub fn wexctrl_ptr(&self) -> *const Wexctrl { 
           self.wexctrl_mut()
    }

    #[doc="Read the WEXCTRL register."]
    #[inline] pub fn wexctrl(&self) -> Wexctrl { 
        unsafe {
            read_volatile(self.wexctrl_ptr())
        }
    }

    #[doc="Write the WEXCTRL register."]
    #[inline] pub fn set_wexctrl<F: FnOnce(Wexctrl) -> Wexctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wexctrl_mut(), f(Wexctrl(0)));
        }
        self
    }

    #[doc="Modify the WEXCTRL register."]
    #[inline] pub fn with_wexctrl<F: FnOnce(Wexctrl) -> Wexctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wexctrl_mut(), f(self.wexctrl()));
        }
        self
    }

}

#[doc="Compare and Capture"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc="Compare and Capture value"]
    #[inline] pub fn cc(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if CC != 0"]
    #[inline] pub fn test_cc(&self) -> bool {
        self.cc() != 0
    }

    #[doc="Sets the CC field."]
    #[inline] pub fn set_cc<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cc {
    #[inline]
    fn from(other: u32) -> Self {
         Cc(other)
    }
}

impl ::core::fmt::Display for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare and Capture Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccb(pub u32);
impl Ccb {
    #[doc="Compare and Capture buffer value"]
    #[inline] pub fn ccb(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if CCB != 0"]
    #[inline] pub fn test_ccb(&self) -> bool {
        self.ccb() != 0
    }

    #[doc="Sets the CCB field."]
    #[inline] pub fn set_ccb<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccb {
    #[inline]
    fn from(other: u32) -> Self {
         Ccb(other)
    }
}

impl ::core::fmt::Display for Ccb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccb() != 0 { try!(write!(f, " ccb=0x{:x}", self.ccb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
    #[doc="Count Value"]
    #[inline] pub fn count(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Count {
    #[inline]
    fn from(other: u32) -> Self {
         Count(other)
    }
}

impl ::core::fmt::Display for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enhanced Resolution"]
    #[inline] pub fn resolution(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if RESOLUTION != 0"]
    #[inline] pub fn test_resolution(&self) -> bool {
        self.resolution() != 0
    }

    #[doc="Sets the RESOLUTION field."]
    #[inline] pub fn set_resolution<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Prescaler"]
    #[inline] pub fn prescaler(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Prescaler and Counter Synchronization Selection"]
    #[inline] pub fn prescsync(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if PRESCSYNC != 0"]
    #[inline] pub fn test_prescsync(&self) -> bool {
        self.prescsync() != 0
    }

    #[doc="Sets the PRESCSYNC field."]
    #[inline] pub fn set_prescsync<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Auto Lock"]
    #[inline] pub fn alock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ALOCK != 0"]
    #[inline] pub fn test_alock(&self) -> bool {
        self.alock() != 0
    }

    #[doc="Sets the ALOCK field."]
    #[inline] pub fn set_alock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Master Synchronization"]
    #[inline] pub fn msync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if MSYNC != 0"]
    #[inline] pub fn test_msync(&self) -> bool {
        self.msync() != 0
    }

    #[doc="Sets the MSYNC field."]
    #[inline] pub fn set_msync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Capture Channel n Enable"]
    #[inline] pub fn cpten<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 24 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CPTEN != 0"]
    #[inline] pub fn test_cpten<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.cpten(index) != 0
    }

    #[doc="Sets the CPTEN field."]
    #[inline] pub fn set_cpten<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 24 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ctrla {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrla(other)
    }
}

impl ::core::fmt::Display for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.resolution() != 0 { try!(write!(f, " resolution=0x{:x}", self.resolution()))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
        if self.alock() != 0 { try!(write!(f, " alock"))}
        if self.msync() != 0 { try!(write!(f, " msync"))}
        if self.cpten(0) != 0 { try!(write!(f, " cpten[0]"))}
        if self.cpten(1) != 0 { try!(write!(f, " cpten[1]"))}
        if self.cpten(2) != 0 { try!(write!(f, " cpten[2]"))}
        if self.cpten(3) != 0 { try!(write!(f, " cpten[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
    #[doc="Counter Direction"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Lock Update"]
    #[inline] pub fn lupd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LUPD != 0"]
    #[inline] pub fn test_lupd(&self) -> bool {
        self.lupd() != 0
    }

    #[doc="Sets the LUPD field."]
    #[inline] pub fn set_lupd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="One-Shot"]
    #[inline] pub fn oneshot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ONESHOT != 0"]
    #[inline] pub fn test_oneshot(&self) -> bool {
        self.oneshot() != 0
    }

    #[doc="Sets the ONESHOT field."]
    #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Ramp Index Command"]
    #[inline] pub fn idxcmd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if IDXCMD != 0"]
    #[inline] pub fn test_idxcmd(&self) -> bool {
        self.idxcmd() != 0
    }

    #[doc="Sets the IDXCMD field."]
    #[inline] pub fn set_idxcmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TCC Command"]
    #[inline] pub fn cmd(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Ctrlbclr {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlbclr(other)
    }
}

impl ::core::fmt::Display for Ctrlbclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlbclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.lupd() != 0 { try!(write!(f, " lupd"))}
        if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
        if self.idxcmd() != 0 { try!(write!(f, " idxcmd=0x{:x}", self.idxcmd()))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
    #[doc="Counter Direction"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Lock update"]
    #[inline] pub fn lupd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LUPD != 0"]
    #[inline] pub fn test_lupd(&self) -> bool {
        self.lupd() != 0
    }

    #[doc="Sets the LUPD field."]
    #[inline] pub fn set_lupd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="One-Shot"]
    #[inline] pub fn oneshot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ONESHOT != 0"]
    #[inline] pub fn test_oneshot(&self) -> bool {
        self.oneshot() != 0
    }

    #[doc="Sets the ONESHOT field."]
    #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Ramp Index Command"]
    #[inline] pub fn idxcmd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if IDXCMD != 0"]
    #[inline] pub fn test_idxcmd(&self) -> bool {
        self.idxcmd() != 0
    }

    #[doc="Sets the IDXCMD field."]
    #[inline] pub fn set_idxcmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TCC Command"]
    #[inline] pub fn cmd(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Ctrlbset {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlbset(other)
    }
}

impl ::core::fmt::Display for Ctrlbset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlbset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.lupd() != 0 { try!(write!(f, " lupd"))}
        if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
        if self.idxcmd() != 0 { try!(write!(f, " idxcmd=0x{:x}", self.idxcmd()))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Running Mode"]
    #[inline] pub fn dbgrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGRUN != 0"]
    #[inline] pub fn test_dbgrun(&self) -> bool {
        self.dbgrun() != 0
    }

    #[doc="Sets the DBGRUN field."]
    #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fault Detection on Debug Break Detection"]
    #[inline] pub fn fddbd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FDDBD != 0"]
    #[inline] pub fn test_fddbd(&self) -> bool {
        self.fddbd() != 0
    }

    #[doc="Sets the FDDBD field."]
    #[inline] pub fn set_fddbd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Dbgctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Dbgctrl(other)
    }
}

impl ::core::fmt::Display for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
        if self.fddbd() != 0 { try!(write!(f, " fddbd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Driver Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Drvctrl(pub u32);
impl Drvctrl {
    #[doc="Non-Recoverable State 0 Output Enable"]
    #[inline] pub fn nre0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if NRE0 != 0"]
    #[inline] pub fn test_nre0(&self) -> bool {
        self.nre0() != 0
    }

    #[doc="Sets the NRE0 field."]
    #[inline] pub fn set_nre0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Non-Recoverable State 1 Output Enable"]
    #[inline] pub fn nre1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if NRE1 != 0"]
    #[inline] pub fn test_nre1(&self) -> bool {
        self.nre1() != 0
    }

    #[doc="Sets the NRE1 field."]
    #[inline] pub fn set_nre1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Non-Recoverable State 2 Output Enable"]
    #[inline] pub fn nre2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NRE2 != 0"]
    #[inline] pub fn test_nre2(&self) -> bool {
        self.nre2() != 0
    }

    #[doc="Sets the NRE2 field."]
    #[inline] pub fn set_nre2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Non-Recoverable State 3 Output Enable"]
    #[inline] pub fn nre3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if NRE3 != 0"]
    #[inline] pub fn test_nre3(&self) -> bool {
        self.nre3() != 0
    }

    #[doc="Sets the NRE3 field."]
    #[inline] pub fn set_nre3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Non-Recoverable State 4 Output Enable"]
    #[inline] pub fn nre4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NRE4 != 0"]
    #[inline] pub fn test_nre4(&self) -> bool {
        self.nre4() != 0
    }

    #[doc="Sets the NRE4 field."]
    #[inline] pub fn set_nre4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Non-Recoverable State 5 Output Enable"]
    #[inline] pub fn nre5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if NRE5 != 0"]
    #[inline] pub fn test_nre5(&self) -> bool {
        self.nre5() != 0
    }

    #[doc="Sets the NRE5 field."]
    #[inline] pub fn set_nre5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Non-Recoverable State 6 Output Enable"]
    #[inline] pub fn nre6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NRE6 != 0"]
    #[inline] pub fn test_nre6(&self) -> bool {
        self.nre6() != 0
    }

    #[doc="Sets the NRE6 field."]
    #[inline] pub fn set_nre6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Non-Recoverable State 7 Output Enable"]
    #[inline] pub fn nre7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NRE7 != 0"]
    #[inline] pub fn test_nre7(&self) -> bool {
        self.nre7() != 0
    }

    #[doc="Sets the NRE7 field."]
    #[inline] pub fn set_nre7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Non-Recoverable State 0 Output Value"]
    #[inline] pub fn nrv0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if NRV0 != 0"]
    #[inline] pub fn test_nrv0(&self) -> bool {
        self.nrv0() != 0
    }

    #[doc="Sets the NRV0 field."]
    #[inline] pub fn set_nrv0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Non-Recoverable State 1 Output Value"]
    #[inline] pub fn nrv1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if NRV1 != 0"]
    #[inline] pub fn test_nrv1(&self) -> bool {
        self.nrv1() != 0
    }

    #[doc="Sets the NRV1 field."]
    #[inline] pub fn set_nrv1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Non-Recoverable State 2 Output Value"]
    #[inline] pub fn nrv2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if NRV2 != 0"]
    #[inline] pub fn test_nrv2(&self) -> bool {
        self.nrv2() != 0
    }

    #[doc="Sets the NRV2 field."]
    #[inline] pub fn set_nrv2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Non-Recoverable State 3 Output Value"]
    #[inline] pub fn nrv3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if NRV3 != 0"]
    #[inline] pub fn test_nrv3(&self) -> bool {
        self.nrv3() != 0
    }

    #[doc="Sets the NRV3 field."]
    #[inline] pub fn set_nrv3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Non-Recoverable State 4 Output Value"]
    #[inline] pub fn nrv4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if NRV4 != 0"]
    #[inline] pub fn test_nrv4(&self) -> bool {
        self.nrv4() != 0
    }

    #[doc="Sets the NRV4 field."]
    #[inline] pub fn set_nrv4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Non-Recoverable State 5 Output Value"]
    #[inline] pub fn nrv5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if NRV5 != 0"]
    #[inline] pub fn test_nrv5(&self) -> bool {
        self.nrv5() != 0
    }

    #[doc="Sets the NRV5 field."]
    #[inline] pub fn set_nrv5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Non-Recoverable State 6 Output Value"]
    #[inline] pub fn nrv6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if NRV6 != 0"]
    #[inline] pub fn test_nrv6(&self) -> bool {
        self.nrv6() != 0
    }

    #[doc="Sets the NRV6 field."]
    #[inline] pub fn set_nrv6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Non-Recoverable State 7 Output Value"]
    #[inline] pub fn nrv7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if NRV7 != 0"]
    #[inline] pub fn test_nrv7(&self) -> bool {
        self.nrv7() != 0
    }

    #[doc="Sets the NRV7 field."]
    #[inline] pub fn set_nrv7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Output Waveform 0 Inversion"]
    #[inline] pub fn inven0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if INVEN0 != 0"]
    #[inline] pub fn test_inven0(&self) -> bool {
        self.inven0() != 0
    }

    #[doc="Sets the INVEN0 field."]
    #[inline] pub fn set_inven0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Output Waveform 1 Inversion"]
    #[inline] pub fn inven1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if INVEN1 != 0"]
    #[inline] pub fn test_inven1(&self) -> bool {
        self.inven1() != 0
    }

    #[doc="Sets the INVEN1 field."]
    #[inline] pub fn set_inven1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Output Waveform 2 Inversion"]
    #[inline] pub fn inven2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if INVEN2 != 0"]
    #[inline] pub fn test_inven2(&self) -> bool {
        self.inven2() != 0
    }

    #[doc="Sets the INVEN2 field."]
    #[inline] pub fn set_inven2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Output Waveform 3 Inversion"]
    #[inline] pub fn inven3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if INVEN3 != 0"]
    #[inline] pub fn test_inven3(&self) -> bool {
        self.inven3() != 0
    }

    #[doc="Sets the INVEN3 field."]
    #[inline] pub fn set_inven3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Output Waveform 4 Inversion"]
    #[inline] pub fn inven4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if INVEN4 != 0"]
    #[inline] pub fn test_inven4(&self) -> bool {
        self.inven4() != 0
    }

    #[doc="Sets the INVEN4 field."]
    #[inline] pub fn set_inven4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Output Waveform 5 Inversion"]
    #[inline] pub fn inven5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if INVEN5 != 0"]
    #[inline] pub fn test_inven5(&self) -> bool {
        self.inven5() != 0
    }

    #[doc="Sets the INVEN5 field."]
    #[inline] pub fn set_inven5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Output Waveform 6 Inversion"]
    #[inline] pub fn inven6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if INVEN6 != 0"]
    #[inline] pub fn test_inven6(&self) -> bool {
        self.inven6() != 0
    }

    #[doc="Sets the INVEN6 field."]
    #[inline] pub fn set_inven6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Output Waveform 7 Inversion"]
    #[inline] pub fn inven7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if INVEN7 != 0"]
    #[inline] pub fn test_inven7(&self) -> bool {
        self.inven7() != 0
    }

    #[doc="Sets the INVEN7 field."]
    #[inline] pub fn set_inven7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Non-Recoverable Fault Input 0 Filter Value"]
    #[inline] pub fn filterval0(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FILTERVAL0 != 0"]
    #[inline] pub fn test_filterval0(&self) -> bool {
        self.filterval0() != 0
    }

    #[doc="Sets the FILTERVAL0 field."]
    #[inline] pub fn set_filterval0<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Non-Recoverable Fault Input 1 Filter Value"]
    #[inline] pub fn filterval1(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if FILTERVAL1 != 0"]
    #[inline] pub fn test_filterval1(&self) -> bool {
        self.filterval1() != 0
    }

    #[doc="Sets the FILTERVAL1 field."]
    #[inline] pub fn set_filterval1<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Drvctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Drvctrl(other)
    }
}

impl ::core::fmt::Display for Drvctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Drvctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nre0() != 0 { try!(write!(f, " nre0"))}
        if self.nre1() != 0 { try!(write!(f, " nre1"))}
        if self.nre2() != 0 { try!(write!(f, " nre2"))}
        if self.nre3() != 0 { try!(write!(f, " nre3"))}
        if self.nre4() != 0 { try!(write!(f, " nre4"))}
        if self.nre5() != 0 { try!(write!(f, " nre5"))}
        if self.nre6() != 0 { try!(write!(f, " nre6"))}
        if self.nre7() != 0 { try!(write!(f, " nre7"))}
        if self.nrv0() != 0 { try!(write!(f, " nrv0"))}
        if self.nrv1() != 0 { try!(write!(f, " nrv1"))}
        if self.nrv2() != 0 { try!(write!(f, " nrv2"))}
        if self.nrv3() != 0 { try!(write!(f, " nrv3"))}
        if self.nrv4() != 0 { try!(write!(f, " nrv4"))}
        if self.nrv5() != 0 { try!(write!(f, " nrv5"))}
        if self.nrv6() != 0 { try!(write!(f, " nrv6"))}
        if self.nrv7() != 0 { try!(write!(f, " nrv7"))}
        if self.inven0() != 0 { try!(write!(f, " inven0"))}
        if self.inven1() != 0 { try!(write!(f, " inven1"))}
        if self.inven2() != 0 { try!(write!(f, " inven2"))}
        if self.inven3() != 0 { try!(write!(f, " inven3"))}
        if self.inven4() != 0 { try!(write!(f, " inven4"))}
        if self.inven5() != 0 { try!(write!(f, " inven5"))}
        if self.inven6() != 0 { try!(write!(f, " inven6"))}
        if self.inven7() != 0 { try!(write!(f, " inven7"))}
        if self.filterval0() != 0 { try!(write!(f, " filterval0=0x{:x}", self.filterval0()))}
        if self.filterval1() != 0 { try!(write!(f, " filterval1=0x{:x}", self.filterval1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u32);
impl Evctrl {
    #[doc="Timer/counter Input Event0 Action"]
    #[inline] pub fn evact0(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EVACT0 != 0"]
    #[inline] pub fn test_evact0(&self) -> bool {
        self.evact0() != 0
    }

    #[doc="Sets the EVACT0 field."]
    #[inline] pub fn set_evact0<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer/counter Input Event1 Action"]
    #[inline] pub fn evact1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if EVACT1 != 0"]
    #[inline] pub fn test_evact1(&self) -> bool {
        self.evact1() != 0
    }

    #[doc="Sets the EVACT1 field."]
    #[inline] pub fn set_evact1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Timer/counter Output Event Mode"]
    #[inline] pub fn cntsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CNTSEL != 0"]
    #[inline] pub fn test_cntsel(&self) -> bool {
        self.cntsel() != 0
    }

    #[doc="Sets the CNTSEL field."]
    #[inline] pub fn set_cntsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow/Underflow Output Event Enable"]
    #[inline] pub fn ovfeo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVFEO != 0"]
    #[inline] pub fn test_ovfeo(&self) -> bool {
        self.ovfeo() != 0
    }

    #[doc="Sets the OVFEO field."]
    #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Retrigger Output Event Enable"]
    #[inline] pub fn trgeo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TRGEO != 0"]
    #[inline] pub fn test_trgeo(&self) -> bool {
        self.trgeo() != 0
    }

    #[doc="Sets the TRGEO field."]
    #[inline] pub fn set_trgeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Timer/counter Output Event Enable"]
    #[inline] pub fn cnteo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CNTEO != 0"]
    #[inline] pub fn test_cnteo(&self) -> bool {
        self.cnteo() != 0
    }

    #[doc="Sets the CNTEO field."]
    #[inline] pub fn set_cnteo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Inverted Event 0 Input Enable"]
    #[inline] pub fn tcinv0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TCINV0 != 0"]
    #[inline] pub fn test_tcinv0(&self) -> bool {
        self.tcinv0() != 0
    }

    #[doc="Sets the TCINV0 field."]
    #[inline] pub fn set_tcinv0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Inverted Event 1 Input Enable"]
    #[inline] pub fn tcinv1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TCINV1 != 0"]
    #[inline] pub fn test_tcinv1(&self) -> bool {
        self.tcinv1() != 0
    }

    #[doc="Sets the TCINV1 field."]
    #[inline] pub fn set_tcinv1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Timer/counter Event 0 Input Enable"]
    #[inline] pub fn tcei0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TCEI0 != 0"]
    #[inline] pub fn test_tcei0(&self) -> bool {
        self.tcei0() != 0
    }

    #[doc="Sets the TCEI0 field."]
    #[inline] pub fn set_tcei0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Timer/counter Event 1 Input Enable"]
    #[inline] pub fn tcei1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TCEI1 != 0"]
    #[inline] pub fn test_tcei1(&self) -> bool {
        self.tcei1() != 0
    }

    #[doc="Sets the TCEI1 field."]
    #[inline] pub fn set_tcei1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Match or Capture Channel 0 Event Input Enable"]
    #[inline] pub fn mcei0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if MCEI0 != 0"]
    #[inline] pub fn test_mcei0(&self) -> bool {
        self.mcei0() != 0
    }

    #[doc="Sets the MCEI0 field."]
    #[inline] pub fn set_mcei0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Match or Capture Channel 1 Event Input Enable"]
    #[inline] pub fn mcei1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if MCEI1 != 0"]
    #[inline] pub fn test_mcei1(&self) -> bool {
        self.mcei1() != 0
    }

    #[doc="Sets the MCEI1 field."]
    #[inline] pub fn set_mcei1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Match or Capture Channel 2 Event Input Enable"]
    #[inline] pub fn mcei2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if MCEI2 != 0"]
    #[inline] pub fn test_mcei2(&self) -> bool {
        self.mcei2() != 0
    }

    #[doc="Sets the MCEI2 field."]
    #[inline] pub fn set_mcei2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Match or Capture Channel 3 Event Input Enable"]
    #[inline] pub fn mcei3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if MCEI3 != 0"]
    #[inline] pub fn test_mcei3(&self) -> bool {
        self.mcei3() != 0
    }

    #[doc="Sets the MCEI3 field."]
    #[inline] pub fn set_mcei3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Match or Capture Channel 0 Event Output Enable"]
    #[inline] pub fn mceo0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if MCEO0 != 0"]
    #[inline] pub fn test_mceo0(&self) -> bool {
        self.mceo0() != 0
    }

    #[doc="Sets the MCEO0 field."]
    #[inline] pub fn set_mceo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Match or Capture Channel 1 Event Output Enable"]
    #[inline] pub fn mceo1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if MCEO1 != 0"]
    #[inline] pub fn test_mceo1(&self) -> bool {
        self.mceo1() != 0
    }

    #[doc="Sets the MCEO1 field."]
    #[inline] pub fn set_mceo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Match or Capture Channel 2 Event Output Enable"]
    #[inline] pub fn mceo2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if MCEO2 != 0"]
    #[inline] pub fn test_mceo2(&self) -> bool {
        self.mceo2() != 0
    }

    #[doc="Sets the MCEO2 field."]
    #[inline] pub fn set_mceo2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Match or Capture Channel 3 Event Output Enable"]
    #[inline] pub fn mceo3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if MCEO3 != 0"]
    #[inline] pub fn test_mceo3(&self) -> bool {
        self.mceo3() != 0
    }

    #[doc="Sets the MCEO3 field."]
    #[inline] pub fn set_mceo3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Evctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.evact0() != 0 { try!(write!(f, " evact0=0x{:x}", self.evact0()))}
        if self.evact1() != 0 { try!(write!(f, " evact1=0x{:x}", self.evact1()))}
        if self.cntsel() != 0 { try!(write!(f, " cntsel=0x{:x}", self.cntsel()))}
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        if self.trgeo() != 0 { try!(write!(f, " trgeo"))}
        if self.cnteo() != 0 { try!(write!(f, " cnteo"))}
        if self.tcinv0() != 0 { try!(write!(f, " tcinv0"))}
        if self.tcinv1() != 0 { try!(write!(f, " tcinv1"))}
        if self.tcei0() != 0 { try!(write!(f, " tcei0"))}
        if self.tcei1() != 0 { try!(write!(f, " tcei1"))}
        if self.mcei0() != 0 { try!(write!(f, " mcei0"))}
        if self.mcei1() != 0 { try!(write!(f, " mcei1"))}
        if self.mcei2() != 0 { try!(write!(f, " mcei2"))}
        if self.mcei3() != 0 { try!(write!(f, " mcei3"))}
        if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
        if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
        if self.mceo2() != 0 { try!(write!(f, " mceo2"))}
        if self.mceo3() != 0 { try!(write!(f, " mceo3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Recoverable FaultA Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fctrla(pub u32);
impl Fctrla {
    #[doc="FaultA Source"]
    #[inline] pub fn src(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SRC != 0"]
    #[inline] pub fn test_src(&self) -> bool {
        self.src() != 0
    }

    #[doc="Sets the SRC field."]
    #[inline] pub fn set_src<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="FaultA Keeper"]
    #[inline] pub fn keep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if KEEP != 0"]
    #[inline] pub fn test_keep(&self) -> bool {
        self.keep() != 0
    }

    #[doc="Sets the KEEP field."]
    #[inline] pub fn set_keep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FaultA Qualification"]
    #[inline] pub fn qual(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if QUAL != 0"]
    #[inline] pub fn test_qual(&self) -> bool {
        self.qual() != 0
    }

    #[doc="Sets the QUAL field."]
    #[inline] pub fn set_qual<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FaultA Blanking Mode"]
    #[inline] pub fn blank(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if BLANK != 0"]
    #[inline] pub fn test_blank(&self) -> bool {
        self.blank() != 0
    }

    #[doc="Sets the BLANK field."]
    #[inline] pub fn set_blank<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FaultA Restart"]
    #[inline] pub fn restart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RESTART != 0"]
    #[inline] pub fn test_restart(&self) -> bool {
        self.restart() != 0
    }

    #[doc="Sets the RESTART field."]
    #[inline] pub fn set_restart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="FaultA Halt Mode"]
    #[inline] pub fn halt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if HALT != 0"]
    #[inline] pub fn test_halt(&self) -> bool {
        self.halt() != 0
    }

    #[doc="Sets the HALT field."]
    #[inline] pub fn set_halt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FaultA Capture Channel"]
    #[inline] pub fn chsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if CHSEL != 0"]
    #[inline] pub fn test_chsel(&self) -> bool {
        self.chsel() != 0
    }

    #[doc="Sets the CHSEL field."]
    #[inline] pub fn set_chsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="FaultA Capture Action"]
    #[inline] pub fn capture(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if CAPTURE != 0"]
    #[inline] pub fn test_capture(&self) -> bool {
        self.capture() != 0
    }

    #[doc="Sets the CAPTURE field."]
    #[inline] pub fn set_capture<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="FaultA Blanking Time"]
    #[inline] pub fn blankval(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if BLANKVAL != 0"]
    #[inline] pub fn test_blankval(&self) -> bool {
        self.blankval() != 0
    }

    #[doc="Sets the BLANKVAL field."]
    #[inline] pub fn set_blankval<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="FaultA Filter Value"]
    #[inline] pub fn filterval(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FILTERVAL != 0"]
    #[inline] pub fn test_filterval(&self) -> bool {
        self.filterval() != 0
    }

    #[doc="Sets the FILTERVAL field."]
    #[inline] pub fn set_filterval<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Fctrla {
    #[inline]
    fn from(other: u32) -> Self {
         Fctrla(other)
    }
}

impl ::core::fmt::Display for Fctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
        if self.keep() != 0 { try!(write!(f, " keep"))}
        if self.qual() != 0 { try!(write!(f, " qual"))}
        if self.blank() != 0 { try!(write!(f, " blank=0x{:x}", self.blank()))}
        if self.restart() != 0 { try!(write!(f, " restart"))}
        if self.halt() != 0 { try!(write!(f, " halt=0x{:x}", self.halt()))}
        if self.chsel() != 0 { try!(write!(f, " chsel=0x{:x}", self.chsel()))}
        if self.capture() != 0 { try!(write!(f, " capture=0x{:x}", self.capture()))}
        if self.blankval() != 0 { try!(write!(f, " blankval=0x{:x}", self.blankval()))}
        if self.filterval() != 0 { try!(write!(f, " filterval=0x{:x}", self.filterval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Recoverable FaultB Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fctrlb(pub u32);
impl Fctrlb {
    #[doc="FaultB Source"]
    #[inline] pub fn src(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SRC != 0"]
    #[inline] pub fn test_src(&self) -> bool {
        self.src() != 0
    }

    #[doc="Sets the SRC field."]
    #[inline] pub fn set_src<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="FaultB Keeper"]
    #[inline] pub fn keep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if KEEP != 0"]
    #[inline] pub fn test_keep(&self) -> bool {
        self.keep() != 0
    }

    #[doc="Sets the KEEP field."]
    #[inline] pub fn set_keep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FaultB Qualification"]
    #[inline] pub fn qual(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if QUAL != 0"]
    #[inline] pub fn test_qual(&self) -> bool {
        self.qual() != 0
    }

    #[doc="Sets the QUAL field."]
    #[inline] pub fn set_qual<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FaultB Blanking Mode"]
    #[inline] pub fn blank(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if BLANK != 0"]
    #[inline] pub fn test_blank(&self) -> bool {
        self.blank() != 0
    }

    #[doc="Sets the BLANK field."]
    #[inline] pub fn set_blank<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FaultB Restart"]
    #[inline] pub fn restart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RESTART != 0"]
    #[inline] pub fn test_restart(&self) -> bool {
        self.restart() != 0
    }

    #[doc="Sets the RESTART field."]
    #[inline] pub fn set_restart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="FaultB Halt Mode"]
    #[inline] pub fn halt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if HALT != 0"]
    #[inline] pub fn test_halt(&self) -> bool {
        self.halt() != 0
    }

    #[doc="Sets the HALT field."]
    #[inline] pub fn set_halt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FaultB Capture Channel"]
    #[inline] pub fn chsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if CHSEL != 0"]
    #[inline] pub fn test_chsel(&self) -> bool {
        self.chsel() != 0
    }

    #[doc="Sets the CHSEL field."]
    #[inline] pub fn set_chsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="FaultB Capture Action"]
    #[inline] pub fn capture(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if CAPTURE != 0"]
    #[inline] pub fn test_capture(&self) -> bool {
        self.capture() != 0
    }

    #[doc="Sets the CAPTURE field."]
    #[inline] pub fn set_capture<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="FaultB Blanking Time"]
    #[inline] pub fn blankval(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if BLANKVAL != 0"]
    #[inline] pub fn test_blankval(&self) -> bool {
        self.blankval() != 0
    }

    #[doc="Sets the BLANKVAL field."]
    #[inline] pub fn set_blankval<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="FaultB Filter Value"]
    #[inline] pub fn filterval(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FILTERVAL != 0"]
    #[inline] pub fn test_filterval(&self) -> bool {
        self.filterval() != 0
    }

    #[doc="Sets the FILTERVAL field."]
    #[inline] pub fn set_filterval<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Fctrlb {
    #[inline]
    fn from(other: u32) -> Self {
         Fctrlb(other)
    }
}

impl ::core::fmt::Display for Fctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
        if self.keep() != 0 { try!(write!(f, " keep"))}
        if self.qual() != 0 { try!(write!(f, " qual"))}
        if self.blank() != 0 { try!(write!(f, " blank=0x{:x}", self.blank()))}
        if self.restart() != 0 { try!(write!(f, " restart"))}
        if self.halt() != 0 { try!(write!(f, " halt=0x{:x}", self.halt()))}
        if self.chsel() != 0 { try!(write!(f, " chsel=0x{:x}", self.chsel()))}
        if self.capture() != 0 { try!(write!(f, " capture=0x{:x}", self.capture()))}
        if self.blankval() != 0 { try!(write!(f, " blankval=0x{:x}", self.blankval()))}
        if self.filterval() != 0 { try!(write!(f, " filterval=0x{:x}", self.filterval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Retrigger Interrupt Enable"]
    #[inline] pub fn trg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRG != 0"]
    #[inline] pub fn test_trg(&self) -> bool {
        self.trg() != 0
    }

    #[doc="Sets the TRG field."]
    #[inline] pub fn set_trg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter Interrupt Enable"]
    #[inline] pub fn cnt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Error Interrupt Enable"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Non-recoverable Debug Fault Interrupt Enable"]
    #[inline] pub fn dfs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFS != 0"]
    #[inline] pub fn test_dfs(&self) -> bool {
        self.dfs() != 0
    }

    #[doc="Sets the DFS field."]
    #[inline] pub fn set_dfs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Recoverable FaultA Interrupt Enable"]
    #[inline] pub fn faulta(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FAULTA != 0"]
    #[inline] pub fn test_faulta(&self) -> bool {
        self.faulta() != 0
    }

    #[doc="Sets the FAULTA field."]
    #[inline] pub fn set_faulta<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Recoverable FaultB Interrupt Enable"]
    #[inline] pub fn faultb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FAULTB != 0"]
    #[inline] pub fn test_faultb(&self) -> bool {
        self.faultb() != 0
    }

    #[doc="Sets the FAULTB field."]
    #[inline] pub fn set_faultb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Non-Recoverable Fault 0 Interrupt Enable"]
    #[inline] pub fn fault0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FAULT0 != 0"]
    #[inline] pub fn test_fault0(&self) -> bool {
        self.fault0() != 0
    }

    #[doc="Sets the FAULT0 field."]
    #[inline] pub fn set_fault0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Non-Recoverable Fault 1 Interrupt Enable"]
    #[inline] pub fn fault1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FAULT1 != 0"]
    #[inline] pub fn test_fault1(&self) -> bool {
        self.fault1() != 0
    }

    #[doc="Sets the FAULT1 field."]
    #[inline] pub fn set_fault1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Match or Capture Channel 0 Interrupt Enable"]
    #[inline] pub fn mc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Match or Capture Channel 1 Interrupt Enable"]
    #[inline] pub fn mc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Match or Capture Channel 2 Interrupt Enable"]
    #[inline] pub fn mc2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if MC2 != 0"]
    #[inline] pub fn test_mc2(&self) -> bool {
        self.mc2() != 0
    }

    #[doc="Sets the MC2 field."]
    #[inline] pub fn set_mc2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Match or Capture Channel 3 Interrupt Enable"]
    #[inline] pub fn mc3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if MC3 != 0"]
    #[inline] pub fn test_mc3(&self) -> bool {
        self.mc3() != 0
    }

    #[doc="Sets the MC3 field."]
    #[inline] pub fn set_mc3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

}

impl From<u32> for Intenclr {
    #[inline]
    fn from(other: u32) -> Self {
         Intenclr(other)
    }
}

impl ::core::fmt::Display for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.trg() != 0 { try!(write!(f, " trg"))}
        if self.cnt() != 0 { try!(write!(f, " cnt"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.dfs() != 0 { try!(write!(f, " dfs"))}
        if self.faulta() != 0 { try!(write!(f, " faulta"))}
        if self.faultb() != 0 { try!(write!(f, " faultb"))}
        if self.fault0() != 0 { try!(write!(f, " fault0"))}
        if self.fault1() != 0 { try!(write!(f, " fault1"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        if self.mc2() != 0 { try!(write!(f, " mc2"))}
        if self.mc3() != 0 { try!(write!(f, " mc3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Retrigger Interrupt Enable"]
    #[inline] pub fn trg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRG != 0"]
    #[inline] pub fn test_trg(&self) -> bool {
        self.trg() != 0
    }

    #[doc="Sets the TRG field."]
    #[inline] pub fn set_trg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter Interrupt Enable"]
    #[inline] pub fn cnt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Error Interrupt Enable"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Non-Recoverable Debug Fault Interrupt Enable"]
    #[inline] pub fn dfs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFS != 0"]
    #[inline] pub fn test_dfs(&self) -> bool {
        self.dfs() != 0
    }

    #[doc="Sets the DFS field."]
    #[inline] pub fn set_dfs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Recoverable FaultA Interrupt Enable"]
    #[inline] pub fn faulta(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FAULTA != 0"]
    #[inline] pub fn test_faulta(&self) -> bool {
        self.faulta() != 0
    }

    #[doc="Sets the FAULTA field."]
    #[inline] pub fn set_faulta<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Recoverable FaultB Interrupt Enable"]
    #[inline] pub fn faultb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FAULTB != 0"]
    #[inline] pub fn test_faultb(&self) -> bool {
        self.faultb() != 0
    }

    #[doc="Sets the FAULTB field."]
    #[inline] pub fn set_faultb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Non-Recoverable Fault 0 Interrupt Enable"]
    #[inline] pub fn fault0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FAULT0 != 0"]
    #[inline] pub fn test_fault0(&self) -> bool {
        self.fault0() != 0
    }

    #[doc="Sets the FAULT0 field."]
    #[inline] pub fn set_fault0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Non-Recoverable Fault 1 Interrupt Enabl"]
    #[inline] pub fn fault1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FAULT1 != 0"]
    #[inline] pub fn test_fault1(&self) -> bool {
        self.fault1() != 0
    }

    #[doc="Sets the FAULT1 field."]
    #[inline] pub fn set_fault1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Match or Capture Channel n Interrupt Enable"]
    #[inline] pub fn mc<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if MC != 0"]
    #[inline] pub fn test_mc<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.mc(index) != 0
    }

    #[doc="Sets the MC field."]
    #[inline] pub fn set_mc<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Intenset {
    #[inline]
    fn from(other: u32) -> Self {
         Intenset(other)
    }
}

impl ::core::fmt::Display for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.trg() != 0 { try!(write!(f, " trg"))}
        if self.cnt() != 0 { try!(write!(f, " cnt"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.dfs() != 0 { try!(write!(f, " dfs"))}
        if self.faulta() != 0 { try!(write!(f, " faulta"))}
        if self.faultb() != 0 { try!(write!(f, " faultb"))}
        if self.fault0() != 0 { try!(write!(f, " fault0"))}
        if self.fault1() != 0 { try!(write!(f, " fault1"))}
        if self.mc(0) != 0 { try!(write!(f, " mc[0]"))}
        if self.mc(1) != 0 { try!(write!(f, " mc[1]"))}
        if self.mc(2) != 0 { try!(write!(f, " mc[2]"))}
        if self.mc(3) != 0 { try!(write!(f, " mc[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u32);
impl Intflag {
    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Retrigger"]
    #[inline] pub fn trg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRG != 0"]
    #[inline] pub fn test_trg(&self) -> bool {
        self.trg() != 0
    }

    #[doc="Sets the TRG field."]
    #[inline] pub fn set_trg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter"]
    #[inline] pub fn cnt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Error"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Non-Recoverable Debug Fault"]
    #[inline] pub fn dfs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFS != 0"]
    #[inline] pub fn test_dfs(&self) -> bool {
        self.dfs() != 0
    }

    #[doc="Sets the DFS field."]
    #[inline] pub fn set_dfs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Recoverable FaultA"]
    #[inline] pub fn faulta(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FAULTA != 0"]
    #[inline] pub fn test_faulta(&self) -> bool {
        self.faulta() != 0
    }

    #[doc="Sets the FAULTA field."]
    #[inline] pub fn set_faulta<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Recoverable FaultB"]
    #[inline] pub fn faultb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FAULTB != 0"]
    #[inline] pub fn test_faultb(&self) -> bool {
        self.faultb() != 0
    }

    #[doc="Sets the FAULTB field."]
    #[inline] pub fn set_faultb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Non-Recoverable Fault 0"]
    #[inline] pub fn fault0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FAULT0 != 0"]
    #[inline] pub fn test_fault0(&self) -> bool {
        self.fault0() != 0
    }

    #[doc="Sets the FAULT0 field."]
    #[inline] pub fn set_fault0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Non-Recoverable Fault 1"]
    #[inline] pub fn fault1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FAULT1 != 0"]
    #[inline] pub fn test_fault1(&self) -> bool {
        self.fault1() != 0
    }

    #[doc="Sets the FAULT1 field."]
    #[inline] pub fn set_fault1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Match or Capture n"]
    #[inline] pub fn mc<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if MC != 0"]
    #[inline] pub fn test_mc<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.mc(index) != 0
    }

    #[doc="Sets the MC field."]
    #[inline] pub fn set_mc<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Intflag {
    #[inline]
    fn from(other: u32) -> Self {
         Intflag(other)
    }
}

impl ::core::fmt::Display for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.trg() != 0 { try!(write!(f, " trg"))}
        if self.cnt() != 0 { try!(write!(f, " cnt"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.dfs() != 0 { try!(write!(f, " dfs"))}
        if self.faulta() != 0 { try!(write!(f, " faulta"))}
        if self.faultb() != 0 { try!(write!(f, " faultb"))}
        if self.fault0() != 0 { try!(write!(f, " fault0"))}
        if self.fault1() != 0 { try!(write!(f, " fault1"))}
        if self.mc(0) != 0 { try!(write!(f, " mc[0]"))}
        if self.mc(1) != 0 { try!(write!(f, " mc[1]"))}
        if self.mc(2) != 0 { try!(write!(f, " mc[2]"))}
        if self.mc(3) != 0 { try!(write!(f, " mc[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pattern"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Patt(pub u16);
impl Patt {
    #[doc="Pattern Generator n Output Enable"]
    #[inline] pub fn pge<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PGE != 0"]
    #[inline] pub fn test_pge<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.pge(index) != 0
    }

    #[doc="Sets the PGE field."]
    #[inline] pub fn set_pge<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Pattern Generator n Output Value"]
    #[inline] pub fn pgv<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PGV != 0"]
    #[inline] pub fn test_pgv<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.pgv(index) != 0
    }

    #[doc="Sets the PGV field."]
    #[inline] pub fn set_pgv<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u16> for Patt {
    #[inline]
    fn from(other: u16) -> Self {
         Patt(other)
    }
}

impl ::core::fmt::Display for Patt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Patt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pge(0) != 0 { try!(write!(f, " pge[0]"))}
        if self.pge(1) != 0 { try!(write!(f, " pge[1]"))}
        if self.pge(2) != 0 { try!(write!(f, " pge[2]"))}
        if self.pge(3) != 0 { try!(write!(f, " pge[3]"))}
        if self.pge(4) != 0 { try!(write!(f, " pge[4]"))}
        if self.pge(5) != 0 { try!(write!(f, " pge[5]"))}
        if self.pge(6) != 0 { try!(write!(f, " pge[6]"))}
        if self.pge(7) != 0 { try!(write!(f, " pge[7]"))}
        if self.pgv(0) != 0 { try!(write!(f, " pgv[0]"))}
        if self.pgv(1) != 0 { try!(write!(f, " pgv[1]"))}
        if self.pgv(2) != 0 { try!(write!(f, " pgv[2]"))}
        if self.pgv(3) != 0 { try!(write!(f, " pgv[3]"))}
        if self.pgv(4) != 0 { try!(write!(f, " pgv[4]"))}
        if self.pgv(5) != 0 { try!(write!(f, " pgv[5]"))}
        if self.pgv(6) != 0 { try!(write!(f, " pgv[6]"))}
        if self.pgv(7) != 0 { try!(write!(f, " pgv[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pattern Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pattb(pub u16);
impl Pattb {
    #[doc="Pattern Generator n Output Enable Buffer"]
    #[inline] pub fn pgeb<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PGEB != 0"]
    #[inline] pub fn test_pgeb<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.pgeb(index) != 0
    }

    #[doc="Sets the PGEB field."]
    #[inline] pub fn set_pgeb<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Pattern Generator n Output Enable"]
    #[inline] pub fn pgvb<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PGVB != 0"]
    #[inline] pub fn test_pgvb<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.pgvb(index) != 0
    }

    #[doc="Sets the PGVB field."]
    #[inline] pub fn set_pgvb<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u16> for Pattb {
    #[inline]
    fn from(other: u16) -> Self {
         Pattb(other)
    }
}

impl ::core::fmt::Display for Pattb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pattb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pgeb(0) != 0 { try!(write!(f, " pgeb[0]"))}
        if self.pgeb(1) != 0 { try!(write!(f, " pgeb[1]"))}
        if self.pgeb(2) != 0 { try!(write!(f, " pgeb[2]"))}
        if self.pgeb(3) != 0 { try!(write!(f, " pgeb[3]"))}
        if self.pgeb(4) != 0 { try!(write!(f, " pgeb[4]"))}
        if self.pgeb(5) != 0 { try!(write!(f, " pgeb[5]"))}
        if self.pgeb(6) != 0 { try!(write!(f, " pgeb[6]"))}
        if self.pgeb(7) != 0 { try!(write!(f, " pgeb[7]"))}
        if self.pgvb(0) != 0 { try!(write!(f, " pgvb[0]"))}
        if self.pgvb(1) != 0 { try!(write!(f, " pgvb[1]"))}
        if self.pgvb(2) != 0 { try!(write!(f, " pgvb[2]"))}
        if self.pgvb(3) != 0 { try!(write!(f, " pgvb[3]"))}
        if self.pgvb(4) != 0 { try!(write!(f, " pgvb[4]"))}
        if self.pgvb(5) != 0 { try!(write!(f, " pgvb[5]"))}
        if self.pgvb(6) != 0 { try!(write!(f, " pgvb[6]"))}
        if self.pgvb(7) != 0 { try!(write!(f, " pgvb[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Period"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Per(pub u32);
impl Per {
    #[doc="Period Value"]
    #[inline] pub fn per(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Per {
    #[inline]
    fn from(other: u32) -> Self {
         Per(other)
    }
}

impl ::core::fmt::Display for Per {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Per {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Period Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Perb(pub u32);
impl Perb {
    #[doc="Period Value"]
    #[inline] pub fn perb(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if PERB != 0"]
    #[inline] pub fn test_perb(&self) -> bool {
        self.perb() != 0
    }

    #[doc="Sets the PERB field."]
    #[inline] pub fn set_perb<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Perb {
    #[inline]
    fn from(other: u32) -> Self {
         Perb(other)
    }
}

impl ::core::fmt::Display for Perb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Perb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.perb() != 0 { try!(write!(f, " perb=0x{:x}", self.perb()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="Stop"]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Ramp"]
    #[inline] pub fn idx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IDX != 0"]
    #[inline] pub fn test_idx(&self) -> bool {
        self.idx() != 0
    }

    #[doc="Sets the IDX field."]
    #[inline] pub fn set_idx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Non-Recoverable Debug Fault State"]
    #[inline] pub fn dfs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DFS != 0"]
    #[inline] pub fn test_dfs(&self) -> bool {
        self.dfs() != 0
    }

    #[doc="Sets the DFS field."]
    #[inline] pub fn set_dfs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pattern Buffer Valid"]
    #[inline] pub fn pattbv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PATTBV != 0"]
    #[inline] pub fn test_pattbv(&self) -> bool {
        self.pattbv() != 0
    }

    #[doc="Sets the PATTBV field."]
    #[inline] pub fn set_pattbv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Wave Buffer Valid"]
    #[inline] pub fn wavebv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if WAVEBV != 0"]
    #[inline] pub fn test_wavebv(&self) -> bool {
        self.wavebv() != 0
    }

    #[doc="Sets the WAVEBV field."]
    #[inline] pub fn set_wavebv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Period Buffer Valid"]
    #[inline] pub fn perbv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PERBV != 0"]
    #[inline] pub fn test_perbv(&self) -> bool {
        self.perbv() != 0
    }

    #[doc="Sets the PERBV field."]
    #[inline] pub fn set_perbv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Recoverable FaultA Input"]
    #[inline] pub fn faultain(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FAULTAIN != 0"]
    #[inline] pub fn test_faultain(&self) -> bool {
        self.faultain() != 0
    }

    #[doc="Sets the FAULTAIN field."]
    #[inline] pub fn set_faultain<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Recoverable FaultB Input"]
    #[inline] pub fn faultbin(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FAULTBIN != 0"]
    #[inline] pub fn test_faultbin(&self) -> bool {
        self.faultbin() != 0
    }

    #[doc="Sets the FAULTBIN field."]
    #[inline] pub fn set_faultbin<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Non-Recoverable Fault0 Input"]
    #[inline] pub fn fault0in(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FAULT0IN != 0"]
    #[inline] pub fn test_fault0in(&self) -> bool {
        self.fault0in() != 0
    }

    #[doc="Sets the FAULT0IN field."]
    #[inline] pub fn set_fault0in<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Non-Recoverable Fault1 Input"]
    #[inline] pub fn fault1in(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FAULT1IN != 0"]
    #[inline] pub fn test_fault1in(&self) -> bool {
        self.fault1in() != 0
    }

    #[doc="Sets the FAULT1IN field."]
    #[inline] pub fn set_fault1in<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Recoverable FaultA State"]
    #[inline] pub fn faulta(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FAULTA != 0"]
    #[inline] pub fn test_faulta(&self) -> bool {
        self.faulta() != 0
    }

    #[doc="Sets the FAULTA field."]
    #[inline] pub fn set_faulta<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Recoverable FaultB State"]
    #[inline] pub fn faultb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FAULTB != 0"]
    #[inline] pub fn test_faultb(&self) -> bool {
        self.faultb() != 0
    }

    #[doc="Sets the FAULTB field."]
    #[inline] pub fn set_faultb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Non-Recoverable Fault 0 State"]
    #[inline] pub fn fault0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FAULT0 != 0"]
    #[inline] pub fn test_fault0(&self) -> bool {
        self.fault0() != 0
    }

    #[doc="Sets the FAULT0 field."]
    #[inline] pub fn set_fault0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Non-Recoverable Fault 1 State"]
    #[inline] pub fn fault1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FAULT1 != 0"]
    #[inline] pub fn test_fault1(&self) -> bool {
        self.fault1() != 0
    }

    #[doc="Sets the FAULT1 field."]
    #[inline] pub fn set_fault1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Compare Channel n Buffer Valid"]
    #[inline] pub fn ccbv<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CCBV != 0"]
    #[inline] pub fn test_ccbv<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.ccbv(index) != 0
    }

    #[doc="Sets the CCBV field."]
    #[inline] pub fn set_ccbv<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Compare Channel n Value"]
    #[inline] pub fn cmp<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 24 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CMP != 0"]
    #[inline] pub fn test_cmp<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.cmp(index) != 0
    }

    #[doc="Sets the CMP field."]
    #[inline] pub fn set_cmp<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 24 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Status {
    #[inline]
    fn from(other: u32) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.idx() != 0 { try!(write!(f, " idx"))}
        if self.dfs() != 0 { try!(write!(f, " dfs"))}
        if self.pattbv() != 0 { try!(write!(f, " pattbv"))}
        if self.wavebv() != 0 { try!(write!(f, " wavebv"))}
        if self.perbv() != 0 { try!(write!(f, " perbv"))}
        if self.faultain() != 0 { try!(write!(f, " faultain"))}
        if self.faultbin() != 0 { try!(write!(f, " faultbin"))}
        if self.fault0in() != 0 { try!(write!(f, " fault0in"))}
        if self.fault1in() != 0 { try!(write!(f, " fault1in"))}
        if self.faulta() != 0 { try!(write!(f, " faulta"))}
        if self.faultb() != 0 { try!(write!(f, " faultb"))}
        if self.fault0() != 0 { try!(write!(f, " fault0"))}
        if self.fault1() != 0 { try!(write!(f, " fault1"))}
        if self.ccbv(0) != 0 { try!(write!(f, " ccbv[0]"))}
        if self.ccbv(1) != 0 { try!(write!(f, " ccbv[1]"))}
        if self.ccbv(2) != 0 { try!(write!(f, " ccbv[2]"))}
        if self.ccbv(3) != 0 { try!(write!(f, " ccbv[3]"))}
        if self.cmp(0) != 0 { try!(write!(f, " cmp[0]"))}
        if self.cmp(1) != 0 { try!(write!(f, " cmp[1]"))}
        if self.cmp(2) != 0 { try!(write!(f, " cmp[2]"))}
        if self.cmp(3) != 0 { try!(write!(f, " cmp[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Swrst Busy"]
    #[inline] pub fn swrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable Busy"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Ctrlb Busy"]
    #[inline] pub fn ctrlb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTRLB != 0"]
    #[inline] pub fn test_ctrlb(&self) -> bool {
        self.ctrlb() != 0
    }

    #[doc="Sets the CTRLB field."]
    #[inline] pub fn set_ctrlb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Status Busy"]
    #[inline] pub fn status(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STATUS != 0"]
    #[inline] pub fn test_status(&self) -> bool {
        self.status() != 0
    }

    #[doc="Sets the STATUS field."]
    #[inline] pub fn set_status<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Count Busy"]
    #[inline] pub fn count(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pattern Busy"]
    #[inline] pub fn patt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PATT != 0"]
    #[inline] pub fn test_patt(&self) -> bool {
        self.patt() != 0
    }

    #[doc="Sets the PATT field."]
    #[inline] pub fn set_patt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Wave Busy"]
    #[inline] pub fn wave(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if WAVE != 0"]
    #[inline] pub fn test_wave(&self) -> bool {
        self.wave() != 0
    }

    #[doc="Sets the WAVE field."]
    #[inline] pub fn set_wave<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Period busy"]
    #[inline] pub fn per(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Compare Channel Buffer n Busy"]
    #[inline] pub fn cc<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CC != 0"]
    #[inline] pub fn test_cc<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.cc(index) != 0
    }

    #[doc="Sets the CC field."]
    #[inline] pub fn set_cc<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Pattern Buffer Busy"]
    #[inline] pub fn pattb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PATTB != 0"]
    #[inline] pub fn test_pattb(&self) -> bool {
        self.pattb() != 0
    }

    #[doc="Sets the PATTB field."]
    #[inline] pub fn set_pattb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Wave Buffer Busy"]
    #[inline] pub fn waveb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if WAVEB != 0"]
    #[inline] pub fn test_waveb(&self) -> bool {
        self.waveb() != 0
    }

    #[doc="Sets the WAVEB field."]
    #[inline] pub fn set_waveb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Period Buffer Busy"]
    #[inline] pub fn perb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PERB != 0"]
    #[inline] pub fn test_perb(&self) -> bool {
        self.perb() != 0
    }

    #[doc="Sets the PERB field."]
    #[inline] pub fn set_perb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Compare Channel Buffer n Busy"]
    #[inline] pub fn ccb<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 19 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CCB != 0"]
    #[inline] pub fn test_ccb<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.ccb(index) != 0
    }

    #[doc="Sets the CCB field."]
    #[inline] pub fn set_ccb<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 19 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Syncbusy {
    #[inline]
    fn from(other: u32) -> Self {
         Syncbusy(other)
    }
}

impl ::core::fmt::Display for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.ctrlb() != 0 { try!(write!(f, " ctrlb"))}
        if self.status() != 0 { try!(write!(f, " status"))}
        if self.count() != 0 { try!(write!(f, " count"))}
        if self.patt() != 0 { try!(write!(f, " patt"))}
        if self.wave() != 0 { try!(write!(f, " wave"))}
        if self.per() != 0 { try!(write!(f, " per"))}
        if self.cc(0) != 0 { try!(write!(f, " cc[0]"))}
        if self.cc(1) != 0 { try!(write!(f, " cc[1]"))}
        if self.cc(2) != 0 { try!(write!(f, " cc[2]"))}
        if self.cc(3) != 0 { try!(write!(f, " cc[3]"))}
        if self.pattb() != 0 { try!(write!(f, " pattb"))}
        if self.waveb() != 0 { try!(write!(f, " waveb"))}
        if self.perb() != 0 { try!(write!(f, " perb"))}
        if self.ccb(0) != 0 { try!(write!(f, " ccb[0]"))}
        if self.ccb(1) != 0 { try!(write!(f, " ccb[1]"))}
        if self.ccb(2) != 0 { try!(write!(f, " ccb[2]"))}
        if self.ccb(3) != 0 { try!(write!(f, " ccb[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Waveform Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wave(pub u32);
impl Wave {
    #[doc="Waveform Generation"]
    #[inline] pub fn wavegen(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if WAVEGEN != 0"]
    #[inline] pub fn test_wavegen(&self) -> bool {
        self.wavegen() != 0
    }

    #[doc="Sets the WAVEGEN field."]
    #[inline] pub fn set_wavegen<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Ramp Mode"]
    #[inline] pub fn ramp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if RAMP != 0"]
    #[inline] pub fn test_ramp(&self) -> bool {
        self.ramp() != 0
    }

    #[doc="Sets the RAMP field."]
    #[inline] pub fn set_ramp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Circular period Enable"]
    #[inline] pub fn ciperen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CIPEREN != 0"]
    #[inline] pub fn test_ciperen(&self) -> bool {
        self.ciperen() != 0
    }

    #[doc="Sets the CIPEREN field."]
    #[inline] pub fn set_ciperen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Circular Channel n Enable"]
    #[inline] pub fn ciccen<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CICCEN != 0"]
    #[inline] pub fn test_ciccen<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.ciccen(index) != 0
    }

    #[doc="Sets the CICCEN field."]
    #[inline] pub fn set_ciccen<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Channel n Polarity"]
    #[inline] pub fn pol<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if POL != 0"]
    #[inline] pub fn test_pol<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.pol(index) != 0
    }

    #[doc="Sets the POL field."]
    #[inline] pub fn set_pol<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Swap DTI Output Pair n"]
    #[inline] pub fn swap<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 24 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SWAP != 0"]
    #[inline] pub fn test_swap<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.swap(index) != 0
    }

    #[doc="Sets the SWAP field."]
    #[inline] pub fn set_swap<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 24 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Wave {
    #[inline]
    fn from(other: u32) -> Self {
         Wave(other)
    }
}

impl ::core::fmt::Display for Wave {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wave {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
        if self.ramp() != 0 { try!(write!(f, " ramp=0x{:x}", self.ramp()))}
        if self.ciperen() != 0 { try!(write!(f, " ciperen"))}
        if self.ciccen(0) != 0 { try!(write!(f, " ciccen[0]"))}
        if self.ciccen(1) != 0 { try!(write!(f, " ciccen[1]"))}
        if self.ciccen(2) != 0 { try!(write!(f, " ciccen[2]"))}
        if self.ciccen(3) != 0 { try!(write!(f, " ciccen[3]"))}
        if self.pol(0) != 0 { try!(write!(f, " pol[0]"))}
        if self.pol(1) != 0 { try!(write!(f, " pol[1]"))}
        if self.pol(2) != 0 { try!(write!(f, " pol[2]"))}
        if self.pol(3) != 0 { try!(write!(f, " pol[3]"))}
        if self.swap(0) != 0 { try!(write!(f, " swap[0]"))}
        if self.swap(1) != 0 { try!(write!(f, " swap[1]"))}
        if self.swap(2) != 0 { try!(write!(f, " swap[2]"))}
        if self.swap(3) != 0 { try!(write!(f, " swap[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Waveform Control Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Waveb(pub u32);
impl Waveb {
    #[doc="Waveform Generation Buffer"]
    #[inline] pub fn wavegenb(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if WAVEGENB != 0"]
    #[inline] pub fn test_wavegenb(&self) -> bool {
        self.wavegenb() != 0
    }

    #[doc="Sets the WAVEGENB field."]
    #[inline] pub fn set_wavegenb<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Ramp Mode Buffer"]
    #[inline] pub fn rampb(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if RAMPB != 0"]
    #[inline] pub fn test_rampb(&self) -> bool {
        self.rampb() != 0
    }

    #[doc="Sets the RAMPB field."]
    #[inline] pub fn set_rampb<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Circular Period Enable Buffer"]
    #[inline] pub fn ciperenb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CIPERENB != 0"]
    #[inline] pub fn test_ciperenb(&self) -> bool {
        self.ciperenb() != 0
    }

    #[doc="Sets the CIPERENB field."]
    #[inline] pub fn set_ciperenb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Circular Channel n Enable Buffer"]
    #[inline] pub fn ciccenb<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CICCENB != 0"]
    #[inline] pub fn test_ciccenb<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.ciccenb(index) != 0
    }

    #[doc="Sets the CICCENB field."]
    #[inline] pub fn set_ciccenb<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Channel n Polarity Buffer"]
    #[inline] pub fn polb<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if POLB != 0"]
    #[inline] pub fn test_polb<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.polb(index) != 0
    }

    #[doc="Sets the POLB field."]
    #[inline] pub fn set_polb<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Swap DTI Output Pair n Buffer"]
    #[inline] pub fn swapb<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 24 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SWAPB != 0"]
    #[inline] pub fn test_swapb<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.swapb(index) != 0
    }

    #[doc="Sets the SWAPB field."]
    #[inline] pub fn set_swapb<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 24 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Waveb {
    #[inline]
    fn from(other: u32) -> Self {
         Waveb(other)
    }
}

impl ::core::fmt::Display for Waveb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Waveb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wavegenb() != 0 { try!(write!(f, " wavegenb=0x{:x}", self.wavegenb()))}
        if self.rampb() != 0 { try!(write!(f, " rampb=0x{:x}", self.rampb()))}
        if self.ciperenb() != 0 { try!(write!(f, " ciperenb"))}
        if self.ciccenb(0) != 0 { try!(write!(f, " ciccenb[0]"))}
        if self.ciccenb(1) != 0 { try!(write!(f, " ciccenb[1]"))}
        if self.ciccenb(2) != 0 { try!(write!(f, " ciccenb[2]"))}
        if self.ciccenb(3) != 0 { try!(write!(f, " ciccenb[3]"))}
        if self.polb(0) != 0 { try!(write!(f, " polb[0]"))}
        if self.polb(1) != 0 { try!(write!(f, " polb[1]"))}
        if self.polb(2) != 0 { try!(write!(f, " polb[2]"))}
        if self.polb(3) != 0 { try!(write!(f, " polb[3]"))}
        if self.swapb(0) != 0 { try!(write!(f, " swapb[0]"))}
        if self.swapb(1) != 0 { try!(write!(f, " swapb[1]"))}
        if self.swapb(2) != 0 { try!(write!(f, " swapb[2]"))}
        if self.swapb(3) != 0 { try!(write!(f, " swapb[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Waveform Extension Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wexctrl(pub u32);
impl Wexctrl {
    #[doc="Output Matrix"]
    #[inline] pub fn otmx(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if OTMX != 0"]
    #[inline] pub fn test_otmx(&self) -> bool {
        self.otmx() != 0
    }

    #[doc="Sets the OTMX field."]
    #[inline] pub fn set_otmx<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Dead-time Insertion Generator n Enable"]
    #[inline] pub fn dtien<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DTIEN != 0"]
    #[inline] pub fn test_dtien<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.dtien(index) != 0
    }

    #[doc="Sets the DTIEN field."]
    #[inline] pub fn set_dtien<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Dead-time Low Side Outputs Value"]
    #[inline] pub fn dtls(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DTLS != 0"]
    #[inline] pub fn test_dtls(&self) -> bool {
        self.dtls() != 0
    }

    #[doc="Sets the DTLS field."]
    #[inline] pub fn set_dtls<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Dead-time High Side Outputs Value"]
    #[inline] pub fn dths(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DTHS != 0"]
    #[inline] pub fn test_dths(&self) -> bool {
        self.dths() != 0
    }

    #[doc="Sets the DTHS field."]
    #[inline] pub fn set_dths<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Wexctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Wexctrl(other)
    }
}

impl ::core::fmt::Display for Wexctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wexctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.otmx() != 0 { try!(write!(f, " otmx=0x{:x}", self.otmx()))}
        if self.dtien(0) != 0 { try!(write!(f, " dtien[0]"))}
        if self.dtien(1) != 0 { try!(write!(f, " dtien[1]"))}
        if self.dtien(2) != 0 { try!(write!(f, " dtien[2]"))}
        if self.dtien(3) != 0 { try!(write!(f, " dtien[3]"))}
        if self.dtls() != 0 { try!(write!(f, " dtls=0x{:x}", self.dtls()))}
        if self.dths() != 0 { try!(write!(f, " dths=0x{:x}", self.dths()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

