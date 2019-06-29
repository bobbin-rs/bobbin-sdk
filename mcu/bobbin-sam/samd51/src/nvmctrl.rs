::bobbin_mcu::periph!( NVMCTRL, Nvmctrl, NVMCTRL_PERIPH, NvmctrlPeriph, NVMCTRL_OWNED, NVMCTRL_REF_COUNT, 0x41004000, 0x00, 0x11);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="NVMCTRL Peripheral"]
pub struct NvmctrlPeriph(pub usize); 

impl NvmctrlPeriph {
    #[doc="Get the CTRLA Register."]
    #[inline] pub fn ctrla_reg(&self) -> ::bobbin_mcu::register::Register<Ctrla> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrla, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRLA register."]
    #[inline] pub fn ctrla_mut(&self) -> *mut Ctrla { 
        self.ctrla_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLA register."]
    #[inline] pub fn ctrla_ptr(&self) -> *const Ctrla { 
        self.ctrla_reg().ptr()
    }

    #[doc="Read the CTRLA register."]
    #[inline] pub fn ctrla(&self) -> Ctrla { 
        self.ctrla_reg().read()
    }

    #[doc="Write the CTRLA register."]
    #[inline] pub fn write_ctrla(&self, value: Ctrla) -> &Self { 
        self.ctrla_reg().write(value);
        self
    }

    #[doc="Set the CTRLA register."]
    #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        self.ctrla_reg().set(f);
        self
    }

    #[doc="Modify the CTRLA register."]
    #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        self.ctrla_reg().with(f);
        self
    }

    #[doc="Get the CTRLB Register."]
    #[inline] pub fn ctrlb_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlb, 0x4)
    }

    #[doc="Get the *mut pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_mut(&self) -> *mut Ctrlb { 
        self.ctrlb_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_ptr(&self) -> *const Ctrlb { 
        self.ctrlb_reg().ptr()
    }

    #[doc="Write the CTRLB register."]
    #[inline] pub fn write_ctrlb(&self, value: Ctrlb) -> &Self { 
        self.ctrlb_reg().write(value);
        self
    }

    #[doc="Set the CTRLB register."]
    #[inline] pub fn set_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
        self.ctrlb_reg().set(f);
        self
    }

    #[doc="Get the PARAM Register."]
    #[inline] pub fn param_reg(&self) -> ::bobbin_mcu::register::Register<Param> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Param, 0x8)
    }

    #[doc="Get the *mut pointer for the PARAM register."]
    #[inline] pub fn param_mut(&self) -> *mut Param { 
        self.param_reg().ptr()
    }

    #[doc="Get the *const pointer for the PARAM register."]
    #[inline] pub fn param_ptr(&self) -> *const Param { 
        self.param_reg().ptr()
    }

    #[doc="Read the PARAM register."]
    #[inline] pub fn param(&self) -> Param { 
        self.param_reg().read()
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0xc)
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        self.intenclr_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
        self.intenclr_reg().ptr()
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        self.intenclr_reg().read()
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn write_intenclr(&self, value: Intenclr) -> &Self { 
        self.intenclr_reg().write(value);
        self
    }

    #[doc="Set the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        self.intenclr_reg().set(f);
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        self.intenclr_reg().with(f);
        self
    }

    #[doc="Get the INTENSET Register."]
    #[inline] pub fn intenset_reg(&self) -> ::bobbin_mcu::register::Register<Intenset> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0xe)
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        self.intenset_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
        self.intenset_reg().ptr()
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        self.intenset_reg().read()
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn write_intenset(&self, value: Intenset) -> &Self { 
        self.intenset_reg().write(value);
        self
    }

    #[doc="Set the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        self.intenset_reg().set(f);
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        self.intenset_reg().with(f);
        self
    }

    #[doc="Get the INTFLAG Register."]
    #[inline] pub fn intflag_reg(&self) -> ::bobbin_mcu::register::Register<Intflag> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x10)
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        self.intflag_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
        self.intflag_reg().ptr()
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        self.intflag_reg().read()
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn write_intflag(&self, value: Intflag) -> &Self { 
        self.intflag_reg().write(value);
        self
    }

    #[doc="Set the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        self.intflag_reg().set(f);
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        self.intflag_reg().with(f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x12)
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        self.status_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
        self.status_reg().ptr()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        self.status_reg().read()
    }

    #[doc="Get the ADDR Register."]
    #[inline] pub fn addr_reg(&self) -> ::bobbin_mcu::register::Register<Addr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Addr, 0x14)
    }

    #[doc="Get the *mut pointer for the ADDR register."]
    #[inline] pub fn addr_mut(&self) -> *mut Addr { 
        self.addr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ADDR register."]
    #[inline] pub fn addr_ptr(&self) -> *const Addr { 
        self.addr_reg().ptr()
    }

    #[doc="Read the ADDR register."]
    #[inline] pub fn addr(&self) -> Addr { 
        self.addr_reg().read()
    }

    #[doc="Write the ADDR register."]
    #[inline] pub fn write_addr(&self, value: Addr) -> &Self { 
        self.addr_reg().write(value);
        self
    }

    #[doc="Set the ADDR register."]
    #[inline] pub fn set_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
        self.addr_reg().set(f);
        self
    }

    #[doc="Modify the ADDR register."]
    #[inline] pub fn with_addr<F: FnOnce(Addr) -> Addr>(&self, f: F) -> &Self {
        self.addr_reg().with(f);
        self
    }

    #[doc="Get the RUNLOCK Register."]
    #[inline] pub fn runlock_reg(&self) -> ::bobbin_mcu::register::Register<Runlock> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Runlock, 0x18)
    }

    #[doc="Get the *mut pointer for the RUNLOCK register."]
    #[inline] pub fn runlock_mut(&self) -> *mut Runlock { 
        self.runlock_reg().ptr()
    }

    #[doc="Get the *const pointer for the RUNLOCK register."]
    #[inline] pub fn runlock_ptr(&self) -> *const Runlock { 
        self.runlock_reg().ptr()
    }

    #[doc="Read the RUNLOCK register."]
    #[inline] pub fn runlock(&self) -> Runlock { 
        self.runlock_reg().read()
    }

    #[doc="Get the PBLDATA Register."]
    #[inline] pub fn pbldata_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pbldata, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pbldata, 0x1c, 0x4)
    }

    #[doc="Get the *mut pointer for the PBLDATA register."]
    #[inline] pub fn pbldata_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Pbldata { 
        self.pbldata_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PBLDATA register."]
    #[inline] pub fn pbldata_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Pbldata { 
        self.pbldata_reg().ptr(index.into())
    }

    #[doc="Read the PBLDATA register."]
    #[inline] pub fn pbldata<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Pbldata { 
        self.pbldata_reg().read(index.into())
    }

    #[doc="Get the ECCERR Register."]
    #[inline] pub fn eccerr_reg(&self) -> ::bobbin_mcu::register::Register<Eccerr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Eccerr, 0x24)
    }

    #[doc="Get the *mut pointer for the ECCERR register."]
    #[inline] pub fn eccerr_mut(&self) -> *mut Eccerr { 
        self.eccerr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ECCERR register."]
    #[inline] pub fn eccerr_ptr(&self) -> *const Eccerr { 
        self.eccerr_reg().ptr()
    }

    #[doc="Read the ECCERR register."]
    #[inline] pub fn eccerr(&self) -> Eccerr { 
        self.eccerr_reg().read()
    }

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0x28)
    }

    #[doc="Get the *mut pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_mut(&self) -> *mut Dbgctrl { 
        self.dbgctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_ptr(&self) -> *const Dbgctrl { 
        self.dbgctrl_reg().ptr()
    }

    #[doc="Read the DBGCTRL register."]
    #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
        self.dbgctrl_reg().read()
    }

    #[doc="Write the DBGCTRL register."]
    #[inline] pub fn write_dbgctrl(&self, value: Dbgctrl) -> &Self { 
        self.dbgctrl_reg().write(value);
        self
    }

    #[doc="Set the DBGCTRL register."]
    #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        self.dbgctrl_reg().set(f);
        self
    }

    #[doc="Modify the DBGCTRL register."]
    #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        self.dbgctrl_reg().with(f);
        self
    }

    #[doc="Get the SEECFG Register."]
    #[inline] pub fn seecfg_reg(&self) -> ::bobbin_mcu::register::Register<Seecfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Seecfg, 0x2a)
    }

    #[doc="Get the *mut pointer for the SEECFG register."]
    #[inline] pub fn seecfg_mut(&self) -> *mut Seecfg { 
        self.seecfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the SEECFG register."]
    #[inline] pub fn seecfg_ptr(&self) -> *const Seecfg { 
        self.seecfg_reg().ptr()
    }

    #[doc="Read the SEECFG register."]
    #[inline] pub fn seecfg(&self) -> Seecfg { 
        self.seecfg_reg().read()
    }

    #[doc="Write the SEECFG register."]
    #[inline] pub fn write_seecfg(&self, value: Seecfg) -> &Self { 
        self.seecfg_reg().write(value);
        self
    }

    #[doc="Set the SEECFG register."]
    #[inline] pub fn set_seecfg<F: FnOnce(Seecfg) -> Seecfg>(&self, f: F) -> &Self {
        self.seecfg_reg().set(f);
        self
    }

    #[doc="Modify the SEECFG register."]
    #[inline] pub fn with_seecfg<F: FnOnce(Seecfg) -> Seecfg>(&self, f: F) -> &Self {
        self.seecfg_reg().with(f);
        self
    }

    #[doc="Get the SEESTAT Register."]
    #[inline] pub fn seestat_reg(&self) -> ::bobbin_mcu::register::Register<Seestat> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Seestat, 0x2c)
    }

    #[doc="Get the *mut pointer for the SEESTAT register."]
    #[inline] pub fn seestat_mut(&self) -> *mut Seestat { 
        self.seestat_reg().ptr()
    }

    #[doc="Get the *const pointer for the SEESTAT register."]
    #[inline] pub fn seestat_ptr(&self) -> *const Seestat { 
        self.seestat_reg().ptr()
    }

    #[doc="Read the SEESTAT register."]
    #[inline] pub fn seestat(&self) -> Seestat { 
        self.seestat_reg().read()
    }

}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
    #[doc="Auto Wait State Enable"]
    #[inline] pub fn autows(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AUTOWS != 0"]
    #[inline] pub fn test_autows(&self) -> bool {
        self.autows() != 0
    }

    #[doc="Sets the AUTOWS field."]
    #[inline] pub fn set_autows<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Suspend Enable"]
    #[inline] pub fn suspen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SUSPEN != 0"]
    #[inline] pub fn test_suspen(&self) -> bool {
        self.suspen() != 0
    }

    #[doc="Sets the SUSPEN field."]
    #[inline] pub fn set_suspen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Write Mode"]
    #[inline] pub fn wmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if WMODE != 0"]
    #[inline] pub fn test_wmode(&self) -> bool {
        self.wmode() != 0
    }

    #[doc="Sets the WMODE field."]
    #[inline] pub fn set_wmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Power Reduction Mode during Sleep"]
    #[inline] pub fn prm(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if PRM != 0"]
    #[inline] pub fn test_prm(&self) -> bool {
        self.prm() != 0
    }

    #[doc="Sets the PRM field."]
    #[inline] pub fn set_prm<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="NVM Read Wait States"]
    #[inline] pub fn rws(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if RWS != 0"]
    #[inline] pub fn test_rws(&self) -> bool {
        self.rws() != 0
    }

    #[doc="Sets the RWS field."]
    #[inline] pub fn set_rws<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline] pub fn ahbns0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if AHBNS0 != 0"]
    #[inline] pub fn test_ahbns0(&self) -> bool {
        self.ahbns0() != 0
    }

    #[doc="Sets the AHBNS0 field."]
    #[inline] pub fn set_ahbns0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline] pub fn ahbns1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if AHBNS1 != 0"]
    #[inline] pub fn test_ahbns1(&self) -> bool {
        self.ahbns1() != 0
    }

    #[doc="Sets the AHBNS1 field."]
    #[inline] pub fn set_ahbns1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="AHB0 Cache Disable"]
    #[inline] pub fn cachedis0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CACHEDIS0 != 0"]
    #[inline] pub fn test_cachedis0(&self) -> bool {
        self.cachedis0() != 0
    }

    #[doc="Sets the CACHEDIS0 field."]
    #[inline] pub fn set_cachedis0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="AHB1 Cache Disable"]
    #[inline] pub fn cachedis1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CACHEDIS1 != 0"]
    #[inline] pub fn test_cachedis1(&self) -> bool {
        self.cachedis1() != 0
    }

    #[doc="Sets the CACHEDIS1 field."]
    #[inline] pub fn set_cachedis1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Ctrla {
    #[inline]
    fn from(other: u16) -> Self {
         Ctrla(other)
    }
}

