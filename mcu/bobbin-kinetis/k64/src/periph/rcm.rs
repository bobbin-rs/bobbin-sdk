#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Reset Control Module"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RcmPeriph(pub usize);
impl RcmPeriph {
    #[doc="Get the SRS0 Register."]
    #[inline] pub fn srs0_reg(&self) -> Register<Srs0> { 
        Register::new(self.0 as *mut Srs0, 0x0)
    }

    #[doc="Get the *mut pointer for the SRS0 register."]
    #[inline] pub fn srs0_mut(&self) -> *mut Srs0 { 
        self.srs0_reg().ptr()
    }

    #[doc="Get the *const pointer for the SRS0 register."]
    #[inline] pub fn srs0_ptr(&self) -> *const Srs0 { 
        self.srs0_reg().ptr()
    }

    #[doc="Read the SRS0 register."]
    #[inline] pub fn srs0(&self) -> Srs0 { 
        self.srs0_reg().read()
    }

    #[doc="Get the SRS1 Register."]
    #[inline] pub fn srs1_reg(&self) -> Register<Srs1> { 
        Register::new(self.0 as *mut Srs1, 0x1)
    }

    #[doc="Get the *mut pointer for the SRS1 register."]
    #[inline] pub fn srs1_mut(&self) -> *mut Srs1 { 
        self.srs1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SRS1 register."]
    #[inline] pub fn srs1_ptr(&self) -> *const Srs1 { 
        self.srs1_reg().ptr()
    }

    #[doc="Read the SRS1 register."]
    #[inline] pub fn srs1(&self) -> Srs1 { 
        self.srs1_reg().read()
    }

    #[doc="Get the RPFC Register."]
    #[inline] pub fn rpfc_reg(&self) -> Register<Rpfc> { 
        Register::new(self.0 as *mut Rpfc, 0x4)
    }

    #[doc="Get the *mut pointer for the RPFC register."]
    #[inline] pub fn rpfc_mut(&self) -> *mut Rpfc { 
        self.rpfc_reg().ptr()
    }

    #[doc="Get the *const pointer for the RPFC register."]
    #[inline] pub fn rpfc_ptr(&self) -> *const Rpfc { 
        self.rpfc_reg().ptr()
    }

    #[doc="Read the RPFC register."]
    #[inline] pub fn rpfc(&self) -> Rpfc { 
        self.rpfc_reg().read()
    }

    #[doc="Write the RPFC register."]
    #[inline] pub fn write_rpfc(&self, value: Rpfc) -> &Self { 
        self.rpfc_reg().write(value);
        self
    }

    #[doc="Set the RPFC register."]
    #[inline] pub fn set_rpfc<F: FnOnce(Rpfc) -> Rpfc>(&self, f: F) -> &Self {
        self.rpfc_reg().set(f);
        self
    }

    #[doc="Modify the RPFC register."]
    #[inline] pub fn with_rpfc<F: FnOnce(Rpfc) -> Rpfc>(&self, f: F) -> &Self {
        self.rpfc_reg().with(f);
        self
    }

    #[doc="Get the RPFW Register."]
    #[inline] pub fn rpfw_reg(&self) -> Register<Rpfw> { 
        Register::new(self.0 as *mut Rpfw, 0x5)
    }

    #[doc="Get the *mut pointer for the RPFW register."]
    #[inline] pub fn rpfw_mut(&self) -> *mut Rpfw { 
        self.rpfw_reg().ptr()
    }

    #[doc="Get the *const pointer for the RPFW register."]
    #[inline] pub fn rpfw_ptr(&self) -> *const Rpfw { 
        self.rpfw_reg().ptr()
    }

    #[doc="Read the RPFW register."]
    #[inline] pub fn rpfw(&self) -> Rpfw { 
        self.rpfw_reg().read()
    }

    #[doc="Write the RPFW register."]
    #[inline] pub fn write_rpfw(&self, value: Rpfw) -> &Self { 
        self.rpfw_reg().write(value);
        self
    }

    #[doc="Set the RPFW register."]
    #[inline] pub fn set_rpfw<F: FnOnce(Rpfw) -> Rpfw>(&self, f: F) -> &Self {
        self.rpfw_reg().set(f);
        self
    }

    #[doc="Modify the RPFW register."]
    #[inline] pub fn with_rpfw<F: FnOnce(Rpfw) -> Rpfw>(&self, f: F) -> &Self {
        self.rpfw_reg().with(f);
        self
    }

    #[doc="Get the MR Register."]
    #[inline] pub fn mr_reg(&self) -> Register<Mr> { 
        Register::new(self.0 as *mut Mr, 0x7)
    }

    #[doc="Get the *mut pointer for the MR register."]
    #[inline] pub fn mr_mut(&self) -> *mut Mr { 
        self.mr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MR register."]
    #[inline] pub fn mr_ptr(&self) -> *const Mr { 
        self.mr_reg().ptr()
    }

    #[doc="Read the MR register."]
    #[inline] pub fn mr(&self) -> Mr { 
        self.mr_reg().read()
    }

}

