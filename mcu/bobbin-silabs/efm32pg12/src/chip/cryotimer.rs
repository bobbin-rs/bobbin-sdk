//! CRYOTIMER
#[allow(unused_imports)] use bobbin_common::*;

periph!(CRYOTIMER, Cryotimer, 0x4001e000);

#[doc="CRYOTIMER"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cryotimer(pub usize);
impl Cryotimer {
    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        (self.0 + 0x0) as *mut Ctrl
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
           self.ctrl_mut()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        unsafe {
            read_volatile(self.ctrl_ptr())
        }
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(Ctrl(0)));
        }
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrl_mut(), f(self.ctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PERIODSEL register."]
    #[inline] pub fn periodsel_mut(&self) -> *mut Periodsel { 
        (self.0 + 0x4) as *mut Periodsel
    }

    #[doc="Get the *const pointer for the PERIODSEL register."]
    #[inline] pub fn periodsel_ptr(&self) -> *const Periodsel { 
           self.periodsel_mut()
    }

    #[doc="Read the PERIODSEL register."]
    #[inline] pub fn periodsel(&self) -> Periodsel { 
        unsafe {
            read_volatile(self.periodsel_ptr())
        }
    }

    #[doc="Write the PERIODSEL register."]
    #[inline] pub fn set_periodsel<F: FnOnce(Periodsel) -> Periodsel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.periodsel_mut(), f(Periodsel(0)));
        }
        self
    }

    #[doc="Modify the PERIODSEL register."]
    #[inline] pub fn with_periodsel<F: FnOnce(Periodsel) -> Periodsel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.periodsel_mut(), f(self.periodsel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CNT register."]
    #[inline] pub fn cnt_mut(&self) -> *mut Cnt { 
        (self.0 + 0x8) as *mut Cnt
    }

    #[doc="Get the *const pointer for the CNT register."]
    #[inline] pub fn cnt_ptr(&self) -> *const Cnt { 
           self.cnt_mut()
    }

    #[doc="Read the CNT register."]
    #[inline] pub fn cnt(&self) -> Cnt { 
        unsafe {
            read_volatile(self.cnt_ptr())
        }
    }

    #[doc="Get the *mut pointer for the EM4WUEN register."]
    #[inline] pub fn em4wuen_mut(&self) -> *mut Em4wuen { 
        (self.0 + 0xc) as *mut Em4wuen
    }

    #[doc="Get the *const pointer for the EM4WUEN register."]
    #[inline] pub fn em4wuen_ptr(&self) -> *const Em4wuen { 
           self.em4wuen_mut()
    }

    #[doc="Read the EM4WUEN register."]
    #[inline] pub fn em4wuen(&self) -> Em4wuen { 
        unsafe {
            read_volatile(self.em4wuen_ptr())
        }
    }

    #[doc="Write the EM4WUEN register."]
    #[inline] pub fn set_em4wuen<F: FnOnce(Em4wuen) -> Em4wuen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.em4wuen_mut(), f(Em4wuen(0)));
        }
        self
    }

    #[doc="Modify the EM4WUEN register."]
    #[inline] pub fn with_em4wuen<F: FnOnce(Em4wuen) -> Em4wuen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.em4wuen_mut(), f(self.em4wuen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IF register."]
    #[inline] pub fn if_mut(&self) -> *mut If { 
        (self.0 + 0x10) as *mut If
    }

    #[doc="Get the *const pointer for the IF register."]
    #[inline] pub fn if_ptr(&self) -> *const If { 
           self.if_mut()
    }

    #[doc="Read the IF register."]
    #[inline] pub fn _if(&self) -> If { 
        unsafe {
            read_volatile(self.if_ptr())
        }
    }

    #[doc="Get the *mut pointer for the IFS register."]
    #[inline] pub fn ifs_mut(&self) -> *mut Ifs { 
        (self.0 + 0x14) as *mut Ifs
    }

    #[doc="Get the *const pointer for the IFS register."]
    #[inline] pub fn ifs_ptr(&self) -> *const Ifs { 
           self.ifs_mut()
    }

    #[doc="Write the IFS register."]
    #[inline] pub fn set_ifs<F: FnOnce(Ifs) -> Ifs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifs_mut(), f(Ifs(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IFC register."]
    #[inline] pub fn ifc_mut(&self) -> *mut Ifc { 
        (self.0 + 0x18) as *mut Ifc
    }

    #[doc="Get the *const pointer for the IFC register."]
    #[inline] pub fn ifc_ptr(&self) -> *const Ifc { 
           self.ifc_mut()
    }

    #[doc="Write the IFC register."]
    #[inline] pub fn set_ifc<F: FnOnce(Ifc) -> Ifc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifc_mut(), f(Ifc(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IEN register."]
    #[inline] pub fn ien_mut(&self) -> *mut Ien { 
        (self.0 + 0x1c) as *mut Ien
    }

    #[doc="Get the *const pointer for the IEN register."]
    #[inline] pub fn ien_ptr(&self) -> *const Ien { 
           self.ien_mut()
    }

    #[doc="Read the IEN register."]
    #[inline] pub fn ien(&self) -> Ien { 
        unsafe {
            read_volatile(self.ien_ptr())
        }
    }

    #[doc="Write the IEN register."]
    #[inline] pub fn set_ien<F: FnOnce(Ien) -> Ien>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ien_mut(), f(Ien(0)));
        }
        self
    }

    #[doc="Modify the IEN register."]
    #[inline] pub fn with_ien<F: FnOnce(Ien) -> Ien>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ien_mut(), f(self.ien()));
        }
        self
    }

}

