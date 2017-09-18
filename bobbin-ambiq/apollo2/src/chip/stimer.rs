//! System Timer
#[allow(unused_imports)] use bobbin_common::*;

periph!(STIMER, Stimer, 0x40008000);

#[doc="System Timer"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stimer(pub usize);
impl Stimer {
    #[doc="Get the *mut pointer for the STCFG register."]
    #[inline] pub fn stcfg_mut(&self) -> *mut Stcfg { 
        (self.0 + 0x100) as *mut Stcfg
    }

    #[doc="Get the *const pointer for the STCFG register."]
    #[inline] pub fn stcfg_ptr(&self) -> *const Stcfg { 
           self.stcfg_mut()
    }

    #[doc="Read the STCFG register."]
    #[inline] pub fn stcfg(&self) -> Stcfg { 
        unsafe {
            read_volatile(self.stcfg_ptr())
        }
    }

    #[doc="Write the STCFG register."]
    #[inline] pub fn set_stcfg<F: FnOnce(Stcfg) -> Stcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stcfg_mut(), f(Stcfg(0)));
        }
        self
    }

    #[doc="Modify the STCFG register."]
    #[inline] pub fn with_stcfg<F: FnOnce(Stcfg) -> Stcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stcfg_mut(), f(self.stcfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STTMR register."]
    #[inline] pub fn sttmr_mut(&self) -> *mut Sttmr { 
        (self.0 + 0x104) as *mut Sttmr
    }

    #[doc="Get the *const pointer for the STTMR register."]
    #[inline] pub fn sttmr_ptr(&self) -> *const Sttmr { 
           self.sttmr_mut()
    }

    #[doc="Read the STTMR register."]
    #[inline] pub fn sttmr(&self) -> Sttmr { 
        unsafe {
            read_volatile(self.sttmr_ptr())
        }
    }

    #[doc="Write the STTMR register."]
    #[inline] pub fn set_sttmr<F: FnOnce(Sttmr) -> Sttmr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sttmr_mut(), f(Sttmr(0)));
        }
        self
    }

    #[doc="Modify the STTMR register."]
    #[inline] pub fn with_sttmr<F: FnOnce(Sttmr) -> Sttmr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sttmr_mut(), f(self.sttmr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CAPTURE_CONTROL register."]
    #[inline] pub fn capture_control_mut(&self) -> *mut CaptureControl { 
        (self.0 + 0x108) as *mut CaptureControl
    }

    #[doc="Get the *const pointer for the CAPTURE_CONTROL register."]
    #[inline] pub fn capture_control_ptr(&self) -> *const CaptureControl { 
           self.capture_control_mut()
    }

    #[doc="Read the CAPTURE_CONTROL register."]
    #[inline] pub fn capture_control(&self) -> CaptureControl { 
        unsafe {
            read_volatile(self.capture_control_ptr())
        }
    }

    #[doc="Write the CAPTURE_CONTROL register."]
    #[inline] pub fn set_capture_control<F: FnOnce(CaptureControl) -> CaptureControl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.capture_control_mut(), f(CaptureControl(0)));
        }
        self
    }

    #[doc="Modify the CAPTURE_CONTROL register."]
    #[inline] pub fn with_capture_control<F: FnOnce(CaptureControl) -> CaptureControl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.capture_control_mut(), f(self.capture_control()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCMPR register."]
    #[inline] pub fn scmpr_mut<I: Into<bits::R8>>(&self, index: I) -> *mut Scmpr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x110 + (index << 2)) as *mut Scmpr
    }

    #[doc="Get the *const pointer for the SCMPR register."]
    #[inline] pub fn scmpr_ptr<I: Into<bits::R8>>(&self, index: I) -> *const Scmpr { 
           self.scmpr_mut(index)
    }

    #[doc="Read the SCMPR register."]
    #[inline] pub fn scmpr<I: Into<bits::R8>>(&self, index: I) -> Scmpr { 
        unsafe {
            read_volatile(self.scmpr_ptr(index))
        }
    }

    #[doc="Write the SCMPR register."]
    #[inline] pub fn set_scmpr<I: Into<bits::R8>, F: FnOnce(Scmpr) -> Scmpr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr_mut(index), f(Scmpr(0)));
        }
        self
    }

    #[doc="Modify the SCMPR register."]
    #[inline] pub fn with_scmpr<I: Into<bits::R8> + Copy, F: FnOnce(Scmpr) -> Scmpr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.scmpr_mut(index), f(self.scmpr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCAPT register."]
    #[inline] pub fn scapt_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Scapt { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1e0 + (index << 2)) as *mut Scapt
    }

    #[doc="Get the *const pointer for the SCAPT register."]
    #[inline] pub fn scapt_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Scapt { 
           self.scapt_mut(index)
    }

    #[doc="Read the SCAPT register."]
    #[inline] pub fn scapt<I: Into<bits::R4>>(&self, index: I) -> Scapt { 
        unsafe {
            read_volatile(self.scapt_ptr(index))
        }
    }

    #[doc="Write the SCAPT register."]
    #[inline] pub fn set_scapt<I: Into<bits::R4>, F: FnOnce(Scapt) -> Scapt>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.scapt_mut(index), f(Scapt(0)));
        }
        self
    }

    #[doc="Modify the SCAPT register."]
    #[inline] pub fn with_scapt<I: Into<bits::R4> + Copy, F: FnOnce(Scapt) -> Scapt>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.scapt_mut(index), f(self.scapt(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SNVR register."]
    #[inline] pub fn snvr_mut<I: Into<bits::R3>>(&self, index: I) -> *mut Snvr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1f0 + (index << 2)) as *mut Snvr
    }

    #[doc="Get the *const pointer for the SNVR register."]
    #[inline] pub fn snvr_ptr<I: Into<bits::R3>>(&self, index: I) -> *const Snvr { 
           self.snvr_mut(index)
    }

    #[doc="Read the SNVR register."]
    #[inline] pub fn snvr<I: Into<bits::R3>>(&self, index: I) -> Snvr { 
        unsafe {
            read_volatile(self.snvr_ptr(index))
        }
    }

    #[doc="Write the SNVR register."]
    #[inline] pub fn set_snvr<I: Into<bits::R3>, F: FnOnce(Snvr) -> Snvr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr_mut(index), f(Snvr(0)));
        }
        self
    }

    #[doc="Modify the SNVR register."]
    #[inline] pub fn with_snvr<I: Into<bits::R3> + Copy, F: FnOnce(Snvr) -> Snvr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr_mut(index), f(self.snvr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SNVR1 register."]
    #[inline] pub fn snvr1_mut(&self) -> *mut Snvr1 { 
        (self.0 + 0x1f4) as *mut Snvr1
    }

    #[doc="Get the *const pointer for the SNVR1 register."]
    #[inline] pub fn snvr1_ptr(&self) -> *const Snvr1 { 
           self.snvr1_mut()
    }

    #[doc="Read the SNVR1 register."]
    #[inline] pub fn snvr1(&self) -> Snvr1 { 
        unsafe {
            read_volatile(self.snvr1_ptr())
        }
    }

    #[doc="Write the SNVR1 register."]
    #[inline] pub fn set_snvr1<F: FnOnce(Snvr1) -> Snvr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr1_mut(), f(Snvr1(0)));
        }
        self
    }

    #[doc="Modify the SNVR1 register."]
    #[inline] pub fn with_snvr1<F: FnOnce(Snvr1) -> Snvr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr1_mut(), f(self.snvr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SNVR2 register."]
    #[inline] pub fn snvr2_mut(&self) -> *mut Snvr2 { 
        (self.0 + 0x1f8) as *mut Snvr2
    }

    #[doc="Get the *const pointer for the SNVR2 register."]
    #[inline] pub fn snvr2_ptr(&self) -> *const Snvr2 { 
           self.snvr2_mut()
    }

    #[doc="Read the SNVR2 register."]
    #[inline] pub fn snvr2(&self) -> Snvr2 { 
        unsafe {
            read_volatile(self.snvr2_ptr())
        }
    }

    #[doc="Write the SNVR2 register."]
    #[inline] pub fn set_snvr2<F: FnOnce(Snvr2) -> Snvr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr2_mut(), f(Snvr2(0)));
        }
        self
    }

    #[doc="Modify the SNVR2 register."]
    #[inline] pub fn with_snvr2<F: FnOnce(Snvr2) -> Snvr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.snvr2_mut(), f(self.snvr2()));
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

    #[doc="Get the *mut pointer for the STMINTEN register."]
    #[inline] pub fn stminten_mut(&self) -> *mut Stminten { 
        (self.0 + 0x300) as *mut Stminten
    }

    #[doc="Get the *const pointer for the STMINTEN register."]
    #[inline] pub fn stminten_ptr(&self) -> *const Stminten { 
           self.stminten_mut()
    }

    #[doc="Read the STMINTEN register."]
    #[inline] pub fn stminten(&self) -> Stminten { 
        unsafe {
            read_volatile(self.stminten_ptr())
        }
    }

    #[doc="Write the STMINTEN register."]
    #[inline] pub fn set_stminten<F: FnOnce(Stminten) -> Stminten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stminten_mut(), f(Stminten(0)));
        }
        self
    }

    #[doc="Modify the STMINTEN register."]
    #[inline] pub fn with_stminten<F: FnOnce(Stminten) -> Stminten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stminten_mut(), f(self.stminten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STMINTSTAT register."]
    #[inline] pub fn stmintstat_mut(&self) -> *mut Stmintstat { 
        (self.0 + 0x304) as *mut Stmintstat
    }

    #[doc="Get the *const pointer for the STMINTSTAT register."]
    #[inline] pub fn stmintstat_ptr(&self) -> *const Stmintstat { 
           self.stmintstat_mut()
    }

    #[doc="Read the STMINTSTAT register."]
    #[inline] pub fn stmintstat(&self) -> Stmintstat { 
        unsafe {
            read_volatile(self.stmintstat_ptr())
        }
    }

    #[doc="Write the STMINTSTAT register."]
    #[inline] pub fn set_stmintstat<F: FnOnce(Stmintstat) -> Stmintstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintstat_mut(), f(Stmintstat(0)));
        }
        self
    }

    #[doc="Modify the STMINTSTAT register."]
    #[inline] pub fn with_stmintstat<F: FnOnce(Stmintstat) -> Stmintstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintstat_mut(), f(self.stmintstat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STMINTCLR register."]
    #[inline] pub fn stmintclr_mut(&self) -> *mut Stmintclr { 
        (self.0 + 0x308) as *mut Stmintclr
    }

    #[doc="Get the *const pointer for the STMINTCLR register."]
    #[inline] pub fn stmintclr_ptr(&self) -> *const Stmintclr { 
           self.stmintclr_mut()
    }

    #[doc="Read the STMINTCLR register."]
    #[inline] pub fn stmintclr(&self) -> Stmintclr { 
        unsafe {
            read_volatile(self.stmintclr_ptr())
        }
    }

    #[doc="Write the STMINTCLR register."]
    #[inline] pub fn set_stmintclr<F: FnOnce(Stmintclr) -> Stmintclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintclr_mut(), f(Stmintclr(0)));
        }
        self
    }

    #[doc="Modify the STMINTCLR register."]
    #[inline] pub fn with_stmintclr<F: FnOnce(Stmintclr) -> Stmintclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintclr_mut(), f(self.stmintclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STMINTSET register."]
    #[inline] pub fn stmintset_mut(&self) -> *mut Stmintset { 
        (self.0 + 0x30c) as *mut Stmintset
    }

    #[doc="Get the *const pointer for the STMINTSET register."]
    #[inline] pub fn stmintset_ptr(&self) -> *const Stmintset { 
           self.stmintset_mut()
    }

    #[doc="Read the STMINTSET register."]
    #[inline] pub fn stmintset(&self) -> Stmintset { 
        unsafe {
            read_volatile(self.stmintset_ptr())
        }
    }

    #[doc="Write the STMINTSET register."]
    #[inline] pub fn set_stmintset<F: FnOnce(Stmintset) -> Stmintset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintset_mut(), f(Stmintset(0)));
        }
        self
    }

    #[doc="Modify the STMINTSET register."]
    #[inline] pub fn with_stmintset<F: FnOnce(Stmintset) -> Stmintset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.stmintset_mut(), f(self.stmintset()));
        }
        self
    }

}

