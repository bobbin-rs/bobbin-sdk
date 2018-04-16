#[allow(unused_imports)] use ::bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="FTFE Peripheral"]
pub struct FtfePeriph(pub usize); 

impl FtfePeriph {
    #[doc="Get the FSTAT Register."]
    #[inline] pub fn fstat_reg(&self) -> Register<Fstat> { 
        Register::new(self.0 as *mut Fstat, 0x0)
    }

    #[doc="Get the *mut pointer for the FSTAT register."]
    #[inline] pub fn fstat_mut(&self) -> *mut Fstat { 
        self.fstat_reg().ptr()
    }

    #[doc="Get the *const pointer for the FSTAT register."]
    #[inline] pub fn fstat_ptr(&self) -> *const Fstat { 
        self.fstat_reg().ptr()
    }

    #[doc="Read the FSTAT register."]
    #[inline] pub fn fstat(&self) -> Fstat { 
        self.fstat_reg().read()
    }

    #[doc="Write the FSTAT register."]
    #[inline] pub fn write_fstat(&self, value: Fstat) -> &Self { 
        self.fstat_reg().write(value);
        self
    }

    #[doc="Set the FSTAT register."]
    #[inline] pub fn set_fstat<F: FnOnce(Fstat) -> Fstat>(&self, f: F) -> &Self {
        self.fstat_reg().set(f);
        self
    }

    #[doc="Modify the FSTAT register."]
    #[inline] pub fn with_fstat<F: FnOnce(Fstat) -> Fstat>(&self, f: F) -> &Self {
        self.fstat_reg().with(f);
        self
    }

    #[doc="Get the FCNFG Register."]
    #[inline] pub fn fcnfg_reg(&self) -> Register<Fcnfg> { 
        Register::new(self.0 as *mut Fcnfg, 0x1)
    }

    #[doc="Get the *mut pointer for the FCNFG register."]
    #[inline] pub fn fcnfg_mut(&self) -> *mut Fcnfg { 
        self.fcnfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the FCNFG register."]
    #[inline] pub fn fcnfg_ptr(&self) -> *const Fcnfg { 
        self.fcnfg_reg().ptr()
    }

    #[doc="Read the FCNFG register."]
    #[inline] pub fn fcnfg(&self) -> Fcnfg { 
        self.fcnfg_reg().read()
    }

    #[doc="Write the FCNFG register."]
    #[inline] pub fn write_fcnfg(&self, value: Fcnfg) -> &Self { 
        self.fcnfg_reg().write(value);
        self
    }

    #[doc="Set the FCNFG register."]
    #[inline] pub fn set_fcnfg<F: FnOnce(Fcnfg) -> Fcnfg>(&self, f: F) -> &Self {
        self.fcnfg_reg().set(f);
        self
    }

    #[doc="Modify the FCNFG register."]
    #[inline] pub fn with_fcnfg<F: FnOnce(Fcnfg) -> Fcnfg>(&self, f: F) -> &Self {
        self.fcnfg_reg().with(f);
        self
    }

    #[doc="Get the FSEC Register."]
    #[inline] pub fn fsec_reg(&self) -> Register<Fsec> { 
        Register::new(self.0 as *mut Fsec, 0x2)
    }

    #[doc="Get the *mut pointer for the FSEC register."]
    #[inline] pub fn fsec_mut(&self) -> *mut Fsec { 
        self.fsec_reg().ptr()
    }

    #[doc="Get the *const pointer for the FSEC register."]
    #[inline] pub fn fsec_ptr(&self) -> *const Fsec { 
        self.fsec_reg().ptr()
    }

    #[doc="Read the FSEC register."]
    #[inline] pub fn fsec(&self) -> Fsec { 
        self.fsec_reg().read()
    }

    #[doc="Get the FOPT Register."]
    #[inline] pub fn fopt_reg(&self) -> Register<Fopt> { 
        Register::new(self.0 as *mut Fopt, 0x3)
    }

    #[doc="Get the *mut pointer for the FOPT register."]
    #[inline] pub fn fopt_mut(&self) -> *mut Fopt { 
        self.fopt_reg().ptr()
    }

    #[doc="Get the *const pointer for the FOPT register."]
    #[inline] pub fn fopt_ptr(&self) -> *const Fopt { 
        self.fopt_reg().ptr()
    }

    #[doc="Read the FOPT register."]
    #[inline] pub fn fopt(&self) -> Fopt { 
        self.fopt_reg().read()
    }

    #[doc="Get the FCCOB Register."]
    #[inline] pub fn fccob_reg(&self) -> RegisterArray<Fccob, bits::R12> { 
        RegisterArray::new(self.0 as *mut Fccob, 0x4, 0x1)
    }

    #[doc="Get the *mut pointer for the FCCOB register."]
    #[inline] pub fn fccob_mut<I: Into<bits::R12>>(&self, index: I) -> *mut Fccob { 
        self.fccob_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the FCCOB register."]
    #[inline] pub fn fccob_ptr<I: Into<bits::R12>>(&self, index: I) -> *const Fccob { 
        self.fccob_reg().ptr(index.into())
    }

    #[doc="Read the FCCOB register."]
    #[inline] pub fn fccob<I: Into<bits::R12>>(&self, index: I) -> Fccob { 
        self.fccob_reg().read(index.into())
    }

    #[doc="Write the FCCOB register."]
    #[inline] pub fn write_fccob<I: Into<bits::R12>>(&self, index: I, value: Fccob) -> &Self {
        self.fccob_reg().write(index.into(), value);
        self
    }

    #[doc="Set the FCCOB register."]
    #[inline] pub fn set_fccob<I: Into<bits::R12>, F: FnOnce(Fccob) -> Fccob>(&self, index: I, f: F) -> &Self {
        self.fccob_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the FCCOB register."]
    #[inline] pub fn with_fccob<I: Into<bits::R12> + Copy, F: FnOnce(Fccob) -> Fccob>(&self, index: I, f: F) -> &Self {
        self.fccob_reg().with(index.into(), f);
        self
    }

    #[doc="Get the FPROT Register."]
    #[inline] pub fn fprot_reg(&self) -> RegisterArray<Fprot, bits::R4> { 
        RegisterArray::new(self.0 as *mut Fprot, 0x10, 0x1)
    }

    #[doc="Get the *mut pointer for the FPROT register."]
    #[inline] pub fn fprot_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Fprot { 
        self.fprot_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the FPROT register."]
    #[inline] pub fn fprot_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Fprot { 
        self.fprot_reg().ptr(index.into())
    }

    #[doc="Read the FPROT register."]
    #[inline] pub fn fprot<I: Into<bits::R4>>(&self, index: I) -> Fprot { 
        self.fprot_reg().read(index.into())
    }

    #[doc="Write the FPROT register."]
    #[inline] pub fn write_fprot<I: Into<bits::R4>>(&self, index: I, value: Fprot) -> &Self {
        self.fprot_reg().write(index.into(), value);
        self
    }

    #[doc="Set the FPROT register."]
    #[inline] pub fn set_fprot<I: Into<bits::R4>, F: FnOnce(Fprot) -> Fprot>(&self, index: I, f: F) -> &Self {
        self.fprot_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the FPROT register."]
    #[inline] pub fn with_fprot<I: Into<bits::R4> + Copy, F: FnOnce(Fprot) -> Fprot>(&self, index: I, f: F) -> &Self {
        self.fprot_reg().with(index.into(), f);
        self
    }

    #[doc="Get the FEPROT Register."]
    #[inline] pub fn feprot_reg(&self) -> Register<Feprot> { 
        Register::new(self.0 as *mut Feprot, 0x16)
    }

    #[doc="Get the *mut pointer for the FEPROT register."]
    #[inline] pub fn feprot_mut(&self) -> *mut Feprot { 
        self.feprot_reg().ptr()
    }

    #[doc="Get the *const pointer for the FEPROT register."]
    #[inline] pub fn feprot_ptr(&self) -> *const Feprot { 
        self.feprot_reg().ptr()
    }

    #[doc="Read the FEPROT register."]
    #[inline] pub fn feprot(&self) -> Feprot { 
        self.feprot_reg().read()
    }

    #[doc="Write the FEPROT register."]
    #[inline] pub fn write_feprot(&self, value: Feprot) -> &Self { 
        self.feprot_reg().write(value);
        self
    }

    #[doc="Set the FEPROT register."]
    #[inline] pub fn set_feprot<F: FnOnce(Feprot) -> Feprot>(&self, f: F) -> &Self {
        self.feprot_reg().set(f);
        self
    }

    #[doc="Modify the FEPROT register."]
    #[inline] pub fn with_feprot<F: FnOnce(Feprot) -> Feprot>(&self, f: F) -> &Self {
        self.feprot_reg().with(f);
        self
    }

    #[doc="Get the FDPROT Register."]
    #[inline] pub fn fdprot_reg(&self) -> Register<Fdprot> { 
        Register::new(self.0 as *mut Fdprot, 0x17)
    }

    #[doc="Get the *mut pointer for the FDPROT register."]
    #[inline] pub fn fdprot_mut(&self) -> *mut Fdprot { 
        self.fdprot_reg().ptr()
    }

    #[doc="Get the *const pointer for the FDPROT register."]
    #[inline] pub fn fdprot_ptr(&self) -> *const Fdprot { 
        self.fdprot_reg().ptr()
    }

    #[doc="Read the FDPROT register."]
    #[inline] pub fn fdprot(&self) -> Fdprot { 
        self.fdprot_reg().read()
    }

    #[doc="Write the FDPROT register."]
    #[inline] pub fn write_fdprot(&self, value: Fdprot) -> &Self { 
        self.fdprot_reg().write(value);
        self
    }

    #[doc="Set the FDPROT register."]
    #[inline] pub fn set_fdprot<F: FnOnce(Fdprot) -> Fdprot>(&self, f: F) -> &Self {
        self.fdprot_reg().set(f);
        self
    }

    #[doc="Modify the FDPROT register."]
    #[inline] pub fn with_fdprot<F: FnOnce(Fdprot) -> Fdprot>(&self, f: F) -> &Self {
        self.fdprot_reg().with(f);
        self
    }

}

