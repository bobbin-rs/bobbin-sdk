//! Secure Real Time Clock
#[allow(unused_imports)] use bobbin_common::*;

periph!(RTC, Rtc, 0x4003d000);

#[doc="Secure Real Time Clock"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rtc(pub usize);
impl Rtc {
    #[doc="Get the *const pointer for the TSR register."]
    #[inline] pub fn tsr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x0) as *const u32
    }

    #[doc="Get the *mut pointer for the TSR register."]
    #[inline] pub fn tsr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x0) as *mut u32
    }

    #[doc="Read the TSR register."]
    #[inline] pub fn tsr(&self) -> Tsr { 
        unsafe {
            Tsr(read_volatile((self.0 + 0x0) as *const u32))
        }
    }

    #[doc="Write the TSR register."]
    #[inline] pub fn set_tsr<F: FnOnce(Tsr) -> Tsr>(&self, f: F) -> &Self {
        let value = f(Tsr(0));
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TSR register."]
    #[inline] pub fn with_tsr<F: FnOnce(Tsr) -> Tsr>(&self, f: F) -> &Self {
        let tmp = self.tsr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TPR register."]
    #[inline] pub fn tpr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x4) as *const u32
    }

    #[doc="Get the *mut pointer for the TPR register."]
    #[inline] pub fn tpr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x4) as *mut u32
    }

    #[doc="Read the TPR register."]
    #[inline] pub fn tpr(&self) -> Tpr { 
        unsafe {
            Tpr(read_volatile((self.0 + 0x4) as *const u32))
        }
    }

    #[doc="Write the TPR register."]
    #[inline] pub fn set_tpr<F: FnOnce(Tpr) -> Tpr>(&self, f: F) -> &Self {
        let value = f(Tpr(0));
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TPR register."]
    #[inline] pub fn with_tpr<F: FnOnce(Tpr) -> Tpr>(&self, f: F) -> &Self {
        let tmp = self.tpr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TAR register."]
    #[inline] pub fn tar_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x8) as *const u32
    }

    #[doc="Get the *mut pointer for the TAR register."]
    #[inline] pub fn tar_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x8) as *mut u32
    }

    #[doc="Read the TAR register."]
    #[inline] pub fn tar(&self) -> Tar { 
        unsafe {
            Tar(read_volatile((self.0 + 0x8) as *const u32))
        }
    }

    #[doc="Write the TAR register."]
    #[inline] pub fn set_tar<F: FnOnce(Tar) -> Tar>(&self, f: F) -> &Self {
        let value = f(Tar(0));
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TAR register."]
    #[inline] pub fn with_tar<F: FnOnce(Tar) -> Tar>(&self, f: F) -> &Self {
        let tmp = self.tar();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the TCR register."]
    #[inline] pub fn tcr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc) as *const u32
    }

    #[doc="Get the *mut pointer for the TCR register."]
    #[inline] pub fn tcr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc) as *mut u32
    }

    #[doc="Read the TCR register."]
    #[inline] pub fn tcr(&self) -> Tcr { 
        unsafe {
            Tcr(read_volatile((self.0 + 0xc) as *const u32))
        }
    }

    #[doc="Write the TCR register."]
    #[inline] pub fn set_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        let value = f(Tcr(0));
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the TCR register."]
    #[inline] pub fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&self, f: F) -> &Self {
        let tmp = self.tcr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x10) as *const u32
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x10) as *mut u32
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            Cr(read_volatile((self.0 + 0x10) as *const u32))
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        let value = f(Cr(0));
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        let tmp = self.cr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x14) as *const u32
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x14) as *mut u32
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        unsafe {
            Sr(read_volatile((self.0 + 0x14) as *const u32))
        }
    }

    #[doc="Write the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        let value = f(Sr(0));
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        let tmp = self.sr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the LR register."]
    #[inline] pub fn lr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x18) as *const u32
    }

    #[doc="Get the *mut pointer for the LR register."]
    #[inline] pub fn lr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x18) as *mut u32
    }

    #[doc="Read the LR register."]
    #[inline] pub fn lr(&self) -> Lr { 
        unsafe {
            Lr(read_volatile((self.0 + 0x18) as *const u32))
        }
    }

    #[doc="Write the LR register."]
    #[inline] pub fn set_lr<F: FnOnce(Lr) -> Lr>(&self, f: F) -> &Self {
        let value = f(Lr(0));
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the LR register."]
    #[inline] pub fn with_lr<F: FnOnce(Lr) -> Lr>(&self, f: F) -> &Self {
        let tmp = self.lr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x1c) as *const u32
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x1c) as *mut u32
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        unsafe {
            Ier(read_volatile((self.0 + 0x1c) as *const u32))
        }
    }

    #[doc="Write the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        let value = f(Ier(0));
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        let tmp = self.ier();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u32, value.0);
        }
        self
    }

}

