#[allow(unused_imports)] use bobbin_common::*;

periph!( CTIMER, Ctimer, _CTIMER, CtimerPeriph, 0x40008000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="CTIMER Peripheral"]
pub struct CtimerPeriph(pub usize); 

impl super::sig::Signal<super::sig::Tcta0> for Ctimer {}
impl super::sig::SignalTctA0<super::sig::Tcta0> for Ctimer {}
impl super::sig::Signal<super::sig::Tcta1> for Ctimer {}
impl super::sig::SignalTctA1<super::sig::Tcta1> for Ctimer {}
impl super::sig::Signal<super::sig::Tcta2> for Ctimer {}
impl super::sig::SignalTctA2<super::sig::Tcta2> for Ctimer {}
impl super::sig::Signal<super::sig::Tcta3> for Ctimer {}
impl super::sig::SignalTctA3<super::sig::Tcta3> for Ctimer {}
impl super::sig::Signal<super::sig::Tctb0> for Ctimer {}
impl super::sig::SignalTctB0<super::sig::Tctb0> for Ctimer {}
impl super::sig::Signal<super::sig::Tctb1> for Ctimer {}
impl super::sig::SignalTctB1<super::sig::Tctb1> for Ctimer {}
impl super::sig::Signal<super::sig::Tctb2> for Ctimer {}
impl super::sig::SignalTctB2<super::sig::Tctb2> for Ctimer {}
impl super::sig::Signal<super::sig::Tctb3> for Ctimer {}
impl super::sig::SignalTctB3<super::sig::Tctb3> for Ctimer {}


impl CtimerPeriph {
    #[doc="Get the *mut pointer for the TMR register."]
    #[inline] pub fn tmr_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Tmr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x0 + (index << 4)) as *mut Tmr
    }

    #[doc="Get the *const pointer for the TMR register."]
    #[inline] pub fn tmr_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Tmr { 
           self.tmr_mut(index)
    }

    #[doc="Read the TMR register."]
    #[inline] pub fn tmr<I: Into<bits::R4>>(&self, index: I) -> Tmr { 
        unsafe {
            read_volatile(self.tmr_ptr(index))
        }
    }

    #[doc="Write the TMR register."]
    #[inline] pub fn set_tmr<I: Into<bits::R4>, F: FnOnce(Tmr) -> Tmr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr_mut(index), f(Tmr(0)));
        }
        self
    }

    #[doc="Modify the TMR register."]
    #[inline] pub fn with_tmr<I: Into<bits::R4> + Copy, F: FnOnce(Tmr) -> Tmr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr_mut(index), f(self.tmr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMPRA register."]
    #[inline] pub fn cmpra_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Cmpra { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x4 + (index << 4)) as *mut Cmpra
    }

    #[doc="Get the *const pointer for the CMPRA register."]
    #[inline] pub fn cmpra_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Cmpra { 
           self.cmpra_mut(index)
    }

    #[doc="Read the CMPRA register."]
    #[inline] pub fn cmpra<I: Into<bits::R4>>(&self, index: I) -> Cmpra { 
        unsafe {
            read_volatile(self.cmpra_ptr(index))
        }
    }

    #[doc="Write the CMPRA register."]
    #[inline] pub fn set_cmpra<I: Into<bits::R4>, F: FnOnce(Cmpra) -> Cmpra>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmpra_mut(index), f(Cmpra(0)));
        }
        self
    }

    #[doc="Modify the CMPRA register."]
    #[inline] pub fn with_cmpra<I: Into<bits::R4> + Copy, F: FnOnce(Cmpra) -> Cmpra>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmpra_mut(index), f(self.cmpra(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMPRB register."]
    #[inline] pub fn cmprb_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Cmprb { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x8 + (index << 4)) as *mut Cmprb
    }

    #[doc="Get the *const pointer for the CMPRB register."]
    #[inline] pub fn cmprb_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Cmprb { 
           self.cmprb_mut(index)
    }

    #[doc="Read the CMPRB register."]
    #[inline] pub fn cmprb<I: Into<bits::R4>>(&self, index: I) -> Cmprb { 
        unsafe {
            read_volatile(self.cmprb_ptr(index))
        }
    }

    #[doc="Write the CMPRB register."]
    #[inline] pub fn set_cmprb<I: Into<bits::R4>, F: FnOnce(Cmprb) -> Cmprb>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmprb_mut(index), f(Cmprb(0)));
        }
        self
    }

    #[doc="Modify the CMPRB register."]
    #[inline] pub fn with_cmprb<I: Into<bits::R4> + Copy, F: FnOnce(Cmprb) -> Cmprb>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmprb_mut(index), f(self.cmprb(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Ctrl { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xc + (index << 4)) as *mut Ctrl
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Ctrl { 
           self.ctrl_mut(index)
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl<I: Into<bits::R4>>(&self, index: I) -> Ctrl { 
        unsafe {
            read_volatile(self.ctrl_ptr(index))
        }
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn set_ctrl<I: Into<bits::R4>, F: FnOnce(Ctrl) -> Ctrl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(index), f(Ctrl(0)));
        }
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<I: Into<bits::R4> + Copy, F: FnOnce(Ctrl) -> Ctrl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(index), f(self.ctrl(index)));
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

