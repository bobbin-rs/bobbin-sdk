//! System Integration Module
pub const SIM: Sim = Sim(0x40048000);

#[doc="System Integration Module"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sim(pub u32);
impl Sim {
#[doc="Get the *const pointer for the CHIPCTL register."]
  #[inline] pub fn chipctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CHIPCTL register."]
  #[inline] pub fn chipctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CHIPCTL register."]
  #[inline] pub fn chipctl(&self) -> Chipctl { 
     unsafe {
        Chipctl(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CHIPCTL register."]
  #[inline] pub fn set_chipctl(&self, value: Chipctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CHIPCTL register."]
  #[inline] pub fn with_chipctl<F: FnOnce(Chipctl) -> Chipctl>(&self, f: F) -> &Self {
     let tmp = self.chipctl();
     self.set_chipctl(f(tmp))
  }

#[doc="Get the *const pointer for the FTMOPT0 register."]
  #[inline] pub fn ftmopt0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the FTMOPT0 register."]
  #[inline] pub fn ftmopt0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the FTMOPT0 register."]
  #[inline] pub fn ftmopt0(&self) -> Ftmopt0 { 
     unsafe {
        Ftmopt0(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the FTMOPT0 register."]
  #[inline] pub fn set_ftmopt0(&self, value: Ftmopt0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FTMOPT0 register."]
  #[inline] pub fn with_ftmopt0<F: FnOnce(Ftmopt0) -> Ftmopt0>(&self, f: F) -> &Self {
     let tmp = self.ftmopt0();
     self.set_ftmopt0(f(tmp))
  }

#[doc="Get the *const pointer for the LPOCLKS register."]
  #[inline] pub fn lpoclks_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the LPOCLKS register."]
  #[inline] pub fn lpoclks_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the LPOCLKS register."]
  #[inline] pub fn lpoclks(&self) -> Lpoclks { 
     unsafe {
        Lpoclks(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the LPOCLKS register."]
  #[inline] pub fn set_lpoclks(&self, value: Lpoclks) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LPOCLKS register."]
  #[inline] pub fn with_lpoclks<F: FnOnce(Lpoclks) -> Lpoclks>(&self, f: F) -> &Self {
     let tmp = self.lpoclks();
     self.set_lpoclks(f(tmp))
  }

#[doc="Get the *const pointer for the ADCOPT register."]
  #[inline] pub fn adcopt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the ADCOPT register."]
  #[inline] pub fn adcopt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the ADCOPT register."]
  #[inline] pub fn adcopt(&self) -> Adcopt { 
     unsafe {
        Adcopt(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the ADCOPT register."]
  #[inline] pub fn set_adcopt(&self, value: Adcopt) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ADCOPT register."]
  #[inline] pub fn with_adcopt<F: FnOnce(Adcopt) -> Adcopt>(&self, f: F) -> &Self {
     let tmp = self.adcopt();
     self.set_adcopt(f(tmp))
  }

#[doc="Get the *const pointer for the FTMOPT1 register."]
  #[inline] pub fn ftmopt1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the FTMOPT1 register."]
  #[inline] pub fn ftmopt1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the FTMOPT1 register."]
  #[inline] pub fn ftmopt1(&self) -> Ftmopt1 { 
     unsafe {
        Ftmopt1(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the FTMOPT1 register."]
  #[inline] pub fn set_ftmopt1(&self, value: Ftmopt1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FTMOPT1 register."]
  #[inline] pub fn with_ftmopt1<F: FnOnce(Ftmopt1) -> Ftmopt1>(&self, f: F) -> &Self {
     let tmp = self.ftmopt1();
     self.set_ftmopt1(f(tmp))
  }

#[doc="Get the *const pointer for the MISCTRL0 register."]
  #[inline] pub fn misctrl0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the MISCTRL0 register."]
  #[inline] pub fn misctrl0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the MISCTRL0 register."]
  #[inline] pub fn misctrl0(&self) -> Misctrl0 { 
     unsafe {
        Misctrl0(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the MISCTRL0 register."]
  #[inline] pub fn set_misctrl0(&self, value: Misctrl0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MISCTRL0 register."]
  #[inline] pub fn with_misctrl0<F: FnOnce(Misctrl0) -> Misctrl0>(&self, f: F) -> &Self {
     let tmp = self.misctrl0();
     self.set_misctrl0(f(tmp))
  }

#[doc="Get the *const pointer for the SDID register."]
  #[inline] pub fn sdid_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the SDID register."]
  #[inline] pub fn sdid_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the SDID register."]
  #[inline] pub fn sdid(&self) -> Sdid { 
     unsafe {
        Sdid(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }

#[doc="Get the *const pointer for the PLATCGC register."]
  #[inline] pub fn platcgc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
#[doc="Get the *mut pointer for the PLATCGC register."]
  #[inline] pub fn platcgc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
#[doc="Read the PLATCGC register."]
  #[inline] pub fn platcgc(&self) -> Platcgc { 
     unsafe {
        Platcgc(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
#[doc="Write the PLATCGC register."]
  #[inline] pub fn set_platcgc(&self, value: Platcgc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PLATCGC register."]
  #[inline] pub fn with_platcgc<F: FnOnce(Platcgc) -> Platcgc>(&self, f: F) -> &Self {
     let tmp = self.platcgc();
     self.set_platcgc(f(tmp))
  }

#[doc="Get the *const pointer for the FCFG1 register."]
  #[inline] pub fn fcfg1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
#[doc="Get the *mut pointer for the FCFG1 register."]
  #[inline] pub fn fcfg1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
#[doc="Read the FCFG1 register."]
  #[inline] pub fn fcfg1(&self) -> Fcfg1 { 
     unsafe {
        Fcfg1(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
#[doc="Write the FCFG1 register."]
  #[inline] pub fn set_fcfg1(&self, value: Fcfg1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FCFG1 register."]
  #[inline] pub fn with_fcfg1<F: FnOnce(Fcfg1) -> Fcfg1>(&self, f: F) -> &Self {
     let tmp = self.fcfg1();
     self.set_fcfg1(f(tmp))
  }

#[doc="Get the *const pointer for the UIDH register."]
  #[inline] pub fn uidh_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x54) as *const u32
  }
#[doc="Get the *mut pointer for the UIDH register."]
  #[inline] pub fn uidh_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x54) as *mut u32
  }
#[doc="Read the UIDH register."]
  #[inline] pub fn uidh(&self) -> Uidh { 
     unsafe {
        Uidh(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }

#[doc="Get the *const pointer for the UIDMH register."]
  #[inline] pub fn uidmh_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
#[doc="Get the *mut pointer for the UIDMH register."]
  #[inline] pub fn uidmh_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
#[doc="Read the UIDMH register."]
  #[inline] pub fn uidmh(&self) -> Uidmh { 
     unsafe {
        Uidmh(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }

#[doc="Get the *const pointer for the UIDML register."]
  #[inline] pub fn uidml_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x5c) as *const u32
  }
#[doc="Get the *mut pointer for the UIDML register."]
  #[inline] pub fn uidml_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x5c) as *mut u32
  }
#[doc="Read the UIDML register."]
  #[inline] pub fn uidml(&self) -> Uidml { 
     unsafe {
        Uidml(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
     }
  }

#[doc="Get the *const pointer for the UIDL register."]
  #[inline] pub fn uidl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x60) as *const u32
  }
#[doc="Get the *mut pointer for the UIDL register."]
  #[inline] pub fn uidl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x60) as *mut u32
  }
#[doc="Read the UIDL register."]
  #[inline] pub fn uidl(&self) -> Uidl { 
     unsafe {
        Uidl(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
     }
  }

#[doc="Get the *const pointer for the CLKDIV4 register."]
  #[inline] pub fn clkdiv4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x68) as *const u32
  }
#[doc="Get the *mut pointer for the CLKDIV4 register."]
  #[inline] pub fn clkdiv4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x68) as *mut u32
  }
#[doc="Read the CLKDIV4 register."]
  #[inline] pub fn clkdiv4(&self) -> Clkdiv4 { 
     unsafe {
        Clkdiv4(::core::ptr::read_volatile(((self.0 as usize) + 0x68) as *const u32))
     }
  }
#[doc="Write the CLKDIV4 register."]
  #[inline] pub fn set_clkdiv4(&self, value: Clkdiv4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x68) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CLKDIV4 register."]
  #[inline] pub fn with_clkdiv4<F: FnOnce(Clkdiv4) -> Clkdiv4>(&self, f: F) -> &Self {
     let tmp = self.clkdiv4();
     self.set_clkdiv4(f(tmp))
  }

#[doc="Get the *const pointer for the MISCTRL1 register."]
  #[inline] pub fn misctrl1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x6c) as *const u32
  }
#[doc="Get the *mut pointer for the MISCTRL1 register."]
  #[inline] pub fn misctrl1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x6c) as *mut u32
  }
