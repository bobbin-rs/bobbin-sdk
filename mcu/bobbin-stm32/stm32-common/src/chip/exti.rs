#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="EXTI Peripheral"]
pub struct ExtiPeriph(pub usize); 


impl ExtiPeriph {
    #[doc="Get the *mut pointer for the IMR register."]
    #[inline] pub fn imr_mut(&self) -> *mut Imr { 
        (self.0 + 0x0) as *mut Imr
    }

    #[doc="Get the *const pointer for the IMR register."]
    #[inline] pub fn imr_ptr(&self) -> *const Imr { 
           self.imr_mut()
    }

    #[doc="Read the IMR register."]
    #[inline] pub fn imr(&self) -> Imr { 
        unsafe {
            read_volatile(self.imr_ptr())
        }
    }

    #[doc="Write the IMR register."]
    #[inline] pub fn set_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imr_mut(), f(Imr(0)));
        }
        self
    }

    #[doc="Modify the IMR register."]
    #[inline] pub fn with_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imr_mut(), f(self.imr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EMR register."]
    #[inline] pub fn emr_mut(&self) -> *mut Emr { 
        (self.0 + 0x4) as *mut Emr
    }

    #[doc="Get the *const pointer for the EMR register."]
    #[inline] pub fn emr_ptr(&self) -> *const Emr { 
           self.emr_mut()
    }

    #[doc="Read the EMR register."]
    #[inline] pub fn emr(&self) -> Emr { 
        unsafe {
            read_volatile(self.emr_ptr())
        }
    }

    #[doc="Write the EMR register."]
    #[inline] pub fn set_emr<F: FnOnce(Emr) -> Emr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.emr_mut(), f(Emr(0)));
        }
        self
    }

    #[doc="Modify the EMR register."]
    #[inline] pub fn with_emr<F: FnOnce(Emr) -> Emr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.emr_mut(), f(self.emr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RTSR register."]
    #[inline] pub fn rtsr_mut(&self) -> *mut Rtsr { 
        (self.0 + 0x8) as *mut Rtsr
    }

    #[doc="Get the *const pointer for the RTSR register."]
    #[inline] pub fn rtsr_ptr(&self) -> *const Rtsr { 
           self.rtsr_mut()
    }

    #[doc="Read the RTSR register."]
    #[inline] pub fn rtsr(&self) -> Rtsr { 
        unsafe {
            read_volatile(self.rtsr_ptr())
        }
    }

    #[doc="Write the RTSR register."]
    #[inline] pub fn set_rtsr<F: FnOnce(Rtsr) -> Rtsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rtsr_mut(), f(Rtsr(0)));
        }
        self
    }

    #[doc="Modify the RTSR register."]
    #[inline] pub fn with_rtsr<F: FnOnce(Rtsr) -> Rtsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rtsr_mut(), f(self.rtsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FTSR register."]
    #[inline] pub fn ftsr_mut(&self) -> *mut Ftsr { 
        (self.0 + 0xc) as *mut Ftsr
    }

    #[doc="Get the *const pointer for the FTSR register."]
    #[inline] pub fn ftsr_ptr(&self) -> *const Ftsr { 
           self.ftsr_mut()
    }

    #[doc="Read the FTSR register."]
    #[inline] pub fn ftsr(&self) -> Ftsr { 
        unsafe {
            read_volatile(self.ftsr_ptr())
        }
    }

    #[doc="Write the FTSR register."]
    #[inline] pub fn set_ftsr<F: FnOnce(Ftsr) -> Ftsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftsr_mut(), f(Ftsr(0)));
        }
        self
    }

    #[doc="Modify the FTSR register."]
    #[inline] pub fn with_ftsr<F: FnOnce(Ftsr) -> Ftsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ftsr_mut(), f(self.ftsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SWIER register."]
    #[inline] pub fn swier_mut(&self) -> *mut Swier { 
        (self.0 + 0x10) as *mut Swier
    }

    #[doc="Get the *const pointer for the SWIER register."]
    #[inline] pub fn swier_ptr(&self) -> *const Swier { 
           self.swier_mut()
    }

    #[doc="Read the SWIER register."]
    #[inline] pub fn swier(&self) -> Swier { 
        unsafe {
            read_volatile(self.swier_ptr())
        }
    }

    #[doc="Write the SWIER register."]
    #[inline] pub fn set_swier<F: FnOnce(Swier) -> Swier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swier_mut(), f(Swier(0)));
        }
        self
    }

    #[doc="Modify the SWIER register."]
    #[inline] pub fn with_swier<F: FnOnce(Swier) -> Swier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.swier_mut(), f(self.swier()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PR register."]
    #[inline] pub fn pr_mut(&self) -> *mut Pr { 
        (self.0 + 0x14) as *mut Pr
    }

    #[doc="Get the *const pointer for the PR register."]
    #[inline] pub fn pr_ptr(&self) -> *const Pr { 
           self.pr_mut()
    }

    #[doc="Read the PR register."]
    #[inline] pub fn pr(&self) -> Pr { 
        unsafe {
            read_volatile(self.pr_ptr())
        }
    }

    #[doc="Write the PR register."]
    #[inline] pub fn set_pr<F: FnOnce(Pr) -> Pr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pr_mut(), f(Pr(0)));
        }
        self
    }

    #[doc="Modify the PR register."]
    #[inline] pub fn with_pr<F: FnOnce(Pr) -> Pr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pr_mut(), f(self.pr()));
        }
        self
    }

}

