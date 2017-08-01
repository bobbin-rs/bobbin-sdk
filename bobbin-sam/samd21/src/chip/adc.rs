//! Analog-to-Digital Converter
pub const ADC: Adc = Periph(0x42004000, AdcId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct AdcId {}
pub type Adc = Periph<AdcId>;



impl<T> Periph<T> {
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
        Avgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
     }
  }
#[doc="Write the AVGCTRL register."]
  #[inline] pub fn set_avgctrl(&self, value: Avgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the AVGCTRL register."]
  #[inline] pub fn with_avgctrl<F: FnOnce(Avgctrl) -> Avgctrl>(&self, f: F) -> &Self {
     let tmp = self.avgctrl();
     self.set_avgctrl(f(tmp))
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
        Calib(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u16))
     }
  }
#[doc="Write the CALIB register."]
  #[inline] pub fn set_calib(&self, value: Calib) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the CALIB register."]
  #[inline] pub fn with_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
     let tmp = self.calib();
     self.set_calib(f(tmp))
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
        Ctrla(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }
#[doc="Write the CTRLA register."]
  #[inline] pub fn set_ctrla(&self, value: Ctrla) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the CTRLA register."]
  #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
     let tmp = self.ctrla();
     self.set_ctrla(f(tmp))
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
        Ctrlb(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u16))
     }
  }
#[doc="Write the CTRLB register."]
  #[inline] pub fn set_ctrlb(&self, value: Ctrlb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the CTRLB register."]
  #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
     let tmp = self.ctrlb();
     self.set_ctrlb(f(tmp))
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
        Dbgctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x2a) as *const u8))
     }
  }
#[doc="Write the DBGCTRL register."]
  #[inline] pub fn set_dbgctrl(&self, value: Dbgctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2a) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the DBGCTRL register."]
  #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
     let tmp = self.dbgctrl();
     self.set_dbgctrl(f(tmp))
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
        Evctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u8))
     }
  }
#[doc="Write the EVCTRL register."]
  #[inline] pub fn set_evctrl(&self, value: Evctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the EVCTRL register."]
  #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
     let tmp = self.evctrl();
     self.set_evctrl(f(tmp))
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
        Gaincorr(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u16))
     }
  }
#[doc="Write the GAINCORR register."]
  #[inline] pub fn set_gaincorr(&self, value: Gaincorr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the GAINCORR register."]
  #[inline] pub fn with_gaincorr<F: FnOnce(Gaincorr) -> Gaincorr>(&self, f: F) -> &Self {
     let tmp = self.gaincorr();
     self.set_gaincorr(f(tmp))
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
        Inputctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the INPUTCTRL register."]
  #[inline] pub fn set_inputctrl(&self, value: Inputctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the INPUTCTRL register."]
  #[inline] pub fn with_inputctrl<F: FnOnce(Inputctrl) -> Inputctrl>(&self, f: F) -> &Self {
     let tmp = self.inputctrl();
     self.set_inputctrl(f(tmp))
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
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x16) as *const u8))
     }
  }
#[doc="Write the INTENCLR register."]
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x16) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENCLR register."]
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
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
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x17) as *const u8))
     }
  }
#[doc="Write the INTENSET register."]
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x17) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTENSET register."]
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
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
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u8))
     }
  }
#[doc="Write the INTFLAG register."]
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the INTFLAG register."]
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
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
        Offsetcorr(::core::ptr::read_volatile(((self.0 as usize) + 0x26) as *const u16))
     }
  }
#[doc="Write the OFFSETCORR register."]
  #[inline] pub fn set_offsetcorr(&self, value: Offsetcorr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x26) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the OFFSETCORR register."]
  #[inline] pub fn with_offsetcorr<F: FnOnce(Offsetcorr) -> Offsetcorr>(&self, f: F) -> &Self {
     let tmp = self.offsetcorr();
     self.set_offsetcorr(f(tmp))
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
        Refctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }
#[doc="Write the REFCTRL register."]
  #[inline] pub fn set_refctrl(&self, value: Refctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the REFCTRL register."]
  #[inline] pub fn with_refctrl<F: FnOnce(Refctrl) -> Refctrl>(&self, f: F) -> &Self {
     let tmp = self.refctrl();
     self.set_refctrl(f(tmp))
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
        Result(::core::ptr::read_volatile(((self.0 as usize) + 0x1a) as *const u16))
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
        Sampctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x3) as *const u8))
     }
  }
#[doc="Write the SAMPCTRL register."]
  #[inline] pub fn set_sampctrl(&self, value: Sampctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the SAMPCTRL register."]
  #[inline] pub fn with_sampctrl<F: FnOnce(Sampctrl) -> Sampctrl>(&self, f: F) -> &Self {
     let tmp = self.sampctrl();
     self.set_sampctrl(f(tmp))
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
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x19) as *const u8))
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
        Swtrig(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u8))
     }
  }
