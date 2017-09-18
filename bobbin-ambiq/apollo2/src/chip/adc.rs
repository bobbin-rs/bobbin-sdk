#[allow(unused_imports)] use bobbin_common::*;

periph!( ADC, Adc, _ADC, AdcPeriph, 0x50010000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 



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

    #[doc="Get the *mut pointer for the SL0CFG register."]
    #[inline] pub fn sl0cfg_mut(&self) -> *mut Sl0cfg { 
        (self.0 + 0xc) as *mut Sl0cfg
    }

    #[doc="Get the *const pointer for the SL0CFG register."]
    #[inline] pub fn sl0cfg_ptr(&self) -> *const Sl0cfg { 
           self.sl0cfg_mut()
    }

    #[doc="Read the SL0CFG register."]
    #[inline] pub fn sl0cfg(&self) -> Sl0cfg { 
        unsafe {
            read_volatile(self.sl0cfg_ptr())
        }
    }

    #[doc="Write the SL0CFG register."]
    #[inline] pub fn set_sl0cfg<F: FnOnce(Sl0cfg) -> Sl0cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl0cfg_mut(), f(Sl0cfg(0)));
        }
        self
    }

    #[doc="Modify the SL0CFG register."]
    #[inline] pub fn with_sl0cfg<F: FnOnce(Sl0cfg) -> Sl0cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl0cfg_mut(), f(self.sl0cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SL1CFG register."]
    #[inline] pub fn sl1cfg_mut(&self) -> *mut Sl1cfg { 
        (self.0 + 0x10) as *mut Sl1cfg
    }

    #[doc="Get the *const pointer for the SL1CFG register."]
    #[inline] pub fn sl1cfg_ptr(&self) -> *const Sl1cfg { 
           self.sl1cfg_mut()
    }

    #[doc="Read the SL1CFG register."]
    #[inline] pub fn sl1cfg(&self) -> Sl1cfg { 
        unsafe {
            read_volatile(self.sl1cfg_ptr())
        }
    }

    #[doc="Write the SL1CFG register."]
    #[inline] pub fn set_sl1cfg<F: FnOnce(Sl1cfg) -> Sl1cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl1cfg_mut(), f(Sl1cfg(0)));
        }
        self
    }

    #[doc="Modify the SL1CFG register."]
    #[inline] pub fn with_sl1cfg<F: FnOnce(Sl1cfg) -> Sl1cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl1cfg_mut(), f(self.sl1cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SL2CFG register."]
    #[inline] pub fn sl2cfg_mut(&self) -> *mut Sl2cfg { 
        (self.0 + 0x14) as *mut Sl2cfg
    }

    #[doc="Get the *const pointer for the SL2CFG register."]
    #[inline] pub fn sl2cfg_ptr(&self) -> *const Sl2cfg { 
           self.sl2cfg_mut()
    }

    #[doc="Read the SL2CFG register."]
    #[inline] pub fn sl2cfg(&self) -> Sl2cfg { 
        unsafe {
            read_volatile(self.sl2cfg_ptr())
        }
    }

    #[doc="Write the SL2CFG register."]
    #[inline] pub fn set_sl2cfg<F: FnOnce(Sl2cfg) -> Sl2cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl2cfg_mut(), f(Sl2cfg(0)));
        }
        self
    }

    #[doc="Modify the SL2CFG register."]
    #[inline] pub fn with_sl2cfg<F: FnOnce(Sl2cfg) -> Sl2cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl2cfg_mut(), f(self.sl2cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SL3CFG register."]
    #[inline] pub fn sl3cfg_mut(&self) -> *mut Sl3cfg { 
        (self.0 + 0x18) as *mut Sl3cfg
    }

    #[doc="Get the *const pointer for the SL3CFG register."]
    #[inline] pub fn sl3cfg_ptr(&self) -> *const Sl3cfg { 
           self.sl3cfg_mut()
    }

    #[doc="Read the SL3CFG register."]
    #[inline] pub fn sl3cfg(&self) -> Sl3cfg { 
        unsafe {
            read_volatile(self.sl3cfg_ptr())
        }
    }

    #[doc="Write the SL3CFG register."]
    #[inline] pub fn set_sl3cfg<F: FnOnce(Sl3cfg) -> Sl3cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl3cfg_mut(), f(Sl3cfg(0)));
        }
        self
    }

    #[doc="Modify the SL3CFG register."]
    #[inline] pub fn with_sl3cfg<F: FnOnce(Sl3cfg) -> Sl3cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl3cfg_mut(), f(self.sl3cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SL4CFG register."]
    #[inline] pub fn sl4cfg_mut(&self) -> *mut Sl4cfg { 
        (self.0 + 0x1c) as *mut Sl4cfg
    }

    #[doc="Get the *const pointer for the SL4CFG register."]
    #[inline] pub fn sl4cfg_ptr(&self) -> *const Sl4cfg { 
           self.sl4cfg_mut()
    }

    #[doc="Read the SL4CFG register."]
    #[inline] pub fn sl4cfg(&self) -> Sl4cfg { 
        unsafe {
            read_volatile(self.sl4cfg_ptr())
        }
    }

    #[doc="Write the SL4CFG register."]
    #[inline] pub fn set_sl4cfg<F: FnOnce(Sl4cfg) -> Sl4cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl4cfg_mut(), f(Sl4cfg(0)));
        }
        self
    }

    #[doc="Modify the SL4CFG register."]
    #[inline] pub fn with_sl4cfg<F: FnOnce(Sl4cfg) -> Sl4cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl4cfg_mut(), f(self.sl4cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SL5CFG register."]
    #[inline] pub fn sl5cfg_mut(&self) -> *mut Sl5cfg { 
        (self.0 + 0x20) as *mut Sl5cfg
    }

    #[doc="Get the *const pointer for the SL5CFG register."]
    #[inline] pub fn sl5cfg_ptr(&self) -> *const Sl5cfg { 
           self.sl5cfg_mut()
    }

    #[doc="Read the SL5CFG register."]
    #[inline] pub fn sl5cfg(&self) -> Sl5cfg { 
        unsafe {
            read_volatile(self.sl5cfg_ptr())
        }
    }

    #[doc="Write the SL5CFG register."]
    #[inline] pub fn set_sl5cfg<F: FnOnce(Sl5cfg) -> Sl5cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl5cfg_mut(), f(Sl5cfg(0)));
        }
        self
    }

    #[doc="Modify the SL5CFG register."]
    #[inline] pub fn with_sl5cfg<F: FnOnce(Sl5cfg) -> Sl5cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl5cfg_mut(), f(self.sl5cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SL6CFG register."]
    #[inline] pub fn sl6cfg_mut(&self) -> *mut Sl6cfg { 
        (self.0 + 0x24) as *mut Sl6cfg
    }

    #[doc="Get the *const pointer for the SL6CFG register."]
    #[inline] pub fn sl6cfg_ptr(&self) -> *const Sl6cfg { 
           self.sl6cfg_mut()
    }

    #[doc="Read the SL6CFG register."]
    #[inline] pub fn sl6cfg(&self) -> Sl6cfg { 
        unsafe {
            read_volatile(self.sl6cfg_ptr())
        }
    }

    #[doc="Write the SL6CFG register."]
    #[inline] pub fn set_sl6cfg<F: FnOnce(Sl6cfg) -> Sl6cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl6cfg_mut(), f(Sl6cfg(0)));
        }
        self
    }

    #[doc="Modify the SL6CFG register."]
    #[inline] pub fn with_sl6cfg<F: FnOnce(Sl6cfg) -> Sl6cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl6cfg_mut(), f(self.sl6cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SL7CFG register."]
    #[inline] pub fn sl7cfg_mut(&self) -> *mut Sl7cfg { 
        (self.0 + 0x28) as *mut Sl7cfg
    }

    #[doc="Get the *const pointer for the SL7CFG register."]
    #[inline] pub fn sl7cfg_ptr(&self) -> *const Sl7cfg { 
           self.sl7cfg_mut()
    }

    #[doc="Read the SL7CFG register."]
    #[inline] pub fn sl7cfg(&self) -> Sl7cfg { 
        unsafe {
            read_volatile(self.sl7cfg_ptr())
        }
    }

    #[doc="Write the SL7CFG register."]
    #[inline] pub fn set_sl7cfg<F: FnOnce(Sl7cfg) -> Sl7cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl7cfg_mut(), f(Sl7cfg(0)));
        }
        self
    }

    #[doc="Modify the SL7CFG register."]
    #[inline] pub fn with_sl7cfg<F: FnOnce(Sl7cfg) -> Sl7cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sl7cfg_mut(), f(self.sl7cfg()));
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
pub struct Sl0cfg(pub u32);
impl Sl0cfg {
    #[doc="Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline] pub fn adsel0(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if ADSEL0 != 0"]
    #[inline] pub fn test_adsel0(&self) -> bool {
        self.adsel0() != 0
    }

    #[doc="Sets the ADSEL0 field."]
    #[inline] pub fn set_adsel0<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Set the Precision Mode For Slot."]
    #[inline] pub fn prmode0(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PRMODE0 != 0"]
    #[inline] pub fn test_prmode0(&self) -> bool {
        self.prmode0() != 0
    }

    #[doc="Sets the PRMODE0 field."]
    #[inline] pub fn set_prmode0<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Select one of the 14 channel inputs for this slot."]
    #[inline] pub fn chsel0(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CHSEL0 != 0"]
    #[inline] pub fn test_chsel0(&self) -> bool {
        self.chsel0() != 0
    }

    #[doc="Sets the CHSEL0 field."]
    #[inline] pub fn set_chsel0<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit enables the window compare function for slot 0."]
    #[inline] pub fn wcen0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WCEN0 != 0"]
    #[inline] pub fn test_wcen0(&self) -> bool {
        self.wcen0() != 0
    }

    #[doc="Sets the WCEN0 field."]
    #[inline] pub fn set_wcen0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit enables slot 0 for ADC conversions."]
    #[inline] pub fn slen0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEN0 != 0"]
    #[inline] pub fn test_slen0(&self) -> bool {
        self.slen0() != 0
    }

    #[doc="Sets the SLEN0 field."]
    #[inline] pub fn set_slen0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sl0cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sl0cfg(other)
    }
}

