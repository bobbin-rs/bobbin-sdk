
::bobbin_mcu::periph!( SYSTICK, Systick, SYSTICK_PERIPH, SystickPeriph, SYSTICK_OWNED, SYSTICK_REF_COUNT, 0xe000e000, 0x00, 0x02);


#[doc="System Timer, SysTick"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SystickPeriph(pub usize);
impl SystickPeriph {
    #[doc="Get the CSR Register."]
    #[inline] pub fn csr_reg(&self) -> ::bobbin_mcu::register::Register<Csr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Csr, 0x10)
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

    #[doc="Get the RVR Register."]
    #[inline] pub fn rvr_reg(&self) -> ::bobbin_mcu::register::Register<Rvr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rvr, 0x14)
    }

    #[doc="Get the *mut pointer for the RVR register."]
    #[inline] pub fn rvr_mut(&self) -> *mut Rvr { 
        self.rvr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RVR register."]
    #[inline] pub fn rvr_ptr(&self) -> *const Rvr { 
        self.rvr_reg().ptr()
    }

    #[doc="Read the RVR register."]
    #[inline] pub fn rvr(&self) -> Rvr { 
        self.rvr_reg().read()
    }

    #[doc="Write the RVR register."]
    #[inline] pub fn write_rvr(&self, value: Rvr) -> &Self { 
        self.rvr_reg().write(value);
        self
    }

    #[doc="Set the RVR register."]
    #[inline] pub fn set_rvr<F: FnOnce(Rvr) -> Rvr>(&self, f: F) -> &Self {
        self.rvr_reg().set(f);
        self
    }

    #[doc="Modify the RVR register."]
    #[inline] pub fn with_rvr<F: FnOnce(Rvr) -> Rvr>(&self, f: F) -> &Self {
        self.rvr_reg().with(f);
        self
    }

    #[doc="Get the CVR Register."]
    #[inline] pub fn cvr_reg(&self) -> ::bobbin_mcu::register::Register<Cvr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cvr, 0x18)
    }

    #[doc="Get the *mut pointer for the CVR register."]
    #[inline] pub fn cvr_mut(&self) -> *mut Cvr { 
        self.cvr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CVR register."]
    #[inline] pub fn cvr_ptr(&self) -> *const Cvr { 
        self.cvr_reg().ptr()
    }

    #[doc="Read the CVR register."]
    #[inline] pub fn cvr(&self) -> Cvr { 
        self.cvr_reg().read()
    }

    #[doc="Write the CVR register."]
    #[inline] pub fn write_cvr(&self, value: Cvr) -> &Self { 
        self.cvr_reg().write(value);
        self
    }

    #[doc="Set the CVR register."]
    #[inline] pub fn set_cvr<F: FnOnce(Cvr) -> Cvr>(&self, f: F) -> &Self {
        self.cvr_reg().set(f);
        self
    }

    #[doc="Modify the CVR register."]
    #[inline] pub fn with_cvr<F: FnOnce(Cvr) -> Cvr>(&self, f: F) -> &Self {
        self.cvr_reg().with(f);
        self
    }

    #[doc="Get the CALIB Register."]
    #[inline] pub fn calib_reg(&self) -> ::bobbin_mcu::register::Register<Calib> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Calib, 0x1c)
    }

    #[doc="Get the *mut pointer for the CALIB register."]
    #[inline] pub fn calib_mut(&self) -> *mut Calib { 
        self.calib_reg().ptr()
    }

    #[doc="Get the *const pointer for the CALIB register."]
    #[inline] pub fn calib_ptr(&self) -> *const Calib { 
        self.calib_reg().ptr()
    }

    #[doc="Read the CALIB register."]
    #[inline] pub fn calib(&self) -> Calib { 
        self.calib_reg().read()
    }

    #[doc="Write the CALIB register."]
    #[inline] pub fn write_calib(&self, value: Calib) -> &Self { 
        self.calib_reg().write(value);
        self
    }

    #[doc="Set the CALIB register."]
    #[inline] pub fn set_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        self.calib_reg().set(f);
        self
    }

    #[doc="Modify the CALIB register."]
    #[inline] pub fn with_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        self.calib_reg().with(f);
        self
    }

}