#[doc="Counter/Timer Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tmr(pub u32);
impl Tmr {
    #[doc="Counter/Timer B."]
    #[inline] pub fn cttmrb(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CTTMRB != 0"]
    #[inline] pub fn test_cttmrb(&self) -> bool {
        self.cttmrb() != 0
    }

    #[doc="Sets the CTTMRB field."]
    #[inline] pub fn set_cttmrb<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A."]
    #[inline] pub fn cttmra(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CTTMRA != 0"]
    #[inline] pub fn test_cttmra(&self) -> bool {
        self.cttmra() != 0
    }

    #[doc="Sets the CTTMRA field."]
    #[inline] pub fn set_cttmra<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tmr {
    #[inline]
    fn from(other: u32) -> Self {
         Tmr(other)
    }
}

impl ::core::fmt::Display for Tmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cttmrb() != 0 { try!(write!(f, " cttmrb=0x{:x}", self.cttmrb()))}
        if self.cttmra() != 0 { try!(write!(f, " cttmra=0x{:x}", self.cttmra()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer A Compare Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmpra(pub u32);
impl Cmpra {
    #[doc="Counter/Timer A Compare Register 1. Holds the upper limit for timer half A."]
    #[inline] pub fn cmpr1a0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CMPR1A0 != 0"]
    #[inline] pub fn test_cmpr1a0(&self) -> bool {
        self.cmpr1a0() != 0
    }

    #[doc="Sets the CMPR1A0 field."]
    #[inline] pub fn set_cmpr1a0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A Compare Register 0. Holds the lower limit for timer half A."]
    #[inline] pub fn cmpr0a0(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMPR0A0 != 0"]
    #[inline] pub fn test_cmpr0a0(&self) -> bool {
        self.cmpr0a0() != 0
    }

    #[doc="Sets the CMPR0A0 field."]
    #[inline] pub fn set_cmpr0a0<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmpra {
    #[inline]
    fn from(other: u32) -> Self {
         Cmpra(other)
    }
}

impl ::core::fmt::Display for Cmpra {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmpra {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmpr1a0() != 0 { try!(write!(f, " cmpr1a0=0x{:x}", self.cmpr1a0()))}
        if self.cmpr0a0() != 0 { try!(write!(f, " cmpr0a0=0x{:x}", self.cmpr0a0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer B Compare Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmprb(pub u32);
impl Cmprb {
    #[doc="Counter/Timer B Compare Register 1. Holds the upper limit for timer half B."]
    #[inline] pub fn cmpr1b(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if CMPR1B != 0"]
    #[inline] pub fn test_cmpr1b(&self) -> bool {
        self.cmpr1b() != 0
    }

    #[doc="Sets the CMPR1B field."]
    #[inline] pub fn set_cmpr1b<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer B Compare Register 0. Holds the lower limit for timer half B."]
    #[inline] pub fn cmpr0b(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CMPR0B != 0"]
    #[inline] pub fn test_cmpr0b(&self) -> bool {
        self.cmpr0b() != 0
    }

    #[doc="Sets the CMPR0B field."]
    #[inline] pub fn set_cmpr0b<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmprb {
    #[inline]
    fn from(other: u32) -> Self {
         Cmprb(other)
    }
}

impl ::core::fmt::Display for Cmprb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmprb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmpr1b() != 0 { try!(write!(f, " cmpr1b=0x{:x}", self.cmpr1b()))}
        if self.cmpr0b() != 0 { try!(write!(f, " cmpr0b=0x{:x}", self.cmpr0b()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter/Timer Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Counter/Timer A/B Link bit."]
    #[inline] pub fn ctlink(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CTLINK != 0"]
    #[inline] pub fn test_ctlink(&self) -> bool {
        self.ctlink() != 0
    }

    #[doc="Sets the CTLINK field."]
    #[inline] pub fn set_ctlink<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Counter/Timer B Output Enable bit."]
    #[inline] pub fn tmrbpe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if TMRBPE != 0"]
    #[inline] pub fn test_tmrbpe(&self) -> bool {
        self.tmrbpe() != 0
    }

    #[doc="Sets the TMRBPE field."]
    #[inline] pub fn set_tmrbpe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Counter/Timer B output polarity."]
    #[inline] pub fn tmrbpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TMRBPOL != 0"]
    #[inline] pub fn test_tmrbpol(&self) -> bool {
        self.tmrbpol() != 0
    }

    #[doc="Sets the TMRBPOL field."]
    #[inline] pub fn set_tmrbpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Counter/Timer B Clear bit."]
    #[inline] pub fn tmrbclr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if TMRBCLR != 0"]
    #[inline] pub fn test_tmrbclr(&self) -> bool {
        self.tmrbclr() != 0
    }

    #[doc="Sets the TMRBCLR field."]
    #[inline] pub fn set_tmrbclr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Counter/Timer B Interrupt Enable bit for COMPR1."]
    #[inline] pub fn tmrbie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if TMRBIE1 != 0"]
    #[inline] pub fn test_tmrbie1(&self) -> bool {
        self.tmrbie1() != 0
    }

    #[doc="Sets the TMRBIE1 field."]
    #[inline] pub fn set_tmrbie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Counter/Timer B Interrupt Enable bit for COMPR0."]
    #[inline] pub fn tmrbie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if TMRBIE0 != 0"]
    #[inline] pub fn test_tmrbie0(&self) -> bool {
        self.tmrbie0() != 0
    }

    #[doc="Sets the TMRBIE0 field."]
    #[inline] pub fn set_tmrbie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Counter/Timer B Function Select."]
    #[inline] pub fn tmrbfn(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x7) as u8) } // [24:22]
    }

    #[doc="Returns true if TMRBFN != 0"]
    #[inline] pub fn test_tmrbfn(&self) -> bool {
        self.tmrbfn() != 0
    }

    #[doc="Sets the TMRBFN field."]
    #[inline] pub fn set_tmrbfn<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Counter/Timer B Clock Select."]
    #[inline] pub fn tmrbclk(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1f) as u8) } // [21:17]
    }

    #[doc="Returns true if TMRBCLK != 0"]
    #[inline] pub fn test_tmrbclk(&self) -> bool {
        self.tmrbclk() != 0
    }

    #[doc="Sets the TMRBCLK field."]
    #[inline] pub fn set_tmrbclk<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Counter/Timer B Enable bit."]
    #[inline] pub fn tmrben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TMRBEN != 0"]
    #[inline] pub fn test_tmrben(&self) -> bool {
        self.tmrben() != 0
    }

    #[doc="Sets the TMRBEN field."]
    #[inline] pub fn set_tmrben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter/Timer A Output Enable bit."]
    #[inline] pub fn tmrape(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TMRAPE != 0"]
    #[inline] pub fn test_tmrape(&self) -> bool {
        self.tmrape() != 0
    }

    #[doc="Sets the TMRAPE field."]
    #[inline] pub fn set_tmrape<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Counter/Timer A output polarity."]
    #[inline] pub fn tmra0pol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TMRA0POL != 0"]
    #[inline] pub fn test_tmra0pol(&self) -> bool {
        self.tmra0pol() != 0
    }

    #[doc="Sets the TMRA0POL field."]
    #[inline] pub fn set_tmra0pol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Counter/Timer A Clear bit."]
    #[inline] pub fn tmraclr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TMRACLR != 0"]
    #[inline] pub fn test_tmraclr(&self) -> bool {
        self.tmraclr() != 0
    }

    #[doc="Sets the TMRACLR field."]
    #[inline] pub fn set_tmraclr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Counter/Timer A Interrupt Enable bit based on COMPR1."]
    #[inline] pub fn tmraie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TMRAIE1 != 0"]
    #[inline] pub fn test_tmraie1(&self) -> bool {
        self.tmraie1() != 0
    }

    #[doc="Sets the TMRAIE1 field."]
    #[inline] pub fn set_tmraie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Counter/Timer A Interrupt Enable bit based on COMPR0."]
    #[inline] pub fn tmraie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TMRAIE0 != 0"]
    #[inline] pub fn test_tmraie0(&self) -> bool {
        self.tmraie0() != 0
    }

    #[doc="Sets the TMRAIE0 field."]
    #[inline] pub fn set_tmraie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter/Timer A Function Select."]
    #[inline] pub fn tmrafn(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7) as u8) } // [8:6]
    }

    #[doc="Returns true if TMRAFN != 0"]
    #[inline] pub fn test_tmrafn(&self) -> bool {
        self.tmrafn() != 0
    }

    #[doc="Sets the TMRAFN field."]
    #[inline] pub fn set_tmrafn<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter/Timer A Clock Select."]
    #[inline] pub fn tmraclk(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1f) as u8) } // [5:1]
    }

    #[doc="Returns true if TMRACLK != 0"]
    #[inline] pub fn test_tmraclk(&self) -> bool {
        self.tmraclk() != 0
    }

    #[doc="Sets the TMRACLK field."]
    #[inline] pub fn set_tmraclk<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter/Timer A Enable bit."]
    #[inline] pub fn tmraen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TMRAEN != 0"]
    #[inline] pub fn test_tmraen(&self) -> bool {
        self.tmraen() != 0
    }

    #[doc="Sets the TMRAEN field."]
    #[inline] pub fn set_tmraen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Ctrl(other)
    }
}

