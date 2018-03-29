#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="System configuration controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SyscfgPeriph(pub usize);
impl SyscfgPeriph {
    #[doc="Get the *mut pointer for the MEMRMP register."]
    #[inline] pub fn memrmp_mut(&self) -> *mut Memrmp { 
        (self.0 + 0x0) as *mut Memrmp
    }

    #[doc="Get the *const pointer for the MEMRMP register."]
    #[inline] pub fn memrmp_ptr(&self) -> *const Memrmp { 
           self.memrmp_mut()
    }

    #[doc="Read the MEMRMP register."]
    #[inline] pub fn memrmp(&self) -> Memrmp { 
        unsafe {
            read_volatile(self.memrmp_ptr())
        }
    }

    #[doc="Write the MEMRMP register."]
    #[inline] pub fn set_memrmp<F: FnOnce(Memrmp) -> Memrmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.memrmp_mut(), f(Memrmp(0)));
        }
        self
    }

    #[doc="Modify the MEMRMP register."]
    #[inline] pub fn with_memrmp<F: FnOnce(Memrmp) -> Memrmp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.memrmp_mut(), f(self.memrmp()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_mut(&self) -> *mut Cfgr1 { 
        (self.0 + 0x4) as *mut Cfgr1
    }

    #[doc="Get the *const pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_ptr(&self) -> *const Cfgr1 { 
           self.cfgr1_mut()
    }

    #[doc="Read the CFGR1 register."]
    #[inline] pub fn cfgr1(&self) -> Cfgr1 { 
        unsafe {
            read_volatile(self.cfgr1_ptr())
        }
    }

    #[doc="Write the CFGR1 register."]
    #[inline] pub fn set_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr1_mut(), f(Cfgr1(0)));
        }
        self
    }

    #[doc="Modify the CFGR1 register."]
    #[inline] pub fn with_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr1_mut(), f(self.cfgr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_mut(&self) -> *mut Exticr1 { 
        (self.0 + 0x8) as *mut Exticr1
    }

    #[doc="Get the *const pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_ptr(&self) -> *const Exticr1 { 
           self.exticr1_mut()
    }

    #[doc="Read the EXTICR1 register."]
    #[inline] pub fn exticr1(&self) -> Exticr1 { 
        unsafe {
            read_volatile(self.exticr1_ptr())
        }
    }

    #[doc="Write the EXTICR1 register."]
    #[inline] pub fn set_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr1_mut(), f(Exticr1(0)));
        }
        self
    }

    #[doc="Modify the EXTICR1 register."]
    #[inline] pub fn with_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr1_mut(), f(self.exticr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_mut(&self) -> *mut Exticr2 { 
        (self.0 + 0xc) as *mut Exticr2
    }

    #[doc="Get the *const pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_ptr(&self) -> *const Exticr2 { 
           self.exticr2_mut()
    }

    #[doc="Read the EXTICR2 register."]
    #[inline] pub fn exticr2(&self) -> Exticr2 { 
        unsafe {
            read_volatile(self.exticr2_ptr())
        }
    }

    #[doc="Write the EXTICR2 register."]
    #[inline] pub fn set_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr2_mut(), f(Exticr2(0)));
        }
        self
    }

    #[doc="Modify the EXTICR2 register."]
    #[inline] pub fn with_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr2_mut(), f(self.exticr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_mut(&self) -> *mut Exticr3 { 
        (self.0 + 0x10) as *mut Exticr3
    }

    #[doc="Get the *const pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_ptr(&self) -> *const Exticr3 { 
           self.exticr3_mut()
    }

    #[doc="Read the EXTICR3 register."]
    #[inline] pub fn exticr3(&self) -> Exticr3 { 
        unsafe {
            read_volatile(self.exticr3_ptr())
        }
    }

    #[doc="Write the EXTICR3 register."]
    #[inline] pub fn set_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr3_mut(), f(Exticr3(0)));
        }
        self
    }

    #[doc="Modify the EXTICR3 register."]
    #[inline] pub fn with_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr3_mut(), f(self.exticr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_mut(&self) -> *mut Exticr4 { 
        (self.0 + 0x14) as *mut Exticr4
    }

    #[doc="Get the *const pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_ptr(&self) -> *const Exticr4 { 
           self.exticr4_mut()
    }

    #[doc="Read the EXTICR4 register."]
    #[inline] pub fn exticr4(&self) -> Exticr4 { 
        unsafe {
            read_volatile(self.exticr4_ptr())
        }
    }

    #[doc="Write the EXTICR4 register."]
    #[inline] pub fn set_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr4_mut(), f(Exticr4(0)));
        }
        self
    }

    #[doc="Modify the EXTICR4 register."]
    #[inline] pub fn with_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr4_mut(), f(self.exticr4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCSR register."]
    #[inline] pub fn scsr_mut(&self) -> *mut Scsr { 
        (self.0 + 0x18) as *mut Scsr
    }

    #[doc="Get the *const pointer for the SCSR register."]
    #[inline] pub fn scsr_ptr(&self) -> *const Scsr { 
           self.scsr_mut()
    }

    #[doc="Read the SCSR register."]
    #[inline] pub fn scsr(&self) -> Scsr { 
        unsafe {
            read_volatile(self.scsr_ptr())
        }
    }

    #[doc="Write the SCSR register."]
    #[inline] pub fn set_scsr<F: FnOnce(Scsr) -> Scsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scsr_mut(), f(Scsr(0)));
        }
        self
    }

    #[doc="Modify the SCSR register."]
    #[inline] pub fn with_scsr<F: FnOnce(Scsr) -> Scsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scsr_mut(), f(self.scsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGR2 register."]
    #[inline] pub fn cfgr2_mut(&self) -> *mut Cfgr2 { 
        (self.0 + 0x1c) as *mut Cfgr2
    }

    #[doc="Get the *const pointer for the CFGR2 register."]
    #[inline] pub fn cfgr2_ptr(&self) -> *const Cfgr2 { 
           self.cfgr2_mut()
    }

    #[doc="Read the CFGR2 register."]
    #[inline] pub fn cfgr2(&self) -> Cfgr2 { 
        unsafe {
            read_volatile(self.cfgr2_ptr())
        }
    }

    #[doc="Write the CFGR2 register."]
    #[inline] pub fn set_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr2_mut(), f(Cfgr2(0)));
        }
        self
    }

    #[doc="Modify the CFGR2 register."]
    #[inline] pub fn with_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr2_mut(), f(self.cfgr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SWPR register."]
    #[inline] pub fn swpr_mut(&self) -> *mut Swpr { 
        (self.0 + 0x20) as *mut Swpr
    }

    #[doc="Get the *const pointer for the SWPR register."]
    #[inline] pub fn swpr_ptr(&self) -> *const Swpr { 
           self.swpr_mut()
    }

    #[doc="Write the SWPR register."]
    #[inline] pub fn set_swpr<F: FnOnce(Swpr) -> Swpr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swpr_mut(), f(Swpr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SKR register."]
    #[inline] pub fn skr_mut(&self) -> *mut Skr { 
        (self.0 + 0x24) as *mut Skr
    }

    #[doc="Get the *const pointer for the SKR register."]
    #[inline] pub fn skr_ptr(&self) -> *const Skr { 
           self.skr_mut()
    }

    #[doc="Write the SKR register."]
    #[inline] pub fn set_skr<F: FnOnce(Skr) -> Skr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.skr_mut(), f(Skr(0)));
        }
        self
    }

}

