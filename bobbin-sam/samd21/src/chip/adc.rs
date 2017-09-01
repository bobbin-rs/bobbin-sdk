//! Analog-to-Digital Converter
#[allow(unused_imports)] use bobbin_common::*;

periph!( ADC, Adc, _ADC, AdcPeriph, 0x42004000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 

impl super::sig::Signal<super::sig::Ain0> for AdcCh0 {}
impl super::sig::SignalAin<super::sig::Ain0> for AdcCh0 {}
impl super::sig::Signal<super::sig::Ain1> for AdcCh1 {}
impl super::sig::SignalAin<super::sig::Ain1> for AdcCh1 {}
impl super::sig::Signal<super::sig::Ain2> for AdcCh2 {}
impl super::sig::SignalAin<super::sig::Ain2> for AdcCh2 {}
impl super::sig::Signal<super::sig::Ain3> for AdcCh3 {}
impl super::sig::SignalAin<super::sig::Ain3> for AdcCh3 {}
impl super::sig::Signal<super::sig::Ain4> for AdcCh4 {}
impl super::sig::SignalAin<super::sig::Ain4> for AdcCh4 {}
impl super::sig::Signal<super::sig::Ain5> for AdcCh5 {}
impl super::sig::SignalAin<super::sig::Ain5> for AdcCh5 {}
impl super::sig::Signal<super::sig::Ain6> for AdcCh6 {}
impl super::sig::SignalAin<super::sig::Ain6> for AdcCh6 {}
impl super::sig::Signal<super::sig::Ain7> for AdcCh7 {}
impl super::sig::SignalAin<super::sig::Ain7> for AdcCh7 {}
impl super::sig::Signal<super::sig::Ain8> for AdcCh8 {}
impl super::sig::SignalAin<super::sig::Ain8> for AdcCh8 {}
impl super::sig::Signal<super::sig::Ain9> for AdcCh9 {}
impl super::sig::SignalAin<super::sig::Ain9> for AdcCh9 {}
impl super::sig::Signal<super::sig::Ain10> for AdcCh10 {}
impl super::sig::SignalAin<super::sig::Ain10> for AdcCh10 {}
impl super::sig::Signal<super::sig::Ain11> for AdcCh11 {}
impl super::sig::SignalAin<super::sig::Ain11> for AdcCh11 {}
impl super::sig::Signal<super::sig::Ain12> for AdcCh12 {}
impl super::sig::SignalAin<super::sig::Ain12> for AdcCh12 {}
impl super::sig::Signal<super::sig::Ain13> for AdcCh13 {}
impl super::sig::SignalAin<super::sig::Ain13> for AdcCh13 {}
impl super::sig::Signal<super::sig::Ain14> for AdcCh14 {}
impl super::sig::SignalAin<super::sig::Ain14> for AdcCh14 {}
impl super::sig::Signal<super::sig::Ain15> for AdcCh15 {}
impl super::sig::SignalAin<super::sig::Ain15> for AdcCh15 {}
impl super::sig::Signal<super::sig::Ain16> for AdcCh16 {}
impl super::sig::SignalAin<super::sig::Ain16> for AdcCh16 {}
impl super::sig::Signal<super::sig::Ain17> for AdcCh17 {}
impl super::sig::SignalAin<super::sig::Ain17> for AdcCh17 {}
impl super::sig::Signal<super::sig::Ain18> for AdcCh18 {}
impl super::sig::SignalAin<super::sig::Ain18> for AdcCh18 {}
impl super::sig::Signal<super::sig::Ain19> for AdcCh19 {}
impl super::sig::SignalAin<super::sig::Ain19> for AdcCh19 {}


impl AdcPeriph {
    #[doc="Get the *const pointer for the AVGCTRL register."]
    #[inline] pub fn avgctrl_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x2) as *const u8
    }

    #[doc="Get the *mut pointer for the AVGCTRL register."]
    #[inline] pub fn avgctrl_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x2) as *mut u8
    }

    #[doc="Read the AVGCTRL register."]
    #[inline] pub fn avgctrl(&self) -> Avgctrl { 
        unsafe {
            Avgctrl(read_volatile((self.0 + 0x2) as *const u8))
        }
    }

    #[doc="Write the AVGCTRL register."]
    #[inline] pub fn set_avgctrl<F: FnOnce(Avgctrl) -> Avgctrl>(&self, f: F) -> &Self {
        let value = f(Avgctrl(0));
        unsafe {
            write_volatile((self.0 + 0x2) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the AVGCTRL register."]
    #[inline] pub fn with_avgctrl<F: FnOnce(Avgctrl) -> Avgctrl>(&self, f: F) -> &Self {
        let tmp = self.avgctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x2) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CALIB register."]
    #[inline] pub fn calib_ptr(&self) -> *const u16 { 
        ((self.0 as usize) + 0x28) as *const u16
    }

    #[doc="Get the *mut pointer for the CALIB register."]
    #[inline] pub fn calib_mut(&self) -> *mut u16 { 
        ((self.0 as usize) + 0x28) as *mut u16
    }

    #[doc="Read the CALIB register."]
    #[inline] pub fn calib(&self) -> Calib { 
        unsafe {
            Calib(read_volatile((self.0 + 0x28) as *const u16))
        }
    }

    #[doc="Write the CALIB register."]
    #[inline] pub fn set_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        let value = f(Calib(0));
        unsafe {
            write_volatile((self.0 + 0x28) as *mut u16, value.0);
        }
        self
    }

    #[doc="Modify the CALIB register."]
    #[inline] pub fn with_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        let tmp = self.calib();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x28) as *mut u16, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CTRLA register."]
    #[inline] pub fn ctrla_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x0) as *const u8
    }

    #[doc="Get the *mut pointer for the CTRLA register."]
    #[inline] pub fn ctrla_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x0) as *mut u8
    }

    #[doc="Read the CTRLA register."]
    #[inline] pub fn ctrla(&self) -> Ctrla { 
        unsafe {
            Ctrla(read_volatile((self.0 + 0x0) as *const u8))
        }
    }

    #[doc="Write the CTRLA register."]
    #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        let value = f(Ctrla(0));
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the CTRLA register."]
    #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        let tmp = self.ctrla();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_ptr(&self) -> *const u16 { 
        ((self.0 as usize) + 0x4) as *const u16
    }

    #[doc="Get the *mut pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_mut(&self) -> *mut u16 { 
        ((self.0 as usize) + 0x4) as *mut u16
    }

    #[doc="Read the CTRLB register."]
    #[inline] pub fn ctrlb(&self) -> Ctrlb { 
        unsafe {
            Ctrlb(read_volatile((self.0 + 0x4) as *const u16))
        }
    }

    #[doc="Write the CTRLB register."]
    #[inline] pub fn set_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
        let value = f(Ctrlb(0));
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u16, value.0);
        }
        self
    }

    #[doc="Modify the CTRLB register."]
    #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
        let tmp = self.ctrlb();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u16, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x2a) as *const u8
    }

    #[doc="Get the *mut pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x2a) as *mut u8
    }

    #[doc="Read the DBGCTRL register."]
    #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
        unsafe {
            Dbgctrl(read_volatile((self.0 + 0x2a) as *const u8))
        }
    }

    #[doc="Write the DBGCTRL register."]
    #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        let value = f(Dbgctrl(0));
        unsafe {
            write_volatile((self.0 + 0x2a) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the DBGCTRL register."]
    #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        let tmp = self.dbgctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x2a) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x14) as *const u8
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x14) as *mut u8
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        unsafe {
            Evctrl(read_volatile((self.0 + 0x14) as *const u8))
        }
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        let value = f(Evctrl(0));
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        let tmp = self.evctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the GAINCORR register."]
    #[inline] pub fn gaincorr_ptr(&self) -> *const u16 { 
        ((self.0 as usize) + 0x24) as *const u16
    }

    #[doc="Get the *mut pointer for the GAINCORR register."]
    #[inline] pub fn gaincorr_mut(&self) -> *mut u16 { 
        ((self.0 as usize) + 0x24) as *mut u16
    }

    #[doc="Read the GAINCORR register."]
    #[inline] pub fn gaincorr(&self) -> Gaincorr { 
        unsafe {
            Gaincorr(read_volatile((self.0 + 0x24) as *const u16))
        }
    }

    #[doc="Write the GAINCORR register."]
    #[inline] pub fn set_gaincorr<F: FnOnce(Gaincorr) -> Gaincorr>(&self, f: F) -> &Self {
        let value = f(Gaincorr(0));
        unsafe {
            write_volatile((self.0 + 0x24) as *mut u16, value.0);
        }
        self
    }

    #[doc="Modify the GAINCORR register."]
    #[inline] pub fn with_gaincorr<F: FnOnce(Gaincorr) -> Gaincorr>(&self, f: F) -> &Self {
        let tmp = self.gaincorr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x24) as *mut u16, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the INPUTCTRL register."]
    #[inline] pub fn inputctrl_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x10) as *const u32
    }

    #[doc="Get the *mut pointer for the INPUTCTRL register."]
    #[inline] pub fn inputctrl_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x10) as *mut u32
    }

    #[doc="Read the INPUTCTRL register."]
    #[inline] pub fn inputctrl(&self) -> Inputctrl { 
        unsafe {
            Inputctrl(read_volatile((self.0 + 0x10) as *const u32))
        }
    }

    #[doc="Write the INPUTCTRL register."]
    #[inline] pub fn set_inputctrl<F: FnOnce(Inputctrl) -> Inputctrl>(&self, f: F) -> &Self {
        let value = f(Inputctrl(0));
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the INPUTCTRL register."]
    #[inline] pub fn with_inputctrl<F: FnOnce(Inputctrl) -> Inputctrl>(&self, f: F) -> &Self {
        let tmp = self.inputctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x16) as *const u8
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x16) as *mut u8
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        unsafe {
            Intenclr(read_volatile((self.0 + 0x16) as *const u8))
        }
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        let value = f(Intenclr(0));
        unsafe {
            write_volatile((self.0 + 0x16) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        let tmp = self.intenclr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x16) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x17) as *const u8
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x17) as *mut u8
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        unsafe {
            Intenset(read_volatile((self.0 + 0x17) as *const u8))
        }
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        let value = f(Intenset(0));
        unsafe {
            write_volatile((self.0 + 0x17) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        let tmp = self.intenset();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x17) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x18) as *const u8
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x18) as *mut u8
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        unsafe {
            Intflag(read_volatile((self.0 + 0x18) as *const u8))
        }
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        let value = f(Intflag(0));
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        let tmp = self.intflag();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the OFFSETCORR register."]
    #[inline] pub fn offsetcorr_ptr(&self) -> *const u16 { 
        ((self.0 as usize) + 0x26) as *const u16
    }

    #[doc="Get the *mut pointer for the OFFSETCORR register."]
    #[inline] pub fn offsetcorr_mut(&self) -> *mut u16 { 
        ((self.0 as usize) + 0x26) as *mut u16
    }

    #[doc="Read the OFFSETCORR register."]
    #[inline] pub fn offsetcorr(&self) -> Offsetcorr { 
        unsafe {
            Offsetcorr(read_volatile((self.0 + 0x26) as *const u16))
        }
    }

    #[doc="Write the OFFSETCORR register."]
    #[inline] pub fn set_offsetcorr<F: FnOnce(Offsetcorr) -> Offsetcorr>(&self, f: F) -> &Self {
        let value = f(Offsetcorr(0));
        unsafe {
            write_volatile((self.0 + 0x26) as *mut u16, value.0);
        }
        self
    }

    #[doc="Modify the OFFSETCORR register."]
    #[inline] pub fn with_offsetcorr<F: FnOnce(Offsetcorr) -> Offsetcorr>(&self, f: F) -> &Self {
        let tmp = self.offsetcorr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x26) as *mut u16, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the REFCTRL register."]
    #[inline] pub fn refctrl_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x1) as *const u8
    }

    #[doc="Get the *mut pointer for the REFCTRL register."]
    #[inline] pub fn refctrl_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x1) as *mut u8
    }

    #[doc="Read the REFCTRL register."]
    #[inline] pub fn refctrl(&self) -> Refctrl { 
        unsafe {
            Refctrl(read_volatile((self.0 + 0x1) as *const u8))
        }
    }

    #[doc="Write the REFCTRL register."]
    #[inline] pub fn set_refctrl<F: FnOnce(Refctrl) -> Refctrl>(&self, f: F) -> &Self {
        let value = f(Refctrl(0));
        unsafe {
            write_volatile((self.0 + 0x1) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the REFCTRL register."]
    #[inline] pub fn with_refctrl<F: FnOnce(Refctrl) -> Refctrl>(&self, f: F) -> &Self {
        let tmp = self.refctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x1) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RESULT register."]
    #[inline] pub fn result_ptr(&self) -> *const u16 { 
        ((self.0 as usize) + 0x1a) as *const u16
    }

    #[doc="Get the *mut pointer for the RESULT register."]
    #[inline] pub fn result_mut(&self) -> *mut u16 { 
        ((self.0 as usize) + 0x1a) as *mut u16
    }

    #[doc="Read the RESULT register."]
    #[inline] pub fn result(&self) -> Result { 
        unsafe {
            Result(read_volatile((self.0 + 0x1a) as *const u16))
        }
    }

    #[doc="Get the *const pointer for the SAMPCTRL register."]
    #[inline] pub fn sampctrl_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x3) as *const u8
    }

    #[doc="Get the *mut pointer for the SAMPCTRL register."]
    #[inline] pub fn sampctrl_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x3) as *mut u8
    }

    #[doc="Read the SAMPCTRL register."]
    #[inline] pub fn sampctrl(&self) -> Sampctrl { 
        unsafe {
            Sampctrl(read_volatile((self.0 + 0x3) as *const u8))
        }
    }

    #[doc="Write the SAMPCTRL register."]
    #[inline] pub fn set_sampctrl<F: FnOnce(Sampctrl) -> Sampctrl>(&self, f: F) -> &Self {
        let value = f(Sampctrl(0));
        unsafe {
            write_volatile((self.0 + 0x3) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the SAMPCTRL register."]
    #[inline] pub fn with_sampctrl<F: FnOnce(Sampctrl) -> Sampctrl>(&self, f: F) -> &Self {
        let tmp = self.sampctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x3) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x19) as *const u8
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x19) as *mut u8
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            Status(read_volatile((self.0 + 0x19) as *const u8))
        }
    }

    #[doc="Get the *const pointer for the SWTRIG register."]
    #[inline] pub fn swtrig_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0xc) as *const u8
    }

    #[doc="Get the *mut pointer for the SWTRIG register."]
    #[inline] pub fn swtrig_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0xc) as *mut u8
    }

    #[doc="Read the SWTRIG register."]
    #[inline] pub fn swtrig(&self) -> Swtrig { 
        unsafe {
            Swtrig(read_volatile((self.0 + 0xc) as *const u8))
        }
    }

    #[doc="Write the SWTRIG register."]
    #[inline] pub fn set_swtrig<F: FnOnce(Swtrig) -> Swtrig>(&self, f: F) -> &Self {
        let value = f(Swtrig(0));
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the SWTRIG register."]
    #[inline] pub fn with_swtrig<F: FnOnce(Swtrig) -> Swtrig>(&self, f: F) -> &Self {
        let tmp = self.swtrig();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the WINCTRL register."]
    #[inline] pub fn winctrl_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x8) as *const u8
    }

    #[doc="Get the *mut pointer for the WINCTRL register."]
    #[inline] pub fn winctrl_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x8) as *mut u8
    }

    #[doc="Read the WINCTRL register."]
    #[inline] pub fn winctrl(&self) -> Winctrl { 
        unsafe {
            Winctrl(read_volatile((self.0 + 0x8) as *const u8))
        }
    }

    #[doc="Write the WINCTRL register."]
    #[inline] pub fn set_winctrl<F: FnOnce(Winctrl) -> Winctrl>(&self, f: F) -> &Self {
        let value = f(Winctrl(0));
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the WINCTRL register."]
    #[inline] pub fn with_winctrl<F: FnOnce(Winctrl) -> Winctrl>(&self, f: F) -> &Self {
        let tmp = self.winctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the WINLT register."]
    #[inline] pub fn winlt_ptr(&self) -> *const u16 { 
        ((self.0 as usize) + 0x1c) as *const u16
    }

    #[doc="Get the *mut pointer for the WINLT register."]
    #[inline] pub fn winlt_mut(&self) -> *mut u16 { 
        ((self.0 as usize) + 0x1c) as *mut u16
    }

    #[doc="Read the WINLT register."]
    #[inline] pub fn winlt(&self) -> Winlt { 
        unsafe {
            Winlt(read_volatile((self.0 + 0x1c) as *const u16))
        }
    }

    #[doc="Write the WINLT register."]
    #[inline] pub fn set_winlt<F: FnOnce(Winlt) -> Winlt>(&self, f: F) -> &Self {
        let value = f(Winlt(0));
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u16, value.0);
        }
        self
    }

    #[doc="Modify the WINLT register."]
    #[inline] pub fn with_winlt<F: FnOnce(Winlt) -> Winlt>(&self, f: F) -> &Self {
        let tmp = self.winlt();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u16, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the WINUT register."]
    #[inline] pub fn winut_ptr(&self) -> *const u16 { 
        ((self.0 as usize) + 0x20) as *const u16
    }

    #[doc="Get the *mut pointer for the WINUT register."]
    #[inline] pub fn winut_mut(&self) -> *mut u16 { 
        ((self.0 as usize) + 0x20) as *mut u16
    }

    #[doc="Read the WINUT register."]
    #[inline] pub fn winut(&self) -> Winut { 
        unsafe {
            Winut(read_volatile((self.0 + 0x20) as *const u16))
        }
    }

    #[doc="Write the WINUT register."]
    #[inline] pub fn set_winut<F: FnOnce(Winut) -> Winut>(&self, f: F) -> &Self {
        let value = f(Winut(0));
        unsafe {
            write_volatile((self.0 + 0x20) as *mut u16, value.0);
        }
        self
    }

    #[doc="Modify the WINUT register."]
    #[inline] pub fn with_winut<F: FnOnce(Winut) -> Winut>(&self, f: F) -> &Self {
        let tmp = self.winut();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x20) as *mut u16, value.0);
        }
        self
    }

}

