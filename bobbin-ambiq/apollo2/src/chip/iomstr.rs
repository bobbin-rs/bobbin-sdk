#[allow(unused_imports)] use bobbin_common::*;

periph!( IOMSTR0, Iomstr0, _IOMSTR0, IomstrPeriph, 0x50004000);
periph!( IOMSTR1, Iomstr1, _IOMSTR1, IomstrPeriph, 0x50005000);
periph!( IOMSTR2, Iomstr2, _IOMSTR2, IomstrPeriph, 0x50006000);
periph!( IOMSTR3, Iomstr3, _IOMSTR3, IomstrPeriph, 0x50007000);
periph!( IOMSTR4, Iomstr4, _IOMSTR4, IomstrPeriph, 0x50008000);
periph!( IOMSTR5, Iomstr5, _IOMSTR5, IomstrPeriph, 0x50009000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="IOMSTR Peripheral"]
pub struct IomstrPeriph(pub usize); 








impl IomstrPeriph {
    #[doc="Get the *mut pointer for the FIFO register."]
    #[inline] pub fn fifo_mut(&self) -> *mut Fifo { 
        (self.0 + 0x0) as *mut Fifo
    }

    #[doc="Get the *const pointer for the FIFO register."]
    #[inline] pub fn fifo_ptr(&self) -> *const Fifo { 
           self.fifo_mut()
    }

    #[doc="Read the FIFO register."]
    #[inline] pub fn fifo(&self) -> Fifo { 
        unsafe {
            read_volatile(self.fifo_ptr())
        }
    }

    #[doc="Write the FIFO register."]
    #[inline] pub fn set_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifo_mut(), f(Fifo(0)));
        }
        self
    }

    #[doc="Modify the FIFO register."]
    #[inline] pub fn with_fifo<F: FnOnce(Fifo) -> Fifo>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifo_mut(), f(self.fifo()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFOPTR register."]
    #[inline] pub fn fifoptr_mut(&self) -> *mut Fifoptr { 
        (self.0 + 0x100) as *mut Fifoptr
    }

    #[doc="Get the *const pointer for the FIFOPTR register."]
    #[inline] pub fn fifoptr_ptr(&self) -> *const Fifoptr { 
           self.fifoptr_mut()
    }

    #[doc="Read the FIFOPTR register."]
    #[inline] pub fn fifoptr(&self) -> Fifoptr { 
        unsafe {
            read_volatile(self.fifoptr_ptr())
        }
    }

    #[doc="Write the FIFOPTR register."]
    #[inline] pub fn set_fifoptr<F: FnOnce(Fifoptr) -> Fifoptr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifoptr_mut(), f(Fifoptr(0)));
        }
        self
    }

    #[doc="Modify the FIFOPTR register."]
    #[inline] pub fn with_fifoptr<F: FnOnce(Fifoptr) -> Fifoptr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifoptr_mut(), f(self.fifoptr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TLNGTH register."]
    #[inline] pub fn tlngth_mut(&self) -> *mut Tlngth { 
        (self.0 + 0x104) as *mut Tlngth
    }

    #[doc="Get the *const pointer for the TLNGTH register."]
    #[inline] pub fn tlngth_ptr(&self) -> *const Tlngth { 
           self.tlngth_mut()
    }

    #[doc="Read the TLNGTH register."]
    #[inline] pub fn tlngth(&self) -> Tlngth { 
        unsafe {
            read_volatile(self.tlngth_ptr())
        }
    }

    #[doc="Write the TLNGTH register."]
    #[inline] pub fn set_tlngth<F: FnOnce(Tlngth) -> Tlngth>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tlngth_mut(), f(Tlngth(0)));
        }
        self
    }

    #[doc="Modify the TLNGTH register."]
    #[inline] pub fn with_tlngth<F: FnOnce(Tlngth) -> Tlngth>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tlngth_mut(), f(self.tlngth()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFOTHR register."]
    #[inline] pub fn fifothr_mut(&self) -> *mut Fifothr { 
        (self.0 + 0x108) as *mut Fifothr
    }

    #[doc="Get the *const pointer for the FIFOTHR register."]
    #[inline] pub fn fifothr_ptr(&self) -> *const Fifothr { 
           self.fifothr_mut()
    }

    #[doc="Read the FIFOTHR register."]
    #[inline] pub fn fifothr(&self) -> Fifothr { 
        unsafe {
            read_volatile(self.fifothr_ptr())
        }
    }

    #[doc="Write the FIFOTHR register."]
    #[inline] pub fn set_fifothr<F: FnOnce(Fifothr) -> Fifothr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifothr_mut(), f(Fifothr(0)));
        }
        self
    }

    #[doc="Modify the FIFOTHR register."]
    #[inline] pub fn with_fifothr<F: FnOnce(Fifothr) -> Fifothr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifothr_mut(), f(self.fifothr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLKCFG register."]
    #[inline] pub fn clkcfg_mut(&self) -> *mut Clkcfg { 
        (self.0 + 0x10c) as *mut Clkcfg
    }

    #[doc="Get the *const pointer for the CLKCFG register."]
    #[inline] pub fn clkcfg_ptr(&self) -> *const Clkcfg { 
           self.clkcfg_mut()
    }

    #[doc="Read the CLKCFG register."]
    #[inline] pub fn clkcfg(&self) -> Clkcfg { 
        unsafe {
            read_volatile(self.clkcfg_ptr())
        }
    }

    #[doc="Write the CLKCFG register."]
    #[inline] pub fn set_clkcfg<F: FnOnce(Clkcfg) -> Clkcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkcfg_mut(), f(Clkcfg(0)));
        }
        self
    }

    #[doc="Modify the CLKCFG register."]
    #[inline] pub fn with_clkcfg<F: FnOnce(Clkcfg) -> Clkcfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkcfg_mut(), f(self.clkcfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMD register."]
    #[inline] pub fn cmd_mut(&self) -> *mut Cmd { 
        (self.0 + 0x110) as *mut Cmd
    }

    #[doc="Get the *const pointer for the CMD register."]
    #[inline] pub fn cmd_ptr(&self) -> *const Cmd { 
           self.cmd_mut()
    }

    #[doc="Read the CMD register."]
    #[inline] pub fn cmd(&self) -> Cmd { 
        unsafe {
            read_volatile(self.cmd_ptr())
        }
    }

    #[doc="Write the CMD register."]
    #[inline] pub fn set_cmd<F: FnOnce(Cmd) -> Cmd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmd_mut(), f(Cmd(0)));
        }
        self
    }

    #[doc="Modify the CMD register."]
    #[inline] pub fn with_cmd<F: FnOnce(Cmd) -> Cmd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmd_mut(), f(self.cmd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CMDRPT register."]
    #[inline] pub fn cmdrpt_mut(&self) -> *mut Cmdrpt { 
        (self.0 + 0x114) as *mut Cmdrpt
    }

    #[doc="Get the *const pointer for the CMDRPT register."]
    #[inline] pub fn cmdrpt_ptr(&self) -> *const Cmdrpt { 
           self.cmdrpt_mut()
    }

    #[doc="Read the CMDRPT register."]
    #[inline] pub fn cmdrpt(&self) -> Cmdrpt { 
        unsafe {
            read_volatile(self.cmdrpt_ptr())
        }
    }

    #[doc="Write the CMDRPT register."]
    #[inline] pub fn set_cmdrpt<F: FnOnce(Cmdrpt) -> Cmdrpt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmdrpt_mut(), f(Cmdrpt(0)));
        }
        self
    }

    #[doc="Modify the CMDRPT register."]
    #[inline] pub fn with_cmdrpt<F: FnOnce(Cmdrpt) -> Cmdrpt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cmdrpt_mut(), f(self.cmdrpt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0x118) as *mut Status
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
           self.status_mut()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            read_volatile(self.status_ptr())
        }
    }

    #[doc="Write the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.status_mut(), f(Status(0)));
        }
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.status_mut(), f(self.status()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFG register."]
    #[inline] pub fn cfg_mut(&self) -> *mut Cfg { 
        (self.0 + 0x11c) as *mut Cfg
    }

    #[doc="Get the *const pointer for the CFG register."]
    #[inline] pub fn cfg_ptr(&self) -> *const Cfg { 
           self.cfg_mut()
    }

    #[doc="Read the CFG register."]
    #[inline] pub fn cfg(&self) -> Cfg { 
        unsafe {
            read_volatile(self.cfg_ptr())
        }
    }

    #[doc="Write the CFG register."]
    #[inline] pub fn set_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(), f(Cfg(0)));
        }
        self
    }

    #[doc="Modify the CFG register."]
    #[inline] pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(), f(self.cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTEN register."]
    #[inline] pub fn inten_mut(&self) -> *mut Inten { 
        (self.0 + 0x200) as *mut Inten
    }

    #[doc="Get the *const pointer for the INTEN register."]
    #[inline] pub fn inten_ptr(&self) -> *const Inten { 
           self.inten_mut()
    }

    #[doc="Read the INTEN register."]
    #[inline] pub fn inten(&self) -> Inten { 
        unsafe {
            read_volatile(self.inten_ptr())
        }
    }

    #[doc="Write the INTEN register."]
    #[inline] pub fn set_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(), f(Inten(0)));
        }
        self
    }

    #[doc="Modify the INTEN register."]
    #[inline] pub fn with_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.inten_mut(), f(self.inten()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSTAT register."]
    #[inline] pub fn intstat_mut(&self) -> *mut Intstat { 
        (self.0 + 0x204) as *mut Intstat
    }

    #[doc="Get the *const pointer for the INTSTAT register."]
    #[inline] pub fn intstat_ptr(&self) -> *const Intstat { 
           self.intstat_mut()
    }

    #[doc="Read the INTSTAT register."]
    #[inline] pub fn intstat(&self) -> Intstat { 
        unsafe {
            read_volatile(self.intstat_ptr())
        }
    }

    #[doc="Write the INTSTAT register."]
    #[inline] pub fn set_intstat<F: FnOnce(Intstat) -> Intstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(), f(Intstat(0)));
        }
        self
    }

    #[doc="Modify the INTSTAT register."]
    #[inline] pub fn with_intstat<F: FnOnce(Intstat) -> Intstat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intstat_mut(), f(self.intstat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTCLR register."]
    #[inline] pub fn intclr_mut(&self) -> *mut Intclr { 
        (self.0 + 0x208) as *mut Intclr
    }

    #[doc="Get the *const pointer for the INTCLR register."]
    #[inline] pub fn intclr_ptr(&self) -> *const Intclr { 
           self.intclr_mut()
    }

    #[doc="Read the INTCLR register."]
    #[inline] pub fn intclr(&self) -> Intclr { 
        unsafe {
            read_volatile(self.intclr_ptr())
        }
    }

    #[doc="Write the INTCLR register."]
    #[inline] pub fn set_intclr<F: FnOnce(Intclr) -> Intclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(), f(Intclr(0)));
        }
        self
    }

    #[doc="Modify the INTCLR register."]
    #[inline] pub fn with_intclr<F: FnOnce(Intclr) -> Intclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intclr_mut(), f(self.intclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTSET register."]
    #[inline] pub fn intset_mut(&self) -> *mut Intset { 
        (self.0 + 0x20c) as *mut Intset
    }

    #[doc="Get the *const pointer for the INTSET register."]
    #[inline] pub fn intset_ptr(&self) -> *const Intset { 
           self.intset_mut()
    }

    #[doc="Read the INTSET register."]
    #[inline] pub fn intset(&self) -> Intset { 
        unsafe {
            read_volatile(self.intset_ptr())
        }
    }

    #[doc="Write the INTSET register."]
    #[inline] pub fn set_intset<F: FnOnce(Intset) -> Intset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intset_mut(), f(Intset(0)));
        }
        self
    }

    #[doc="Modify the INTSET register."]
    #[inline] pub fn with_intset<F: FnOnce(Intset) -> Intset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intset_mut(), f(self.intset()));
        }
        self
    }

}