#[doc="Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stcfg(pub u32);
impl Stcfg {
    #[doc="Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline] pub fn freeze(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if FREEZE != 0"]
    #[inline] pub fn test_freeze(&self) -> bool {
        self.freeze() != 0
    }

    #[doc="Sets the FREEZE field."]
    #[inline] pub fn set_freeze<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Set this bit to one to clear the System Timer register. If this bit is set to \'1\', the system timer register will stay cleared. It needs to be set to \'0\' for the system timer to start running."]
    #[inline] pub fn clear(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CLEAR != 0"]
    #[inline] pub fn test_clear(&self) -> bool {
        self.clear() != 0
    }

    #[doc="Sets the CLEAR field."]
    #[inline] pub fn set_clear<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline] pub fn compare_en<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if COMPARE_EN != 0"]
    #[inline] pub fn test_compare_en<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.compare_en(index) != 0
    }

    #[doc="Sets the COMPARE_EN field."]
    #[inline] pub fn set_compare_en<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline] pub fn clksel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CLKSEL != 0"]
    #[inline] pub fn test_clksel(&self) -> bool {
        self.clksel() != 0
    }

    #[doc="Sets the CLKSEL field."]
    #[inline] pub fn set_clksel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Stcfg(other)
    }
}

impl ::core::fmt::Display for Stcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.freeze() != 0 { try!(write!(f, " freeze"))}
        if self.clear() != 0 { try!(write!(f, " clear"))}
        if self.compare_en(0) != 0 { try!(write!(f, " compare_en[0]"))}
        if self.compare_en(1) != 0 { try!(write!(f, " compare_en[1]"))}
        if self.compare_en(2) != 0 { try!(write!(f, " compare_en[2]"))}
        if self.compare_en(3) != 0 { try!(write!(f, " compare_en[3]"))}
        if self.compare_en(4) != 0 { try!(write!(f, " compare_en[4]"))}
        if self.compare_en(5) != 0 { try!(write!(f, " compare_en[5]"))}
        if self.compare_en(6) != 0 { try!(write!(f, " compare_en[6]"))}
        if self.compare_en(7) != 0 { try!(write!(f, " compare_en[7]"))}
        if self.clksel() != 0 { try!(write!(f, " clksel=0x{:x}", self.clksel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Timer Count Register (Real Time Counter)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sttmr(pub u32);
impl Sttmr {
    #[doc="Value of the 32-bit counter as it ticks over."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sttmr {
    #[inline]
    fn from(other: u32) -> Self {
         Sttmr(other)
    }
}

impl ::core::fmt::Display for Sttmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sttmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CaptureControl(pub u32);
impl CaptureControl {
    #[doc="Selects whether capture is enabled for the specified capture register."]
    #[inline] pub fn capture<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CAPTURE != 0"]
    #[inline] pub fn test_capture<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.capture(index) != 0
    }

    #[doc="Sets the CAPTURE field."]
    #[inline] pub fn set_capture<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for CaptureControl {
    #[inline]
    fn from(other: u32) -> Self {
         CaptureControl(other)
    }
}

impl ::core::fmt::Display for CaptureControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CaptureControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.capture(0) != 0 { try!(write!(f, " capture[0]"))}
        if self.capture(1) != 0 { try!(write!(f, " capture[1]"))}
        if self.capture(2) != 0 { try!(write!(f, " capture[2]"))}
        if self.capture(3) != 0 { try!(write!(f, " capture[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scmpr(pub u32);
impl Scmpr {
    #[doc="Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_n_EN bit in the REG_CTIMER_STCGF register."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scmpr {
    #[inline]
    fn from(other: u32) -> Self {
         Scmpr(other)
    }
}

impl ::core::fmt::Display for Scmpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scmpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture Register n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scapt(pub u32);
impl Scapt {
    #[doc="Whenever the event is detected, the value in the COUNTER is copied into this register and the corresponding interrupt status bit is set."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scapt {
    #[inline]
    fn from(other: u32) -> Self {
         Scapt(other)
    }
}

impl ::core::fmt::Display for Scapt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scapt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Timer NVRAM_A Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Snvr(pub u32);
impl Snvr {
    #[doc="Value of the 32-bit counter as it ticks over."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Snvr {
    #[inline]
    fn from(other: u32) -> Self {
         Snvr(other)
    }
}

impl ::core::fmt::Display for Snvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Snvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Timer NVRAM_B Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Snvr1(pub u32);
impl Snvr1 {
    #[doc="Value of the 32-bit counter as it ticks over."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Snvr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Snvr1(other)
    }
}

impl ::core::fmt::Display for Snvr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Snvr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Timer NVRAM_C Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Snvr2(pub u32);
impl Snvr2 {
    #[doc="Value of the 32-bit counter as it ticks over."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Snvr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Snvr2(other)
    }
}

impl ::core::fmt::Display for Snvr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Snvr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Interrupts: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="Counter/Timer B3 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTMRB3C1INT != 0"]
    #[inline] pub fn test_ctmrb3c1int(&self) -> bool {
        self.ctmrb3c1int() != 0
    }

    #[doc="Sets the CTMRB3C1INT field."]
    #[inline] pub fn set_ctmrb3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR1."]
    #[inline] pub fn ctmra3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CTMRA3C1INT != 0"]
    #[inline] pub fn test_ctmra3c1int(&self) -> bool {
        self.ctmra3c1int() != 0
    }

    #[doc="Sets the CTMRA3C1INT field."]
    #[inline] pub fn set_ctmra3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CTMRB2C1INT != 0"]
    #[inline] pub fn test_ctmrb2c1int(&self) -> bool {
        self.ctmrb2c1int() != 0
    }

    #[doc="Sets the CTMRB2C1INT field."]
    #[inline] pub fn set_ctmrb2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR1."]
    #[inline] pub fn ctmra2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CTMRA2C1INT != 0"]
    #[inline] pub fn test_ctmra2c1int(&self) -> bool {
        self.ctmra2c1int() != 0
    }

    #[doc="Sets the CTMRA2C1INT field."]
    #[inline] pub fn set_ctmra2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CTMRB1C1INT != 0"]
    #[inline] pub fn test_ctmrb1c1int(&self) -> bool {
        self.ctmrb1c1int() != 0
    }

    #[doc="Sets the CTMRB1C1INT field."]
    #[inline] pub fn set_ctmrb1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR1."]
    #[inline] pub fn ctmra1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTMRA1C1INT != 0"]
    #[inline] pub fn test_ctmra1c1int(&self) -> bool {
        self.ctmra1c1int() != 0
    }

    #[doc="Sets the CTMRA1C1INT field."]
    #[inline] pub fn set_ctmra1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTMRB0C1INT != 0"]
    #[inline] pub fn test_ctmrb0c1int(&self) -> bool {
        self.ctmrb0c1int() != 0
    }

    #[doc="Sets the CTMRB0C1INT field."]
    #[inline] pub fn set_ctmrb0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR1."]
    #[inline] pub fn ctmra0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CTMRA0C1INT != 0"]
    #[inline] pub fn test_ctmra0c1int(&self) -> bool {
        self.ctmra0c1int() != 0
    }

    #[doc="Sets the CTMRA0C1INT field."]
    #[inline] pub fn set_ctmra0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Counter/Timer B3 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTMRB3C0INT != 0"]
    #[inline] pub fn test_ctmrb3c0int(&self) -> bool {
        self.ctmrb3c0int() != 0
    }

    #[doc="Sets the CTMRB3C0INT field."]
    #[inline] pub fn set_ctmrb3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR0."]
    #[inline] pub fn ctmra3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CTMRA3C0INT != 0"]
    #[inline] pub fn test_ctmra3c0int(&self) -> bool {
        self.ctmra3c0int() != 0
    }

    #[doc="Sets the CTMRA3C0INT field."]
    #[inline] pub fn set_ctmra3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CTMRB2C0INT != 0"]
    #[inline] pub fn test_ctmrb2c0int(&self) -> bool {
        self.ctmrb2c0int() != 0
    }

    #[doc="Sets the CTMRB2C0INT field."]
    #[inline] pub fn set_ctmrb2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR0."]
    #[inline] pub fn ctmra2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTMRA2C0INT != 0"]
    #[inline] pub fn test_ctmra2c0int(&self) -> bool {
        self.ctmra2c0int() != 0
    }

    #[doc="Sets the CTMRA2C0INT field."]
    #[inline] pub fn set_ctmra2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTMRB1C0INT != 0"]
    #[inline] pub fn test_ctmrb1c0int(&self) -> bool {
        self.ctmrb1c0int() != 0
    }

    #[doc="Sets the CTMRB1C0INT field."]
    #[inline] pub fn set_ctmrb1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR0."]
    #[inline] pub fn ctmra1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTMRA1C0INT != 0"]
    #[inline] pub fn test_ctmra1c0int(&self) -> bool {
        self.ctmra1c0int() != 0
    }

    #[doc="Sets the CTMRA1C0INT field."]
    #[inline] pub fn set_ctmra1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTMRB0C0INT != 0"]
    #[inline] pub fn test_ctmrb0c0int(&self) -> bool {
        self.ctmrb0c0int() != 0
    }

    #[doc="Sets the CTMRB0C0INT field."]
    #[inline] pub fn set_ctmrb0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR0."]
    #[inline] pub fn ctmra0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTMRA0C0INT != 0"]
    #[inline] pub fn test_ctmra0c0int(&self) -> bool {
        self.ctmra0c0int() != 0
    }

    #[doc="Sets the CTMRA0C0INT field."]
    #[inline] pub fn set_ctmra0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.ctmrb3c1int() != 0 { try!(write!(f, " ctmrb3c1int"))}
        if self.ctmra3c1int() != 0 { try!(write!(f, " ctmra3c1int"))}
        if self.ctmrb2c1int() != 0 { try!(write!(f, " ctmrb2c1int"))}
        if self.ctmra2c1int() != 0 { try!(write!(f, " ctmra2c1int"))}
        if self.ctmrb1c1int() != 0 { try!(write!(f, " ctmrb1c1int"))}
        if self.ctmra1c1int() != 0 { try!(write!(f, " ctmra1c1int"))}
        if self.ctmrb0c1int() != 0 { try!(write!(f, " ctmrb0c1int"))}
        if self.ctmra0c1int() != 0 { try!(write!(f, " ctmra0c1int"))}
        if self.ctmrb3c0int() != 0 { try!(write!(f, " ctmrb3c0int"))}
        if self.ctmra3c0int() != 0 { try!(write!(f, " ctmra3c0int"))}
        if self.ctmrb2c0int() != 0 { try!(write!(f, " ctmrb2c0int"))}
        if self.ctmra2c0int() != 0 { try!(write!(f, " ctmra2c0int"))}
        if self.ctmrb1c0int() != 0 { try!(write!(f, " ctmrb1c0int"))}
        if self.ctmra1c0int() != 0 { try!(write!(f, " ctmra1c0int"))}
        if self.ctmrb0c0int() != 0 { try!(write!(f, " ctmrb0c0int"))}
        if self.ctmra0c0int() != 0 { try!(write!(f, " ctmra0c0int"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Interrupts: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="Counter/Timer B3 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTMRB3C1INT != 0"]
    #[inline] pub fn test_ctmrb3c1int(&self) -> bool {
        self.ctmrb3c1int() != 0
    }

    #[doc="Sets the CTMRB3C1INT field."]
    #[inline] pub fn set_ctmrb3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR1."]
    #[inline] pub fn ctmra3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CTMRA3C1INT != 0"]
    #[inline] pub fn test_ctmra3c1int(&self) -> bool {
        self.ctmra3c1int() != 0
    }

    #[doc="Sets the CTMRA3C1INT field."]
    #[inline] pub fn set_ctmra3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CTMRB2C1INT != 0"]
    #[inline] pub fn test_ctmrb2c1int(&self) -> bool {
        self.ctmrb2c1int() != 0
    }

    #[doc="Sets the CTMRB2C1INT field."]
    #[inline] pub fn set_ctmrb2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR1."]
    #[inline] pub fn ctmra2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CTMRA2C1INT != 0"]
    #[inline] pub fn test_ctmra2c1int(&self) -> bool {
        self.ctmra2c1int() != 0
    }

    #[doc="Sets the CTMRA2C1INT field."]
    #[inline] pub fn set_ctmra2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CTMRB1C1INT != 0"]
    #[inline] pub fn test_ctmrb1c1int(&self) -> bool {
        self.ctmrb1c1int() != 0
    }

    #[doc="Sets the CTMRB1C1INT field."]
    #[inline] pub fn set_ctmrb1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR1."]
    #[inline] pub fn ctmra1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTMRA1C1INT != 0"]
    #[inline] pub fn test_ctmra1c1int(&self) -> bool {
        self.ctmra1c1int() != 0
    }

    #[doc="Sets the CTMRA1C1INT field."]
    #[inline] pub fn set_ctmra1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTMRB0C1INT != 0"]
    #[inline] pub fn test_ctmrb0c1int(&self) -> bool {
        self.ctmrb0c1int() != 0
    }

    #[doc="Sets the CTMRB0C1INT field."]
    #[inline] pub fn set_ctmrb0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR1."]
    #[inline] pub fn ctmra0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CTMRA0C1INT != 0"]
    #[inline] pub fn test_ctmra0c1int(&self) -> bool {
        self.ctmra0c1int() != 0
    }

    #[doc="Sets the CTMRA0C1INT field."]
    #[inline] pub fn set_ctmra0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Counter/Timer B3 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTMRB3C0INT != 0"]
    #[inline] pub fn test_ctmrb3c0int(&self) -> bool {
        self.ctmrb3c0int() != 0
    }

    #[doc="Sets the CTMRB3C0INT field."]
    #[inline] pub fn set_ctmrb3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR0."]
    #[inline] pub fn ctmra3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CTMRA3C0INT != 0"]
    #[inline] pub fn test_ctmra3c0int(&self) -> bool {
        self.ctmra3c0int() != 0
    }

    #[doc="Sets the CTMRA3C0INT field."]
    #[inline] pub fn set_ctmra3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CTMRB2C0INT != 0"]
    #[inline] pub fn test_ctmrb2c0int(&self) -> bool {
        self.ctmrb2c0int() != 0
    }

    #[doc="Sets the CTMRB2C0INT field."]
    #[inline] pub fn set_ctmrb2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR0."]
    #[inline] pub fn ctmra2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTMRA2C0INT != 0"]
    #[inline] pub fn test_ctmra2c0int(&self) -> bool {
        self.ctmra2c0int() != 0
    }

    #[doc="Sets the CTMRA2C0INT field."]
    #[inline] pub fn set_ctmra2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTMRB1C0INT != 0"]
    #[inline] pub fn test_ctmrb1c0int(&self) -> bool {
        self.ctmrb1c0int() != 0
    }

    #[doc="Sets the CTMRB1C0INT field."]
    #[inline] pub fn set_ctmrb1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR0."]
    #[inline] pub fn ctmra1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTMRA1C0INT != 0"]
    #[inline] pub fn test_ctmra1c0int(&self) -> bool {
        self.ctmra1c0int() != 0
    }

    #[doc="Sets the CTMRA1C0INT field."]
    #[inline] pub fn set_ctmra1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTMRB0C0INT != 0"]
    #[inline] pub fn test_ctmrb0c0int(&self) -> bool {
        self.ctmrb0c0int() != 0
    }

    #[doc="Sets the CTMRB0C0INT field."]
    #[inline] pub fn set_ctmrb0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR0."]
    #[inline] pub fn ctmra0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTMRA0C0INT != 0"]
    #[inline] pub fn test_ctmra0c0int(&self) -> bool {
        self.ctmra0c0int() != 0
    }

    #[doc="Sets the CTMRA0C0INT field."]
    #[inline] pub fn set_ctmra0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.ctmrb3c1int() != 0 { try!(write!(f, " ctmrb3c1int"))}
        if self.ctmra3c1int() != 0 { try!(write!(f, " ctmra3c1int"))}
        if self.ctmrb2c1int() != 0 { try!(write!(f, " ctmrb2c1int"))}
        if self.ctmra2c1int() != 0 { try!(write!(f, " ctmra2c1int"))}
        if self.ctmrb1c1int() != 0 { try!(write!(f, " ctmrb1c1int"))}
        if self.ctmra1c1int() != 0 { try!(write!(f, " ctmra1c1int"))}
        if self.ctmrb0c1int() != 0 { try!(write!(f, " ctmrb0c1int"))}
        if self.ctmra0c1int() != 0 { try!(write!(f, " ctmra0c1int"))}
        if self.ctmrb3c0int() != 0 { try!(write!(f, " ctmrb3c0int"))}
        if self.ctmra3c0int() != 0 { try!(write!(f, " ctmra3c0int"))}
        if self.ctmrb2c0int() != 0 { try!(write!(f, " ctmrb2c0int"))}
        if self.ctmra2c0int() != 0 { try!(write!(f, " ctmra2c0int"))}
        if self.ctmrb1c0int() != 0 { try!(write!(f, " ctmrb1c0int"))}
        if self.ctmra1c0int() != 0 { try!(write!(f, " ctmra1c0int"))}
        if self.ctmrb0c0int() != 0 { try!(write!(f, " ctmrb0c0int"))}
        if self.ctmra0c0int() != 0 { try!(write!(f, " ctmra0c0int"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Interrupts: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="Counter/Timer B3 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTMRB3C1INT != 0"]
    #[inline] pub fn test_ctmrb3c1int(&self) -> bool {
        self.ctmrb3c1int() != 0
    }

    #[doc="Sets the CTMRB3C1INT field."]
    #[inline] pub fn set_ctmrb3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR1."]
    #[inline] pub fn ctmra3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CTMRA3C1INT != 0"]
    #[inline] pub fn test_ctmra3c1int(&self) -> bool {
        self.ctmra3c1int() != 0
    }

    #[doc="Sets the CTMRA3C1INT field."]
    #[inline] pub fn set_ctmra3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CTMRB2C1INT != 0"]
    #[inline] pub fn test_ctmrb2c1int(&self) -> bool {
        self.ctmrb2c1int() != 0
    }

    #[doc="Sets the CTMRB2C1INT field."]
    #[inline] pub fn set_ctmrb2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR1."]
    #[inline] pub fn ctmra2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CTMRA2C1INT != 0"]
    #[inline] pub fn test_ctmra2c1int(&self) -> bool {
        self.ctmra2c1int() != 0
    }

    #[doc="Sets the CTMRA2C1INT field."]
    #[inline] pub fn set_ctmra2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CTMRB1C1INT != 0"]
    #[inline] pub fn test_ctmrb1c1int(&self) -> bool {
        self.ctmrb1c1int() != 0
    }

    #[doc="Sets the CTMRB1C1INT field."]
    #[inline] pub fn set_ctmrb1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR1."]
    #[inline] pub fn ctmra1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTMRA1C1INT != 0"]
    #[inline] pub fn test_ctmra1c1int(&self) -> bool {
        self.ctmra1c1int() != 0
    }

    #[doc="Sets the CTMRA1C1INT field."]
    #[inline] pub fn set_ctmra1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTMRB0C1INT != 0"]
    #[inline] pub fn test_ctmrb0c1int(&self) -> bool {
        self.ctmrb0c1int() != 0
    }

    #[doc="Sets the CTMRB0C1INT field."]
    #[inline] pub fn set_ctmrb0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR1."]
    #[inline] pub fn ctmra0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CTMRA0C1INT != 0"]
    #[inline] pub fn test_ctmra0c1int(&self) -> bool {
        self.ctmra0c1int() != 0
    }

    #[doc="Sets the CTMRA0C1INT field."]
    #[inline] pub fn set_ctmra0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Counter/Timer B3 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTMRB3C0INT != 0"]
    #[inline] pub fn test_ctmrb3c0int(&self) -> bool {
        self.ctmrb3c0int() != 0
    }

    #[doc="Sets the CTMRB3C0INT field."]
    #[inline] pub fn set_ctmrb3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR0."]
    #[inline] pub fn ctmra3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CTMRA3C0INT != 0"]
    #[inline] pub fn test_ctmra3c0int(&self) -> bool {
        self.ctmra3c0int() != 0
    }

    #[doc="Sets the CTMRA3C0INT field."]
    #[inline] pub fn set_ctmra3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CTMRB2C0INT != 0"]
    #[inline] pub fn test_ctmrb2c0int(&self) -> bool {
        self.ctmrb2c0int() != 0
    }

    #[doc="Sets the CTMRB2C0INT field."]
    #[inline] pub fn set_ctmrb2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR0."]
    #[inline] pub fn ctmra2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTMRA2C0INT != 0"]
    #[inline] pub fn test_ctmra2c0int(&self) -> bool {
        self.ctmra2c0int() != 0
    }

    #[doc="Sets the CTMRA2C0INT field."]
    #[inline] pub fn set_ctmra2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTMRB1C0INT != 0"]
    #[inline] pub fn test_ctmrb1c0int(&self) -> bool {
        self.ctmrb1c0int() != 0
    }

    #[doc="Sets the CTMRB1C0INT field."]
    #[inline] pub fn set_ctmrb1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR0."]
    #[inline] pub fn ctmra1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTMRA1C0INT != 0"]
    #[inline] pub fn test_ctmra1c0int(&self) -> bool {
        self.ctmra1c0int() != 0
    }

    #[doc="Sets the CTMRA1C0INT field."]
    #[inline] pub fn set_ctmra1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTMRB0C0INT != 0"]
    #[inline] pub fn test_ctmrb0c0int(&self) -> bool {
        self.ctmrb0c0int() != 0
    }

    #[doc="Sets the CTMRB0C0INT field."]
    #[inline] pub fn set_ctmrb0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR0."]
    #[inline] pub fn ctmra0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTMRA0C0INT != 0"]
    #[inline] pub fn test_ctmra0c0int(&self) -> bool {
        self.ctmra0c0int() != 0
    }

    #[doc="Sets the CTMRA0C0INT field."]
    #[inline] pub fn set_ctmra0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.ctmrb3c1int() != 0 { try!(write!(f, " ctmrb3c1int"))}
        if self.ctmra3c1int() != 0 { try!(write!(f, " ctmra3c1int"))}
        if self.ctmrb2c1int() != 0 { try!(write!(f, " ctmrb2c1int"))}
        if self.ctmra2c1int() != 0 { try!(write!(f, " ctmra2c1int"))}
        if self.ctmrb1c1int() != 0 { try!(write!(f, " ctmrb1c1int"))}
        if self.ctmra1c1int() != 0 { try!(write!(f, " ctmra1c1int"))}
        if self.ctmrb0c1int() != 0 { try!(write!(f, " ctmrb0c1int"))}
        if self.ctmra0c1int() != 0 { try!(write!(f, " ctmra0c1int"))}
        if self.ctmrb3c0int() != 0 { try!(write!(f, " ctmrb3c0int"))}
        if self.ctmra3c0int() != 0 { try!(write!(f, " ctmra3c0int"))}
        if self.ctmrb2c0int() != 0 { try!(write!(f, " ctmrb2c0int"))}
        if self.ctmra2c0int() != 0 { try!(write!(f, " ctmra2c0int"))}
        if self.ctmrb1c0int() != 0 { try!(write!(f, " ctmrb1c0int"))}
        if self.ctmra1c0int() != 0 { try!(write!(f, " ctmra1c0int"))}
        if self.ctmrb0c0int() != 0 { try!(write!(f, " ctmrb0c0int"))}
        if self.ctmra0c0int() != 0 { try!(write!(f, " ctmra0c0int"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Interrupts: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc="Counter/Timer B3 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTMRB3C1INT != 0"]
    #[inline] pub fn test_ctmrb3c1int(&self) -> bool {
        self.ctmrb3c1int() != 0
    }

    #[doc="Sets the CTMRB3C1INT field."]
    #[inline] pub fn set_ctmrb3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR1."]
    #[inline] pub fn ctmra3c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CTMRA3C1INT != 0"]
    #[inline] pub fn test_ctmra3c1int(&self) -> bool {
        self.ctmra3c1int() != 0
    }

    #[doc="Sets the CTMRA3C1INT field."]
    #[inline] pub fn set_ctmra3c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CTMRB2C1INT != 0"]
    #[inline] pub fn test_ctmrb2c1int(&self) -> bool {
        self.ctmrb2c1int() != 0
    }

    #[doc="Sets the CTMRB2C1INT field."]
    #[inline] pub fn set_ctmrb2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR1."]
    #[inline] pub fn ctmra2c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CTMRA2C1INT != 0"]
    #[inline] pub fn test_ctmra2c1int(&self) -> bool {
        self.ctmra2c1int() != 0
    }

    #[doc="Sets the CTMRA2C1INT field."]
    #[inline] pub fn set_ctmra2c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CTMRB1C1INT != 0"]
    #[inline] pub fn test_ctmrb1c1int(&self) -> bool {
        self.ctmrb1c1int() != 0
    }

    #[doc="Sets the CTMRB1C1INT field."]
    #[inline] pub fn set_ctmrb1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR1."]
    #[inline] pub fn ctmra1c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTMRA1C1INT != 0"]
    #[inline] pub fn test_ctmra1c1int(&self) -> bool {
        self.ctmra1c1int() != 0
    }

    #[doc="Sets the CTMRA1C1INT field."]
    #[inline] pub fn set_ctmra1c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR1."]
    #[inline] pub fn ctmrb0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTMRB0C1INT != 0"]
    #[inline] pub fn test_ctmrb0c1int(&self) -> bool {
        self.ctmrb0c1int() != 0
    }

    #[doc="Sets the CTMRB0C1INT field."]
    #[inline] pub fn set_ctmrb0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR1."]
    #[inline] pub fn ctmra0c1int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CTMRA0C1INT != 0"]
    #[inline] pub fn test_ctmra0c1int(&self) -> bool {
        self.ctmra0c1int() != 0
    }

    #[doc="Sets the CTMRA0C1INT field."]
    #[inline] pub fn set_ctmra0c1int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Counter/Timer B3 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CTMRB3C0INT != 0"]
    #[inline] pub fn test_ctmrb3c0int(&self) -> bool {
        self.ctmrb3c0int() != 0
    }

    #[doc="Sets the CTMRB3C0INT field."]
    #[inline] pub fn set_ctmrb3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Counter/Timer A3 interrupt based on COMPR0."]
    #[inline] pub fn ctmra3c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CTMRA3C0INT != 0"]
    #[inline] pub fn test_ctmra3c0int(&self) -> bool {
        self.ctmra3c0int() != 0
    }

    #[doc="Sets the CTMRA3C0INT field."]
    #[inline] pub fn set_ctmra3c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer B2 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CTMRB2C0INT != 0"]
    #[inline] pub fn test_ctmrb2c0int(&self) -> bool {
        self.ctmrb2c0int() != 0
    }

    #[doc="Sets the CTMRB2C0INT field."]
    #[inline] pub fn set_ctmrb2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Counter/Timer A2 interrupt based on COMPR0."]
    #[inline] pub fn ctmra2c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CTMRA2C0INT != 0"]
    #[inline] pub fn test_ctmra2c0int(&self) -> bool {
        self.ctmra2c0int() != 0
    }

    #[doc="Sets the CTMRA2C0INT field."]
    #[inline] pub fn set_ctmra2c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Counter/Timer B1 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTMRB1C0INT != 0"]
    #[inline] pub fn test_ctmrb1c0int(&self) -> bool {
        self.ctmrb1c0int() != 0
    }

    #[doc="Sets the CTMRB1C0INT field."]
    #[inline] pub fn set_ctmrb1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Counter/Timer A1 interrupt based on COMPR0."]
    #[inline] pub fn ctmra1c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTMRA1C0INT != 0"]
    #[inline] pub fn test_ctmra1c0int(&self) -> bool {
        self.ctmra1c0int() != 0
    }

    #[doc="Sets the CTMRA1C0INT field."]
    #[inline] pub fn set_ctmra1c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Counter/Timer B0 interrupt based on COMPR0."]
    #[inline] pub fn ctmrb0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTMRB0C0INT != 0"]
    #[inline] pub fn test_ctmrb0c0int(&self) -> bool {
        self.ctmrb0c0int() != 0
    }

    #[doc="Sets the CTMRB0C0INT field."]
    #[inline] pub fn set_ctmrb0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A0 interrupt based on COMPR0."]
    #[inline] pub fn ctmra0c0int(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTMRA0C0INT != 0"]
    #[inline] pub fn test_ctmra0c0int(&self) -> bool {
        self.ctmra0c0int() != 0
    }

    #[doc="Sets the CTMRA0C0INT field."]
    #[inline] pub fn set_ctmra0c0int<V: Into<bits::U1>>(mut self, value: V) -> Self {
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
        if self.ctmrb3c1int() != 0 { try!(write!(f, " ctmrb3c1int"))}
        if self.ctmra3c1int() != 0 { try!(write!(f, " ctmra3c1int"))}
        if self.ctmrb2c1int() != 0 { try!(write!(f, " ctmrb2c1int"))}
        if self.ctmra2c1int() != 0 { try!(write!(f, " ctmra2c1int"))}
        if self.ctmrb1c1int() != 0 { try!(write!(f, " ctmrb1c1int"))}
        if self.ctmra1c1int() != 0 { try!(write!(f, " ctmra1c1int"))}
        if self.ctmrb0c1int() != 0 { try!(write!(f, " ctmrb0c1int"))}
        if self.ctmra0c1int() != 0 { try!(write!(f, " ctmra0c1int"))}
        if self.ctmrb3c0int() != 0 { try!(write!(f, " ctmrb3c0int"))}
        if self.ctmra3c0int() != 0 { try!(write!(f, " ctmra3c0int"))}
        if self.ctmrb2c0int() != 0 { try!(write!(f, " ctmrb2c0int"))}
        if self.ctmra2c0int() != 0 { try!(write!(f, " ctmra2c0int"))}
        if self.ctmrb1c0int() != 0 { try!(write!(f, " ctmrb1c0int"))}
        if self.ctmra1c0int() != 0 { try!(write!(f, " ctmra1c0int"))}
        if self.ctmrb0c0int() != 0 { try!(write!(f, " ctmrb0c0int"))}
        if self.ctmra0c0int() != 0 { try!(write!(f, " ctmra0c0int"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stminten(pub u32);
impl Stminten {
    #[doc="CAPTURE register D has grabbed the value in the counter"]
    #[inline] pub fn captured(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CAPTURED != 0"]
    #[inline] pub fn test_captured(&self) -> bool {
        self.captured() != 0
    }

    #[doc="Sets the CAPTURED field."]
    #[inline] pub fn set_captured<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CAPTURE register C has grabbed the value in the counter"]
    #[inline] pub fn capturec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CAPTUREC != 0"]
    #[inline] pub fn test_capturec(&self) -> bool {
        self.capturec() != 0
    }

    #[doc="Sets the CAPTUREC field."]
    #[inline] pub fn set_capturec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CAPTURE register B has grabbed the value in the counter"]
    #[inline] pub fn captureb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CAPTUREB != 0"]
    #[inline] pub fn test_captureb(&self) -> bool {
        self.captureb() != 0
    }

    #[doc="Sets the CAPTUREB field."]
    #[inline] pub fn set_captureb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CAPTURE register A has grabbed the value in the counter"]
    #[inline] pub fn capturea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CAPTUREA != 0"]
    #[inline] pub fn test_capturea(&self) -> bool {
        self.capturea() != 0
    }

    #[doc="Sets the CAPTUREA field."]
    #[inline] pub fn set_capturea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline] pub fn overflow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVERFLOW != 0"]
    #[inline] pub fn test_overflow(&self) -> bool {
        self.overflow() != 0
    }

    #[doc="Sets the OVERFLOW field."]
    #[inline] pub fn set_overflow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register H."]
    #[inline] pub fn compareh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COMPAREH != 0"]
    #[inline] pub fn test_compareh(&self) -> bool {
        self.compareh() != 0
    }

    #[doc="Sets the COMPAREH field."]
    #[inline] pub fn set_compareh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register G."]
    #[inline] pub fn compareg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COMPAREG != 0"]
    #[inline] pub fn test_compareg(&self) -> bool {
        self.compareg() != 0
    }

    #[doc="Sets the COMPAREG field."]
    #[inline] pub fn set_compareg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register F."]
    #[inline] pub fn comparef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMPAREF != 0"]
    #[inline] pub fn test_comparef(&self) -> bool {
        self.comparef() != 0
    }

    #[doc="Sets the COMPAREF field."]
    #[inline] pub fn set_comparef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register E."]
    #[inline] pub fn comparee(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COMPAREE != 0"]
    #[inline] pub fn test_comparee(&self) -> bool {
        self.comparee() != 0
    }

    #[doc="Sets the COMPAREE field."]
    #[inline] pub fn set_comparee<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register D."]
    #[inline] pub fn compared(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COMPARED != 0"]
    #[inline] pub fn test_compared(&self) -> bool {
        self.compared() != 0
    }

    #[doc="Sets the COMPARED field."]
    #[inline] pub fn set_compared<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register C."]
    #[inline] pub fn comparec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if COMPAREC != 0"]
    #[inline] pub fn test_comparec(&self) -> bool {
        self.comparec() != 0
    }

    #[doc="Sets the COMPAREC field."]
    #[inline] pub fn set_comparec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register B."]
    #[inline] pub fn compareb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMPAREB != 0"]
    #[inline] pub fn test_compareb(&self) -> bool {
        self.compareb() != 0
    }

    #[doc="Sets the COMPAREB field."]
    #[inline] pub fn set_compareb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register A."]
    #[inline] pub fn comparea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPAREA != 0"]
    #[inline] pub fn test_comparea(&self) -> bool {
        self.comparea() != 0
    }

    #[doc="Sets the COMPAREA field."]
    #[inline] pub fn set_comparea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stminten {
    #[inline]
    fn from(other: u32) -> Self {
         Stminten(other)
    }
}