#[doc="memory remap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Memrmp(pub u32);
impl Memrmp {
    #[doc="Flash Bank mode selection"]
    #[inline] pub fn fb_mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FB_MODE != 0"]
    #[inline] pub fn test_fb_mode(&self) -> bool {
        self.fb_mode() != 0
    }

    #[doc="Sets the FB_MODE field."]
    #[inline] pub fn set_fb_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="QUADSPI memory mapping swap"]
    #[inline] pub fn qfs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if QFS != 0"]
    #[inline] pub fn test_qfs(&self) -> bool {
        self.qfs() != 0
    }

    #[doc="Sets the QFS field."]
    #[inline] pub fn set_qfs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Memory mapping selection"]
    #[inline] pub fn mem_mode(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if MEM_MODE != 0"]
    #[inline] pub fn test_mem_mode(&self) -> bool {
        self.mem_mode() != 0
    }

    #[doc="Sets the MEM_MODE field."]
    #[inline] pub fn set_mem_mode<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Memrmp {
    #[inline]
    fn from(other: u32) -> Self {
         Memrmp(other)
    }
}

impl ::core::fmt::Display for Memrmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Memrmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fb_mode() != 0 { try!(write!(f, " fb_mode"))}
        if self.qfs() != 0 { try!(write!(f, " qfs"))}
        if self.mem_mode() != 0 { try!(write!(f, " mem_mode=0x{:x}", self.mem_mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="configuration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
    #[doc="Floating Point Unit interrupts enable bits"]
    #[inline] pub fn fpu_ie(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
    }

    #[doc="Returns true if FPU_IE != 0"]
    #[inline] pub fn test_fpu_ie(&self) -> bool {
        self.fpu_ie() != 0
    }

    #[doc="Sets the FPU_IE field."]
    #[inline] pub fn set_fpu_ie<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="I2C3 Fast-mode Plus driving capability activation"]
    #[inline] pub fn i2c3_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C3_FMP != 0"]
    #[inline] pub fn test_i2c3_fmp(&self) -> bool {
        self.i2c3_fmp() != 0
    }

    #[doc="Sets the I2C3_FMP field."]
    #[inline] pub fn set_i2c3_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I2C2 Fast-mode Plus driving capability activation"]
    #[inline] pub fn i2c2_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C2_FMP != 0"]
    #[inline] pub fn test_i2c2_fmp(&self) -> bool {
        self.i2c2_fmp() != 0
    }

    #[doc="Sets the I2C2_FMP field."]
    #[inline] pub fn set_i2c2_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="I2C1 Fast-mode Plus driving capability activation"]
    #[inline] pub fn i2c1_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if I2C1_FMP != 0"]
    #[inline] pub fn test_i2c1_fmp(&self) -> bool {
        self.i2c1_fmp() != 0
    }

    #[doc="Sets the I2C1_FMP field."]
    #[inline] pub fn set_i2c1_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline] pub fn i2c_pb9_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if I2C_PB9_FMP != 0"]
    #[inline] pub fn test_i2c_pb9_fmp(&self) -> bool {
        self.i2c_pb9_fmp() != 0
    }

    #[doc="Sets the I2C_PB9_FMP field."]
    #[inline] pub fn set_i2c_pb9_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline] pub fn i2c_pb8_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if I2C_PB8_FMP != 0"]
    #[inline] pub fn test_i2c_pb8_fmp(&self) -> bool {
        self.i2c_pb8_fmp() != 0
    }

    #[doc="Sets the I2C_PB8_FMP field."]
    #[inline] pub fn set_i2c_pb8_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline] pub fn i2c_pb7_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if I2C_PB7_FMP != 0"]
    #[inline] pub fn test_i2c_pb7_fmp(&self) -> bool {
        self.i2c_pb7_fmp() != 0
    }

    #[doc="Sets the I2C_PB7_FMP field."]
    #[inline] pub fn set_i2c_pb7_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline] pub fn i2c_pb6_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if I2C_PB6_FMP != 0"]
    #[inline] pub fn test_i2c_pb6_fmp(&self) -> bool {
        self.i2c_pb6_fmp() != 0
    }

    #[doc="Sets the I2C_PB6_FMP field."]
    #[inline] pub fn set_i2c_pb6_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="I/O analog switch voltage booster enable"]
    #[inline] pub fn boosten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BOOSTEN != 0"]
    #[inline] pub fn test_boosten(&self) -> bool {
        self.boosten() != 0
    }

    #[doc="Sets the BOOSTEN field."]
    #[inline] pub fn set_boosten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Firewall disable"]
    #[inline] pub fn fwdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FWDIS != 0"]
    #[inline] pub fn test_fwdis(&self) -> bool {
        self.fwdis() != 0
    }

    #[doc="Sets the FWDIS field."]
    #[inline] pub fn set_fwdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr1(other)
    }
}

