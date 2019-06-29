::bobbin_mcu::periph!( OSCCTRL, Oscctrl, OSCCTRL_PERIPH, OscctrlPeriph, OSCCTRL_OWNED, OSCCTRL_REF_COUNT, 0x40001000, 0x00, 0x12);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="OSCCTRL Peripheral"]
pub struct OscctrlPeriph(pub usize); 

impl OscctrlPeriph {
    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::Register<Evctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Evctrl, 0x0)
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
        self.evctrl_reg().ptr()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        self.evctrl_reg().read()
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn write_evctrl(&self, value: Evctrl) -> &Self { 
        self.evctrl_reg().write(value);
        self
    }

    #[doc="Set the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().set(f);
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        self.evctrl_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x4)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x8)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0xc)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x10)
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

    #[doc="Get the XOSCCTRL Register."]
    #[inline] pub fn xoscctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Xoscctrl, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Xoscctrl, 0x14, 0x4)
    }

    #[doc="Get the *mut pointer for the XOSCCTRL register."]
    #[inline] pub fn xoscctrl_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Xoscctrl { 
        self.xoscctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the XOSCCTRL register."]
    #[inline] pub fn xoscctrl_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Xoscctrl { 
        self.xoscctrl_reg().ptr(index.into())
    }

    #[doc="Read the XOSCCTRL register."]
    #[inline] pub fn xoscctrl<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Xoscctrl { 
        self.xoscctrl_reg().read(index.into())
    }

    #[doc="Write the XOSCCTRL register."]
    #[inline] pub fn write_xoscctrl<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Xoscctrl) -> &Self {
        self.xoscctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the XOSCCTRL register."]
    #[inline] pub fn set_xoscctrl<I: Into<::bobbin_bits::R2>, F: FnOnce(Xoscctrl) -> Xoscctrl>(&self, index: I, f: F) -> &Self {
        self.xoscctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the XOSCCTRL register."]
    #[inline] pub fn with_xoscctrl<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Xoscctrl) -> Xoscctrl>(&self, index: I, f: F) -> &Self {
        self.xoscctrl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DFLLCTRLA Register."]
    #[inline] pub fn dfllctrla_reg(&self) -> ::bobbin_mcu::register::Register<Dfllctrla> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dfllctrla, 0x1c)
    }

    #[doc="Get the *mut pointer for the DFLLCTRLA register."]
    #[inline] pub fn dfllctrla_mut(&self) -> *mut Dfllctrla { 
        self.dfllctrla_reg().ptr()
    }

    #[doc="Get the *const pointer for the DFLLCTRLA register."]
    #[inline] pub fn dfllctrla_ptr(&self) -> *const Dfllctrla { 
        self.dfllctrla_reg().ptr()
    }

    #[doc="Read the DFLLCTRLA register."]
    #[inline] pub fn dfllctrla(&self) -> Dfllctrla { 
        self.dfllctrla_reg().read()
    }

    #[doc="Write the DFLLCTRLA register."]
    #[inline] pub fn write_dfllctrla(&self, value: Dfllctrla) -> &Self { 
        self.dfllctrla_reg().write(value);
        self
    }

    #[doc="Set the DFLLCTRLA register."]
    #[inline] pub fn set_dfllctrla<F: FnOnce(Dfllctrla) -> Dfllctrla>(&self, f: F) -> &Self {
        self.dfllctrla_reg().set(f);
        self
    }

    #[doc="Modify the DFLLCTRLA register."]
    #[inline] pub fn with_dfllctrla<F: FnOnce(Dfllctrla) -> Dfllctrla>(&self, f: F) -> &Self {
        self.dfllctrla_reg().with(f);
        self
    }

    #[doc="Get the DFLLCTRLB Register."]
    #[inline] pub fn dfllctrlb_reg(&self) -> ::bobbin_mcu::register::Register<Dfllctrlb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dfllctrlb, 0x20)
    }

    #[doc="Get the *mut pointer for the DFLLCTRLB register."]
    #[inline] pub fn dfllctrlb_mut(&self) -> *mut Dfllctrlb { 
        self.dfllctrlb_reg().ptr()
    }

    #[doc="Get the *const pointer for the DFLLCTRLB register."]
    #[inline] pub fn dfllctrlb_ptr(&self) -> *const Dfllctrlb { 
        self.dfllctrlb_reg().ptr()
    }

    #[doc="Read the DFLLCTRLB register."]
    #[inline] pub fn dfllctrlb(&self) -> Dfllctrlb { 
        self.dfllctrlb_reg().read()
    }

    #[doc="Write the DFLLCTRLB register."]
    #[inline] pub fn write_dfllctrlb(&self, value: Dfllctrlb) -> &Self { 
        self.dfllctrlb_reg().write(value);
        self
    }

    #[doc="Set the DFLLCTRLB register."]
    #[inline] pub fn set_dfllctrlb<F: FnOnce(Dfllctrlb) -> Dfllctrlb>(&self, f: F) -> &Self {
        self.dfllctrlb_reg().set(f);
        self
    }

    #[doc="Modify the DFLLCTRLB register."]
    #[inline] pub fn with_dfllctrlb<F: FnOnce(Dfllctrlb) -> Dfllctrlb>(&self, f: F) -> &Self {
        self.dfllctrlb_reg().with(f);
        self
    }

    #[doc="Get the DFLLVAL Register."]
    #[inline] pub fn dfllval_reg(&self) -> ::bobbin_mcu::register::Register<Dfllval> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dfllval, 0x24)
    }

    #[doc="Get the *mut pointer for the DFLLVAL register."]
    #[inline] pub fn dfllval_mut(&self) -> *mut Dfllval { 
        self.dfllval_reg().ptr()
    }

    #[doc="Get the *const pointer for the DFLLVAL register."]
    #[inline] pub fn dfllval_ptr(&self) -> *const Dfllval { 
        self.dfllval_reg().ptr()
    }

    #[doc="Read the DFLLVAL register."]
    #[inline] pub fn dfllval(&self) -> Dfllval { 
        self.dfllval_reg().read()
    }

    #[doc="Write the DFLLVAL register."]
    #[inline] pub fn write_dfllval(&self, value: Dfllval) -> &Self { 
        self.dfllval_reg().write(value);
        self
    }

    #[doc="Set the DFLLVAL register."]
    #[inline] pub fn set_dfllval<F: FnOnce(Dfllval) -> Dfllval>(&self, f: F) -> &Self {
        self.dfllval_reg().set(f);
        self
    }

    #[doc="Modify the DFLLVAL register."]
    #[inline] pub fn with_dfllval<F: FnOnce(Dfllval) -> Dfllval>(&self, f: F) -> &Self {
        self.dfllval_reg().with(f);
        self
    }

    #[doc="Get the DFLLMUL Register."]
    #[inline] pub fn dfllmul_reg(&self) -> ::bobbin_mcu::register::Register<Dfllmul> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dfllmul, 0x28)
    }

    #[doc="Get the *mut pointer for the DFLLMUL register."]
    #[inline] pub fn dfllmul_mut(&self) -> *mut Dfllmul { 
        self.dfllmul_reg().ptr()
    }

    #[doc="Get the *const pointer for the DFLLMUL register."]
    #[inline] pub fn dfllmul_ptr(&self) -> *const Dfllmul { 
        self.dfllmul_reg().ptr()
    }

    #[doc="Read the DFLLMUL register."]
    #[inline] pub fn dfllmul(&self) -> Dfllmul { 
        self.dfllmul_reg().read()
    }

    #[doc="Write the DFLLMUL register."]
    #[inline] pub fn write_dfllmul(&self, value: Dfllmul) -> &Self { 
        self.dfllmul_reg().write(value);
        self
    }

    #[doc="Set the DFLLMUL register."]
    #[inline] pub fn set_dfllmul<F: FnOnce(Dfllmul) -> Dfllmul>(&self, f: F) -> &Self {
        self.dfllmul_reg().set(f);
        self
    }

    #[doc="Modify the DFLLMUL register."]
    #[inline] pub fn with_dfllmul<F: FnOnce(Dfllmul) -> Dfllmul>(&self, f: F) -> &Self {
        self.dfllmul_reg().with(f);
        self
    }

    #[doc="Get the DFLLSYNC Register."]
    #[inline] pub fn dfllsync_reg(&self) -> ::bobbin_mcu::register::Register<Dfllsync> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dfllsync, 0x2c)
    }

    #[doc="Get the *mut pointer for the DFLLSYNC register."]
    #[inline] pub fn dfllsync_mut(&self) -> *mut Dfllsync { 
        self.dfllsync_reg().ptr()
    }

    #[doc="Get the *const pointer for the DFLLSYNC register."]
    #[inline] pub fn dfllsync_ptr(&self) -> *const Dfllsync { 
        self.dfllsync_reg().ptr()
    }

    #[doc="Read the DFLLSYNC register."]
    #[inline] pub fn dfllsync(&self) -> Dfllsync { 
        self.dfllsync_reg().read()
    }

    #[doc="Write the DFLLSYNC register."]
    #[inline] pub fn write_dfllsync(&self, value: Dfllsync) -> &Self { 
        self.dfllsync_reg().write(value);
        self
    }

    #[doc="Set the DFLLSYNC register."]
    #[inline] pub fn set_dfllsync<F: FnOnce(Dfllsync) -> Dfllsync>(&self, f: F) -> &Self {
        self.dfllsync_reg().set(f);
        self
    }

    #[doc="Modify the DFLLSYNC register."]
    #[inline] pub fn with_dfllsync<F: FnOnce(Dfllsync) -> Dfllsync>(&self, f: F) -> &Self {
        self.dfllsync_reg().with(f);
        self
    }

    #[doc="Get the DPLLCTRLA Register."]
    #[inline] pub fn dpllctrla_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dpllctrla, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dpllctrla, 0x30, 0x14)
    }

    #[doc="Get the *mut pointer for the DPLLCTRLA register."]
    #[inline] pub fn dpllctrla_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dpllctrla { 
        self.dpllctrla_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DPLLCTRLA register."]
    #[inline] pub fn dpllctrla_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dpllctrla { 
        self.dpllctrla_reg().ptr(index.into())
    }

    #[doc="Read the DPLLCTRLA register."]
    #[inline] pub fn dpllctrla<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dpllctrla { 
        self.dpllctrla_reg().read(index.into())
    }

    #[doc="Write the DPLLCTRLA register."]
    #[inline] pub fn write_dpllctrla<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dpllctrla) -> &Self {
        self.dpllctrla_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DPLLCTRLA register."]
    #[inline] pub fn set_dpllctrla<I: Into<::bobbin_bits::R2>, F: FnOnce(Dpllctrla) -> Dpllctrla>(&self, index: I, f: F) -> &Self {
        self.dpllctrla_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DPLLCTRLA register."]
    #[inline] pub fn with_dpllctrla<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dpllctrla) -> Dpllctrla>(&self, index: I, f: F) -> &Self {
        self.dpllctrla_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DPLLRATIO Register."]
    #[inline] pub fn dpllratio_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dpllratio, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dpllratio, 0x34, 0x14)
    }

    #[doc="Get the *mut pointer for the DPLLRATIO register."]
    #[inline] pub fn dpllratio_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dpllratio { 
        self.dpllratio_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DPLLRATIO register."]
    #[inline] pub fn dpllratio_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dpllratio { 
        self.dpllratio_reg().ptr(index.into())
    }

    #[doc="Read the DPLLRATIO register."]
    #[inline] pub fn dpllratio<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dpllratio { 
        self.dpllratio_reg().read(index.into())
    }

    #[doc="Write the DPLLRATIO register."]
    #[inline] pub fn write_dpllratio<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dpllratio) -> &Self {
        self.dpllratio_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DPLLRATIO register."]
    #[inline] pub fn set_dpllratio<I: Into<::bobbin_bits::R2>, F: FnOnce(Dpllratio) -> Dpllratio>(&self, index: I, f: F) -> &Self {
        self.dpllratio_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DPLLRATIO register."]
    #[inline] pub fn with_dpllratio<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dpllratio) -> Dpllratio>(&self, index: I, f: F) -> &Self {
        self.dpllratio_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DPLLCTRLB Register."]
    #[inline] pub fn dpllctrlb_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dpllctrlb, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dpllctrlb, 0x38, 0x14)
    }

    #[doc="Get the *mut pointer for the DPLLCTRLB register."]
    #[inline] pub fn dpllctrlb_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dpllctrlb { 
        self.dpllctrlb_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DPLLCTRLB register."]
    #[inline] pub fn dpllctrlb_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dpllctrlb { 
        self.dpllctrlb_reg().ptr(index.into())
    }

    #[doc="Read the DPLLCTRLB register."]
    #[inline] pub fn dpllctrlb<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dpllctrlb { 
        self.dpllctrlb_reg().read(index.into())
    }

    #[doc="Write the DPLLCTRLB register."]
    #[inline] pub fn write_dpllctrlb<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dpllctrlb) -> &Self {
        self.dpllctrlb_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DPLLCTRLB register."]
    #[inline] pub fn set_dpllctrlb<I: Into<::bobbin_bits::R2>, F: FnOnce(Dpllctrlb) -> Dpllctrlb>(&self, index: I, f: F) -> &Self {
        self.dpllctrlb_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DPLLCTRLB register."]
    #[inline] pub fn with_dpllctrlb<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dpllctrlb) -> Dpllctrlb>(&self, index: I, f: F) -> &Self {
        self.dpllctrlb_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DPLLSYNCBUSY Register."]
    #[inline] pub fn dpllsyncbusy_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dpllsyncbusy, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dpllsyncbusy, 0x3c, 0x14)
    }

    #[doc="Get the *mut pointer for the DPLLSYNCBUSY register."]
    #[inline] pub fn dpllsyncbusy_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dpllsyncbusy { 
        self.dpllsyncbusy_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DPLLSYNCBUSY register."]
    #[inline] pub fn dpllsyncbusy_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dpllsyncbusy { 
        self.dpllsyncbusy_reg().ptr(index.into())
    }

    #[doc="Read the DPLLSYNCBUSY register."]
    #[inline] pub fn dpllsyncbusy<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dpllsyncbusy { 
        self.dpllsyncbusy_reg().read(index.into())
    }

    #[doc="Get the DPLLSTATUS Register."]
    #[inline] pub fn dpllstatus_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dpllstatus, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dpllstatus, 0x40, 0x14)
    }

    #[doc="Get the *mut pointer for the DPLLSTATUS register."]
    #[inline] pub fn dpllstatus_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dpllstatus { 
        self.dpllstatus_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DPLLSTATUS register."]
    #[inline] pub fn dpllstatus_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dpllstatus { 
        self.dpllstatus_reg().ptr(index.into())
    }

    #[doc="Read the DPLLSTATUS register."]
    #[inline] pub fn dpllstatus<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dpllstatus { 
        self.dpllstatus_reg().read(index.into())
    }

}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u8);
impl Evctrl {
    #[doc="Clock 0 Failure Detector Event Output Enable"]
    #[inline] pub fn cfdeo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CFDEO0 != 0"]
    #[inline] pub fn test_cfdeo0(&self) -> bool {
        self.cfdeo0() != 0
    }

    #[doc="Sets the CFDEO0 field."]
    #[inline] pub fn set_cfdeo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock 1 Failure Detector Event Output Enable"]
    #[inline] pub fn cfdeo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CFDEO1 != 0"]
    #[inline] pub fn test_cfdeo1(&self) -> bool {
        self.cfdeo1() != 0
    }

    #[doc="Sets the CFDEO1 field."]
    #[inline] pub fn set_cfdeo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Evctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cfdeo0() != 0 { try!(write!(f, " cfdeo0"))}
        if self.cfdeo1() != 0 { try!(write!(f, " cfdeo1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc="XOSC 0 Ready Interrupt Enable"]
    #[inline] pub fn xoscrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSCRDY0 != 0"]
    #[inline] pub fn test_xoscrdy0(&self) -> bool {
        self.xoscrdy0() != 0
    }

    #[doc="Sets the XOSCRDY0 field."]
    #[inline] pub fn set_xoscrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC 1 Ready Interrupt Enable"]
    #[inline] pub fn xoscrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if XOSCRDY1 != 0"]
    #[inline] pub fn test_xoscrdy1(&self) -> bool {
        self.xoscrdy1() != 0
    }

    #[doc="Sets the XOSCRDY1 field."]
    #[inline] pub fn set_xoscrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XOSC 0 Clock Failure Detector Interrupt Enable"]
    #[inline] pub fn xoscfail0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XOSCFAIL0 != 0"]
    #[inline] pub fn test_xoscfail0(&self) -> bool {
        self.xoscfail0() != 0
    }

    #[doc="Sets the XOSCFAIL0 field."]
    #[inline] pub fn set_xoscfail0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="XOSC 1 Clock Failure Detector Interrupt Enable"]
    #[inline] pub fn xoscfail1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if XOSCFAIL1 != 0"]
    #[inline] pub fn test_xoscfail1(&self) -> bool {
        self.xoscfail1() != 0
    }

    #[doc="Sets the XOSCFAIL1 field."]
    #[inline] pub fn set_xoscfail1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DFLL Ready Interrupt Enable"]
    #[inline] pub fn dfllrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DFLLRDY != 0"]
    #[inline] pub fn test_dfllrdy(&self) -> bool {
        self.dfllrdy() != 0
    }

    #[doc="Sets the DFLLRDY field."]
    #[inline] pub fn set_dfllrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DFLL Out Of Bounds Interrupt Enable"]
    #[inline] pub fn dflloob(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DFLLOOB != 0"]
    #[inline] pub fn test_dflloob(&self) -> bool {
        self.dflloob() != 0
    }

    #[doc="Sets the DFLLOOB field."]
    #[inline] pub fn set_dflloob<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="DFLL Lock Fine Interrupt Enable"]
    #[inline] pub fn dflllckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DFLLLCKF != 0"]
    #[inline] pub fn test_dflllckf(&self) -> bool {
        self.dflllckf() != 0
    }

    #[doc="Sets the DFLLLCKF field."]
    #[inline] pub fn set_dflllckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="DFLL Lock Coarse Interrupt Enable"]
    #[inline] pub fn dflllckc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFLLLCKC != 0"]
    #[inline] pub fn test_dflllckc(&self) -> bool {
        self.dflllckc() != 0
    }

    #[doc="Sets the DFLLLCKC field."]
    #[inline] pub fn set_dflllckc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline] pub fn dfllrcs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DFLLRCS != 0"]
    #[inline] pub fn test_dfllrcs(&self) -> bool {
        self.dfllrcs() != 0
    }

    #[doc="Sets the DFLLRCS field."]
    #[inline] pub fn set_dfllrcs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DPLL0 Lock Rise Interrupt Enable"]
    #[inline] pub fn dpll0lckr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLL0LCKR != 0"]
    #[inline] pub fn test_dpll0lckr(&self) -> bool {
        self.dpll0lckr() != 0
    }

    #[doc="Sets the DPLL0LCKR field."]
    #[inline] pub fn set_dpll0lckr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DPLL0 Lock Fall Interrupt Enable"]
    #[inline] pub fn dpll0lckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLL0LCKF != 0"]
    #[inline] pub fn test_dpll0lckf(&self) -> bool {
        self.dpll0lckf() != 0
    }

    #[doc="Sets the DPLL0LCKF field."]
    #[inline] pub fn set_dpll0lckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="DPLL0 Lock Timeout Interrupt Enable"]
    #[inline] pub fn dpll0lto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DPLL0LTO != 0"]
    #[inline] pub fn test_dpll0lto(&self) -> bool {
        self.dpll0lto() != 0
    }

    #[doc="Sets the DPLL0LTO field."]
    #[inline] pub fn set_dpll0lto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline] pub fn dpll0ldrto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if DPLL0LDRTO != 0"]
    #[inline] pub fn test_dpll0ldrto(&self) -> bool {
        self.dpll0ldrto() != 0
    }

    #[doc="Sets the DPLL0LDRTO field."]
    #[inline] pub fn set_dpll0ldrto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DPLL1 Lock Rise Interrupt Enable"]
    #[inline] pub fn dpll1lckr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DPLL1LCKR != 0"]
    #[inline] pub fn test_dpll1lckr(&self) -> bool {
        self.dpll1lckr() != 0
    }

    #[doc="Sets the DPLL1LCKR field."]
    #[inline] pub fn set_dpll1lckr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DPLL1 Lock Fall Interrupt Enable"]
    #[inline] pub fn dpll1lckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DPLL1LCKF != 0"]
    #[inline] pub fn test_dpll1lckf(&self) -> bool {
        self.dpll1lckf() != 0
    }

    #[doc="Sets the DPLL1LCKF field."]
    #[inline] pub fn set_dpll1lckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DPLL1 Lock Timeout Interrupt Enable"]
    #[inline] pub fn dpll1lto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if DPLL1LTO != 0"]
    #[inline] pub fn test_dpll1lto(&self) -> bool {
        self.dpll1lto() != 0
    }

    #[doc="Sets the DPLL1LTO field."]
    #[inline] pub fn set_dpll1lto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline] pub fn dpll1ldrto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if DPLL1LDRTO != 0"]
    #[inline] pub fn test_dpll1ldrto(&self) -> bool {
        self.dpll1ldrto() != 0
    }

    #[doc="Sets the DPLL1LDRTO field."]
    #[inline] pub fn set_dpll1ldrto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Intenclr {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.xoscrdy0() != 0 { try!(write!(f, " xoscrdy0"))}
        if self.xoscrdy1() != 0 { try!(write!(f, " xoscrdy1"))}
        if self.xoscfail0() != 0 { try!(write!(f, " xoscfail0"))}
        if self.xoscfail1() != 0 { try!(write!(f, " xoscfail1"))}
        if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
        if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
        if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
        if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
        if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
        if self.dpll0lckr() != 0 { try!(write!(f, " dpll0lckr"))}
        if self.dpll0lckf() != 0 { try!(write!(f, " dpll0lckf"))}
        if self.dpll0lto() != 0 { try!(write!(f, " dpll0lto"))}
        if self.dpll0ldrto() != 0 { try!(write!(f, " dpll0ldrto"))}
        if self.dpll1lckr() != 0 { try!(write!(f, " dpll1lckr"))}
        if self.dpll1lckf() != 0 { try!(write!(f, " dpll1lckf"))}
        if self.dpll1lto() != 0 { try!(write!(f, " dpll1lto"))}
        if self.dpll1ldrto() != 0 { try!(write!(f, " dpll1ldrto"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc="XOSC 0 Ready Interrupt Enable"]
    #[inline] pub fn xoscrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSCRDY0 != 0"]
    #[inline] pub fn test_xoscrdy0(&self) -> bool {
        self.xoscrdy0() != 0
    }

    #[doc="Sets the XOSCRDY0 field."]
    #[inline] pub fn set_xoscrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC 1 Ready Interrupt Enable"]
    #[inline] pub fn xoscrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if XOSCRDY1 != 0"]
    #[inline] pub fn test_xoscrdy1(&self) -> bool {
        self.xoscrdy1() != 0
    }

    #[doc="Sets the XOSCRDY1 field."]
    #[inline] pub fn set_xoscrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XOSC 0 Clock Failure Detector Interrupt Enable"]
    #[inline] pub fn xoscfail0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XOSCFAIL0 != 0"]
    #[inline] pub fn test_xoscfail0(&self) -> bool {
        self.xoscfail0() != 0
    }

    #[doc="Sets the XOSCFAIL0 field."]
    #[inline] pub fn set_xoscfail0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="XOSC 1 Clock Failure Detector Interrupt Enable"]
    #[inline] pub fn xoscfail1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if XOSCFAIL1 != 0"]
    #[inline] pub fn test_xoscfail1(&self) -> bool {
        self.xoscfail1() != 0
    }

    #[doc="Sets the XOSCFAIL1 field."]
    #[inline] pub fn set_xoscfail1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DFLL Ready Interrupt Enable"]
    #[inline] pub fn dfllrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DFLLRDY != 0"]
    #[inline] pub fn test_dfllrdy(&self) -> bool {
        self.dfllrdy() != 0
    }

    #[doc="Sets the DFLLRDY field."]
    #[inline] pub fn set_dfllrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DFLL Out Of Bounds Interrupt Enable"]
    #[inline] pub fn dflloob(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DFLLOOB != 0"]
    #[inline] pub fn test_dflloob(&self) -> bool {
        self.dflloob() != 0
    }

    #[doc="Sets the DFLLOOB field."]
    #[inline] pub fn set_dflloob<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="DFLL Lock Fine Interrupt Enable"]
    #[inline] pub fn dflllckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DFLLLCKF != 0"]
    #[inline] pub fn test_dflllckf(&self) -> bool {
        self.dflllckf() != 0
    }

    #[doc="Sets the DFLLLCKF field."]
    #[inline] pub fn set_dflllckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="DFLL Lock Coarse Interrupt Enable"]
    #[inline] pub fn dflllckc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFLLLCKC != 0"]
    #[inline] pub fn test_dflllckc(&self) -> bool {
        self.dflllckc() != 0
    }

    #[doc="Sets the DFLLLCKC field."]
    #[inline] pub fn set_dflllckc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline] pub fn dfllrcs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DFLLRCS != 0"]
    #[inline] pub fn test_dfllrcs(&self) -> bool {
        self.dfllrcs() != 0
    }

    #[doc="Sets the DFLLRCS field."]
    #[inline] pub fn set_dfllrcs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DPLL0 Lock Rise Interrupt Enable"]
    #[inline] pub fn dpll0lckr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLL0LCKR != 0"]
    #[inline] pub fn test_dpll0lckr(&self) -> bool {
        self.dpll0lckr() != 0
    }

    #[doc="Sets the DPLL0LCKR field."]
    #[inline] pub fn set_dpll0lckr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DPLL0 Lock Fall Interrupt Enable"]
    #[inline] pub fn dpll0lckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLL0LCKF != 0"]
    #[inline] pub fn test_dpll0lckf(&self) -> bool {
        self.dpll0lckf() != 0
    }

    #[doc="Sets the DPLL0LCKF field."]
    #[inline] pub fn set_dpll0lckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="DPLL0 Lock Timeout Interrupt Enable"]
    #[inline] pub fn dpll0lto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DPLL0LTO != 0"]
    #[inline] pub fn test_dpll0lto(&self) -> bool {
        self.dpll0lto() != 0
    }

    #[doc="Sets the DPLL0LTO field."]
    #[inline] pub fn set_dpll0lto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline] pub fn dpll0ldrto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if DPLL0LDRTO != 0"]
    #[inline] pub fn test_dpll0ldrto(&self) -> bool {
        self.dpll0ldrto() != 0
    }

    #[doc="Sets the DPLL0LDRTO field."]
    #[inline] pub fn set_dpll0ldrto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DPLL1 Lock Rise Interrupt Enable"]
    #[inline] pub fn dpll1lckr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DPLL1LCKR != 0"]
    #[inline] pub fn test_dpll1lckr(&self) -> bool {
        self.dpll1lckr() != 0
    }

    #[doc="Sets the DPLL1LCKR field."]
    #[inline] pub fn set_dpll1lckr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DPLL1 Lock Fall Interrupt Enable"]
    #[inline] pub fn dpll1lckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DPLL1LCKF != 0"]
    #[inline] pub fn test_dpll1lckf(&self) -> bool {
        self.dpll1lckf() != 0
    }

    #[doc="Sets the DPLL1LCKF field."]
    #[inline] pub fn set_dpll1lckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DPLL1 Lock Timeout Interrupt Enable"]
    #[inline] pub fn dpll1lto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if DPLL1LTO != 0"]
    #[inline] pub fn test_dpll1lto(&self) -> bool {
        self.dpll1lto() != 0
    }

    #[doc="Sets the DPLL1LTO field."]
    #[inline] pub fn set_dpll1lto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline] pub fn dpll1ldrto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if DPLL1LDRTO != 0"]
    #[inline] pub fn test_dpll1ldrto(&self) -> bool {
        self.dpll1ldrto() != 0
    }

    #[doc="Sets the DPLL1LDRTO field."]
    #[inline] pub fn set_dpll1ldrto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Intenset {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.xoscrdy0() != 0 { try!(write!(f, " xoscrdy0"))}
        if self.xoscrdy1() != 0 { try!(write!(f, " xoscrdy1"))}
        if self.xoscfail0() != 0 { try!(write!(f, " xoscfail0"))}
        if self.xoscfail1() != 0 { try!(write!(f, " xoscfail1"))}
        if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
        if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
        if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
        if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
        if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
        if self.dpll0lckr() != 0 { try!(write!(f, " dpll0lckr"))}
        if self.dpll0lckf() != 0 { try!(write!(f, " dpll0lckf"))}
        if self.dpll0lto() != 0 { try!(write!(f, " dpll0lto"))}
        if self.dpll0ldrto() != 0 { try!(write!(f, " dpll0ldrto"))}
        if self.dpll1lckr() != 0 { try!(write!(f, " dpll1lckr"))}
        if self.dpll1lckf() != 0 { try!(write!(f, " dpll1lckf"))}
        if self.dpll1lto() != 0 { try!(write!(f, " dpll1lto"))}
        if self.dpll1ldrto() != 0 { try!(write!(f, " dpll1ldrto"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u32);
impl Intflag {
    #[doc="XOSC 0 Ready"]
    #[inline] pub fn xoscrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSCRDY0 != 0"]
    #[inline] pub fn test_xoscrdy0(&self) -> bool {
        self.xoscrdy0() != 0
    }

    #[doc="Sets the XOSCRDY0 field."]
    #[inline] pub fn set_xoscrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC 1 Ready"]
    #[inline] pub fn xoscrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if XOSCRDY1 != 0"]
    #[inline] pub fn test_xoscrdy1(&self) -> bool {
        self.xoscrdy1() != 0
    }

    #[doc="Sets the XOSCRDY1 field."]
    #[inline] pub fn set_xoscrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XOSC 0 Clock Failure Detector"]
    #[inline] pub fn xoscfail0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XOSCFAIL0 != 0"]
    #[inline] pub fn test_xoscfail0(&self) -> bool {
        self.xoscfail0() != 0
    }

    #[doc="Sets the XOSCFAIL0 field."]
    #[inline] pub fn set_xoscfail0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="XOSC 1 Clock Failure Detector"]
    #[inline] pub fn xoscfail1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if XOSCFAIL1 != 0"]
    #[inline] pub fn test_xoscfail1(&self) -> bool {
        self.xoscfail1() != 0
    }

    #[doc="Sets the XOSCFAIL1 field."]
    #[inline] pub fn set_xoscfail1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DFLL Ready"]
    #[inline] pub fn dfllrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DFLLRDY != 0"]
    #[inline] pub fn test_dfllrdy(&self) -> bool {
        self.dfllrdy() != 0
    }

    #[doc="Sets the DFLLRDY field."]
    #[inline] pub fn set_dfllrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DFLL Out Of Bounds"]
    #[inline] pub fn dflloob(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DFLLOOB != 0"]
    #[inline] pub fn test_dflloob(&self) -> bool {
        self.dflloob() != 0
    }

    #[doc="Sets the DFLLOOB field."]
    #[inline] pub fn set_dflloob<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="DFLL Lock Fine"]
    #[inline] pub fn dflllckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DFLLLCKF != 0"]
    #[inline] pub fn test_dflllckf(&self) -> bool {
        self.dflllckf() != 0
    }

    #[doc="Sets the DFLLLCKF field."]
    #[inline] pub fn set_dflllckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="DFLL Lock Coarse"]
    #[inline] pub fn dflllckc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFLLLCKC != 0"]
    #[inline] pub fn test_dflllckc(&self) -> bool {
        self.dflllckc() != 0
    }

    #[doc="Sets the DFLLLCKC field."]
    #[inline] pub fn set_dflllckc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="DFLL Reference Clock Stopped"]
    #[inline] pub fn dfllrcs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DFLLRCS != 0"]
    #[inline] pub fn test_dfllrcs(&self) -> bool {
        self.dfllrcs() != 0
    }

    #[doc="Sets the DFLLRCS field."]
    #[inline] pub fn set_dfllrcs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DPLL0 Lock Rise"]
    #[inline] pub fn dpll0lckr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLL0LCKR != 0"]
    #[inline] pub fn test_dpll0lckr(&self) -> bool {
        self.dpll0lckr() != 0
    }

    #[doc="Sets the DPLL0LCKR field."]
    #[inline] pub fn set_dpll0lckr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DPLL0 Lock Fall"]
    #[inline] pub fn dpll0lckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLL0LCKF != 0"]
    #[inline] pub fn test_dpll0lckf(&self) -> bool {
        self.dpll0lckf() != 0
    }

    #[doc="Sets the DPLL0LCKF field."]
    #[inline] pub fn set_dpll0lckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="DPLL0 Lock Timeout"]
    #[inline] pub fn dpll0lto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DPLL0LTO != 0"]
    #[inline] pub fn test_dpll0lto(&self) -> bool {
        self.dpll0lto() != 0
    }

    #[doc="Sets the DPLL0LTO field."]
    #[inline] pub fn set_dpll0lto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="DPLL0 Loop Divider Ratio Update Complete"]
    #[inline] pub fn dpll0ldrto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if DPLL0LDRTO != 0"]
    #[inline] pub fn test_dpll0ldrto(&self) -> bool {
        self.dpll0ldrto() != 0
    }

    #[doc="Sets the DPLL0LDRTO field."]
    #[inline] pub fn set_dpll0ldrto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DPLL1 Lock Rise"]
    #[inline] pub fn dpll1lckr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DPLL1LCKR != 0"]
    #[inline] pub fn test_dpll1lckr(&self) -> bool {
        self.dpll1lckr() != 0
    }

    #[doc="Sets the DPLL1LCKR field."]
    #[inline] pub fn set_dpll1lckr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DPLL1 Lock Fall"]
    #[inline] pub fn dpll1lckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DPLL1LCKF != 0"]
    #[inline] pub fn test_dpll1lckf(&self) -> bool {
        self.dpll1lckf() != 0
    }

    #[doc="Sets the DPLL1LCKF field."]
    #[inline] pub fn set_dpll1lckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DPLL1 Lock Timeout"]
    #[inline] pub fn dpll1lto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if DPLL1LTO != 0"]
    #[inline] pub fn test_dpll1lto(&self) -> bool {
        self.dpll1lto() != 0
    }

    #[doc="Sets the DPLL1LTO field."]
    #[inline] pub fn set_dpll1lto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="DPLL1 Loop Divider Ratio Update Complete"]
    #[inline] pub fn dpll1ldrto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if DPLL1LDRTO != 0"]
    #[inline] pub fn test_dpll1ldrto(&self) -> bool {
        self.dpll1ldrto() != 0
    }

    #[doc="Sets the DPLL1LDRTO field."]
    #[inline] pub fn set_dpll1ldrto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Intflag {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.xoscrdy0() != 0 { try!(write!(f, " xoscrdy0"))}
        if self.xoscrdy1() != 0 { try!(write!(f, " xoscrdy1"))}
        if self.xoscfail0() != 0 { try!(write!(f, " xoscfail0"))}
        if self.xoscfail1() != 0 { try!(write!(f, " xoscfail1"))}
        if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
        if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
        if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
        if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
        if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
        if self.dpll0lckr() != 0 { try!(write!(f, " dpll0lckr"))}
        if self.dpll0lckf() != 0 { try!(write!(f, " dpll0lckf"))}
        if self.dpll0lto() != 0 { try!(write!(f, " dpll0lto"))}
        if self.dpll0ldrto() != 0 { try!(write!(f, " dpll0ldrto"))}
        if self.dpll1lckr() != 0 { try!(write!(f, " dpll1lckr"))}
        if self.dpll1lckf() != 0 { try!(write!(f, " dpll1lckf"))}
        if self.dpll1lto() != 0 { try!(write!(f, " dpll1lto"))}
        if self.dpll1ldrto() != 0 { try!(write!(f, " dpll1ldrto"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="XOSC 0 Ready"]
    #[inline] pub fn xoscrdy0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSCRDY0 != 0"]
    #[inline] pub fn test_xoscrdy0(&self) -> bool {
        self.xoscrdy0() != 0
    }

    #[doc="Sets the XOSCRDY0 field."]
    #[inline] pub fn set_xoscrdy0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC 1 Ready"]
    #[inline] pub fn xoscrdy1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if XOSCRDY1 != 0"]
    #[inline] pub fn test_xoscrdy1(&self) -> bool {
        self.xoscrdy1() != 0
    }

    #[doc="Sets the XOSCRDY1 field."]
    #[inline] pub fn set_xoscrdy1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XOSC 0 Clock Failure Detector"]
    #[inline] pub fn xoscfail0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XOSCFAIL0 != 0"]
    #[inline] pub fn test_xoscfail0(&self) -> bool {
        self.xoscfail0() != 0
    }

    #[doc="Sets the XOSCFAIL0 field."]
    #[inline] pub fn set_xoscfail0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="XOSC 1 Clock Failure Detector"]
    #[inline] pub fn xoscfail1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if XOSCFAIL1 != 0"]
    #[inline] pub fn test_xoscfail1(&self) -> bool {
        self.xoscfail1() != 0
    }

    #[doc="Sets the XOSCFAIL1 field."]
    #[inline] pub fn set_xoscfail1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="XOSC 0 Clock Switch"]
    #[inline] pub fn xosccksw0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if XOSCCKSW0 != 0"]
    #[inline] pub fn test_xosccksw0(&self) -> bool {
        self.xosccksw0() != 0
    }

    #[doc="Sets the XOSCCKSW0 field."]
    #[inline] pub fn set_xosccksw0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="XOSC 1 Clock Switch"]
    #[inline] pub fn xosccksw1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if XOSCCKSW1 != 0"]
    #[inline] pub fn test_xosccksw1(&self) -> bool {
        self.xosccksw1() != 0
    }

    #[doc="Sets the XOSCCKSW1 field."]
    #[inline] pub fn set_xosccksw1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="DFLL Ready"]
    #[inline] pub fn dfllrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DFLLRDY != 0"]
    #[inline] pub fn test_dfllrdy(&self) -> bool {
        self.dfllrdy() != 0
    }

    #[doc="Sets the DFLLRDY field."]
    #[inline] pub fn set_dfllrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DFLL Out Of Bounds"]
    #[inline] pub fn dflloob(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DFLLOOB != 0"]
    #[inline] pub fn test_dflloob(&self) -> bool {
        self.dflloob() != 0
    }

    #[doc="Sets the DFLLOOB field."]
    #[inline] pub fn set_dflloob<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="DFLL Lock Fine"]
    #[inline] pub fn dflllckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DFLLLCKF != 0"]
    #[inline] pub fn test_dflllckf(&self) -> bool {
        self.dflllckf() != 0
    }

    #[doc="Sets the DFLLLCKF field."]
    #[inline] pub fn set_dflllckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="DFLL Lock Coarse"]
    #[inline] pub fn dflllckc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFLLLCKC != 0"]
    #[inline] pub fn test_dflllckc(&self) -> bool {
        self.dflllckc() != 0
    }

    #[doc="Sets the DFLLLCKC field."]
    #[inline] pub fn set_dflllckc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="DFLL Reference Clock Stopped"]
    #[inline] pub fn dfllrcs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DFLLRCS != 0"]
    #[inline] pub fn test_dfllrcs(&self) -> bool {
        self.dfllrcs() != 0
    }

    #[doc="Sets the DFLLRCS field."]
    #[inline] pub fn set_dfllrcs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DPLL0 Lock Rise"]
    #[inline] pub fn dpll0lckr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLL0LCKR != 0"]
    #[inline] pub fn test_dpll0lckr(&self) -> bool {
        self.dpll0lckr() != 0
    }

    #[doc="Sets the DPLL0LCKR field."]
    #[inline] pub fn set_dpll0lckr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DPLL0 Lock Fall"]
    #[inline] pub fn dpll0lckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLL0LCKF != 0"]
    #[inline] pub fn test_dpll0lckf(&self) -> bool {
        self.dpll0lckf() != 0
    }

    #[doc="Sets the DPLL0LCKF field."]
    #[inline] pub fn set_dpll0lckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="DPLL0 Timeout"]
    #[inline] pub fn dpll0to(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DPLL0TO != 0"]
    #[inline] pub fn test_dpll0to(&self) -> bool {
        self.dpll0to() != 0
    }

    #[doc="Sets the DPLL0TO field."]
    #[inline] pub fn set_dpll0to<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="DPLL0 Loop Divider Ratio Update Complete"]
    #[inline] pub fn dpll0ldrto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if DPLL0LDRTO != 0"]
    #[inline] pub fn test_dpll0ldrto(&self) -> bool {
        self.dpll0ldrto() != 0
    }

    #[doc="Sets the DPLL0LDRTO field."]
    #[inline] pub fn set_dpll0ldrto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DPLL1 Lock Rise"]
    #[inline] pub fn dpll1lckr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DPLL1LCKR != 0"]
    #[inline] pub fn test_dpll1lckr(&self) -> bool {
        self.dpll1lckr() != 0
    }

    #[doc="Sets the DPLL1LCKR field."]
    #[inline] pub fn set_dpll1lckr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DPLL1 Lock Fall"]
    #[inline] pub fn dpll1lckf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DPLL1LCKF != 0"]
    #[inline] pub fn test_dpll1lckf(&self) -> bool {
        self.dpll1lckf() != 0
    }

    #[doc="Sets the DPLL1LCKF field."]
    #[inline] pub fn set_dpll1lckf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="DPLL1 Timeout"]
    #[inline] pub fn dpll1to(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if DPLL1TO != 0"]
    #[inline] pub fn test_dpll1to(&self) -> bool {
        self.dpll1to() != 0
    }

    #[doc="Sets the DPLL1TO field."]
    #[inline] pub fn set_dpll1to<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="DPLL1 Loop Divider Ratio Update Complete"]
    #[inline] pub fn dpll1ldrto(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if DPLL1LDRTO != 0"]
    #[inline] pub fn test_dpll1ldrto(&self) -> bool {
        self.dpll1ldrto() != 0
    }

    #[doc="Sets the DPLL1LDRTO field."]
    #[inline] pub fn set_dpll1ldrto<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
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
        if self.xoscrdy0() != 0 { try!(write!(f, " xoscrdy0"))}
        if self.xoscrdy1() != 0 { try!(write!(f, " xoscrdy1"))}
        if self.xoscfail0() != 0 { try!(write!(f, " xoscfail0"))}
        if self.xoscfail1() != 0 { try!(write!(f, " xoscfail1"))}
        if self.xosccksw0() != 0 { try!(write!(f, " xosccksw0"))}
        if self.xosccksw1() != 0 { try!(write!(f, " xosccksw1"))}
        if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
        if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
        if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
        if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
        if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
        if self.dpll0lckr() != 0 { try!(write!(f, " dpll0lckr"))}
        if self.dpll0lckf() != 0 { try!(write!(f, " dpll0lckf"))}
        if self.dpll0to() != 0 { try!(write!(f, " dpll0to"))}
        if self.dpll0ldrto() != 0 { try!(write!(f, " dpll0ldrto"))}
        if self.dpll1lckr() != 0 { try!(write!(f, " dpll1lckr"))}
        if self.dpll1lckf() != 0 { try!(write!(f, " dpll1lckf"))}
        if self.dpll1to() != 0 { try!(write!(f, " dpll1to"))}
        if self.dpll1ldrto() != 0 { try!(write!(f, " dpll1ldrto"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Multipurpose Crystal Oscillator Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Xoscctrl(pub u32);
impl Xoscctrl {
    #[doc="Oscillator Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Crystal Oscillator Enable"]
    #[inline] pub fn xtalen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XTALEN != 0"]
    #[inline] pub fn test_xtalen(&self) -> bool {
        self.xtalen() != 0
    }

    #[doc="Sets the XTALEN field."]
    #[inline] pub fn set_xtalen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="On Demand Control"]
    #[inline] pub fn ondemand(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Low Buffer Gain Enable"]
    #[inline] pub fn lowbufgain(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LOWBUFGAIN != 0"]
    #[inline] pub fn test_lowbufgain(&self) -> bool {
        self.lowbufgain() != 0
    }

    #[doc="Sets the LOWBUFGAIN field."]
    #[inline] pub fn set_lowbufgain<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Oscillator Current Reference"]
    #[inline] pub fn iptat(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if IPTAT != 0"]
    #[inline] pub fn test_iptat(&self) -> bool {
        self.iptat() != 0
    }

    #[doc="Sets the IPTAT field."]
    #[inline] pub fn set_iptat<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Oscillator Current Multiplier"]
    #[inline] pub fn imult(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0xf) as u8) } // [14:11]
    }

    #[doc="Returns true if IMULT != 0"]
    #[inline] pub fn test_imult(&self) -> bool {
        self.imult() != 0
    }

    #[doc="Sets the IMULT field."]
    #[inline] pub fn set_imult<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Automatic Loop Control Enable"]
    #[inline] pub fn enalc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ENALC != 0"]
    #[inline] pub fn test_enalc(&self) -> bool {
        self.enalc() != 0
    }

    #[doc="Sets the ENALC field."]
    #[inline] pub fn set_enalc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Clock Failure Detector Enable"]
    #[inline] pub fn cfden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CFDEN != 0"]
    #[inline] pub fn test_cfden(&self) -> bool {
        self.cfden() != 0
    }

    #[doc="Sets the CFDEN field."]
    #[inline] pub fn set_cfden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Xosc Clock Switch Enable"]
    #[inline] pub fn swben(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SWBEN != 0"]
    #[inline] pub fn test_swben(&self) -> bool {
        self.swben() != 0
    }

    #[doc="Sets the SWBEN field."]
    #[inline] pub fn set_swben<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Start-Up Time"]
    #[inline] pub fn startup(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if STARTUP != 0"]
    #[inline] pub fn test_startup(&self) -> bool {
        self.startup() != 0
    }

    #[doc="Sets the STARTUP field."]
    #[inline] pub fn set_startup<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Clock Failure Detector Prescaler"]
    #[inline] pub fn cfdpresc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if CFDPRESC != 0"]
    #[inline] pub fn test_cfdpresc(&self) -> bool {
        self.cfdpresc() != 0
    }

    #[doc="Sets the CFDPRESC field."]
    #[inline] pub fn set_cfdpresc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Xoscctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Xoscctrl(other)
    }
}