impl ::core::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ctlink() != 0 { try!(write!(f, " ctlink"))}
        if self.tmrbpe() != 0 { try!(write!(f, " tmrbpe"))}
        if self.tmrbpol() != 0 { try!(write!(f, " tmrbpol"))}
        if self.tmrbclr() != 0 { try!(write!(f, " tmrbclr"))}
        if self.tmrbie1() != 0 { try!(write!(f, " tmrbie1"))}
        if self.tmrbie0() != 0 { try!(write!(f, " tmrbie0"))}
        if self.tmrbfn() != 0 { try!(write!(f, " tmrbfn=0x{:x}", self.tmrbfn()))}
        if self.tmrbclk() != 0 { try!(write!(f, " tmrbclk=0x{:x}", self.tmrbclk()))}
        if self.tmrben() != 0 { try!(write!(f, " tmrben"))}
        if self.tmrape() != 0 { try!(write!(f, " tmrape"))}
        if self.tmra0pol() != 0 { try!(write!(f, " tmra0pol"))}
        if self.tmraclr() != 0 { try!(write!(f, " tmraclr"))}
        if self.tmraie1() != 0 { try!(write!(f, " tmraie1"))}
        if self.tmraie0() != 0 { try!(write!(f, " tmraie0"))}
        if self.tmrafn() != 0 { try!(write!(f, " tmrafn=0x{:x}", self.tmrafn()))}
        if self.tmraclk() != 0 { try!(write!(f, " tmraclk=0x{:x}", self.tmraclk()))}
        if self.tmraen() != 0 { try!(write!(f, " tmraen"))}
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