impl ::core::fmt::Display for Cfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fpu_ie() != 0 { try!(write!(f, " fpu_ie=0x{:x}", self.fpu_ie()))}
        if self.i2c3_fmp() != 0 { try!(write!(f, " i2c3_fmp"))}
        if self.i2c2_fmp() != 0 { try!(write!(f, " i2c2_fmp"))}
        if self.i2c1_fmp() != 0 { try!(write!(f, " i2c1_fmp"))}
        if self.i2c_pb9_fmp() != 0 { try!(write!(f, " i2c_pb9_fmp"))}
        if self.i2c_pb8_fmp() != 0 { try!(write!(f, " i2c_pb8_fmp"))}
        if self.i2c_pb7_fmp() != 0 { try!(write!(f, " i2c_pb7_fmp"))}
        if self.i2c_pb6_fmp() != 0 { try!(write!(f, " i2c_pb6_fmp"))}
        if self.boosten() != 0 { try!(write!(f, " boosten"))}
        if self.fwdis() != 0 { try!(write!(f, " fwdis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
    #[doc="EXTI 3 configuration bits"]
    #[inline] pub fn exti3(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if EXTI3 != 0"]
    #[inline] pub fn test_exti3(&self) -> bool {
        self.exti3() != 0
    }

    #[doc="Sets the EXTI3 field."]
    #[inline] pub fn set_exti3<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI 2 configuration bits"]
    #[inline] pub fn exti2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if EXTI2 != 0"]
    #[inline] pub fn test_exti2(&self) -> bool {
        self.exti2() != 0
    }

    #[doc="Sets the EXTI2 field."]
    #[inline] pub fn set_exti2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI 1 configuration bits"]
    #[inline] pub fn exti1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if EXTI1 != 0"]
    #[inline] pub fn test_exti1(&self) -> bool {
        self.exti1() != 0
    }

    #[doc="Sets the EXTI1 field."]
    #[inline] pub fn set_exti1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI 0 configuration bits"]
    #[inline] pub fn exti0(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXTI0 != 0"]
    #[inline] pub fn test_exti0(&self) -> bool {
        self.exti0() != 0
    }

    #[doc="Sets the EXTI0 field."]
    #[inline] pub fn set_exti0<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr1(other)
    }
}