impl ::core::fmt::Display for Xoscctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Xoscctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.xtalen() != 0 { try!(write!(f, " xtalen"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        if self.lowbufgain() != 0 { try!(write!(f, " lowbufgain"))}
        if self.iptat() != 0 { try!(write!(f, " iptat=0x{:x}", self.iptat()))}
        if self.imult() != 0 { try!(write!(f, " imult=0x{:x}", self.imult()))}
        if self.enalc() != 0 { try!(write!(f, " enalc"))}
        if self.cfden() != 0 { try!(write!(f, " cfden"))}
        if self.swben() != 0 { try!(write!(f, " swben"))}
        if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
        if self.cfdpresc() != 0 { try!(write!(f, " cfdpresc=0x{:x}", self.cfdpresc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DFLL48M Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfllctrla(pub u8);
impl Dfllctrla {
    #[doc="DFLL Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="On Demand Control"]
    #[inline] pub fn ondemand(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Dfllctrla {
    #[inline]
    fn from(other: u8) -> Self {
         Dfllctrla(other)
    }
}

impl ::core::fmt::Display for Dfllctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfllctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DFLL48M Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfllctrlb(pub u8);
impl Dfllctrlb {
    #[doc="Operating Mode Selection"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Stable DFLL Frequency"]
    #[inline] pub fn stable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if STABLE != 0"]
    #[inline] pub fn test_stable(&self) -> bool {
        self.stable() != 0
    }

    #[doc="Sets the STABLE field."]
    #[inline] pub fn set_stable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Lose Lock After Wake"]
    #[inline] pub fn llaw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LLAW != 0"]
    #[inline] pub fn test_llaw(&self) -> bool {
        self.llaw() != 0
    }

    #[doc="Sets the LLAW field."]
    #[inline] pub fn set_llaw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="USB Clock Recovery Mode"]
    #[inline] pub fn usbcrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if USBCRM != 0"]
    #[inline] pub fn test_usbcrm(&self) -> bool {
        self.usbcrm() != 0
    }

    #[doc="Sets the USBCRM field."]
    #[inline] pub fn set_usbcrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Chill Cycle Disable"]
    #[inline] pub fn ccdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CCDIS != 0"]
    #[inline] pub fn test_ccdis(&self) -> bool {
        self.ccdis() != 0
    }

    #[doc="Sets the CCDIS field."]
    #[inline] pub fn set_ccdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Quick Lock Disable"]
    #[inline] pub fn qldis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if QLDIS != 0"]
    #[inline] pub fn test_qldis(&self) -> bool {
        self.qldis() != 0
    }

    #[doc="Sets the QLDIS field."]
    #[inline] pub fn set_qldis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Bypass Coarse Lock"]
    #[inline] pub fn bplckc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BPLCKC != 0"]
    #[inline] pub fn test_bplckc(&self) -> bool {
        self.bplckc() != 0
    }