impl ::core::fmt::Display for Stminten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stminten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.captured() != 0 { try!(write!(f, " captured"))}
        if self.capturec() != 0 { try!(write!(f, " capturec"))}
        if self.captureb() != 0 { try!(write!(f, " captureb"))}
        if self.capturea() != 0 { try!(write!(f, " capturea"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compareh() != 0 { try!(write!(f, " compareh"))}
        if self.compareg() != 0 { try!(write!(f, " compareg"))}
        if self.comparef() != 0 { try!(write!(f, " comparef"))}
        if self.comparee() != 0 { try!(write!(f, " comparee"))}
        if self.compared() != 0 { try!(write!(f, " compared"))}
        if self.comparec() != 0 { try!(write!(f, " comparec"))}
        if self.compareb() != 0 { try!(write!(f, " compareb"))}
        if self.comparea() != 0 { try!(write!(f, " comparea"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmintstat(pub u32);
impl Stmintstat {
    #[doc="CAPTURE register D has grabbed the value in the counter"]
    #[inline] pub fn captured(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CAPTURED != 0"]
    #[inline] pub fn test_captured(&self) -> bool {
        self.captured() != 0
    }

    #[doc="Sets the CAPTURED field."]
    #[inline] pub fn set_captured<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CAPTURE register C has grabbed the value in the counter"]
    #[inline] pub fn capturec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CAPTUREC != 0"]
    #[inline] pub fn test_capturec(&self) -> bool {
        self.capturec() != 0
    }

    #[doc="Sets the CAPTUREC field."]
    #[inline] pub fn set_capturec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CAPTURE register B has grabbed the value in the counter"]
    #[inline] pub fn captureb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CAPTUREB != 0"]
    #[inline] pub fn test_captureb(&self) -> bool {
        self.captureb() != 0
    }

    #[doc="Sets the CAPTUREB field."]
    #[inline] pub fn set_captureb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CAPTURE register A has grabbed the value in the counter"]
    #[inline] pub fn capturea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CAPTUREA != 0"]
    #[inline] pub fn test_capturea(&self) -> bool {
        self.capturea() != 0
    }

    #[doc="Sets the CAPTUREA field."]
    #[inline] pub fn set_capturea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline] pub fn overflow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVERFLOW != 0"]
    #[inline] pub fn test_overflow(&self) -> bool {
        self.overflow() != 0
    }

    #[doc="Sets the OVERFLOW field."]
    #[inline] pub fn set_overflow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register H."]
    #[inline] pub fn compareh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COMPAREH != 0"]
    #[inline] pub fn test_compareh(&self) -> bool {
        self.compareh() != 0
    }

    #[doc="Sets the COMPAREH field."]
    #[inline] pub fn set_compareh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register G."]
    #[inline] pub fn compareg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COMPAREG != 0"]
    #[inline] pub fn test_compareg(&self) -> bool {
        self.compareg() != 0
    }

    #[doc="Sets the COMPAREG field."]
    #[inline] pub fn set_compareg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register F."]
    #[inline] pub fn comparef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMPAREF != 0"]
    #[inline] pub fn test_comparef(&self) -> bool {
        self.comparef() != 0
    }

    #[doc="Sets the COMPAREF field."]
    #[inline] pub fn set_comparef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register E."]
    #[inline] pub fn comparee(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COMPAREE != 0"]
    #[inline] pub fn test_comparee(&self) -> bool {
        self.comparee() != 0
    }

    #[doc="Sets the COMPAREE field."]
    #[inline] pub fn set_comparee<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register D."]
    #[inline] pub fn compared(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COMPARED != 0"]
    #[inline] pub fn test_compared(&self) -> bool {
        self.compared() != 0
    }

    #[doc="Sets the COMPARED field."]
    #[inline] pub fn set_compared<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register C."]
    #[inline] pub fn comparec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if COMPAREC != 0"]
    #[inline] pub fn test_comparec(&self) -> bool {
        self.comparec() != 0
    }

    #[doc="Sets the COMPAREC field."]
    #[inline] pub fn set_comparec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register B."]
    #[inline] pub fn compareb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMPAREB != 0"]
    #[inline] pub fn test_compareb(&self) -> bool {
        self.compareb() != 0
    }

    #[doc="Sets the COMPAREB field."]
    #[inline] pub fn set_compareb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register A."]
    #[inline] pub fn comparea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPAREA != 0"]
    #[inline] pub fn test_comparea(&self) -> bool {
        self.comparea() != 0
    }

    #[doc="Sets the COMPAREA field."]
    #[inline] pub fn set_comparea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stmintstat {
    #[inline]
    fn from(other: u32) -> Self {
         Stmintstat(other)
    }
}