impl ::core::fmt::Display for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti3() != 0 { try!(write!(f, " exti3=0x{:x}", self.exti3()))}
        if self.exti2() != 0 { try!(write!(f, " exti2=0x{:x}", self.exti2()))}
        if self.exti1() != 0 { try!(write!(f, " exti1=0x{:x}", self.exti1()))}
        if self.exti0() != 0 { try!(write!(f, " exti0=0x{:x}", self.exti0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr2(pub u32);
impl Exticr2 {
    #[doc="EXTI 7 configuration bits"]
    #[inline] pub fn exti7(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if EXTI7 != 0"]
    #[inline] pub fn test_exti7(&self) -> bool {
        self.exti7() != 0
    }

    #[doc="Sets the EXTI7 field."]
    #[inline] pub fn set_exti7<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI 6 configuration bits"]
    #[inline] pub fn exti6(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if EXTI6 != 0"]
    #[inline] pub fn test_exti6(&self) -> bool {
        self.exti6() != 0
    }

    #[doc="Sets the EXTI6 field."]
    #[inline] pub fn set_exti6<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI 5 configuration bits"]
    #[inline] pub fn exti5(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if EXTI5 != 0"]
    #[inline] pub fn test_exti5(&self) -> bool {
        self.exti5() != 0
    }

    #[doc="Sets the EXTI5 field."]
    #[inline] pub fn set_exti5<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI 4 configuration bits"]
    #[inline] pub fn exti4(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXTI4 != 0"]
    #[inline] pub fn test_exti4(&self) -> bool {
        self.exti4() != 0
    }

    #[doc="Sets the EXTI4 field."]
    #[inline] pub fn set_exti4<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr2(other)
    }
}

impl ::core::fmt::Display for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti7() != 0 { try!(write!(f, " exti7=0x{:x}", self.exti7()))}
        if self.exti6() != 0 { try!(write!(f, " exti6=0x{:x}", self.exti6()))}
        if self.exti5() != 0 { try!(write!(f, " exti5=0x{:x}", self.exti5()))}
        if self.exti4() != 0 { try!(write!(f, " exti4=0x{:x}", self.exti4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr3(pub u32);
impl Exticr3 {
    #[doc="EXTI 11 configuration bits"]
    #[inline] pub fn exti11(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if EXTI11 != 0"]
    #[inline] pub fn test_exti11(&self) -> bool {
        self.exti11() != 0
    }

    #[doc="Sets the EXTI11 field."]
    #[inline] pub fn set_exti11<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI 10 configuration bits"]
    #[inline] pub fn exti10(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if EXTI10 != 0"]
    #[inline] pub fn test_exti10(&self) -> bool {
        self.exti10() != 0
    }

    #[doc="Sets the EXTI10 field."]
    #[inline] pub fn set_exti10<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI 9 configuration bits"]
    #[inline] pub fn exti9(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if EXTI9 != 0"]
    #[inline] pub fn test_exti9(&self) -> bool {
        self.exti9() != 0
    }

    #[doc="Sets the EXTI9 field."]
    #[inline] pub fn set_exti9<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI 8 configuration bits"]
    #[inline] pub fn exti8(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXTI8 != 0"]
    #[inline] pub fn test_exti8(&self) -> bool {
        self.exti8() != 0
    }

    #[doc="Sets the EXTI8 field."]
    #[inline] pub fn set_exti8<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr3(other)
    }
}