impl ::core::fmt::Display for Sl0cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sl0cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adsel0() != 0 { try!(write!(f, " adsel0=0x{:x}", self.adsel0()))}
        if self.prmode0() != 0 { try!(write!(f, " prmode0=0x{:x}", self.prmode0()))}
        if self.chsel0() != 0 { try!(write!(f, " chsel0=0x{:x}", self.chsel0()))}
        if self.wcen0() != 0 { try!(write!(f, " wcen0"))}
        if self.slen0() != 0 { try!(write!(f, " slen0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slot 1 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sl1cfg(pub u32);
impl Sl1cfg {
    #[doc="Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline] pub fn adsel1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if ADSEL1 != 0"]
    #[inline] pub fn test_adsel1(&self) -> bool {
        self.adsel1() != 0
    }

    #[doc="Sets the ADSEL1 field."]
    #[inline] pub fn set_adsel1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Set the Precision Mode For Slot."]
    #[inline] pub fn prmode1(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PRMODE1 != 0"]
    #[inline] pub fn test_prmode1(&self) -> bool {
        self.prmode1() != 0
    }

    #[doc="Sets the PRMODE1 field."]
    #[inline] pub fn set_prmode1<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Select one of the 14 channel inputs for this slot."]
    #[inline] pub fn chsel1(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CHSEL1 != 0"]
    #[inline] pub fn test_chsel1(&self) -> bool {
        self.chsel1() != 0
    }

    #[doc="Sets the CHSEL1 field."]
    #[inline] pub fn set_chsel1<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit enables the window compare function for slot 1."]
    #[inline] pub fn wcen1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WCEN1 != 0"]
    #[inline] pub fn test_wcen1(&self) -> bool {
        self.wcen1() != 0
    }

    #[doc="Sets the WCEN1 field."]
    #[inline] pub fn set_wcen1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit enables slot 1 for ADC conversions."]
    #[inline] pub fn slen1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEN1 != 0"]
    #[inline] pub fn test_slen1(&self) -> bool {
        self.slen1() != 0
    }

    #[doc="Sets the SLEN1 field."]
    #[inline] pub fn set_slen1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sl1cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sl1cfg(other)
    }
}