impl ::core::fmt::Display for Stmintstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stmintstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.captured() != 0 { try!(write!(f, " captured"))}
        if self.capturec() != 0 { try!(write!(f, " capturec"))}
        if self.captureb() != 0 { try!(write!(f, " captureb"))}
        if self.capturea() != 0 { try!(write!(f, " capturea"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compareh() != 0 { try!(write!(f, " compareh"))}
        if self.compareg() != 0 { try!(write!(f, " compareg"))}
        if self.comparef() != 0 { try!(write!(f, " comparef"))}
        if self.comparee() != 0 { try!(write!(f, " comparee"))}
        if self.compared() != 0 { try!(write!(f, " compared"))}
        if self.comparec() != 0 { try!(write!(f, " comparec"))}
        if self.compareb() != 0 { try!(write!(f, " compareb"))}
        if self.comparea() != 0 { try!(write!(f, " comparea"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmintclr(pub u32);
impl Stmintclr {
    #[doc="CAPTURE register D has grabbed the value in the counter"]
    #[inline] pub fn captured(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CAPTURED != 0"]
    #[inline] pub fn test_captured(&self) -> bool {
        self.captured() != 0
    }

    #[doc="Sets the CAPTURED field."]
    #[inline] pub fn set_captured<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CAPTURE register C has grabbed the value in the counter"]
    #[inline] pub fn capturec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CAPTUREC != 0"]
    #[inline] pub fn test_capturec(&self) -> bool {
        self.capturec() != 0
    }

    #[doc="Sets the CAPTUREC field."]
    #[inline] pub fn set_capturec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CAPTURE register B has grabbed the value in the counter"]
    #[inline] pub fn captureb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CAPTUREB != 0"]
    #[inline] pub fn test_captureb(&self) -> bool {
        self.captureb() != 0
    }

    #[doc="Sets the CAPTUREB field."]
    #[inline] pub fn set_captureb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CAPTURE register A has grabbed the value in the counter"]
    #[inline] pub fn capturea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CAPTUREA != 0"]
    #[inline] pub fn test_capturea(&self) -> bool {
        self.capturea() != 0
    }

    #[doc="Sets the CAPTUREA field."]
    #[inline] pub fn set_capturea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline] pub fn overflow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVERFLOW != 0"]
    #[inline] pub fn test_overflow(&self) -> bool {
        self.overflow() != 0
    }

    #[doc="Sets the OVERFLOW field."]
    #[inline] pub fn set_overflow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register H."]
    #[inline] pub fn compareh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COMPAREH != 0"]
    #[inline] pub fn test_compareh(&self) -> bool {
        self.compareh() != 0
    }

    #[doc="Sets the COMPAREH field."]
    #[inline] pub fn set_compareh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register G."]
    #[inline] pub fn compareg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COMPAREG != 0"]
    #[inline] pub fn test_compareg(&self) -> bool {
        self.compareg() != 0
    }

    #[doc="Sets the COMPAREG field."]
    #[inline] pub fn set_compareg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register F."]
    #[inline] pub fn comparef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMPAREF != 0"]
    #[inline] pub fn test_comparef(&self) -> bool {
        self.comparef() != 0
    }

    #[doc="Sets the COMPAREF field."]
    #[inline] pub fn set_comparef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register E."]
    #[inline] pub fn comparee(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COMPAREE != 0"]
    #[inline] pub fn test_comparee(&self) -> bool {
        self.comparee() != 0
    }

    #[doc="Sets the COMPAREE field."]
    #[inline] pub fn set_comparee<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register D."]
    #[inline] pub fn compared(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COMPARED != 0"]
    #[inline] pub fn test_compared(&self) -> bool {
        self.compared() != 0
    }

    #[doc="Sets the COMPARED field."]
    #[inline] pub fn set_compared<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register C."]
    #[inline] pub fn comparec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if COMPAREC != 0"]
    #[inline] pub fn test_comparec(&self) -> bool {
        self.comparec() != 0
    }

    #[doc="Sets the COMPAREC field."]
    #[inline] pub fn set_comparec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register B."]
    #[inline] pub fn compareb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMPAREB != 0"]
    #[inline] pub fn test_compareb(&self) -> bool {
        self.compareb() != 0
    }

    #[doc="Sets the COMPAREB field."]
    #[inline] pub fn set_compareb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register A."]
    #[inline] pub fn comparea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPAREA != 0"]
    #[inline] pub fn test_comparea(&self) -> bool {
        self.comparea() != 0
    }

    #[doc="Sets the COMPAREA field."]
    #[inline] pub fn set_comparea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stmintclr {
    #[inline]
    fn from(other: u32) -> Self {
         Stmintclr(other)
    }
}

