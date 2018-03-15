//! System Integration Module

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="System Integration Module"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SimPeriph(pub usize);
impl SimPeriph {
    #[doc="Get the *mut pointer for the CHIPCTL register."]
    #[inline] pub fn chipctl_mut(&self) -> *mut Chipctl { 
        (self.0 + 0x4) as *mut Chipctl
    }

    #[doc="Get the *const pointer for the CHIPCTL register."]
    #[inline] pub fn chipctl_ptr(&self) -> *const Chipctl { 
           self.chipctl_mut()
    }

    #[doc="Read the CHIPCTL register."]
    #[inline] pub fn chipctl(&self) -> Chipctl { 
        unsafe {
            read_volatile(self.chipctl_ptr())
        }
    }

    #[doc="Write the CHIPCTL register."]
    #[inline] pub fn set_chipctl<F: FnOnce(Chipctl) -> Chipctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chipctl_mut(), f(Chipctl(0)));
        }
        self
    }

    #[doc="Modify the CHIPCTL register."]
    #[inline] pub fn with_chipctl<F: FnOnce(Chipctl) -> Chipctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chipctl_mut(), f(self.chipctl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FTMOPT0 register."]
    #[inline] pub fn ftmopt0_mut(&self) -> *mut Ftmopt0 { 
        (self.0 + 0xc) as *mut Ftmopt0
    }

    #[doc="Get the *const pointer for the FTMOPT0 register."]
    #[inline] pub fn ftmopt0_ptr(&self) -> *const Ftmopt0 { 
           self.ftmopt0_mut()
    }

    #[doc="Read the FTMOPT0 register."]
    #[inline] pub fn ftmopt0(&self) -> Ftmopt0 { 
        unsafe {
            read_volatile(self.ftmopt0_ptr())
        }
    }

    #[doc="Write the FTMOPT0 register."]
    #[inline] pub fn set_ftmopt0<F: FnOnce(Ftmopt0) -> Ftmopt0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftmopt0_mut(), f(Ftmopt0(0)));
        }
        self
    }

    #[doc="Modify the FTMOPT0 register."]
    #[inline] pub fn with_ftmopt0<F: FnOnce(Ftmopt0) -> Ftmopt0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftmopt0_mut(), f(self.ftmopt0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LPOCLKS register."]
    #[inline] pub fn lpoclks_mut(&self) -> *mut Lpoclks { 
        (self.0 + 0x10) as *mut Lpoclks
    }

    #[doc="Get the *const pointer for the LPOCLKS register."]
    #[inline] pub fn lpoclks_ptr(&self) -> *const Lpoclks { 
           self.lpoclks_mut()
    }

    #[doc="Read the LPOCLKS register."]
    #[inline] pub fn lpoclks(&self) -> Lpoclks { 
        unsafe {
            read_volatile(self.lpoclks_ptr())
        }
    }

    #[doc="Write the LPOCLKS register."]
    #[inline] pub fn set_lpoclks<F: FnOnce(Lpoclks) -> Lpoclks>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpoclks_mut(), f(Lpoclks(0)));
        }
        self
    }

    #[doc="Modify the LPOCLKS register."]
    #[inline] pub fn with_lpoclks<F: FnOnce(Lpoclks) -> Lpoclks>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lpoclks_mut(), f(self.lpoclks()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADCOPT register."]
    #[inline] pub fn adcopt_mut(&self) -> *mut Adcopt { 
        (self.0 + 0x18) as *mut Adcopt
    }

    #[doc="Get the *const pointer for the ADCOPT register."]
    #[inline] pub fn adcopt_ptr(&self) -> *const Adcopt { 
           self.adcopt_mut()
    }

    #[doc="Read the ADCOPT register."]
    #[inline] pub fn adcopt(&self) -> Adcopt { 
        unsafe {
            read_volatile(self.adcopt_ptr())
        }
    }

    #[doc="Write the ADCOPT register."]
    #[inline] pub fn set_adcopt<F: FnOnce(Adcopt) -> Adcopt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcopt_mut(), f(Adcopt(0)));
        }
        self
    }

    #[doc="Modify the ADCOPT register."]
    #[inline] pub fn with_adcopt<F: FnOnce(Adcopt) -> Adcopt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcopt_mut(), f(self.adcopt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FTMOPT1 register."]
    #[inline] pub fn ftmopt1_mut(&self) -> *mut Ftmopt1 { 
        (self.0 + 0x1c) as *mut Ftmopt1
    }

    #[doc="Get the *const pointer for the FTMOPT1 register."]
    #[inline] pub fn ftmopt1_ptr(&self) -> *const Ftmopt1 { 
           self.ftmopt1_mut()
    }

    #[doc="Read the FTMOPT1 register."]
    #[inline] pub fn ftmopt1(&self) -> Ftmopt1 { 
        unsafe {
            read_volatile(self.ftmopt1_ptr())
        }
    }

    #[doc="Write the FTMOPT1 register."]
    #[inline] pub fn set_ftmopt1<F: FnOnce(Ftmopt1) -> Ftmopt1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftmopt1_mut(), f(Ftmopt1(0)));
        }
        self
    }

    #[doc="Modify the FTMOPT1 register."]
    #[inline] pub fn with_ftmopt1<F: FnOnce(Ftmopt1) -> Ftmopt1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftmopt1_mut(), f(self.ftmopt1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MISCTRL0 register."]
    #[inline] pub fn misctrl0_mut(&self) -> *mut Misctrl0 { 
        (self.0 + 0x20) as *mut Misctrl0
    }

    #[doc="Get the *const pointer for the MISCTRL0 register."]
    #[inline] pub fn misctrl0_ptr(&self) -> *const Misctrl0 { 
           self.misctrl0_mut()
    }

    #[doc="Read the MISCTRL0 register."]
    #[inline] pub fn misctrl0(&self) -> Misctrl0 { 
        unsafe {
            read_volatile(self.misctrl0_ptr())
        }
    }

    #[doc="Write the MISCTRL0 register."]
    #[inline] pub fn set_misctrl0<F: FnOnce(Misctrl0) -> Misctrl0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.misctrl0_mut(), f(Misctrl0(0)));
        }
        self
    }

    #[doc="Modify the MISCTRL0 register."]
    #[inline] pub fn with_misctrl0<F: FnOnce(Misctrl0) -> Misctrl0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.misctrl0_mut(), f(self.misctrl0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SDID register."]
    #[inline] pub fn sdid_mut(&self) -> *mut Sdid { 
        (self.0 + 0x24) as *mut Sdid
    }

    #[doc="Get the *const pointer for the SDID register."]
    #[inline] pub fn sdid_ptr(&self) -> *const Sdid { 
           self.sdid_mut()
    }

    #[doc="Read the SDID register."]
    #[inline] pub fn sdid(&self) -> Sdid { 
        unsafe {
            read_volatile(self.sdid_ptr())
        }
    }

    #[doc="Get the *mut pointer for the PLATCGC register."]
    #[inline] pub fn platcgc_mut(&self) -> *mut Platcgc { 
        (self.0 + 0x40) as *mut Platcgc
    }

    #[doc="Get the *const pointer for the PLATCGC register."]
    #[inline] pub fn platcgc_ptr(&self) -> *const Platcgc { 
           self.platcgc_mut()
    }

    #[doc="Read the PLATCGC register."]
    #[inline] pub fn platcgc(&self) -> Platcgc { 
        unsafe {
            read_volatile(self.platcgc_ptr())
        }
    }

    #[doc="Write the PLATCGC register."]
    #[inline] pub fn set_platcgc<F: FnOnce(Platcgc) -> Platcgc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.platcgc_mut(), f(Platcgc(0)));
        }
        self
    }

    #[doc="Modify the PLATCGC register."]
    #[inline] pub fn with_platcgc<F: FnOnce(Platcgc) -> Platcgc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.platcgc_mut(), f(self.platcgc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FCFG1 register."]
    #[inline] pub fn fcfg1_mut(&self) -> *mut Fcfg1 { 
        (self.0 + 0x4c) as *mut Fcfg1
    }

    #[doc="Get the *const pointer for the FCFG1 register."]
    #[inline] pub fn fcfg1_ptr(&self) -> *const Fcfg1 { 
           self.fcfg1_mut()
    }

    #[doc="Read the FCFG1 register."]
    #[inline] pub fn fcfg1(&self) -> Fcfg1 { 
        unsafe {
            read_volatile(self.fcfg1_ptr())
        }
    }

    #[doc="Write the FCFG1 register."]
    #[inline] pub fn set_fcfg1<F: FnOnce(Fcfg1) -> Fcfg1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fcfg1_mut(), f(Fcfg1(0)));
        }
        self
    }

    #[doc="Modify the FCFG1 register."]
    #[inline] pub fn with_fcfg1<F: FnOnce(Fcfg1) -> Fcfg1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fcfg1_mut(), f(self.fcfg1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the UIDH register."]
    #[inline] pub fn uidh_mut(&self) -> *mut Uidh { 
        (self.0 + 0x54) as *mut Uidh
    }

    #[doc="Get the *const pointer for the UIDH register."]
    #[inline] pub fn uidh_ptr(&self) -> *const Uidh { 
           self.uidh_mut()
    }

    #[doc="Read the UIDH register."]
    #[inline] pub fn uidh(&self) -> Uidh { 
        unsafe {
            read_volatile(self.uidh_ptr())
        }
    }

    #[doc="Get the *mut pointer for the UIDMH register."]
    #[inline] pub fn uidmh_mut(&self) -> *mut Uidmh { 
        (self.0 + 0x58) as *mut Uidmh
    }

    #[doc="Get the *const pointer for the UIDMH register."]
    #[inline] pub fn uidmh_ptr(&self) -> *const Uidmh { 
           self.uidmh_mut()
    }

    #[doc="Read the UIDMH register."]
    #[inline] pub fn uidmh(&self) -> Uidmh { 
        unsafe {
            read_volatile(self.uidmh_ptr())
        }
    }

    #[doc="Get the *mut pointer for the UIDML register."]
    #[inline] pub fn uidml_mut(&self) -> *mut Uidml { 
        (self.0 + 0x5c) as *mut Uidml
    }

    #[doc="Get the *const pointer for the UIDML register."]
    #[inline] pub fn uidml_ptr(&self) -> *const Uidml { 
           self.uidml_mut()
    }

    #[doc="Read the UIDML register."]
    #[inline] pub fn uidml(&self) -> Uidml { 
        unsafe {
            read_volatile(self.uidml_ptr())
        }
    }

    #[doc="Get the *mut pointer for the UIDL register."]
    #[inline] pub fn uidl_mut(&self) -> *mut Uidl { 
        (self.0 + 0x60) as *mut Uidl
    }

    #[doc="Get the *const pointer for the UIDL register."]
    #[inline] pub fn uidl_ptr(&self) -> *const Uidl { 
           self.uidl_mut()
    }

    #[doc="Read the UIDL register."]
    #[inline] pub fn uidl(&self) -> Uidl { 
        unsafe {
            read_volatile(self.uidl_ptr())
        }
    }

    #[doc="Get the *mut pointer for the CLKDIV4 register."]
    #[inline] pub fn clkdiv4_mut(&self) -> *mut Clkdiv4 { 
        (self.0 + 0x68) as *mut Clkdiv4
    }

    #[doc="Get the *const pointer for the CLKDIV4 register."]
    #[inline] pub fn clkdiv4_ptr(&self) -> *const Clkdiv4 { 
           self.clkdiv4_mut()
    }

    #[doc="Read the CLKDIV4 register."]
    #[inline] pub fn clkdiv4(&self) -> Clkdiv4 { 
        unsafe {
            read_volatile(self.clkdiv4_ptr())
        }
    }

    #[doc="Write the CLKDIV4 register."]
    #[inline] pub fn set_clkdiv4<F: FnOnce(Clkdiv4) -> Clkdiv4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkdiv4_mut(), f(Clkdiv4(0)));
        }
        self
    }

    #[doc="Modify the CLKDIV4 register."]
    #[inline] pub fn with_clkdiv4<F: FnOnce(Clkdiv4) -> Clkdiv4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkdiv4_mut(), f(self.clkdiv4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MISCTRL1 register."]
    #[inline] pub fn misctrl1_mut(&self) -> *mut Misctrl1 { 
        (self.0 + 0x6c) as *mut Misctrl1
    }

    #[doc="Get the *const pointer for the MISCTRL1 register."]
    #[inline] pub fn misctrl1_ptr(&self) -> *const Misctrl1 { 
           self.misctrl1_mut()
    }

    #[doc="Read the MISCTRL1 register."]
    #[inline] pub fn misctrl1(&self) -> Misctrl1 { 
        unsafe {
            read_volatile(self.misctrl1_ptr())
        }
    }

    #[doc="Write the MISCTRL1 register."]
    #[inline] pub fn set_misctrl1<F: FnOnce(Misctrl1) -> Misctrl1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.misctrl1_mut(), f(Misctrl1(0)));
        }
        self
    }

    #[doc="Modify the MISCTRL1 register."]
    #[inline] pub fn with_misctrl1<F: FnOnce(Misctrl1) -> Misctrl1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.misctrl1_mut(), f(self.misctrl1()));
        }
        self
    }

}