impl ::core::fmt::Display for Sl1cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sl1cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adsel1() != 0 { try!(write!(f, " adsel1=0x{:x}", self.adsel1()))}
        if self.prmode1() != 0 { try!(write!(f, " prmode1=0x{:x}", self.prmode1()))}
        if self.chsel1() != 0 { try!(write!(f, " chsel1=0x{:x}", self.chsel1()))}
        if self.wcen1() != 0 { try!(write!(f, " wcen1"))}
        if self.slen1() != 0 { try!(write!(f, " slen1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slot 2 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sl2cfg(pub u32);
impl Sl2cfg {
    #[doc="Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline] pub fn adsel2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if ADSEL2 != 0"]
    #[inline] pub fn test_adsel2(&self) -> bool {
        self.adsel2() != 0
    }

    #[doc="Sets the ADSEL2 field."]
    #[inline] pub fn set_adsel2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Set the Precision Mode For Slot."]
    #[inline] pub fn prmode2(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PRMODE2 != 0"]
    #[inline] pub fn test_prmode2(&self) -> bool {
        self.prmode2() != 0
    }

    #[doc="Sets the PRMODE2 field."]
    #[inline] pub fn set_prmode2<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Select one of the 14 channel inputs for this slot."]
    #[inline] pub fn chsel2(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CHSEL2 != 0"]
    #[inline] pub fn test_chsel2(&self) -> bool {
        self.chsel2() != 0
    }

    #[doc="Sets the CHSEL2 field."]
    #[inline] pub fn set_chsel2<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit enables the window compare function for slot 2."]
    #[inline] pub fn wcen2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WCEN2 != 0"]
    #[inline] pub fn test_wcen2(&self) -> bool {
        self.wcen2() != 0
    }

    #[doc="Sets the WCEN2 field."]
    #[inline] pub fn set_wcen2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit enables slot 2 for ADC conversions."]
    #[inline] pub fn slen2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEN2 != 0"]
    #[inline] pub fn test_slen2(&self) -> bool {
        self.slen2() != 0
    }

    #[doc="Sets the SLEN2 field."]
    #[inline] pub fn set_slen2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sl2cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sl2cfg(other)
    }
}

