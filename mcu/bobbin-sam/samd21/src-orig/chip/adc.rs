//! Analog-to-Digital Converter

periph!( ADC, Adc, ADC_PERIPH, AdcPeriph, 0x42004000, 0x08);

pub struct AdcCh { pub periph: AdcPeriph, pub index: usize }
channel!(ADC_CH0, AdcCh0, ADC, Adc, ADC_CH0_CH, AdcCh, ADC_PERIPH, 0);
channel!(ADC_CH1, AdcCh1, ADC, Adc, ADC_CH1_CH, AdcCh, ADC_PERIPH, 1);
channel!(ADC_CH2, AdcCh2, ADC, Adc, ADC_CH2_CH, AdcCh, ADC_PERIPH, 2);
channel!(ADC_CH3, AdcCh3, ADC, Adc, ADC_CH3_CH, AdcCh, ADC_PERIPH, 3);
channel!(ADC_CH4, AdcCh4, ADC, Adc, ADC_CH4_CH, AdcCh, ADC_PERIPH, 4);
channel!(ADC_CH5, AdcCh5, ADC, Adc, ADC_CH5_CH, AdcCh, ADC_PERIPH, 5);
channel!(ADC_CH6, AdcCh6, ADC, Adc, ADC_CH6_CH, AdcCh, ADC_PERIPH, 6);
channel!(ADC_CH7, AdcCh7, ADC, Adc, ADC_CH7_CH, AdcCh, ADC_PERIPH, 7);
channel!(ADC_CH8, AdcCh8, ADC, Adc, ADC_CH8_CH, AdcCh, ADC_PERIPH, 8);
channel!(ADC_CH9, AdcCh9, ADC, Adc, ADC_CH9_CH, AdcCh, ADC_PERIPH, 9);
channel!(ADC_CH10, AdcCh10, ADC, Adc, ADC_CH10_CH, AdcCh, ADC_PERIPH, 10);
channel!(ADC_CH11, AdcCh11, ADC, Adc, ADC_CH11_CH, AdcCh, ADC_PERIPH, 11);
channel!(ADC_CH12, AdcCh12, ADC, Adc, ADC_CH12_CH, AdcCh, ADC_PERIPH, 12);
channel!(ADC_CH13, AdcCh13, ADC, Adc, ADC_CH13_CH, AdcCh, ADC_PERIPH, 13);
channel!(ADC_CH14, AdcCh14, ADC, Adc, ADC_CH14_CH, AdcCh, ADC_PERIPH, 14);
channel!(ADC_CH15, AdcCh15, ADC, Adc, ADC_CH15_CH, AdcCh, ADC_PERIPH, 15);
channel!(ADC_CH16, AdcCh16, ADC, Adc, ADC_CH16_CH, AdcCh, ADC_PERIPH, 16);
channel!(ADC_CH17, AdcCh17, ADC, Adc, ADC_CH17_CH, AdcCh, ADC_PERIPH, 17);
channel!(ADC_CH18, AdcCh18, ADC, Adc, ADC_CH18_CH, AdcCh, ADC_PERIPH, 18);
channel!(ADC_CH19, AdcCh19, ADC, Adc, ADC_CH19_CH, AdcCh, ADC_PERIPH, 19);
channel!(ADC_TEMP, AdcTemp, ADC, Adc, ADC_TEMP_CH, AdcCh, ADC_PERIPH, 24);
channel!(ADC_BANDGAP, AdcBandgap, ADC, Adc, ADC_BANDGAP_CH, AdcCh, ADC_PERIPH, 25);
channel!(ADC_SCALED_CORE, AdcScaledCore, ADC, Adc, ADC_SCALED_CORE_CH, AdcCh, ADC_PERIPH, 26);
channel!(ADC_SCALED_IO, AdcScaledIo, ADC, Adc, ADC_SCALED_IO_CH, AdcCh, ADC_PERIPH, 27);
channel!(ADC_DAC, AdcDac, ADC, Adc, ADC_DAC_CH, AdcCh, ADC_PERIPH, 28);
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 