#[doc="Average Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Avgctrl(pub u8);
impl Avgctrl {
    #[doc="Number of Samples to be Collected"]
    #[inline] pub fn samplenum(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Number of Samples to be Collected"]
    #[inline] pub fn test_samplenum(&self) -> bool {
        self.samplenum != 0
    }

    #[doc="Number of Samples to be Collected"]
    #[inline] pub fn set_samplenum<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Adjusting Result / Division Coefficient"]
    #[inline] pub fn adjres(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Adjusting Result / Division Coefficient"]
    #[inline] pub fn test_adjres(&self) -> bool {
        self.adjres != 0
    }

    #[doc="Adjusting Result / Division Coefficient"]
    #[inline] pub fn set_adjres<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

}

impl ::core::fmt::Display for Avgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Avgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.samplenum() != 0 { try!(write!(f, " samplenum=0x{:x}", self.samplenum()))}
        if self.adjres() != 0 { try!(write!(f, " adjres=0x{:x}", self.adjres()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Calibration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calib(pub u16);
impl Calib {
    #[doc="Linearity Calibration Value"]
    #[inline] pub fn linearity_cal(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Linearity Calibration Value"]
    #[inline] pub fn test_linearity_cal(&self) -> bool {
        self.linearity_cal != 0
    }

    #[doc="Linearity Calibration Value"]
    #[inline] pub fn set_linearity_cal<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bias Calibration Value"]
    #[inline] pub fn bias_cal(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Bias Calibration Value"]
    #[inline] pub fn test_bias_cal(&self) -> bool {
        self.bias_cal != 0
    }

    #[doc="Bias Calibration Value"]
    #[inline] pub fn set_bias_cal<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl ::core::fmt::Display for Calib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.linearity_cal() != 0 { try!(write!(f, " linearity_cal=0x{:x}", self.linearity_cal()))}
        if self.bias_cal() != 0 { try!(write!(f, " bias_cal=0x{:x}", self.bias_cal()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u8);
impl Ctrla {
    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Software Reset"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst != 0
    }

    #[doc="Software Reset"]
    #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Enable"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable != 0
    }

    #[doc="Enable"]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Run in Standby"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby != 0
    }

    #[doc="Run in Standby"]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
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
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u16);
impl Ctrlb {
    #[doc="Differential Mode"]
    #[inline] pub fn diffmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Differential Mode"]
    #[inline] pub fn test_diffmode(&self) -> bool {
        self.diffmode != 0
    }

    #[doc="Differential Mode"]
    #[inline] pub fn set_diffmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Left-Adjusted Result"]
    #[inline] pub fn leftadj(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Left-Adjusted Result"]
    #[inline] pub fn test_leftadj(&self) -> bool {
        self.leftadj != 0
    }

    #[doc="Left-Adjusted Result"]
    #[inline] pub fn set_leftadj<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Free Running Mode"]
    #[inline] pub fn freerun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Free Running Mode"]
    #[inline] pub fn test_freerun(&self) -> bool {
        self.freerun != 0
    }

    #[doc="Free Running Mode"]
    #[inline] pub fn set_freerun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Digital Correction Logic Enabled"]
    #[inline] pub fn corren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Digital Correction Logic Enabled"]
    #[inline] pub fn test_corren(&self) -> bool {
        self.corren != 0
    }

    #[doc="Digital Correction Logic Enabled"]
    #[inline] pub fn set_corren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Conversion Result Resolution"]
    #[inline] pub fn ressel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Conversion Result Resolution"]
    #[inline] pub fn test_ressel(&self) -> bool {
        self.ressel != 0
    }

    #[doc="Conversion Result Resolution"]
    #[inline] pub fn set_ressel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Prescaler Configuration"]
    #[inline] pub fn prescaler(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Prescaler Configuration"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler != 0
    }

    #[doc="Prescaler Configuration"]
    #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl ::core::fmt::Display for Ctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.diffmode() != 0 { try!(write!(f, " diffmode"))}
        if self.leftadj() != 0 { try!(write!(f, " leftadj"))}
        if self.freerun() != 0 { try!(write!(f, " freerun"))}
        if self.corren() != 0 { try!(write!(f, " corren"))}
        if self.ressel() != 0 { try!(write!(f, " ressel=0x{:x}", self.ressel()))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Run"]
    #[inline] pub fn dbgrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Debug Run"]
    #[inline] pub fn test_dbgrun(&self) -> bool {
        self.dbgrun != 0
    }

    #[doc="Debug Run"]
    #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u8);