#[doc="Flash Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fstat(pub u8);
impl Fstat {
    #[doc="Memory Controller Command Completion Status Flag"]
    #[inline] pub fn mgstat0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MGSTAT0 != 0"]
    #[inline] pub fn test_mgstat0(&self) -> bool {
        self.mgstat0() != 0
    }

    #[doc="Sets the MGSTAT0 field."]
    #[inline] pub fn set_mgstat0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Flash Protection Violation Flag"]
    #[inline] pub fn fpviol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FPVIOL != 0"]
    #[inline] pub fn test_fpviol(&self) -> bool {
        self.fpviol() != 0
    }

    #[doc="Sets the FPVIOL field."]
    #[inline] pub fn set_fpviol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Flash Access Error Flag"]
    #[inline] pub fn accerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACCERR != 0"]
    #[inline] pub fn test_accerr(&self) -> bool {
        self.accerr() != 0
    }

    #[doc="Sets the ACCERR field."]
    #[inline] pub fn set_accerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FTFE Read Collision Error Flag"]
    #[inline] pub fn rdcolerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RDCOLERR != 0"]
    #[inline] pub fn test_rdcolerr(&self) -> bool {
        self.rdcolerr() != 0
    }

    #[doc="Sets the RDCOLERR field."]
    #[inline] pub fn set_rdcolerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Command Complete Interrupt Flag"]
    #[inline] pub fn ccif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CCIF != 0"]
    #[inline] pub fn test_ccif(&self) -> bool {
        self.ccif() != 0
    }

    #[doc="Sets the CCIF field."]
    #[inline] pub fn set_ccif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Fstat {
    #[inline]
    fn from(other: u8) -> Self {
         Fstat(other)
    }
}