#[doc="Write the SWTRIG register."]
  #[inline] pub fn set_swtrig(&self, value: Swtrig) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the SWTRIG register."]
  #[inline] pub fn with_swtrig<F: FnOnce(Swtrig) -> Swtrig>(&self, f: F) -> &Self {
     let tmp = self.swtrig();
     self.set_swtrig(f(tmp))
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
        Winctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u8))
     }
  }
#[doc="Write the WINCTRL register."]
  #[inline] pub fn set_winctrl(&self, value: Winctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the WINCTRL register."]
  #[inline] pub fn with_winctrl<F: FnOnce(Winctrl) -> Winctrl>(&self, f: F) -> &Self {
     let tmp = self.winctrl();
     self.set_winctrl(f(tmp))
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
        Winlt(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u16))
     }
  }
#[doc="Write the WINLT register."]
  #[inline] pub fn set_winlt(&self, value: Winlt) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the WINLT register."]
  #[inline] pub fn with_winlt<F: FnOnce(Winlt) -> Winlt>(&self, f: F) -> &Self {
     let tmp = self.winlt();
     self.set_winlt(f(tmp))
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
        Winut(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u16))
     }
  }
#[doc="Write the WINUT register."]
  #[inline] pub fn set_winut(&self, value: Winut) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the WINUT register."]
  #[inline] pub fn with_winut<F: FnOnce(Winut) -> Winut>(&self, f: F) -> &Self {
     let tmp = self.winut();
     self.set_winut(f(tmp))
  }

}

#[doc="Average Control"]
#[derive(PartialEq, Eq)]
pub struct Avgctrl(pub u8);
impl Avgctrl {
#[doc="Number of Samples to be Collected"]
  #[inline] pub fn samplenum(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
#[doc="Number of Samples to be Collected"]
  #[inline] pub fn set_samplenum(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Adjusting Result / Division Coefficient"]
  #[inline] pub fn adjres(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x7 // [6:4]
  }
#[doc="Adjusting Result / Division Coefficient"]
  #[inline] pub fn set_adjres(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Calib(pub u16);
impl Calib {
#[doc="Linearity Calibration Value"]
  #[inline] pub fn linearity_cal(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xff // [7:0]
  }
#[doc="Linearity Calibration Value"]
  #[inline] pub fn set_linearity_cal(mut self, value: u16) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Bias Calibration Value"]
  #[inline] pub fn bias_cal(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
#[doc="Bias Calibration Value"]
  #[inline] pub fn set_bias_cal(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Ctrla(pub u8);