#[doc="Chip Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chipctl(pub u32);
impl Chipctl {
    #[doc="ADC interleave channel enable"]
    #[inline] pub fn adc_interleave_en(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADC_INTERLEAVE_EN != 0"]
    #[inline] pub fn test_adc_interleave_en(&self) -> bool {
        self.adc_interleave_en() != 0
    }

    #[doc="Sets the ADC_INTERLEAVE_EN field."]
    #[inline] pub fn set_adc_interleave_en<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CLKOUT Select"]
    #[inline] pub fn clkoutsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if CLKOUTSEL != 0"]
    #[inline] pub fn test_clkoutsel(&self) -> bool {
        self.clkoutsel() != 0
    }

    #[doc="Sets the CLKOUTSEL field."]
    #[inline] pub fn set_clkoutsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="CLKOUT Divide Ratio"]
    #[inline] pub fn clkoutdiv(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if CLKOUTDIV != 0"]
    #[inline] pub fn test_clkoutdiv(&self) -> bool {
        self.clkoutdiv() != 0
    }

    #[doc="Sets the CLKOUTDIV field."]
    #[inline] pub fn set_clkoutdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CLKOUT enable"]
    #[inline] pub fn clkouten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CLKOUTEN != 0"]
    #[inline] pub fn test_clkouten(&self) -> bool {
        self.clkouten() != 0
    }

    #[doc="Sets the CLKOUTEN field."]
    #[inline] pub fn set_clkouten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Debug trace clock select"]
    #[inline] pub fn traceclk_sel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TRACECLK_SEL != 0"]
    #[inline] pub fn test_traceclk_sel(&self) -> bool {
        self.traceclk_sel() != 0
    }

    #[doc="Sets the TRACECLK_SEL field."]
    #[inline] pub fn set_traceclk_sel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="PDB back-to-back select"]
    #[inline] pub fn pdb_bb_sel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PDB_BB_SEL != 0"]
    #[inline] pub fn test_pdb_bb_sel(&self) -> bool {
        self.pdb_bb_sel() != 0
    }

    #[doc="Sets the PDB_BB_SEL field."]
    #[inline] pub fn set_pdb_bb_sel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="ADC_SUPPLY"]
    #[inline] pub fn adc_supply(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if ADC_SUPPLY != 0"]
    #[inline] pub fn test_adc_supply(&self) -> bool {
        self.adc_supply() != 0
    }

    #[doc="Sets the ADC_SUPPLY field."]
    #[inline] pub fn set_adc_supply<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ADC_SUPPLYEN"]
    #[inline] pub fn adc_supplyen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if ADC_SUPPLYEN != 0"]
    #[inline] pub fn test_adc_supplyen(&self) -> bool {
        self.adc_supplyen() != 0
    }

    #[doc="Sets the ADC_SUPPLYEN field."]
    #[inline] pub fn set_adc_supplyen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="SRAMU_RETEN"]
    #[inline] pub fn sramu_reten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SRAMU_RETEN != 0"]
    #[inline] pub fn test_sramu_reten(&self) -> bool {
        self.sramu_reten() != 0
    }

    #[doc="Sets the SRAMU_RETEN field."]
    #[inline] pub fn set_sramu_reten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="SRAML_RETEN"]
    #[inline] pub fn sraml_reten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SRAML_RETEN != 0"]
    #[inline] pub fn test_sraml_reten(&self) -> bool {
        self.sraml_reten() != 0
    }

    #[doc="Sets the SRAML_RETEN field."]
    #[inline] pub fn set_sraml_reten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

}

