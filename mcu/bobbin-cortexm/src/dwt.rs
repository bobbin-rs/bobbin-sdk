
::bobbin_mcu::periph!( DWT, Dwt, DWT_PERIPH, DwtPeriph, DWT_OWNED, DWT_REF_COUNT, 0xe0001000, 0x00, 0x07);


#[doc="Debug Core Block"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DwtPeriph(pub usize);
impl DwtPeriph {
    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        self.ctrl_reg().read()
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn write_ctrl(&self, value: Ctrl) -> &Self { 
        self.ctrl_reg().write(value);
        self
    }

    #[doc="Set the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().set(f);
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().with(f);
        self
    }

    #[doc="Get the CYCCNT Register."]
    #[inline] pub fn cyccnt_reg(&self) -> ::bobbin_mcu::register::Register<Cyccnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cyccnt, 0x4)
    }

    #[doc="Get the *mut pointer for the CYCCNT register."]
    #[inline] pub fn cyccnt_mut(&self) -> *mut Cyccnt { 
        self.cyccnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the CYCCNT register."]
    #[inline] pub fn cyccnt_ptr(&self) -> *const Cyccnt { 
        self.cyccnt_reg().ptr()
    }

    #[doc="Read the CYCCNT register."]
    #[inline] pub fn cyccnt(&self) -> Cyccnt { 
        self.cyccnt_reg().read()
    }

    #[doc="Write the CYCCNT register."]
    #[inline] pub fn write_cyccnt(&self, value: Cyccnt) -> &Self { 
        self.cyccnt_reg().write(value);
        self
    }

    #[doc="Set the CYCCNT register."]
    #[inline] pub fn set_cyccnt<F: FnOnce(Cyccnt) -> Cyccnt>(&self, f: F) -> &Self {
        self.cyccnt_reg().set(f);
        self
    }

    #[doc="Modify the CYCCNT register."]
    #[inline] pub fn with_cyccnt<F: FnOnce(Cyccnt) -> Cyccnt>(&self, f: F) -> &Self {
        self.cyccnt_reg().with(f);
        self
    }

    #[doc="Get the CPICNT Register."]
    #[inline] pub fn cpicnt_reg(&self) -> ::bobbin_mcu::register::Register<Cpicnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cpicnt, 0x8)
    }

    #[doc="Get the *mut pointer for the CPICNT register."]
    #[inline] pub fn cpicnt_mut(&self) -> *mut Cpicnt { 
        self.cpicnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the CPICNT register."]
    #[inline] pub fn cpicnt_ptr(&self) -> *const Cpicnt { 
        self.cpicnt_reg().ptr()
    }

    #[doc="Read the CPICNT register."]
    #[inline] pub fn cpicnt(&self) -> Cpicnt { 
        self.cpicnt_reg().read()
    }

    #[doc="Write the CPICNT register."]
    #[inline] pub fn write_cpicnt(&self, value: Cpicnt) -> &Self { 
        self.cpicnt_reg().write(value);
        self
    }

    #[doc="Set the CPICNT register."]
    #[inline] pub fn set_cpicnt<F: FnOnce(Cpicnt) -> Cpicnt>(&self, f: F) -> &Self {
        self.cpicnt_reg().set(f);
        self
    }

    #[doc="Modify the CPICNT register."]
    #[inline] pub fn with_cpicnt<F: FnOnce(Cpicnt) -> Cpicnt>(&self, f: F) -> &Self {
        self.cpicnt_reg().with(f);
        self
    }

    #[doc="Get the EXCCNT Register."]
    #[inline] pub fn exccnt_reg(&self) -> ::bobbin_mcu::register::Register<Exccnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exccnt, 0xc)
    }

    #[doc="Get the *mut pointer for the EXCCNT register."]
    #[inline] pub fn exccnt_mut(&self) -> *mut Exccnt { 
        self.exccnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXCCNT register."]
    #[inline] pub fn exccnt_ptr(&self) -> *const Exccnt { 
        self.exccnt_reg().ptr()
    }

    #[doc="Read the EXCCNT register."]
    #[inline] pub fn exccnt(&self) -> Exccnt { 
        self.exccnt_reg().read()
    }

    #[doc="Write the EXCCNT register."]
    #[inline] pub fn write_exccnt(&self, value: Exccnt) -> &Self { 
        self.exccnt_reg().write(value);
        self
    }

    #[doc="Set the EXCCNT register."]
    #[inline] pub fn set_exccnt<F: FnOnce(Exccnt) -> Exccnt>(&self, f: F) -> &Self {
        self.exccnt_reg().set(f);
        self
    }

    #[doc="Modify the EXCCNT register."]
    #[inline] pub fn with_exccnt<F: FnOnce(Exccnt) -> Exccnt>(&self, f: F) -> &Self {
        self.exccnt_reg().with(f);
        self
    }

    #[doc="Get the SLEEPCNT Register."]
    #[inline] pub fn sleepcnt_reg(&self) -> ::bobbin_mcu::register::Register<Sleepcnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sleepcnt, 0x10)
    }

    #[doc="Get the *mut pointer for the SLEEPCNT register."]
    #[inline] pub fn sleepcnt_mut(&self) -> *mut Sleepcnt { 
        self.sleepcnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the SLEEPCNT register."]
    #[inline] pub fn sleepcnt_ptr(&self) -> *const Sleepcnt { 
        self.sleepcnt_reg().ptr()
    }

    #[doc="Read the SLEEPCNT register."]
    #[inline] pub fn sleepcnt(&self) -> Sleepcnt { 
        self.sleepcnt_reg().read()
    }

    #[doc="Write the SLEEPCNT register."]
    #[inline] pub fn write_sleepcnt(&self, value: Sleepcnt) -> &Self { 
        self.sleepcnt_reg().write(value);
        self
    }

    #[doc="Set the SLEEPCNT register."]
    #[inline] pub fn set_sleepcnt<F: FnOnce(Sleepcnt) -> Sleepcnt>(&self, f: F) -> &Self {
        self.sleepcnt_reg().set(f);
        self
    }

    #[doc="Modify the SLEEPCNT register."]
    #[inline] pub fn with_sleepcnt<F: FnOnce(Sleepcnt) -> Sleepcnt>(&self, f: F) -> &Self {
        self.sleepcnt_reg().with(f);
        self
    }

    #[doc="Get the LSUCNT Register."]
    #[inline] pub fn lsucnt_reg(&self) -> ::bobbin_mcu::register::Register<Lsucnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Lsucnt, 0x14)
    }

    #[doc="Get the *mut pointer for the LSUCNT register."]
    #[inline] pub fn lsucnt_mut(&self) -> *mut Lsucnt { 
        self.lsucnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the LSUCNT register."]
    #[inline] pub fn lsucnt_ptr(&self) -> *const Lsucnt { 
        self.lsucnt_reg().ptr()
    }

    #[doc="Read the LSUCNT register."]
    #[inline] pub fn lsucnt(&self) -> Lsucnt { 
        self.lsucnt_reg().read()
    }

    #[doc="Write the LSUCNT register."]
    #[inline] pub fn write_lsucnt(&self, value: Lsucnt) -> &Self { 
        self.lsucnt_reg().write(value);
        self
    }

    #[doc="Set the LSUCNT register."]
    #[inline] pub fn set_lsucnt<F: FnOnce(Lsucnt) -> Lsucnt>(&self, f: F) -> &Self {
        self.lsucnt_reg().set(f);
        self
    }

    #[doc="Modify the LSUCNT register."]
    #[inline] pub fn with_lsucnt<F: FnOnce(Lsucnt) -> Lsucnt>(&self, f: F) -> &Self {
        self.lsucnt_reg().with(f);
        self
    }

    #[doc="Get the FOLDCNT Register."]
    #[inline] pub fn foldcnt_reg(&self) -> ::bobbin_mcu::register::Register<Foldcnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Foldcnt, 0x19)
    }

    #[doc="Get the *mut pointer for the FOLDCNT register."]
    #[inline] pub fn foldcnt_mut(&self) -> *mut Foldcnt { 
        self.foldcnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the FOLDCNT register."]
    #[inline] pub fn foldcnt_ptr(&self) -> *const Foldcnt { 
        self.foldcnt_reg().ptr()
    }

    #[doc="Read the FOLDCNT register."]
    #[inline] pub fn foldcnt(&self) -> Foldcnt { 
        self.foldcnt_reg().read()
    }

    #[doc="Write the FOLDCNT register."]
    #[inline] pub fn write_foldcnt(&self, value: Foldcnt) -> &Self { 
        self.foldcnt_reg().write(value);
        self
    }

    #[doc="Set the FOLDCNT register."]
    #[inline] pub fn set_foldcnt<F: FnOnce(Foldcnt) -> Foldcnt>(&self, f: F) -> &Self {
        self.foldcnt_reg().set(f);
        self
    }

    #[doc="Modify the FOLDCNT register."]
    #[inline] pub fn with_foldcnt<F: FnOnce(Foldcnt) -> Foldcnt>(&self, f: F) -> &Self {
        self.foldcnt_reg().with(f);
        self
    }

    #[doc="Get the PCSR Register."]
    #[inline] pub fn pcsr_reg(&self) -> ::bobbin_mcu::register::Register<Pcsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcsr, 0x14)
    }

    #[doc="Get the *mut pointer for the PCSR register."]
    #[inline] pub fn pcsr_mut(&self) -> *mut Pcsr { 
        self.pcsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCSR register."]
    #[inline] pub fn pcsr_ptr(&self) -> *const Pcsr { 
        self.pcsr_reg().ptr()
    }

    #[doc="Read the PCSR register."]
    #[inline] pub fn pcsr(&self) -> Pcsr { 
        self.pcsr_reg().read()
    }

    #[doc="Write the PCSR register."]
    #[inline] pub fn write_pcsr(&self, value: Pcsr) -> &Self { 
        self.pcsr_reg().write(value);
        self
    }

    #[doc="Set the PCSR register."]
    #[inline] pub fn set_pcsr<F: FnOnce(Pcsr) -> Pcsr>(&self, f: F) -> &Self {
        self.pcsr_reg().set(f);
        self
    }

    #[doc="Modify the PCSR register."]
    #[inline] pub fn with_pcsr<F: FnOnce(Pcsr) -> Pcsr>(&self, f: F) -> &Self {
        self.pcsr_reg().with(f);
        self
    }

}