impl ::core::fmt::Display for Fstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mgstat0() != 0 { try!(write!(f, " mgstat0"))}
        if self.fpviol() != 0 { try!(write!(f, " fpviol"))}
        if self.accerr() != 0 { try!(write!(f, " accerr"))}
        if self.rdcolerr() != 0 { try!(write!(f, " rdcolerr"))}
        if self.ccif() != 0 { try!(write!(f, " ccif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fcnfg(pub u8);
impl Fcnfg {
    #[doc="For devices with FlexNVM: This flag indicates if the EEPROM backup data has been copied to the FlexRAM and is therefore available for read access"]
    #[inline] pub fn eeerdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EEERDY != 0"]
    #[inline] pub fn test_eeerdy(&self) -> bool {
        self.eeerdy() != 0
    }

    #[doc="Sets the EEERDY field."]
    #[inline] pub fn set_eeerdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RAM Ready"]
    #[inline] pub fn ramrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RAMRDY != 0"]
    #[inline] pub fn test_ramrdy(&self) -> bool {
        self.ramrdy() != 0
    }

    #[doc="Sets the RAMRDY field."]
    #[inline] pub fn set_ramrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FTFE configuration"]
    #[inline] pub fn pflsh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PFLSH != 0"]
    #[inline] pub fn test_pflsh(&self) -> bool {
        self.pflsh() != 0
    }

    #[doc="Sets the PFLSH field."]
    #[inline] pub fn set_pflsh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Swap"]
    #[inline] pub fn swap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SWAP != 0"]
    #[inline] pub fn test_swap(&self) -> bool {
        self.swap() != 0
    }

    #[doc="Sets the SWAP field."]
    #[inline] pub fn set_swap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Erase Suspend"]
    #[inline] pub fn erssusp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ERSSUSP != 0"]
    #[inline] pub fn test_erssusp(&self) -> bool {
        self.erssusp() != 0
    }

    #[doc="Sets the ERSSUSP field."]
    #[inline] pub fn set_erssusp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Erase All Request"]
    #[inline] pub fn ersareq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ERSAREQ != 0"]
    #[inline] pub fn test_ersareq(&self) -> bool {
        self.ersareq() != 0
    }

    #[doc="Sets the ERSAREQ field."]
    #[inline] pub fn set_ersareq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Read Collision Error Interrupt Enable"]
    #[inline] pub fn rdcollie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RDCOLLIE != 0"]
    #[inline] pub fn test_rdcollie(&self) -> bool {
        self.rdcollie() != 0
    }

    #[doc="Sets the RDCOLLIE field."]
    #[inline] pub fn set_rdcollie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Command Complete Interrupt Enable"]
    #[inline] pub fn ccie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CCIE != 0"]
    #[inline] pub fn test_ccie(&self) -> bool {
        self.ccie() != 0
    }

    #[doc="Sets the CCIE field."]
    #[inline] pub fn set_ccie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Fcnfg {
    #[inline]
    fn from(other: u8) -> Self {
         Fcnfg(other)
    }
}

