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

#[doc="STIMER Interrupt registers: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stminten(pub u32);
impl Stminten {
    #[doc="CAPTURE register n has grabbed the value in the counter"]
    #[inline] pub fn capture<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 9 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [9]
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
        let shift: usize = 9 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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

    #[doc="COUNTER is greater than or equal to COMPARE register n."]
    #[inline] pub fn compare<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPARE != 0"]
    #[inline] pub fn test_compare<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.compare(index) != 0
    }

    #[doc="Sets the COMPARE field."]
    #[inline] pub fn set_compare<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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
        if self.capture(0) != 0 { try!(write!(f, " capture[0]"))}
        if self.capture(1) != 0 { try!(write!(f, " capture[1]"))}
        if self.capture(2) != 0 { try!(write!(f, " capture[2]"))}
        if self.capture(3) != 0 { try!(write!(f, " capture[3]"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compare(0) != 0 { try!(write!(f, " compare[0]"))}
        if self.compare(1) != 0 { try!(write!(f, " compare[1]"))}
        if self.compare(2) != 0 { try!(write!(f, " compare[2]"))}
        if self.compare(3) != 0 { try!(write!(f, " compare[3]"))}
        if self.compare(4) != 0 { try!(write!(f, " compare[4]"))}
        if self.compare(5) != 0 { try!(write!(f, " compare[5]"))}
        if self.compare(6) != 0 { try!(write!(f, " compare[6]"))}
        if self.compare(7) != 0 { try!(write!(f, " compare[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmintstat(pub u32);
impl Stmintstat {
    #[doc="CAPTURE register n has grabbed the value in the counter"]
    #[inline] pub fn capture<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 9 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [9]
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
        let shift: usize = 9 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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

    #[doc="COUNTER is greater than or equal to COMPARE register n."]
    #[inline] pub fn compare<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPARE != 0"]
    #[inline] pub fn test_compare<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.compare(index) != 0
    }

    #[doc="Sets the COMPARE field."]
    #[inline] pub fn set_compare<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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
        if self.capture(0) != 0 { try!(write!(f, " capture[0]"))}
        if self.capture(1) != 0 { try!(write!(f, " capture[1]"))}
        if self.capture(2) != 0 { try!(write!(f, " capture[2]"))}
        if self.capture(3) != 0 { try!(write!(f, " capture[3]"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compare(0) != 0 { try!(write!(f, " compare[0]"))}
        if self.compare(1) != 0 { try!(write!(f, " compare[1]"))}
        if self.compare(2) != 0 { try!(write!(f, " compare[2]"))}
        if self.compare(3) != 0 { try!(write!(f, " compare[3]"))}
        if self.compare(4) != 0 { try!(write!(f, " compare[4]"))}
        if self.compare(5) != 0 { try!(write!(f, " compare[5]"))}
        if self.compare(6) != 0 { try!(write!(f, " compare[6]"))}
        if self.compare(7) != 0 { try!(write!(f, " compare[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmintclr(pub u32);
impl Stmintclr {
    #[doc="CAPTURE register n has grabbed the value in the counter"]
    #[inline] pub fn capture<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 9 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [9]
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
        let shift: usize = 9 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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

    #[doc="COUNTER is greater than or equal to COMPARE register n."]
    #[inline] pub fn compare<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPARE != 0"]
    #[inline] pub fn test_compare<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.compare(index) != 0
    }

    #[doc="Sets the COMPARE field."]
    #[inline] pub fn set_compare<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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
        if self.capture(0) != 0 { try!(write!(f, " capture[0]"))}
        if self.capture(1) != 0 { try!(write!(f, " capture[1]"))}
        if self.capture(2) != 0 { try!(write!(f, " capture[2]"))}
        if self.capture(3) != 0 { try!(write!(f, " capture[3]"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compare(0) != 0 { try!(write!(f, " compare[0]"))}
        if self.compare(1) != 0 { try!(write!(f, " compare[1]"))}
        if self.compare(2) != 0 { try!(write!(f, " compare[2]"))}
        if self.compare(3) != 0 { try!(write!(f, " compare[3]"))}
        if self.compare(4) != 0 { try!(write!(f, " compare[4]"))}
        if self.compare(5) != 0 { try!(write!(f, " compare[5]"))}
        if self.compare(6) != 0 { try!(write!(f, " compare[6]"))}
        if self.compare(7) != 0 { try!(write!(f, " compare[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="STIMER Interrupt registers: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stmintset(pub u32);
impl Stmintset {
    #[doc="CAPTURE register n has grabbed the value in the counter"]
    #[inline] pub fn capture<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 9 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [9]
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
        let shift: usize = 9 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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

    #[doc="COUNTER is greater than or equal to COMPARE register A."]
    #[inline] pub fn compare<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMPARE != 0"]
    #[inline] pub fn test_compare<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.compare(index) != 0
    }

    #[doc="Sets the COMPARE field."]
    #[inline] pub fn set_compare<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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
        if self.capture(0) != 0 { try!(write!(f, " capture[0]"))}
        if self.capture(1) != 0 { try!(write!(f, " capture[1]"))}
        if self.capture(2) != 0 { try!(write!(f, " capture[2]"))}
        if self.capture(3) != 0 { try!(write!(f, " capture[3]"))}
        if self.overflow() != 0 { try!(write!(f, " overflow"))}
        if self.compare(0) != 0 { try!(write!(f, " compare[0]"))}
        if self.compare(1) != 0 { try!(write!(f, " compare[1]"))}
        if self.compare(2) != 0 { try!(write!(f, " compare[2]"))}
        if self.compare(3) != 0 { try!(write!(f, " compare[3]"))}
        if self.compare(4) != 0 { try!(write!(f, " compare[4]"))}
        if self.compare(5) != 0 { try!(write!(f, " compare[5]"))}
        if self.compare(6) != 0 { try!(write!(f, " compare[6]"))}
        if self.compare(7) != 0 { try!(write!(f, " compare[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