#[doc="FIFO Access Port"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc="FIFO access port."]
    #[inline] pub fn fifo(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if FIFO != 0"]
    #[inline] pub fn test_fifo(&self) -> bool {
        self.fifo() != 0
    }

    #[doc="Sets the FIFO field."]
    #[inline] pub fn set_fifo<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifo {
    #[inline]
    fn from(other: u32) -> Self {
         Fifo(other)
    }
}

impl ::core::fmt::Display for Fifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Current FIFO Pointers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifoptr(pub u32);
impl Fifoptr {
    #[doc="The number of bytes remaining in the FIFO (i.e. 128-FIFOSIZ if FULLDUP = 0 or 64-FIFOSIZ if FULLDUP = 1))."]
    #[inline] pub fn fiforem(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if FIFOREM != 0"]
    #[inline] pub fn test_fiforem(&self) -> bool {
        self.fiforem() != 0
    }

    #[doc="Sets the FIFOREM field."]
    #[inline] pub fn set_fiforem<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="The number of bytes currently in the FIFO."]
    #[inline] pub fn fifosiz(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FIFOSIZ != 0"]
    #[inline] pub fn test_fifosiz(&self) -> bool {
        self.fifosiz() != 0
    }

    #[doc="Sets the FIFOSIZ field."]
    #[inline] pub fn set_fifosiz<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifoptr {
    #[inline]
    fn from(other: u32) -> Self {
         Fifoptr(other)
    }
}