#[doc="System Reset Status Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srs0(pub u8);
impl Srs0 {
    #[doc="Low Leakage Wakeup Reset"]
    #[inline] pub fn wakeup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WAKEUP != 0"]
    #[inline] pub fn test_wakeup(&self) -> bool {
        self.wakeup() != 0
    }

    #[doc="Sets the WAKEUP field."]
    #[inline] pub fn set_wakeup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low-Voltage Detect Reset"]
    #[inline] pub fn lvd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LVD != 0"]
    #[inline] pub fn test_lvd(&self) -> bool {
        self.lvd() != 0
    }

    #[doc="Sets the LVD field."]
    #[inline] pub fn set_lvd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Loss-of-Clock Reset"]
    #[inline] pub fn loc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LOC != 0"]
    #[inline] pub fn test_loc(&self) -> bool {
        self.loc() != 0
    }

    #[doc="Sets the LOC field."]
    #[inline] pub fn set_loc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Loss-of-Lock Reset"]
    #[inline] pub fn lol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LOL != 0"]
    #[inline] pub fn test_lol(&self) -> bool {
        self.lol() != 0
    }

    #[doc="Sets the LOL field."]
    #[inline] pub fn set_lol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Watchdog"]
    #[inline] pub fn wdog(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WDOG != 0"]
    #[inline] pub fn test_wdog(&self) -> bool {
        self.wdog() != 0
    }

    #[doc="Sets the WDOG field."]
    #[inline] pub fn set_wdog<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="External Reset Pin"]
    #[inline] pub fn pin(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PIN != 0"]
    #[inline] pub fn test_pin(&self) -> bool {
        self.pin() != 0
    }

    #[doc="Sets the PIN field."]
    #[inline] pub fn set_pin<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Power-On Reset"]
    #[inline] pub fn por(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if POR != 0"]
    #[inline] pub fn test_por(&self) -> bool {
        self.por() != 0
    }

    #[doc="Sets the POR field."]
    #[inline] pub fn set_por<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Srs0 {
    #[inline]
    fn from(other: u8) -> Self {
         Srs0(other)
    }
}

impl ::core::fmt::Display for Srs0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srs0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
        if self.lvd() != 0 { try!(write!(f, " lvd"))}
        if self.loc() != 0 { try!(write!(f, " loc"))}
        if self.lol() != 0 { try!(write!(f, " lol"))}
        if self.wdog() != 0 { try!(write!(f, " wdog"))}
        if self.pin() != 0 { try!(write!(f, " pin"))}
        if self.por() != 0 { try!(write!(f, " por"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Reset Status Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srs1(pub u8);
impl Srs1 {
    #[doc="JTAG Generated Reset"]
    #[inline] pub fn jtag(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if JTAG != 0"]
    #[inline] pub fn test_jtag(&self) -> bool {
        self.jtag() != 0
    }

    #[doc="Sets the JTAG field."]
    #[inline] pub fn set_jtag<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Core Lockup"]
    #[inline] pub fn lockup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LOCKUP != 0"]
    #[inline] pub fn test_lockup(&self) -> bool {
        self.lockup() != 0
    }

    #[doc="Sets the LOCKUP field."]
    #[inline] pub fn set_lockup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Software"]
    #[inline] pub fn sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SW != 0"]
    #[inline] pub fn test_sw(&self) -> bool {
        self.sw() != 0
    }

    #[doc="Sets the SW field."]
    #[inline] pub fn set_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="MDM-AP System Reset Request"]
    #[inline] pub fn mdm_ap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MDM_AP != 0"]
    #[inline] pub fn test_mdm_ap(&self) -> bool {
        self.mdm_ap() != 0
    }

    #[doc="Sets the MDM_AP field."]
    #[inline] pub fn set_mdm_ap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EzPort Reset"]
    #[inline] pub fn ezpt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EZPT != 0"]
    #[inline] pub fn test_ezpt(&self) -> bool {
        self.ezpt() != 0
    }

    #[doc="Sets the EZPT field."]
    #[inline] pub fn set_ezpt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Stop Mode Acknowledge Error Reset"]
    #[inline] pub fn sackerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SACKERR != 0"]
    #[inline] pub fn test_sackerr(&self) -> bool {
        self.sackerr() != 0
    }

    #[doc="Sets the SACKERR field."]
    #[inline] pub fn set_sackerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Srs1 {
    #[inline]
    fn from(other: u8) -> Self {
         Srs1(other)
    }
}