impl ::core::fmt::Display for Sl2cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sl2cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adsel2() != 0 { try!(write!(f, " adsel2=0x{:x}", self.adsel2()))}
        if self.prmode2() != 0 { try!(write!(f, " prmode2=0x{:x}", self.prmode2()))}
        if self.chsel2() != 0 { try!(write!(f, " chsel2=0x{:x}", self.chsel2()))}
        if self.wcen2() != 0 { try!(write!(f, " wcen2"))}
        if self.slen2() != 0 { try!(write!(f, " slen2"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slot 3 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sl3cfg(pub u32);
impl Sl3cfg {
    #[doc="Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline] pub fn adsel3(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if ADSEL3 != 0"]
    #[inline] pub fn test_adsel3(&self) -> bool {
        self.adsel3() != 0
    }

    #[doc="Sets the ADSEL3 field."]
    #[inline] pub fn set_adsel3<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Set the Precision Mode For Slot."]
    #[inline] pub fn prmode3(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PRMODE3 != 0"]
    #[inline] pub fn test_prmode3(&self) -> bool {
        self.prmode3() != 0
    }

    #[doc="Sets the PRMODE3 field."]
    #[inline] pub fn set_prmode3<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Select one of the 14 channel inputs for this slot."]
    #[inline] pub fn chsel3(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CHSEL3 != 0"]
    #[inline] pub fn test_chsel3(&self) -> bool {
        self.chsel3() != 0
    }

    #[doc="Sets the CHSEL3 field."]
    #[inline] pub fn set_chsel3<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit enables the window compare function for slot 3."]
    #[inline] pub fn wcen3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WCEN3 != 0"]
    #[inline] pub fn test_wcen3(&self) -> bool {
        self.wcen3() != 0
    }

    #[doc="Sets the WCEN3 field."]
    #[inline] pub fn set_wcen3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit enables slot 3 for ADC conversions."]
    #[inline] pub fn slen3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEN3 != 0"]
    #[inline] pub fn test_slen3(&self) -> bool {
        self.slen3() != 0
    }

    #[doc="Sets the SLEN3 field."]
    #[inline] pub fn set_slen3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sl3cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sl3cfg(other)
    }
}