    #[doc="Sets the BPLCKC field."]
    #[inline] pub fn set_bplckc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Wait Lock"]
    #[inline] pub fn waitlock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if WAITLOCK != 0"]
    #[inline] pub fn test_waitlock(&self) -> bool {
        self.waitlock() != 0
    }

    #[doc="Sets the WAITLOCK field."]
    #[inline] pub fn set_waitlock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Dfllctrlb {
    #[inline]
    fn from(other: u8) -> Self {
         Dfllctrlb(other)
    }
}

impl ::core::fmt::Display for Dfllctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfllctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mode() != 0 { try!(write!(f, " mode"))}
        if self.stable() != 0 { try!(write!(f, " stable"))}
        if self.llaw() != 0 { try!(write!(f, " llaw"))}
        if self.usbcrm() != 0 { try!(write!(f, " usbcrm"))}
        if self.ccdis() != 0 { try!(write!(f, " ccdis"))}
        if self.qldis() != 0 { try!(write!(f, " qldis"))}
        if self.bplckc() != 0 { try!(write!(f, " bplckc"))}
        if self.waitlock() != 0 { try!(write!(f, " waitlock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DFLL48M Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfllval(pub u32);
impl Dfllval {
    #[doc="Fine Value"]
    #[inline] pub fn fine(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FINE != 0"]
    #[inline] pub fn test_fine(&self) -> bool {
        self.fine() != 0
    }

    #[doc="Sets the FINE field."]
    #[inline] pub fn set_fine<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Coarse Value"]
    #[inline] pub fn coarse(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3f) as u8) } // [15:10]
    }

    #[doc="Returns true if COARSE != 0"]
    #[inline] pub fn test_coarse(&self) -> bool {
        self.coarse() != 0
    }

    #[doc="Sets the COARSE field."]
    #[inline] pub fn set_coarse<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Multiplication Ratio Difference"]
    #[inline] pub fn diff(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if DIFF != 0"]
    #[inline] pub fn test_diff(&self) -> bool {
        self.diff() != 0
    }

    #[doc="Sets the DIFF field."]
    #[inline] pub fn set_diff<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Dfllval {
    #[inline]
    fn from(other: u32) -> Self {
         Dfllval(other)
    }
}