impl ::core::fmt::Display for Srs1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srs1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.jtag() != 0 { try!(write!(f, " jtag"))}
        if self.lockup() != 0 { try!(write!(f, " lockup"))}
        if self.sw() != 0 { try!(write!(f, " sw"))}
        if self.mdm_ap() != 0 { try!(write!(f, " mdm_ap"))}
        if self.ezpt() != 0 { try!(write!(f, " ezpt"))}
        if self.sackerr() != 0 { try!(write!(f, " sackerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reset Pin Filter Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rpfc(pub u8);
impl Rpfc {
    #[doc="Reset Pin Filter Select in Run and Wait Modes"]
    #[inline] pub fn rstfltsrw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if RSTFLTSRW != 0"]
    #[inline] pub fn test_rstfltsrw(&self) -> bool {
        self.rstfltsrw() != 0
    }

    #[doc="Sets the RSTFLTSRW field."]
    #[inline] pub fn set_rstfltsrw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Reset Pin Filter Select in Stop Mode"]
    #[inline] pub fn rstfltss(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RSTFLTSS != 0"]
    #[inline] pub fn test_rstfltss(&self) -> bool {
        self.rstfltss() != 0
    }

    #[doc="Sets the RSTFLTSS field."]
    #[inline] pub fn set_rstfltss<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Rpfc {
    #[inline]
    fn from(other: u8) -> Self {
         Rpfc(other)
    }
}

impl ::core::fmt::Display for Rpfc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rpfc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rstfltsrw() != 0 { try!(write!(f, " rstfltsrw=0x{:x}", self.rstfltsrw()))}
        if self.rstfltss() != 0 { try!(write!(f, " rstfltss"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reset Pin Filter Width register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rpfw(pub u8);
impl Rpfw {
    #[doc="Reset Pin Filter Bus Clock Select"]
    #[inline] pub fn rstfltsel(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if RSTFLTSEL != 0"]
    #[inline] pub fn test_rstfltsel(&self) -> bool {
        self.rstfltsel() != 0
    }

    #[doc="Sets the RSTFLTSEL field."]
    #[inline] pub fn set_rstfltsel<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Rpfw {
    #[inline]
    fn from(other: u8) -> Self {
         Rpfw(other)
    }
}

impl ::core::fmt::Display for Rpfw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rpfw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rstfltsel() != 0 { try!(write!(f, " rstfltsel=0x{:x}", self.rstfltsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Mode Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mr(pub u8);
impl Mr {
    #[doc="EZP_MS_B pin state"]
    #[inline] pub fn ezp_ms(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EZP_MS != 0"]
    #[inline] pub fn test_ezp_ms(&self) -> bool {
        self.ezp_ms() != 0
    }

    #[doc="Sets the EZP_MS field."]
    #[inline] pub fn set_ezp_ms<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Mr {
    #[inline]
    fn from(other: u8) -> Self {
         Mr(other)
    }
}

impl ::core::fmt::Display for Mr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ezp_ms() != 0 { try!(write!(f, " ezp_ms"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