#[doc="Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cycle Count Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cyccnt(pub u32);
impl Cyccnt {
}

impl From<u32> for Cyccnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cyccnt(other)
    }
}

impl ::core::fmt::Display for Cyccnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cyccnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPI Count Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpicnt(pub u32);
impl Cpicnt {
}

impl From<u32> for Cpicnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cpicnt(other)
    }
}

impl ::core::fmt::Display for Cpicnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cpicnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Exception Overhead Count Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exccnt(pub u32);
impl Exccnt {
}

impl From<u32> for Exccnt {
    #[inline]
    fn from(other: u32) -> Self {
         Exccnt(other)
    }
}

impl ::core::fmt::Display for Exccnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exccnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Sleep Count Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sleepcnt(pub u32);
impl Sleepcnt {
}

impl From<u32> for Sleepcnt {
    #[inline]
    fn from(other: u32) -> Self {
         Sleepcnt(other)
    }
}

impl ::core::fmt::Display for Sleepcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sleepcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LSU Count Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lsucnt(pub u32);
impl Lsucnt {
}

impl From<u32> for Lsucnt {
    #[inline]
    fn from(other: u32) -> Self {
         Lsucnt(other)
    }
}

impl ::core::fmt::Display for Lsucnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lsucnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Folded-instruction Count Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Foldcnt(pub u32);
impl Foldcnt {
}

impl From<u32> for Foldcnt {
    #[inline]
    fn from(other: u32) -> Self {
         Foldcnt(other)
    }
}

impl ::core::fmt::Display for Foldcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Foldcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Program Count Sample Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcsr(pub u32);
impl Pcsr {
}

impl From<u32> for Pcsr {
    #[inline]
    fn from(other: u32) -> Self {
         Pcsr(other)
    }
}

impl ::core::fmt::Display for Pcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

