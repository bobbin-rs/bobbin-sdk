#[allow(unused_imports)] use bobbin_common::*;

periph!( ADC, Adc, _ADC, AdcPeriph, 0x50010000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 

impl super::sig::Signal<super::sig::Adcse0> for Adc {}
impl super::sig::SignalAdcSe0<super::sig::Adcse0> for Adc {}
impl super::sig::Signal<super::sig::Adcse1> for Adc {}
impl super::sig::SignalAdcSe1<super::sig::Adcse1> for Adc {}
impl super::sig::Signal<super::sig::Adcse2> for Adc {}
impl super::sig::SignalAdcSe2<super::sig::Adcse2> for Adc {}
impl super::sig::Signal<super::sig::Adcse3> for Adc {}
impl super::sig::SignalAdcSe3<super::sig::Adcse3> for Adc {}
impl super::sig::Signal<super::sig::Adcse4> for Adc {}
impl super::sig::SignalAdcSe4<super::sig::Adcse4> for Adc {}
impl super::sig::Signal<super::sig::Adcse5> for Adc {}
impl super::sig::SignalAdcSe5<super::sig::Adcse5> for Adc {}
impl super::sig::Signal<super::sig::Adcse6> for Adc {}
impl super::sig::SignalAdcSe6<super::sig::Adcse6> for Adc {}
impl super::sig::Signal<super::sig::Adcse7> for Adc {}
impl super::sig::SignalAdcSe7<super::sig::Adcse7> for Adc {}
impl super::sig::Signal<super::sig::Adcd0pse8> for Adc {}
impl super::sig::SignalAdcD0p<super::sig::Adcd0pse8> for Adc {}
impl super::sig::SignalAdcSe8<super::sig::Adcd0pse8> for Adc {}
impl super::sig::Signal<super::sig::Adcd0nse9> for Adc {}
impl super::sig::SignalAdcD0n<super::sig::Adcd0nse9> for Adc {}
impl super::sig::SignalAdcSe9<super::sig::Adcd0nse9> for Adc {}
impl super::sig::Signal<super::sig::Adcd1p> for Adc {}
impl super::sig::SignalAdcD1p<super::sig::Adcd1p> for Adc {}
impl super::sig::Signal<super::sig::Adcd1n> for Adc {}
impl super::sig::SignalAdcD1n<super::sig::Adcd1n> for Adc {}
impl super::sig::Signal<super::sig::Trig0> for Adc {}
impl super::sig::SignalAdcTrig0<super::sig::Trig0> for Adc {}
impl super::sig::Signal<super::sig::Trig1> for Adc {}
impl super::sig::SignalAdcTrig1<super::sig::Trig1> for Adc {}
impl super::sig::Signal<super::sig::Trig2> for Adc {}
impl super::sig::SignalAdcTrig2<super::sig::Trig2> for Adc {}
impl super::sig::Signal<super::sig::Trig3> for Adc {}
impl super::sig::SignalAdcTrig3<super::sig::Trig3> for Adc {}


impl AdcPeriph {
    #[doc="Get the *mut pointer for the CFG register."]
    #[inline] pub fn cfg_mut(&self) -> *mut Cfg { 
        (self.0 + 0x0) as *mut Cfg
    }

    #[doc="Get the *const pointer for the CFG register."]
    #[inline] pub fn cfg_ptr(&self) -> *const Cfg { 
           self.cfg_mut()
    }

    #[doc="Read the CFG register."]
    #[inline] pub fn cfg(&self) -> Cfg { 
        unsafe {
            read_volatile(self.cfg_ptr())
        }
    }

    #[doc="Write the CFG register."]
    #[inline] pub fn set_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(), f(Cfg(0)));
        }
        self
    }

    #[doc="Modify the CFG register."]
    #[inline] pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(), f(self.cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STAT register."]
    #[inline] pub fn stat_mut(&self) -> *mut Stat { 
        (self.0 + 0x4) as *mut Stat
    }

    #[doc="Get the *const pointer for the STAT register."]
    #[inline] pub fn stat_ptr(&self) -> *const Stat { 
           self.stat_mut()
    }

    #[doc="Read the STAT register."]
    #[inline] pub fn stat(&self) -> Stat { 
        unsafe {
            read_volatile(self.stat_ptr())
        }
    }

    #[doc="Write the STAT register."]
    #[inline] pub fn set_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stat_mut(), f(Stat(0)));
        }
        self
    }

    #[doc="Modify the STAT register."]
    #[inline] pub fn with_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stat_mut(), f(self.stat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SWT register."]
    #[inline] pub fn swt_mut(&self) -> *mut Swt { 
        (self.0 + 0x8) as *mut Swt
    }

    #[doc="Get the *const pointer for the SWT register."]
    #[inline] pub fn swt_ptr(&self) -> *const Swt { 
           self.swt_mut()
    }

    #[doc="Read the SWT register."]
    #[inline] pub fn swt(&self) -> Swt { 
        unsafe {
            read_volatile(self.swt_ptr())
        }
    }

    #[doc="Write the SWT register."]
    #[inline] pub fn set_swt<F: FnOnce(Swt) -> Swt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swt_mut(), f(Swt(0)));
        }
        self
    }

    #[doc="Modify the SWT register."]
    #[inline] pub fn with_swt<F: FnOnce(Swt) -> Swt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swt_mut(), f(self.swt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SLCFG register."]
    #[inline] pub fn slcfg_mut<I: Into<bits::R8>>(&self, index: I) -> *mut Slcfg { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xc + (index << 2)) as *mut Slcfg
    }

    #[doc="Get the *const pointer for the SLCFG register."]
    #[inline] pub fn slcfg_ptr<I: Into<bits::R8>>(&self, index: I) -> *const Slcfg { 
           self.slcfg_mut(index)
    }

    #[doc="Read the SLCFG register."]
    #[inline] pub fn slcfg<I: Into<bits::R8>>(&self, index: I) -> Slcfg { 
        unsafe {
            read_volatile(self.slcfg_ptr(index))
        }
    }

    #[doc="Write the SLCFG register."]
    #[inline] pub fn set_slcfg<I: Into<bits::R8>, F: FnOnce(Slcfg) -> Slcfg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.slcfg_mut(index), f(Slcfg(0)));
        }
        self
    }

    #[doc="Modify the SLCFG register."]
    #[inline] pub fn with_slcfg<I: Into<bits::R8> + Copy, F: FnOnce(Slcfg) -> Slcfg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.slcfg_mut(index), f(self.slcfg(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the WULIM register."]
    #[inline] pub fn wulim_mut(&self) -> *mut Wulim { 
        (self.0 + 0x2c) as *mut Wulim
    }

    #[doc="Get the *const pointer for the WULIM register."]
    #[inline] pub fn wulim_ptr(&self) -> *const Wulim { 
           self.wulim_mut()
    }

    #[doc="Read the WULIM register."]
    #[inline] pub fn wulim(&self) -> Wulim { 
        unsafe {
            read_volatile(self.wulim_ptr())
        }
    }

    #[doc="Write the WULIM register."]
    #[inline] pub fn set_wulim<F: FnOnce(Wulim) -> Wulim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wulim_mut(), f(Wulim(0)));
        }
        self
    }

    #[doc="Modify the WULIM register."]
    #[inline] pub fn with_wulim<F: FnOnce(Wulim) -> Wulim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wulim_mut(), f(self.wulim()));
        }
        self
    }

    #[doc="Get the *mut pointer for the WLLIM register."]
    #[inline] pub fn wllim_mut(&self) -> *mut Wllim { 
        (self.0 + 0x30) as *mut Wllim
    }

    #[doc="Get the *const pointer for the WLLIM register."]
    #[inline] pub fn wllim_ptr(&self) -> *const Wllim { 
           self.wllim_mut()
    }

    #[doc="Read the WLLIM register."]
    #[inline] pub fn wllim(&self) -> Wllim { 
        unsafe {
            read_volatile(self.wllim_ptr())
        }
    }

    #[doc="Write the WLLIM register."]
    #[inline] pub fn set_wllim<F: FnOnce(Wllim) -> Wllim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wllim_mut(), f(Wllim(0)));
        }
        self
    }

    #[doc="Modify the WLLIM register."]
    #[inline] pub fn with_wllim<F: FnOnce(Wllim) -> Wllim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wllim_mut(), f(self.wllim()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFO register."]
    #[inline] pub fn fifo_mut(&self) -> *mut Fifo { 
        (self.0 + 0x38) as *mut Fifo
    }

    #[doc="Get the *const pointer for the FIFO register."]
    #[inline] pub fn fifo_ptr(&self) -> *const Fifo { 
           self.fifo_mut()
    }

    #[doc="Read the FIFO register."]
    #[inline] pub fn fifo(&self) -> Fifo { 
        unsafe {
            read_volatile(self.fifo_ptr())
        }
    }

    #[doc="Write the FIFO register."]
    #[inline] pub fn set_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifo_mut(), f(Fifo(0)));
        }
        self
    }

    #[doc="Modify the FIFO register."]
    #[inline] pub fn with_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifo_mut(), f(self.fifo()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTEN register."]
    #[inline] pub fn inten_mut(&self) -> *mut Inten { 
        (self.0 + 0x200) as *mut Inten
    }

    #[doc="Get the *const pointer for the INTEN register."]
    #[inline] pub fn inten_ptr(&self) -> *const Inten { 
           self.inten_mut()
    }

    #[doc="Read the INTEN register."]
    #[inline] pub fn inten(&self) -> Inten { 
        unsafe {
            read_volatile(self.inten_ptr())
        }
    }

    #[doc="Write the INTEN register."]
    #[inline] pub fn set_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(), f(Inten(0)));
        }
        self
    }

    #[doc="Modify the INTEN register."]
    #[inline] pub fn with_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(), f(self.inten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSTAT register."]
    #[inline] pub fn intstat_mut(&self) -> *mut Intstat { 
        (self.0 + 0x204) as *mut Intstat
    }

    #[doc="Get the *const pointer for the INTSTAT register."]
    #[inline] pub fn intstat_ptr(&self) -> *const Intstat { 
           self.intstat_mut()
    }

    #[doc="Read the INTSTAT register."]
    #[inline] pub fn intstat(&self) -> Intstat { 
        unsafe {
            read_volatile(self.intstat_ptr())
        }
    }

    #[doc="Write the INTSTAT register."]
    #[inline] pub fn set_intstat<F: FnOnce(Intstat) -> Intstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(), f(Intstat(0)));
        }
        self
    }

    #[doc="Modify the INTSTAT register."]
    #[inline] pub fn with_intstat<F: FnOnce(Intstat) -> Intstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(), f(self.intstat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTCLR register."]
    #[inline] pub fn intclr_mut(&self) -> *mut Intclr { 
        (self.0 + 0x208) as *mut Intclr
    }

    #[doc="Get the *const pointer for the INTCLR register."]
    #[inline] pub fn intclr_ptr(&self) -> *const Intclr { 
           self.intclr_mut()
    }

    #[doc="Read the INTCLR register."]
    #[inline] pub fn intclr(&self) -> Intclr { 
        unsafe {
            read_volatile(self.intclr_ptr())
        }
    }

    #[doc="Write the INTCLR register."]
    #[inline] pub fn set_intclr<F: FnOnce(Intclr) -> Intclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(), f(Intclr(0)));
        }
        self
    }

    #[doc="Modify the INTCLR register."]
    #[inline] pub fn with_intclr<F: FnOnce(Intclr) -> Intclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(), f(self.intclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSET register."]
    #[inline] pub fn intset_mut(&self) -> *mut Intset { 
        (self.0 + 0x20c) as *mut Intset
    }

    #[doc="Get the *const pointer for the INTSET register."]
    #[inline] pub fn intset_ptr(&self) -> *const Intset { 
           self.intset_mut()
    }

    #[doc="Read the INTSET register."]
    #[inline] pub fn intset(&self) -> Intset { 
        unsafe {
            read_volatile(self.intset_ptr())
        }
    }

    #[doc="Write the INTSET register."]
    #[inline] pub fn set_intset<F: FnOnce(Intset) -> Intset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intset_mut(), f(Intset(0)));
        }
        self
    }

    #[doc="Modify the INTSET register."]
    #[inline] pub fn with_intset<F: FnOnce(Intset) -> Intset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intset_mut(), f(self.intset()));
        }
        self
    }

}