impl ::core::fmt::Display for Dfllval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfllval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fine() != 0 { try!(write!(f, " fine=0x{:x}", self.fine()))}
        if self.coarse() != 0 { try!(write!(f, " coarse=0x{:x}", self.coarse()))}
        if self.diff() != 0 { try!(write!(f, " diff=0x{:x}", self.diff()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DFLL48M Multiplier"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfllmul(pub u32);
impl Dfllmul {
    #[doc="DFLL Multiply Factor"]
    #[inline] pub fn mul(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MUL != 0"]
    #[inline] pub fn test_mul(&self) -> bool {
        self.mul() != 0
    }

    #[doc="Sets the MUL field."]
    #[inline] pub fn set_mul<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fine Maximum Step"]
    #[inline] pub fn fstep(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if FSTEP != 0"]
    #[inline] pub fn test_fstep(&self) -> bool {
        self.fstep() != 0
    }

    #[doc="Sets the FSTEP field."]
    #[inline] pub fn set_fstep<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Coarse Maximum Step"]
    #[inline] pub fn cstep(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
    }

    #[doc="Returns true if CSTEP != 0"]
    #[inline] pub fn test_cstep(&self) -> bool {
        self.cstep() != 0
    }

    #[doc="Sets the CSTEP field."]
    #[inline] pub fn set_cstep<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Dfllmul {
    #[inline]
    fn from(other: u32) -> Self {
         Dfllmul(other)
    }
}