impl ::core::fmt::Display for Fifoptr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifoptr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fiforem() != 0 { try!(write!(f, " fiforem=0x{:x}", self.fiforem()))}
        if self.fifosiz() != 0 { try!(write!(f, " fifosiz=0x{:x}", self.fifosiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transfer Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tlngth(pub u32);
impl Tlngth {
    #[doc="Remaining transfer length."]
    #[inline] pub fn tlngth(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if TLNGTH != 0"]
    #[inline] pub fn test_tlngth(&self) -> bool {
        self.tlngth() != 0
    }

    #[doc="Sets the TLNGTH field."]
    #[inline] pub fn set_tlngth<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tlngth {
    #[inline]
    fn from(other: u32) -> Self {
         Tlngth(other)
    }
}

impl ::core::fmt::Display for Tlngth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tlngth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tlngth() != 0 { try!(write!(f, " tlngth=0x{:x}", self.tlngth()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FIFO Threshold Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifothr(pub u32);
impl Fifothr {
    #[doc="FIFO write threshold."]
    #[inline] pub fn fifowthr(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if FIFOWTHR != 0"]
    #[inline] pub fn test_fifowthr(&self) -> bool {
        self.fifowthr() != 0
    }

    #[doc="Sets the FIFOWTHR field."]
    #[inline] pub fn set_fifowthr<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FIFO read threshold."]
    #[inline] pub fn fiforthr(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if FIFORTHR != 0"]
    #[inline] pub fn test_fiforthr(&self) -> bool {
        self.fiforthr() != 0
    }

    #[doc="Sets the FIFORTHR field."]
    #[inline] pub fn set_fiforthr<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifothr {
    #[inline]
    fn from(other: u32) -> Self {
         Fifothr(other)
    }
}

impl ::core::fmt::Display for Fifothr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifothr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fifowthr() != 0 { try!(write!(f, " fifowthr=0x{:x}", self.fifowthr()))}
        if self.fiforthr() != 0 { try!(write!(f, " fiforthr=0x{:x}", self.fiforthr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Clock Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkcfg(pub u32);
impl Clkcfg {
    #[doc="Clock total count minus 1."]
    #[inline] pub fn totper(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if TOTPER != 0"]
    #[inline] pub fn test_totper(&self) -> bool {
        self.totper() != 0
    }

    #[doc="Sets the TOTPER field."]
    #[inline] pub fn set_totper<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock low count minus 1."]
    #[inline] pub fn lowper(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if LOWPER != 0"]
    #[inline] pub fn test_lowper(&self) -> bool {
        self.lowper() != 0
    }

    #[doc="Sets the LOWPER field."]
    #[inline] pub fn set_lowper<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Enable clock division by TOTPER."]
    #[inline] pub fn diven(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DIVEN != 0"]
    #[inline] pub fn test_diven(&self) -> bool {
        self.diven() != 0
    }

    #[doc="Sets the DIVEN field."]
    #[inline] pub fn set_diven<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enable divide by 3."]
    #[inline] pub fn div3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DIV3 != 0"]
    #[inline] pub fn test_div3(&self) -> bool {
        self.div3() != 0
    }

    #[doc="Sets the DIV3 field."]
    #[inline] pub fn set_div3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Select the input clock frequency."]
    #[inline] pub fn fsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if FSEL != 0"]
    #[inline] pub fn test_fsel(&self) -> bool {
        self.fsel() != 0
    }

    #[doc="Sets the FSEL field."]
    #[inline] pub fn set_fsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Clkcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Clkcfg(other)
    }
}