#[doc="Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Enable CRYOTIMER"]
    #[inline] pub fn en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Debug Mode Run Enable"]
    #[inline] pub fn debugrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DEBUGRUN != 0"]
    #[inline] pub fn test_debugrun(&self) -> bool {
        self.debugrun() != 0
    }

    #[doc="Sets the DEBUGRUN field."]
    #[inline] pub fn set_debugrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Select Low frequency oscillator"]
    #[inline] pub fn oscsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if OSCSEL != 0"]
    #[inline] pub fn test_oscsel(&self) -> bool {
        self.oscsel() != 0
    }

    #[doc="Sets the OSCSEL field."]
    #[inline] pub fn set_oscsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Prescaler Setting"]
    #[inline] pub fn presc(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if PRESC != 0"]
    #[inline] pub fn test_presc(&self) -> bool {
        self.presc() != 0
    }

    #[doc="Sets the PRESC field."]
    #[inline] pub fn set_presc<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
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
        if self.en() != 0 { try!(write!(f, " en"))}
        if self.debugrun() != 0 { try!(write!(f, " debugrun"))}
        if self.oscsel() != 0 { try!(write!(f, " oscsel=0x{:x}", self.oscsel()))}
        if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Duration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Periodsel(pub u32);
impl Periodsel {
    #[doc="Interrupts/Wakeup events period setting"]
    #[inline] pub fn periodsel(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if PERIODSEL != 0"]
    #[inline] pub fn test_periodsel(&self) -> bool {
        self.periodsel() != 0
    }

    #[doc="Sets the PERIODSEL field."]
    #[inline] pub fn set_periodsel<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Periodsel {
    #[inline]
    fn from(other: u32) -> Self {
         Periodsel(other)
    }
}

impl ::core::fmt::Display for Periodsel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Periodsel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.periodsel() != 0 { try!(write!(f, " periodsel=0x{:x}", self.periodsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="Counter Value"]
    #[inline] pub fn cnt(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cnt(other)
    }
}

impl ::core::fmt::Display for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Wake Up Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Em4wuen(pub u32);
impl Em4wuen {
    #[doc="EM4 Wake-up enable"]
    #[inline] pub fn em4wu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EM4WU != 0"]
    #[inline] pub fn test_em4wu(&self) -> bool {
        self.em4wu() != 0
    }

    #[doc="Sets the EM4WU field."]
    #[inline] pub fn set_em4wu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Em4wuen {
    #[inline]
    fn from(other: u32) -> Self {
         Em4wuen(other)
    }
}

impl ::core::fmt::Display for Em4wuen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Em4wuen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.em4wu() != 0 { try!(write!(f, " em4wu"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct If(pub u32);
impl If {
    #[doc="Wakeup event/Interrupt"]
    #[inline] pub fn period(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PERIOD != 0"]
    #[inline] pub fn test_period(&self) -> bool {
        self.period() != 0
    }

    #[doc="Sets the PERIOD field."]
    #[inline] pub fn set_period<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for If {
    #[inline]
    fn from(other: u32) -> Self {
         If(other)
    }
}

impl ::core::fmt::Display for If {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for If {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.period() != 0 { try!(write!(f, " period"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Set Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifs(pub u32);
impl Ifs {
    #[doc="Set PERIOD Interrupt Flag"]
    #[inline] pub fn period(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PERIOD != 0"]
    #[inline] pub fn test_period(&self) -> bool {
        self.period() != 0
    }

    #[doc="Sets the PERIOD field."]
    #[inline] pub fn set_period<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ifs {
    #[inline]
    fn from(other: u32) -> Self {
         Ifs(other)
    }
}

impl ::core::fmt::Display for Ifs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.period() != 0 { try!(write!(f, " period"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Clear Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifc(pub u32);
impl Ifc {
    #[doc="Clear PERIOD Interrupt Flag"]
    #[inline] pub fn period(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PERIOD != 0"]
    #[inline] pub fn test_period(&self) -> bool {
        self.period() != 0
    }

    #[doc="Sets the PERIOD field."]
    #[inline] pub fn set_period<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ifc {
    #[inline]
    fn from(other: u32) -> Self {
         Ifc(other)
    }
}

impl ::core::fmt::Display for Ifc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.period() != 0 { try!(write!(f, " period"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ien(pub u32);
impl Ien {
    #[doc="PERIOD Interrupt Enable"]
    #[inline] pub fn period(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PERIOD != 0"]
    #[inline] pub fn test_period(&self) -> bool {
        self.period() != 0
    }

    #[doc="Sets the PERIOD field."]
    #[inline] pub fn set_period<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ien {
    #[inline]
    fn from(other: u32) -> Self {
         Ien(other)
    }
}

impl ::core::fmt::Display for Ien {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ien {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.period() != 0 { try!(write!(f, " period"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