#[doc="Interrupt mask register (EXTI_IMR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc="Interrupt Mask on line n = 0..32"]
    #[inline] pub fn mr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MR != 0"]
    #[inline] pub fn test_mr<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.mr(index) != 0
    }

    #[doc="Sets the MR field."]
    #[inline] pub fn set_mr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Imr {
    #[inline]
    fn from(other: u32) -> Self {
         Imr(other)
    }
}

impl ::core::fmt::Display for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mr(0) != 0 { try!(write!(f, " mr[0]"))}
        if self.mr(1) != 0 { try!(write!(f, " mr[1]"))}
        if self.mr(2) != 0 { try!(write!(f, " mr[2]"))}
        if self.mr(3) != 0 { try!(write!(f, " mr[3]"))}
        if self.mr(4) != 0 { try!(write!(f, " mr[4]"))}
        if self.mr(5) != 0 { try!(write!(f, " mr[5]"))}
        if self.mr(6) != 0 { try!(write!(f, " mr[6]"))}
        if self.mr(7) != 0 { try!(write!(f, " mr[7]"))}
        if self.mr(8) != 0 { try!(write!(f, " mr[8]"))}
        if self.mr(9) != 0 { try!(write!(f, " mr[9]"))}
        if self.mr(10) != 0 { try!(write!(f, " mr[10]"))}
        if self.mr(11) != 0 { try!(write!(f, " mr[11]"))}
        if self.mr(12) != 0 { try!(write!(f, " mr[12]"))}
        if self.mr(13) != 0 { try!(write!(f, " mr[13]"))}
        if self.mr(14) != 0 { try!(write!(f, " mr[14]"))}
        if self.mr(15) != 0 { try!(write!(f, " mr[15]"))}
        if self.mr(16) != 0 { try!(write!(f, " mr[16]"))}
        if self.mr(17) != 0 { try!(write!(f, " mr[17]"))}
        if self.mr(18) != 0 { try!(write!(f, " mr[18]"))}
        if self.mr(19) != 0 { try!(write!(f, " mr[19]"))}
        if self.mr(20) != 0 { try!(write!(f, " mr[20]"))}
        if self.mr(21) != 0 { try!(write!(f, " mr[21]"))}
        if self.mr(22) != 0 { try!(write!(f, " mr[22]"))}
        if self.mr(23) != 0 { try!(write!(f, " mr[23]"))}
        if self.mr(24) != 0 { try!(write!(f, " mr[24]"))}
        if self.mr(25) != 0 { try!(write!(f, " mr[25]"))}
        if self.mr(26) != 0 { try!(write!(f, " mr[26]"))}
        if self.mr(27) != 0 { try!(write!(f, " mr[27]"))}
        if self.mr(28) != 0 { try!(write!(f, " mr[28]"))}
        if self.mr(29) != 0 { try!(write!(f, " mr[29]"))}
        if self.mr(30) != 0 { try!(write!(f, " mr[30]"))}
        if self.mr(31) != 0 { try!(write!(f, " mr[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event mask register (EXTI_EMR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Emr(pub u32);
impl Emr {
    #[doc="Event Mask on line n = 0..32"]
    #[inline] pub fn mr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MR != 0"]
    #[inline] pub fn test_mr<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.mr(index) != 0
    }

    #[doc="Sets the MR field."]
    #[inline] pub fn set_mr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Emr {
    #[inline]
    fn from(other: u32) -> Self {
         Emr(other)
    }
}

impl ::core::fmt::Display for Emr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Emr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mr(0) != 0 { try!(write!(f, " mr[0]"))}
        if self.mr(1) != 0 { try!(write!(f, " mr[1]"))}
        if self.mr(2) != 0 { try!(write!(f, " mr[2]"))}
        if self.mr(3) != 0 { try!(write!(f, " mr[3]"))}
        if self.mr(4) != 0 { try!(write!(f, " mr[4]"))}
        if self.mr(5) != 0 { try!(write!(f, " mr[5]"))}
        if self.mr(6) != 0 { try!(write!(f, " mr[6]"))}
        if self.mr(7) != 0 { try!(write!(f, " mr[7]"))}
        if self.mr(8) != 0 { try!(write!(f, " mr[8]"))}
        if self.mr(9) != 0 { try!(write!(f, " mr[9]"))}
        if self.mr(10) != 0 { try!(write!(f, " mr[10]"))}
        if self.mr(11) != 0 { try!(write!(f, " mr[11]"))}
        if self.mr(12) != 0 { try!(write!(f, " mr[12]"))}
        if self.mr(13) != 0 { try!(write!(f, " mr[13]"))}
        if self.mr(14) != 0 { try!(write!(f, " mr[14]"))}
        if self.mr(15) != 0 { try!(write!(f, " mr[15]"))}
        if self.mr(16) != 0 { try!(write!(f, " mr[16]"))}
        if self.mr(17) != 0 { try!(write!(f, " mr[17]"))}
        if self.mr(18) != 0 { try!(write!(f, " mr[18]"))}
        if self.mr(19) != 0 { try!(write!(f, " mr[19]"))}
        if self.mr(20) != 0 { try!(write!(f, " mr[20]"))}
        if self.mr(21) != 0 { try!(write!(f, " mr[21]"))}
        if self.mr(22) != 0 { try!(write!(f, " mr[22]"))}
        if self.mr(23) != 0 { try!(write!(f, " mr[23]"))}
        if self.mr(24) != 0 { try!(write!(f, " mr[24]"))}
        if self.mr(25) != 0 { try!(write!(f, " mr[25]"))}
        if self.mr(26) != 0 { try!(write!(f, " mr[26]"))}
        if self.mr(27) != 0 { try!(write!(f, " mr[27]"))}
        if self.mr(28) != 0 { try!(write!(f, " mr[28]"))}
        if self.mr(29) != 0 { try!(write!(f, " mr[29]"))}
        if self.mr(30) != 0 { try!(write!(f, " mr[30]"))}
        if self.mr(31) != 0 { try!(write!(f, " mr[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rising Trigger selection register (EXTI_RTSR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rtsr(pub u32);
impl Rtsr {
    #[doc="Rising trigger event configuration of line n = 0..32"]
    #[inline] pub fn tr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TR != 0"]
    #[inline] pub fn test_tr<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.tr(index) != 0
    }

    #[doc="Sets the TR field."]
    #[inline] pub fn set_tr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Rtsr {
    #[inline]
    fn from(other: u32) -> Self {
         Rtsr(other)
    }
}

impl ::core::fmt::Display for Rtsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rtsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tr(0) != 0 { try!(write!(f, " tr[0]"))}
        if self.tr(1) != 0 { try!(write!(f, " tr[1]"))}
        if self.tr(2) != 0 { try!(write!(f, " tr[2]"))}
        if self.tr(3) != 0 { try!(write!(f, " tr[3]"))}
        if self.tr(4) != 0 { try!(write!(f, " tr[4]"))}
        if self.tr(5) != 0 { try!(write!(f, " tr[5]"))}
        if self.tr(6) != 0 { try!(write!(f, " tr[6]"))}
        if self.tr(7) != 0 { try!(write!(f, " tr[7]"))}
        if self.tr(8) != 0 { try!(write!(f, " tr[8]"))}
        if self.tr(9) != 0 { try!(write!(f, " tr[9]"))}
        if self.tr(10) != 0 { try!(write!(f, " tr[10]"))}
        if self.tr(11) != 0 { try!(write!(f, " tr[11]"))}
        if self.tr(12) != 0 { try!(write!(f, " tr[12]"))}
        if self.tr(13) != 0 { try!(write!(f, " tr[13]"))}
        if self.tr(14) != 0 { try!(write!(f, " tr[14]"))}
        if self.tr(15) != 0 { try!(write!(f, " tr[15]"))}
        if self.tr(16) != 0 { try!(write!(f, " tr[16]"))}
        if self.tr(17) != 0 { try!(write!(f, " tr[17]"))}
        if self.tr(18) != 0 { try!(write!(f, " tr[18]"))}
        if self.tr(19) != 0 { try!(write!(f, " tr[19]"))}
        if self.tr(20) != 0 { try!(write!(f, " tr[20]"))}
        if self.tr(21) != 0 { try!(write!(f, " tr[21]"))}
        if self.tr(22) != 0 { try!(write!(f, " tr[22]"))}
        if self.tr(23) != 0 { try!(write!(f, " tr[23]"))}
        if self.tr(24) != 0 { try!(write!(f, " tr[24]"))}
        if self.tr(25) != 0 { try!(write!(f, " tr[25]"))}
        if self.tr(26) != 0 { try!(write!(f, " tr[26]"))}
        if self.tr(27) != 0 { try!(write!(f, " tr[27]"))}
        if self.tr(28) != 0 { try!(write!(f, " tr[28]"))}
        if self.tr(29) != 0 { try!(write!(f, " tr[29]"))}
        if self.tr(30) != 0 { try!(write!(f, " tr[30]"))}
        if self.tr(31) != 0 { try!(write!(f, " tr[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Falling Trigger selection register (EXTI_FTSR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftsr(pub u32);
impl Ftsr {
    #[doc="Falling trigger event configuration of line n = 0..32"]
    #[inline] pub fn tr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TR != 0"]
    #[inline] pub fn test_tr<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.tr(index) != 0
    }

    #[doc="Sets the TR field."]
    #[inline] pub fn set_tr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ftsr {
    #[inline]
    fn from(other: u32) -> Self {
         Ftsr(other)
    }
}

impl ::core::fmt::Display for Ftsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ftsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tr(0) != 0 { try!(write!(f, " tr[0]"))}
        if self.tr(1) != 0 { try!(write!(f, " tr[1]"))}
        if self.tr(2) != 0 { try!(write!(f, " tr[2]"))}
        if self.tr(3) != 0 { try!(write!(f, " tr[3]"))}
        if self.tr(4) != 0 { try!(write!(f, " tr[4]"))}
        if self.tr(5) != 0 { try!(write!(f, " tr[5]"))}
        if self.tr(6) != 0 { try!(write!(f, " tr[6]"))}
        if self.tr(7) != 0 { try!(write!(f, " tr[7]"))}
        if self.tr(8) != 0 { try!(write!(f, " tr[8]"))}
        if self.tr(9) != 0 { try!(write!(f, " tr[9]"))}
        if self.tr(10) != 0 { try!(write!(f, " tr[10]"))}
        if self.tr(11) != 0 { try!(write!(f, " tr[11]"))}
        if self.tr(12) != 0 { try!(write!(f, " tr[12]"))}
        if self.tr(13) != 0 { try!(write!(f, " tr[13]"))}
        if self.tr(14) != 0 { try!(write!(f, " tr[14]"))}
        if self.tr(15) != 0 { try!(write!(f, " tr[15]"))}
        if self.tr(16) != 0 { try!(write!(f, " tr[16]"))}
        if self.tr(17) != 0 { try!(write!(f, " tr[17]"))}
        if self.tr(18) != 0 { try!(write!(f, " tr[18]"))}
        if self.tr(19) != 0 { try!(write!(f, " tr[19]"))}
        if self.tr(20) != 0 { try!(write!(f, " tr[20]"))}
        if self.tr(21) != 0 { try!(write!(f, " tr[21]"))}
        if self.tr(22) != 0 { try!(write!(f, " tr[22]"))}
        if self.tr(23) != 0 { try!(write!(f, " tr[23]"))}
        if self.tr(24) != 0 { try!(write!(f, " tr[24]"))}
        if self.tr(25) != 0 { try!(write!(f, " tr[25]"))}
        if self.tr(26) != 0 { try!(write!(f, " tr[26]"))}
        if self.tr(27) != 0 { try!(write!(f, " tr[27]"))}
        if self.tr(28) != 0 { try!(write!(f, " tr[28]"))}
        if self.tr(29) != 0 { try!(write!(f, " tr[29]"))}
        if self.tr(30) != 0 { try!(write!(f, " tr[30]"))}
        if self.tr(31) != 0 { try!(write!(f, " tr[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software interrupt event register (EXTI_SWIER)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swier(pub u32);
impl Swier {
    #[doc="Software Interrupt on line n = 0..32"]
    #[inline] pub fn swi<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWI != 0"]
    #[inline] pub fn test_swi<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.swi(index) != 0
    }

    #[doc="Sets the SWI field."]
    #[inline] pub fn set_swi<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Swier {
    #[inline]
    fn from(other: u32) -> Self {
         Swier(other)
    }
}

impl ::core::fmt::Display for Swier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swi(0) != 0 { try!(write!(f, " swi[0]"))}
        if self.swi(1) != 0 { try!(write!(f, " swi[1]"))}
        if self.swi(2) != 0 { try!(write!(f, " swi[2]"))}
        if self.swi(3) != 0 { try!(write!(f, " swi[3]"))}
        if self.swi(4) != 0 { try!(write!(f, " swi[4]"))}
        if self.swi(5) != 0 { try!(write!(f, " swi[5]"))}
        if self.swi(6) != 0 { try!(write!(f, " swi[6]"))}
        if self.swi(7) != 0 { try!(write!(f, " swi[7]"))}
        if self.swi(8) != 0 { try!(write!(f, " swi[8]"))}
        if self.swi(9) != 0 { try!(write!(f, " swi[9]"))}
        if self.swi(10) != 0 { try!(write!(f, " swi[10]"))}
        if self.swi(11) != 0 { try!(write!(f, " swi[11]"))}
        if self.swi(12) != 0 { try!(write!(f, " swi[12]"))}
        if self.swi(13) != 0 { try!(write!(f, " swi[13]"))}
        if self.swi(14) != 0 { try!(write!(f, " swi[14]"))}
        if self.swi(15) != 0 { try!(write!(f, " swi[15]"))}
        if self.swi(16) != 0 { try!(write!(f, " swi[16]"))}
        if self.swi(17) != 0 { try!(write!(f, " swi[17]"))}
        if self.swi(18) != 0 { try!(write!(f, " swi[18]"))}
        if self.swi(19) != 0 { try!(write!(f, " swi[19]"))}
        if self.swi(20) != 0 { try!(write!(f, " swi[20]"))}
        if self.swi(21) != 0 { try!(write!(f, " swi[21]"))}
        if self.swi(22) != 0 { try!(write!(f, " swi[22]"))}
        if self.swi(23) != 0 { try!(write!(f, " swi[23]"))}
        if self.swi(24) != 0 { try!(write!(f, " swi[24]"))}
        if self.swi(25) != 0 { try!(write!(f, " swi[25]"))}
        if self.swi(26) != 0 { try!(write!(f, " swi[26]"))}
        if self.swi(27) != 0 { try!(write!(f, " swi[27]"))}
        if self.swi(28) != 0 { try!(write!(f, " swi[28]"))}
        if self.swi(29) != 0 { try!(write!(f, " swi[29]"))}
        if self.swi(30) != 0 { try!(write!(f, " swi[30]"))}
        if self.swi(31) != 0 { try!(write!(f, " swi[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pending interrupt register (EXTI_PR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc="Pending Interrupt bit n = 0..32"]
    #[inline] pub fn pr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.pr(index) != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pr {
    #[inline]
    fn from(other: u32) -> Self {
         Pr(other)
    }
}

impl ::core::fmt::Display for Pr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pr(0) != 0 { try!(write!(f, " pr[0]"))}
        if self.pr(1) != 0 { try!(write!(f, " pr[1]"))}
        if self.pr(2) != 0 { try!(write!(f, " pr[2]"))}
        if self.pr(3) != 0 { try!(write!(f, " pr[3]"))}
        if self.pr(4) != 0 { try!(write!(f, " pr[4]"))}
        if self.pr(5) != 0 { try!(write!(f, " pr[5]"))}
        if self.pr(6) != 0 { try!(write!(f, " pr[6]"))}
        if self.pr(7) != 0 { try!(write!(f, " pr[7]"))}
        if self.pr(8) != 0 { try!(write!(f, " pr[8]"))}
        if self.pr(9) != 0 { try!(write!(f, " pr[9]"))}
        if self.pr(10) != 0 { try!(write!(f, " pr[10]"))}
        if self.pr(11) != 0 { try!(write!(f, " pr[11]"))}
        if self.pr(12) != 0 { try!(write!(f, " pr[12]"))}
        if self.pr(13) != 0 { try!(write!(f, " pr[13]"))}
        if self.pr(14) != 0 { try!(write!(f, " pr[14]"))}
        if self.pr(15) != 0 { try!(write!(f, " pr[15]"))}
        if self.pr(16) != 0 { try!(write!(f, " pr[16]"))}
        if self.pr(17) != 0 { try!(write!(f, " pr[17]"))}
        if self.pr(18) != 0 { try!(write!(f, " pr[18]"))}
        if self.pr(19) != 0 { try!(write!(f, " pr[19]"))}
        if self.pr(20) != 0 { try!(write!(f, " pr[20]"))}
        if self.pr(21) != 0 { try!(write!(f, " pr[21]"))}
        if self.pr(22) != 0 { try!(write!(f, " pr[22]"))}
        if self.pr(23) != 0 { try!(write!(f, " pr[23]"))}
        if self.pr(24) != 0 { try!(write!(f, " pr[24]"))}
        if self.pr(25) != 0 { try!(write!(f, " pr[25]"))}
        if self.pr(26) != 0 { try!(write!(f, " pr[26]"))}
        if self.pr(27) != 0 { try!(write!(f, " pr[27]"))}
        if self.pr(28) != 0 { try!(write!(f, " pr[28]"))}
        if self.pr(29) != 0 { try!(write!(f, " pr[29]"))}
        if self.pr(30) != 0 { try!(write!(f, " pr[30]"))}
        if self.pr(31) != 0 { try!(write!(f, " pr[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub struct ExtiCh { pub periph: ExtiPeriph, pub index: usize }