impl ::core::fmt::Display for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.autows() != 0 { try!(write!(f, " autows"))}
        if self.suspen() != 0 { try!(write!(f, " suspen"))}
        if self.wmode() != 0 { try!(write!(f, " wmode=0x{:x}", self.wmode()))}
        if self.prm() != 0 { try!(write!(f, " prm=0x{:x}", self.prm()))}
        if self.rws() != 0 { try!(write!(f, " rws=0x{:x}", self.rws()))}
        if self.ahbns0() != 0 { try!(write!(f, " ahbns0"))}
        if self.ahbns1() != 0 { try!(write!(f, " ahbns1"))}
        if self.cachedis0() != 0 { try!(write!(f, " cachedis0"))}
        if self.cachedis1() != 0 { try!(write!(f, " cachedis1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u16);
impl Ctrlb {
    #[doc="Command"]
    #[inline] pub fn cmd(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command Execution"]
    #[inline] pub fn cmdex(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if CMDEX != 0"]
    #[inline] pub fn test_cmdex(&self) -> bool {
        self.cmdex() != 0
    }

    #[doc="Sets the CMDEX field."]
    #[inline] pub fn set_cmdex<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Ctrlb {
    #[inline]
    fn from(other: u16) -> Self {
         Ctrlb(other)
    }
}

impl ::core::fmt::Display for Ctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        if self.cmdex() != 0 { try!(write!(f, " cmdex=0x{:x}", self.cmdex()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="NVM Parameter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
    #[doc="NVM Pages"]
    #[inline] pub fn nvmp(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if NVMP != 0"]
    #[inline] pub fn test_nvmp(&self) -> bool {
        self.nvmp() != 0
    }

    #[doc="Sets the NVMP field."]
    #[inline] pub fn set_nvmp<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Page Size"]
    #[inline] pub fn psz(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if PSZ != 0"]
    #[inline] pub fn test_psz(&self) -> bool {
        self.psz() != 0
    }

    #[doc="Sets the PSZ field."]
    #[inline] pub fn set_psz<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="SmartEEPROM Supported"]
    #[inline] pub fn see(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if SEE != 0"]
    #[inline] pub fn test_see(&self) -> bool {
        self.see() != 0
    }

    #[doc="Sets the SEE field."]
    #[inline] pub fn set_see<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Param {
    #[inline]
    fn from(other: u32) -> Self {
         Param(other)
    }
}

impl ::core::fmt::Display for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nvmp() != 0 { try!(write!(f, " nvmp=0x{:x}", self.nvmp()))}
        if self.psz() != 0 { try!(write!(f, " psz=0x{:x}", self.psz()))}
        if self.see() != 0 { try!(write!(f, " see"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u16);
impl Intenclr {
    #[doc="Command Done Interrupt Clear"]
    #[inline] pub fn done(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DONE != 0"]
    #[inline] pub fn test_done(&self) -> bool {
        self.done() != 0
    }

    #[doc="Sets the DONE field."]
    #[inline] pub fn set_done<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address Error"]
    #[inline] pub fn addre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ADDRE != 0"]
    #[inline] pub fn test_addre(&self) -> bool {
        self.addre() != 0
    }

    #[doc="Sets the ADDRE field."]
    #[inline] pub fn set_addre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Programming Error Interrupt Clear"]
    #[inline] pub fn proge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PROGE != 0"]
    #[inline] pub fn test_proge(&self) -> bool {
        self.proge() != 0
    }

    #[doc="Sets the PROGE field."]
    #[inline] pub fn set_proge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Lock Error Interrupt Clear"]
    #[inline] pub fn locke(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LOCKE != 0"]
    #[inline] pub fn test_locke(&self) -> bool {
        self.locke() != 0
    }

    #[doc="Sets the LOCKE field."]
    #[inline] pub fn set_locke<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ECC Single Error Interrupt Clear"]
    #[inline] pub fn eccse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ECCSE != 0"]
    #[inline] pub fn test_eccse(&self) -> bool {
        self.eccse() != 0
    }

    #[doc="Sets the ECCSE field."]
    #[inline] pub fn set_eccse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ECC Dual Error Interrupt Clear"]
    #[inline] pub fn eccde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ECCDE != 0"]
    #[inline] pub fn test_eccde(&self) -> bool {
        self.eccde() != 0
    }

    #[doc="Sets the ECCDE field."]
    #[inline] pub fn set_eccde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="NVM Error Interrupt Clear"]
    #[inline] pub fn nvme(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NVME != 0"]
    #[inline] pub fn test_nvme(&self) -> bool {
        self.nvme() != 0
    }

    #[doc="Sets the NVME field."]
    #[inline] pub fn set_nvme<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Suspended Write Or Erase Interrupt Clear"]
    #[inline] pub fn susp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Active SEES Full Interrupt Clear"]
    #[inline] pub fn seesfull(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SEESFULL != 0"]
    #[inline] pub fn test_seesfull(&self) -> bool {
        self.seesfull() != 0
    }

    #[doc="Sets the SEESFULL field."]
    #[inline] pub fn set_seesfull<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Active SEES Overflow Interrupt Clear"]
    #[inline] pub fn seesovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SEESOVF != 0"]
    #[inline] pub fn test_seesovf(&self) -> bool {
        self.seesovf() != 0
    }

    #[doc="Sets the SEESOVF field."]
    #[inline] pub fn set_seesovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SEE Write Completed Interrupt Clear"]
    #[inline] pub fn seewrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SEEWRC != 0"]
    #[inline] pub fn test_seewrc(&self) -> bool {
        self.seewrc() != 0
    }

    #[doc="Sets the SEEWRC field."]
    #[inline] pub fn set_seewrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u16> for Intenclr {
    #[inline]
    fn from(other: u16) -> Self {
         Intenclr(other)
    }
}