#[doc="Read the MISCTRL1 register."]
  #[inline] pub fn misctrl1(&self) -> Misctrl1 { 
     unsafe {
        Misctrl1(::core::ptr::read_volatile(((self.0 as usize) + 0x6c) as *const u32))
     }
  }
#[doc="Write the MISCTRL1 register."]
  #[inline] pub fn set_misctrl1(&self, value: Misctrl1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MISCTRL1 register."]
  #[inline] pub fn with_misctrl1<F: FnOnce(Misctrl1) -> Misctrl1>(&self, f: F) -> &Self {
     let tmp = self.misctrl1();
     self.set_misctrl1(f(tmp))
  }

}

#[doc="Chip Control register"]
#[derive(PartialEq, Eq)]
pub struct Chipctl(pub u32);
impl Chipctl {
#[doc="ADC interleave channel enable"]
  #[inline] pub fn adc_interleave_en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="ADC interleave channel enable"]
  #[inline] pub fn set_adc_interleave_en(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="CLKOUT Select"]
  #[inline] pub fn clkoutsel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
#[doc="CLKOUT Select"]
  #[inline] pub fn set_clkoutsel(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="CLKOUT Divide Ratio"]
  #[inline] pub fn clkoutdiv(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
#[doc="CLKOUT Divide Ratio"]
  #[inline] pub fn set_clkoutdiv(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="CLKOUT enable"]
  #[inline] pub fn clkouten(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="CLKOUT enable"]
  #[inline] pub fn set_clkouten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Debug trace clock select"]
  #[inline] pub fn traceclk_sel(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Debug trace clock select"]
  #[inline] pub fn set_traceclk_sel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="PDB back-to-back select"]
  #[inline] pub fn pdb_bb_sel(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="PDB back-to-back select"]
  #[inline] pub fn set_pdb_bb_sel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="ADC_SUPPLY"]
  #[inline] pub fn adc_supply(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7 // [18:16]
  }