#[doc="Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc="Select the source and frequency for the ADC clock. All values not enumerated below are undefined."]
    #[inline] pub fn clksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if CLKSEL != 0"]
    #[inline] pub fn test_clksel(&self) -> bool {
        self.clksel() != 0
    }

    #[doc="Sets the CLKSEL field."]
    #[inline] pub fn set_clksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="This bit selects the ADC trigger polarity for external off chip triggers."]
    #[inline] pub fn trigpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if TRIGPOL != 0"]
    #[inline] pub fn test_trigpol(&self) -> bool {
        self.trigpol() != 0
    }

    #[doc="Sets the TRIGPOL field."]
    #[inline] pub fn set_trigpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Select the ADC trigger source."]
    #[inline] pub fn trigsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if TRIGSEL != 0"]
    #[inline] pub fn test_trigsel(&self) -> bool {
        self.trigsel() != 0
    }

    #[doc="Sets the TRIGSEL field."]
    #[inline] pub fn set_trigsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Select the ADC reference voltage."]
    #[inline] pub fn refsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if REFSEL != 0"]
    #[inline] pub fn test_refsel(&self) -> bool {
        self.refsel() != 0
    }

    #[doc="Sets the REFSEL field."]
    #[inline] pub fn set_refsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock mode register"]
    #[inline] pub fn ckmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CKMODE != 0"]
    #[inline] pub fn test_ckmode(&self) -> bool {
        self.ckmode() != 0
    }

    #[doc="Sets the CKMODE field."]
    #[inline] pub fn set_ckmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Select power mode to enter between active scans."]
    #[inline] pub fn lpmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LPMODE != 0"]
    #[inline] pub fn test_lpmode(&self) -> bool {
        self.lpmode() != 0
    }

    #[doc="Sets the LPMODE field."]
    #[inline] pub fn set_lpmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit enables Repeating Scan Mode."]
    #[inline] pub fn rpten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RPTEN != 0"]
    #[inline] pub fn test_rpten(&self) -> bool {
        self.rpten() != 0
    }

    #[doc="Sets the RPTEN field."]
    #[inline] pub fn set_rpten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to \'1\'."]
    #[inline] pub fn adcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADCEN != 0"]
    #[inline] pub fn test_adcen(&self) -> bool {
        self.adcen() != 0
    }

    #[doc="Sets the ADCEN field."]
    #[inline] pub fn set_adcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.clksel() != 0 { try!(write!(f, " clksel=0x{:x}", self.clksel()))}
        if self.trigpol() != 0 { try!(write!(f, " trigpol"))}
        if self.trigsel() != 0 { try!(write!(f, " trigsel=0x{:x}", self.trigsel()))}
        if self.refsel() != 0 { try!(write!(f, " refsel=0x{:x}", self.refsel()))}
        if self.ckmode() != 0 { try!(write!(f, " ckmode"))}
        if self.lpmode() != 0 { try!(write!(f, " lpmode"))}
        if self.rpten() != 0 { try!(write!(f, " rpten"))}
        if self.adcen() != 0 { try!(write!(f, " adcen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Power Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc="Indicates the power-status of the ADC."]
    #[inline] pub fn pwdstat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PWDSTAT != 0"]
    #[inline] pub fn test_pwdstat(&self) -> bool {
        self.pwdstat() != 0
    }

    #[doc="Sets the PWDSTAT field."]
    #[inline] pub fn set_pwdstat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stat {
    #[inline]
    fn from(other: u32) -> Self {
         Stat(other)
    }
}

impl ::core::fmt::Display for Stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pwdstat() != 0 { try!(write!(f, " pwdstat"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software trigger"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swt(pub u32);
impl Swt {
    #[doc="Writing 0x37 to this register generates a software trigger."]
    #[inline] pub fn swt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SWT != 0"]
    #[inline] pub fn test_swt(&self) -> bool {
        self.swt() != 0
    }

    #[doc="Sets the SWT field."]
    #[inline] pub fn set_swt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Swt {
    #[inline]
    fn from(other: u32) -> Self {
         Swt(other)
    }
}

impl ::core::fmt::Display for Swt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swt() != 0 { try!(write!(f, " swt=0x{:x}", self.swt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slot 0 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Slcfg(pub u32);
impl Slcfg {
    #[doc="Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline] pub fn adsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if ADSEL != 0"]
    #[inline] pub fn test_adsel(&self) -> bool {
        self.adsel() != 0
    }

    #[doc="Sets the ADSEL field."]
    #[inline] pub fn set_adsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Set the Precision Mode For Slot."]
    #[inline] pub fn prmode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PRMODE != 0"]
    #[inline] pub fn test_prmode(&self) -> bool {
        self.prmode() != 0
    }

    #[doc="Sets the PRMODE field."]
    #[inline] pub fn set_prmode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Select one of the 14 channel inputs for this slot."]
    #[inline] pub fn chsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CHSEL != 0"]
    #[inline] pub fn test_chsel(&self) -> bool {
        self.chsel() != 0
    }

    #[doc="Sets the CHSEL field."]
    #[inline] pub fn set_chsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit enables the window compare function for slot 0."]
    #[inline] pub fn wcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WCEN != 0"]
    #[inline] pub fn test_wcen(&self) -> bool {
        self.wcen() != 0
    }

    #[doc="Sets the WCEN field."]
    #[inline] pub fn set_wcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit enables slot 0 for ADC conversions."]
    #[inline] pub fn slen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEN != 0"]
    #[inline] pub fn test_slen(&self) -> bool {
        self.slen() != 0
    }

    #[doc="Sets the SLEN field."]
    #[inline] pub fn set_slen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Slcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Slcfg(other)
    }
}