impl ::core::fmt::Display for Clkcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clkcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.totper() != 0 { try!(write!(f, " totper=0x{:x}", self.totper()))}
        if self.lowper() != 0 { try!(write!(f, " lowper=0x{:x}", self.lowper()))}
        if self.diven() != 0 { try!(write!(f, " diven"))}
        if self.div3() != 0 { try!(write!(f, " div3"))}
        if self.fsel() != 0 { try!(write!(f, " fsel=0x{:x}", self.fsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Command Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc="This register holds the I/O Command"]
    #[inline] pub fn cmd(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmd {
    #[inline]
    fn from(other: u32) -> Self {
         Cmd(other)
    }
}

impl ::core::fmt::Display for Cmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Command Repeat Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmdrpt(pub u32);
impl Cmdrpt {
    #[doc="These bits hold the Command repeat count."]
    #[inline] pub fn cmdrpt(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if CMDRPT != 0"]
    #[inline] pub fn test_cmdrpt(&self) -> bool {
        self.cmdrpt() != 0
    }

    #[doc="Sets the CMDRPT field."]
    #[inline] pub fn set_cmdrpt<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cmdrpt {
    #[inline]
    fn from(other: u32) -> Self {
         Cmdrpt(other)
    }
}

impl ::core::fmt::Display for Cmdrpt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmdrpt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmdrpt() != 0 { try!(write!(f, " cmdrpt=0x{:x}", self.cmdrpt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="This bit indicates if the I/O state machine is IDLE."]
    #[inline] pub fn idlest(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IDLEST != 0"]
    #[inline] pub fn test_idlest(&self) -> bool {
        self.idlest() != 0
    }