impl From<u32> for Chipctl {
    #[inline]
    fn from(other: u32) -> Self {
         Chipctl(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftmopt0(pub u32);
impl Ftmopt0 {
    #[doc="FTM0 Fault X Select"]
    #[inline] pub fn ftm0fltxsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if FTM0FLTxSEL != 0"]
    #[inline] pub fn test_ftm0fltxsel(&self) -> bool {
        self.ftm0fltxsel() != 0
    }

    #[doc="Sets the FTM0FLTxSEL field."]
    #[inline] pub fn set_ftm0fltxsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="FTM1 Fault X Select"]
    #[inline] pub fn ftm1fltxsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if FTM1FLTxSEL != 0"]
    #[inline] pub fn test_ftm1fltxsel(&self) -> bool {
        self.ftm1fltxsel() != 0
    }

    #[doc="Sets the FTM1FLTxSEL field."]
    #[inline] pub fn set_ftm1fltxsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FTM2 Fault X Select"]
    #[inline] pub fn ftm2fltxsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if FTM2FLTxSEL != 0"]
    #[inline] pub fn test_ftm2fltxsel(&self) -> bool {
        self.ftm2fltxsel() != 0
    }

    #[doc="Sets the FTM2FLTxSEL field."]
    #[inline] pub fn set_ftm2fltxsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FTM3 Fault X Select"]
    #[inline] pub fn ftm3fltxsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if FTM3FLTxSEL != 0"]
    #[inline] pub fn test_ftm3fltxsel(&self) -> bool {
        self.ftm3fltxsel() != 0
    }

    #[doc="Sets the FTM3FLTxSEL field."]
    #[inline] pub fn set_ftm3fltxsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="FTM0 External Clock Pin Select"]
    #[inline] pub fn ftm0clksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if FTM0CLKSEL != 0"]
    #[inline] pub fn test_ftm0clksel(&self) -> bool {
        self.ftm0clksel() != 0
    }

    #[doc="Sets the FTM0CLKSEL field."]
    #[inline] pub fn set_ftm0clksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="FTM1 External Clock Pin Select"]
    #[inline] pub fn ftm1clksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3) as u8) } // [27:26]
    }

    #[doc="Returns true if FTM1CLKSEL != 0"]
    #[inline] pub fn test_ftm1clksel(&self) -> bool {
        self.ftm1clksel() != 0
    }

    #[doc="Sets the FTM1CLKSEL field."]
    #[inline] pub fn set_ftm1clksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="FTM2 External Clock Pin Select"]
    #[inline] pub fn ftm2clksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if FTM2CLKSEL != 0"]
    #[inline] pub fn test_ftm2clksel(&self) -> bool {
        self.ftm2clksel() != 0
    }

    #[doc="Sets the FTM2CLKSEL field."]
    #[inline] pub fn set_ftm2clksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="FTM3 External Clock Pin Select"]
    #[inline] pub fn ftm3clksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if FTM3CLKSEL != 0"]
    #[inline] pub fn test_ftm3clksel(&self) -> bool {
        self.ftm3clksel() != 0
    }

    #[doc="Sets the FTM3CLKSEL field."]
    #[inline] pub fn set_ftm3clksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Ftmopt0 {
    #[inline]
    fn from(other: u32) -> Self {
         Ftmopt0(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpoclks(pub u32);
impl Lpoclks {
    #[doc="1 kHz LPO_CLK enable"]
    #[inline] pub fn lpo1kclken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPO1KCLKEN != 0"]
    #[inline] pub fn test_lpo1kclken(&self) -> bool {
        self.lpo1kclken() != 0
    }

    #[doc="Sets the LPO1KCLKEN field."]
    #[inline] pub fn set_lpo1kclken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="32 kHz LPO_CLK enable"]
    #[inline] pub fn lpo32kclken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LPO32KCLKEN != 0"]
    #[inline] pub fn test_lpo32kclken(&self) -> bool {
        self.lpo32kclken() != 0
    }

    #[doc="Sets the LPO32KCLKEN field."]
    #[inline] pub fn set_lpo32kclken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LPO clock source select"]
    #[inline] pub fn lpoclksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if LPOCLKSEL != 0"]
    #[inline] pub fn test_lpoclksel(&self) -> bool {
        self.lpoclksel() != 0
    }

    #[doc="Sets the LPOCLKSEL field."]
    #[inline] pub fn set_lpoclksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="32 kHz clock source select"]
    #[inline] pub fn rtcclksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if RTCCLKSEL != 0"]
    #[inline] pub fn test_rtcclksel(&self) -> bool {
        self.rtcclksel() != 0
    }

    #[doc="Sets the RTCCLKSEL field."]
    #[inline] pub fn set_rtcclksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Lpoclks {
    #[inline]
    fn from(other: u32) -> Self {
         Lpoclks(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adcopt(pub u32);
impl Adcopt {
    #[doc="ADC0 trigger source select"]
    #[inline] pub fn adc0trgsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADC0TRGSEL != 0"]
    #[inline] pub fn test_adc0trgsel(&self) -> bool {
        self.adc0trgsel() != 0
    }

    #[doc="Sets the ADC0TRGSEL field."]
    #[inline] pub fn set_adc0trgsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC0 software pretrigger sources"]
    #[inline] pub fn adc0swpretrg(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if ADC0SWPRETRG != 0"]
    #[inline] pub fn test_adc0swpretrg(&self) -> bool {
        self.adc0swpretrg() != 0
    }

    #[doc="Sets the ADC0SWPRETRG field."]
    #[inline] pub fn set_adc0swpretrg<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="ADC0 pretrigger source select"]
    #[inline] pub fn adc0pretrgsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if ADC0PRETRGSEL != 0"]
    #[inline] pub fn test_adc0pretrgsel(&self) -> bool {
        self.adc0pretrgsel() != 0
    }

    #[doc="Sets the ADC0PRETRGSEL field."]
    #[inline] pub fn set_adc0pretrgsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADC1 trigger source select"]
    #[inline] pub fn adc1trgsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADC1TRGSEL != 0"]
    #[inline] pub fn test_adc1trgsel(&self) -> bool {
        self.adc1trgsel() != 0
    }

    #[doc="Sets the ADC1TRGSEL field."]
    #[inline] pub fn set_adc1trgsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADC1 software pretrigger sources"]
    #[inline] pub fn adc1swpretrg(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x7) as u8) } // [11:9]
    }

    #[doc="Returns true if ADC1SWPRETRG != 0"]
    #[inline] pub fn test_adc1swpretrg(&self) -> bool {
        self.adc1swpretrg() != 0
    }

    #[doc="Sets the ADC1SWPRETRG field."]
    #[inline] pub fn set_adc1swpretrg<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="ADC1 pretrigger source select"]
    #[inline] pub fn adc1pretrgsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if ADC1PRETRGSEL != 0"]
    #[inline] pub fn test_adc1pretrgsel(&self) -> bool {
        self.adc1pretrgsel() != 0
    }

    #[doc="Sets the ADC1PRETRGSEL field."]
    #[inline] pub fn set_adc1pretrgsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Adcopt {
    #[inline]
    fn from(other: u32) -> Self {
         Adcopt(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftmopt1(pub u32);
impl Ftmopt1 {
    #[doc="FTM0 Sync Bit"]
    #[inline] pub fn ftm0syncbit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FTM0SYNCBIT != 0"]
    #[inline] pub fn test_ftm0syncbit(&self) -> bool {
        self.ftm0syncbit() != 0
    }

    #[doc="Sets the FTM0SYNCBIT field."]
    #[inline] pub fn set_ftm0syncbit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="FTM1 Sync Bit"]
    #[inline] pub fn ftm1syncbit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FTM1SYNCBIT != 0"]
    #[inline] pub fn test_ftm1syncbit(&self) -> bool {
        self.ftm1syncbit() != 0
    }

    #[doc="Sets the FTM1SYNCBIT field."]
    #[inline] pub fn set_ftm1syncbit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FTM2 Sync Bit"]
    #[inline] pub fn ftm2syncbit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FTM2SYNCBIT != 0"]
    #[inline] pub fn test_ftm2syncbit(&self) -> bool {
        self.ftm2syncbit() != 0
    }

    #[doc="Sets the FTM2SYNCBIT field."]
    #[inline] pub fn set_ftm2syncbit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FTM3 Sync Bit"]
    #[inline] pub fn ftm3syncbit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FTM3SYNCBIT != 0"]
    #[inline] pub fn test_ftm3syncbit(&self) -> bool {
        self.ftm3syncbit() != 0
    }

    #[doc="Sets the FTM3SYNCBIT field."]
    #[inline] pub fn set_ftm3syncbit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FTM1 CH0 Select"]
    #[inline] pub fn ftm1ch0sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if FTM1CH0SEL != 0"]
    #[inline] pub fn test_ftm1ch0sel(&self) -> bool {
        self.ftm1ch0sel() != 0
    }

    #[doc="Sets the FTM1CH0SEL field."]
    #[inline] pub fn set_ftm1ch0sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FTM2 CH0 Select"]
    #[inline] pub fn ftm2ch0sel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if FTM2CH0SEL != 0"]
    #[inline] pub fn test_ftm2ch0sel(&self) -> bool {
        self.ftm2ch0sel() != 0
    }

    #[doc="Sets the FTM2CH0SEL field."]
    #[inline] pub fn set_ftm2ch0sel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="FTM2 CH1 Select"]
    #[inline] pub fn ftm2ch1sel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FTM2CH1SEL != 0"]
    #[inline] pub fn test_ftm2ch1sel(&self) -> bool {
        self.ftm2ch1sel() != 0
    }

    #[doc="Sets the FTM2CH1SEL field."]
    #[inline] pub fn set_ftm2ch1sel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FTM global load enable"]
    #[inline] pub fn ftmgldok(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FTMGLDOK != 0"]
    #[inline] pub fn test_ftmgldok(&self) -> bool {
        self.ftmgldok() != 0
    }

    #[doc="Sets the FTMGLDOK field."]
    #[inline] pub fn set_ftmgldok<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="FTM0 channel modulation select with FTM1_CH1"]
    #[inline] pub fn ftm0_outsel(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if FTM0_OUTSEL != 0"]
    #[inline] pub fn test_ftm0_outsel(&self) -> bool {
        self.ftm0_outsel() != 0
    }

    #[doc="Sets the FTM0_OUTSEL field."]
    #[inline] pub fn set_ftm0_outsel<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="FTM3 channel modulation select with FTM2_CH1"]
    #[inline] pub fn ftm3_outsel(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if FTM3_OUTSEL != 0"]
    #[inline] pub fn test_ftm3_outsel(&self) -> bool {
        self.ftm3_outsel() != 0
    }

    #[doc="Sets the FTM3_OUTSEL field."]
    #[inline] pub fn set_ftm3_outsel<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Ftmopt1 {
    #[inline]
    fn from(other: u32) -> Self {
         Ftmopt1(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Misctrl0(pub u32);
impl Misctrl0 {
    #[doc="FTM0 OBE CTRL bit"]
    #[inline] pub fn ftm0_obe_ctrl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if FTM0_OBE_CTRL != 0"]
    #[inline] pub fn test_ftm0_obe_ctrl(&self) -> bool {
        self.ftm0_obe_ctrl() != 0
    }

    #[doc="Sets the FTM0_OBE_CTRL field."]
    #[inline] pub fn set_ftm0_obe_ctrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="FTM1 OBE CTRL bit"]
    #[inline] pub fn ftm1_obe_ctrl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if FTM1_OBE_CTRL != 0"]
    #[inline] pub fn test_ftm1_obe_ctrl(&self) -> bool {
        self.ftm1_obe_ctrl() != 0
    }

    #[doc="Sets the FTM1_OBE_CTRL field."]
    #[inline] pub fn set_ftm1_obe_ctrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="FTM2 OBE CTRL bit"]
    #[inline] pub fn ftm2_obe_ctrl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FTM2_OBE_CTRL != 0"]
    #[inline] pub fn test_ftm2_obe_ctrl(&self) -> bool {
        self.ftm2_obe_ctrl() != 0
    }

    #[doc="Sets the FTM2_OBE_CTRL field."]
    #[inline] pub fn set_ftm2_obe_ctrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="FTM3 OBE CTRL bit"]
    #[inline] pub fn ftm3_obe_ctrl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if FTM3_OBE_CTRL != 0"]
    #[inline] pub fn test_ftm3_obe_ctrl(&self) -> bool {
        self.ftm3_obe_ctrl() != 0
    }

    #[doc="Sets the FTM3_OBE_CTRL field."]
    #[inline] pub fn set_ftm3_obe_ctrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

}