#[doc="ADC_SUPPLY"]
  #[inline] pub fn set_adc_supply(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="ADC_SUPPLYEN"]
  #[inline] pub fn adc_supplyen(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="ADC_SUPPLYEN"]
  #[inline] pub fn set_adc_supplyen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="SRAMU_RETEN"]
  #[inline] pub fn sramu_reten(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="SRAMU_RETEN"]
  #[inline] pub fn set_sramu_reten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="SRAML_RETEN"]
  #[inline] pub fn sraml_reten(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="SRAML_RETEN"]
  #[inline] pub fn set_sraml_reten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

}
impl ::core::fmt::Display for Chipctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chipctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adc_interleave_en() != 0 { try!(write!(f, " adc_interleave_en=0x{:x}", self.adc_interleave_en()))}
      if self.clkoutsel() != 0 { try!(write!(f, " clkoutsel=0x{:x}", self.clkoutsel()))}
      if self.clkoutdiv() != 0 { try!(write!(f, " clkoutdiv=0x{:x}", self.clkoutdiv()))}
      if self.clkouten() != 0 { try!(write!(f, " clkouten"))}
      if self.traceclk_sel() != 0 { try!(write!(f, " traceclk_sel"))}
      if self.pdb_bb_sel() != 0 { try!(write!(f, " pdb_bb_sel"))}
      if self.adc_supply() != 0 { try!(write!(f, " adc_supply=0x{:x}", self.adc_supply()))}
      if self.adc_supplyen() != 0 { try!(write!(f, " adc_supplyen"))}
      if self.sramu_reten() != 0 { try!(write!(f, " sramu_reten"))}
      if self.sraml_reten() != 0 { try!(write!(f, " sraml_reten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FTM Option Register 0"]
#[derive(PartialEq, Eq)]
pub struct Ftmopt0(pub u32);
impl Ftmopt0 {
#[doc="FTM0 Fault X Select"]
  #[inline] pub fn ftm0fltxsel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
#[doc="FTM0 Fault X Select"]
  #[inline] pub fn set_ftm0fltxsel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="FTM1 Fault X Select"]
  #[inline] pub fn ftm1fltxsel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
#[doc="FTM1 Fault X Select"]
  #[inline] pub fn set_ftm1fltxsel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="FTM2 Fault X Select"]
  #[inline] pub fn ftm2fltxsel(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
#[doc="FTM2 Fault X Select"]
  #[inline] pub fn set_ftm2fltxsel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="FTM3 Fault X Select"]
  #[inline] pub fn ftm3fltxsel(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
#[doc="FTM3 Fault X Select"]
  #[inline] pub fn set_ftm3fltxsel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="FTM0 External Clock Pin Select"]
  #[inline] pub fn ftm0clksel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3 // [25:24]
  }
#[doc="FTM0 External Clock Pin Select"]
  #[inline] pub fn set_ftm0clksel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="FTM1 External Clock Pin Select"]
  #[inline] pub fn ftm1clksel(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x3 // [27:26]
  }
#[doc="FTM1 External Clock Pin Select"]
  #[inline] pub fn set_ftm1clksel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="FTM2 External Clock Pin Select"]
  #[inline] pub fn ftm2clksel(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
#[doc="FTM2 External Clock Pin Select"]
  #[inline] pub fn set_ftm2clksel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="FTM3 External Clock Pin Select"]
  #[inline] pub fn ftm3clksel(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x3 // [31:30]
  }