    #[doc="Sets the IDLEST field."]
    #[inline] pub fn set_idlest<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit indicates if the I/O Command is active."]
    #[inline] pub fn cmdact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMDACT != 0"]
    #[inline] pub fn test_cmdact(&self) -> bool {
        self.cmdact() != 0
    }

    #[doc="Sets the CMDACT field."]
    #[inline] pub fn set_cmdact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit indicates if an error interrupt has occurred."]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Status {
    #[inline]
    fn from(other: u32) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.idlest() != 0 { try!(write!(f, " idlest"))}
        if self.cmdact() != 0 { try!(write!(f, " cmdact"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I/O Master Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc="This bit enables the IO Master."]
    #[inline] pub fn ifcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if IFCEN != 0"]
    #[inline] pub fn test_ifcen(&self) -> bool {
        self.ifcen() != 0
    }

    #[doc="Sets the IFCEN field."]
    #[inline] pub fn set_ifcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="This bit selects the read flow control signal polarity."]
    #[inline] pub fn rdfcpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RDFCPOL != 0"]
    #[inline] pub fn test_rdfcpol(&self) -> bool {
        self.rdfcpol() != 0
    }

    #[doc="Sets the RDFCPOL field."]
    #[inline] pub fn set_rdfcpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="This bit selects the write flow control signal polarity."]
    #[inline] pub fn wtfcpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if WTFCPOL != 0"]
    #[inline] pub fn test_wtfcpol(&self) -> bool {
        self.wtfcpol() != 0
    }

    #[doc="Sets the WTFCPOL field."]
    #[inline] pub fn set_wtfcpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="This bit selects the write mode flow control signal."]
    #[inline] pub fn wtfcirq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WTFCIRQ != 0"]
    #[inline] pub fn test_wtfcirq(&self) -> bool {
        self.wtfcirq() != 0
    }

    #[doc="Sets the WTFCIRQ field."]
    #[inline] pub fn set_wtfcirq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="This bit must be left at the default value of 0."]
    #[inline] pub fn fcdel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FCDEL != 0"]
    #[inline] pub fn test_fcdel(&self) -> bool {
        self.fcdel() != 0
    }

    #[doc="Sets the FCDEL field."]
    #[inline] pub fn set_fcdel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="This bit invewrts MOSI when flow control is enabled."]
    #[inline] pub fn mosiinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if MOSIINV != 0"]
    #[inline] pub fn test_mosiinv(&self) -> bool {
        self.mosiinv() != 0
    }

    #[doc="Sets the MOSIINV field."]
    #[inline] pub fn set_mosiinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This bit enables read mode flow control."]
    #[inline] pub fn rdfc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RDFC != 0"]
    #[inline] pub fn test_rdfc(&self) -> bool {
        self.rdfc() != 0
    }

    #[doc="Sets the RDFC field."]
    #[inline] pub fn set_rdfc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This bit enables write mode flow control."]
    #[inline] pub fn wtfc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WTFC != 0"]
    #[inline] pub fn test_wtfc(&self) -> bool {
        self.wtfc() != 0
    }

    #[doc="Sets the WTFC field."]
    #[inline] pub fn set_wtfc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This bit selects the preread timing."]
    #[inline] pub fn startrd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if STARTRD != 0"]
    #[inline] pub fn test_startrd(&self) -> bool {
        self.startrd() != 0
    }

    #[doc="Sets the STARTRD field."]
    #[inline] pub fn set_startrd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This bit selects full duplex mode."]
    #[inline] pub fn fulldup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FULLDUP != 0"]
    #[inline] pub fn test_fulldup(&self) -> bool {
        self.fulldup() != 0
    }

    #[doc="Sets the FULLDUP field."]
    #[inline] pub fn set_fulldup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This bit selects SPI phase."]
    #[inline] pub fn spha(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SPHA != 0"]
    #[inline] pub fn test_spha(&self) -> bool {
        self.spha() != 0
    }

    #[doc="Sets the SPHA field."]
    #[inline] pub fn set_spha<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This bit selects SPI polarity."]
    #[inline] pub fn spol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SPOL != 0"]
    #[inline] pub fn test_spol(&self) -> bool {
        self.spol() != 0
    }

    #[doc="Sets the SPOL field."]
    #[inline] pub fn set_spol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This bit selects the I/O interface."]
    #[inline] pub fn ifcsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IFCSEL != 0"]
    #[inline] pub fn test_ifcsel(&self) -> bool {
        self.ifcsel() != 0
    }

    #[doc="Sets the IFCSEL field."]
    #[inline] pub fn set_ifcsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Cfg(other)
    }
}

