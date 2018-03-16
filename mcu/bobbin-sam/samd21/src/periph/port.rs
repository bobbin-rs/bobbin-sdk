#[allow(unused_imports)] use ::bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PORT Peripheral"]
pub struct PortPeriph(pub usize); 

pub struct PortPin { pub port: PortPeriph, pub index: usize }
impl PortPeriph {
    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        (self.0 + 0x24) as *mut Ctrl
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

    #[doc="Get the *mut pointer for the DIR register."]
    #[inline] pub fn dir_mut(&self) -> *mut Dir { 
        (self.0 + 0x0) as *mut Dir
    }

    #[doc="Get the *const pointer for the DIR register."]
    #[inline] pub fn dir_ptr(&self) -> *const Dir { 
           self.dir_mut()
    }

    #[doc="Read the DIR register."]
    #[inline] pub fn dir(&self) -> Dir { 
        unsafe {
            read_volatile(self.dir_ptr())
        }
    }

    #[doc="Write the DIR register."]
    #[inline] pub fn set_dir<F: FnOnce(Dir) -> Dir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dir_mut(), f(Dir(0)));
        }
        self
    }

    #[doc="Modify the DIR register."]
    #[inline] pub fn with_dir<F: FnOnce(Dir) -> Dir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dir_mut(), f(self.dir()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIRCLR register."]
    #[inline] pub fn dirclr_mut(&self) -> *mut Dirclr { 
        (self.0 + 0x4) as *mut Dirclr
    }

    #[doc="Get the *const pointer for the DIRCLR register."]
    #[inline] pub fn dirclr_ptr(&self) -> *const Dirclr { 
           self.dirclr_mut()
    }

    #[doc="Read the DIRCLR register."]
    #[inline] pub fn dirclr(&self) -> Dirclr { 
        unsafe {
            read_volatile(self.dirclr_ptr())
        }
    }

    #[doc="Write the DIRCLR register."]
    #[inline] pub fn set_dirclr<F: FnOnce(Dirclr) -> Dirclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dirclr_mut(), f(Dirclr(0)));
        }
        self
    }

    #[doc="Modify the DIRCLR register."]
    #[inline] pub fn with_dirclr<F: FnOnce(Dirclr) -> Dirclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dirclr_mut(), f(self.dirclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIRSET register."]
    #[inline] pub fn dirset_mut(&self) -> *mut Dirset { 
        (self.0 + 0x8) as *mut Dirset
    }

    #[doc="Get the *const pointer for the DIRSET register."]
    #[inline] pub fn dirset_ptr(&self) -> *const Dirset { 
           self.dirset_mut()
    }

    #[doc="Read the DIRSET register."]
    #[inline] pub fn dirset(&self) -> Dirset { 
        unsafe {
            read_volatile(self.dirset_ptr())
        }
    }

    #[doc="Write the DIRSET register."]
    #[inline] pub fn set_dirset<F: FnOnce(Dirset) -> Dirset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dirset_mut(), f(Dirset(0)));
        }
        self
    }

    #[doc="Modify the DIRSET register."]
    #[inline] pub fn with_dirset<F: FnOnce(Dirset) -> Dirset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dirset_mut(), f(self.dirset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DIRTGL register."]
    #[inline] pub fn dirtgl_mut(&self) -> *mut Dirtgl { 
        (self.0 + 0xc) as *mut Dirtgl
    }

    #[doc="Get the *const pointer for the DIRTGL register."]
    #[inline] pub fn dirtgl_ptr(&self) -> *const Dirtgl { 
           self.dirtgl_mut()
    }

    #[doc="Read the DIRTGL register."]
    #[inline] pub fn dirtgl(&self) -> Dirtgl { 
        unsafe {
            read_volatile(self.dirtgl_ptr())
        }
    }

    #[doc="Write the DIRTGL register."]
    #[inline] pub fn set_dirtgl<F: FnOnce(Dirtgl) -> Dirtgl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dirtgl_mut(), f(Dirtgl(0)));
        }
        self
    }

    #[doc="Modify the DIRTGL register."]
    #[inline] pub fn with_dirtgl<F: FnOnce(Dirtgl) -> Dirtgl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dirtgl_mut(), f(self.dirtgl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IN register."]
    #[inline] pub fn in_mut(&self) -> *mut In { 
        (self.0 + 0x20) as *mut In
    }

    #[doc="Get the *const pointer for the IN register."]
    #[inline] pub fn in_ptr(&self) -> *const In { 
           self.in_mut()
    }

    #[doc="Read the IN register."]
    #[inline] pub fn _in(&self) -> In { 
        unsafe {
            read_volatile(self.in_ptr())
        }
    }

    #[doc="Get the *mut pointer for the OUT register."]
    #[inline] pub fn out_mut(&self) -> *mut Out { 
        (self.0 + 0x10) as *mut Out
    }

    #[doc="Get the *const pointer for the OUT register."]
    #[inline] pub fn out_ptr(&self) -> *const Out { 
           self.out_mut()
    }

    #[doc="Read the OUT register."]
    #[inline] pub fn out(&self) -> Out { 
        unsafe {
            read_volatile(self.out_ptr())
        }
    }

    #[doc="Write the OUT register."]
    #[inline] pub fn set_out<F: FnOnce(Out) -> Out>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.out_mut(), f(Out(0)));
        }
        self
    }

    #[doc="Modify the OUT register."]
    #[inline] pub fn with_out<F: FnOnce(Out) -> Out>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.out_mut(), f(self.out()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OUTCLR register."]
    #[inline] pub fn outclr_mut(&self) -> *mut Outclr { 
        (self.0 + 0x14) as *mut Outclr
    }

    #[doc="Get the *const pointer for the OUTCLR register."]
    #[inline] pub fn outclr_ptr(&self) -> *const Outclr { 
           self.outclr_mut()
    }

    #[doc="Read the OUTCLR register."]
    #[inline] pub fn outclr(&self) -> Outclr { 
        unsafe {
            read_volatile(self.outclr_ptr())
        }
    }

    #[doc="Write the OUTCLR register."]
    #[inline] pub fn set_outclr<F: FnOnce(Outclr) -> Outclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.outclr_mut(), f(Outclr(0)));
        }
        self
    }

    #[doc="Modify the OUTCLR register."]
    #[inline] pub fn with_outclr<F: FnOnce(Outclr) -> Outclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.outclr_mut(), f(self.outclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OUTSET register."]
    #[inline] pub fn outset_mut(&self) -> *mut Outset { 
        (self.0 + 0x18) as *mut Outset
    }

    #[doc="Get the *const pointer for the OUTSET register."]
    #[inline] pub fn outset_ptr(&self) -> *const Outset { 
           self.outset_mut()
    }

    #[doc="Read the OUTSET register."]
    #[inline] pub fn outset(&self) -> Outset { 
        unsafe {
            read_volatile(self.outset_ptr())
        }
    }

    #[doc="Write the OUTSET register."]
    #[inline] pub fn set_outset<F: FnOnce(Outset) -> Outset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.outset_mut(), f(Outset(0)));
        }
        self
    }

    #[doc="Modify the OUTSET register."]
    #[inline] pub fn with_outset<F: FnOnce(Outset) -> Outset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.outset_mut(), f(self.outset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OUTTGL register."]
    #[inline] pub fn outtgl_mut(&self) -> *mut Outtgl { 
        (self.0 + 0x1c) as *mut Outtgl
    }

    #[doc="Get the *const pointer for the OUTTGL register."]
    #[inline] pub fn outtgl_ptr(&self) -> *const Outtgl { 
           self.outtgl_mut()
    }

    #[doc="Read the OUTTGL register."]
    #[inline] pub fn outtgl(&self) -> Outtgl { 
        unsafe {
            read_volatile(self.outtgl_ptr())
        }
    }

    #[doc="Write the OUTTGL register."]
    #[inline] pub fn set_outtgl<F: FnOnce(Outtgl) -> Outtgl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.outtgl_mut(), f(Outtgl(0)));
        }
        self
    }

    #[doc="Modify the OUTTGL register."]
    #[inline] pub fn with_outtgl<F: FnOnce(Outtgl) -> Outtgl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.outtgl_mut(), f(self.outtgl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PINCFG register."]
    #[inline] pub fn pincfg_mut<I: Into<bits::R32>>(&self, index: I) -> *mut Pincfg { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x40 + (index)) as *mut Pincfg
    }

    #[doc="Get the *const pointer for the PINCFG register."]
    #[inline] pub fn pincfg_ptr<I: Into<bits::R32>>(&self, index: I) -> *const Pincfg { 
           self.pincfg_mut(index)
    }

    #[doc="Read the PINCFG register."]
    #[inline] pub fn pincfg<I: Into<bits::R32>>(&self, index: I) -> Pincfg { 
        unsafe {
            read_volatile(self.pincfg_ptr(index))
        }
    }

    #[doc="Write the PINCFG register."]
    #[inline] pub fn set_pincfg<I: Into<bits::R32>, F: FnOnce(Pincfg) -> Pincfg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.pincfg_mut(index), f(Pincfg(0)));
        }
        self
    }

    #[doc="Modify the PINCFG register."]
    #[inline] pub fn with_pincfg<I: Into<bits::R32> + Copy, F: FnOnce(Pincfg) -> Pincfg>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.pincfg_mut(index), f(self.pincfg(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the PMUX register."]
    #[inline] pub fn pmux_mut<I: Into<bits::R16>>(&self, index: I) -> *mut Pmux { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x30 + (index)) as *mut Pmux
    }

    #[doc="Get the *const pointer for the PMUX register."]
    #[inline] pub fn pmux_ptr<I: Into<bits::R16>>(&self, index: I) -> *const Pmux { 
           self.pmux_mut(index)
    }

    #[doc="Read the PMUX register."]
    #[inline] pub fn pmux<I: Into<bits::R16>>(&self, index: I) -> Pmux { 
        unsafe {
            read_volatile(self.pmux_ptr(index))
        }
    }

    #[doc="Write the PMUX register."]
    #[inline] pub fn set_pmux<I: Into<bits::R16>, F: FnOnce(Pmux) -> Pmux>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.pmux_mut(index), f(Pmux(0)));
        }
        self
    }

    #[doc="Modify the PMUX register."]
    #[inline] pub fn with_pmux<I: Into<bits::R16> + Copy, F: FnOnce(Pmux) -> Pmux>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.pmux_mut(index), f(self.pmux(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the WRCONFIG register."]
    #[inline] pub fn wrconfig_mut(&self) -> *mut Wrconfig { 
        (self.0 + 0x28) as *mut Wrconfig
    }

    #[doc="Get the *const pointer for the WRCONFIG register."]
    #[inline] pub fn wrconfig_ptr(&self) -> *const Wrconfig { 
           self.wrconfig_mut()
    }

    #[doc="Write the WRCONFIG register."]
    #[inline] pub fn set_wrconfig<F: FnOnce(Wrconfig) -> Wrconfig>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.wrconfig_mut(), f(Wrconfig(0)));
        }
        self
    }

}