impl ::core::fmt::Display for Sl3cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sl3cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adsel3() != 0 { try!(write!(f, " adsel3=0x{:x}", self.adsel3()))}
        if self.prmode3() != 0 { try!(write!(f, " prmode3=0x{:x}", self.prmode3()))}
        if self.chsel3() != 0 { try!(write!(f, " chsel3=0x{:x}", self.chsel3()))}
        if self.wcen3() != 0 { try!(write!(f, " wcen3"))}
        if self.slen3() != 0 { try!(write!(f, " slen3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slot 4 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sl4cfg(pub u32);
impl Sl4cfg {
    #[doc="Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline] pub fn adsel4(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if ADSEL4 != 0"]
    #[inline] pub fn test_adsel4(&self) -> bool {
        self.adsel4() != 0
    }

    #[doc="Sets the ADSEL4 field."]
    #[inline] pub fn set_adsel4<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Set the Precision Mode For Slot."]
    #[inline] pub fn prmode4(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PRMODE4 != 0"]
    #[inline] pub fn test_prmode4(&self) -> bool {
        self.prmode4() != 0
    }

    #[doc="Sets the PRMODE4 field."]
    #[inline] pub fn set_prmode4<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Select one of the 14 channel inputs for this slot."]
    #[inline] pub fn chsel4(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CHSEL4 != 0"]
    #[inline] pub fn test_chsel4(&self) -> bool {
        self.chsel4() != 0
    }

    #[doc="Sets the CHSEL4 field."]
    #[inline] pub fn set_chsel4<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit enables the window compare function for slot 4."]
    #[inline] pub fn wcen4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WCEN4 != 0"]
    #[inline] pub fn test_wcen4(&self) -> bool {
        self.wcen4() != 0
    }

    #[doc="Sets the WCEN4 field."]
    #[inline] pub fn set_wcen4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit enables slot 4 for ADC conversions."]
    #[inline] pub fn slen4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEN4 != 0"]
    #[inline] pub fn test_slen4(&self) -> bool {
        self.slen4() != 0
    }

    #[doc="Sets the SLEN4 field."]
    #[inline] pub fn set_slen4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sl4cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sl4cfg(other)
    }
}

impl ::core::fmt::Display for Sl4cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sl4cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adsel4() != 0 { try!(write!(f, " adsel4=0x{:x}", self.adsel4()))}
        if self.prmode4() != 0 { try!(write!(f, " prmode4=0x{:x}", self.prmode4()))}
        if self.chsel4() != 0 { try!(write!(f, " chsel4=0x{:x}", self.chsel4()))}
        if self.wcen4() != 0 { try!(write!(f, " wcen4"))}
        if self.slen4() != 0 { try!(write!(f, " slen4"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slot 5 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sl5cfg(pub u32);
impl Sl5cfg {
    #[doc="Select number of measurements to average in the accumulate divide module for this slot."]
    #[inline] pub fn adsel5(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if ADSEL5 != 0"]
    #[inline] pub fn test_adsel5(&self) -> bool {
        self.adsel5() != 0
    }

    #[doc="Sets the ADSEL5 field."]
    #[inline] pub fn set_adsel5<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Set the Precision Mode For Slot."]
    #[inline] pub fn prmode5(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PRMODE5 != 0"]
    #[inline] pub fn test_prmode5(&self) -> bool {
        self.prmode5() != 0
    }

    #[doc="Sets the PRMODE5 field."]
    #[inline] pub fn set_prmode5<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Select one of the 14 channel inputs for this slot."]
    #[inline] pub fn chsel5(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CHSEL5 != 0"]
    #[inline] pub fn test_chsel5(&self) -> bool {
        self.chsel5() != 0
    }

    #[doc="Sets the CHSEL5 field."]
    #[inline] pub fn set_chsel5<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit enables the window compare function for slot 5."]
    #[inline] pub fn wcen5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WCEN5 != 0"]
    #[inline] pub fn test_wcen5(&self) -> bool {
        self.wcen5() != 0
    }

    #[doc="Sets the WCEN5 field."]
    #[inline] pub fn set_wcen5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit enables slot 5 for ADC conversions."]
    #[inline] pub fn slen5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEN5 != 0"]
    #[inline] pub fn test_slen5(&self) -> bool {
        self.slen5() != 0
    }

    #[doc="Sets the SLEN5 field."]
    #[inline] pub fn set_slen5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sl5cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sl5cfg(other)
    }
}