impl ::core::fmt::Display for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti11() != 0 { try!(write!(f, " exti11=0x{:x}", self.exti11()))}
        if self.exti10() != 0 { try!(write!(f, " exti10=0x{:x}", self.exti10()))}
        if self.exti9() != 0 { try!(write!(f, " exti9=0x{:x}", self.exti9()))}
        if self.exti8() != 0 { try!(write!(f, " exti8=0x{:x}", self.exti8()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr4(pub u32);
impl Exticr4 {
    #[doc="EXTI15 configuration bits"]
    #[inline] pub fn exti15(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if EXTI15 != 0"]
    #[inline] pub fn test_exti15(&self) -> bool {
        self.exti15() != 0
    }

    #[doc="Sets the EXTI15 field."]
    #[inline] pub fn set_exti15<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI14 configuration bits"]
    #[inline] pub fn exti14(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if EXTI14 != 0"]
    #[inline] pub fn test_exti14(&self) -> bool {
        self.exti14() != 0
    }

    #[doc="Sets the EXTI14 field."]
    #[inline] pub fn set_exti14<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI13 configuration bits"]
    #[inline] pub fn exti13(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if EXTI13 != 0"]
    #[inline] pub fn test_exti13(&self) -> bool {
        self.exti13() != 0
    }

    #[doc="Sets the EXTI13 field."]
    #[inline] pub fn set_exti13<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI12 configuration bits"]
    #[inline] pub fn exti12(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXTI12 != 0"]
    #[inline] pub fn test_exti12(&self) -> bool {
        self.exti12() != 0
    }

    #[doc="Sets the EXTI12 field."]
    #[inline] pub fn set_exti12<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr4(other)
    }
}

impl ::core::fmt::Display for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti15() != 0 { try!(write!(f, " exti15=0x{:x}", self.exti15()))}
        if self.exti14() != 0 { try!(write!(f, " exti14=0x{:x}", self.exti14()))}
        if self.exti13() != 0 { try!(write!(f, " exti13=0x{:x}", self.exti13()))}
        if self.exti12() != 0 { try!(write!(f, " exti12=0x{:x}", self.exti12()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SCSR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scsr(pub u32);
impl Scsr {
    #[doc="SRAM2 busy by erase operation"]
    #[inline] pub fn sram2bsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SRAM2BSY != 0"]
    #[inline] pub fn test_sram2bsy(&self) -> bool {
        self.sram2bsy() != 0
    }

    #[doc="Sets the SRAM2BSY field."]
    #[inline] pub fn set_sram2bsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SRAM2 Erase"]
    #[inline] pub fn sram2er(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SRAM2ER != 0"]
    #[inline] pub fn test_sram2er(&self) -> bool {
        self.sram2er() != 0
    }

    #[doc="Sets the SRAM2ER field."]
    #[inline] pub fn set_sram2er<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Scsr {
    #[inline]
    fn from(other: u32) -> Self {
         Scsr(other)
    }
}

impl ::core::fmt::Display for Scsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sram2bsy() != 0 { try!(write!(f, " sram2bsy"))}
        if self.sram2er() != 0 { try!(write!(f, " sram2er"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CFGR2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
    #[doc="SRAM2 parity error flag"]
    #[inline] pub fn spf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SPF != 0"]
    #[inline] pub fn test_spf(&self) -> bool {
        self.spf() != 0
    }

    #[doc="Sets the SPF field."]
    #[inline] pub fn set_spf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ECC Lock"]
    #[inline] pub fn eccl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ECCL != 0"]
    #[inline] pub fn test_eccl(&self) -> bool {
        self.eccl() != 0
    }

    #[doc="Sets the ECCL field."]
    #[inline] pub fn set_eccl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PVD lock enable bit"]
    #[inline] pub fn pvdl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PVDL != 0"]
    #[inline] pub fn test_pvdl(&self) -> bool {
        self.pvdl() != 0
    }

    #[doc="Sets the PVDL field."]
    #[inline] pub fn set_pvdl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SRAM2 parity lock bit"]
    #[inline] pub fn spl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SPL != 0"]
    #[inline] pub fn test_spl(&self) -> bool {
        self.spl() != 0
    }

    #[doc="Sets the SPL field."]
    #[inline] pub fn set_spl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CortexÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¾ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¾ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¾ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¾ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¾ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢-M4 LOCKUP (Hardfault) output enable bit"]
    #[inline] pub fn cll(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CLL != 0"]
    #[inline] pub fn test_cll(&self) -> bool {
        self.cll() != 0
    }

    #[doc="Sets the CLL field."]
    #[inline] pub fn set_cll<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr2(other)
    }
}

