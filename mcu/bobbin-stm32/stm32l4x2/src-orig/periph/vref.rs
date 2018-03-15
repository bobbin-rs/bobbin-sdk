#[allow(unused_imports)] use ::bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="VREF Peripheral"]
pub struct VrefPeriph(pub usize); 

impl VrefPeriph {
    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        (self.0 + 0x0) as *mut Csr
    }

    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const Csr { 
           self.csr_mut()
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        unsafe {
            read_volatile(self.csr_ptr())
        }
    }

    #[doc="Write the CSR register."]
    #[inline] pub fn set_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr_mut(), f(Csr(0)));
        }
        self
    }

    #[doc="Modify the CSR register."]
    #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr_mut(), f(self.csr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CCR register."]
    #[inline] pub fn ccr_mut(&self) -> *mut Ccr { 
        (self.0 + 0x4) as *mut Ccr
    }

    #[doc="Get the *const pointer for the CCR register."]
    #[inline] pub fn ccr_ptr(&self) -> *const Ccr { 
           self.ccr_mut()
    }

    #[doc="Read the CCR register."]
    #[inline] pub fn ccr(&self) -> Ccr { 
        unsafe {
            read_volatile(self.ccr_ptr())
        }
    }

    #[doc="Write the CCR register."]
    #[inline] pub fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccr_mut(), f(Ccr(0)));
        }
        self
    }

    #[doc="Modify the CCR register."]
    #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccr_mut(), f(self.ccr()));
        }
        self
    }

}

#[doc="VREF control and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="Voltage reference buffer enable"]
    #[inline] pub fn envr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENVR != 0"]
    #[inline] pub fn test_envr(&self) -> bool {
        self.envr() != 0
    }

    #[doc="Sets the ENVR field."]
    #[inline] pub fn set_envr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="High impedance mode"]
    #[inline] pub fn hiz(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HIZ != 0"]
    #[inline] pub fn test_hiz(&self) -> bool {
        self.hiz() != 0
    }

    #[doc="Sets the HIZ field."]
    #[inline] pub fn set_hiz<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Voltage reference scale"]
    #[inline] pub fn vrs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if VRS != 0"]
    #[inline] pub fn test_vrs(&self) -> bool {
        self.vrs() != 0
    }

    #[doc="Sets the VRS field."]
    #[inline] pub fn set_vrs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Voltage reference buffer ready"]
    #[inline] pub fn vrr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if VRR != 0"]
    #[inline] pub fn test_vrr(&self) -> bool {
        self.vrr() != 0
    }

    #[doc="Sets the VRR field."]
    #[inline] pub fn set_vrr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Csr(other)
    }
}

impl ::core::fmt::Display for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.envr() != 0 { try!(write!(f, " envr"))}
        if self.hiz() != 0 { try!(write!(f, " hiz"))}
        if self.vrs() != 0 { try!(write!(f, " vrs"))}
        if self.vrr() != 0 { try!(write!(f, " vrr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="calibration control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc="Trimming code"]
    #[inline] pub fn trim(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if TRIM != 0"]
    #[inline] pub fn test_trim(&self) -> bool {
        self.trim() != 0
    }

    #[doc="Sets the TRIM field."]
    #[inline] pub fn set_trim<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccr {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr(other)
    }
}

impl ::core::fmt::Display for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trim() != 0 { try!(write!(f, " trim=0x{:x}", self.trim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