impl ::core::fmt::Display for Sl5cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sl5cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adsel5() != 0 { try!(write!(f, " adsel5=0x{:x}", self.adsel5()))}
        if self.prmode5() != 0 { try!(write!(f, " prmode5=0x{:x}", self.prmode5()))}
        if self.chsel5() != 0 { try!(write!(f, " chsel5=0x{:x}", self.chsel5()))}
        if self.wcen5() != 0 { try!(write!(f, " wcen5"))}
        if self.slen5() != 0 { try!(write!(f, " slen5"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slot 6 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sl6cfg(pub u32);
impl Sl6cfg {
    #[doc="Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline] pub fn adsel6(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if ADSEL6 != 0"]
    #[inline] pub fn test_adsel6(&self) -> bool {
        self.adsel6() != 0
    }

    #[doc="Sets the ADSEL6 field."]
    #[inline] pub fn set_adsel6<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Set the Precision Mode For Slot."]
    #[inline] pub fn prmode6(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PRMODE6 != 0"]
    #[inline] pub fn test_prmode6(&self) -> bool {
        self.prmode6() != 0
    }

    #[doc="Sets the PRMODE6 field."]
    #[inline] pub fn set_prmode6<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Select one of the 14 channel inputs for this slot."]
    #[inline] pub fn chsel6(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CHSEL6 != 0"]
    #[inline] pub fn test_chsel6(&self) -> bool {
        self.chsel6() != 0
    }

    #[doc="Sets the CHSEL6 field."]
    #[inline] pub fn set_chsel6<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit enables the window compare function for slot 6."]
    #[inline] pub fn wcen6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WCEN6 != 0"]
    #[inline] pub fn test_wcen6(&self) -> bool {
        self.wcen6() != 0
    }

    #[doc="Sets the WCEN6 field."]
    #[inline] pub fn set_wcen6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit enables slot 6 for ADC conversions."]
    #[inline] pub fn slen6(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEN6 != 0"]
    #[inline] pub fn test_slen6(&self) -> bool {
        self.slen6() != 0
    }

    #[doc="Sets the SLEN6 field."]
    #[inline] pub fn set_slen6<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sl6cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sl6cfg(other)
    }
}

impl ::core::fmt::Display for Sl6cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sl6cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adsel6() != 0 { try!(write!(f, " adsel6=0x{:x}", self.adsel6()))}
        if self.prmode6() != 0 { try!(write!(f, " prmode6=0x{:x}", self.prmode6()))}
        if self.chsel6() != 0 { try!(write!(f, " chsel6=0x{:x}", self.chsel6()))}
        if self.wcen6() != 0 { try!(write!(f, " wcen6"))}
        if self.slen6() != 0 { try!(write!(f, " slen6"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slot 7 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sl7cfg(pub u32);
impl Sl7cfg {
    #[doc="Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline] pub fn adsel7(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if ADSEL7 != 0"]
    #[inline] pub fn test_adsel7(&self) -> bool {
        self.adsel7() != 0
    }

    #[doc="Sets the ADSEL7 field."]
    #[inline] pub fn set_adsel7<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Set the Precision Mode For Slot."]
    #[inline] pub fn prmode7(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PRMODE7 != 0"]
    #[inline] pub fn test_prmode7(&self) -> bool {
        self.prmode7() != 0
    }

    #[doc="Sets the PRMODE7 field."]
    #[inline] pub fn set_prmode7<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Select one of the 14 channel inputs for this slot."]
    #[inline] pub fn chsel7(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CHSEL7 != 0"]
    #[inline] pub fn test_chsel7(&self) -> bool {
        self.chsel7() != 0
    }

    #[doc="Sets the CHSEL7 field."]
    #[inline] pub fn set_chsel7<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit enables the window compare function for slot 7."]
    #[inline] pub fn wcen7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WCEN7 != 0"]
    #[inline] pub fn test_wcen7(&self) -> bool {
        self.wcen7() != 0
    }

    #[doc="Sets the WCEN7 field."]
    #[inline] pub fn set_wcen7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit enables slot 7 for ADC conversions."]
    #[inline] pub fn slen7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SLEN7 != 0"]
    #[inline] pub fn test_slen7(&self) -> bool {
        self.slen7() != 0
    }

    #[doc="Sets the SLEN7 field."]
    #[inline] pub fn set_slen7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sl7cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sl7cfg(other)
    }
}

impl ::core::fmt::Display for Sl7cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sl7cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adsel7() != 0 { try!(write!(f, " adsel7=0x{:x}", self.adsel7()))}
        if self.prmode7() != 0 { try!(write!(f, " prmode7=0x{:x}", self.prmode7()))}
        if self.chsel7() != 0 { try!(write!(f, " chsel7=0x{:x}", self.chsel7()))}
        if self.wcen7() != 0 { try!(write!(f, " wcen7"))}
        if self.slen7() != 0 { try!(write!(f, " slen7"))}
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