impl ::core::fmt::Display for Dfllmul {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfllmul {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mul() != 0 { try!(write!(f, " mul=0x{:x}", self.mul()))}
        if self.fstep() != 0 { try!(write!(f, " fstep=0x{:x}", self.fstep()))}
        if self.cstep() != 0 { try!(write!(f, " cstep=0x{:x}", self.cstep()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DFLL48M Synchronization"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfllsync(pub u8);
impl Dfllsync {
    #[doc="ENABLE Synchronization Busy"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DFLLCTRLB Synchronization Busy"]
    #[inline] pub fn dfllctrlb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DFLLCTRLB != 0"]
    #[inline] pub fn test_dfllctrlb(&self) -> bool {
        self.dfllctrlb() != 0
    }

    #[doc="Sets the DFLLCTRLB field."]
    #[inline] pub fn set_dfllctrlb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DFLLVAL Synchronization Busy"]
    #[inline] pub fn dfllval(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DFLLVAL != 0"]
    #[inline] pub fn test_dfllval(&self) -> bool {
        self.dfllval() != 0
    }

    #[doc="Sets the DFLLVAL field."]
    #[inline] pub fn set_dfllval<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DFLLMUL Synchronization Busy"]
    #[inline] pub fn dfllmul(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DFLLMUL != 0"]
    #[inline] pub fn test_dfllmul(&self) -> bool {
        self.dfllmul() != 0
    }

    #[doc="Sets the DFLLMUL field."]
    #[inline] pub fn set_dfllmul<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Dfllsync {
    #[inline]
    fn from(other: u8) -> Self {
         Dfllsync(other)
    }
}

impl ::core::fmt::Display for Dfllsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfllsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.dfllctrlb() != 0 { try!(write!(f, " dfllctrlb"))}
        if self.dfllval() != 0 { try!(write!(f, " dfllval"))}
        if self.dfllmul() != 0 { try!(write!(f, " dfllmul"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DPLL Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllctrla(pub u8);
impl Dpllctrla {
    #[doc="DPLL Enable"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="On Demand Control"]
    #[inline] pub fn ondemand(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Dpllctrla {
    #[inline]
    fn from(other: u8) -> Self {
         Dpllctrla(other)
    }
}

impl ::core::fmt::Display for Dpllctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dpllctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DPLL Ratio Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllratio(pub u32);
impl Dpllratio {
    #[doc="Loop Divider Ratio"]
    #[inline] pub fn ldr(&self) -> ::bobbin_bits::U13 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1fff) as u16) } // [12:0]
    }

    #[doc="Returns true if LDR != 0"]
    #[inline] pub fn test_ldr(&self) -> bool {
        self.ldr() != 0
    }

    #[doc="Sets the LDR field."]
    #[inline] pub fn set_ldr<V: Into<::bobbin_bits::U13>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U13 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Loop Divider Ratio Fractional Part"]
    #[inline] pub fn ldrfrac(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if LDRFRAC != 0"]
    #[inline] pub fn test_ldrfrac(&self) -> bool {
        self.ldrfrac() != 0
    }

    #[doc="Sets the LDRFRAC field."]
    #[inline] pub fn set_ldrfrac<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Dpllratio {
    #[inline]
    fn from(other: u32) -> Self {
         Dpllratio(other)
    }
}

impl ::core::fmt::Display for Dpllratio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dpllratio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ldr() != 0 { try!(write!(f, " ldr=0x{:x}", self.ldr()))}
        if self.ldrfrac() != 0 { try!(write!(f, " ldrfrac=0x{:x}", self.ldrfrac()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DPLL Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllctrlb(pub u32);
impl Dpllctrlb {
    #[doc="Proportional Integral Filter Selection"]
    #[inline] pub fn filter(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if FILTER != 0"]
    #[inline] pub fn test_filter(&self) -> bool {
        self.filter() != 0
    }

    #[doc="Sets the FILTER field."]
    #[inline] pub fn set_filter<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Wake Up Fast"]
    #[inline] pub fn wuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WUF != 0"]
    #[inline] pub fn test_wuf(&self) -> bool {
        self.wuf() != 0
    }

    #[doc="Sets the WUF field."]
    #[inline] pub fn set_wuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Reference Clock Selection"]
    #[inline] pub fn refclk(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if REFCLK != 0"]
    #[inline] pub fn test_refclk(&self) -> bool {
        self.refclk() != 0
    }

    #[doc="Sets the REFCLK field."]
    #[inline] pub fn set_refclk<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Lock Time"]
    #[inline] pub fn ltime(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if LTIME != 0"]
    #[inline] pub fn test_ltime(&self) -> bool {
        self.ltime() != 0
    }