impl ::core::fmt::Display for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ifcen() != 0 { try!(write!(f, " ifcen"))}
        if self.rdfcpol() != 0 { try!(write!(f, " rdfcpol"))}
        if self.wtfcpol() != 0 { try!(write!(f, " wtfcpol"))}
        if self.wtfcirq() != 0 { try!(write!(f, " wtfcirq"))}
        if self.fcdel() != 0 { try!(write!(f, " fcdel"))}
        if self.mosiinv() != 0 { try!(write!(f, " mosiinv"))}
        if self.rdfc() != 0 { try!(write!(f, " rdfc"))}
        if self.wtfc() != 0 { try!(write!(f, " wtfc"))}
        if self.startrd() != 0 { try!(write!(f, " startrd=0x{:x}", self.startrd()))}
        if self.fulldup() != 0 { try!(write!(f, " fulldup"))}
        if self.spha() != 0 { try!(write!(f, " spha"))}
        if self.spol() != 0 { try!(write!(f, " spol"))}
        if self.ifcsel() != 0 { try!(write!(f, " ifcsel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Master Interrupts: Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc="This is the arbitration loss interrupt. This error occurs if another master collides with an IO Master transfer. Generally, the IOM started an operation but found SDA already low."]
    #[inline] pub fn arb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ARB != 0"]
    #[inline] pub fn test_arb(&self) -> bool {
        self.arb() != 0
    }

    #[doc="Sets the ARB field."]
    #[inline] pub fn set_arb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This is the STOP command interrupt. A STOP bit was detected by the IOM."]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This is the START command interrupt. A START from another master was detected. Software must wait for a STOP before proceeding."]
    #[inline] pub fn start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This is the illegal command interrupt. Software attempted to issue a CMD while another CMD was already in progress. Or an attempt was made to issue a non-zero-length write CMD with an empty FIFO."]
    #[inline] pub fn icmd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ICMD != 0"]
    #[inline] pub fn test_icmd(&self) -> bool {
        self.icmd() != 0
    }

    #[doc="Sets the ICMD field."]
    #[inline] pub fn set_icmd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This is the illegal FIFO access interrupt. An attempt was made to read the FIFO during a write CMD. Or an attempt was made to write the FIFO on a read CMD."]
    #[inline] pub fn iacc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IACC != 0"]
    #[inline] pub fn test_iacc(&self) -> bool {
        self.iacc() != 0
    }

    #[doc="Sets the IACC field."]
    #[inline] pub fn set_iacc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This is the WTLEN interrupt."]
    #[inline] pub fn wtlen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WTLEN != 0"]
    #[inline] pub fn test_wtlen(&self) -> bool {
        self.wtlen() != 0
    }

    #[doc="Sets the WTLEN field."]
    #[inline] pub fn set_wtlen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This is the I2C NAK interrupt. The expected ACK from the slave was not received by the IOM."]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This is the Write FIFO Overflow interrupt. An attempt was made to write the FIFO while it was full (i.e. while FIFOSIZ > 124)."]
    #[inline] pub fn fovfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FOVFL != 0"]
    #[inline] pub fn test_fovfl(&self) -> bool {
        self.fovfl() != 0
    }

    #[doc="Sets the FOVFL field."]
    #[inline] pub fn set_fovfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This is the Read FIFO Underflow interrupt. An attempt was made to read FIFO when empty (i.e. while FIFOSIZ less than 4)."]
    #[inline] pub fn fundfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FUNDFL != 0"]
    #[inline] pub fn test_fundfl(&self) -> bool {
        self.fundfl() != 0
    }

    #[doc="Sets the FUNDFL field."]
    #[inline] pub fn set_fundfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This is the FIFO Threshold interrupt."]
    #[inline] pub fn thr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if THR != 0"]
    #[inline] pub fn test_thr(&self) -> bool {
        self.thr() != 0
    }

    #[doc="Sets the THR field."]
    #[inline] pub fn set_thr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This is the Command Complete interrupt."]
    #[inline] pub fn cmdcmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDCMP != 0"]
    #[inline] pub fn test_cmdcmp(&self) -> bool {
        self.cmdcmp() != 0
    }

    #[doc="Sets the CMDCMP field."]
    #[inline] pub fn set_cmdcmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Inten {
    #[inline]
    fn from(other: u32) -> Self {
         Inten(other)
    }
}

impl ::core::fmt::Display for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.arb() != 0 { try!(write!(f, " arb"))}
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.icmd() != 0 { try!(write!(f, " icmd"))}
        if self.iacc() != 0 { try!(write!(f, " iacc"))}
        if self.wtlen() != 0 { try!(write!(f, " wtlen"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.fovfl() != 0 { try!(write!(f, " fovfl"))}
        if self.fundfl() != 0 { try!(write!(f, " fundfl"))}
        if self.thr() != 0 { try!(write!(f, " thr"))}
        if self.cmdcmp() != 0 { try!(write!(f, " cmdcmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Master Interrupts: Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc="This is the arbitration loss interrupt. This error occurs if another master collides with an IO Master transfer. Generally, the IOM started an operation but found SDA already low."]
    #[inline] pub fn arb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ARB != 0"]
    #[inline] pub fn test_arb(&self) -> bool {
        self.arb() != 0
    }

    #[doc="Sets the ARB field."]
    #[inline] pub fn set_arb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This is the STOP command interrupt. A STOP bit was detected by the IOM."]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This is the START command interrupt. A START from another master was detected. Software must wait for a STOP before proceeding."]
    #[inline] pub fn start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This is the illegal command interrupt. Software attempted to issue a CMD while another CMD was already in progress. Or an attempt was made to issue a non-zero-length write CMD with an empty FIFO."]
    #[inline] pub fn icmd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ICMD != 0"]
    #[inline] pub fn test_icmd(&self) -> bool {
        self.icmd() != 0
    }

    #[doc="Sets the ICMD field."]
    #[inline] pub fn set_icmd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This is the illegal FIFO access interrupt. An attempt was made to read the FIFO during a write CMD. Or an attempt was made to write the FIFO on a read CMD."]
    #[inline] pub fn iacc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IACC != 0"]
    #[inline] pub fn test_iacc(&self) -> bool {
        self.iacc() != 0
    }

    #[doc="Sets the IACC field."]
    #[inline] pub fn set_iacc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This is the WTLEN interrupt."]
    #[inline] pub fn wtlen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WTLEN != 0"]
    #[inline] pub fn test_wtlen(&self) -> bool {
        self.wtlen() != 0
    }

    #[doc="Sets the WTLEN field."]
    #[inline] pub fn set_wtlen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This is the I2C NAK interrupt. The expected ACK from the slave was not received by the IOM."]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This is the Write FIFO Overflow interrupt. An attempt was made to write the FIFO while it was full (i.e. while FIFOSIZ > 124)."]
    #[inline] pub fn fovfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FOVFL != 0"]
    #[inline] pub fn test_fovfl(&self) -> bool {
        self.fovfl() != 0
    }

    #[doc="Sets the FOVFL field."]
    #[inline] pub fn set_fovfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This is the Read FIFO Underflow interrupt. An attempt was made to read FIFO when empty (i.e. while FIFOSIZ less than 4)."]
    #[inline] pub fn fundfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FUNDFL != 0"]
    #[inline] pub fn test_fundfl(&self) -> bool {
        self.fundfl() != 0
    }

    #[doc="Sets the FUNDFL field."]
    #[inline] pub fn set_fundfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This is the FIFO Threshold interrupt."]
    #[inline] pub fn thr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if THR != 0"]
    #[inline] pub fn test_thr(&self) -> bool {
        self.thr() != 0
    }

    #[doc="Sets the THR field."]
    #[inline] pub fn set_thr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This is the Command Complete interrupt."]
    #[inline] pub fn cmdcmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDCMP != 0"]
    #[inline] pub fn test_cmdcmp(&self) -> bool {
        self.cmdcmp() != 0
    }

    #[doc="Sets the CMDCMP field."]
    #[inline] pub fn set_cmdcmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intstat {
    #[inline]
    fn from(other: u32) -> Self {
         Intstat(other)
    }
}