#[doc="FTM3 External Clock Pin Select"]
  #[inline] pub fn set_ftm3clksel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 30);
     self.0 |= value << 30;
     self
  }

}
impl ::core::fmt::Display for Ftmopt0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ftmopt0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ftm0fltxsel() != 0 { try!(write!(f, " ftm0fltxsel=0x{:x}", self.ftm0fltxsel()))}
      if self.ftm1fltxsel() != 0 { try!(write!(f, " ftm1fltxsel=0x{:x}", self.ftm1fltxsel()))}
      if self.ftm2fltxsel() != 0 { try!(write!(f, " ftm2fltxsel=0x{:x}", self.ftm2fltxsel()))}
      if self.ftm3fltxsel() != 0 { try!(write!(f, " ftm3fltxsel=0x{:x}", self.ftm3fltxsel()))}
      if self.ftm0clksel() != 0 { try!(write!(f, " ftm0clksel=0x{:x}", self.ftm0clksel()))}
      if self.ftm1clksel() != 0 { try!(write!(f, " ftm1clksel=0x{:x}", self.ftm1clksel()))}
      if self.ftm2clksel() != 0 { try!(write!(f, " ftm2clksel=0x{:x}", self.ftm2clksel()))}
      if self.ftm3clksel() != 0 { try!(write!(f, " ftm3clksel=0x{:x}", self.ftm3clksel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LPO Clock Select Register"]
#[derive(PartialEq, Eq)]
pub struct Lpoclks(pub u32);
impl Lpoclks {
#[doc="1 kHz LPO_CLK enable"]
  #[inline] pub fn lpo1kclken(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="1 kHz LPO_CLK enable"]
  #[inline] pub fn set_lpo1kclken(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="32 kHz LPO_CLK enable"]
  #[inline] pub fn lpo32kclken(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="32 kHz LPO_CLK enable"]
  #[inline] pub fn set_lpo32kclken(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="LPO clock source select"]
  #[inline] pub fn lpoclksel(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
#[doc="LPO clock source select"]
  #[inline] pub fn set_lpoclksel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="32 kHz clock source select"]
  #[inline] pub fn rtcclksel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="32 kHz clock source select"]
  #[inline] pub fn set_rtcclksel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

}
impl ::core::fmt::Display for Lpoclks {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lpoclks {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lpo1kclken() != 0 { try!(write!(f, " lpo1kclken"))}
      if self.lpo32kclken() != 0 { try!(write!(f, " lpo32kclken"))}
      if self.lpoclksel() != 0 { try!(write!(f, " lpoclksel=0x{:x}", self.lpoclksel()))}
      if self.rtcclksel() != 0 { try!(write!(f, " rtcclksel=0x{:x}", self.rtcclksel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="ADC Options Register"]
#[derive(PartialEq, Eq)]
pub struct Adcopt(pub u32);
impl Adcopt {
#[doc="ADC0 trigger source select"]
  #[inline] pub fn adc0trgsel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="ADC0 trigger source select"]
  #[inline] pub fn set_adc0trgsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="ADC0 software pretrigger sources"]
  #[inline] pub fn adc0swpretrg(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x7 // [3:1]
  }
#[doc="ADC0 software pretrigger sources"]
  #[inline] pub fn set_adc0swpretrg(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="ADC0 pretrigger source select"]
  #[inline] pub fn adc0pretrgsel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="ADC0 pretrigger source select"]
  #[inline] pub fn set_adc0pretrgsel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="ADC1 trigger source select"]
  #[inline] pub fn adc1trgsel(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="ADC1 trigger source select"]
  #[inline] pub fn set_adc1trgsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="ADC1 software pretrigger sources"]
  #[inline] pub fn adc1swpretrg(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x7 // [11:9]
  }
#[doc="ADC1 software pretrigger sources"]
  #[inline] pub fn set_adc1swpretrg(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="ADC1 pretrigger source select"]
  #[inline] pub fn adc1pretrgsel(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
#[doc="ADC1 pretrigger source select"]
  #[inline] pub fn set_adc1pretrgsel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

}
impl ::core::fmt::Display for Adcopt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Adcopt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adc0trgsel() != 0 { try!(write!(f, " adc0trgsel"))}
      if self.adc0swpretrg() != 0 { try!(write!(f, " adc0swpretrg=0x{:x}", self.adc0swpretrg()))}
      if self.adc0pretrgsel() != 0 { try!(write!(f, " adc0pretrgsel=0x{:x}", self.adc0pretrgsel()))}
      if self.adc1trgsel() != 0 { try!(write!(f, " adc1trgsel"))}
      if self.adc1swpretrg() != 0 { try!(write!(f, " adc1swpretrg=0x{:x}", self.adc1swpretrg()))}
      if self.adc1pretrgsel() != 0 { try!(write!(f, " adc1pretrgsel=0x{:x}", self.adc1pretrgsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="FTM Option Register 1"]
#[derive(PartialEq, Eq)]
pub struct Ftmopt1(pub u32);
impl Ftmopt1 {
#[doc="FTM0 Sync Bit"]
  #[inline] pub fn ftm0syncbit(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="FTM0 Sync Bit"]
  #[inline] pub fn set_ftm0syncbit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="FTM1 Sync Bit"]
  #[inline] pub fn ftm1syncbit(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="FTM1 Sync Bit"]
  #[inline] pub fn set_ftm1syncbit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="FTM2 Sync Bit"]
  #[inline] pub fn ftm2syncbit(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="FTM2 Sync Bit"]
  #[inline] pub fn set_ftm2syncbit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="FTM3 Sync Bit"]
  #[inline] pub fn ftm3syncbit(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="FTM3 Sync Bit"]
  #[inline] pub fn set_ftm3syncbit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="FTM1 CH0 Select"]
  #[inline] pub fn ftm1ch0sel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="FTM1 CH0 Select"]
  #[inline] pub fn set_ftm1ch0sel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="FTM2 CH0 Select"]
  #[inline] pub fn ftm2ch0sel(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
#[doc="FTM2 CH0 Select"]
  #[inline] pub fn set_ftm2ch0sel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="FTM2 CH1 Select"]
  #[inline] pub fn ftm2ch1sel(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="FTM2 CH1 Select"]
  #[inline] pub fn set_ftm2ch1sel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="FTM global load enable"]
  #[inline] pub fn ftmgldok(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="FTM global load enable"]
  #[inline] pub fn set_ftmgldok(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="FTM0 channel modulation select with FTM1_CH1"]
  #[inline] pub fn ftm0_outsel(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
#[doc="FTM0 channel modulation select with FTM1_CH1"]
  #[inline] pub fn set_ftm0_outsel(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="FTM3 channel modulation select with FTM2_CH1"]
  #[inline] pub fn ftm3_outsel(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
#[doc="FTM3 channel modulation select with FTM2_CH1"]
  #[inline] pub fn set_ftm3_outsel(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

}
impl ::core::fmt::Display for Ftmopt1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ftmopt1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ftm0syncbit() != 0 { try!(write!(f, " ftm0syncbit"))}
      if self.ftm1syncbit() != 0 { try!(write!(f, " ftm1syncbit"))}
      if self.ftm2syncbit() != 0 { try!(write!(f, " ftm2syncbit"))}
      if self.ftm3syncbit() != 0 { try!(write!(f, " ftm3syncbit"))}
      if self.ftm1ch0sel() != 0 { try!(write!(f, " ftm1ch0sel=0x{:x}", self.ftm1ch0sel()))}
      if self.ftm2ch0sel() != 0 { try!(write!(f, " ftm2ch0sel=0x{:x}", self.ftm2ch0sel()))}
      if self.ftm2ch1sel() != 0 { try!(write!(f, " ftm2ch1sel"))}
      if self.ftmgldok() != 0 { try!(write!(f, " ftmgldok"))}
      if self.ftm0_outsel() != 0 { try!(write!(f, " ftm0_outsel=0x{:x}", self.ftm0_outsel()))}
      if self.ftm3_outsel() != 0 { try!(write!(f, " ftm3_outsel=0x{:x}", self.ftm3_outsel()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Miscellaneous control register 0"]
#[derive(PartialEq, Eq)]
pub struct Misctrl0(pub u32);
impl Misctrl0 {
#[doc="FTM0 OBE CTRL bit"]
  #[inline] pub fn ftm0_obe_ctrl(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="FTM0 OBE CTRL bit"]
  #[inline] pub fn set_ftm0_obe_ctrl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="FTM1 OBE CTRL bit"]
  #[inline] pub fn ftm1_obe_ctrl(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="FTM1 OBE CTRL bit"]
  #[inline] pub fn set_ftm1_obe_ctrl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="FTM2 OBE CTRL bit"]
  #[inline] pub fn ftm2_obe_ctrl(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="FTM2 OBE CTRL bit"]
  #[inline] pub fn set_ftm2_obe_ctrl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="FTM3 OBE CTRL bit"]
  #[inline] pub fn ftm3_obe_ctrl(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="FTM3 OBE CTRL bit"]
  #[inline] pub fn set_ftm3_obe_ctrl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

}
impl ::core::fmt::Display for Misctrl0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Misctrl0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ftm0_obe_ctrl() != 0 { try!(write!(f, " ftm0_obe_ctrl"))}
      if self.ftm1_obe_ctrl() != 0 { try!(write!(f, " ftm1_obe_ctrl"))}
      if self.ftm2_obe_ctrl() != 0 { try!(write!(f, " ftm2_obe_ctrl"))}
      if self.ftm3_obe_ctrl() != 0 { try!(write!(f, " ftm3_obe_ctrl"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Device Identification Register"]
#[derive(PartialEq, Eq)]
pub struct Sdid(pub u32);
impl Sdid {
#[doc="Features"]
  #[inline] pub fn features(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Features"]
  #[inline] pub fn set_features(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Package"]
  #[inline] pub fn package(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
#[doc="Package"]
  #[inline] pub fn set_package(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Device revision number"]
  #[inline] pub fn revid(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
#[doc="Device revision number"]
  #[inline] pub fn set_revid(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

#[doc="RAM size"]
  #[inline] pub fn ramsize(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="RAM size"]
  #[inline] pub fn set_ramsize(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Derivate"]
  #[inline] pub fn derivate(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
#[doc="Derivate"]
  #[inline] pub fn set_derivate(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Subseries"]
  #[inline] pub fn subseries(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
#[doc="Subseries"]
  #[inline] pub fn set_subseries(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="S32K product series generation"]
  #[inline] pub fn generation(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0xf // [31:28]
  }
#[doc="S32K product series generation"]
  #[inline] pub fn set_generation(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Sdid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sdid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.features() != 0 { try!(write!(f, " features=0x{:x}", self.features()))}
      if self.package() != 0 { try!(write!(f, " package=0x{:x}", self.package()))}
      if self.revid() != 0 { try!(write!(f, " revid=0x{:x}", self.revid()))}
      if self.ramsize() != 0 { try!(write!(f, " ramsize=0x{:x}", self.ramsize()))}
      if self.derivate() != 0 { try!(write!(f, " derivate=0x{:x}", self.derivate()))}
      if self.subseries() != 0 { try!(write!(f, " subseries=0x{:x}", self.subseries()))}
      if self.generation() != 0 { try!(write!(f, " generation=0x{:x}", self.generation()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Platform Clock Gating Control Register"]
#[derive(PartialEq, Eq)]
pub struct Platcgc(pub u32);
impl Platcgc {
#[doc="MSCM Clock Gating Control"]
  #[inline] pub fn cgcmscm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="MSCM Clock Gating Control"]
  #[inline] pub fn set_cgcmscm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="MPU Clock Gating Control"]
  #[inline] pub fn cgcmpu(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="MPU Clock Gating Control"]
  #[inline] pub fn set_cgcmpu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="DMA Clock Gating Control"]
  #[inline] pub fn cgcdma(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="DMA Clock Gating Control"]
  #[inline] pub fn set_cgcdma(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="ERM Clock Gating Control"]
  #[inline] pub fn cgcerm(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="ERM Clock Gating Control"]
  #[inline] pub fn set_cgcerm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="EIM Clock Gating Control"]
  #[inline] pub fn cgceim(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="EIM Clock Gating Control"]
  #[inline] pub fn set_cgceim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}
impl ::core::fmt::Display for Platcgc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Platcgc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgcmscm() != 0 { try!(write!(f, " cgcmscm"))}
      if self.cgcmpu() != 0 { try!(write!(f, " cgcmpu"))}
      if self.cgcdma() != 0 { try!(write!(f, " cgcdma"))}
      if self.cgcerm() != 0 { try!(write!(f, " cgcerm"))}
      if self.cgceim() != 0 { try!(write!(f, " cgceim"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash Configuration Register 1"]
#[derive(PartialEq, Eq)]
pub struct Fcfg1(pub u32);
impl Fcfg1 {
#[doc="FlexNVM partition"]
  #[inline] pub fn depart(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
#[doc="FlexNVM partition"]
  #[inline] pub fn set_depart(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

#[doc="EEE SRAM SIZE"]
  #[inline] pub fn eeeramsize(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="EEE SRAM SIZE"]
  #[inline] pub fn set_eeeramsize(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Fcfg1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fcfg1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.depart() != 0 { try!(write!(f, " depart=0x{:x}", self.depart()))}
      if self.eeeramsize() != 0 { try!(write!(f, " eeeramsize=0x{:x}", self.eeeramsize()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Unique Identification Register High"]
#[derive(PartialEq, Eq)]
pub struct Uidh(pub u32);
impl Uidh {
#[doc="Unique Identification"]
  #[inline] pub fn uid127_96(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Unique Identification"]
  #[inline] pub fn set_uid127_96(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Uidh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Uidh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Unique Identification Register Mid-High"]
#[derive(PartialEq, Eq)]
pub struct Uidmh(pub u32);
impl Uidmh {
#[doc="Unique Identification"]
  #[inline] pub fn uid95_64(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Unique Identification"]
  #[inline] pub fn set_uid95_64(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Uidmh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Uidmh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Unique Identification Register Mid Low"]
#[derive(PartialEq, Eq)]
pub struct Uidml(pub u32);
impl Uidml {
#[doc="Unique Identification"]
  #[inline] pub fn uid63_32(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Unique Identification"]
  #[inline] pub fn set_uid63_32(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Uidml {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Uidml {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Unique Identification Register Low"]
#[derive(PartialEq, Eq)]
pub struct Uidl(pub u32);
impl Uidl {
#[doc="Unique Identification"]
  #[inline] pub fn uid31_0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Unique Identification"]
  #[inline] pub fn set_uid31_0(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Uidl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Uidl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Clock Divider Register 4"]
#[derive(PartialEq, Eq)]
pub struct Clkdiv4(pub u32);
impl Clkdiv4 {
#[doc="Trace Clock Divider fraction To configure TRACEDIV and TRACEFRAC, you must first clear TRACEDIVEN to disable the trace clock divide function."]
  #[inline] pub fn tracefrac(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Trace Clock Divider fraction To configure TRACEDIV and TRACEFRAC, you must first clear TRACEDIVEN to disable the trace clock divide function."]
  #[inline] pub fn set_tracefrac(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Trace Clock Divider value To configure TRACEDIV, you must first disable TRACEDIVEN, then enable it after setting TRACEDIV."]
  #[inline] pub fn tracediv(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x7 // [3:1]
  }
#[doc="Trace Clock Divider value To configure TRACEDIV, you must first disable TRACEDIVEN, then enable it after setting TRACEDIV."]
  #[inline] pub fn set_tracediv(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Debug Trace Divider control"]
  #[inline] pub fn tracediven(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="Debug Trace Divider control"]
  #[inline] pub fn set_tracediven(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

}
impl ::core::fmt::Display for Clkdiv4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clkdiv4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tracefrac() != 0 { try!(write!(f, " tracefrac"))}
      if self.tracediv() != 0 { try!(write!(f, " tracediv=0x{:x}", self.tracediv()))}
      if self.tracediven() != 0 { try!(write!(f, " tracediven"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Miscellaneous Control register 1"]
#[derive(PartialEq, Eq)]
pub struct Misctrl1(pub u32);
impl Misctrl1 {
#[doc="Software trigger to TRGMUX. Writing to this bit generates software trigger to peripherals through TRGMUX (Refer to Figure: Trigger interconnectivity)."]
  #[inline] pub fn sw_trg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Software trigger to TRGMUX. Writing to this bit generates software trigger to peripherals through TRGMUX (Refer to Figure: Trigger interconnectivity)."]
  #[inline] pub fn set_sw_trg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Misctrl1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Misctrl1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sw_trg() != 0 { try!(write!(f, " sw_trg"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