impl ::core::fmt::Display for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.done() != 0 { try!(write!(f, " done"))}
        if self.addre() != 0 { try!(write!(f, " addre"))}
        if self.proge() != 0 { try!(write!(f, " proge"))}
        if self.locke() != 0 { try!(write!(f, " locke"))}
        if self.eccse() != 0 { try!(write!(f, " eccse"))}
        if self.eccde() != 0 { try!(write!(f, " eccde"))}
        if self.nvme() != 0 { try!(write!(f, " nvme"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        if self.seesfull() != 0 { try!(write!(f, " seesfull"))}
        if self.seesovf() != 0 { try!(write!(f, " seesovf"))}
        if self.seewrc() != 0 { try!(write!(f, " seewrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u16);
impl Intenset {
    #[doc="Command Done Interrupt Enable"]
    #[inline] pub fn done(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DONE != 0"]
    #[inline] pub fn test_done(&self) -> bool {
        self.done() != 0
    }

    #[doc="Sets the DONE field."]
    #[inline] pub fn set_done<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address Error Interrupt Enable"]
    #[inline] pub fn addre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ADDRE != 0"]
    #[inline] pub fn test_addre(&self) -> bool {
        self.addre() != 0
    }

    #[doc="Sets the ADDRE field."]
    #[inline] pub fn set_addre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Programming Error Interrupt Enable"]
    #[inline] pub fn proge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PROGE != 0"]
    #[inline] pub fn test_proge(&self) -> bool {
        self.proge() != 0
    }

    #[doc="Sets the PROGE field."]
    #[inline] pub fn set_proge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Lock Error Interrupt Enable"]
    #[inline] pub fn locke(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LOCKE != 0"]
    #[inline] pub fn test_locke(&self) -> bool {
        self.locke() != 0
    }

    #[doc="Sets the LOCKE field."]
    #[inline] pub fn set_locke<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ECC Single Error Interrupt Enable"]
    #[inline] pub fn eccse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ECCSE != 0"]
    #[inline] pub fn test_eccse(&self) -> bool {
        self.eccse() != 0
    }

    #[doc="Sets the ECCSE field."]
    #[inline] pub fn set_eccse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ECC Dual Error Interrupt Enable"]
    #[inline] pub fn eccde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ECCDE != 0"]
    #[inline] pub fn test_eccde(&self) -> bool {
        self.eccde() != 0
    }

    #[doc="Sets the ECCDE field."]
    #[inline] pub fn set_eccde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="NVM Error Interrupt Enable"]
    #[inline] pub fn nvme(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NVME != 0"]
    #[inline] pub fn test_nvme(&self) -> bool {
        self.nvme() != 0
    }

    #[doc="Sets the NVME field."]
    #[inline] pub fn set_nvme<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Suspended Write Or Erase Interrupt Enable"]
    #[inline] pub fn susp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Active SEES Full Interrupt Enable"]
    #[inline] pub fn seesfull(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SEESFULL != 0"]
    #[inline] pub fn test_seesfull(&self) -> bool {
        self.seesfull() != 0
    }

    #[doc="Sets the SEESFULL field."]
    #[inline] pub fn set_seesfull<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Active SEES Overflow Interrupt Enable"]
    #[inline] pub fn seesovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SEESOVF != 0"]
    #[inline] pub fn test_seesovf(&self) -> bool {
        self.seesovf() != 0
    }

    #[doc="Sets the SEESOVF field."]
    #[inline] pub fn set_seesovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SEE Write Completed Interrupt Enable"]
    #[inline] pub fn seewrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SEEWRC != 0"]
    #[inline] pub fn test_seewrc(&self) -> bool {
        self.seewrc() != 0
    }

    #[doc="Sets the SEEWRC field."]
    #[inline] pub fn set_seewrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u16> for Intenset {
    #[inline]
    fn from(other: u16) -> Self {
         Intenset(other)
    }
}