impl ::core::fmt::Display for Intstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.arb() != 0 { try!(write!(f, " arb"))}
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.icmd() != 0 { try!(write!(f, " icmd"))}
        if self.iacc() != 0 { try!(write!(f, " iacc"))}
        if self.wtlen() != 0 { try!(write!(f, " wtlen"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.fovfl() != 0 { try!(write!(f, " fovfl"))}
        if self.fundfl() != 0 { try!(write!(f, " fundfl"))}
        if self.thr() != 0 { try!(write!(f, " thr"))}
        if self.cmdcmp() != 0 { try!(write!(f, " cmdcmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Master Interrupts: Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc="This is the arbitration loss interrupt. This error occurs if another master collides with an IO Master transfer. Generally, the IOM started an operation but found SDA already low."]
    #[inline] pub fn arb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ARB != 0"]
    #[inline] pub fn test_arb(&self) -> bool {
        self.arb() != 0
    }

    #[doc="Sets the ARB field."]
    #[inline] pub fn set_arb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This is the STOP command interrupt. A STOP bit was detected by the IOM."]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This is the START command interrupt. A START from another master was detected. Software must wait for a STOP before proceeding."]
    #[inline] pub fn start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This is the illegal command interrupt. Software attempted to issue a CMD while another CMD was already in progress. Or an attempt was made to issue a non-zero-length write CMD with an empty FIFO."]
    #[inline] pub fn icmd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ICMD != 0"]
    #[inline] pub fn test_icmd(&self) -> bool {
        self.icmd() != 0
    }

    #[doc="Sets the ICMD field."]
    #[inline] pub fn set_icmd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This is the illegal FIFO access interrupt. An attempt was made to read the FIFO during a write CMD. Or an attempt was made to write the FIFO on a read CMD."]
    #[inline] pub fn iacc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IACC != 0"]
    #[inline] pub fn test_iacc(&self) -> bool {
        self.iacc() != 0
    }

    #[doc="Sets the IACC field."]
    #[inline] pub fn set_iacc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This is the WTLEN interrupt."]
    #[inline] pub fn wtlen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WTLEN != 0"]
    #[inline] pub fn test_wtlen(&self) -> bool {
        self.wtlen() != 0
    }

    #[doc="Sets the WTLEN field."]
    #[inline] pub fn set_wtlen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This is the I2C NAK interrupt. The expected ACK from the slave was not received by the IOM."]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This is the Write FIFO Overflow interrupt. An attempt was made to write the FIFO while it was full (i.e. while FIFOSIZ > 124)."]
    #[inline] pub fn fovfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FOVFL != 0"]
    #[inline] pub fn test_fovfl(&self) -> bool {
        self.fovfl() != 0
    }

    #[doc="Sets the FOVFL field."]
    #[inline] pub fn set_fovfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This is the Read FIFO Underflow interrupt. An attempt was made to read FIFO when empty (i.e. while FIFOSIZ less than 4)."]
    #[inline] pub fn fundfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FUNDFL != 0"]
    #[inline] pub fn test_fundfl(&self) -> bool {
        self.fundfl() != 0
    }

    #[doc="Sets the FUNDFL field."]
    #[inline] pub fn set_fundfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This is the FIFO Threshold interrupt."]
    #[inline] pub fn thr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if THR != 0"]
    #[inline] pub fn test_thr(&self) -> bool {
        self.thr() != 0
    }

    #[doc="Sets the THR field."]
    #[inline] pub fn set_thr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This is the Command Complete interrupt."]
    #[inline] pub fn cmdcmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDCMP != 0"]
    #[inline] pub fn test_cmdcmp(&self) -> bool {
        self.cmdcmp() != 0
    }

    #[doc="Sets the CMDCMP field."]
    #[inline] pub fn set_cmdcmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intclr {
    #[inline]
    fn from(other: u32) -> Self {
         Intclr(other)
    }
}