impl ::core::fmt::Display for Fcnfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fcnfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eeerdy() != 0 { try!(write!(f, " eeerdy"))}
        if self.ramrdy() != 0 { try!(write!(f, " ramrdy"))}
        if self.pflsh() != 0 { try!(write!(f, " pflsh"))}
        if self.swap() != 0 { try!(write!(f, " swap"))}
        if self.erssusp() != 0 { try!(write!(f, " erssusp"))}
        if self.ersareq() != 0 { try!(write!(f, " ersareq"))}
        if self.rdcollie() != 0 { try!(write!(f, " rdcollie"))}
        if self.ccie() != 0 { try!(write!(f, " ccie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Security Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fsec(pub u8);
impl Fsec {
    #[doc="Flash Security"]
    #[inline] pub fn sec(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SEC != 0"]
    #[inline] pub fn test_sec(&self) -> bool {
        self.sec() != 0
    }

    #[doc="Sets the SEC field."]
    #[inline] pub fn set_sec<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Freescale Failure Analysis Access Code"]
    #[inline] pub fn fslacc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if FSLACC != 0"]
    #[inline] pub fn test_fslacc(&self) -> bool {
        self.fslacc() != 0
    }

    #[doc="Sets the FSLACC field."]
    #[inline] pub fn set_fslacc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mass Erase Enable Bits"]
    #[inline] pub fn meen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if MEEN != 0"]
    #[inline] pub fn test_meen(&self) -> bool {
        self.meen() != 0
    }

    #[doc="Sets the MEEN field."]
    #[inline] pub fn set_meen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Backdoor Key Security Enable"]
    #[inline] pub fn keyen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if KEYEN != 0"]
    #[inline] pub fn test_keyen(&self) -> bool {
        self.keyen() != 0
    }

    #[doc="Sets the KEYEN field."]
    #[inline] pub fn set_keyen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Fsec {
    #[inline]
    fn from(other: u8) -> Self {
         Fsec(other)
    }
}