impl ::core::fmt::Display for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.done() != 0 { try!(write!(f, " done"))}
        if self.addre() != 0 { try!(write!(f, " addre"))}
        if self.proge() != 0 { try!(write!(f, " proge"))}
        if self.locke() != 0 { try!(write!(f, " locke"))}
        if self.eccse() != 0 { try!(write!(f, " eccse"))}
        if self.eccde() != 0 { try!(write!(f, " eccde"))}
        if self.nvme() != 0 { try!(write!(f, " nvme"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        if self.seesfull() != 0 { try!(write!(f, " seesfull"))}
        if self.seesovf() != 0 { try!(write!(f, " seesovf"))}
        if self.seewrc() != 0 { try!(write!(f, " seewrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u16);
impl Intflag {
    #[doc="Command Done"]
    #[inline] pub fn done(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DONE != 0"]
    #[inline] pub fn test_done(&self) -> bool {
        self.done() != 0
    }

    #[doc="Sets the DONE field."]
    #[inline] pub fn set_done<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address Error"]
    #[inline] pub fn addre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ADDRE != 0"]
    #[inline] pub fn test_addre(&self) -> bool {
        self.addre() != 0
    }

    #[doc="Sets the ADDRE field."]
    #[inline] pub fn set_addre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Programming Error"]
    #[inline] pub fn proge(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PROGE != 0"]
    #[inline] pub fn test_proge(&self) -> bool {
        self.proge() != 0
    }

    #[doc="Sets the PROGE field."]
    #[inline] pub fn set_proge<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Lock Error"]
    #[inline] pub fn locke(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LOCKE != 0"]
    #[inline] pub fn test_locke(&self) -> bool {
        self.locke() != 0
    }

    #[doc="Sets the LOCKE field."]
    #[inline] pub fn set_locke<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ECC Single Error"]
    #[inline] pub fn eccse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ECCSE != 0"]
    #[inline] pub fn test_eccse(&self) -> bool {
        self.eccse() != 0
    }

    #[doc="Sets the ECCSE field."]
    #[inline] pub fn set_eccse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ECC Dual Error"]
    #[inline] pub fn eccde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ECCDE != 0"]
    #[inline] pub fn test_eccde(&self) -> bool {
        self.eccde() != 0
    }

    #[doc="Sets the ECCDE field."]
    #[inline] pub fn set_eccde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="NVM Error"]
    #[inline] pub fn nvme(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NVME != 0"]
    #[inline] pub fn test_nvme(&self) -> bool {
        self.nvme() != 0
    }

    #[doc="Sets the NVME field."]
    #[inline] pub fn set_nvme<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Suspended Write Or Erase Operation"]
    #[inline] pub fn susp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Active SEES Full"]
    #[inline] pub fn seesfull(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SEESFULL != 0"]
    #[inline] pub fn test_seesfull(&self) -> bool {
        self.seesfull() != 0
    }

    #[doc="Sets the SEESFULL field."]
    #[inline] pub fn set_seesfull<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Active SEES Overflow"]
    #[inline] pub fn seesovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SEESOVF != 0"]
    #[inline] pub fn test_seesovf(&self) -> bool {
        self.seesovf() != 0
    }

    #[doc="Sets the SEESOVF field."]
    #[inline] pub fn set_seesovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SEE Write Completed"]
    #[inline] pub fn seewrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SEEWRC != 0"]
    #[inline] pub fn test_seewrc(&self) -> bool {
        self.seewrc() != 0
    }

    #[doc="Sets the SEEWRC field."]
    #[inline] pub fn set_seewrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u16> for Intflag {
    #[inline]
    fn from(other: u16) -> Self {
         Intflag(other)
    }
}