impl ::core::fmt::Display for Intclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.arb() != 0 { try!(write!(f, " arb"))}
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.icmd() != 0 { try!(write!(f, " icmd"))}
        if self.iacc() != 0 { try!(write!(f, " iacc"))}
        if self.wtlen() != 0 { try!(write!(f, " wtlen"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.fovfl() != 0 { try!(write!(f, " fovfl"))}
        if self.fundfl() != 0 { try!(write!(f, " fundfl"))}
        if self.thr() != 0 { try!(write!(f, " thr"))}
        if self.cmdcmp() != 0 { try!(write!(f, " cmdcmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="IO Master Interrupts: Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc="This is the arbitration loss interrupt. This error occurs if another master collides with an IO Master transfer. Generally, the IOM started an operation but found SDA already low."]
    #[inline] pub fn arb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ARB != 0"]
    #[inline] pub fn test_arb(&self) -> bool {
        self.arb() != 0
    }

    #[doc="Sets the ARB field."]
    #[inline] pub fn set_arb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="This is the STOP command interrupt. A STOP bit was detected by the IOM."]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="This is the START command interrupt. A START from another master was detected. Software must wait for a STOP before proceeding."]
    #[inline] pub fn start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="This is the illegal command interrupt. Software attempted to issue a CMD while another CMD was already in progress. Or an attempt was made to issue a non-zero-length write CMD with an empty FIFO."]
    #[inline] pub fn icmd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ICMD != 0"]
    #[inline] pub fn test_icmd(&self) -> bool {
        self.icmd() != 0
    }

    #[doc="Sets the ICMD field."]
    #[inline] pub fn set_icmd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="This is the illegal FIFO access interrupt. An attempt was made to read the FIFO during a write CMD. Or an attempt was made to write the FIFO on a read CMD."]
    #[inline] pub fn iacc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IACC != 0"]
    #[inline] pub fn test_iacc(&self) -> bool {
        self.iacc() != 0
    }

    #[doc="Sets the IACC field."]
    #[inline] pub fn set_iacc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="This is the WTLEN interrupt."]
    #[inline] pub fn wtlen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WTLEN != 0"]
    #[inline] pub fn test_wtlen(&self) -> bool {
        self.wtlen() != 0
    }

    #[doc="Sets the WTLEN field."]
    #[inline] pub fn set_wtlen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="This is the I2C NAK interrupt. The expected ACK from the slave was not received by the IOM."]
    #[inline] pub fn nak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NAK != 0"]
    #[inline] pub fn test_nak(&self) -> bool {
        self.nak() != 0
    }

    #[doc="Sets the NAK field."]
    #[inline] pub fn set_nak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="This is the Write FIFO Overflow interrupt. An attempt was made to write the FIFO while it was full (i.e. while FIFOSIZ > 124)."]
    #[inline] pub fn fovfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FOVFL != 0"]
    #[inline] pub fn test_fovfl(&self) -> bool {
        self.fovfl() != 0
    }

    #[doc="Sets the FOVFL field."]
    #[inline] pub fn set_fovfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="This is the Read FIFO Underflow interrupt. An attempt was made to read FIFO when empty (i.e. while FIFOSIZ less than 4)."]
    #[inline] pub fn fundfl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FUNDFL != 0"]
    #[inline] pub fn test_fundfl(&self) -> bool {
        self.fundfl() != 0
    }

    #[doc="Sets the FUNDFL field."]
    #[inline] pub fn set_fundfl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="This is the FIFO Threshold interrupt."]
    #[inline] pub fn thr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if THR != 0"]
    #[inline] pub fn test_thr(&self) -> bool {
        self.thr() != 0
    }

    #[doc="Sets the THR field."]
    #[inline] pub fn set_thr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="This is the Command Complete interrupt."]
    #[inline] pub fn cmdcmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMDCMP != 0"]
    #[inline] pub fn test_cmdcmp(&self) -> bool {
        self.cmdcmp() != 0
    }

    #[doc="Sets the CMDCMP field."]
    #[inline] pub fn set_cmdcmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Intset {
    #[inline]
    fn from(other: u32) -> Self {
         Intset(other)
    }
}

impl ::core::fmt::Display for Intset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.arb() != 0 { try!(write!(f, " arb"))}
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.icmd() != 0 { try!(write!(f, " icmd"))}
        if self.iacc() != 0 { try!(write!(f, " iacc"))}
        if self.wtlen() != 0 { try!(write!(f, " wtlen"))}
        if self.nak() != 0 { try!(write!(f, " nak"))}
        if self.fovfl() != 0 { try!(write!(f, " fovfl"))}
        if self.fundfl() != 0 { try!(write!(f, " fundfl"))}
        if self.thr() != 0 { try!(write!(f, " thr"))}
        if self.cmdcmp() != 0 { try!(write!(f, " cmdcmp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}


