//! Debug Core Block
#[allow(unused_imports)] use bobbin_common::*;

periph!(DWT, Dwt, 0xe0001000);

#[doc="Debug Core Block"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dwt(pub usize);
impl Dwt {
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

    #[doc="Get the *mut pointer for the CYCCNT register."]
    #[inline] pub fn cyccnt_mut(&self) -> *mut Cyccnt { 
        (self.0 + 0x4) as *mut Cyccnt
    }

    #[doc="Get the *const pointer for the CYCCNT register."]
    #[inline] pub fn cyccnt_ptr(&self) -> *const Cyccnt { 
           self.cyccnt_mut()
    }

    #[doc="Read the CYCCNT register."]
    #[inline] pub fn cyccnt(&self) -> Cyccnt { 
        unsafe {
            read_volatile(self.cyccnt_ptr())
        }
    }

    #[doc="Write the CYCCNT register."]
    #[inline] pub fn set_cyccnt<F: FnOnce(Cyccnt) -> Cyccnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cyccnt_mut(), f(Cyccnt(0)));
        }
        self
    }

    #[doc="Modify the CYCCNT register."]
    #[inline] pub fn with_cyccnt<F: FnOnce(Cyccnt) -> Cyccnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cyccnt_mut(), f(self.cyccnt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CPICNT register."]
    #[inline] pub fn cpicnt_mut(&self) -> *mut Cpicnt { 
        (self.0 + 0x8) as *mut Cpicnt
    }

    #[doc="Get the *const pointer for the CPICNT register."]
    #[inline] pub fn cpicnt_ptr(&self) -> *const Cpicnt { 
           self.cpicnt_mut()
    }

    #[doc="Read the CPICNT register."]
    #[inline] pub fn cpicnt(&self) -> Cpicnt { 
        unsafe {
            read_volatile(self.cpicnt_ptr())
        }
    }

    #[doc="Write the CPICNT register."]
    #[inline] pub fn set_cpicnt<F: FnOnce(Cpicnt) -> Cpicnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cpicnt_mut(), f(Cpicnt(0)));
        }
        self
    }

    #[doc="Modify the CPICNT register."]
    #[inline] pub fn with_cpicnt<F: FnOnce(Cpicnt) -> Cpicnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cpicnt_mut(), f(self.cpicnt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXCCNT register."]
    #[inline] pub fn exccnt_mut(&self) -> *mut Exccnt { 
        (self.0 + 0xc) as *mut Exccnt
    }

    #[doc="Get the *const pointer for the EXCCNT register."]
    #[inline] pub fn exccnt_ptr(&self) -> *const Exccnt { 
           self.exccnt_mut()
    }

    #[doc="Read the EXCCNT register."]
    #[inline] pub fn exccnt(&self) -> Exccnt { 
        unsafe {
            read_volatile(self.exccnt_ptr())
        }
    }

    #[doc="Write the EXCCNT register."]
    #[inline] pub fn set_exccnt<F: FnOnce(Exccnt) -> Exccnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exccnt_mut(), f(Exccnt(0)));
        }
        self
    }

    #[doc="Modify the EXCCNT register."]
    #[inline] pub fn with_exccnt<F: FnOnce(Exccnt) -> Exccnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exccnt_mut(), f(self.exccnt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SLEEPCNT register."]
    #[inline] pub fn sleepcnt_mut(&self) -> *mut Sleepcnt { 
        (self.0 + 0x10) as *mut Sleepcnt
    }

    #[doc="Get the *const pointer for the SLEEPCNT register."]
    #[inline] pub fn sleepcnt_ptr(&self) -> *const Sleepcnt { 
           self.sleepcnt_mut()
    }

    #[doc="Read the SLEEPCNT register."]
    #[inline] pub fn sleepcnt(&self) -> Sleepcnt { 
        unsafe {
            read_volatile(self.sleepcnt_ptr())
        }
    }

    #[doc="Write the SLEEPCNT register."]
    #[inline] pub fn set_sleepcnt<F: FnOnce(Sleepcnt) -> Sleepcnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sleepcnt_mut(), f(Sleepcnt(0)));
        }
        self
    }

    #[doc="Modify the SLEEPCNT register."]
    #[inline] pub fn with_sleepcnt<F: FnOnce(Sleepcnt) -> Sleepcnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sleepcnt_mut(), f(self.sleepcnt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LSUCNT register."]
    #[inline] pub fn lsucnt_mut(&self) -> *mut Lsucnt { 
        (self.0 + 0x14) as *mut Lsucnt
    }

    #[doc="Get the *const pointer for the LSUCNT register."]
    #[inline] pub fn lsucnt_ptr(&self) -> *const Lsucnt { 
           self.lsucnt_mut()
    }

    #[doc="Read the LSUCNT register."]
    #[inline] pub fn lsucnt(&self) -> Lsucnt { 
        unsafe {
            read_volatile(self.lsucnt_ptr())
        }
    }

    #[doc="Write the LSUCNT register."]
    #[inline] pub fn set_lsucnt<F: FnOnce(Lsucnt) -> Lsucnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lsucnt_mut(), f(Lsucnt(0)));
        }
        self
    }

    #[doc="Modify the LSUCNT register."]
    #[inline] pub fn with_lsucnt<F: FnOnce(Lsucnt) -> Lsucnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lsucnt_mut(), f(self.lsucnt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FOLDCNT register."]
    #[inline] pub fn foldcnt_mut(&self) -> *mut Foldcnt { 
        (self.0 + 0x19) as *mut Foldcnt
    }

    #[doc="Get the *const pointer for the FOLDCNT register."]
    #[inline] pub fn foldcnt_ptr(&self) -> *const Foldcnt { 
           self.foldcnt_mut()
    }

    #[doc="Read the FOLDCNT register."]
    #[inline] pub fn foldcnt(&self) -> Foldcnt { 
        unsafe {
            read_volatile(self.foldcnt_ptr())
        }
    }

    #[doc="Write the FOLDCNT register."]
    #[inline] pub fn set_foldcnt<F: FnOnce(Foldcnt) -> Foldcnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.foldcnt_mut(), f(Foldcnt(0)));
        }
        self
    }

    #[doc="Modify the FOLDCNT register."]
    #[inline] pub fn with_foldcnt<F: FnOnce(Foldcnt) -> Foldcnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.foldcnt_mut(), f(self.foldcnt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PCSR register."]
    #[inline] pub fn pcsr_mut(&self) -> *mut Pcsr { 
        (self.0 + 0x14) as *mut Pcsr
    }

    #[doc="Get the *const pointer for the PCSR register."]
    #[inline] pub fn pcsr_ptr(&self) -> *const Pcsr { 
           self.pcsr_mut()
    }

    #[doc="Read the PCSR register."]
    #[inline] pub fn pcsr(&self) -> Pcsr { 
        unsafe {
            read_volatile(self.pcsr_ptr())
        }
    }

    #[doc="Write the PCSR register."]
    #[inline] pub fn set_pcsr<F: FnOnce(Pcsr) -> Pcsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pcsr_mut(), f(Pcsr(0)));
        }
        self
    }

    #[doc="Modify the PCSR register."]
    #[inline] pub fn with_pcsr<F: FnOnce(Pcsr) -> Pcsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pcsr_mut(), f(self.pcsr()));
        }
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