impl Evctrl {
    #[doc="Start Conversion Event In"]
    #[inline] pub fn startei(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Start Conversion Event In"]
    #[inline] pub fn test_startei(&self) -> bool {
        self.startei != 0
    }

    #[doc="Start Conversion Event In"]
    #[inline] pub fn set_startei<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Synchronization Event In"]
    #[inline] pub fn syncei(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Synchronization Event In"]
    #[inline] pub fn test_syncei(&self) -> bool {
        self.syncei != 0
    }

    #[doc="Synchronization Event In"]
    #[inline] pub fn set_syncei<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Result Ready Event Out"]
    #[inline] pub fn resrdyeo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Result Ready Event Out"]
    #[inline] pub fn test_resrdyeo(&self) -> bool {
        self.resrdyeo != 0
    }

    #[doc="Result Ready Event Out"]
    #[inline] pub fn set_resrdyeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Window Monitor Event Out"]
    #[inline] pub fn winmoneo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Window Monitor Event Out"]
    #[inline] pub fn test_winmoneo(&self) -> bool {
        self.winmoneo != 0
    }

    #[doc="Window Monitor Event Out"]
    #[inline] pub fn set_winmoneo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
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
        if self.startei() != 0 { try!(write!(f, " startei"))}
        if self.syncei() != 0 { try!(write!(f, " syncei"))}
        if self.resrdyeo() != 0 { try!(write!(f, " resrdyeo"))}
        if self.winmoneo() != 0 { try!(write!(f, " winmoneo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Gain Correction"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gaincorr(pub u16);
impl Gaincorr {
    #[doc="Gain Correction Value"]
    #[inline] pub fn gaincorr(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Gain Correction Value"]
    #[inline] pub fn test_gaincorr(&self) -> bool {
        self.gaincorr != 0
    }

    #[doc="Gain Correction Value"]
    #[inline] pub fn set_gaincorr<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Gaincorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gaincorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gaincorr() != 0 { try!(write!(f, " gaincorr=0x{:x}", self.gaincorr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Input Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inputctrl(pub u32);
impl Inputctrl {
    #[doc="Positive Mux Input Selection"]
    #[inline] pub fn muxpos(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Positive Mux Input Selection"]
    #[inline] pub fn test_muxpos(&self) -> bool {
        self.muxpos != 0
    }

    #[doc="Positive Mux Input Selection"]
    #[inline] pub fn set_muxpos<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Negative Mux Input Selection"]
    #[inline] pub fn muxneg(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Negative Mux Input Selection"]
    #[inline] pub fn test_muxneg(&self) -> bool {
        self.muxneg != 0
    }

    #[doc="Negative Mux Input Selection"]
    #[inline] pub fn set_muxneg<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Number of Input Channels Included in Scan"]
    #[inline] pub fn inputscan(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Number of Input Channels Included in Scan"]
    #[inline] pub fn test_inputscan(&self) -> bool {
        self.inputscan != 0
    }

    #[doc="Number of Input Channels Included in Scan"]
    #[inline] pub fn set_inputscan<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Positive Mux Setting Offset"]
    #[inline] pub fn inputoffset(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Positive Mux Setting Offset"]
    #[inline] pub fn test_inputoffset(&self) -> bool {
        self.inputoffset != 0
    }

    #[doc="Positive Mux Setting Offset"]
    #[inline] pub fn set_inputoffset<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Gain Factor Selection"]
    #[inline] pub fn gain(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Gain Factor Selection"]
    #[inline] pub fn test_gain(&self) -> bool {
        self.gain != 0
    }

    #[doc="Gain Factor Selection"]
    #[inline] pub fn set_gain<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl ::core::fmt::Display for Inputctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Inputctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.muxpos() != 0 { try!(write!(f, " muxpos=0x{:x}", self.muxpos()))}
        if self.muxneg() != 0 { try!(write!(f, " muxneg=0x{:x}", self.muxneg()))}
        if self.inputscan() != 0 { try!(write!(f, " inputscan=0x{:x}", self.inputscan()))}
        if self.inputoffset() != 0 { try!(write!(f, " inputoffset=0x{:x}", self.inputoffset()))}
        if self.gain() != 0 { try!(write!(f, " gain=0x{:x}", self.gain()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Result Ready Interrupt Enable"]
    #[inline] pub fn resrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Result Ready Interrupt Enable"]
    #[inline] pub fn test_resrdy(&self) -> bool {
        self.resrdy != 0
    }

    #[doc="Result Ready Interrupt Enable"]
    #[inline] pub fn set_resrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn overrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn test_overrun(&self) -> bool {
        self.overrun != 0
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn set_overrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window Monitor Interrupt Enable"]
    #[inline] pub fn winmon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Window Monitor Interrupt Enable"]
    #[inline] pub fn test_winmon(&self) -> bool {
        self.winmon != 0
    }

    #[doc="Window Monitor Interrupt Enable"]
    #[inline] pub fn set_winmon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy != 0
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
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
        if self.resrdy() != 0 { try!(write!(f, " resrdy"))}
        if self.overrun() != 0 { try!(write!(f, " overrun"))}
        if self.winmon() != 0 { try!(write!(f, " winmon"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Result Ready Interrupt Enable"]
    #[inline] pub fn resrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Result Ready Interrupt Enable"]
    #[inline] pub fn test_resrdy(&self) -> bool {
        self.resrdy != 0
    }

    #[doc="Result Ready Interrupt Enable"]
    #[inline] pub fn set_resrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn overrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn test_overrun(&self) -> bool {
        self.overrun != 0
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn set_overrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window Monitor Interrupt Enable"]
    #[inline] pub fn winmon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Window Monitor Interrupt Enable"]
    #[inline] pub fn test_winmon(&self) -> bool {
        self.winmon != 0
    }

    #[doc="Window Monitor Interrupt Enable"]
    #[inline] pub fn set_winmon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy != 0
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
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
        if self.resrdy() != 0 { try!(write!(f, " resrdy"))}
        if self.overrun() != 0 { try!(write!(f, " overrun"))}
        if self.winmon() != 0 { try!(write!(f, " winmon"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Result Ready"]
    #[inline] pub fn resrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Result Ready"]
    #[inline] pub fn test_resrdy(&self) -> bool {
        self.resrdy != 0
    }

    #[doc="Result Ready"]
    #[inline] pub fn set_resrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overrun"]
    #[inline] pub fn overrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Overrun"]
    #[inline] pub fn test_overrun(&self) -> bool {
        self.overrun != 0
    }

    #[doc="Overrun"]
    #[inline] pub fn set_overrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window Monitor"]
    #[inline] pub fn winmon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Window Monitor"]
    #[inline] pub fn test_winmon(&self) -> bool {
        self.winmon != 0
    }

    #[doc="Window Monitor"]
    #[inline] pub fn set_winmon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Synchronization Ready"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Synchronization Ready"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy != 0
    }

    #[doc="Synchronization Ready"]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
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
        if self.resrdy() != 0 { try!(write!(f, " resrdy"))}
        if self.overrun() != 0 { try!(write!(f, " overrun"))}
        if self.winmon() != 0 { try!(write!(f, " winmon"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Offset Correction"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Offsetcorr(pub u16);
impl Offsetcorr {
    #[doc="Offset Correction Value"]
    #[inline] pub fn offsetcorr(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Offset Correction Value"]
    #[inline] pub fn test_offsetcorr(&self) -> bool {
        self.offsetcorr != 0
    }

    #[doc="Offset Correction Value"]
    #[inline] pub fn set_offsetcorr<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Offsetcorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Offsetcorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offsetcorr() != 0 { try!(write!(f, " offsetcorr=0x{:x}", self.offsetcorr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reference Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Refctrl(pub u8);
impl Refctrl {
    #[doc="Reference Selection"]
    #[inline] pub fn refsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Reference Selection"]
    #[inline] pub fn test_refsel(&self) -> bool {
        self.refsel != 0
    }

    #[doc="Reference Selection"]
    #[inline] pub fn set_refsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Reference Buffer Offset Compensation Enable"]
    #[inline] pub fn refcomp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Reference Buffer Offset Compensation Enable"]
    #[inline] pub fn test_refcomp(&self) -> bool {
        self.refcomp != 0
    }

    #[doc="Reference Buffer Offset Compensation Enable"]
    #[inline] pub fn set_refcomp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl ::core::fmt::Display for Refctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Refctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.refsel() != 0 { try!(write!(f, " refsel=0x{:x}", self.refsel()))}
        if self.refcomp() != 0 { try!(write!(f, " refcomp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Result"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Result(pub u16);
impl Result {
    #[doc="Result Conversion Value"]
    #[inline] pub fn result(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Result Conversion Value"]
    #[inline] pub fn test_result(&self) -> bool {
        self.result != 0
    }

    #[doc="Result Conversion Value"]
    #[inline] pub fn set_result<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Result Conversion Value (16 bits)"]
    #[inline] pub fn result_16(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Result Conversion Value (16 bits)"]
    #[inline] pub fn test_result_16(&self) -> bool {
        self.result_16 != 0
    }

    #[doc="Result Conversion Value (16 bits)"]
    #[inline] pub fn set_result_16<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Result Conversion Value (12 bits)"]
    #[inline] pub fn result_12(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Result Conversion Value (12 bits)"]
    #[inline] pub fn test_result_12(&self) -> bool {
        self.result_12 != 0
    }

    #[doc="Result Conversion Value (12 bits)"]
    #[inline] pub fn set_result_12<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Result Conversion Value (10 bits)"]
    #[inline] pub fn result_10(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Result Conversion Value (10 bits)"]
    #[inline] pub fn test_result_10(&self) -> bool {
        self.result_10 != 0
    }

    #[doc="Result Conversion Value (10 bits)"]
    #[inline] pub fn set_result_10<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Result Conversion Value (8 bits)"]
    #[inline] pub fn result_8(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Result Conversion Value (8 bits)"]
    #[inline] pub fn test_result_8(&self) -> bool {
        self.result_8 != 0
    }

    #[doc="Result Conversion Value (8 bits)"]
    #[inline] pub fn set_result_8<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Result {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Result {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.result() != 0 { try!(write!(f, " result=0x{:x}", self.result()))}
        if self.result_16() != 0 { try!(write!(f, " result_16=0x{:x}", self.result_16()))}
        if self.result_12() != 0 { try!(write!(f, " result_12=0x{:x}", self.result_12()))}
        if self.result_10() != 0 { try!(write!(f, " result_10=0x{:x}", self.result_10()))}
        if self.result_8() != 0 { try!(write!(f, " result_8=0x{:x}", self.result_8()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Sampling Time Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sampctrl(pub u8);
impl Sampctrl {
    #[doc="Sampling Time Length"]
    #[inline] pub fn samplen(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Sampling Time Length"]
    #[inline] pub fn test_samplen(&self) -> bool {
        self.samplen != 0
    }

    #[doc="Sampling Time Length"]
    #[inline] pub fn set_samplen<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Sampctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sampctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.samplen() != 0 { try!(write!(f, " samplen=0x{:x}", self.samplen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Synchronization Busy"]
    #[inline] pub fn syncbusy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Synchronization Busy"]
    #[inline] pub fn test_syncbusy(&self) -> bool {
        self.syncbusy != 0
    }

    #[doc="Synchronization Busy"]
    #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
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
        if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software Trigger"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swtrig(pub u8);
impl Swtrig {
    #[doc="ADC Conversion Flush"]
    #[inline] pub fn flush(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="ADC Conversion Flush"]
    #[inline] pub fn test_flush(&self) -> bool {
        self.flush != 0
    }

    #[doc="ADC Conversion Flush"]
    #[inline] pub fn set_flush<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC Start Conversion"]
    #[inline] pub fn start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="ADC Start Conversion"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start != 0
    }

    #[doc="ADC Start Conversion"]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl ::core::fmt::Display for Swtrig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swtrig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flush() != 0 { try!(write!(f, " flush"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Window Monitor Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winctrl(pub u8);
impl Winctrl {
    #[doc="Window Monitor Mode"]
    #[inline] pub fn winmode(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Window Monitor Mode"]
    #[inline] pub fn test_winmode(&self) -> bool {
        self.winmode != 0
    }

    #[doc="Window Monitor Mode"]
    #[inline] pub fn set_winmode<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Winctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.winmode() != 0 { try!(write!(f, " winmode=0x{:x}", self.winmode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Window Monitor Lower Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winlt(pub u16);
impl Winlt {
    #[doc="Window Lower Threshold"]
    #[inline] pub fn winlt(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Window Lower Threshold"]
    #[inline] pub fn test_winlt(&self) -> bool {
        self.winlt != 0
    }

    #[doc="Window Lower Threshold"]
    #[inline] pub fn set_winlt<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Winlt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winlt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.winlt() != 0 { try!(write!(f, " winlt=0x{:x}", self.winlt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Window Monitor Upper Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winut(pub u16);
impl Winut {
    #[doc="Window Upper Threshold"]
    #[inline] pub fn winut(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Window Upper Threshold"]
    #[inline] pub fn test_winut(&self) -> bool {
        self.winut != 0
    }

    #[doc="Window Upper Threshold"]
    #[inline] pub fn set_winut<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Winut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.winut() != 0 { try!(write!(f, " winut=0x{:x}", self.winut()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub struct AdcCh { pub periph: AdcPeriph, pub index: usize }
channel!(ADC_CH0, AdcCh0, ADC, Adc, _ADC_CH0, AdcCh, _ADC, 0);
channel!(ADC_CH1, AdcCh1, ADC, Adc, _ADC_CH1, AdcCh, _ADC, 1);
channel!(ADC_CH2, AdcCh2, ADC, Adc, _ADC_CH2, AdcCh, _ADC, 2);
channel!(ADC_CH3, AdcCh3, ADC, Adc, _ADC_CH3, AdcCh, _ADC, 3);
channel!(ADC_CH4, AdcCh4, ADC, Adc, _ADC_CH4, AdcCh, _ADC, 4);
channel!(ADC_CH5, AdcCh5, ADC, Adc, _ADC_CH5, AdcCh, _ADC, 5);
channel!(ADC_CH6, AdcCh6, ADC, Adc, _ADC_CH6, AdcCh, _ADC, 6);
channel!(ADC_CH7, AdcCh7, ADC, Adc, _ADC_CH7, AdcCh, _ADC, 7);
channel!(ADC_CH8, AdcCh8, ADC, Adc, _ADC_CH8, AdcCh, _ADC, 8);
channel!(ADC_CH9, AdcCh9, ADC, Adc, _ADC_CH9, AdcCh, _ADC, 9);
channel!(ADC_CH10, AdcCh10, ADC, Adc, _ADC_CH10, AdcCh, _ADC, 10);
channel!(ADC_CH11, AdcCh11, ADC, Adc, _ADC_CH11, AdcCh, _ADC, 11);
channel!(ADC_CH12, AdcCh12, ADC, Adc, _ADC_CH12, AdcCh, _ADC, 12);
channel!(ADC_CH13, AdcCh13, ADC, Adc, _ADC_CH13, AdcCh, _ADC, 13);
channel!(ADC_CH14, AdcCh14, ADC, Adc, _ADC_CH14, AdcCh, _ADC, 14);
channel!(ADC_CH15, AdcCh15, ADC, Adc, _ADC_CH15, AdcCh, _ADC, 15);
channel!(ADC_CH16, AdcCh16, ADC, Adc, _ADC_CH16, AdcCh, _ADC, 16);
channel!(ADC_CH17, AdcCh17, ADC, Adc, _ADC_CH17, AdcCh, _ADC, 17);
channel!(ADC_CH18, AdcCh18, ADC, Adc, _ADC_CH18, AdcCh, _ADC, 18);
channel!(ADC_CH19, AdcCh19, ADC, Adc, _ADC_CH19, AdcCh, _ADC, 19);

