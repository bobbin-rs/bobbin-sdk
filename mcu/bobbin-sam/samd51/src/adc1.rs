::bobbin_mcu::periph!( HMATRIX, Hmatrix, HMATRIX_PERIPH, Adc1Periph, HMATRIX_OWNED, HMATRIX_REF_COUNT, 0x4100c000, 0x00, 0x0e);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC1 Peripheral"]
pub struct Adc1Periph(pub usize); 

impl Adc1Periph {
    #[doc="Get the PRAS Register."]
    #[inline] pub fn pras_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pras, ::bobbin_bits::R16> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pras, 0x80, 0x8)
    }

    #[doc="Get the *mut pointer for the PRAS register."]
    #[inline] pub fn pras_mut<I: Into<::bobbin_bits::R16>>(&self, index: I) -> *mut Pras { 
        self.pras_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PRAS register."]
    #[inline] pub fn pras_ptr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> *const Pras { 
        self.pras_reg().ptr(index.into())
    }

    #[doc="Read the PRAS register."]
    #[inline] pub fn pras<I: Into<::bobbin_bits::R16>>(&self, index: I) -> Pras { 
        self.pras_reg().read(index.into())
    }

    #[doc="Write the PRAS register."]
    #[inline] pub fn write_pras<I: Into<::bobbin_bits::R16>>(&self, index: I, value: Pras) -> &Self {
        self.pras_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PRAS register."]
    #[inline] pub fn set_pras<I: Into<::bobbin_bits::R16>, F: FnOnce(Pras) -> Pras>(&self, index: I, f: F) -> &Self {
        self.pras_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PRAS register."]
    #[inline] pub fn with_pras<I: Into<::bobbin_bits::R16> + Copy, F: FnOnce(Pras) -> Pras>(&self, index: I, f: F) -> &Self {
        self.pras_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PRBS Register."]
    #[inline] pub fn prbs_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Prbs, ::bobbin_bits::R16> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Prbs, 0x84, 0x8)
    }

    #[doc="Get the *mut pointer for the PRBS register."]
    #[inline] pub fn prbs_mut<I: Into<::bobbin_bits::R16>>(&self, index: I) -> *mut Prbs { 
        self.prbs_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PRBS register."]
    #[inline] pub fn prbs_ptr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> *const Prbs { 
        self.prbs_reg().ptr(index.into())
    }

    #[doc="Read the PRBS register."]
    #[inline] pub fn prbs<I: Into<::bobbin_bits::R16>>(&self, index: I) -> Prbs { 
        self.prbs_reg().read(index.into())
    }

    #[doc="Write the PRBS register."]
    #[inline] pub fn write_prbs<I: Into<::bobbin_bits::R16>>(&self, index: I, value: Prbs) -> &Self {
        self.prbs_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PRBS register."]
    #[inline] pub fn set_prbs<I: Into<::bobbin_bits::R16>, F: FnOnce(Prbs) -> Prbs>(&self, index: I, f: F) -> &Self {
        self.prbs_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PRBS register."]
    #[inline] pub fn with_prbs<I: Into<::bobbin_bits::R16> + Copy, F: FnOnce(Prbs) -> Prbs>(&self, index: I, f: F) -> &Self {
        self.prbs_reg().with(index.into(), f);
        self
    }

}

#[doc="Priority A for Slave"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pras(pub u32);
impl Pras {
}

impl From<u32> for Pras {
    #[inline]
    fn from(other: u32) -> Self {
         Pras(other)
    }
}

impl ::core::fmt::Display for Pras {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pras {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Priority B for Slave"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prbs(pub u32);
impl Prbs {
}

impl From<u32> for Prbs {
    #[inline]
    fn from(other: u32) -> Self {
         Prbs(other)
    }
}

impl ::core::fmt::Display for Prbs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prbs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