impl ::core::fmt::Display for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.done() != 0 { try!(write!(f, " done"))}
        if self.addre() != 0 { try!(write!(f, " addre"))}
        if self.proge() != 0 { try!(write!(f, " proge"))}
        if self.locke() != 0 { try!(write!(f, " locke"))}
        if self.eccse() != 0 { try!(write!(f, " eccse"))}
        if self.eccde() != 0 { try!(write!(f, " eccde"))}
        if self.nvme() != 0 { try!(write!(f, " nvme"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        if self.seesfull() != 0 { try!(write!(f, " seesfull"))}
        if self.seesovf() != 0 { try!(write!(f, " seesovf"))}
        if self.seewrc() != 0 { try!(write!(f, " seewrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u16);
impl Status {
    #[doc="Ready to accept a command"]
    #[inline] pub fn ready(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if READY != 0"]
    #[inline] pub fn test_ready(&self) -> bool {
        self.ready() != 0
    }

    #[doc="Sets the READY field."]
    #[inline] pub fn set_ready<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Power Reduction Mode"]
    #[inline] pub fn prm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PRM != 0"]
    #[inline] pub fn test_prm(&self) -> bool {
        self.prm() != 0
    }

    #[doc="Sets the PRM field."]
    #[inline] pub fn set_prm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="NVM Page Buffer Active Loading"]
    #[inline] pub fn load(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LOAD != 0"]
    #[inline] pub fn test_load(&self) -> bool {
        self.load() != 0
    }

    #[doc="Sets the LOAD field."]
    #[inline] pub fn set_load<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="NVM Write Or Erase Operation Is Suspended"]
    #[inline] pub fn susp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="BANKA First"]
    #[inline] pub fn afirst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AFIRST != 0"]
    #[inline] pub fn test_afirst(&self) -> bool {
        self.afirst() != 0
    }

    #[doc="Sets the AFIRST field."]
    #[inline] pub fn set_afirst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Boot Loader Protection Disable"]
    #[inline] pub fn bpdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BPDIS != 0"]
    #[inline] pub fn test_bpdis(&self) -> bool {
        self.bpdis() != 0
    }

    #[doc="Sets the BPDIS field."]
    #[inline] pub fn set_bpdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Boot Loader Protection Size"]
    #[inline] pub fn bootprot(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if BOOTPROT != 0"]
    #[inline] pub fn test_bootprot(&self) -> bool {
        self.bootprot() != 0
    }

    #[doc="Sets the BOOTPROT field."]
    #[inline] pub fn set_bootprot<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Status {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.ready() != 0 { try!(write!(f, " ready"))}
        if self.prm() != 0 { try!(write!(f, " prm"))}
        if self.load() != 0 { try!(write!(f, " load"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        if self.afirst() != 0 { try!(write!(f, " afirst"))}
        if self.bpdis() != 0 { try!(write!(f, " bpdis"))}
        if self.bootprot() != 0 { try!(write!(f, " bootprot=0x{:x}", self.bootprot()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Addr(pub u32);
impl Addr {
    #[doc="NVM Address"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Addr {
    #[inline]
    fn from(other: u32) -> Self {
         Addr(other)
    }
}

impl ::core::fmt::Display for Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Lock Section"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Runlock(pub u32);
impl Runlock {
    #[doc="Region Un-Lock Bits"]
    #[inline] pub fn runlock(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if RUNLOCK != 0"]
    #[inline] pub fn test_runlock(&self) -> bool {
        self.runlock() != 0
    }

    #[doc="Sets the RUNLOCK field."]
    #[inline] pub fn set_runlock<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Runlock {
    #[inline]
    fn from(other: u32) -> Self {
         Runlock(other)
    }
}

impl ::core::fmt::Display for Runlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Runlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Page Buffer Load Data x"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pbldata(pub u32);
impl Pbldata {
    #[doc="Page Buffer Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pbldata {
    #[inline]
    fn from(other: u32) -> Self {
         Pbldata(other)
    }
}

impl ::core::fmt::Display for Pbldata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pbldata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ECC Error Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eccerr(pub u32);
impl Eccerr {
    #[doc="Error Address"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low Double-Word Error Type"]
    #[inline] pub fn typel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if TYPEL != 0"]
    #[inline] pub fn test_typel(&self) -> bool {
        self.typel() != 0
    }

    #[doc="Sets the TYPEL field."]
    #[inline] pub fn set_typel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="High Double-Word Error Type"]
    #[inline] pub fn typeh(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if TYPEH != 0"]
    #[inline] pub fn test_typeh(&self) -> bool {
        self.typeh() != 0
    }

    #[doc="Sets the TYPEH field."]
    #[inline] pub fn set_typeh<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Eccerr {
    #[inline]
    fn from(other: u32) -> Self {
         Eccerr(other)
    }
}

impl ::core::fmt::Display for Eccerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eccerr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.typel() != 0 { try!(write!(f, " typel=0x{:x}", self.typel()))}
        if self.typeh() != 0 { try!(write!(f, " typeh=0x{:x}", self.typeh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debugger ECC Read Disable"]
    #[inline] pub fn eccdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ECCDIS != 0"]
    #[inline] pub fn test_eccdis(&self) -> bool {
        self.eccdis() != 0
    }

    #[doc="Sets the ECCDIS field."]
    #[inline] pub fn set_eccdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Debugger ECC Error Tracking Mode"]
    #[inline] pub fn eccelog(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ECCELOG != 0"]
    #[inline] pub fn test_eccelog(&self) -> bool {
        self.eccelog() != 0
    }

    #[doc="Sets the ECCELOG field."]
    #[inline] pub fn set_eccelog<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Dbgctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Dbgctrl(other)
    }
}

impl ::core::fmt::Display for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eccdis() != 0 { try!(write!(f, " eccdis"))}
        if self.eccelog() != 0 { try!(write!(f, " eccelog"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SmartEEPROM Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Seecfg(pub u8);
impl Seecfg {
    #[doc="Write Mode"]
    #[inline] pub fn wmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if WMODE != 0"]
    #[inline] pub fn test_wmode(&self) -> bool {
        self.wmode() != 0
    }

    #[doc="Sets the WMODE field."]
    #[inline] pub fn set_wmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Automatic Page Reallocation Disable"]
    #[inline] pub fn aprdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if APRDIS != 0"]
    #[inline] pub fn test_aprdis(&self) -> bool {
        self.aprdis() != 0
    }

    #[doc="Sets the APRDIS field."]
    #[inline] pub fn set_aprdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Seecfg {
    #[inline]
    fn from(other: u8) -> Self {
         Seecfg(other)
    }
}

impl ::core::fmt::Display for Seecfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Seecfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wmode() != 0 { try!(write!(f, " wmode"))}
        if self.aprdis() != 0 { try!(write!(f, " aprdis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SmartEEPROM Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Seestat(pub u32);
impl Seestat {
    #[doc="Active SmartEEPROM Sector"]
    #[inline] pub fn asees(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ASEES != 0"]
    #[inline] pub fn test_asees(&self) -> bool {
        self.asees() != 0
    }

    #[doc="Sets the ASEES field."]
    #[inline] pub fn set_asees<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Page Buffer Loaded"]
    #[inline] pub fn load(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LOAD != 0"]
    #[inline] pub fn test_load(&self) -> bool {
        self.load() != 0
    }

    #[doc="Sets the LOAD field."]
    #[inline] pub fn set_load<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SmartEEPROM Write Access Is Locked"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="SmartEEPROM Write Access To Register Address Space Is Locked"]
    #[inline] pub fn rlock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RLOCK != 0"]
    #[inline] pub fn test_rlock(&self) -> bool {
        self.rlock() != 0
    }

    #[doc="Sets the RLOCK field."]
    #[inline] pub fn set_rlock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Blocks Number In a Sector"]
    #[inline] pub fn sblk(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if SBLK != 0"]
    #[inline] pub fn test_sblk(&self) -> bool {
        self.sblk() != 0
    }

    #[doc="Sets the SBLK field."]
    #[inline] pub fn set_sblk<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="SmartEEPROM Page Size"]
    #[inline] pub fn psz(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if PSZ != 0"]
    #[inline] pub fn test_psz(&self) -> bool {
        self.psz() != 0
    }

    #[doc="Sets the PSZ field."]
    #[inline] pub fn set_psz<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Seestat {
    #[inline]
    fn from(other: u32) -> Self {
         Seestat(other)
    }
}

impl ::core::fmt::Display for Seestat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Seestat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.asees() != 0 { try!(write!(f, " asees"))}
        if self.load() != 0 { try!(write!(f, " load"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.rlock() != 0 { try!(write!(f, " rlock"))}
        if self.sblk() != 0 { try!(write!(f, " sblk=0x{:x}", self.sblk()))}
        if self.psz() != 0 { try!(write!(f, " psz=0x{:x}", self.psz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