impl ::core::fmt::Display for Cfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.spf() != 0 { try!(write!(f, " spf"))}
        if self.eccl() != 0 { try!(write!(f, " eccl"))}
        if self.pvdl() != 0 { try!(write!(f, " pvdl"))}
        if self.spl() != 0 { try!(write!(f, " spl"))}
        if self.cll() != 0 { try!(write!(f, " cll"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SWPR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swpr(pub u32);
impl Swpr {
    #[doc="SRAM2 page 31 write protection"]
    #[inline] pub fn p31wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if P31WP != 0"]
    #[inline] pub fn test_p31wp(&self) -> bool {
        self.p31wp() != 0
    }

    #[doc="Sets the P31WP field."]
    #[inline] pub fn set_p31wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="P30WP"]
    #[inline] pub fn p30wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if P30WP != 0"]
    #[inline] pub fn test_p30wp(&self) -> bool {
        self.p30wp() != 0
    }

    #[doc="Sets the P30WP field."]
    #[inline] pub fn set_p30wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="P29WP"]
    #[inline] pub fn p29wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if P29WP != 0"]
    #[inline] pub fn test_p29wp(&self) -> bool {
        self.p29wp() != 0
    }

    #[doc="Sets the P29WP field."]
    #[inline] pub fn set_p29wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="P28WP"]
    #[inline] pub fn p28wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if P28WP != 0"]
    #[inline] pub fn test_p28wp(&self) -> bool {
        self.p28wp() != 0
    }

    #[doc="Sets the P28WP field."]
    #[inline] pub fn set_p28wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="P27WP"]
    #[inline] pub fn p27wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if P27WP != 0"]
    #[inline] pub fn test_p27wp(&self) -> bool {
        self.p27wp() != 0
    }

    #[doc="Sets the P27WP field."]
    #[inline] pub fn set_p27wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="P26WP"]
    #[inline] pub fn p26wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if P26WP != 0"]
    #[inline] pub fn test_p26wp(&self) -> bool {
        self.p26wp() != 0
    }

    #[doc="Sets the P26WP field."]
    #[inline] pub fn set_p26wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="P25WP"]
    #[inline] pub fn p25wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if P25WP != 0"]
    #[inline] pub fn test_p25wp(&self) -> bool {
        self.p25wp() != 0
    }

    #[doc="Sets the P25WP field."]
    #[inline] pub fn set_p25wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="P24WP"]
    #[inline] pub fn p24wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if P24WP != 0"]
    #[inline] pub fn test_p24wp(&self) -> bool {
        self.p24wp() != 0
    }

    #[doc="Sets the P24WP field."]
    #[inline] pub fn set_p24wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="P23WP"]
    #[inline] pub fn p23wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if P23WP != 0"]
    #[inline] pub fn test_p23wp(&self) -> bool {
        self.p23wp() != 0
    }

    #[doc="Sets the P23WP field."]
    #[inline] pub fn set_p23wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="P22WP"]
    #[inline] pub fn p22wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if P22WP != 0"]
    #[inline] pub fn test_p22wp(&self) -> bool {
        self.p22wp() != 0
    }

    #[doc="Sets the P22WP field."]
    #[inline] pub fn set_p22wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="P21WP"]
    #[inline] pub fn p21wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if P21WP != 0"]
    #[inline] pub fn test_p21wp(&self) -> bool {
        self.p21wp() != 0
    }

    #[doc="Sets the P21WP field."]
    #[inline] pub fn set_p21wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="P20WP"]
    #[inline] pub fn p20wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if P20WP != 0"]
    #[inline] pub fn test_p20wp(&self) -> bool {
        self.p20wp() != 0
    }

    #[doc="Sets the P20WP field."]
    #[inline] pub fn set_p20wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="P19WP"]
    #[inline] pub fn p19wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if P19WP != 0"]
    #[inline] pub fn test_p19wp(&self) -> bool {
        self.p19wp() != 0
    }

    #[doc="Sets the P19WP field."]
    #[inline] pub fn set_p19wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="P18WP"]
    #[inline] pub fn p18wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if P18WP != 0"]
    #[inline] pub fn test_p18wp(&self) -> bool {
        self.p18wp() != 0
    }

    #[doc="Sets the P18WP field."]
    #[inline] pub fn set_p18wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="P17WP"]
    #[inline] pub fn p17wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if P17WP != 0"]
    #[inline] pub fn test_p17wp(&self) -> bool {
        self.p17wp() != 0
    }

    #[doc="Sets the P17WP field."]
    #[inline] pub fn set_p17wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="P16WP"]
    #[inline] pub fn p16wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if P16WP != 0"]
    #[inline] pub fn test_p16wp(&self) -> bool {
        self.p16wp() != 0
    }

    #[doc="Sets the P16WP field."]
    #[inline] pub fn set_p16wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="P15WP"]
    #[inline] pub fn p15wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if P15WP != 0"]
    #[inline] pub fn test_p15wp(&self) -> bool {
        self.p15wp() != 0
    }

    #[doc="Sets the P15WP field."]
    #[inline] pub fn set_p15wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="P14WP"]
    #[inline] pub fn p14wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if P14WP != 0"]
    #[inline] pub fn test_p14wp(&self) -> bool {
        self.p14wp() != 0
    }

    #[doc="Sets the P14WP field."]
    #[inline] pub fn set_p14wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="P13WP"]
    #[inline] pub fn p13wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if P13WP != 0"]
    #[inline] pub fn test_p13wp(&self) -> bool {
        self.p13wp() != 0
    }

    #[doc="Sets the P13WP field."]
    #[inline] pub fn set_p13wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="P12WP"]
    #[inline] pub fn p12wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if P12WP != 0"]
    #[inline] pub fn test_p12wp(&self) -> bool {
        self.p12wp() != 0
    }

    #[doc="Sets the P12WP field."]
    #[inline] pub fn set_p12wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="P11WP"]
    #[inline] pub fn p11wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if P11WP != 0"]
    #[inline] pub fn test_p11wp(&self) -> bool {
        self.p11wp() != 0
    }

    #[doc="Sets the P11WP field."]
    #[inline] pub fn set_p11wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="P10WP"]
    #[inline] pub fn p10wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if P10WP != 0"]
    #[inline] pub fn test_p10wp(&self) -> bool {
        self.p10wp() != 0
    }

    #[doc="Sets the P10WP field."]
    #[inline] pub fn set_p10wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="P9WP"]
    #[inline] pub fn p9wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if P9WP != 0"]
    #[inline] pub fn test_p9wp(&self) -> bool {
        self.p9wp() != 0
    }

    #[doc="Sets the P9WP field."]
    #[inline] pub fn set_p9wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="P8WP"]
    #[inline] pub fn p8wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if P8WP != 0"]
    #[inline] pub fn test_p8wp(&self) -> bool {
        self.p8wp() != 0
    }

    #[doc="Sets the P8WP field."]
    #[inline] pub fn set_p8wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="P7WP"]
    #[inline] pub fn p7wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if P7WP != 0"]
    #[inline] pub fn test_p7wp(&self) -> bool {
        self.p7wp() != 0
    }

    #[doc="Sets the P7WP field."]
    #[inline] pub fn set_p7wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="P6WP"]
    #[inline] pub fn p6wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if P6WP != 0"]
    #[inline] pub fn test_p6wp(&self) -> bool {
        self.p6wp() != 0
    }

    #[doc="Sets the P6WP field."]
    #[inline] pub fn set_p6wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="P5WP"]
    #[inline] pub fn p5wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if P5WP != 0"]
    #[inline] pub fn test_p5wp(&self) -> bool {
        self.p5wp() != 0
    }

    #[doc="Sets the P5WP field."]
    #[inline] pub fn set_p5wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="P4WP"]
    #[inline] pub fn p4wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if P4WP != 0"]
    #[inline] pub fn test_p4wp(&self) -> bool {
        self.p4wp() != 0
    }

    #[doc="Sets the P4WP field."]
    #[inline] pub fn set_p4wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="P3WP"]
    #[inline] pub fn p3wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if P3WP != 0"]
    #[inline] pub fn test_p3wp(&self) -> bool {
        self.p3wp() != 0
    }

    #[doc="Sets the P3WP field."]
    #[inline] pub fn set_p3wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="P2WP"]
    #[inline] pub fn p2wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if P2WP != 0"]
    #[inline] pub fn test_p2wp(&self) -> bool {
        self.p2wp() != 0
    }

    #[doc="Sets the P2WP field."]
    #[inline] pub fn set_p2wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="P1WP"]
    #[inline] pub fn p1wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if P1WP != 0"]
    #[inline] pub fn test_p1wp(&self) -> bool {
        self.p1wp() != 0
    }

    #[doc="Sets the P1WP field."]
    #[inline] pub fn set_p1wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="P0WP"]
    #[inline] pub fn p0wp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if P0WP != 0"]
    #[inline] pub fn test_p0wp(&self) -> bool {
        self.p0wp() != 0
    }

    #[doc="Sets the P0WP field."]
    #[inline] pub fn set_p0wp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Swpr {
    #[inline]
    fn from(other: u32) -> Self {
         Swpr(other)
    }
}

