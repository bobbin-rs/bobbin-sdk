::bobbin_mcu::periph!( VREFBUF, Vrefbuf, VREFBUF_PERIPH, VrefPeriph, VREFBUF_OWNED, VREFBUF_REF_COUNT, 0x40010030, 0x00, 0x28);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="VREF Peripheral"]
pub struct VrefPeriph(pub usize); 

impl VrefPeriph {
    #[doc="Get the CSR Register."]
    #[inline] pub fn csr_reg(&self) -> ::bobbin_mcu::register::Register<Csr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr, 0x0)
    }

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        self.csr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const Csr { 
        self.csr_reg().ptr()
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        self.csr_reg().read()
    }

    #[doc="Write the CSR register."]
    #[inline] pub fn write_csr(&self, value: Csr) -> &Self { 
        self.csr_reg().write(value);
        self
    }

    #[doc="Set the CSR register."]
    #[inline] pub fn set_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        self.csr_reg().set(f);
        self
    }

    #[doc="Modify the CSR register."]
    #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        self.csr_reg().with(f);
        self
    }

    #[doc="Get the CCR Register."]
    #[inline] pub fn ccr_reg(&self) -> ::bobbin_mcu::register::Register<Ccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ccr, 0x4)
    }

    #[doc="Get the *mut pointer for the CCR register."]
    #[inline] pub fn ccr_mut(&self) -> *mut Ccr { 
        self.ccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CCR register."]
    #[inline] pub fn ccr_ptr(&self) -> *const Ccr { 
        self.ccr_reg().ptr()
    }

    #[doc="Read the CCR register."]
    #[inline] pub fn ccr(&self) -> Ccr { 
        self.ccr_reg().read()
    }

    #[doc="Write the CCR register."]
    #[inline] pub fn write_ccr(&self, value: Ccr) -> &Self { 
        self.ccr_reg().write(value);
        self
    }

    #[doc="Set the CCR register."]
    #[inline] pub fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        self.ccr_reg().set(f);
        self
    }

    #[doc="Modify the CCR register."]
    #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        self.ccr_reg().with(f);
        self
    }

}

#[doc="VREF control and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="Voltage reference buffer enable"]
    #[inline] pub fn envr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENVR != 0"]
    #[inline] pub fn test_envr(&self) -> bool {
        self.envr() != 0
    }

    #[doc="Sets the ENVR field."]
    #[inline] pub fn set_envr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="High impedance mode"]
    #[inline] pub fn hiz(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HIZ != 0"]
    #[inline] pub fn test_hiz(&self) -> bool {
        self.hiz() != 0
    }

    #[doc="Sets the HIZ field."]
    #[inline] pub fn set_hiz<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Voltage reference scale"]
    #[inline] pub fn vrs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if VRS != 0"]
    #[inline] pub fn test_vrs(&self) -> bool {
        self.vrs() != 0
    }

    #[doc="Sets the VRS field."]
    #[inline] pub fn set_vrs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Voltage reference buffer ready"]
    #[inline] pub fn vrr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if VRR != 0"]
    #[inline] pub fn test_vrr(&self) -> bool {
        self.vrr() != 0
    }

    #[doc="Sets the VRR field."]
    #[inline] pub fn set_vrr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
    #[inline] pub fn trim(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if TRIM != 0"]
    #[inline] pub fn test_trim(&self) -> bool {
        self.trim() != 0
    }

    #[doc="Sets the TRIM field."]
    #[inline] pub fn set_trim<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
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