    #[doc="Sets the LTIME field."]
    #[inline] pub fn set_ltime<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Lock Bypass"]
    #[inline] pub fn lbypass(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if LBYPASS != 0"]
    #[inline] pub fn test_lbypass(&self) -> bool {
        self.lbypass() != 0
    }

    #[doc="Sets the LBYPASS field."]
    #[inline] pub fn set_lbypass<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Sigma-Delta DCO Filter Selection"]
    #[inline] pub fn dcofilter(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if DCOFILTER != 0"]
    #[inline] pub fn test_dcofilter(&self) -> bool {
        self.dcofilter() != 0
    }

    #[doc="Sets the DCOFILTER field."]
    #[inline] pub fn set_dcofilter<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DCO Filter Enable"]
    #[inline] pub fn dcoen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DCOEN != 0"]
    #[inline] pub fn test_dcoen(&self) -> bool {
        self.dcoen() != 0
    }

    #[doc="Sets the DCOEN field."]
    #[inline] pub fn set_dcoen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Clock Divider"]
    #[inline] pub fn div(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7ff) as u16) } // [26:16]
    }

    #[doc="Returns true if DIV != 0"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Sets the DIV field."]
    #[inline] pub fn set_div<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Dpllctrlb {
    #[inline]
    fn from(other: u32) -> Self {
         Dpllctrlb(other)
    }
}