#[doc="RTC Time Seconds Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc="Time Seconds Register"]
    #[inline] pub fn tsr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Time Seconds Register"]
    #[inline] pub fn test_tsr(&self) -> bool {
        self.tsr() != 0
    }

    #[doc="Time Seconds Register"]
    #[inline] pub fn set_tsr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tsr {
    #[inline]
    fn from(other: u32) -> Self {
         Tsr(other)
    }
}

impl ::core::fmt::Display for Tsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Time Prescaler Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tpr(pub u32);
impl Tpr {
    #[doc="Time Prescaler Register"]
    #[inline] pub fn tpr(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Time Prescaler Register"]
    #[inline] pub fn test_tpr(&self) -> bool {
        self.tpr() != 0
    }

    #[doc="Time Prescaler Register"]
    #[inline] pub fn set_tpr<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tpr {
    #[inline]
    fn from(other: u32) -> Self {
         Tpr(other)
    }
}

impl ::core::fmt::Display for Tpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tpr() != 0 { try!(write!(f, " tpr=0x{:x}", self.tpr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Time Alarm Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tar(pub u32);
impl Tar {
    #[doc="Time Alarm Register"]
    #[inline] pub fn tar(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Time Alarm Register"]
    #[inline] pub fn test_tar(&self) -> bool {
        self.tar() != 0
    }

    #[doc="Time Alarm Register"]
    #[inline] pub fn set_tar<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tar {
    #[inline]
    fn from(other: u32) -> Self {
         Tar(other)
    }
}

impl ::core::fmt::Display for Tar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Time Compensation Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc="Time Compensation Register"]
    #[inline] pub fn tcr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Time Compensation Register"]
    #[inline] pub fn test_tcr(&self) -> bool {
        self.tcr() != 0
    }

    #[doc="Time Compensation Register"]
    #[inline] pub fn set_tcr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Compensation Interval Register"]
    #[inline] pub fn cir(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Compensation Interval Register"]
    #[inline] pub fn test_cir(&self) -> bool {
        self.cir() != 0
    }

    #[doc="Compensation Interval Register"]
    #[inline] pub fn set_cir<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Time Compensation Value"]
    #[inline] pub fn tcv(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Time Compensation Value"]
    #[inline] pub fn test_tcv(&self) -> bool {
        self.tcv() != 0
    }

    #[doc="Time Compensation Value"]
    #[inline] pub fn set_tcv<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Compensation Interval Counter"]
    #[inline] pub fn cic(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Compensation Interval Counter"]
    #[inline] pub fn test_cic(&self) -> bool {
        self.cic() != 0
    }

    #[doc="Compensation Interval Counter"]
    #[inline] pub fn set_cic<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Tcr {
    #[inline]
    fn from(other: u32) -> Self {
         Tcr(other)
    }
}