#[doc="Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Input Sampling Mode"]
    #[inline] pub fn sampling(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SAMPLING != 0"]
    #[inline] pub fn test_sampling(&self) -> bool {
        self.sampling() != 0
    }

    #[doc="Sets the SAMPLING field."]
    #[inline] pub fn set_sampling<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

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

#[doc="Data Direction"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dir(pub u32);
impl Dir {
    #[doc="Port Data Direction"]
    #[inline] pub fn dir<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.dir(index) != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dir {
    #[inline]
    fn from(other: u32) -> Self {
         Dir(other)
    }
}

impl ::core::fmt::Display for Dir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir(0) != 0 { try!(write!(f, " dir[0]"))}
        if self.dir(1) != 0 { try!(write!(f, " dir[1]"))}
        if self.dir(2) != 0 { try!(write!(f, " dir[2]"))}
        if self.dir(3) != 0 { try!(write!(f, " dir[3]"))}
        if self.dir(4) != 0 { try!(write!(f, " dir[4]"))}
        if self.dir(5) != 0 { try!(write!(f, " dir[5]"))}
        if self.dir(6) != 0 { try!(write!(f, " dir[6]"))}
        if self.dir(7) != 0 { try!(write!(f, " dir[7]"))}
        if self.dir(8) != 0 { try!(write!(f, " dir[8]"))}
        if self.dir(9) != 0 { try!(write!(f, " dir[9]"))}
        if self.dir(10) != 0 { try!(write!(f, " dir[10]"))}
        if self.dir(11) != 0 { try!(write!(f, " dir[11]"))}
        if self.dir(12) != 0 { try!(write!(f, " dir[12]"))}
        if self.dir(13) != 0 { try!(write!(f, " dir[13]"))}
        if self.dir(14) != 0 { try!(write!(f, " dir[14]"))}
        if self.dir(15) != 0 { try!(write!(f, " dir[15]"))}
        if self.dir(16) != 0 { try!(write!(f, " dir[16]"))}
        if self.dir(17) != 0 { try!(write!(f, " dir[17]"))}
        if self.dir(18) != 0 { try!(write!(f, " dir[18]"))}
        if self.dir(19) != 0 { try!(write!(f, " dir[19]"))}
        if self.dir(20) != 0 { try!(write!(f, " dir[20]"))}
        if self.dir(21) != 0 { try!(write!(f, " dir[21]"))}
        if self.dir(22) != 0 { try!(write!(f, " dir[22]"))}
        if self.dir(23) != 0 { try!(write!(f, " dir[23]"))}
        if self.dir(24) != 0 { try!(write!(f, " dir[24]"))}
        if self.dir(25) != 0 { try!(write!(f, " dir[25]"))}
        if self.dir(26) != 0 { try!(write!(f, " dir[26]"))}
        if self.dir(27) != 0 { try!(write!(f, " dir[27]"))}
        if self.dir(28) != 0 { try!(write!(f, " dir[28]"))}
        if self.dir(29) != 0 { try!(write!(f, " dir[29]"))}
        if self.dir(30) != 0 { try!(write!(f, " dir[30]"))}
        if self.dir(31) != 0 { try!(write!(f, " dir[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Direction Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dirclr(pub u32);
impl Dirclr {
    #[doc="Port Data Direction Clear"]
    #[inline] pub fn dirclr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIRCLR != 0"]
    #[inline] pub fn test_dirclr<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.dirclr(index) != 0
    }

    #[doc="Sets the DIRCLR field."]
    #[inline] pub fn set_dirclr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dirclr {
    #[inline]
    fn from(other: u32) -> Self {
         Dirclr(other)
    }
}

impl ::core::fmt::Display for Dirclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dirclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dirclr(0) != 0 { try!(write!(f, " dirclr[0]"))}
        if self.dirclr(1) != 0 { try!(write!(f, " dirclr[1]"))}
        if self.dirclr(2) != 0 { try!(write!(f, " dirclr[2]"))}
        if self.dirclr(3) != 0 { try!(write!(f, " dirclr[3]"))}
        if self.dirclr(4) != 0 { try!(write!(f, " dirclr[4]"))}
        if self.dirclr(5) != 0 { try!(write!(f, " dirclr[5]"))}
        if self.dirclr(6) != 0 { try!(write!(f, " dirclr[6]"))}
        if self.dirclr(7) != 0 { try!(write!(f, " dirclr[7]"))}
        if self.dirclr(8) != 0 { try!(write!(f, " dirclr[8]"))}
        if self.dirclr(9) != 0 { try!(write!(f, " dirclr[9]"))}
        if self.dirclr(10) != 0 { try!(write!(f, " dirclr[10]"))}
        if self.dirclr(11) != 0 { try!(write!(f, " dirclr[11]"))}
        if self.dirclr(12) != 0 { try!(write!(f, " dirclr[12]"))}
        if self.dirclr(13) != 0 { try!(write!(f, " dirclr[13]"))}
        if self.dirclr(14) != 0 { try!(write!(f, " dirclr[14]"))}
        if self.dirclr(15) != 0 { try!(write!(f, " dirclr[15]"))}
        if self.dirclr(16) != 0 { try!(write!(f, " dirclr[16]"))}
        if self.dirclr(17) != 0 { try!(write!(f, " dirclr[17]"))}
        if self.dirclr(18) != 0 { try!(write!(f, " dirclr[18]"))}
        if self.dirclr(19) != 0 { try!(write!(f, " dirclr[19]"))}
        if self.dirclr(20) != 0 { try!(write!(f, " dirclr[20]"))}
        if self.dirclr(21) != 0 { try!(write!(f, " dirclr[21]"))}
        if self.dirclr(22) != 0 { try!(write!(f, " dirclr[22]"))}
        if self.dirclr(23) != 0 { try!(write!(f, " dirclr[23]"))}
        if self.dirclr(24) != 0 { try!(write!(f, " dirclr[24]"))}
        if self.dirclr(25) != 0 { try!(write!(f, " dirclr[25]"))}
        if self.dirclr(26) != 0 { try!(write!(f, " dirclr[26]"))}
        if self.dirclr(27) != 0 { try!(write!(f, " dirclr[27]"))}
        if self.dirclr(28) != 0 { try!(write!(f, " dirclr[28]"))}
        if self.dirclr(29) != 0 { try!(write!(f, " dirclr[29]"))}
        if self.dirclr(30) != 0 { try!(write!(f, " dirclr[30]"))}
        if self.dirclr(31) != 0 { try!(write!(f, " dirclr[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Direction Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dirset(pub u32);
impl Dirset {
    #[doc="Port Data Direction Set"]
    #[inline] pub fn dirset<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIRSET != 0"]
    #[inline] pub fn test_dirset<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.dirset(index) != 0
    }

    #[doc="Sets the DIRSET field."]
    #[inline] pub fn set_dirset<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dirset {
    #[inline]
    fn from(other: u32) -> Self {
         Dirset(other)
    }
}

impl ::core::fmt::Display for Dirset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dirset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dirset(0) != 0 { try!(write!(f, " dirset[0]"))}
        if self.dirset(1) != 0 { try!(write!(f, " dirset[1]"))}
        if self.dirset(2) != 0 { try!(write!(f, " dirset[2]"))}
        if self.dirset(3) != 0 { try!(write!(f, " dirset[3]"))}
        if self.dirset(4) != 0 { try!(write!(f, " dirset[4]"))}
        if self.dirset(5) != 0 { try!(write!(f, " dirset[5]"))}
        if self.dirset(6) != 0 { try!(write!(f, " dirset[6]"))}
        if self.dirset(7) != 0 { try!(write!(f, " dirset[7]"))}
        if self.dirset(8) != 0 { try!(write!(f, " dirset[8]"))}
        if self.dirset(9) != 0 { try!(write!(f, " dirset[9]"))}
        if self.dirset(10) != 0 { try!(write!(f, " dirset[10]"))}
        if self.dirset(11) != 0 { try!(write!(f, " dirset[11]"))}
        if self.dirset(12) != 0 { try!(write!(f, " dirset[12]"))}
        if self.dirset(13) != 0 { try!(write!(f, " dirset[13]"))}
        if self.dirset(14) != 0 { try!(write!(f, " dirset[14]"))}
        if self.dirset(15) != 0 { try!(write!(f, " dirset[15]"))}
        if self.dirset(16) != 0 { try!(write!(f, " dirset[16]"))}
        if self.dirset(17) != 0 { try!(write!(f, " dirset[17]"))}
        if self.dirset(18) != 0 { try!(write!(f, " dirset[18]"))}
        if self.dirset(19) != 0 { try!(write!(f, " dirset[19]"))}
        if self.dirset(20) != 0 { try!(write!(f, " dirset[20]"))}
        if self.dirset(21) != 0 { try!(write!(f, " dirset[21]"))}
        if self.dirset(22) != 0 { try!(write!(f, " dirset[22]"))}
        if self.dirset(23) != 0 { try!(write!(f, " dirset[23]"))}
        if self.dirset(24) != 0 { try!(write!(f, " dirset[24]"))}
        if self.dirset(25) != 0 { try!(write!(f, " dirset[25]"))}
        if self.dirset(26) != 0 { try!(write!(f, " dirset[26]"))}
        if self.dirset(27) != 0 { try!(write!(f, " dirset[27]"))}
        if self.dirset(28) != 0 { try!(write!(f, " dirset[28]"))}
        if self.dirset(29) != 0 { try!(write!(f, " dirset[29]"))}
        if self.dirset(30) != 0 { try!(write!(f, " dirset[30]"))}
        if self.dirset(31) != 0 { try!(write!(f, " dirset[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Direction Toggle"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dirtgl(pub u32);
impl Dirtgl {
    #[doc="Port Data Direction Toggle"]
    #[inline] pub fn dirtgl<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIRTGL != 0"]
    #[inline] pub fn test_dirtgl<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.dirtgl(index) != 0
    }

    #[doc="Sets the DIRTGL field."]
    #[inline] pub fn set_dirtgl<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dirtgl {
    #[inline]
    fn from(other: u32) -> Self {
         Dirtgl(other)
    }
}

impl ::core::fmt::Display for Dirtgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dirtgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dirtgl(0) != 0 { try!(write!(f, " dirtgl[0]"))}
        if self.dirtgl(1) != 0 { try!(write!(f, " dirtgl[1]"))}
        if self.dirtgl(2) != 0 { try!(write!(f, " dirtgl[2]"))}
        if self.dirtgl(3) != 0 { try!(write!(f, " dirtgl[3]"))}
        if self.dirtgl(4) != 0 { try!(write!(f, " dirtgl[4]"))}
        if self.dirtgl(5) != 0 { try!(write!(f, " dirtgl[5]"))}
        if self.dirtgl(6) != 0 { try!(write!(f, " dirtgl[6]"))}
        if self.dirtgl(7) != 0 { try!(write!(f, " dirtgl[7]"))}
        if self.dirtgl(8) != 0 { try!(write!(f, " dirtgl[8]"))}
        if self.dirtgl(9) != 0 { try!(write!(f, " dirtgl[9]"))}
        if self.dirtgl(10) != 0 { try!(write!(f, " dirtgl[10]"))}
        if self.dirtgl(11) != 0 { try!(write!(f, " dirtgl[11]"))}
        if self.dirtgl(12) != 0 { try!(write!(f, " dirtgl[12]"))}
        if self.dirtgl(13) != 0 { try!(write!(f, " dirtgl[13]"))}
        if self.dirtgl(14) != 0 { try!(write!(f, " dirtgl[14]"))}
        if self.dirtgl(15) != 0 { try!(write!(f, " dirtgl[15]"))}
        if self.dirtgl(16) != 0 { try!(write!(f, " dirtgl[16]"))}
        if self.dirtgl(17) != 0 { try!(write!(f, " dirtgl[17]"))}
        if self.dirtgl(18) != 0 { try!(write!(f, " dirtgl[18]"))}
        if self.dirtgl(19) != 0 { try!(write!(f, " dirtgl[19]"))}
        if self.dirtgl(20) != 0 { try!(write!(f, " dirtgl[20]"))}
        if self.dirtgl(21) != 0 { try!(write!(f, " dirtgl[21]"))}
        if self.dirtgl(22) != 0 { try!(write!(f, " dirtgl[22]"))}
        if self.dirtgl(23) != 0 { try!(write!(f, " dirtgl[23]"))}
        if self.dirtgl(24) != 0 { try!(write!(f, " dirtgl[24]"))}
        if self.dirtgl(25) != 0 { try!(write!(f, " dirtgl[25]"))}
        if self.dirtgl(26) != 0 { try!(write!(f, " dirtgl[26]"))}
        if self.dirtgl(27) != 0 { try!(write!(f, " dirtgl[27]"))}
        if self.dirtgl(28) != 0 { try!(write!(f, " dirtgl[28]"))}
        if self.dirtgl(29) != 0 { try!(write!(f, " dirtgl[29]"))}
        if self.dirtgl(30) != 0 { try!(write!(f, " dirtgl[30]"))}
        if self.dirtgl(31) != 0 { try!(write!(f, " dirtgl[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Input Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct In(pub u32);
impl In {
    #[doc="Port Data Input Value"]
    #[inline] pub fn _in<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IN != 0"]
    #[inline] pub fn test_in<I: Into<bits::R32>>(&self, index: I) -> bool{
        self._in(index) != 0
    }

    #[doc="Sets the IN field."]
    #[inline] pub fn set_in<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for In {
    #[inline]
    fn from(other: u32) -> Self {
         In(other)
    }
}

impl ::core::fmt::Display for In {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for In {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._in(0) != 0 { try!(write!(f, " _in[0]"))}
        if self._in(1) != 0 { try!(write!(f, " _in[1]"))}
        if self._in(2) != 0 { try!(write!(f, " _in[2]"))}
        if self._in(3) != 0 { try!(write!(f, " _in[3]"))}
        if self._in(4) != 0 { try!(write!(f, " _in[4]"))}
        if self._in(5) != 0 { try!(write!(f, " _in[5]"))}
        if self._in(6) != 0 { try!(write!(f, " _in[6]"))}
        if self._in(7) != 0 { try!(write!(f, " _in[7]"))}
        if self._in(8) != 0 { try!(write!(f, " _in[8]"))}
        if self._in(9) != 0 { try!(write!(f, " _in[9]"))}
        if self._in(10) != 0 { try!(write!(f, " _in[10]"))}
        if self._in(11) != 0 { try!(write!(f, " _in[11]"))}
        if self._in(12) != 0 { try!(write!(f, " _in[12]"))}
        if self._in(13) != 0 { try!(write!(f, " _in[13]"))}
        if self._in(14) != 0 { try!(write!(f, " _in[14]"))}
        if self._in(15) != 0 { try!(write!(f, " _in[15]"))}
        if self._in(16) != 0 { try!(write!(f, " _in[16]"))}
        if self._in(17) != 0 { try!(write!(f, " _in[17]"))}
        if self._in(18) != 0 { try!(write!(f, " _in[18]"))}
        if self._in(19) != 0 { try!(write!(f, " _in[19]"))}
        if self._in(20) != 0 { try!(write!(f, " _in[20]"))}
        if self._in(21) != 0 { try!(write!(f, " _in[21]"))}
        if self._in(22) != 0 { try!(write!(f, " _in[22]"))}
        if self._in(23) != 0 { try!(write!(f, " _in[23]"))}
        if self._in(24) != 0 { try!(write!(f, " _in[24]"))}
        if self._in(25) != 0 { try!(write!(f, " _in[25]"))}
        if self._in(26) != 0 { try!(write!(f, " _in[26]"))}
        if self._in(27) != 0 { try!(write!(f, " _in[27]"))}
        if self._in(28) != 0 { try!(write!(f, " _in[28]"))}
        if self._in(29) != 0 { try!(write!(f, " _in[29]"))}
        if self._in(30) != 0 { try!(write!(f, " _in[30]"))}
        if self._in(31) != 0 { try!(write!(f, " _in[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Output Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Out(pub u32);
impl Out {
    #[doc="Port Data Output Value"]
    #[inline] pub fn out<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUT != 0"]
    #[inline] pub fn test_out<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.out(index) != 0
    }

    #[doc="Sets the OUT field."]
    #[inline] pub fn set_out<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Out {
    #[inline]
    fn from(other: u32) -> Self {
         Out(other)
    }
}

impl ::core::fmt::Display for Out {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Out {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.out(0) != 0 { try!(write!(f, " out[0]"))}
        if self.out(1) != 0 { try!(write!(f, " out[1]"))}
        if self.out(2) != 0 { try!(write!(f, " out[2]"))}
        if self.out(3) != 0 { try!(write!(f, " out[3]"))}
        if self.out(4) != 0 { try!(write!(f, " out[4]"))}
        if self.out(5) != 0 { try!(write!(f, " out[5]"))}
        if self.out(6) != 0 { try!(write!(f, " out[6]"))}
        if self.out(7) != 0 { try!(write!(f, " out[7]"))}
        if self.out(8) != 0 { try!(write!(f, " out[8]"))}
        if self.out(9) != 0 { try!(write!(f, " out[9]"))}
        if self.out(10) != 0 { try!(write!(f, " out[10]"))}
        if self.out(11) != 0 { try!(write!(f, " out[11]"))}
        if self.out(12) != 0 { try!(write!(f, " out[12]"))}
        if self.out(13) != 0 { try!(write!(f, " out[13]"))}
        if self.out(14) != 0 { try!(write!(f, " out[14]"))}
        if self.out(15) != 0 { try!(write!(f, " out[15]"))}
        if self.out(16) != 0 { try!(write!(f, " out[16]"))}
        if self.out(17) != 0 { try!(write!(f, " out[17]"))}
        if self.out(18) != 0 { try!(write!(f, " out[18]"))}
        if self.out(19) != 0 { try!(write!(f, " out[19]"))}
        if self.out(20) != 0 { try!(write!(f, " out[20]"))}
        if self.out(21) != 0 { try!(write!(f, " out[21]"))}
        if self.out(22) != 0 { try!(write!(f, " out[22]"))}
        if self.out(23) != 0 { try!(write!(f, " out[23]"))}
        if self.out(24) != 0 { try!(write!(f, " out[24]"))}
        if self.out(25) != 0 { try!(write!(f, " out[25]"))}
        if self.out(26) != 0 { try!(write!(f, " out[26]"))}
        if self.out(27) != 0 { try!(write!(f, " out[27]"))}
        if self.out(28) != 0 { try!(write!(f, " out[28]"))}
        if self.out(29) != 0 { try!(write!(f, " out[29]"))}
        if self.out(30) != 0 { try!(write!(f, " out[30]"))}
        if self.out(31) != 0 { try!(write!(f, " out[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Output Value Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Outclr(pub u32);
impl Outclr {
    #[doc="Port Data Output Value Clear"]
    #[inline] pub fn outclr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUTCLR != 0"]
    #[inline] pub fn test_outclr<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.outclr(index) != 0
    }

    #[doc="Sets the OUTCLR field."]
    #[inline] pub fn set_outclr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Outclr {
    #[inline]
    fn from(other: u32) -> Self {
         Outclr(other)
    }
}

impl ::core::fmt::Display for Outclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Outclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.outclr(0) != 0 { try!(write!(f, " outclr[0]"))}
        if self.outclr(1) != 0 { try!(write!(f, " outclr[1]"))}
        if self.outclr(2) != 0 { try!(write!(f, " outclr[2]"))}
        if self.outclr(3) != 0 { try!(write!(f, " outclr[3]"))}
        if self.outclr(4) != 0 { try!(write!(f, " outclr[4]"))}
        if self.outclr(5) != 0 { try!(write!(f, " outclr[5]"))}
        if self.outclr(6) != 0 { try!(write!(f, " outclr[6]"))}
        if self.outclr(7) != 0 { try!(write!(f, " outclr[7]"))}
        if self.outclr(8) != 0 { try!(write!(f, " outclr[8]"))}
        if self.outclr(9) != 0 { try!(write!(f, " outclr[9]"))}
        if self.outclr(10) != 0 { try!(write!(f, " outclr[10]"))}
        if self.outclr(11) != 0 { try!(write!(f, " outclr[11]"))}
        if self.outclr(12) != 0 { try!(write!(f, " outclr[12]"))}
        if self.outclr(13) != 0 { try!(write!(f, " outclr[13]"))}
        if self.outclr(14) != 0 { try!(write!(f, " outclr[14]"))}
        if self.outclr(15) != 0 { try!(write!(f, " outclr[15]"))}
        if self.outclr(16) != 0 { try!(write!(f, " outclr[16]"))}
        if self.outclr(17) != 0 { try!(write!(f, " outclr[17]"))}
        if self.outclr(18) != 0 { try!(write!(f, " outclr[18]"))}
        if self.outclr(19) != 0 { try!(write!(f, " outclr[19]"))}
        if self.outclr(20) != 0 { try!(write!(f, " outclr[20]"))}
        if self.outclr(21) != 0 { try!(write!(f, " outclr[21]"))}
        if self.outclr(22) != 0 { try!(write!(f, " outclr[22]"))}
        if self.outclr(23) != 0 { try!(write!(f, " outclr[23]"))}
        if self.outclr(24) != 0 { try!(write!(f, " outclr[24]"))}
        if self.outclr(25) != 0 { try!(write!(f, " outclr[25]"))}
        if self.outclr(26) != 0 { try!(write!(f, " outclr[26]"))}
        if self.outclr(27) != 0 { try!(write!(f, " outclr[27]"))}
        if self.outclr(28) != 0 { try!(write!(f, " outclr[28]"))}
        if self.outclr(29) != 0 { try!(write!(f, " outclr[29]"))}
        if self.outclr(30) != 0 { try!(write!(f, " outclr[30]"))}
        if self.outclr(31) != 0 { try!(write!(f, " outclr[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Output Value Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Outset(pub u32);
impl Outset {
    #[doc="Port Data Output Value Set"]
    #[inline] pub fn outset<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUTSET != 0"]
    #[inline] pub fn test_outset<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.outset(index) != 0
    }

    #[doc="Sets the OUTSET field."]
    #[inline] pub fn set_outset<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Outset {
    #[inline]
    fn from(other: u32) -> Self {
         Outset(other)
    }
}

impl ::core::fmt::Display for Outset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Outset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.outset(0) != 0 { try!(write!(f, " outset[0]"))}
        if self.outset(1) != 0 { try!(write!(f, " outset[1]"))}
        if self.outset(2) != 0 { try!(write!(f, " outset[2]"))}
        if self.outset(3) != 0 { try!(write!(f, " outset[3]"))}
        if self.outset(4) != 0 { try!(write!(f, " outset[4]"))}
        if self.outset(5) != 0 { try!(write!(f, " outset[5]"))}
        if self.outset(6) != 0 { try!(write!(f, " outset[6]"))}
        if self.outset(7) != 0 { try!(write!(f, " outset[7]"))}
        if self.outset(8) != 0 { try!(write!(f, " outset[8]"))}
        if self.outset(9) != 0 { try!(write!(f, " outset[9]"))}
        if self.outset(10) != 0 { try!(write!(f, " outset[10]"))}
        if self.outset(11) != 0 { try!(write!(f, " outset[11]"))}
        if self.outset(12) != 0 { try!(write!(f, " outset[12]"))}
        if self.outset(13) != 0 { try!(write!(f, " outset[13]"))}
        if self.outset(14) != 0 { try!(write!(f, " outset[14]"))}
        if self.outset(15) != 0 { try!(write!(f, " outset[15]"))}
        if self.outset(16) != 0 { try!(write!(f, " outset[16]"))}
        if self.outset(17) != 0 { try!(write!(f, " outset[17]"))}
        if self.outset(18) != 0 { try!(write!(f, " outset[18]"))}
        if self.outset(19) != 0 { try!(write!(f, " outset[19]"))}
        if self.outset(20) != 0 { try!(write!(f, " outset[20]"))}
        if self.outset(21) != 0 { try!(write!(f, " outset[21]"))}
        if self.outset(22) != 0 { try!(write!(f, " outset[22]"))}
        if self.outset(23) != 0 { try!(write!(f, " outset[23]"))}
        if self.outset(24) != 0 { try!(write!(f, " outset[24]"))}
        if self.outset(25) != 0 { try!(write!(f, " outset[25]"))}
        if self.outset(26) != 0 { try!(write!(f, " outset[26]"))}
        if self.outset(27) != 0 { try!(write!(f, " outset[27]"))}
        if self.outset(28) != 0 { try!(write!(f, " outset[28]"))}
        if self.outset(29) != 0 { try!(write!(f, " outset[29]"))}
        if self.outset(30) != 0 { try!(write!(f, " outset[30]"))}
        if self.outset(31) != 0 { try!(write!(f, " outset[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Output Value Toggle"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Outtgl(pub u32);
impl Outtgl {
    #[doc="Port Data Output Value Toggle"]
    #[inline] pub fn outtgl<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUTTGL != 0"]
    #[inline] pub fn test_outtgl<I: Into<bits::R32>>(&self, index: I) -> bool{
        self.outtgl(index) != 0
    }

    #[doc="Sets the OUTTGL field."]
    #[inline] pub fn set_outtgl<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Outtgl {
    #[inline]
    fn from(other: u32) -> Self {
         Outtgl(other)
    }
}

impl ::core::fmt::Display for Outtgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Outtgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.outtgl(0) != 0 { try!(write!(f, " outtgl[0]"))}
        if self.outtgl(1) != 0 { try!(write!(f, " outtgl[1]"))}
        if self.outtgl(2) != 0 { try!(write!(f, " outtgl[2]"))}
        if self.outtgl(3) != 0 { try!(write!(f, " outtgl[3]"))}
        if self.outtgl(4) != 0 { try!(write!(f, " outtgl[4]"))}
        if self.outtgl(5) != 0 { try!(write!(f, " outtgl[5]"))}
        if self.outtgl(6) != 0 { try!(write!(f, " outtgl[6]"))}
        if self.outtgl(7) != 0 { try!(write!(f, " outtgl[7]"))}
        if self.outtgl(8) != 0 { try!(write!(f, " outtgl[8]"))}
        if self.outtgl(9) != 0 { try!(write!(f, " outtgl[9]"))}
        if self.outtgl(10) != 0 { try!(write!(f, " outtgl[10]"))}
        if self.outtgl(11) != 0 { try!(write!(f, " outtgl[11]"))}
        if self.outtgl(12) != 0 { try!(write!(f, " outtgl[12]"))}
        if self.outtgl(13) != 0 { try!(write!(f, " outtgl[13]"))}
        if self.outtgl(14) != 0 { try!(write!(f, " outtgl[14]"))}
        if self.outtgl(15) != 0 { try!(write!(f, " outtgl[15]"))}
        if self.outtgl(16) != 0 { try!(write!(f, " outtgl[16]"))}
        if self.outtgl(17) != 0 { try!(write!(f, " outtgl[17]"))}
        if self.outtgl(18) != 0 { try!(write!(f, " outtgl[18]"))}
        if self.outtgl(19) != 0 { try!(write!(f, " outtgl[19]"))}
        if self.outtgl(20) != 0 { try!(write!(f, " outtgl[20]"))}
        if self.outtgl(21) != 0 { try!(write!(f, " outtgl[21]"))}
        if self.outtgl(22) != 0 { try!(write!(f, " outtgl[22]"))}
        if self.outtgl(23) != 0 { try!(write!(f, " outtgl[23]"))}
        if self.outtgl(24) != 0 { try!(write!(f, " outtgl[24]"))}
        if self.outtgl(25) != 0 { try!(write!(f, " outtgl[25]"))}
        if self.outtgl(26) != 0 { try!(write!(f, " outtgl[26]"))}
        if self.outtgl(27) != 0 { try!(write!(f, " outtgl[27]"))}
        if self.outtgl(28) != 0 { try!(write!(f, " outtgl[28]"))}
        if self.outtgl(29) != 0 { try!(write!(f, " outtgl[29]"))}
        if self.outtgl(30) != 0 { try!(write!(f, " outtgl[30]"))}
        if self.outtgl(31) != 0 { try!(write!(f, " outtgl[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pin Configuration n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pincfg(pub u8);
impl Pincfg {
    #[doc="Peripheral Multiplexer Enable"]
    #[inline] pub fn pmuxen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PMUXEN != 0"]
    #[inline] pub fn test_pmuxen(&self) -> bool {
        self.pmuxen() != 0
    }

    #[doc="Sets the PMUXEN field."]
    #[inline] pub fn set_pmuxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Input Enable"]
    #[inline] pub fn inen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INEN != 0"]
    #[inline] pub fn test_inen(&self) -> bool {
        self.inen() != 0
    }

    #[doc="Sets the INEN field."]
    #[inline] pub fn set_inen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pull Enable"]
    #[inline] pub fn pullen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PULLEN != 0"]
    #[inline] pub fn test_pullen(&self) -> bool {
        self.pullen() != 0
    }

    #[doc="Sets the PULLEN field."]
    #[inline] pub fn set_pullen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Output Driver Strength Selection"]
    #[inline] pub fn drvstr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DRVSTR != 0"]
    #[inline] pub fn test_drvstr(&self) -> bool {
        self.drvstr() != 0
    }

    #[doc="Sets the DRVSTR field."]
    #[inline] pub fn set_drvstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Pincfg {
    #[inline]
    fn from(other: u8) -> Self {
         Pincfg(other)
    }
}

impl ::core::fmt::Display for Pincfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pincfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pmuxen() != 0 { try!(write!(f, " pmuxen"))}
        if self.inen() != 0 { try!(write!(f, " inen"))}
        if self.pullen() != 0 { try!(write!(f, " pullen"))}
        if self.drvstr() != 0 { try!(write!(f, " drvstr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Multiplexing n - Group 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmux(pub u8);
impl Pmux {
    #[doc="Peripheral Multiplexing Even"]
    #[inline] pub fn pmux<I: Into<bits::R2>>(&self, index: I) -> bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PMUX != 0"]
    #[inline] pub fn test_pmux<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.pmux(index) != 0
    }

    #[doc="Sets the PMUX field."]
    #[inline] pub fn set_pmux<I: Into<bits::R2>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u8> for Pmux {
    #[inline]
    fn from(other: u8) -> Self {
         Pmux(other)
    }
}

impl ::core::fmt::Display for Pmux {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmux {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pmux(0) != 0 { try!(write!(f, " pmux[0]=0x{:x}", self.pmux(0)))}
        if self.pmux(1) != 0 { try!(write!(f, " pmux[1]=0x{:x}", self.pmux(1)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Write Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrconfig(pub u32);
impl Wrconfig {
    #[doc="Pin Mask for Multiple Pin Configuration"]
    #[inline] pub fn pinmask(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PINMASK != 0"]
    #[inline] pub fn test_pinmask(&self) -> bool {
        self.pinmask() != 0
    }

    #[doc="Sets the PINMASK field."]
    #[inline] pub fn set_pinmask<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Peripheral Multiplexer Enable"]
    #[inline] pub fn pmuxen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PMUXEN != 0"]
    #[inline] pub fn test_pmuxen(&self) -> bool {
        self.pmuxen() != 0
    }

    #[doc="Sets the PMUXEN field."]
    #[inline] pub fn set_pmuxen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Input Enable"]
    #[inline] pub fn inen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if INEN != 0"]
    #[inline] pub fn test_inen(&self) -> bool {
        self.inen() != 0
    }

    #[doc="Sets the INEN field."]
    #[inline] pub fn set_inen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pull Enable"]
    #[inline] pub fn pullen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PULLEN != 0"]
    #[inline] pub fn test_pullen(&self) -> bool {
        self.pullen() != 0
    }

    #[doc="Sets the PULLEN field."]
    #[inline] pub fn set_pullen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Output Driver Strength Selection"]
    #[inline] pub fn drvstr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DRVSTR != 0"]
    #[inline] pub fn test_drvstr(&self) -> bool {
        self.drvstr() != 0
    }

    #[doc="Sets the DRVSTR field."]
    #[inline] pub fn set_drvstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Peripheral Multiplexing"]
    #[inline] pub fn pmux(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if PMUX != 0"]
    #[inline] pub fn test_pmux(&self) -> bool {
        self.pmux() != 0
    }

    #[doc="Sets the PMUX field."]
    #[inline] pub fn set_pmux<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Write PMUX"]
    #[inline] pub fn wrpmux(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if WRPMUX != 0"]
    #[inline] pub fn test_wrpmux(&self) -> bool {
        self.wrpmux() != 0
    }

    #[doc="Sets the WRPMUX field."]
    #[inline] pub fn set_wrpmux<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Write PINCFG"]
    #[inline] pub fn wrpincfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if WRPINCFG != 0"]
    #[inline] pub fn test_wrpincfg(&self) -> bool {
        self.wrpincfg() != 0
    }

    #[doc="Sets the WRPINCFG field."]
    #[inline] pub fn set_wrpincfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Half-Word Select"]
    #[inline] pub fn hwsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if HWSEL != 0"]
    #[inline] pub fn test_hwsel(&self) -> bool {
        self.hwsel() != 0
    }

    #[doc="Sets the HWSEL field."]
    #[inline] pub fn set_hwsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Wrconfig {
    #[inline]
    fn from(other: u32) -> Self {
         Wrconfig(other)
    }
}

impl ::core::fmt::Display for Wrconfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrconfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pinmask() != 0 { try!(write!(f, " pinmask=0x{:x}", self.pinmask()))}
        if self.pmuxen() != 0 { try!(write!(f, " pmuxen"))}
        if self.inen() != 0 { try!(write!(f, " inen"))}
        if self.pullen() != 0 { try!(write!(f, " pullen"))}
        if self.drvstr() != 0 { try!(write!(f, " drvstr"))}
        if self.pmux() != 0 { try!(write!(f, " pmux=0x{:x}", self.pmux()))}
        if self.wrpmux() != 0 { try!(write!(f, " wrpmux"))}
        if self.wrpincfg() != 0 { try!(write!(f, " wrpincfg"))}
        if self.hwsel() != 0 { try!(write!(f, " hwsel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