impl ::core::fmt::Display for Slcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Slcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adsel() != 0 { try!(write!(f, " adsel=0x{:x}", self.adsel()))}
        if self.prmode() != 0 { try!(write!(f, " prmode=0x{:x}", self.prmode()))}
        if self.chsel() != 0 { try!(write!(f, " chsel=0x{:x}", self.chsel()))}
        if self.wcen() != 0 { try!(write!(f, " wcen"))}
        if self.slen() != 0 { try!(write!(f, " slen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Window Comparator Upper Limits Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wulim(pub u32);
impl Wulim {
    #[doc="Sets the upper limit for the wondow comparator."]
    #[inline] pub fn ulim(&self) -> bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfffff) as u32) } // [19:0]
    }

    #[doc="Returns true if ULIM != 0"]
    #[inline] pub fn test_ulim(&self) -> bool {
        self.ulim() != 0
    }

    #[doc="Sets the ULIM field."]
    #[inline] pub fn set_ulim<V: Into<bits::U20>>(mut self, value: V) -> Self {
        let value: bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wulim {
    #[inline]
    fn from(other: u32) -> Self {
         Wulim(other)
    }
}

impl ::core::fmt::Display for Wulim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wulim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ulim() != 0 { try!(write!(f, " ulim=0x{:x}", self.ulim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Window Comparator Lower Limits Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wllim(pub u32);
impl Wllim {
    #[doc="Sets the lower limit for the wondow comparator."]
    #[inline] pub fn llim(&self) -> bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfffff) as u32) } // [19:0]
    }

    #[doc="Returns true if LLIM != 0"]
    #[inline] pub fn test_llim(&self) -> bool {
        self.llim() != 0
    }

    #[doc="Sets the LLIM field."]
    #[inline] pub fn set_llim<V: Into<bits::U20>>(mut self, value: V) -> Self {
        let value: bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wllim {
    #[inline]
    fn from(other: u32) -> Self {
         Wllim(other)
    }
}

impl ::core::fmt::Display for Wllim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wllim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.llim() != 0 { try!(write!(f, " llim=0x{:x}", self.llim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Data and Valid Count Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc="RESERVED."]
    #[inline] pub fn rsvd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if RSVD != 0"]
    #[inline] pub fn test_rsvd(&self) -> bool {
        self.rsvd() != 0
    }

    #[doc="Sets the RSVD field."]
    #[inline] pub fn set_rsvd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Slot number associated with this FIFO data."]
    #[inline] pub fn slotnum(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
    }

    #[doc="Returns true if SLOTNUM != 0"]
    #[inline] pub fn test_slotnum(&self) -> bool {
        self.slotnum() != 0
    }

    #[doc="Sets the SLOTNUM field."]
    #[inline] pub fn set_slotnum<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Number of valid entries in the ADC FIFO."]
    #[inline] pub fn count(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xff) as u8) } // [27:20]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Oldest data in the FIFO."]
    #[inline] pub fn data(&self) -> bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfffff) as u32) } // [19:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U20>>(mut self, value: V) -> Self {
        let value: bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifo {
    #[inline]
    fn from(other: u32) -> Self {
         Fifo(other)
    }
}

impl ::core::fmt::Display for Fifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rsvd() != 0 { try!(write!(f, " rsvd"))}
        if self.slotnum() != 0 { try!(write!(f, " slotnum=0x{:x}", self.slotnum()))}
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Interrupt registers: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="Window comparator voltage incursion interrupt."]
    #[inline] pub fn wcinc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WCINC != 0"]
    #[inline] pub fn test_wcinc(&self) -> bool {
        self.wcinc() != 0
    }

    #[doc="Sets the WCINC field."]
    #[inline] pub fn set_wcinc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Window comparator voltage excursion interrupt."]
    #[inline] pub fn wcexc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WCEXC != 0"]
    #[inline] pub fn test_wcexc(&self) -> bool {
        self.wcexc() != 0
    }

    #[doc="Sets the WCEXC field."]
    #[inline] pub fn set_wcexc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO 100 percent full interrupt."]
    #[inline] pub fn fifoovr2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FIFOOVR2 != 0"]
    #[inline] pub fn test_fifoovr2(&self) -> bool {
        self.fifoovr2() != 0
    }

    #[doc="Sets the FIFOOVR2 field."]
    #[inline] pub fn set_fifoovr2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO 75 percent full interrupt."]
    #[inline] pub fn fifoovr1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FIFOOVR1 != 0"]
    #[inline] pub fn test_fifoovr1(&self) -> bool {
        self.fifoovr1() != 0
    }

    #[doc="Sets the FIFOOVR1 field."]
    #[inline] pub fn set_fifoovr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ADC scan complete interrupt."]
    #[inline] pub fn scncmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SCNCMP != 0"]
    #[inline] pub fn test_scncmp(&self) -> bool {
        self.scncmp() != 0
    }

    #[doc="Sets the SCNCMP field."]
    #[inline] pub fn set_scncmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="ADC conversion complete interrupt."]
    #[inline] pub fn cnvcmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CNVCMP != 0"]
    #[inline] pub fn test_cnvcmp(&self) -> bool {
        self.cnvcmp() != 0
    }

    #[doc="Sets the CNVCMP field."]
    #[inline] pub fn set_cnvcmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.wcinc() != 0 { try!(write!(f, " wcinc"))}
        if self.wcexc() != 0 { try!(write!(f, " wcexc"))}
        if self.fifoovr2() != 0 { try!(write!(f, " fifoovr2"))}
        if self.fifoovr1() != 0 { try!(write!(f, " fifoovr1"))}
        if self.scncmp() != 0 { try!(write!(f, " scncmp"))}
        if self.cnvcmp() != 0 { try!(write!(f, " cnvcmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Interrupt registers: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="Window comparator voltage incursion interrupt."]
    #[inline] pub fn wcinc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WCINC != 0"]
    #[inline] pub fn test_wcinc(&self) -> bool {
        self.wcinc() != 0
    }

    #[doc="Sets the WCINC field."]
    #[inline] pub fn set_wcinc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Window comparator voltage excursion interrupt."]
    #[inline] pub fn wcexc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WCEXC != 0"]
    #[inline] pub fn test_wcexc(&self) -> bool {
        self.wcexc() != 0
    }

    #[doc="Sets the WCEXC field."]
    #[inline] pub fn set_wcexc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO 100 percent full interrupt."]
    #[inline] pub fn fifoovr2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FIFOOVR2 != 0"]
    #[inline] pub fn test_fifoovr2(&self) -> bool {
        self.fifoovr2() != 0
    }

    #[doc="Sets the FIFOOVR2 field."]
    #[inline] pub fn set_fifoovr2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO 75 percent full interrupt."]
    #[inline] pub fn fifoovr1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FIFOOVR1 != 0"]
    #[inline] pub fn test_fifoovr1(&self) -> bool {
        self.fifoovr1() != 0
    }

    #[doc="Sets the FIFOOVR1 field."]
    #[inline] pub fn set_fifoovr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ADC scan complete interrupt."]
    #[inline] pub fn scncmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SCNCMP != 0"]
    #[inline] pub fn test_scncmp(&self) -> bool {
        self.scncmp() != 0
    }

    #[doc="Sets the SCNCMP field."]
    #[inline] pub fn set_scncmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="ADC conversion complete interrupt."]
    #[inline] pub fn cnvcmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CNVCMP != 0"]
    #[inline] pub fn test_cnvcmp(&self) -> bool {
        self.cnvcmp() != 0
    }

    #[doc="Sets the CNVCMP field."]
    #[inline] pub fn set_cnvcmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.wcinc() != 0 { try!(write!(f, " wcinc"))}
        if self.wcexc() != 0 { try!(write!(f, " wcexc"))}
        if self.fifoovr2() != 0 { try!(write!(f, " fifoovr2"))}
        if self.fifoovr1() != 0 { try!(write!(f, " fifoovr1"))}
        if self.scncmp() != 0 { try!(write!(f, " scncmp"))}
        if self.cnvcmp() != 0 { try!(write!(f, " cnvcmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Interrupt registers: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="Window comparator voltage incursion interrupt."]
    #[inline] pub fn wcinc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WCINC != 0"]
    #[inline] pub fn test_wcinc(&self) -> bool {
        self.wcinc() != 0
    }

    #[doc="Sets the WCINC field."]
    #[inline] pub fn set_wcinc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Window comparator voltage excursion interrupt."]
    #[inline] pub fn wcexc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WCEXC != 0"]
    #[inline] pub fn test_wcexc(&self) -> bool {
        self.wcexc() != 0
    }

    #[doc="Sets the WCEXC field."]
    #[inline] pub fn set_wcexc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO 100 percent full interrupt."]
    #[inline] pub fn fifoovr2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FIFOOVR2 != 0"]
    #[inline] pub fn test_fifoovr2(&self) -> bool {
        self.fifoovr2() != 0
    }

    #[doc="Sets the FIFOOVR2 field."]
    #[inline] pub fn set_fifoovr2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO 75 percent full interrupt."]
    #[inline] pub fn fifoovr1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FIFOOVR1 != 0"]
    #[inline] pub fn test_fifoovr1(&self) -> bool {
        self.fifoovr1() != 0
    }

    #[doc="Sets the FIFOOVR1 field."]
    #[inline] pub fn set_fifoovr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ADC scan complete interrupt."]
    #[inline] pub fn scncmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SCNCMP != 0"]
    #[inline] pub fn test_scncmp(&self) -> bool {
        self.scncmp() != 0
    }

    #[doc="Sets the SCNCMP field."]
    #[inline] pub fn set_scncmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="ADC conversion complete interrupt."]
    #[inline] pub fn cnvcmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CNVCMP != 0"]
    #[inline] pub fn test_cnvcmp(&self) -> bool {
        self.cnvcmp() != 0
    }

    #[doc="Sets the CNVCMP field."]
    #[inline] pub fn set_cnvcmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.wcinc() != 0 { try!(write!(f, " wcinc"))}
        if self.wcexc() != 0 { try!(write!(f, " wcexc"))}
        if self.fifoovr2() != 0 { try!(write!(f, " fifoovr2"))}
        if self.fifoovr1() != 0 { try!(write!(f, " fifoovr1"))}
        if self.scncmp() != 0 { try!(write!(f, " scncmp"))}
        if self.cnvcmp() != 0 { try!(write!(f, " cnvcmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Interrupt registers: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc="Window comparator voltage incursion interrupt."]
    #[inline] pub fn wcinc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WCINC != 0"]
    #[inline] pub fn test_wcinc(&self) -> bool {
        self.wcinc() != 0
    }

    #[doc="Sets the WCINC field."]
    #[inline] pub fn set_wcinc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Window comparator voltage excursion interrupt."]
    #[inline] pub fn wcexc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WCEXC != 0"]
    #[inline] pub fn test_wcexc(&self) -> bool {
        self.wcexc() != 0
    }

    #[doc="Sets the WCEXC field."]
    #[inline] pub fn set_wcexc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO 100 percent full interrupt."]
    #[inline] pub fn fifoovr2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FIFOOVR2 != 0"]
    #[inline] pub fn test_fifoovr2(&self) -> bool {
        self.fifoovr2() != 0
    }

    #[doc="Sets the FIFOOVR2 field."]
    #[inline] pub fn set_fifoovr2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO 75 percent full interrupt."]
    #[inline] pub fn fifoovr1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FIFOOVR1 != 0"]
    #[inline] pub fn test_fifoovr1(&self) -> bool {
        self.fifoovr1() != 0
    }

    #[doc="Sets the FIFOOVR1 field."]
    #[inline] pub fn set_fifoovr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ADC scan complete interrupt."]
    #[inline] pub fn scncmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SCNCMP != 0"]
    #[inline] pub fn test_scncmp(&self) -> bool {
        self.scncmp() != 0
    }

    #[doc="Sets the SCNCMP field."]
    #[inline] pub fn set_scncmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="ADC conversion complete interrupt."]
    #[inline] pub fn cnvcmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CNVCMP != 0"]
    #[inline] pub fn test_cnvcmp(&self) -> bool {
        self.cnvcmp() != 0
    }

    #[doc="Sets the CNVCMP field."]
    #[inline] pub fn set_cnvcmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intset {
    #[inline]
    fn from(other: u32) -> Self {
         Intset(other)
    }
}

impl ::core::fmt::Display for Intset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wcinc() != 0 { try!(write!(f, " wcinc"))}
        if self.wcexc() != 0 { try!(write!(f, " wcexc"))}
        if self.fifoovr2() != 0 { try!(write!(f, " fifoovr2"))}
        if self.fifoovr1() != 0 { try!(write!(f, " fifoovr1"))}
        if self.scncmp() != 0 { try!(write!(f, " scncmp"))}
        if self.cnvcmp() != 0 { try!(write!(f, " cnvcmp"))}
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
channel!(ADC_TEMP, AdcTemp, ADC, Adc, _ADC_TEMP, AdcCh, _ADC, 13);
channel!(ADC_BATT, AdcBatt, ADC, Adc, _ADC_BATT, AdcCh, _ADC, 14);
channel!(ADC_VSS, AdcVss, ADC, Adc, _ADC_VSS, AdcCh, _ADC, 15);