impl ::core::fmt::Display for Swpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p31wp() != 0 { try!(write!(f, " p31wp"))}
        if self.p30wp() != 0 { try!(write!(f, " p30wp"))}
        if self.p29wp() != 0 { try!(write!(f, " p29wp"))}
        if self.p28wp() != 0 { try!(write!(f, " p28wp"))}
        if self.p27wp() != 0 { try!(write!(f, " p27wp"))}
        if self.p26wp() != 0 { try!(write!(f, " p26wp"))}
        if self.p25wp() != 0 { try!(write!(f, " p25wp"))}
        if self.p24wp() != 0 { try!(write!(f, " p24wp"))}
        if self.p23wp() != 0 { try!(write!(f, " p23wp"))}
        if self.p22wp() != 0 { try!(write!(f, " p22wp"))}
        if self.p21wp() != 0 { try!(write!(f, " p21wp"))}
        if self.p20wp() != 0 { try!(write!(f, " p20wp"))}
        if self.p19wp() != 0 { try!(write!(f, " p19wp"))}
        if self.p18wp() != 0 { try!(write!(f, " p18wp"))}
        if self.p17wp() != 0 { try!(write!(f, " p17wp"))}
        if self.p16wp() != 0 { try!(write!(f, " p16wp"))}
        if self.p15wp() != 0 { try!(write!(f, " p15wp"))}
        if self.p14wp() != 0 { try!(write!(f, " p14wp"))}
        if self.p13wp() != 0 { try!(write!(f, " p13wp"))}
        if self.p12wp() != 0 { try!(write!(f, " p12wp"))}
        if self.p11wp() != 0 { try!(write!(f, " p11wp"))}
        if self.p10wp() != 0 { try!(write!(f, " p10wp"))}
        if self.p9wp() != 0 { try!(write!(f, " p9wp"))}
        if self.p8wp() != 0 { try!(write!(f, " p8wp"))}
        if self.p7wp() != 0 { try!(write!(f, " p7wp"))}
        if self.p6wp() != 0 { try!(write!(f, " p6wp"))}
        if self.p5wp() != 0 { try!(write!(f, " p5wp"))}
        if self.p4wp() != 0 { try!(write!(f, " p4wp"))}
        if self.p3wp() != 0 { try!(write!(f, " p3wp"))}
        if self.p2wp() != 0 { try!(write!(f, " p2wp"))}
        if self.p1wp() != 0 { try!(write!(f, " p1wp"))}
        if self.p0wp() != 0 { try!(write!(f, " p0wp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SKR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Skr(pub u32);
impl Skr {
    #[doc="SRAM2 write protection key for software erase"]
    #[inline] pub fn key(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Skr {
    #[inline]
    fn from(other: u32) -> Self {
         Skr(other)
    }
}

impl ::core::fmt::Display for Skr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Skr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