impl ::core::fmt::Display for Fsec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fsec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sec() != 0 { try!(write!(f, " sec=0x{:x}", self.sec()))}
        if self.fslacc() != 0 { try!(write!(f, " fslacc=0x{:x}", self.fslacc()))}
        if self.meen() != 0 { try!(write!(f, " meen=0x{:x}", self.meen()))}
        if self.keyen() != 0 { try!(write!(f, " keyen=0x{:x}", self.keyen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Option Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fopt(pub u8);
impl Fopt {
    #[doc="Nonvolatile Option"]
    #[inline] pub fn opt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if OPT != 0"]
    #[inline] pub fn test_opt(&self) -> bool {
        self.opt() != 0
    }

    #[doc="Sets the OPT field."]
    #[inline] pub fn set_opt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fopt {
    #[inline]
    fn from(other: u8) -> Self {
         Fopt(other)
    }
}

impl ::core::fmt::Display for Fopt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fopt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.opt() != 0 { try!(write!(f, " opt=0x{:x}", self.opt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Common Command Object Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fccob(pub u8);
impl Fccob {
    #[doc="The FCCOB register provides a command code and relevant parameters to the memory controller"]
    #[inline] pub fn ccob(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CCOB != 0"]
    #[inline] pub fn test_ccob(&self) -> bool {
        self.ccob() != 0
    }

    #[doc="Sets the CCOB field."]
    #[inline] pub fn set_ccob<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fccob {
    #[inline]
    fn from(other: u8) -> Self {
         Fccob(other)
    }
}

impl ::core::fmt::Display for Fccob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fccob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccob() != 0 { try!(write!(f, " ccob=0x{:x}", self.ccob()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Program Flash Protection Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fprot(pub u8);
impl Fprot {
    #[doc="Program Flash Region Protect"]
    #[inline] pub fn prot(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PROT != 0"]
    #[inline] pub fn test_prot(&self) -> bool {
        self.prot() != 0
    }

    #[doc="Sets the PROT field."]
    #[inline] pub fn set_prot<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fprot {
    #[inline]
    fn from(other: u8) -> Self {
         Fprot(other)
    }
}

impl ::core::fmt::Display for Fprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prot() != 0 { try!(write!(f, " prot=0x{:x}", self.prot()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EEPROM Protection Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Feprot(pub u8);
impl Feprot {
    #[doc="EEPROM Region Protect"]
    #[inline] pub fn eprot(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if EPROT != 0"]
    #[inline] pub fn test_eprot(&self) -> bool {
        self.eprot() != 0
    }

    #[doc="Sets the EPROT field."]
    #[inline] pub fn set_eprot<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Feprot {
    #[inline]
    fn from(other: u8) -> Self {
         Feprot(other)
    }
}

impl ::core::fmt::Display for Feprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Feprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eprot() != 0 { try!(write!(f, " eprot=0x{:x}", self.eprot()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Flash Protection Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fdprot(pub u8);
impl Fdprot {
    #[doc="Data Flash Region Protect"]
    #[inline] pub fn dprot(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DPROT != 0"]
    #[inline] pub fn test_dprot(&self) -> bool {
        self.dprot() != 0
    }

    #[doc="Sets the DPROT field."]
    #[inline] pub fn set_dprot<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fdprot {
    #[inline]
    fn from(other: u8) -> Self {
         Fdprot(other)
    }
}

impl ::core::fmt::Display for Fdprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fdprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dprot() != 0 { try!(write!(f, " dprot=0x{:x}", self.dprot()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