impl Ctrla {
#[doc="Software Reset"]
  #[inline] pub fn swrst(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Software Reset"]
  #[inline] pub fn set_swrst(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Enable"]
  #[inline] pub fn enable(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Enable"]
  #[inline] pub fn set_enable(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Run in Standby"]
  #[inline] pub fn runstdby(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Run in Standby"]
  #[inline] pub fn set_runstdby(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Ctrlb(pub u16);
impl Ctrlb {
#[doc="Differential Mode"]
  #[inline] pub fn diffmode(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Differential Mode"]
  #[inline] pub fn set_diffmode(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Left-Adjusted Result"]
  #[inline] pub fn leftadj(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Left-Adjusted Result"]
  #[inline] pub fn set_leftadj(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Free Running Mode"]
  #[inline] pub fn freerun(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="Free Running Mode"]
  #[inline] pub fn set_freerun(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Digital Correction Logic Enabled"]
  #[inline] pub fn corren(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
#[doc="Digital Correction Logic Enabled"]
  #[inline] pub fn set_corren(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Conversion Result Resolution"]
  #[inline] pub fn ressel(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x3 // [5:4]
  }
#[doc="Conversion Result Resolution"]
  #[inline] pub fn set_ressel(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Prescaler Configuration"]
  #[inline] pub fn prescaler(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
#[doc="Prescaler Configuration"]
  #[inline] pub fn set_prescaler(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
#[doc="Debug Run"]
  #[inline] pub fn dbgrun(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Debug Run"]
  #[inline] pub fn set_dbgrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Evctrl(pub u8);
impl Evctrl {
#[doc="Start Conversion Event In"]
  #[inline] pub fn startei(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Start Conversion Event In"]
  #[inline] pub fn set_startei(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Synchronization Event In"]
  #[inline] pub fn syncei(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Synchronization Event In"]
  #[inline] pub fn set_syncei(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Result Ready Event Out"]
  #[inline] pub fn resrdyeo(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="Result Ready Event Out"]
  #[inline] pub fn set_resrdyeo(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Window Monitor Event Out"]
  #[inline] pub fn winmoneo(&self) -> u8 {
     ((self.0 as u8) >> 5) & 0x1 // [5]
  }
#[doc="Window Monitor Event Out"]
  #[inline] pub fn set_winmoneo(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Gaincorr(pub u16);
impl Gaincorr {
#[doc="Gain Correction Value"]
  #[inline] pub fn gaincorr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xfff // [11:0]
  }
#[doc="Gain Correction Value"]
  #[inline] pub fn set_gaincorr(mut self, value: u16) -> Self {
     assert!((value & !0xfff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Inputctrl(pub u32);
impl Inputctrl {
#[doc="Positive Mux Input Selection"]
  #[inline] pub fn muxpos(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
#[doc="Positive Mux Input Selection"]
  #[inline] pub fn set_muxpos(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Negative Mux Input Selection"]
  #[inline] pub fn muxneg(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
#[doc="Negative Mux Input Selection"]
  #[inline] pub fn set_muxneg(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Number of Input Channels Included in Scan"]
  #[inline] pub fn inputscan(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="Number of Input Channels Included in Scan"]
  #[inline] pub fn set_inputscan(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Positive Mux Setting Offset"]
  #[inline] pub fn inputoffset(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
#[doc="Positive Mux Setting Offset"]
  #[inline] pub fn set_inputoffset(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Gain Factor Selection"]
  #[inline] pub fn gain(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
#[doc="Gain Factor Selection"]
  #[inline] pub fn set_gain(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
#[doc="Result Ready Interrupt Enable"]
  #[inline] pub fn resrdy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Result Ready Interrupt Enable"]
  #[inline] pub fn set_resrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Overrun Interrupt Enable"]
  #[inline] pub fn overrun(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Overrun Interrupt Enable"]
  #[inline] pub fn set_overrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Window Monitor Interrupt Enable"]
  #[inline] pub fn winmon(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Window Monitor Interrupt Enable"]
  #[inline] pub fn set_winmon(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
#[doc="Result Ready Interrupt Enable"]
  #[inline] pub fn resrdy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Result Ready Interrupt Enable"]
  #[inline] pub fn set_resrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Overrun Interrupt Enable"]
  #[inline] pub fn overrun(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Overrun Interrupt Enable"]
  #[inline] pub fn set_overrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Window Monitor Interrupt Enable"]
  #[inline] pub fn winmon(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Window Monitor Interrupt Enable"]
  #[inline] pub fn set_winmon(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready Interrupt Enable"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
#[doc="Result Ready"]
  #[inline] pub fn resrdy(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Result Ready"]
  #[inline] pub fn set_resrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Overrun"]
  #[inline] pub fn overrun(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Overrun"]
  #[inline] pub fn set_overrun(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Window Monitor"]
  #[inline] pub fn winmon(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Window Monitor"]
  #[inline] pub fn set_winmon(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Synchronization Ready"]
  #[inline] pub fn syncrdy(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="Synchronization Ready"]
  #[inline] pub fn set_syncrdy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Offsetcorr(pub u16);
impl Offsetcorr {
#[doc="Offset Correction Value"]
  #[inline] pub fn offsetcorr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xfff // [11:0]
  }
#[doc="Offset Correction Value"]
  #[inline] pub fn set_offsetcorr(mut self, value: u16) -> Self {
     assert!((value & !0xfff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Refctrl(pub u8);
impl Refctrl {
#[doc="Reference Selection"]
  #[inline] pub fn refsel(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
#[doc="Reference Selection"]
  #[inline] pub fn set_refsel(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Reference Buffer Offset Compensation Enable"]
  #[inline] pub fn refcomp(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Reference Buffer Offset Compensation Enable"]
  #[inline] pub fn set_refcomp(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Result(pub u16);
impl Result {
#[doc="Result Conversion Value"]
  #[inline] pub fn result(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Result Conversion Value"]
  #[inline] pub fn set_result(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
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
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Sampling Time Control"]
#[derive(PartialEq, Eq)]
pub struct Sampctrl(pub u8);
impl Sampctrl {
#[doc="Sampling Time Length"]
  #[inline] pub fn samplen(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x3f // [5:0]
  }
#[doc="Sampling Time Length"]
  #[inline] pub fn set_samplen(mut self, value: u8) -> Self {
     assert!((value & !0x3f) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
#[doc="Synchronization Busy"]
  #[inline] pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="Synchronization Busy"]
  #[inline] pub fn set_syncbusy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Swtrig(pub u8);
impl Swtrig {
#[doc="ADC Conversion Flush"]
  #[inline] pub fn flush(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="ADC Conversion Flush"]
  #[inline] pub fn set_flush(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="ADC Start Conversion"]
  #[inline] pub fn start(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="ADC Start Conversion"]
  #[inline] pub fn set_start(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Winctrl(pub u8);
impl Winctrl {
#[doc="Window Monitor Mode"]
  #[inline] pub fn winmode(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x7 // [2:0]
  }
#[doc="Window Monitor Mode"]
  #[inline] pub fn set_winmode(mut self, value: u8) -> Self {
     assert!((value & !0x7) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Winlt(pub u16);
impl Winlt {
#[doc="Window Lower Threshold"]
  #[inline] pub fn winlt(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Window Lower Threshold"]
  #[inline] pub fn set_winlt(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
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
#[derive(PartialEq, Eq)]
pub struct Winut(pub u16);
impl Winut {
#[doc="Window Upper Threshold"]
  #[inline] pub fn winut(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Window Upper Threshold"]
  #[inline] pub fn set_winut(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
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