impl ::core::fmt::Display for Tcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tcr() != 0 { try!(write!(f, " tcr=0x{:x}", self.tcr()))}
        if self.cir() != 0 { try!(write!(f, " cir=0x{:x}", self.cir()))}
        if self.tcv() != 0 { try!(write!(f, " tcv=0x{:x}", self.tcv()))}
        if self.cic() != 0 { try!(write!(f, " cic=0x{:x}", self.cic()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Software Reset"]
    #[inline] pub fn swr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Software Reset"]
    #[inline] pub fn test_swr(&self) -> bool {
        self.swr() != 0
    }

    #[doc="Software Reset"]
    #[inline] pub fn set_swr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Supervisor Access"]
    #[inline] pub fn sup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Supervisor Access"]
    #[inline] pub fn test_sup(&self) -> bool {
        self.sup() != 0
    }

    #[doc="Supervisor Access"]
    #[inline] pub fn set_sup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Update Mode"]
    #[inline] pub fn um(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Update Mode"]
    #[inline] pub fn test_um(&self) -> bool {
        self.um() != 0
    }

    #[doc="Update Mode"]
    #[inline] pub fn set_um<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clock Pin Select"]
    #[inline] pub fn cps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Clock Pin Select"]
    #[inline] pub fn test_cps(&self) -> bool {
        self.cps() != 0
    }

    #[doc="Clock Pin Select"]
    #[inline] pub fn set_cps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="LPO Select"]
    #[inline] pub fn lpos(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="LPO Select"]
    #[inline] pub fn test_lpos(&self) -> bool {
        self.lpos() != 0
    }

    #[doc="LPO Select"]
    #[inline] pub fn set_lpos<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Clock Pin Enable"]
    #[inline] pub fn cpe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Clock Pin Enable"]
    #[inline] pub fn test_cpe(&self) -> bool {
        self.cpe() != 0
    }

    #[doc="Clock Pin Enable"]
    #[inline] pub fn set_cpe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swr() != 0 { try!(write!(f, " swr"))}
        if self.sup() != 0 { try!(write!(f, " sup"))}
        if self.um() != 0 { try!(write!(f, " um"))}
        if self.cps() != 0 { try!(write!(f, " cps"))}
        if self.lpos() != 0 { try!(write!(f, " lpos"))}
        if self.cpe() != 0 { try!(write!(f, " cpe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Time Invalid Flag"]
    #[inline] pub fn tif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Time Invalid Flag"]
    #[inline] pub fn test_tif(&self) -> bool {
        self.tif() != 0
    }

    #[doc="Time Invalid Flag"]
    #[inline] pub fn set_tif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Time Overflow Flag"]
    #[inline] pub fn tof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Time Overflow Flag"]
    #[inline] pub fn test_tof(&self) -> bool {
        self.tof() != 0
    }

    #[doc="Time Overflow Flag"]
    #[inline] pub fn set_tof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Time Alarm Flag"]
    #[inline] pub fn taf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Time Alarm Flag"]
    #[inline] pub fn test_taf(&self) -> bool {
        self.taf() != 0
    }

    #[doc="Time Alarm Flag"]
    #[inline] pub fn set_taf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Time Counter Enable"]
    #[inline] pub fn tce(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Time Counter Enable"]
    #[inline] pub fn test_tce(&self) -> bool {
        self.tce() != 0
    }

    #[doc="Time Counter Enable"]
    #[inline] pub fn set_tce<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
    }
}

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tif() != 0 { try!(write!(f, " tif"))}
        if self.tof() != 0 { try!(write!(f, " tof"))}
        if self.taf() != 0 { try!(write!(f, " taf"))}
        if self.tce() != 0 { try!(write!(f, " tce"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Lock Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lr(pub u32);
impl Lr {
    #[doc="Time Compensation Lock"]
    #[inline] pub fn tcl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Time Compensation Lock"]
    #[inline] pub fn test_tcl(&self) -> bool {
        self.tcl() != 0
    }

    #[doc="Time Compensation Lock"]
    #[inline] pub fn set_tcl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Control Register Lock"]
    #[inline] pub fn crl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Control Register Lock"]
    #[inline] pub fn test_crl(&self) -> bool {
        self.crl() != 0
    }

    #[doc="Control Register Lock"]
    #[inline] pub fn set_crl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Status Register Lock"]
    #[inline] pub fn srl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Status Register Lock"]
    #[inline] pub fn test_srl(&self) -> bool {
        self.srl() != 0
    }

    #[doc="Status Register Lock"]
    #[inline] pub fn set_srl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Lock Register Lock"]
    #[inline] pub fn lrl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Lock Register Lock"]
    #[inline] pub fn test_lrl(&self) -> bool {
        self.lrl() != 0
    }

    #[doc="Lock Register Lock"]
    #[inline] pub fn set_lrl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Lr {
    #[inline]
    fn from(other: u32) -> Self {
         Lr(other)
    }
}

impl ::core::fmt::Display for Lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tcl() != 0 { try!(write!(f, " tcl"))}
        if self.crl() != 0 { try!(write!(f, " crl"))}
        if self.srl() != 0 { try!(write!(f, " srl"))}
        if self.lrl() != 0 { try!(write!(f, " lrl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Time Invalid Interrupt Enable"]
    #[inline] pub fn tiie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Time Invalid Interrupt Enable"]
    #[inline] pub fn test_tiie(&self) -> bool {
        self.tiie() != 0
    }

    #[doc="Time Invalid Interrupt Enable"]
    #[inline] pub fn set_tiie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Time Overflow Interrupt Enable"]
    #[inline] pub fn toie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Time Overflow Interrupt Enable"]
    #[inline] pub fn test_toie(&self) -> bool {
        self.toie() != 0
    }

    #[doc="Time Overflow Interrupt Enable"]
    #[inline] pub fn set_toie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Time Alarm Interrupt Enable"]
    #[inline] pub fn taie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Time Alarm Interrupt Enable"]
    #[inline] pub fn test_taie(&self) -> bool {
        self.taie() != 0
    }

    #[doc="Time Alarm Interrupt Enable"]
    #[inline] pub fn set_taie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Time Seconds Interrupt Enable"]
    #[inline] pub fn tsie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Time Seconds Interrupt Enable"]
    #[inline] pub fn test_tsie(&self) -> bool {
        self.tsie() != 0
    }

    #[doc="Time Seconds Interrupt Enable"]
    #[inline] pub fn set_tsie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Timer Seconds Interrupt Configuration"]
    #[inline] pub fn tsic(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Timer Seconds Interrupt Configuration"]
    #[inline] pub fn test_tsic(&self) -> bool {
        self.tsic() != 0
    }

    #[doc="Timer Seconds Interrupt Configuration"]
    #[inline] pub fn set_tsic<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Ier {
    #[inline]
    fn from(other: u32) -> Self {
         Ier(other)
    }
}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tiie() != 0 { try!(write!(f, " tiie"))}
        if self.toie() != 0 { try!(write!(f, " toie"))}
        if self.taie() != 0 { try!(write!(f, " taie"))}
        if self.tsie() != 0 { try!(write!(f, " tsie"))}
        if self.tsic() != 0 { try!(write!(f, " tsic=0x{:x}", self.tsic()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