impl AdcPeriph {
    #[doc="Get the *mut pointer for the AVGCTRL register."]
    #[inline] pub fn avgctrl_mut(&self) -> *mut Avgctrl { 
        (self.0 + 0x2) as *mut Avgctrl
    }

    #[doc="Get the *const pointer for the AVGCTRL register."]
    #[inline] pub fn avgctrl_ptr(&self) -> *const Avgctrl { 
           self.avgctrl_mut()
    }

    #[doc="Read the AVGCTRL register."]
    #[inline] pub fn avgctrl(&self) -> Avgctrl { 
        unsafe {
            read_volatile(self.avgctrl_ptr())
        }
    }

    #[doc="Write the AVGCTRL register."]
    #[inline] pub fn set_avgctrl<F: FnOnce(Avgctrl) -> Avgctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.avgctrl_mut(), f(Avgctrl(0)));
        }
        self
    }

    #[doc="Modify the AVGCTRL register."]
    #[inline] pub fn with_avgctrl<F: FnOnce(Avgctrl) -> Avgctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.avgctrl_mut(), f(self.avgctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CALIB register."]
    #[inline] pub fn calib_mut(&self) -> *mut Calib { 
        (self.0 + 0x28) as *mut Calib
    }

    #[doc="Get the *const pointer for the CALIB register."]
    #[inline] pub fn calib_ptr(&self) -> *const Calib { 
           self.calib_mut()
    }

    #[doc="Read the CALIB register."]
    #[inline] pub fn calib(&self) -> Calib { 
        unsafe {
            read_volatile(self.calib_ptr())
        }
    }

    #[doc="Write the CALIB register."]
    #[inline] pub fn set_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.calib_mut(), f(Calib(0)));
        }
        self
    }

    #[doc="Modify the CALIB register."]
    #[inline] pub fn with_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.calib_mut(), f(self.calib()));
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

    #[doc="Get the *mut pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_mut(&self) -> *mut Ctrlb { 
        (self.0 + 0x4) as *mut Ctrlb
    }

    #[doc="Get the *const pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_ptr(&self) -> *const Ctrlb { 
           self.ctrlb_mut()
    }

    #[doc="Read the CTRLB register."]
    #[inline] pub fn ctrlb(&self) -> Ctrlb { 
        unsafe {
            read_volatile(self.ctrlb_ptr())
        }
    }

    #[doc="Write the CTRLB register."]
    #[inline] pub fn set_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlb_mut(), f(Ctrlb(0)));
        }
        self
    }

    #[doc="Modify the CTRLB register."]
    #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlb_mut(), f(self.ctrlb()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_mut(&self) -> *mut Dbgctrl { 
        (self.0 + 0x2a) as *mut Dbgctrl
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

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        (self.0 + 0x14) as *mut Evctrl
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

    #[doc="Get the *mut pointer for the GAINCORR register."]
    #[inline] pub fn gaincorr_mut(&self) -> *mut Gaincorr { 
        (self.0 + 0x24) as *mut Gaincorr
    }

    #[doc="Get the *const pointer for the GAINCORR register."]
    #[inline] pub fn gaincorr_ptr(&self) -> *const Gaincorr { 
           self.gaincorr_mut()
    }

    #[doc="Read the GAINCORR register."]
    #[inline] pub fn gaincorr(&self) -> Gaincorr { 
        unsafe {
            read_volatile(self.gaincorr_ptr())
        }
    }

    #[doc="Write the GAINCORR register."]
    #[inline] pub fn set_gaincorr<F: FnOnce(Gaincorr) -> Gaincorr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.gaincorr_mut(), f(Gaincorr(0)));
        }
        self
    }

    #[doc="Modify the GAINCORR register."]
    #[inline] pub fn with_gaincorr<F: FnOnce(Gaincorr) -> Gaincorr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.gaincorr_mut(), f(self.gaincorr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INPUTCTRL register."]
    #[inline] pub fn inputctrl_mut(&self) -> *mut Inputctrl { 
        (self.0 + 0x10) as *mut Inputctrl
    }

    #[doc="Get the *const pointer for the INPUTCTRL register."]
    #[inline] pub fn inputctrl_ptr(&self) -> *const Inputctrl { 
           self.inputctrl_mut()
    }

    #[doc="Read the INPUTCTRL register."]
    #[inline] pub fn inputctrl(&self) -> Inputctrl { 
        unsafe {
            read_volatile(self.inputctrl_ptr())
        }
    }

    #[doc="Write the INPUTCTRL register."]
    #[inline] pub fn set_inputctrl<F: FnOnce(Inputctrl) -> Inputctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inputctrl_mut(), f(Inputctrl(0)));
        }
        self
    }

    #[doc="Modify the INPUTCTRL register."]
    #[inline] pub fn with_inputctrl<F: FnOnce(Inputctrl) -> Inputctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inputctrl_mut(), f(self.inputctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        (self.0 + 0x16) as *mut Intenclr
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
        (self.0 + 0x17) as *mut Intenset
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
        (self.0 + 0x18) as *mut Intflag
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

    #[doc="Get the *mut pointer for the OFFSETCORR register."]
    #[inline] pub fn offsetcorr_mut(&self) -> *mut Offsetcorr { 
        (self.0 + 0x26) as *mut Offsetcorr
    }

    #[doc="Get the *const pointer for the OFFSETCORR register."]
    #[inline] pub fn offsetcorr_ptr(&self) -> *const Offsetcorr { 
           self.offsetcorr_mut()
    }

    #[doc="Read the OFFSETCORR register."]
    #[inline] pub fn offsetcorr(&self) -> Offsetcorr { 
        unsafe {
            read_volatile(self.offsetcorr_ptr())
        }
    }

    #[doc="Write the OFFSETCORR register."]
    #[inline] pub fn set_offsetcorr<F: FnOnce(Offsetcorr) -> Offsetcorr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.offsetcorr_mut(), f(Offsetcorr(0)));
        }
        self
    }

    #[doc="Modify the OFFSETCORR register."]
    #[inline] pub fn with_offsetcorr<F: FnOnce(Offsetcorr) -> Offsetcorr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.offsetcorr_mut(), f(self.offsetcorr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the REFCTRL register."]
    #[inline] pub fn refctrl_mut(&self) -> *mut Refctrl { 
        (self.0 + 0x1) as *mut Refctrl
    }

    #[doc="Get the *const pointer for the REFCTRL register."]
    #[inline] pub fn refctrl_ptr(&self) -> *const Refctrl { 
           self.refctrl_mut()
    }

    #[doc="Read the REFCTRL register."]
    #[inline] pub fn refctrl(&self) -> Refctrl { 
        unsafe {
            read_volatile(self.refctrl_ptr())
        }
    }

    #[doc="Write the REFCTRL register."]
    #[inline] pub fn set_refctrl<F: FnOnce(Refctrl) -> Refctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.refctrl_mut(), f(Refctrl(0)));
        }
        self
    }

    #[doc="Modify the REFCTRL register."]
    #[inline] pub fn with_refctrl<F: FnOnce(Refctrl) -> Refctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.refctrl_mut(), f(self.refctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RESULT register."]
    #[inline] pub fn result_mut(&self) -> *mut Result { 
        (self.0 + 0x1a) as *mut Result
    }

    #[doc="Get the *const pointer for the RESULT register."]
    #[inline] pub fn result_ptr(&self) -> *const Result { 
           self.result_mut()
    }

    #[doc="Read the RESULT register."]
    #[inline] pub fn result(&self) -> Result { 
        unsafe {
            read_volatile(self.result_ptr())
        }
    }

    #[doc="Get the *mut pointer for the SAMPCTRL register."]
    #[inline] pub fn sampctrl_mut(&self) -> *mut Sampctrl { 
        (self.0 + 0x3) as *mut Sampctrl
    }

    #[doc="Get the *const pointer for the SAMPCTRL register."]
    #[inline] pub fn sampctrl_ptr(&self) -> *const Sampctrl { 
           self.sampctrl_mut()
    }

    #[doc="Read the SAMPCTRL register."]
    #[inline] pub fn sampctrl(&self) -> Sampctrl { 
        unsafe {
            read_volatile(self.sampctrl_ptr())
        }
    }

    #[doc="Write the SAMPCTRL register."]
    #[inline] pub fn set_sampctrl<F: FnOnce(Sampctrl) -> Sampctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sampctrl_mut(), f(Sampctrl(0)));
        }
        self
    }

    #[doc="Modify the SAMPCTRL register."]
    #[inline] pub fn with_sampctrl<F: FnOnce(Sampctrl) -> Sampctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sampctrl_mut(), f(self.sampctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0x19) as *mut Status
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

    #[doc="Get the *mut pointer for the SWTRIG register."]
    #[inline] pub fn swtrig_mut(&self) -> *mut Swtrig { 
        (self.0 + 0xc) as *mut Swtrig
    }

    #[doc="Get the *const pointer for the SWTRIG register."]
    #[inline] pub fn swtrig_ptr(&self) -> *const Swtrig { 
           self.swtrig_mut()
    }

    #[doc="Read the SWTRIG register."]
    #[inline] pub fn swtrig(&self) -> Swtrig { 
        unsafe {
            read_volatile(self.swtrig_ptr())
        }
    }

    #[doc="Write the SWTRIG register."]
    #[inline] pub fn set_swtrig<F: FnOnce(Swtrig) -> Swtrig>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swtrig_mut(), f(Swtrig(0)));
        }
        self
    }

    #[doc="Modify the SWTRIG register."]
    #[inline] pub fn with_swtrig<F: FnOnce(Swtrig) -> Swtrig>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swtrig_mut(), f(self.swtrig()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WINCTRL register."]
    #[inline] pub fn winctrl_mut(&self) -> *mut Winctrl { 
        (self.0 + 0x8) as *mut Winctrl
    }

    #[doc="Get the *const pointer for the WINCTRL register."]
    #[inline] pub fn winctrl_ptr(&self) -> *const Winctrl { 
           self.winctrl_mut()
    }

    #[doc="Read the WINCTRL register."]
    #[inline] pub fn winctrl(&self) -> Winctrl { 
        unsafe {
            read_volatile(self.winctrl_ptr())
        }
    }

    #[doc="Write the WINCTRL register."]
    #[inline] pub fn set_winctrl<F: FnOnce(Winctrl) -> Winctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.winctrl_mut(), f(Winctrl(0)));
        }
        self
    }

    #[doc="Modify the WINCTRL register."]
    #[inline] pub fn with_winctrl<F: FnOnce(Winctrl) -> Winctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.winctrl_mut(), f(self.winctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WINLT register."]
    #[inline] pub fn winlt_mut(&self) -> *mut Winlt { 
        (self.0 + 0x1c) as *mut Winlt
    }

    #[doc="Get the *const pointer for the WINLT register."]
    #[inline] pub fn winlt_ptr(&self) -> *const Winlt { 
           self.winlt_mut()
    }

    #[doc="Read the WINLT register."]
    #[inline] pub fn winlt(&self) -> Winlt { 
        unsafe {
            read_volatile(self.winlt_ptr())
        }
    }

    #[doc="Write the WINLT register."]
    #[inline] pub fn set_winlt<F: FnOnce(Winlt) -> Winlt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.winlt_mut(), f(Winlt(0)));
        }
        self
    }

    #[doc="Modify the WINLT register."]
    #[inline] pub fn with_winlt<F: FnOnce(Winlt) -> Winlt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.winlt_mut(), f(self.winlt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WINUT register."]
    #[inline] pub fn winut_mut(&self) -> *mut Winut { 
        (self.0 + 0x20) as *mut Winut
    }

    #[doc="Get the *const pointer for the WINUT register."]
    #[inline] pub fn winut_ptr(&self) -> *const Winut { 
           self.winut_mut()
    }

    #[doc="Read the WINUT register."]
    #[inline] pub fn winut(&self) -> Winut { 
        unsafe {
            read_volatile(self.winut_ptr())
        }
    }

    #[doc="Write the WINUT register."]
    #[inline] pub fn set_winut<F: FnOnce(Winut) -> Winut>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.winut_mut(), f(Winut(0)));
        }
        self
    }

    #[doc="Modify the WINUT register."]
    #[inline] pub fn with_winut<F: FnOnce(Winut) -> Winut>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.winut_mut(), f(self.winut()));
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

    #[doc="Returns true if SAMPLENUM != 0"]
    #[inline] pub fn test_samplenum(&self) -> bool {
        self.samplenum() != 0
    }

    #[doc="Sets the SAMPLENUM field."]
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

    #[doc="Returns true if ADJRES != 0"]
    #[inline] pub fn test_adjres(&self) -> bool {
        self.adjres() != 0
    }

    #[doc="Sets the ADJRES field."]
    #[inline] pub fn set_adjres<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Avgctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Avgctrl(other)
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

    #[doc="Returns true if LINEARITY_CAL != 0"]
    #[inline] pub fn test_linearity_cal(&self) -> bool {
        self.linearity_cal() != 0
    }

    #[doc="Sets the LINEARITY_CAL field."]
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

    #[doc="Returns true if BIAS_CAL != 0"]
    #[inline] pub fn test_bias_cal(&self) -> bool {
        self.bias_cal() != 0
    }

    #[doc="Sets the BIAS_CAL field."]
    #[inline] pub fn set_bias_cal<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Calib {
    #[inline]
    fn from(other: u16) -> Self {
         Calib(other)
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

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
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

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
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

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Ctrla {
    #[inline]
    fn from(other: u8) -> Self {
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

    #[doc="Returns true if DIFFMODE != 0"]
    #[inline] pub fn test_diffmode(&self) -> bool {
        self.diffmode() != 0
    }

    #[doc="Sets the DIFFMODE field."]
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

    #[doc="Returns true if LEFTADJ != 0"]
    #[inline] pub fn test_leftadj(&self) -> bool {
        self.leftadj() != 0
    }

    #[doc="Sets the LEFTADJ field."]
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

    #[doc="Returns true if FREERUN != 0"]
    #[inline] pub fn test_freerun(&self) -> bool {
        self.freerun() != 0
    }

    #[doc="Sets the FREERUN field."]
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

    #[doc="Returns true if CORREN != 0"]
    #[inline] pub fn test_corren(&self) -> bool {
        self.corren() != 0
    }

    #[doc="Sets the CORREN field."]
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

    #[doc="Returns true if RESSEL != 0"]
    #[inline] pub fn test_ressel(&self) -> bool {
        self.ressel() != 0
    }

    #[doc="Sets the RESSEL field."]
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

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Ctrlb {
    #[inline]
    fn from(other: u16) -> Self {
         Ctrlb(other)
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

    #[doc="Returns true if STARTEI != 0"]
    #[inline] pub fn test_startei(&self) -> bool {
        self.startei() != 0
    }

    #[doc="Sets the STARTEI field."]
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

    #[doc="Returns true if SYNCEI != 0"]
    #[inline] pub fn test_syncei(&self) -> bool {
        self.syncei() != 0
    }

    #[doc="Sets the SYNCEI field."]
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

    #[doc="Returns true if RESRDYEO != 0"]
    #[inline] pub fn test_resrdyeo(&self) -> bool {
        self.resrdyeo() != 0
    }

    #[doc="Sets the RESRDYEO field."]
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

    #[doc="Returns true if WINMONEO != 0"]
    #[inline] pub fn test_winmoneo(&self) -> bool {
        self.winmoneo() != 0
    }

    #[doc="Sets the WINMONEO field."]
    #[inline] pub fn set_winmoneo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Evctrl {
    #[inline]
    fn from(other: u8) -> Self {
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

    #[doc="Returns true if GAINCORR != 0"]
    #[inline] pub fn test_gaincorr(&self) -> bool {
        self.gaincorr() != 0
    }

    #[doc="Sets the GAINCORR field."]
    #[inline] pub fn set_gaincorr<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Gaincorr {
    #[inline]
    fn from(other: u16) -> Self {
         Gaincorr(other)
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

    #[doc="Returns true if MUXPOS != 0"]
    #[inline] pub fn test_muxpos(&self) -> bool {
        self.muxpos() != 0
    }

    #[doc="Sets the MUXPOS field."]
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

    #[doc="Returns true if MUXNEG != 0"]
    #[inline] pub fn test_muxneg(&self) -> bool {
        self.muxneg() != 0
    }

    #[doc="Sets the MUXNEG field."]
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

    #[doc="Returns true if INPUTSCAN != 0"]
    #[inline] pub fn test_inputscan(&self) -> bool {
        self.inputscan() != 0
    }

    #[doc="Sets the INPUTSCAN field."]
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

    #[doc="Returns true if INPUTOFFSET != 0"]
    #[inline] pub fn test_inputoffset(&self) -> bool {
        self.inputoffset() != 0
    }

    #[doc="Sets the INPUTOFFSET field."]
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

    #[doc="Returns true if GAIN != 0"]
    #[inline] pub fn test_gain(&self) -> bool {
        self.gain() != 0
    }

    #[doc="Sets the GAIN field."]
    #[inline] pub fn set_gain<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Inputctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Inputctrl(other)
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

    #[doc="Returns true if RESRDY != 0"]
    #[inline] pub fn test_resrdy(&self) -> bool {
        self.resrdy() != 0
    }

    #[doc="Sets the RESRDY field."]
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

    #[doc="Returns true if OVERRUN != 0"]
    #[inline] pub fn test_overrun(&self) -> bool {
        self.overrun() != 0
    }

    #[doc="Sets the OVERRUN field."]
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

    #[doc="Returns true if WINMON != 0"]
    #[inline] pub fn test_winmon(&self) -> bool {
        self.winmon() != 0
    }

    #[doc="Sets the WINMON field."]
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

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Intenclr {
    #[inline]
    fn from(other: u8) -> Self {
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

    #[doc="Returns true if RESRDY != 0"]
    #[inline] pub fn test_resrdy(&self) -> bool {
        self.resrdy() != 0
    }

    #[doc="Sets the RESRDY field."]
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

    #[doc="Returns true if OVERRUN != 0"]
    #[inline] pub fn test_overrun(&self) -> bool {
        self.overrun() != 0
    }

    #[doc="Sets the OVERRUN field."]
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

    #[doc="Returns true if WINMON != 0"]
    #[inline] pub fn test_winmon(&self) -> bool {
        self.winmon() != 0
    }

    #[doc="Sets the WINMON field."]
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

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Intenset {
    #[inline]
    fn from(other: u8) -> Self {
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

    #[doc="Returns true if RESRDY != 0"]
    #[inline] pub fn test_resrdy(&self) -> bool {
        self.resrdy() != 0
    }

    #[doc="Sets the RESRDY field."]
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

    #[doc="Returns true if OVERRUN != 0"]
    #[inline] pub fn test_overrun(&self) -> bool {
        self.overrun() != 0
    }

    #[doc="Sets the OVERRUN field."]
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

    #[doc="Returns true if WINMON != 0"]
    #[inline] pub fn test_winmon(&self) -> bool {
        self.winmon() != 0
    }

    #[doc="Sets the WINMON field."]
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

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Intflag {
    #[inline]
    fn from(other: u8) -> Self {
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

    #[doc="Returns true if OFFSETCORR != 0"]
    #[inline] pub fn test_offsetcorr(&self) -> bool {
        self.offsetcorr() != 0
    }

    #[doc="Sets the OFFSETCORR field."]
    #[inline] pub fn set_offsetcorr<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Offsetcorr {
    #[inline]
    fn from(other: u16) -> Self {
         Offsetcorr(other)
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

    #[doc="Returns true if REFSEL != 0"]
    #[inline] pub fn test_refsel(&self) -> bool {
        self.refsel() != 0
    }

    #[doc="Sets the REFSEL field."]
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

    #[doc="Returns true if REFCOMP != 0"]
    #[inline] pub fn test_refcomp(&self) -> bool {
        self.refcomp() != 0
    }

    #[doc="Sets the REFCOMP field."]
    #[inline] pub fn set_refcomp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Refctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Refctrl(other)
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

    #[doc="Returns true if RESULT != 0"]
    #[inline] pub fn test_result(&self) -> bool {
        self.result() != 0
    }

    #[doc="Sets the RESULT field."]
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

    #[doc="Returns true if RESULT_16 != 0"]
    #[inline] pub fn test_result_16(&self) -> bool {
        self.result_16() != 0
    }

    #[doc="Sets the RESULT_16 field."]
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

    #[doc="Returns true if RESULT_12 != 0"]
    #[inline] pub fn test_result_12(&self) -> bool {
        self.result_12() != 0
    }

    #[doc="Sets the RESULT_12 field."]
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

    #[doc="Returns true if RESULT_10 != 0"]
    #[inline] pub fn test_result_10(&self) -> bool {
        self.result_10() != 0
    }

    #[doc="Sets the RESULT_10 field."]
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

    #[doc="Returns true if RESULT_8 != 0"]
    #[inline] pub fn test_result_8(&self) -> bool {
        self.result_8() != 0
    }

    #[doc="Sets the RESULT_8 field."]
    #[inline] pub fn set_result_8<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Result {
    #[inline]
    fn from(other: u16) -> Self {
         Result(other)
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

    #[doc="Returns true if SAMPLEN != 0"]
    #[inline] pub fn test_samplen(&self) -> bool {
        self.samplen() != 0
    }

    #[doc="Sets the SAMPLEN field."]
    #[inline] pub fn set_samplen<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Sampctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Sampctrl(other)
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

    #[doc="Returns true if SYNCBUSY != 0"]
    #[inline] pub fn test_syncbusy(&self) -> bool {
        self.syncbusy() != 0
    }

    #[doc="Sets the SYNCBUSY field."]
    #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
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

    #[doc="Returns true if FLUSH != 0"]
    #[inline] pub fn test_flush(&self) -> bool {
        self.flush() != 0
    }

    #[doc="Sets the FLUSH field."]
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

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Swtrig {
    #[inline]
    fn from(other: u8) -> Self {
         Swtrig(other)
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

    #[doc="Returns true if WINMODE != 0"]
    #[inline] pub fn test_winmode(&self) -> bool {
        self.winmode() != 0
    }

    #[doc="Sets the WINMODE field."]
    #[inline] pub fn set_winmode<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Winctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Winctrl(other)
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

    #[doc="Returns true if WINLT != 0"]
    #[inline] pub fn test_winlt(&self) -> bool {
        self.winlt() != 0
    }

    #[doc="Sets the WINLT field."]
    #[inline] pub fn set_winlt<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Winlt {
    #[inline]
    fn from(other: u16) -> Self {
         Winlt(other)
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

    #[doc="Returns true if WINUT != 0"]
    #[inline] pub fn test_winut(&self) -> bool {
        self.winut() != 0
    }

    #[doc="Sets the WINUT field."]
    #[inline] pub fn set_winut<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Winut {
    #[inline]
    fn from(other: u16) -> Self {
         Winut(other)
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