impl ::core::fmt::Display for Dpllctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dpllctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.filter() != 0 { try!(write!(f, " filter=0x{:x}", self.filter()))}
        if self.wuf() != 0 { try!(write!(f, " wuf"))}
        if self.refclk() != 0 { try!(write!(f, " refclk=0x{:x}", self.refclk()))}
        if self.ltime() != 0 { try!(write!(f, " ltime=0x{:x}", self.ltime()))}
        if self.lbypass() != 0 { try!(write!(f, " lbypass"))}
        if self.dcofilter() != 0 { try!(write!(f, " dcofilter=0x{:x}", self.dcofilter()))}
        if self.dcoen() != 0 { try!(write!(f, " dcoen"))}
        if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DPLL Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllsyncbusy(pub u32);
impl Dpllsyncbusy {
    #[doc="DPLL Enable Synchronization Status"]
    #[inline] pub fn enable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DPLL Loop Divider Ratio Synchronization Status"]
    #[inline] pub fn dpllratio(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DPLLRATIO != 0"]
    #[inline] pub fn test_dpllratio(&self) -> bool {
        self.dpllratio() != 0
    }

    #[doc="Sets the DPLLRATIO field."]
    #[inline] pub fn set_dpllratio<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Dpllsyncbusy {
    #[inline]
    fn from(other: u32) -> Self {
         Dpllsyncbusy(other)
    }
}

impl ::core::fmt::Display for Dpllsyncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dpllsyncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.dpllratio() != 0 { try!(write!(f, " dpllratio"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DPLL Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllstatus(pub u32);
impl Dpllstatus {
    #[doc="DPLL Lock Status"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DPLL Clock Ready"]
    #[inline] pub fn clkrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CLKRDY != 0"]
    #[inline] pub fn test_clkrdy(&self) -> bool {
        self.clkrdy() != 0
    }

    #[doc="Sets the CLKRDY field."]
    #[inline] pub fn set_clkrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Dpllstatus {
    #[inline]
    fn from(other: u32) -> Self {
         Dpllstatus(other)
    }
}

impl ::core::fmt::Display for Dpllstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dpllstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.clkrdy() != 0 { try!(write!(f, " clkrdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