impl From<u32> for Misctrl0 {
    #[inline]
    fn from(other: u32) -> Self {
         Misctrl0(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdid(pub u32);
impl Sdid {
    #[doc="Features"]
    #[inline] pub fn features(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FEATURES != 0"]
    #[inline] pub fn test_features(&self) -> bool {
        self.features() != 0
    }

    #[doc="Sets the FEATURES field."]
    #[inline] pub fn set_features<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Package"]
    #[inline] pub fn package(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if PACKAGE != 0"]
    #[inline] pub fn test_package(&self) -> bool {
        self.package() != 0
    }

    #[doc="Sets the PACKAGE field."]
    #[inline] pub fn set_package<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Device revision number"]
    #[inline] pub fn revid(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if REVID != 0"]
    #[inline] pub fn test_revid(&self) -> bool {
        self.revid() != 0
    }

    #[doc="Sets the REVID field."]
    #[inline] pub fn set_revid<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="RAM size"]
    #[inline] pub fn ramsize(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if RAMSIZE != 0"]
    #[inline] pub fn test_ramsize(&self) -> bool {
        self.ramsize() != 0
    }

    #[doc="Sets the RAMSIZE field."]
    #[inline] pub fn set_ramsize<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Derivate"]
    #[inline] pub fn derivate(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if DERIVATE != 0"]
    #[inline] pub fn test_derivate(&self) -> bool {
        self.derivate() != 0
    }

    #[doc="Sets the DERIVATE field."]
    #[inline] pub fn set_derivate<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Subseries"]
    #[inline] pub fn subseries(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if SUBSERIES != 0"]
    #[inline] pub fn test_subseries(&self) -> bool {
        self.subseries() != 0
    }

    #[doc="Sets the SUBSERIES field."]
    #[inline] pub fn set_subseries<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="S32K product series generation"]
    #[inline] pub fn generation(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if GENERATION != 0"]
    #[inline] pub fn test_generation(&self) -> bool {
        self.generation() != 0
    }

    #[doc="Sets the GENERATION field."]
    #[inline] pub fn set_generation<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Sdid {
    #[inline]
    fn from(other: u32) -> Self {
         Sdid(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Platcgc(pub u32);
impl Platcgc {
    #[doc="MSCM Clock Gating Control"]
    #[inline] pub fn cgcmscm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CGCMSCM != 0"]
    #[inline] pub fn test_cgcmscm(&self) -> bool {
        self.cgcmscm() != 0
    }

    #[doc="Sets the CGCMSCM field."]
    #[inline] pub fn set_cgcmscm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="MPU Clock Gating Control"]
    #[inline] pub fn cgcmpu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CGCMPU != 0"]
    #[inline] pub fn test_cgcmpu(&self) -> bool {
        self.cgcmpu() != 0
    }

    #[doc="Sets the CGCMPU field."]
    #[inline] pub fn set_cgcmpu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DMA Clock Gating Control"]
    #[inline] pub fn cgcdma(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CGCDMA != 0"]
    #[inline] pub fn test_cgcdma(&self) -> bool {
        self.cgcdma() != 0
    }

    #[doc="Sets the CGCDMA field."]
    #[inline] pub fn set_cgcdma<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ERM Clock Gating Control"]
    #[inline] pub fn cgcerm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CGCERM != 0"]
    #[inline] pub fn test_cgcerm(&self) -> bool {
        self.cgcerm() != 0
    }

    #[doc="Sets the CGCERM field."]
    #[inline] pub fn set_cgcerm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EIM Clock Gating Control"]
    #[inline] pub fn cgceim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CGCEIM != 0"]
    #[inline] pub fn test_cgceim(&self) -> bool {
        self.cgceim() != 0
    }

    #[doc="Sets the CGCEIM field."]
    #[inline] pub fn set_cgceim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Platcgc {
    #[inline]
    fn from(other: u32) -> Self {
         Platcgc(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fcfg1(pub u32);
impl Fcfg1 {
    #[doc="FlexNVM partition"]
    #[inline] pub fn depart(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if DEPART != 0"]
    #[inline] pub fn test_depart(&self) -> bool {
        self.depart() != 0
    }

    #[doc="Sets the DEPART field."]
    #[inline] pub fn set_depart<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EEE SRAM SIZE"]
    #[inline] pub fn eeeramsize(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if EEERAMSIZE != 0"]
    #[inline] pub fn test_eeeramsize(&self) -> bool {
        self.eeeramsize() != 0
    }

    #[doc="Sets the EEERAMSIZE field."]
    #[inline] pub fn set_eeeramsize<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Fcfg1 {
    #[inline]
    fn from(other: u32) -> Self {
         Fcfg1(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uidh(pub u32);
impl Uidh {
    #[doc="Unique Identification"]
    #[inline] pub fn uid127_96(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if UID127_96 != 0"]
    #[inline] pub fn test_uid127_96(&self) -> bool {
        self.uid127_96() != 0
    }

    #[doc="Sets the UID127_96 field."]
    #[inline] pub fn set_uid127_96<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uidh {
    #[inline]
    fn from(other: u32) -> Self {
         Uidh(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uidmh(pub u32);
impl Uidmh {
    #[doc="Unique Identification"]
    #[inline] pub fn uid95_64(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if UID95_64 != 0"]
    #[inline] pub fn test_uid95_64(&self) -> bool {
        self.uid95_64() != 0
    }

    #[doc="Sets the UID95_64 field."]
    #[inline] pub fn set_uid95_64<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uidmh {
    #[inline]
    fn from(other: u32) -> Self {
         Uidmh(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uidml(pub u32);
impl Uidml {
    #[doc="Unique Identification"]
    #[inline] pub fn uid63_32(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if UID63_32 != 0"]
    #[inline] pub fn test_uid63_32(&self) -> bool {
        self.uid63_32() != 0
    }

    #[doc="Sets the UID63_32 field."]
    #[inline] pub fn set_uid63_32<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uidml {
    #[inline]
    fn from(other: u32) -> Self {
         Uidml(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uidl(pub u32);
impl Uidl {
    #[doc="Unique Identification"]
    #[inline] pub fn uid31_0(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if UID31_0 != 0"]
    #[inline] pub fn test_uid31_0(&self) -> bool {
        self.uid31_0() != 0
    }

    #[doc="Sets the UID31_0 field."]
    #[inline] pub fn set_uid31_0<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uidl {
    #[inline]
    fn from(other: u32) -> Self {
         Uidl(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkdiv4(pub u32);
impl Clkdiv4 {
    #[doc="Trace Clock Divider fraction To configure TRACEDIV and TRACEFRAC, you must first clear TRACEDIVEN to disable the trace clock divide function."]
    #[inline] pub fn tracefrac(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TRACEFRAC != 0"]
    #[inline] pub fn test_tracefrac(&self) -> bool {
        self.tracefrac() != 0
    }

    #[doc="Sets the TRACEFRAC field."]
    #[inline] pub fn set_tracefrac<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Trace Clock Divider value To configure TRACEDIV, you must first disable TRACEDIVEN, then enable it after setting TRACEDIV."]
    #[inline] pub fn tracediv(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if TRACEDIV != 0"]
    #[inline] pub fn test_tracediv(&self) -> bool {
        self.tracediv() != 0
    }

    #[doc="Sets the TRACEDIV field."]
    #[inline] pub fn set_tracediv<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Debug Trace Divider control"]
    #[inline] pub fn tracediven(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TRACEDIVEN != 0"]
    #[inline] pub fn test_tracediven(&self) -> bool {
        self.tracediven() != 0
    }

    #[doc="Sets the TRACEDIVEN field."]
    #[inline] pub fn set_tracediven<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Clkdiv4 {
    #[inline]
    fn from(other: u32) -> Self {
         Clkdiv4(other)
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Misctrl1(pub u32);
impl Misctrl1 {
    #[doc="Software trigger to TRGMUX. Writing to this bit generates software trigger to peripherals through TRGMUX (Refer to Figure: Trigger interconnectivity)."]
    #[inline] pub fn sw_trg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SW_TRG != 0"]
    #[inline] pub fn test_sw_trg(&self) -> bool {
        self.sw_trg() != 0
    }

    #[doc="Sets the SW_TRG field."]
    #[inline] pub fn set_sw_trg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Misctrl1 {
    #[inline]
    fn from(other: u32) -> Self {
         Misctrl1(other)
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