impl ::core::fmt::Display for Stmintclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stmintclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.captured() != 0 { try!(write!(f, " captured"))}
        if self.capturec() != 0 { try!(write!(f, " capturec"))}
        if self.captureb() != 0 { try!(write!(f, " captureb"))}
        if self.capturea() != 0 { try!(write!(f, " capturea"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compareh() != 0 { try!(write!(f, " compareh"))}
        if self.compareg() != 0 { try!(write!(f, " compareg"))}
        if self.comparef() != 0 { try!(write!(f, " comparef"))}
        if self.comparee() != 0 { try!(write!(f, " comparee"))}
        if self.compared() != 0 { try!(write!(f, " compared"))}
        if self.comparec() != 0 { try!(write!(f, " comparec"))}
        if self.compareb() != 0 { try!(write!(f, " compareb"))}
        if self.comparea() != 0 { try!(write!(f, " comparea"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmintset(pub u32);
impl Stmintset {
    #[doc="CAPTURE register D has grabbed the value in the counter"]
    #[inline] pub fn captured(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CAPTURED != 0"]
    #[inline] pub fn test_captured(&self) -> bool {
        self.captured() != 0
    }

    #[doc="Sets the CAPTURED field."]
    #[inline] pub fn set_captured<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CAPTURE register C has grabbed the value in the counter"]
    #[inline] pub fn capturec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CAPTUREC != 0"]
    #[inline] pub fn test_capturec(&self) -> bool {
        self.capturec() != 0
    }

    #[doc="Sets the CAPTUREC field."]
    #[inline] pub fn set_capturec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CAPTURE register B has grabbed the value in the counter"]
    #[inline] pub fn captureb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CAPTUREB != 0"]
    #[inline] pub fn test_captureb(&self) -> bool {
        self.captureb() != 0
    }

    #[doc="Sets the CAPTUREB field."]
    #[inline] pub fn set_captureb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CAPTURE register A has grabbed the value in the counter"]
    #[inline] pub fn capturea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CAPTUREA != 0"]
    #[inline] pub fn test_capturea(&self) -> bool {
        self.capturea() != 0
    }

    #[doc="Sets the CAPTUREA field."]
    #[inline] pub fn set_capturea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline] pub fn overflow(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVERFLOW != 0"]
    #[inline] pub fn test_overflow(&self) -> bool {
        self.overflow() != 0
    }

    #[doc="Sets the OVERFLOW field."]
    #[inline] pub fn set_overflow<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register H."]
    #[inline] pub fn compareh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COMPAREH != 0"]
    #[inline] pub fn test_compareh(&self) -> bool {
        self.compareh() != 0
    }

    #[doc="Sets the COMPAREH field."]
    #[inline] pub fn set_compareh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register G."]
    #[inline] pub fn compareg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COMPAREG != 0"]
    #[inline] pub fn test_compareg(&self) -> bool {
        self.compareg() != 0
    }

    #[doc="Sets the COMPAREG field."]
    #[inline] pub fn set_compareg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register F."]
    #[inline] pub fn comparef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if COMPAREF != 0"]
    #[inline] pub fn test_comparef(&self) -> bool {
        self.comparef() != 0
    }

    #[doc="Sets the COMPAREF field."]
    #[inline] pub fn set_comparef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register E."]
    #[inline] pub fn comparee(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COMPAREE != 0"]
    #[inline] pub fn test_comparee(&self) -> bool {
        self.comparee() != 0
    }

    #[doc="Sets the COMPAREE field."]
    #[inline] pub fn set_comparee<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register D."]
    #[inline] pub fn compared(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if COMPARED != 0"]
    #[inline] pub fn test_compared(&self) -> bool {
        self.compared() != 0
    }

    #[doc="Sets the COMPARED field."]
    #[inline] pub fn set_compared<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register C."]
    #[inline] pub fn comparec(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if COMPAREC != 0"]
    #[inline] pub fn test_comparec(&self) -> bool {
        self.comparec() != 0
    }

    #[doc="Sets the COMPAREC field."]
    #[inline] pub fn set_comparec<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register B."]
    #[inline] pub fn compareb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMPAREB != 0"]
    #[inline] pub fn test_compareb(&self) -> bool {
        self.compareb() != 0
    }

    #[doc="Sets the COMPAREB field."]
    #[inline] pub fn set_compareb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="COUNTER is greater than or equal to COMPARE register A."]
    #[inline] pub fn comparea(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPAREA != 0"]
    #[inline] pub fn test_comparea(&self) -> bool {
        self.comparea() != 0
    }

    #[doc="Sets the COMPAREA field."]
    #[inline] pub fn set_comparea<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stmintset {
    #[inline]
    fn from(other: u32) -> Self {
         Stmintset(other)
    }
}

impl ::core::fmt::Display for Stmintset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stmintset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.captured() != 0 { try!(write!(f, " captured"))}
        if self.capturec() != 0 { try!(write!(f, " capturec"))}
        if self.captureb() != 0 { try!(write!(f, " captureb"))}
        if self.capturea() != 0 { try!(write!(f, " capturea"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compareh() != 0 { try!(write!(f, " compareh"))}
        if self.compareg() != 0 { try!(write!(f, " compareg"))}
        if self.comparef() != 0 { try!(write!(f, " comparef"))}
        if self.comparee() != 0 { try!(write!(f, " comparee"))}
        if self.compared() != 0 { try!(write!(f, " compared"))}
        if self.comparec() != 0 { try!(write!(f, " comparec"))}
        if self.compareb() != 0 { try!(write!(f, " compareb"))}
        if self.comparea() != 0 { try!(write!(f, " comparea"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