#[doc="Control and Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="Returns 1 if timer counted to 0 since last time this was read."]
    #[inline] pub fn countflag(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if COUNTFLAG != 0"]
    #[inline] pub fn test_countflag(&self) -> bool {
        self.countflag() != 0
    }

    #[doc="Sets the COUNTFLAG field."]
    #[inline] pub fn set_countflag<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Indicates the clock source: 0 = external clock, 1 = processor clock."]
    #[inline] pub fn clksource(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CLKSOURCE != 0"]
    #[inline] pub fn test_clksource(&self) -> bool {
        self.clksource() != 0
    }

    #[doc="Sets the CLKSOURCE field."]
    #[inline] pub fn set_clksource<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enables SysTick exception request: 0 = counting down to zero does not assert the SysTick exception request, 1 = counting down to zero asserts the SysTick exception request."]
    #[inline] pub fn tickint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TICKINT != 0"]
    #[inline] pub fn test_tickint(&self) -> bool {
        self.tickint() != 0
    }

    #[doc="Sets the TICKINT field."]
    #[inline] pub fn set_tickint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enables the counter: 0 = counter disabled, 1 = counter enabled."]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.countflag() != 0 { try!(write!(f, " countflag"))}
        if self.clksource() != 0 { try!(write!(f, " clksource"))}
        if self.tickint() != 0 { try!(write!(f, " tickint"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reload Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rvr(pub u32);
impl Rvr {
    #[doc="Value to load into the SYST_CVR register when the counter is enabled and when it reaches 0"]
    #[inline] pub fn reload(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if RELOAD != 0"]
    #[inline] pub fn test_reload(&self) -> bool {
        self.reload() != 0
    }

    #[doc="Sets the RELOAD field."]
    #[inline] pub fn set_reload<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rvr {
    #[inline]
    fn from(other: u32) -> Self {
         Rvr(other)
    }
}

impl ::core::fmt::Display for Rvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.reload() != 0 { try!(write!(f, " reload=0x{:x}", self.reload()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Current Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cvr(pub u32);
impl Cvr {
    #[doc="Reads return the current value of the SysTick counter. A write of any value clears the field to 0, and also clears the SYST_CSR COUNTFLAG bit to 0."]
    #[inline] pub fn current(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if CURRENT != 0"]
    #[inline] pub fn test_current(&self) -> bool {
        self.current() != 0
    }

    #[doc="Sets the CURRENT field."]
    #[inline] pub fn set_current<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cvr {
    #[inline]
    fn from(other: u32) -> Self {
         Cvr(other)
    }
}

impl ::core::fmt::Display for Cvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.current() != 0 { try!(write!(f, " current=0x{:x}", self.current()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calib(pub u32);
impl Calib {
    #[doc="Indicates whether the device provides a reference clock to the processor"]
    #[inline] pub fn noref(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if NOREF != 0"]
    #[inline] pub fn test_noref(&self) -> bool {
        self.noref() != 0
    }

    #[doc="Sets the NOREF field."]
    #[inline] pub fn set_noref<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Indicates whether the TENMS value is exact: 0 = TENMS value is exact, 1 = TENMS value is inexact, or not given."]
    #[inline] pub fn skew(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if SKEW != 0"]
    #[inline] pub fn test_skew(&self) -> bool {
        self.skew() != 0
    }

    #[doc="Sets the SKEW field."]
    #[inline] pub fn set_skew<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Reload value for 10ms (100Hz) timing, subject to system clock skew errors."]
    #[inline] pub fn tenms(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if TENMS != 0"]
    #[inline] pub fn test_tenms(&self) -> bool {
        self.tenms() != 0
    }

    #[doc="Sets the TENMS field."]
    #[inline] pub fn set_tenms<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Calib {
    #[inline]
    fn from(other: u32) -> Self {
         Calib(other)
    }
}

impl ::core::fmt::Display for Calib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.noref() != 0 { try!(write!(f, " noref"))}
        if self.skew() != 0 { try!(write!(f, " skew"))}
        if self.tenms() != 0 { try!(write!(f, " tenms=0x{:x}", self.tenms()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

