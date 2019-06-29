::bobbin_mcu::periph!( TCC0, Tcc0, TCC0_PERIPH, TccPeriph, TCC0_OWNED, TCC0_REF_COUNT, 0x41016000, 0x00, 0x2c);
::bobbin_mcu::periph!( TCC1, Tcc1, TCC1_PERIPH, TccPeriph, TCC1_OWNED, TCC1_REF_COUNT, 0x41018000, 0x01, 0x2d);
::bobbin_mcu::periph!( TCC2, Tcc2, TCC2_PERIPH, TccPeriph, TCC2_OWNED, TCC2_REF_COUNT, 0x42000c00, 0x02, 0x2e);
::bobbin_mcu::periph!( TCC3, Tcc3, TCC3_PERIPH, TccPeriph, TCC3_OWNED, TCC3_REF_COUNT, 0x42001000, 0x03, 0x2f);
::bobbin_mcu::periph!( TCC4, Tcc4, TCC4_PERIPH, TccPeriph, TCC4_OWNED, TCC4_REF_COUNT, 0x43001000, 0x04, 0x30);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TCC Peripheral"]
pub struct TccPeriph(pub usize); 

impl TccPeriph {
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

    #[doc="Get the CTRLBCLR Register."]
    #[inline] pub fn ctrlbclr_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlbclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlbclr, 0x4)
    }

    #[doc="Get the *mut pointer for the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr_mut(&self) -> *mut Ctrlbclr { 
        self.ctrlbclr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr_ptr(&self) -> *const Ctrlbclr { 
        self.ctrlbclr_reg().ptr()
    }

    #[doc="Read the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
        self.ctrlbclr_reg().read()
    }

    #[doc="Write the CTRLBCLR register."]
    #[inline] pub fn write_ctrlbclr(&self, value: Ctrlbclr) -> &Self { 
        self.ctrlbclr_reg().write(value);
        self
    }

    #[doc="Set the CTRLBCLR register."]
    #[inline] pub fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
        self.ctrlbclr_reg().set(f);
        self
    }

    #[doc="Modify the CTRLBCLR register."]
    #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
        self.ctrlbclr_reg().with(f);
        self
    }

    #[doc="Get the CTRLBSET Register."]
    #[inline] pub fn ctrlbset_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlbset> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlbset, 0x5)
    }

    #[doc="Get the *mut pointer for the CTRLBSET register."]
    #[inline] pub fn ctrlbset_mut(&self) -> *mut Ctrlbset { 
        self.ctrlbset_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLBSET register."]
    #[inline] pub fn ctrlbset_ptr(&self) -> *const Ctrlbset { 
        self.ctrlbset_reg().ptr()
    }

    #[doc="Read the CTRLBSET register."]
    #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
        self.ctrlbset_reg().read()
    }

    #[doc="Write the CTRLBSET register."]
    #[inline] pub fn write_ctrlbset(&self, value: Ctrlbset) -> &Self { 
        self.ctrlbset_reg().write(value);
        self
    }

    #[doc="Set the CTRLBSET register."]
    #[inline] pub fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
        self.ctrlbset_reg().set(f);
        self
    }

    #[doc="Modify the CTRLBSET register."]
    #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
        self.ctrlbset_reg().with(f);
        self
    }

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x8)
    }

    #[doc="Get the *mut pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_mut(&self) -> *mut Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNCBUSY register."]
    #[inline] pub fn syncbusy_ptr(&self) -> *const Syncbusy { 
        self.syncbusy_reg().ptr()
    }

    #[doc="Read the SYNCBUSY register."]
    #[inline] pub fn syncbusy(&self) -> Syncbusy { 
        self.syncbusy_reg().read()
    }

    #[doc="Get the FCTRLA Register."]
    #[inline] pub fn fctrla_reg(&self) -> ::bobbin_mcu::register::Register<Fctrla> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fctrla, 0xc)
    }

    #[doc="Get the *mut pointer for the FCTRLA register."]
    #[inline] pub fn fctrla_mut(&self) -> *mut Fctrla { 
        self.fctrla_reg().ptr()
    }

    #[doc="Get the *const pointer for the FCTRLA register."]
    #[inline] pub fn fctrla_ptr(&self) -> *const Fctrla { 
        self.fctrla_reg().ptr()
    }

    #[doc="Read the FCTRLA register."]
    #[inline] pub fn fctrla(&self) -> Fctrla { 
        self.fctrla_reg().read()
    }

    #[doc="Write the FCTRLA register."]
    #[inline] pub fn write_fctrla(&self, value: Fctrla) -> &Self { 
        self.fctrla_reg().write(value);
        self
    }

    #[doc="Set the FCTRLA register."]
    #[inline] pub fn set_fctrla<F: FnOnce(Fctrla) -> Fctrla>(&self, f: F) -> &Self {
        self.fctrla_reg().set(f);
        self
    }

    #[doc="Modify the FCTRLA register."]
    #[inline] pub fn with_fctrla<F: FnOnce(Fctrla) -> Fctrla>(&self, f: F) -> &Self {
        self.fctrla_reg().with(f);
        self
    }

    #[doc="Get the FCTRLB Register."]
    #[inline] pub fn fctrlb_reg(&self) -> ::bobbin_mcu::register::Register<Fctrlb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fctrlb, 0x10)
    }

    #[doc="Get the *mut pointer for the FCTRLB register."]
    #[inline] pub fn fctrlb_mut(&self) -> *mut Fctrlb { 
        self.fctrlb_reg().ptr()
    }

    #[doc="Get the *const pointer for the FCTRLB register."]
    #[inline] pub fn fctrlb_ptr(&self) -> *const Fctrlb { 
        self.fctrlb_reg().ptr()
    }

    #[doc="Read the FCTRLB register."]
    #[inline] pub fn fctrlb(&self) -> Fctrlb { 
        self.fctrlb_reg().read()
    }

    #[doc="Write the FCTRLB register."]
    #[inline] pub fn write_fctrlb(&self, value: Fctrlb) -> &Self { 
        self.fctrlb_reg().write(value);
        self
    }

    #[doc="Set the FCTRLB register."]
    #[inline] pub fn set_fctrlb<F: FnOnce(Fctrlb) -> Fctrlb>(&self, f: F) -> &Self {
        self.fctrlb_reg().set(f);
        self
    }

    #[doc="Modify the FCTRLB register."]
    #[inline] pub fn with_fctrlb<F: FnOnce(Fctrlb) -> Fctrlb>(&self, f: F) -> &Self {
        self.fctrlb_reg().with(f);
        self
    }

    #[doc="Get the WEXCTRL Register."]
    #[inline] pub fn wexctrl_reg(&self) -> ::bobbin_mcu::register::Register<Wexctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wexctrl, 0x14)
    }

    #[doc="Get the *mut pointer for the WEXCTRL register."]
    #[inline] pub fn wexctrl_mut(&self) -> *mut Wexctrl { 
        self.wexctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the WEXCTRL register."]
    #[inline] pub fn wexctrl_ptr(&self) -> *const Wexctrl { 
        self.wexctrl_reg().ptr()
    }

    #[doc="Read the WEXCTRL register."]
    #[inline] pub fn wexctrl(&self) -> Wexctrl { 
        self.wexctrl_reg().read()
    }

    #[doc="Write the WEXCTRL register."]
    #[inline] pub fn write_wexctrl(&self, value: Wexctrl) -> &Self { 
        self.wexctrl_reg().write(value);
        self
    }

    #[doc="Set the WEXCTRL register."]
    #[inline] pub fn set_wexctrl<F: FnOnce(Wexctrl) -> Wexctrl>(&self, f: F) -> &Self {
        self.wexctrl_reg().set(f);
        self
    }

    #[doc="Modify the WEXCTRL register."]
    #[inline] pub fn with_wexctrl<F: FnOnce(Wexctrl) -> Wexctrl>(&self, f: F) -> &Self {
        self.wexctrl_reg().with(f);
        self
    }

    #[doc="Get the DRVCTRL Register."]
    #[inline] pub fn drvctrl_reg(&self) -> ::bobbin_mcu::register::Register<Drvctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Drvctrl, 0x18)
    }

    #[doc="Get the *mut pointer for the DRVCTRL register."]
    #[inline] pub fn drvctrl_mut(&self) -> *mut Drvctrl { 
        self.drvctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the DRVCTRL register."]
    #[inline] pub fn drvctrl_ptr(&self) -> *const Drvctrl { 
        self.drvctrl_reg().ptr()
    }

    #[doc="Read the DRVCTRL register."]
    #[inline] pub fn drvctrl(&self) -> Drvctrl { 
        self.drvctrl_reg().read()
    }

    #[doc="Write the DRVCTRL register."]
    #[inline] pub fn write_drvctrl(&self, value: Drvctrl) -> &Self { 
        self.drvctrl_reg().write(value);
        self
    }

    #[doc="Set the DRVCTRL register."]
    #[inline] pub fn set_drvctrl<F: FnOnce(Drvctrl) -> Drvctrl>(&self, f: F) -> &Self {
        self.drvctrl_reg().set(f);
        self
    }

    #[doc="Modify the DRVCTRL register."]
    #[inline] pub fn with_drvctrl<F: FnOnce(Drvctrl) -> Drvctrl>(&self, f: F) -> &Self {
        self.drvctrl_reg().with(f);
        self
    }

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0x1e)
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

    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::Register<Evctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Evctrl, 0x20)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x24)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x28)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x2c)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x30)
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

    #[doc="Write the STATUS register."]
    #[inline] pub fn write_status(&self, value: Status) -> &Self { 
        self.status_reg().write(value);
        self
    }

    #[doc="Set the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().set(f);
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().with(f);
        self
    }

    #[doc="Get the COUNT Register."]
    #[inline] pub fn count_reg(&self) -> ::bobbin_mcu::register::Register<Count> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Count, 0x34)
    }

    #[doc="Get the *mut pointer for the COUNT register."]
    #[inline] pub fn count_mut(&self) -> *mut Count { 
        self.count_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT register."]
    #[inline] pub fn count_ptr(&self) -> *const Count { 
        self.count_reg().ptr()
    }

    #[doc="Read the COUNT register."]
    #[inline] pub fn count(&self) -> Count { 
        self.count_reg().read()
    }

    #[doc="Write the COUNT register."]
    #[inline] pub fn write_count(&self, value: Count) -> &Self { 
        self.count_reg().write(value);
        self
    }

    #[doc="Set the COUNT register."]
    #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        self.count_reg().set(f);
        self
    }

    #[doc="Modify the COUNT register."]
    #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        self.count_reg().with(f);
        self
    }

    #[doc="Get the COUNT_DITH4 Register."]
    #[inline] pub fn count_dith4_reg(&self) -> ::bobbin_mcu::register::Register<CountDith4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut CountDith4, 0x34)
    }

    #[doc="Get the *mut pointer for the COUNT_DITH4 register."]
    #[inline] pub fn count_dith4_mut(&self) -> *mut CountDith4 { 
        self.count_dith4_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT_DITH4 register."]
    #[inline] pub fn count_dith4_ptr(&self) -> *const CountDith4 { 
        self.count_dith4_reg().ptr()
    }

    #[doc="Read the COUNT_DITH4 register."]
    #[inline] pub fn count_dith4(&self) -> CountDith4 { 
        self.count_dith4_reg().read()
    }

    #[doc="Write the COUNT_DITH4 register."]
    #[inline] pub fn write_count_dith4(&self, value: CountDith4) -> &Self { 
        self.count_dith4_reg().write(value);
        self
    }

    #[doc="Set the COUNT_DITH4 register."]
    #[inline] pub fn set_count_dith4<F: FnOnce(CountDith4) -> CountDith4>(&self, f: F) -> &Self {
        self.count_dith4_reg().set(f);
        self
    }

    #[doc="Modify the COUNT_DITH4 register."]
    #[inline] pub fn with_count_dith4<F: FnOnce(CountDith4) -> CountDith4>(&self, f: F) -> &Self {
        self.count_dith4_reg().with(f);
        self
    }

    #[doc="Get the COUNT_DITH5 Register."]
    #[inline] pub fn count_dith5_reg(&self) -> ::bobbin_mcu::register::Register<CountDith5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut CountDith5, 0x34)
    }

    #[doc="Get the *mut pointer for the COUNT_DITH5 register."]
    #[inline] pub fn count_dith5_mut(&self) -> *mut CountDith5 { 
        self.count_dith5_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT_DITH5 register."]
    #[inline] pub fn count_dith5_ptr(&self) -> *const CountDith5 { 
        self.count_dith5_reg().ptr()
    }

    #[doc="Read the COUNT_DITH5 register."]
    #[inline] pub fn count_dith5(&self) -> CountDith5 { 
        self.count_dith5_reg().read()
    }

    #[doc="Write the COUNT_DITH5 register."]
    #[inline] pub fn write_count_dith5(&self, value: CountDith5) -> &Self { 
        self.count_dith5_reg().write(value);
        self
    }

    #[doc="Set the COUNT_DITH5 register."]
    #[inline] pub fn set_count_dith5<F: FnOnce(CountDith5) -> CountDith5>(&self, f: F) -> &Self {
        self.count_dith5_reg().set(f);
        self
    }

    #[doc="Modify the COUNT_DITH5 register."]
    #[inline] pub fn with_count_dith5<F: FnOnce(CountDith5) -> CountDith5>(&self, f: F) -> &Self {
        self.count_dith5_reg().with(f);
        self
    }

    #[doc="Get the COUNT_DITH6 Register."]
    #[inline] pub fn count_dith6_reg(&self) -> ::bobbin_mcu::register::Register<CountDith6> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut CountDith6, 0x34)
    }

    #[doc="Get the *mut pointer for the COUNT_DITH6 register."]
    #[inline] pub fn count_dith6_mut(&self) -> *mut CountDith6 { 
        self.count_dith6_reg().ptr()
    }

    #[doc="Get the *const pointer for the COUNT_DITH6 register."]
    #[inline] pub fn count_dith6_ptr(&self) -> *const CountDith6 { 
        self.count_dith6_reg().ptr()
    }

    #[doc="Read the COUNT_DITH6 register."]
    #[inline] pub fn count_dith6(&self) -> CountDith6 { 
        self.count_dith6_reg().read()
    }

    #[doc="Write the COUNT_DITH6 register."]
    #[inline] pub fn write_count_dith6(&self, value: CountDith6) -> &Self { 
        self.count_dith6_reg().write(value);
        self
    }

    #[doc="Set the COUNT_DITH6 register."]
    #[inline] pub fn set_count_dith6<F: FnOnce(CountDith6) -> CountDith6>(&self, f: F) -> &Self {
        self.count_dith6_reg().set(f);
        self
    }

    #[doc="Modify the COUNT_DITH6 register."]
    #[inline] pub fn with_count_dith6<F: FnOnce(CountDith6) -> CountDith6>(&self, f: F) -> &Self {
        self.count_dith6_reg().with(f);
        self
    }

    #[doc="Get the PATT Register."]
    #[inline] pub fn patt_reg(&self) -> ::bobbin_mcu::register::Register<Patt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Patt, 0x38)
    }

    #[doc="Get the *mut pointer for the PATT register."]
    #[inline] pub fn patt_mut(&self) -> *mut Patt { 
        self.patt_reg().ptr()
    }

    #[doc="Get the *const pointer for the PATT register."]
    #[inline] pub fn patt_ptr(&self) -> *const Patt { 
        self.patt_reg().ptr()
    }

    #[doc="Read the PATT register."]
    #[inline] pub fn patt(&self) -> Patt { 
        self.patt_reg().read()
    }

    #[doc="Write the PATT register."]
    #[inline] pub fn write_patt(&self, value: Patt) -> &Self { 
        self.patt_reg().write(value);
        self
    }

    #[doc="Set the PATT register."]
    #[inline] pub fn set_patt<F: FnOnce(Patt) -> Patt>(&self, f: F) -> &Self {
        self.patt_reg().set(f);
        self
    }

    #[doc="Modify the PATT register."]
    #[inline] pub fn with_patt<F: FnOnce(Patt) -> Patt>(&self, f: F) -> &Self {
        self.patt_reg().with(f);
        self
    }

    #[doc="Get the WAVE Register."]
    #[inline] pub fn wave_reg(&self) -> ::bobbin_mcu::register::Register<Wave> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wave, 0x3c)
    }

    #[doc="Get the *mut pointer for the WAVE register."]
    #[inline] pub fn wave_mut(&self) -> *mut Wave { 
        self.wave_reg().ptr()
    }

    #[doc="Get the *const pointer for the WAVE register."]
    #[inline] pub fn wave_ptr(&self) -> *const Wave { 
        self.wave_reg().ptr()
    }

    #[doc="Read the WAVE register."]
    #[inline] pub fn wave(&self) -> Wave { 
        self.wave_reg().read()
    }

    #[doc="Write the WAVE register."]
    #[inline] pub fn write_wave(&self, value: Wave) -> &Self { 
        self.wave_reg().write(value);
        self
    }

    #[doc="Set the WAVE register."]
    #[inline] pub fn set_wave<F: FnOnce(Wave) -> Wave>(&self, f: F) -> &Self {
        self.wave_reg().set(f);
        self
    }

    #[doc="Modify the WAVE register."]
    #[inline] pub fn with_wave<F: FnOnce(Wave) -> Wave>(&self, f: F) -> &Self {
        self.wave_reg().with(f);
        self
    }

    #[doc="Get the PER Register."]
    #[inline] pub fn per_reg(&self) -> ::bobbin_mcu::register::Register<Per> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Per, 0x40)
    }

    #[doc="Get the *mut pointer for the PER register."]
    #[inline] pub fn per_mut(&self) -> *mut Per { 
        self.per_reg().ptr()
    }

    #[doc="Get the *const pointer for the PER register."]
    #[inline] pub fn per_ptr(&self) -> *const Per { 
        self.per_reg().ptr()
    }

    #[doc="Read the PER register."]
    #[inline] pub fn per(&self) -> Per { 
        self.per_reg().read()
    }

    #[doc="Write the PER register."]
    #[inline] pub fn write_per(&self, value: Per) -> &Self { 
        self.per_reg().write(value);
        self
    }

    #[doc="Set the PER register."]
    #[inline] pub fn set_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
        self.per_reg().set(f);
        self
    }

    #[doc="Modify the PER register."]
    #[inline] pub fn with_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
        self.per_reg().with(f);
        self
    }

    #[doc="Get the PER_DITH4 Register."]
    #[inline] pub fn per_dith4_reg(&self) -> ::bobbin_mcu::register::Register<PerDith4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut PerDith4, 0x40)
    }

    #[doc="Get the *mut pointer for the PER_DITH4 register."]
    #[inline] pub fn per_dith4_mut(&self) -> *mut PerDith4 { 
        self.per_dith4_reg().ptr()
    }

    #[doc="Get the *const pointer for the PER_DITH4 register."]
    #[inline] pub fn per_dith4_ptr(&self) -> *const PerDith4 { 
        self.per_dith4_reg().ptr()
    }

    #[doc="Read the PER_DITH4 register."]
    #[inline] pub fn per_dith4(&self) -> PerDith4 { 
        self.per_dith4_reg().read()
    }

    #[doc="Write the PER_DITH4 register."]
    #[inline] pub fn write_per_dith4(&self, value: PerDith4) -> &Self { 
        self.per_dith4_reg().write(value);
        self
    }

    #[doc="Set the PER_DITH4 register."]
    #[inline] pub fn set_per_dith4<F: FnOnce(PerDith4) -> PerDith4>(&self, f: F) -> &Self {
        self.per_dith4_reg().set(f);
        self
    }

    #[doc="Modify the PER_DITH4 register."]
    #[inline] pub fn with_per_dith4<F: FnOnce(PerDith4) -> PerDith4>(&self, f: F) -> &Self {
        self.per_dith4_reg().with(f);
        self
    }

    #[doc="Get the PER_DITH5 Register."]
    #[inline] pub fn per_dith5_reg(&self) -> ::bobbin_mcu::register::Register<PerDith5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut PerDith5, 0x40)
    }

    #[doc="Get the *mut pointer for the PER_DITH5 register."]
    #[inline] pub fn per_dith5_mut(&self) -> *mut PerDith5 { 
        self.per_dith5_reg().ptr()
    }

    #[doc="Get the *const pointer for the PER_DITH5 register."]
    #[inline] pub fn per_dith5_ptr(&self) -> *const PerDith5 { 
        self.per_dith5_reg().ptr()
    }

    #[doc="Read the PER_DITH5 register."]
    #[inline] pub fn per_dith5(&self) -> PerDith5 { 
        self.per_dith5_reg().read()
    }

    #[doc="Write the PER_DITH5 register."]
    #[inline] pub fn write_per_dith5(&self, value: PerDith5) -> &Self { 
        self.per_dith5_reg().write(value);
        self
    }

    #[doc="Set the PER_DITH5 register."]
    #[inline] pub fn set_per_dith5<F: FnOnce(PerDith5) -> PerDith5>(&self, f: F) -> &Self {
        self.per_dith5_reg().set(f);
        self
    }

    #[doc="Modify the PER_DITH5 register."]
    #[inline] pub fn with_per_dith5<F: FnOnce(PerDith5) -> PerDith5>(&self, f: F) -> &Self {
        self.per_dith5_reg().with(f);
        self
    }

    #[doc="Get the PER_DITH6 Register."]
    #[inline] pub fn per_dith6_reg(&self) -> ::bobbin_mcu::register::Register<PerDith6> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut PerDith6, 0x40)
    }

    #[doc="Get the *mut pointer for the PER_DITH6 register."]
    #[inline] pub fn per_dith6_mut(&self) -> *mut PerDith6 { 
        self.per_dith6_reg().ptr()
    }

    #[doc="Get the *const pointer for the PER_DITH6 register."]
    #[inline] pub fn per_dith6_ptr(&self) -> *const PerDith6 { 
        self.per_dith6_reg().ptr()
    }

    #[doc="Read the PER_DITH6 register."]
    #[inline] pub fn per_dith6(&self) -> PerDith6 { 
        self.per_dith6_reg().read()
    }

    #[doc="Write the PER_DITH6 register."]
    #[inline] pub fn write_per_dith6(&self, value: PerDith6) -> &Self { 
        self.per_dith6_reg().write(value);
        self
    }

    #[doc="Set the PER_DITH6 register."]
    #[inline] pub fn set_per_dith6<F: FnOnce(PerDith6) -> PerDith6>(&self, f: F) -> &Self {
        self.per_dith6_reg().set(f);
        self
    }

    #[doc="Modify the PER_DITH6 register."]
    #[inline] pub fn with_per_dith6<F: FnOnce(PerDith6) -> PerDith6>(&self, f: F) -> &Self {
        self.per_dith6_reg().with(f);
        self
    }

    #[doc="Get the CC Register."]
    #[inline] pub fn cc_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Cc, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Cc, 0x44, 0x4)
    }

    #[doc="Get the *mut pointer for the CC register."]
    #[inline] pub fn cc_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Cc { 
        self.cc_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CC register."]
    #[inline] pub fn cc_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Cc { 
        self.cc_reg().ptr(index.into())
    }

    #[doc="Read the CC register."]
    #[inline] pub fn cc<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Cc { 
        self.cc_reg().read(index.into())
    }

    #[doc="Write the CC register."]
    #[inline] pub fn write_cc<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Cc) -> &Self {
        self.cc_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CC register."]
    #[inline] pub fn set_cc<I: Into<::bobbin_bits::R6>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
        self.cc_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CC register."]
    #[inline] pub fn with_cc<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
        self.cc_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CC_DITH4 Register."]
    #[inline] pub fn cc_dith4_reg(&self) -> ::bobbin_mcu::register::RegisterArray<CcDith4, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut CcDith4, 0x44, 0x4)
    }

    #[doc="Get the *mut pointer for the CC_DITH4 register."]
    #[inline] pub fn cc_dith4_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut CcDith4 { 
        self.cc_dith4_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CC_DITH4 register."]
    #[inline] pub fn cc_dith4_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const CcDith4 { 
        self.cc_dith4_reg().ptr(index.into())
    }

    #[doc="Read the CC_DITH4 register."]
    #[inline] pub fn cc_dith4<I: Into<::bobbin_bits::R6>>(&self, index: I) -> CcDith4 { 
        self.cc_dith4_reg().read(index.into())
    }

    #[doc="Write the CC_DITH4 register."]
    #[inline] pub fn write_cc_dith4<I: Into<::bobbin_bits::R6>>(&self, index: I, value: CcDith4) -> &Self {
        self.cc_dith4_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CC_DITH4 register."]
    #[inline] pub fn set_cc_dith4<I: Into<::bobbin_bits::R6>, F: FnOnce(CcDith4) -> CcDith4>(&self, index: I, f: F) -> &Self {
        self.cc_dith4_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CC_DITH4 register."]
    #[inline] pub fn with_cc_dith4<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(CcDith4) -> CcDith4>(&self, index: I, f: F) -> &Self {
        self.cc_dith4_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CC_DITH5 Register."]
    #[inline] pub fn cc_dith5_reg(&self) -> ::bobbin_mcu::register::RegisterArray<CcDith5, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut CcDith5, 0x44, 0x4)
    }

    #[doc="Get the *mut pointer for the CC_DITH5 register."]
    #[inline] pub fn cc_dith5_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut CcDith5 { 
        self.cc_dith5_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CC_DITH5 register."]
    #[inline] pub fn cc_dith5_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const CcDith5 { 
        self.cc_dith5_reg().ptr(index.into())
    }

    #[doc="Read the CC_DITH5 register."]
    #[inline] pub fn cc_dith5<I: Into<::bobbin_bits::R6>>(&self, index: I) -> CcDith5 { 
        self.cc_dith5_reg().read(index.into())
    }

    #[doc="Write the CC_DITH5 register."]
    #[inline] pub fn write_cc_dith5<I: Into<::bobbin_bits::R6>>(&self, index: I, value: CcDith5) -> &Self {
        self.cc_dith5_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CC_DITH5 register."]
    #[inline] pub fn set_cc_dith5<I: Into<::bobbin_bits::R6>, F: FnOnce(CcDith5) -> CcDith5>(&self, index: I, f: F) -> &Self {
        self.cc_dith5_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CC_DITH5 register."]
    #[inline] pub fn with_cc_dith5<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(CcDith5) -> CcDith5>(&self, index: I, f: F) -> &Self {
        self.cc_dith5_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CC_DITH6 Register."]
    #[inline] pub fn cc_dith6_reg(&self) -> ::bobbin_mcu::register::RegisterArray<CcDith6, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut CcDith6, 0x44, 0x4)
    }

    #[doc="Get the *mut pointer for the CC_DITH6 register."]
    #[inline] pub fn cc_dith6_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut CcDith6 { 
        self.cc_dith6_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CC_DITH6 register."]
    #[inline] pub fn cc_dith6_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const CcDith6 { 
        self.cc_dith6_reg().ptr(index.into())
    }

    #[doc="Read the CC_DITH6 register."]
    #[inline] pub fn cc_dith6<I: Into<::bobbin_bits::R6>>(&self, index: I) -> CcDith6 { 
        self.cc_dith6_reg().read(index.into())
    }

    #[doc="Write the CC_DITH6 register."]
    #[inline] pub fn write_cc_dith6<I: Into<::bobbin_bits::R6>>(&self, index: I, value: CcDith6) -> &Self {
        self.cc_dith6_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CC_DITH6 register."]
    #[inline] pub fn set_cc_dith6<I: Into<::bobbin_bits::R6>, F: FnOnce(CcDith6) -> CcDith6>(&self, index: I, f: F) -> &Self {
        self.cc_dith6_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CC_DITH6 register."]
    #[inline] pub fn with_cc_dith6<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(CcDith6) -> CcDith6>(&self, index: I, f: F) -> &Self {
        self.cc_dith6_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PATTBUF Register."]
    #[inline] pub fn pattbuf_reg(&self) -> ::bobbin_mcu::register::Register<Pattbuf> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pattbuf, 0x64)
    }

    #[doc="Get the *mut pointer for the PATTBUF register."]
    #[inline] pub fn pattbuf_mut(&self) -> *mut Pattbuf { 
        self.pattbuf_reg().ptr()
    }

    #[doc="Get the *const pointer for the PATTBUF register."]
    #[inline] pub fn pattbuf_ptr(&self) -> *const Pattbuf { 
        self.pattbuf_reg().ptr()
    }

    #[doc="Read the PATTBUF register."]
    #[inline] pub fn pattbuf(&self) -> Pattbuf { 
        self.pattbuf_reg().read()
    }

    #[doc="Write the PATTBUF register."]
    #[inline] pub fn write_pattbuf(&self, value: Pattbuf) -> &Self { 
        self.pattbuf_reg().write(value);
        self
    }

    #[doc="Set the PATTBUF register."]
    #[inline] pub fn set_pattbuf<F: FnOnce(Pattbuf) -> Pattbuf>(&self, f: F) -> &Self {
        self.pattbuf_reg().set(f);
        self
    }

    #[doc="Modify the PATTBUF register."]
    #[inline] pub fn with_pattbuf<F: FnOnce(Pattbuf) -> Pattbuf>(&self, f: F) -> &Self {
        self.pattbuf_reg().with(f);
        self
    }

    #[doc="Get the PERBUF Register."]
    #[inline] pub fn perbuf_reg(&self) -> ::bobbin_mcu::register::Register<Perbuf> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Perbuf, 0x6c)
    }

    #[doc="Get the *mut pointer for the PERBUF register."]
    #[inline] pub fn perbuf_mut(&self) -> *mut Perbuf { 
        self.perbuf_reg().ptr()
    }

    #[doc="Get the *const pointer for the PERBUF register."]
    #[inline] pub fn perbuf_ptr(&self) -> *const Perbuf { 
        self.perbuf_reg().ptr()
    }

    #[doc="Read the PERBUF register."]
    #[inline] pub fn perbuf(&self) -> Perbuf { 
        self.perbuf_reg().read()
    }

    #[doc="Write the PERBUF register."]
    #[inline] pub fn write_perbuf(&self, value: Perbuf) -> &Self { 
        self.perbuf_reg().write(value);
        self
    }

    #[doc="Set the PERBUF register."]
    #[inline] pub fn set_perbuf<F: FnOnce(Perbuf) -> Perbuf>(&self, f: F) -> &Self {
        self.perbuf_reg().set(f);
        self
    }

    #[doc="Modify the PERBUF register."]
    #[inline] pub fn with_perbuf<F: FnOnce(Perbuf) -> Perbuf>(&self, f: F) -> &Self {
        self.perbuf_reg().with(f);
        self
    }

    #[doc="Get the PERBUF_DITH4 Register."]
    #[inline] pub fn perbuf_dith4_reg(&self) -> ::bobbin_mcu::register::Register<PerbufDith4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut PerbufDith4, 0x6c)
    }

    #[doc="Get the *mut pointer for the PERBUF_DITH4 register."]
    #[inline] pub fn perbuf_dith4_mut(&self) -> *mut PerbufDith4 { 
        self.perbuf_dith4_reg().ptr()
    }

    #[doc="Get the *const pointer for the PERBUF_DITH4 register."]
    #[inline] pub fn perbuf_dith4_ptr(&self) -> *const PerbufDith4 { 
        self.perbuf_dith4_reg().ptr()
    }

    #[doc="Read the PERBUF_DITH4 register."]
    #[inline] pub fn perbuf_dith4(&self) -> PerbufDith4 { 
        self.perbuf_dith4_reg().read()
    }

    #[doc="Write the PERBUF_DITH4 register."]
    #[inline] pub fn write_perbuf_dith4(&self, value: PerbufDith4) -> &Self { 
        self.perbuf_dith4_reg().write(value);
        self
    }

    #[doc="Set the PERBUF_DITH4 register."]
    #[inline] pub fn set_perbuf_dith4<F: FnOnce(PerbufDith4) -> PerbufDith4>(&self, f: F) -> &Self {
        self.perbuf_dith4_reg().set(f);
        self
    }

    #[doc="Modify the PERBUF_DITH4 register."]
    #[inline] pub fn with_perbuf_dith4<F: FnOnce(PerbufDith4) -> PerbufDith4>(&self, f: F) -> &Self {
        self.perbuf_dith4_reg().with(f);
        self
    }

    #[doc="Get the PERBUF_DITH5 Register."]
    #[inline] pub fn perbuf_dith5_reg(&self) -> ::bobbin_mcu::register::Register<PerbufDith5> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut PerbufDith5, 0x6c)
    }

    #[doc="Get the *mut pointer for the PERBUF_DITH5 register."]
    #[inline] pub fn perbuf_dith5_mut(&self) -> *mut PerbufDith5 { 
        self.perbuf_dith5_reg().ptr()
    }

    #[doc="Get the *const pointer for the PERBUF_DITH5 register."]
    #[inline] pub fn perbuf_dith5_ptr(&self) -> *const PerbufDith5 { 
        self.perbuf_dith5_reg().ptr()
    }

    #[doc="Read the PERBUF_DITH5 register."]
    #[inline] pub fn perbuf_dith5(&self) -> PerbufDith5 { 
        self.perbuf_dith5_reg().read()
    }

    #[doc="Write the PERBUF_DITH5 register."]
    #[inline] pub fn write_perbuf_dith5(&self, value: PerbufDith5) -> &Self { 
        self.perbuf_dith5_reg().write(value);
        self
    }

    #[doc="Set the PERBUF_DITH5 register."]
    #[inline] pub fn set_perbuf_dith5<F: FnOnce(PerbufDith5) -> PerbufDith5>(&self, f: F) -> &Self {
        self.perbuf_dith5_reg().set(f);
        self
    }

    #[doc="Modify the PERBUF_DITH5 register."]
    #[inline] pub fn with_perbuf_dith5<F: FnOnce(PerbufDith5) -> PerbufDith5>(&self, f: F) -> &Self {
        self.perbuf_dith5_reg().with(f);
        self
    }

    #[doc="Get the PERBUF_DITH6 Register."]
    #[inline] pub fn perbuf_dith6_reg(&self) -> ::bobbin_mcu::register::Register<PerbufDith6> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut PerbufDith6, 0x6c)
    }

    #[doc="Get the *mut pointer for the PERBUF_DITH6 register."]
    #[inline] pub fn perbuf_dith6_mut(&self) -> *mut PerbufDith6 { 
        self.perbuf_dith6_reg().ptr()
    }

    #[doc="Get the *const pointer for the PERBUF_DITH6 register."]
    #[inline] pub fn perbuf_dith6_ptr(&self) -> *const PerbufDith6 { 
        self.perbuf_dith6_reg().ptr()
    }

    #[doc="Read the PERBUF_DITH6 register."]
    #[inline] pub fn perbuf_dith6(&self) -> PerbufDith6 { 
        self.perbuf_dith6_reg().read()
    }

    #[doc="Write the PERBUF_DITH6 register."]
    #[inline] pub fn write_perbuf_dith6(&self, value: PerbufDith6) -> &Self { 
        self.perbuf_dith6_reg().write(value);
        self
    }

    #[doc="Set the PERBUF_DITH6 register."]
    #[inline] pub fn set_perbuf_dith6<F: FnOnce(PerbufDith6) -> PerbufDith6>(&self, f: F) -> &Self {
        self.perbuf_dith6_reg().set(f);
        self
    }

    #[doc="Modify the PERBUF_DITH6 register."]
    #[inline] pub fn with_perbuf_dith6<F: FnOnce(PerbufDith6) -> PerbufDith6>(&self, f: F) -> &Self {
        self.perbuf_dith6_reg().with(f);
        self
    }

    #[doc="Get the CCBUF Register."]
    #[inline] pub fn ccbuf_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Ccbuf, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Ccbuf, 0x70, 0x4)
    }

    #[doc="Get the *mut pointer for the CCBUF register."]
    #[inline] pub fn ccbuf_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Ccbuf { 
        self.ccbuf_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CCBUF register."]
    #[inline] pub fn ccbuf_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Ccbuf { 
        self.ccbuf_reg().ptr(index.into())
    }

    #[doc="Read the CCBUF register."]
    #[inline] pub fn ccbuf<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Ccbuf { 
        self.ccbuf_reg().read(index.into())
    }

    #[doc="Write the CCBUF register."]
    #[inline] pub fn write_ccbuf<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Ccbuf) -> &Self {
        self.ccbuf_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CCBUF register."]
    #[inline] pub fn set_ccbuf<I: Into<::bobbin_bits::R6>, F: FnOnce(Ccbuf) -> Ccbuf>(&self, index: I, f: F) -> &Self {
        self.ccbuf_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CCBUF register."]
    #[inline] pub fn with_ccbuf<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Ccbuf) -> Ccbuf>(&self, index: I, f: F) -> &Self {
        self.ccbuf_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CCBUF_DITH4 Register."]
    #[inline] pub fn ccbuf_dith4_reg(&self) -> ::bobbin_mcu::register::RegisterArray<CcbufDith4, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut CcbufDith4, 0x70, 0x4)
    }

    #[doc="Get the *mut pointer for the CCBUF_DITH4 register."]
    #[inline] pub fn ccbuf_dith4_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut CcbufDith4 { 
        self.ccbuf_dith4_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CCBUF_DITH4 register."]
    #[inline] pub fn ccbuf_dith4_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const CcbufDith4 { 
        self.ccbuf_dith4_reg().ptr(index.into())
    }

    #[doc="Read the CCBUF_DITH4 register."]
    #[inline] pub fn ccbuf_dith4<I: Into<::bobbin_bits::R6>>(&self, index: I) -> CcbufDith4 { 
        self.ccbuf_dith4_reg().read(index.into())
    }

    #[doc="Write the CCBUF_DITH4 register."]
    #[inline] pub fn write_ccbuf_dith4<I: Into<::bobbin_bits::R6>>(&self, index: I, value: CcbufDith4) -> &Self {
        self.ccbuf_dith4_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CCBUF_DITH4 register."]
    #[inline] pub fn set_ccbuf_dith4<I: Into<::bobbin_bits::R6>, F: FnOnce(CcbufDith4) -> CcbufDith4>(&self, index: I, f: F) -> &Self {
        self.ccbuf_dith4_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CCBUF_DITH4 register."]
    #[inline] pub fn with_ccbuf_dith4<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(CcbufDith4) -> CcbufDith4>(&self, index: I, f: F) -> &Self {
        self.ccbuf_dith4_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CCBUF_DITH5 Register."]
    #[inline] pub fn ccbuf_dith5_reg(&self) -> ::bobbin_mcu::register::RegisterArray<CcbufDith5, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut CcbufDith5, 0x70, 0x4)
    }

    #[doc="Get the *mut pointer for the CCBUF_DITH5 register."]
    #[inline] pub fn ccbuf_dith5_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut CcbufDith5 { 
        self.ccbuf_dith5_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CCBUF_DITH5 register."]
    #[inline] pub fn ccbuf_dith5_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const CcbufDith5 { 
        self.ccbuf_dith5_reg().ptr(index.into())
    }

    #[doc="Read the CCBUF_DITH5 register."]
    #[inline] pub fn ccbuf_dith5<I: Into<::bobbin_bits::R6>>(&self, index: I) -> CcbufDith5 { 
        self.ccbuf_dith5_reg().read(index.into())
    }

    #[doc="Write the CCBUF_DITH5 register."]
    #[inline] pub fn write_ccbuf_dith5<I: Into<::bobbin_bits::R6>>(&self, index: I, value: CcbufDith5) -> &Self {
        self.ccbuf_dith5_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CCBUF_DITH5 register."]
    #[inline] pub fn set_ccbuf_dith5<I: Into<::bobbin_bits::R6>, F: FnOnce(CcbufDith5) -> CcbufDith5>(&self, index: I, f: F) -> &Self {
        self.ccbuf_dith5_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CCBUF_DITH5 register."]
    #[inline] pub fn with_ccbuf_dith5<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(CcbufDith5) -> CcbufDith5>(&self, index: I, f: F) -> &Self {
        self.ccbuf_dith5_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CCBUF_DITH6 Register."]
    #[inline] pub fn ccbuf_dith6_reg(&self) -> ::bobbin_mcu::register::RegisterArray<CcbufDith6, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut CcbufDith6, 0x70, 0x4)
    }

    #[doc="Get the *mut pointer for the CCBUF_DITH6 register."]
    #[inline] pub fn ccbuf_dith6_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut CcbufDith6 { 
        self.ccbuf_dith6_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CCBUF_DITH6 register."]
    #[inline] pub fn ccbuf_dith6_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const CcbufDith6 { 
        self.ccbuf_dith6_reg().ptr(index.into())
    }

    #[doc="Read the CCBUF_DITH6 register."]
    #[inline] pub fn ccbuf_dith6<I: Into<::bobbin_bits::R6>>(&self, index: I) -> CcbufDith6 { 
        self.ccbuf_dith6_reg().read(index.into())
    }

    #[doc="Write the CCBUF_DITH6 register."]
    #[inline] pub fn write_ccbuf_dith6<I: Into<::bobbin_bits::R6>>(&self, index: I, value: CcbufDith6) -> &Self {
        self.ccbuf_dith6_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CCBUF_DITH6 register."]
    #[inline] pub fn set_ccbuf_dith6<I: Into<::bobbin_bits::R6>, F: FnOnce(CcbufDith6) -> CcbufDith6>(&self, index: I, f: F) -> &Self {
        self.ccbuf_dith6_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CCBUF_DITH6 register."]
    #[inline] pub fn with_ccbuf_dith6<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(CcbufDith6) -> CcbufDith6>(&self, index: I, f: F) -> &Self {
        self.ccbuf_dith6_reg().with(index.into(), f);
        self
    }

}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u32);
impl Ctrla {
    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable"]
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

    #[doc="Enhanced Resolution"]
    #[inline] pub fn resolution(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if RESOLUTION != 0"]
    #[inline] pub fn test_resolution(&self) -> bool {
        self.resolution() != 0
    }

    #[doc="Sets the RESOLUTION field."]
    #[inline] pub fn set_resolution<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Prescaler"]
    #[inline] pub fn prescaler(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Prescaler and Counter Synchronization Selection"]
    #[inline] pub fn prescsync(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if PRESCSYNC != 0"]
    #[inline] pub fn test_prescsync(&self) -> bool {
        self.prescsync() != 0
    }

    #[doc="Sets the PRESCSYNC field."]
    #[inline] pub fn set_prescsync<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Auto Lock"]
    #[inline] pub fn alock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ALOCK != 0"]
    #[inline] pub fn test_alock(&self) -> bool {
        self.alock() != 0
    }

    #[doc="Sets the ALOCK field."]
    #[inline] pub fn set_alock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Master Synchronization (only for TCC Slave Instance)"]
    #[inline] pub fn msync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if MSYNC != 0"]
    #[inline] pub fn test_msync(&self) -> bool {
        self.msync() != 0
    }

    #[doc="Sets the MSYNC field."]
    #[inline] pub fn set_msync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="DMA One-shot Trigger Mode"]
    #[inline] pub fn dmaos(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if DMAOS != 0"]
    #[inline] pub fn test_dmaos(&self) -> bool {
        self.dmaos() != 0
    }

    #[doc="Sets the DMAOS field."]
    #[inline] pub fn set_dmaos<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Capture Channel 0 Enable"]
    #[inline] pub fn cpten0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CPTEN0 != 0"]
    #[inline] pub fn test_cpten0(&self) -> bool {
        self.cpten0() != 0
    }

    #[doc="Sets the CPTEN0 field."]
    #[inline] pub fn set_cpten0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Capture Channel 1 Enable"]
    #[inline] pub fn cpten1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CPTEN1 != 0"]
    #[inline] pub fn test_cpten1(&self) -> bool {
        self.cpten1() != 0
    }

    #[doc="Sets the CPTEN1 field."]
    #[inline] pub fn set_cpten1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Capture Channel 2 Enable"]
    #[inline] pub fn cpten2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CPTEN2 != 0"]
    #[inline] pub fn test_cpten2(&self) -> bool {
        self.cpten2() != 0
    }

    #[doc="Sets the CPTEN2 field."]
    #[inline] pub fn set_cpten2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Capture Channel 3 Enable"]
    #[inline] pub fn cpten3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if CPTEN3 != 0"]
    #[inline] pub fn test_cpten3(&self) -> bool {
        self.cpten3() != 0
    }

    #[doc="Sets the CPTEN3 field."]
    #[inline] pub fn set_cpten3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Capture Channel 4 Enable"]
    #[inline] pub fn cpten4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if CPTEN4 != 0"]
    #[inline] pub fn test_cpten4(&self) -> bool {
        self.cpten4() != 0
    }

    #[doc="Sets the CPTEN4 field."]
    #[inline] pub fn set_cpten4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Capture Channel 5 Enable"]
    #[inline] pub fn cpten5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if CPTEN5 != 0"]
    #[inline] pub fn test_cpten5(&self) -> bool {
        self.cpten5() != 0
    }

    #[doc="Sets the CPTEN5 field."]
    #[inline] pub fn set_cpten5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Ctrla {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.resolution() != 0 { try!(write!(f, " resolution=0x{:x}", self.resolution()))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
        if self.alock() != 0 { try!(write!(f, " alock"))}
        if self.msync() != 0 { try!(write!(f, " msync"))}
        if self.dmaos() != 0 { try!(write!(f, " dmaos"))}
        if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
        if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
        if self.cpten2() != 0 { try!(write!(f, " cpten2"))}
        if self.cpten3() != 0 { try!(write!(f, " cpten3"))}
        if self.cpten4() != 0 { try!(write!(f, " cpten4"))}
        if self.cpten5() != 0 { try!(write!(f, " cpten5"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
    #[doc="Counter Direction"]
    #[inline] pub fn dir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Lock Update"]
    #[inline] pub fn lupd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LUPD != 0"]
    #[inline] pub fn test_lupd(&self) -> bool {
        self.lupd() != 0
    }

    #[doc="Sets the LUPD field."]
    #[inline] pub fn set_lupd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="One-Shot"]
    #[inline] pub fn oneshot(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ONESHOT != 0"]
    #[inline] pub fn test_oneshot(&self) -> bool {
        self.oneshot() != 0
    }

    #[doc="Sets the ONESHOT field."]
    #[inline] pub fn set_oneshot<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Ramp Index Command"]
    #[inline] pub fn idxcmd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if IDXCMD != 0"]
    #[inline] pub fn test_idxcmd(&self) -> bool {
        self.idxcmd() != 0
    }

    #[doc="Sets the IDXCMD field."]
    #[inline] pub fn set_idxcmd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TCC Command"]
    #[inline] pub fn cmd(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Ctrlbclr {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlbclr(other)
    }
}

impl ::core::fmt::Display for Ctrlbclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlbclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.lupd() != 0 { try!(write!(f, " lupd"))}
        if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
        if self.idxcmd() != 0 { try!(write!(f, " idxcmd=0x{:x}", self.idxcmd()))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
    #[doc="Counter Direction"]
    #[inline] pub fn dir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Lock Update"]
    #[inline] pub fn lupd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LUPD != 0"]
    #[inline] pub fn test_lupd(&self) -> bool {
        self.lupd() != 0
    }

    #[doc="Sets the LUPD field."]
    #[inline] pub fn set_lupd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="One-Shot"]
    #[inline] pub fn oneshot(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ONESHOT != 0"]
    #[inline] pub fn test_oneshot(&self) -> bool {
        self.oneshot() != 0
    }

    #[doc="Sets the ONESHOT field."]
    #[inline] pub fn set_oneshot<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Ramp Index Command"]
    #[inline] pub fn idxcmd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if IDXCMD != 0"]
    #[inline] pub fn test_idxcmd(&self) -> bool {
        self.idxcmd() != 0
    }

    #[doc="Sets the IDXCMD field."]
    #[inline] pub fn set_idxcmd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TCC Command"]
    #[inline] pub fn cmd(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Ctrlbset {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlbset(other)
    }
}

impl ::core::fmt::Display for Ctrlbset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlbset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.lupd() != 0 { try!(write!(f, " lupd"))}
        if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
        if self.idxcmd() != 0 { try!(write!(f, " idxcmd=0x{:x}", self.idxcmd()))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="Swrst Busy"]
    #[inline] pub fn swrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable Busy"]
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

    #[doc="Ctrlb Busy"]
    #[inline] pub fn ctrlb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CTRLB != 0"]
    #[inline] pub fn test_ctrlb(&self) -> bool {
        self.ctrlb() != 0
    }

    #[doc="Sets the CTRLB field."]
    #[inline] pub fn set_ctrlb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Status Busy"]
    #[inline] pub fn status(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STATUS != 0"]
    #[inline] pub fn test_status(&self) -> bool {
        self.status() != 0
    }

    #[doc="Sets the STATUS field."]
    #[inline] pub fn set_status<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Count Busy"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pattern Busy"]
    #[inline] pub fn patt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PATT != 0"]
    #[inline] pub fn test_patt(&self) -> bool {
        self.patt() != 0
    }

    #[doc="Sets the PATT field."]
    #[inline] pub fn set_patt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Wave Busy"]
    #[inline] pub fn wave(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if WAVE != 0"]
    #[inline] pub fn test_wave(&self) -> bool {
        self.wave() != 0
    }

    #[doc="Sets the WAVE field."]
    #[inline] pub fn set_wave<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Period Busy"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Compare Channel 0 Busy"]
    #[inline] pub fn cc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CC0 != 0"]
    #[inline] pub fn test_cc0(&self) -> bool {
        self.cc0() != 0
    }

    #[doc="Sets the CC0 field."]
    #[inline] pub fn set_cc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Compare Channel 1 Busy"]
    #[inline] pub fn cc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CC1 != 0"]
    #[inline] pub fn test_cc1(&self) -> bool {
        self.cc1() != 0
    }

    #[doc="Sets the CC1 field."]
    #[inline] pub fn set_cc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Compare Channel 2 Busy"]
    #[inline] pub fn cc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CC2 != 0"]
    #[inline] pub fn test_cc2(&self) -> bool {
        self.cc2() != 0
    }

    #[doc="Sets the CC2 field."]
    #[inline] pub fn set_cc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Compare Channel 3 Busy"]
    #[inline] pub fn cc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CC3 != 0"]
    #[inline] pub fn test_cc3(&self) -> bool {
        self.cc3() != 0
    }

    #[doc="Sets the CC3 field."]
    #[inline] pub fn set_cc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Compare Channel 4 Busy"]
    #[inline] pub fn cc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CC4 != 0"]
    #[inline] pub fn test_cc4(&self) -> bool {
        self.cc4() != 0
    }

    #[doc="Sets the CC4 field."]
    #[inline] pub fn set_cc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Compare Channel 5 Busy"]
    #[inline] pub fn cc5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CC5 != 0"]
    #[inline] pub fn test_cc5(&self) -> bool {
        self.cc5() != 0
    }

    #[doc="Sets the CC5 field."]
    #[inline] pub fn set_cc5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Syncbusy {
    #[inline]
    fn from(other: u32) -> Self {
         Syncbusy(other)
    }
}

impl ::core::fmt::Display for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Syncbusy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.ctrlb() != 0 { try!(write!(f, " ctrlb"))}
        if self.status() != 0 { try!(write!(f, " status"))}
        if self.count() != 0 { try!(write!(f, " count"))}
        if self.patt() != 0 { try!(write!(f, " patt"))}
        if self.wave() != 0 { try!(write!(f, " wave"))}
        if self.per() != 0 { try!(write!(f, " per"))}
        if self.cc0() != 0 { try!(write!(f, " cc0"))}
        if self.cc1() != 0 { try!(write!(f, " cc1"))}
        if self.cc2() != 0 { try!(write!(f, " cc2"))}
        if self.cc3() != 0 { try!(write!(f, " cc3"))}
        if self.cc4() != 0 { try!(write!(f, " cc4"))}
        if self.cc5() != 0 { try!(write!(f, " cc5"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Recoverable Fault A Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fctrla(pub u32);
impl Fctrla {
    #[doc="Fault A Source"]
    #[inline] pub fn src(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SRC != 0"]
    #[inline] pub fn test_src(&self) -> bool {
        self.src() != 0
    }

    #[doc="Sets the SRC field."]
    #[inline] pub fn set_src<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fault A Keeper"]
    #[inline] pub fn keep(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if KEEP != 0"]
    #[inline] pub fn test_keep(&self) -> bool {
        self.keep() != 0
    }

    #[doc="Sets the KEEP field."]
    #[inline] pub fn set_keep<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Fault A Qualification"]
    #[inline] pub fn qual(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if QUAL != 0"]
    #[inline] pub fn test_qual(&self) -> bool {
        self.qual() != 0
    }

    #[doc="Sets the QUAL field."]
    #[inline] pub fn set_qual<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Fault A Blanking Mode"]
    #[inline] pub fn blank(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if BLANK != 0"]
    #[inline] pub fn test_blank(&self) -> bool {
        self.blank() != 0
    }

    #[doc="Sets the BLANK field."]
    #[inline] pub fn set_blank<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Fault A Restart"]
    #[inline] pub fn restart(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RESTART != 0"]
    #[inline] pub fn test_restart(&self) -> bool {
        self.restart() != 0
    }

    #[doc="Sets the RESTART field."]
    #[inline] pub fn set_restart<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Fault A Halt Mode"]
    #[inline] pub fn halt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if HALT != 0"]
    #[inline] pub fn test_halt(&self) -> bool {
        self.halt() != 0
    }

    #[doc="Sets the HALT field."]
    #[inline] pub fn set_halt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Fault A Capture Channel"]
    #[inline] pub fn chsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if CHSEL != 0"]
    #[inline] pub fn test_chsel(&self) -> bool {
        self.chsel() != 0
    }

    #[doc="Sets the CHSEL field."]
    #[inline] pub fn set_chsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Fault A Capture Action"]
    #[inline] pub fn capture(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if CAPTURE != 0"]
    #[inline] pub fn test_capture(&self) -> bool {
        self.capture() != 0
    }

    #[doc="Sets the CAPTURE field."]
    #[inline] pub fn set_capture<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Fault A Blanking Prescaler"]
    #[inline] pub fn blankpresc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BLANKPRESC != 0"]
    #[inline] pub fn test_blankpresc(&self) -> bool {
        self.blankpresc() != 0
    }

    #[doc="Sets the BLANKPRESC field."]
    #[inline] pub fn set_blankpresc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Fault A Blanking Time"]
    #[inline] pub fn blankval(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if BLANKVAL != 0"]
    #[inline] pub fn test_blankval(&self) -> bool {
        self.blankval() != 0
    }

    #[doc="Sets the BLANKVAL field."]
    #[inline] pub fn set_blankval<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Fault A Filter Value"]
    #[inline] pub fn filterval(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FILTERVAL != 0"]
    #[inline] pub fn test_filterval(&self) -> bool {
        self.filterval() != 0
    }

    #[doc="Sets the FILTERVAL field."]
    #[inline] pub fn set_filterval<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Fctrla {
    #[inline]
    fn from(other: u32) -> Self {
         Fctrla(other)
    }
}

impl ::core::fmt::Display for Fctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
        if self.keep() != 0 { try!(write!(f, " keep"))}
        if self.qual() != 0 { try!(write!(f, " qual"))}
        if self.blank() != 0 { try!(write!(f, " blank=0x{:x}", self.blank()))}
        if self.restart() != 0 { try!(write!(f, " restart"))}
        if self.halt() != 0 { try!(write!(f, " halt=0x{:x}", self.halt()))}
        if self.chsel() != 0 { try!(write!(f, " chsel=0x{:x}", self.chsel()))}
        if self.capture() != 0 { try!(write!(f, " capture=0x{:x}", self.capture()))}
        if self.blankpresc() != 0 { try!(write!(f, " blankpresc"))}
        if self.blankval() != 0 { try!(write!(f, " blankval=0x{:x}", self.blankval()))}
        if self.filterval() != 0 { try!(write!(f, " filterval=0x{:x}", self.filterval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Recoverable Fault B Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fctrlb(pub u32);
impl Fctrlb {
    #[doc="Fault B Source"]
    #[inline] pub fn src(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SRC != 0"]
    #[inline] pub fn test_src(&self) -> bool {
        self.src() != 0
    }

    #[doc="Sets the SRC field."]
    #[inline] pub fn set_src<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fault B Keeper"]
    #[inline] pub fn keep(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if KEEP != 0"]
    #[inline] pub fn test_keep(&self) -> bool {
        self.keep() != 0
    }

    #[doc="Sets the KEEP field."]
    #[inline] pub fn set_keep<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Fault B Qualification"]
    #[inline] pub fn qual(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if QUAL != 0"]
    #[inline] pub fn test_qual(&self) -> bool {
        self.qual() != 0
    }

    #[doc="Sets the QUAL field."]
    #[inline] pub fn set_qual<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Fault B Blanking Mode"]
    #[inline] pub fn blank(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if BLANK != 0"]
    #[inline] pub fn test_blank(&self) -> bool {
        self.blank() != 0
    }

    #[doc="Sets the BLANK field."]
    #[inline] pub fn set_blank<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Fault B Restart"]
    #[inline] pub fn restart(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RESTART != 0"]
    #[inline] pub fn test_restart(&self) -> bool {
        self.restart() != 0
    }

    #[doc="Sets the RESTART field."]
    #[inline] pub fn set_restart<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Fault B Halt Mode"]
    #[inline] pub fn halt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if HALT != 0"]
    #[inline] pub fn test_halt(&self) -> bool {
        self.halt() != 0
    }

    #[doc="Sets the HALT field."]
    #[inline] pub fn set_halt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Fault B Capture Channel"]
    #[inline] pub fn chsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if CHSEL != 0"]
    #[inline] pub fn test_chsel(&self) -> bool {
        self.chsel() != 0
    }

    #[doc="Sets the CHSEL field."]
    #[inline] pub fn set_chsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Fault B Capture Action"]
    #[inline] pub fn capture(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if CAPTURE != 0"]
    #[inline] pub fn test_capture(&self) -> bool {
        self.capture() != 0
    }

    #[doc="Sets the CAPTURE field."]
    #[inline] pub fn set_capture<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Fault B Blanking Prescaler"]
    #[inline] pub fn blankpresc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BLANKPRESC != 0"]
    #[inline] pub fn test_blankpresc(&self) -> bool {
        self.blankpresc() != 0
    }

    #[doc="Sets the BLANKPRESC field."]
    #[inline] pub fn set_blankpresc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Fault B Blanking Time"]
    #[inline] pub fn blankval(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if BLANKVAL != 0"]
    #[inline] pub fn test_blankval(&self) -> bool {
        self.blankval() != 0
    }

    #[doc="Sets the BLANKVAL field."]
    #[inline] pub fn set_blankval<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Fault B Filter Value"]
    #[inline] pub fn filterval(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FILTERVAL != 0"]
    #[inline] pub fn test_filterval(&self) -> bool {
        self.filterval() != 0
    }

    #[doc="Sets the FILTERVAL field."]
    #[inline] pub fn set_filterval<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Fctrlb {
    #[inline]
    fn from(other: u32) -> Self {
         Fctrlb(other)
    }
}

impl ::core::fmt::Display for Fctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
        if self.keep() != 0 { try!(write!(f, " keep"))}
        if self.qual() != 0 { try!(write!(f, " qual"))}
        if self.blank() != 0 { try!(write!(f, " blank=0x{:x}", self.blank()))}
        if self.restart() != 0 { try!(write!(f, " restart"))}
        if self.halt() != 0 { try!(write!(f, " halt=0x{:x}", self.halt()))}
        if self.chsel() != 0 { try!(write!(f, " chsel=0x{:x}", self.chsel()))}
        if self.capture() != 0 { try!(write!(f, " capture=0x{:x}", self.capture()))}
        if self.blankpresc() != 0 { try!(write!(f, " blankpresc"))}
        if self.blankval() != 0 { try!(write!(f, " blankval=0x{:x}", self.blankval()))}
        if self.filterval() != 0 { try!(write!(f, " filterval=0x{:x}", self.filterval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Waveform Extension Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wexctrl(pub u32);
impl Wexctrl {
    #[doc="Output Matrix"]
    #[inline] pub fn otmx(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if OTMX != 0"]
    #[inline] pub fn test_otmx(&self) -> bool {
        self.otmx() != 0
    }

    #[doc="Sets the OTMX field."]
    #[inline] pub fn set_otmx<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Dead-time Insertion Generator 0 Enable"]
    #[inline] pub fn dtien0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DTIEN0 != 0"]
    #[inline] pub fn test_dtien0(&self) -> bool {
        self.dtien0() != 0
    }

    #[doc="Sets the DTIEN0 field."]
    #[inline] pub fn set_dtien0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Dead-time Insertion Generator 1 Enable"]
    #[inline] pub fn dtien1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DTIEN1 != 0"]
    #[inline] pub fn test_dtien1(&self) -> bool {
        self.dtien1() != 0
    }

    #[doc="Sets the DTIEN1 field."]
    #[inline] pub fn set_dtien1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Dead-time Insertion Generator 2 Enable"]
    #[inline] pub fn dtien2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTIEN2 != 0"]
    #[inline] pub fn test_dtien2(&self) -> bool {
        self.dtien2() != 0
    }

    #[doc="Sets the DTIEN2 field."]
    #[inline] pub fn set_dtien2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Dead-time Insertion Generator 3 Enable"]
    #[inline] pub fn dtien3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DTIEN3 != 0"]
    #[inline] pub fn test_dtien3(&self) -> bool {
        self.dtien3() != 0
    }

    #[doc="Sets the DTIEN3 field."]
    #[inline] pub fn set_dtien3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Dead-time Low Side Outputs Value"]
    #[inline] pub fn dtls(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DTLS != 0"]
    #[inline] pub fn test_dtls(&self) -> bool {
        self.dtls() != 0
    }

    #[doc="Sets the DTLS field."]
    #[inline] pub fn set_dtls<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Dead-time High Side Outputs Value"]
    #[inline] pub fn dths(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DTHS != 0"]
    #[inline] pub fn test_dths(&self) -> bool {
        self.dths() != 0
    }

    #[doc="Sets the DTHS field."]
    #[inline] pub fn set_dths<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Wexctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Wexctrl(other)
    }
}

impl ::core::fmt::Display for Wexctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wexctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.otmx() != 0 { try!(write!(f, " otmx=0x{:x}", self.otmx()))}
        if self.dtien0() != 0 { try!(write!(f, " dtien0"))}
        if self.dtien1() != 0 { try!(write!(f, " dtien1"))}
        if self.dtien2() != 0 { try!(write!(f, " dtien2"))}
        if self.dtien3() != 0 { try!(write!(f, " dtien3"))}
        if self.dtls() != 0 { try!(write!(f, " dtls=0x{:x}", self.dtls()))}
        if self.dths() != 0 { try!(write!(f, " dths=0x{:x}", self.dths()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Driver Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Drvctrl(pub u32);
impl Drvctrl {
    #[doc="Non-Recoverable State 0 Output Enable"]
    #[inline] pub fn nre0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if NRE0 != 0"]
    #[inline] pub fn test_nre0(&self) -> bool {
        self.nre0() != 0
    }

    #[doc="Sets the NRE0 field."]
    #[inline] pub fn set_nre0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Non-Recoverable State 1 Output Enable"]
    #[inline] pub fn nre1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if NRE1 != 0"]
    #[inline] pub fn test_nre1(&self) -> bool {
        self.nre1() != 0
    }

    #[doc="Sets the NRE1 field."]
    #[inline] pub fn set_nre1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Non-Recoverable State 2 Output Enable"]
    #[inline] pub fn nre2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NRE2 != 0"]
    #[inline] pub fn test_nre2(&self) -> bool {
        self.nre2() != 0
    }

    #[doc="Sets the NRE2 field."]
    #[inline] pub fn set_nre2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Non-Recoverable State 3 Output Enable"]
    #[inline] pub fn nre3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if NRE3 != 0"]
    #[inline] pub fn test_nre3(&self) -> bool {
        self.nre3() != 0
    }

    #[doc="Sets the NRE3 field."]
    #[inline] pub fn set_nre3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Non-Recoverable State 4 Output Enable"]
    #[inline] pub fn nre4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NRE4 != 0"]
    #[inline] pub fn test_nre4(&self) -> bool {
        self.nre4() != 0
    }

    #[doc="Sets the NRE4 field."]
    #[inline] pub fn set_nre4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Non-Recoverable State 5 Output Enable"]
    #[inline] pub fn nre5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if NRE5 != 0"]
    #[inline] pub fn test_nre5(&self) -> bool {
        self.nre5() != 0
    }

    #[doc="Sets the NRE5 field."]
    #[inline] pub fn set_nre5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Non-Recoverable State 6 Output Enable"]
    #[inline] pub fn nre6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if NRE6 != 0"]
    #[inline] pub fn test_nre6(&self) -> bool {
        self.nre6() != 0
    }

    #[doc="Sets the NRE6 field."]
    #[inline] pub fn set_nre6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Non-Recoverable State 7 Output Enable"]
    #[inline] pub fn nre7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NRE7 != 0"]
    #[inline] pub fn test_nre7(&self) -> bool {
        self.nre7() != 0
    }

    #[doc="Sets the NRE7 field."]
    #[inline] pub fn set_nre7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Non-Recoverable State 0 Output Value"]
    #[inline] pub fn nrv0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if NRV0 != 0"]
    #[inline] pub fn test_nrv0(&self) -> bool {
        self.nrv0() != 0
    }

    #[doc="Sets the NRV0 field."]
    #[inline] pub fn set_nrv0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Non-Recoverable State 1 Output Value"]
    #[inline] pub fn nrv1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if NRV1 != 0"]
    #[inline] pub fn test_nrv1(&self) -> bool {
        self.nrv1() != 0
    }

    #[doc="Sets the NRV1 field."]
    #[inline] pub fn set_nrv1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Non-Recoverable State 2 Output Value"]
    #[inline] pub fn nrv2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if NRV2 != 0"]
    #[inline] pub fn test_nrv2(&self) -> bool {
        self.nrv2() != 0
    }

    #[doc="Sets the NRV2 field."]
    #[inline] pub fn set_nrv2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Non-Recoverable State 3 Output Value"]
    #[inline] pub fn nrv3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if NRV3 != 0"]
    #[inline] pub fn test_nrv3(&self) -> bool {
        self.nrv3() != 0
    }

    #[doc="Sets the NRV3 field."]
    #[inline] pub fn set_nrv3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Non-Recoverable State 4 Output Value"]
    #[inline] pub fn nrv4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if NRV4 != 0"]
    #[inline] pub fn test_nrv4(&self) -> bool {
        self.nrv4() != 0
    }

    #[doc="Sets the NRV4 field."]
    #[inline] pub fn set_nrv4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Non-Recoverable State 5 Output Value"]
    #[inline] pub fn nrv5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if NRV5 != 0"]
    #[inline] pub fn test_nrv5(&self) -> bool {
        self.nrv5() != 0
    }

    #[doc="Sets the NRV5 field."]
    #[inline] pub fn set_nrv5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Non-Recoverable State 6 Output Value"]
    #[inline] pub fn nrv6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if NRV6 != 0"]
    #[inline] pub fn test_nrv6(&self) -> bool {
        self.nrv6() != 0
    }

    #[doc="Sets the NRV6 field."]
    #[inline] pub fn set_nrv6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Non-Recoverable State 7 Output Value"]
    #[inline] pub fn nrv7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if NRV7 != 0"]
    #[inline] pub fn test_nrv7(&self) -> bool {
        self.nrv7() != 0
    }

    #[doc="Sets the NRV7 field."]
    #[inline] pub fn set_nrv7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Output Waveform 0 Inversion"]
    #[inline] pub fn inven0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if INVEN0 != 0"]
    #[inline] pub fn test_inven0(&self) -> bool {
        self.inven0() != 0
    }

    #[doc="Sets the INVEN0 field."]
    #[inline] pub fn set_inven0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Output Waveform 1 Inversion"]
    #[inline] pub fn inven1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if INVEN1 != 0"]
    #[inline] pub fn test_inven1(&self) -> bool {
        self.inven1() != 0
    }

    #[doc="Sets the INVEN1 field."]
    #[inline] pub fn set_inven1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Output Waveform 2 Inversion"]
    #[inline] pub fn inven2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if INVEN2 != 0"]
    #[inline] pub fn test_inven2(&self) -> bool {
        self.inven2() != 0
    }

    #[doc="Sets the INVEN2 field."]
    #[inline] pub fn set_inven2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Output Waveform 3 Inversion"]
    #[inline] pub fn inven3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if INVEN3 != 0"]
    #[inline] pub fn test_inven3(&self) -> bool {
        self.inven3() != 0
    }

    #[doc="Sets the INVEN3 field."]
    #[inline] pub fn set_inven3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Output Waveform 4 Inversion"]
    #[inline] pub fn inven4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if INVEN4 != 0"]
    #[inline] pub fn test_inven4(&self) -> bool {
        self.inven4() != 0
    }

    #[doc="Sets the INVEN4 field."]
    #[inline] pub fn set_inven4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Output Waveform 5 Inversion"]
    #[inline] pub fn inven5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if INVEN5 != 0"]
    #[inline] pub fn test_inven5(&self) -> bool {
        self.inven5() != 0
    }

    #[doc="Sets the INVEN5 field."]
    #[inline] pub fn set_inven5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Output Waveform 6 Inversion"]
    #[inline] pub fn inven6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if INVEN6 != 0"]
    #[inline] pub fn test_inven6(&self) -> bool {
        self.inven6() != 0
    }

    #[doc="Sets the INVEN6 field."]
    #[inline] pub fn set_inven6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Output Waveform 7 Inversion"]
    #[inline] pub fn inven7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if INVEN7 != 0"]
    #[inline] pub fn test_inven7(&self) -> bool {
        self.inven7() != 0
    }

    #[doc="Sets the INVEN7 field."]
    #[inline] pub fn set_inven7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Non-Recoverable Fault Input 0 Filter Value"]
    #[inline] pub fn filterval0(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FILTERVAL0 != 0"]
    #[inline] pub fn test_filterval0(&self) -> bool {
        self.filterval0() != 0
    }

    #[doc="Sets the FILTERVAL0 field."]
    #[inline] pub fn set_filterval0<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Non-Recoverable Fault Input 1 Filter Value"]
    #[inline] pub fn filterval1(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if FILTERVAL1 != 0"]
    #[inline] pub fn test_filterval1(&self) -> bool {
        self.filterval1() != 0
    }

    #[doc="Sets the FILTERVAL1 field."]
    #[inline] pub fn set_filterval1<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Drvctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Drvctrl(other)
    }
}

impl ::core::fmt::Display for Drvctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Drvctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nre0() != 0 { try!(write!(f, " nre0"))}
        if self.nre1() != 0 { try!(write!(f, " nre1"))}
        if self.nre2() != 0 { try!(write!(f, " nre2"))}
        if self.nre3() != 0 { try!(write!(f, " nre3"))}
        if self.nre4() != 0 { try!(write!(f, " nre4"))}
        if self.nre5() != 0 { try!(write!(f, " nre5"))}
        if self.nre6() != 0 { try!(write!(f, " nre6"))}
        if self.nre7() != 0 { try!(write!(f, " nre7"))}
        if self.nrv0() != 0 { try!(write!(f, " nrv0"))}
        if self.nrv1() != 0 { try!(write!(f, " nrv1"))}
        if self.nrv2() != 0 { try!(write!(f, " nrv2"))}
        if self.nrv3() != 0 { try!(write!(f, " nrv3"))}
        if self.nrv4() != 0 { try!(write!(f, " nrv4"))}
        if self.nrv5() != 0 { try!(write!(f, " nrv5"))}
        if self.nrv6() != 0 { try!(write!(f, " nrv6"))}
        if self.nrv7() != 0 { try!(write!(f, " nrv7"))}
        if self.inven0() != 0 { try!(write!(f, " inven0"))}
        if self.inven1() != 0 { try!(write!(f, " inven1"))}
        if self.inven2() != 0 { try!(write!(f, " inven2"))}
        if self.inven3() != 0 { try!(write!(f, " inven3"))}
        if self.inven4() != 0 { try!(write!(f, " inven4"))}
        if self.inven5() != 0 { try!(write!(f, " inven5"))}
        if self.inven6() != 0 { try!(write!(f, " inven6"))}
        if self.inven7() != 0 { try!(write!(f, " inven7"))}
        if self.filterval0() != 0 { try!(write!(f, " filterval0=0x{:x}", self.filterval0()))}
        if self.filterval1() != 0 { try!(write!(f, " filterval1=0x{:x}", self.filterval1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Running Mode"]
    #[inline] pub fn dbgrun(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGRUN != 0"]
    #[inline] pub fn test_dbgrun(&self) -> bool {
        self.dbgrun() != 0
    }

    #[doc="Sets the DBGRUN field."]
    #[inline] pub fn set_dbgrun<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fault Detection on Debug Break Detection"]
    #[inline] pub fn fddbd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FDDBD != 0"]
    #[inline] pub fn test_fddbd(&self) -> bool {
        self.fddbd() != 0
    }

    #[doc="Sets the FDDBD field."]
    #[inline] pub fn set_fddbd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
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
        if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
        if self.fddbd() != 0 { try!(write!(f, " fddbd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u32);
impl Evctrl {
    #[doc="Timer/counter Input Event0 Action"]
    #[inline] pub fn evact0(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EVACT0 != 0"]
    #[inline] pub fn test_evact0(&self) -> bool {
        self.evact0() != 0
    }

    #[doc="Sets the EVACT0 field."]
    #[inline] pub fn set_evact0<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer/counter Input Event1 Action"]
    #[inline] pub fn evact1(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if EVACT1 != 0"]
    #[inline] pub fn test_evact1(&self) -> bool {
        self.evact1() != 0
    }

    #[doc="Sets the EVACT1 field."]
    #[inline] pub fn set_evact1<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Timer/counter Output Event Mode"]
    #[inline] pub fn cntsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CNTSEL != 0"]
    #[inline] pub fn test_cntsel(&self) -> bool {
        self.cntsel() != 0
    }

    #[doc="Sets the CNTSEL field."]
    #[inline] pub fn set_cntsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Overflow/Underflow Output Event Enable"]
    #[inline] pub fn ovfeo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVFEO != 0"]
    #[inline] pub fn test_ovfeo(&self) -> bool {
        self.ovfeo() != 0
    }

    #[doc="Sets the OVFEO field."]
    #[inline] pub fn set_ovfeo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Retrigger Output Event Enable"]
    #[inline] pub fn trgeo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TRGEO != 0"]
    #[inline] pub fn test_trgeo(&self) -> bool {
        self.trgeo() != 0
    }

    #[doc="Sets the TRGEO field."]
    #[inline] pub fn set_trgeo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Timer/counter Output Event Enable"]
    #[inline] pub fn cnteo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CNTEO != 0"]
    #[inline] pub fn test_cnteo(&self) -> bool {
        self.cnteo() != 0
    }

    #[doc="Sets the CNTEO field."]
    #[inline] pub fn set_cnteo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Inverted Event 0 Input Enable"]
    #[inline] pub fn tcinv0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TCINV0 != 0"]
    #[inline] pub fn test_tcinv0(&self) -> bool {
        self.tcinv0() != 0
    }

    #[doc="Sets the TCINV0 field."]
    #[inline] pub fn set_tcinv0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Inverted Event 1 Input Enable"]
    #[inline] pub fn tcinv1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TCINV1 != 0"]
    #[inline] pub fn test_tcinv1(&self) -> bool {
        self.tcinv1() != 0
    }

    #[doc="Sets the TCINV1 field."]
    #[inline] pub fn set_tcinv1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Timer/counter Event 0 Input Enable"]
    #[inline] pub fn tcei0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TCEI0 != 0"]
    #[inline] pub fn test_tcei0(&self) -> bool {
        self.tcei0() != 0
    }

    #[doc="Sets the TCEI0 field."]
    #[inline] pub fn set_tcei0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Timer/counter Event 1 Input Enable"]
    #[inline] pub fn tcei1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TCEI1 != 0"]
    #[inline] pub fn test_tcei1(&self) -> bool {
        self.tcei1() != 0
    }

    #[doc="Sets the TCEI1 field."]
    #[inline] pub fn set_tcei1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Match or Capture Channel 0 Event Input Enable"]
    #[inline] pub fn mcei0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if MCEI0 != 0"]
    #[inline] pub fn test_mcei0(&self) -> bool {
        self.mcei0() != 0
    }

    #[doc="Sets the MCEI0 field."]
    #[inline] pub fn set_mcei0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Match or Capture Channel 1 Event Input Enable"]
    #[inline] pub fn mcei1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if MCEI1 != 0"]
    #[inline] pub fn test_mcei1(&self) -> bool {
        self.mcei1() != 0
    }

    #[doc="Sets the MCEI1 field."]
    #[inline] pub fn set_mcei1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Match or Capture Channel 2 Event Input Enable"]
    #[inline] pub fn mcei2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if MCEI2 != 0"]
    #[inline] pub fn test_mcei2(&self) -> bool {
        self.mcei2() != 0
    }

    #[doc="Sets the MCEI2 field."]
    #[inline] pub fn set_mcei2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Match or Capture Channel 3 Event Input Enable"]
    #[inline] pub fn mcei3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if MCEI3 != 0"]
    #[inline] pub fn test_mcei3(&self) -> bool {
        self.mcei3() != 0
    }

    #[doc="Sets the MCEI3 field."]
    #[inline] pub fn set_mcei3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Match or Capture Channel 4 Event Input Enable"]
    #[inline] pub fn mcei4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if MCEI4 != 0"]
    #[inline] pub fn test_mcei4(&self) -> bool {
        self.mcei4() != 0
    }

    #[doc="Sets the MCEI4 field."]
    #[inline] pub fn set_mcei4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Match or Capture Channel 5 Event Input Enable"]
    #[inline] pub fn mcei5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if MCEI5 != 0"]
    #[inline] pub fn test_mcei5(&self) -> bool {
        self.mcei5() != 0
    }

    #[doc="Sets the MCEI5 field."]
    #[inline] pub fn set_mcei5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Match or Capture Channel 0 Event Output Enable"]
    #[inline] pub fn mceo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if MCEO0 != 0"]
    #[inline] pub fn test_mceo0(&self) -> bool {
        self.mceo0() != 0
    }

    #[doc="Sets the MCEO0 field."]
    #[inline] pub fn set_mceo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Match or Capture Channel 1 Event Output Enable"]
    #[inline] pub fn mceo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if MCEO1 != 0"]
    #[inline] pub fn test_mceo1(&self) -> bool {
        self.mceo1() != 0
    }

    #[doc="Sets the MCEO1 field."]
    #[inline] pub fn set_mceo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Match or Capture Channel 2 Event Output Enable"]
    #[inline] pub fn mceo2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if MCEO2 != 0"]
    #[inline] pub fn test_mceo2(&self) -> bool {
        self.mceo2() != 0
    }

    #[doc="Sets the MCEO2 field."]
    #[inline] pub fn set_mceo2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Match or Capture Channel 3 Event Output Enable"]
    #[inline] pub fn mceo3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if MCEO3 != 0"]
    #[inline] pub fn test_mceo3(&self) -> bool {
        self.mceo3() != 0
    }

    #[doc="Sets the MCEO3 field."]
    #[inline] pub fn set_mceo3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Match or Capture Channel 4 Event Output Enable"]
    #[inline] pub fn mceo4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if MCEO4 != 0"]
    #[inline] pub fn test_mceo4(&self) -> bool {
        self.mceo4() != 0
    }

    #[doc="Sets the MCEO4 field."]
    #[inline] pub fn set_mceo4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Match or Capture Channel 5 Event Output Enable"]
    #[inline] pub fn mceo5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if MCEO5 != 0"]
    #[inline] pub fn test_mceo5(&self) -> bool {
        self.mceo5() != 0
    }

    #[doc="Sets the MCEO5 field."]
    #[inline] pub fn set_mceo5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Evctrl {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.evact0() != 0 { try!(write!(f, " evact0=0x{:x}", self.evact0()))}
        if self.evact1() != 0 { try!(write!(f, " evact1=0x{:x}", self.evact1()))}
        if self.cntsel() != 0 { try!(write!(f, " cntsel=0x{:x}", self.cntsel()))}
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        if self.trgeo() != 0 { try!(write!(f, " trgeo"))}
        if self.cnteo() != 0 { try!(write!(f, " cnteo"))}
        if self.tcinv0() != 0 { try!(write!(f, " tcinv0"))}
        if self.tcinv1() != 0 { try!(write!(f, " tcinv1"))}
        if self.tcei0() != 0 { try!(write!(f, " tcei0"))}
        if self.tcei1() != 0 { try!(write!(f, " tcei1"))}
        if self.mcei0() != 0 { try!(write!(f, " mcei0"))}
        if self.mcei1() != 0 { try!(write!(f, " mcei1"))}
        if self.mcei2() != 0 { try!(write!(f, " mcei2"))}
        if self.mcei3() != 0 { try!(write!(f, " mcei3"))}
        if self.mcei4() != 0 { try!(write!(f, " mcei4"))}
        if self.mcei5() != 0 { try!(write!(f, " mcei5"))}
        if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
        if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
        if self.mceo2() != 0 { try!(write!(f, " mceo2"))}
        if self.mceo3() != 0 { try!(write!(f, " mceo3"))}
        if self.mceo4() != 0 { try!(write!(f, " mceo4"))}
        if self.mceo5() != 0 { try!(write!(f, " mceo5"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Retrigger Interrupt Enable"]
    #[inline] pub fn trg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRG != 0"]
    #[inline] pub fn test_trg(&self) -> bool {
        self.trg() != 0
    }

    #[doc="Sets the TRG field."]
    #[inline] pub fn set_trg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter Interrupt Enable"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Error Interrupt Enable"]
    #[inline] pub fn err(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Non-Recoverable Update Fault Interrupt Enable"]
    #[inline] pub fn ufs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if UFS != 0"]
    #[inline] pub fn test_ufs(&self) -> bool {
        self.ufs() != 0
    }

    #[doc="Sets the UFS field."]
    #[inline] pub fn set_ufs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Non-Recoverable Debug Fault Interrupt Enable"]
    #[inline] pub fn dfs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFS != 0"]
    #[inline] pub fn test_dfs(&self) -> bool {
        self.dfs() != 0
    }

    #[doc="Sets the DFS field."]
    #[inline] pub fn set_dfs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Recoverable Fault A Interrupt Enable"]
    #[inline] pub fn faulta(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FAULTA != 0"]
    #[inline] pub fn test_faulta(&self) -> bool {
        self.faulta() != 0
    }

    #[doc="Sets the FAULTA field."]
    #[inline] pub fn set_faulta<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Recoverable Fault B Interrupt Enable"]
    #[inline] pub fn faultb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FAULTB != 0"]
    #[inline] pub fn test_faultb(&self) -> bool {
        self.faultb() != 0
    }

    #[doc="Sets the FAULTB field."]
    #[inline] pub fn set_faultb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Non-Recoverable Fault 0 Interrupt Enable"]
    #[inline] pub fn fault0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FAULT0 != 0"]
    #[inline] pub fn test_fault0(&self) -> bool {
        self.fault0() != 0
    }

    #[doc="Sets the FAULT0 field."]
    #[inline] pub fn set_fault0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Non-Recoverable Fault 1 Interrupt Enable"]
    #[inline] pub fn fault1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FAULT1 != 0"]
    #[inline] pub fn test_fault1(&self) -> bool {
        self.fault1() != 0
    }

    #[doc="Sets the FAULT1 field."]
    #[inline] pub fn set_fault1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Match or Capture Channel 0 Interrupt Enable"]
    #[inline] pub fn mc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Match or Capture Channel 1 Interrupt Enable"]
    #[inline] pub fn mc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Match or Capture Channel 2 Interrupt Enable"]
    #[inline] pub fn mc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if MC2 != 0"]
    #[inline] pub fn test_mc2(&self) -> bool {
        self.mc2() != 0
    }

    #[doc="Sets the MC2 field."]
    #[inline] pub fn set_mc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Match or Capture Channel 3 Interrupt Enable"]
    #[inline] pub fn mc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if MC3 != 0"]
    #[inline] pub fn test_mc3(&self) -> bool {
        self.mc3() != 0
    }

    #[doc="Sets the MC3 field."]
    #[inline] pub fn set_mc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Match or Capture Channel 4 Interrupt Enable"]
    #[inline] pub fn mc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if MC4 != 0"]
    #[inline] pub fn test_mc4(&self) -> bool {
        self.mc4() != 0
    }

    #[doc="Sets the MC4 field."]
    #[inline] pub fn set_mc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Match or Capture Channel 5 Interrupt Enable"]
    #[inline] pub fn mc5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if MC5 != 0"]
    #[inline] pub fn test_mc5(&self) -> bool {
        self.mc5() != 0
    }

    #[doc="Sets the MC5 field."]
    #[inline] pub fn set_mc5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
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
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.trg() != 0 { try!(write!(f, " trg"))}
        if self.cnt() != 0 { try!(write!(f, " cnt"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.ufs() != 0 { try!(write!(f, " ufs"))}
        if self.dfs() != 0 { try!(write!(f, " dfs"))}
        if self.faulta() != 0 { try!(write!(f, " faulta"))}
        if self.faultb() != 0 { try!(write!(f, " faultb"))}
        if self.fault0() != 0 { try!(write!(f, " fault0"))}
        if self.fault1() != 0 { try!(write!(f, " fault1"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        if self.mc2() != 0 { try!(write!(f, " mc2"))}
        if self.mc3() != 0 { try!(write!(f, " mc3"))}
        if self.mc4() != 0 { try!(write!(f, " mc4"))}
        if self.mc5() != 0 { try!(write!(f, " mc5"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Retrigger Interrupt Enable"]
    #[inline] pub fn trg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRG != 0"]
    #[inline] pub fn test_trg(&self) -> bool {
        self.trg() != 0
    }

    #[doc="Sets the TRG field."]
    #[inline] pub fn set_trg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter Interrupt Enable"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Error Interrupt Enable"]
    #[inline] pub fn err(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Non-Recoverable Update Fault Interrupt Enable"]
    #[inline] pub fn ufs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if UFS != 0"]
    #[inline] pub fn test_ufs(&self) -> bool {
        self.ufs() != 0
    }

    #[doc="Sets the UFS field."]
    #[inline] pub fn set_ufs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Non-Recoverable Debug Fault Interrupt Enable"]
    #[inline] pub fn dfs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFS != 0"]
    #[inline] pub fn test_dfs(&self) -> bool {
        self.dfs() != 0
    }

    #[doc="Sets the DFS field."]
    #[inline] pub fn set_dfs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Recoverable Fault A Interrupt Enable"]
    #[inline] pub fn faulta(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FAULTA != 0"]
    #[inline] pub fn test_faulta(&self) -> bool {
        self.faulta() != 0
    }

    #[doc="Sets the FAULTA field."]
    #[inline] pub fn set_faulta<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Recoverable Fault B Interrupt Enable"]
    #[inline] pub fn faultb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FAULTB != 0"]
    #[inline] pub fn test_faultb(&self) -> bool {
        self.faultb() != 0
    }

    #[doc="Sets the FAULTB field."]
    #[inline] pub fn set_faultb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Non-Recoverable Fault 0 Interrupt Enable"]
    #[inline] pub fn fault0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FAULT0 != 0"]
    #[inline] pub fn test_fault0(&self) -> bool {
        self.fault0() != 0
    }

    #[doc="Sets the FAULT0 field."]
    #[inline] pub fn set_fault0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Non-Recoverable Fault 1 Interrupt Enable"]
    #[inline] pub fn fault1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FAULT1 != 0"]
    #[inline] pub fn test_fault1(&self) -> bool {
        self.fault1() != 0
    }

    #[doc="Sets the FAULT1 field."]
    #[inline] pub fn set_fault1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Match or Capture Channel 0 Interrupt Enable"]
    #[inline] pub fn mc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Match or Capture Channel 1 Interrupt Enable"]
    #[inline] pub fn mc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Match or Capture Channel 2 Interrupt Enable"]
    #[inline] pub fn mc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if MC2 != 0"]
    #[inline] pub fn test_mc2(&self) -> bool {
        self.mc2() != 0
    }

    #[doc="Sets the MC2 field."]
    #[inline] pub fn set_mc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Match or Capture Channel 3 Interrupt Enable"]
    #[inline] pub fn mc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if MC3 != 0"]
    #[inline] pub fn test_mc3(&self) -> bool {
        self.mc3() != 0
    }

    #[doc="Sets the MC3 field."]
    #[inline] pub fn set_mc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Match or Capture Channel 4 Interrupt Enable"]
    #[inline] pub fn mc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if MC4 != 0"]
    #[inline] pub fn test_mc4(&self) -> bool {
        self.mc4() != 0
    }

    #[doc="Sets the MC4 field."]
    #[inline] pub fn set_mc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Match or Capture Channel 5 Interrupt Enable"]
    #[inline] pub fn mc5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if MC5 != 0"]
    #[inline] pub fn test_mc5(&self) -> bool {
        self.mc5() != 0
    }

    #[doc="Sets the MC5 field."]
    #[inline] pub fn set_mc5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
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
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.trg() != 0 { try!(write!(f, " trg"))}
        if self.cnt() != 0 { try!(write!(f, " cnt"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.ufs() != 0 { try!(write!(f, " ufs"))}
        if self.dfs() != 0 { try!(write!(f, " dfs"))}
        if self.faulta() != 0 { try!(write!(f, " faulta"))}
        if self.faultb() != 0 { try!(write!(f, " faultb"))}
        if self.fault0() != 0 { try!(write!(f, " fault0"))}
        if self.fault1() != 0 { try!(write!(f, " fault1"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        if self.mc2() != 0 { try!(write!(f, " mc2"))}
        if self.mc3() != 0 { try!(write!(f, " mc3"))}
        if self.mc4() != 0 { try!(write!(f, " mc4"))}
        if self.mc5() != 0 { try!(write!(f, " mc5"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u32);
impl Intflag {
    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Retrigger"]
    #[inline] pub fn trg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRG != 0"]
    #[inline] pub fn test_trg(&self) -> bool {
        self.trg() != 0
    }

    #[doc="Sets the TRG field."]
    #[inline] pub fn set_trg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Counter"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Error"]
    #[inline] pub fn err(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Non-Recoverable Update Fault"]
    #[inline] pub fn ufs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if UFS != 0"]
    #[inline] pub fn test_ufs(&self) -> bool {
        self.ufs() != 0
    }

    #[doc="Sets the UFS field."]
    #[inline] pub fn set_ufs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Non-Recoverable Debug Fault"]
    #[inline] pub fn dfs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFS != 0"]
    #[inline] pub fn test_dfs(&self) -> bool {
        self.dfs() != 0
    }

    #[doc="Sets the DFS field."]
    #[inline] pub fn set_dfs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Recoverable Fault A"]
    #[inline] pub fn faulta(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FAULTA != 0"]
    #[inline] pub fn test_faulta(&self) -> bool {
        self.faulta() != 0
    }

    #[doc="Sets the FAULTA field."]
    #[inline] pub fn set_faulta<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Recoverable Fault B"]
    #[inline] pub fn faultb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FAULTB != 0"]
    #[inline] pub fn test_faultb(&self) -> bool {
        self.faultb() != 0
    }

    #[doc="Sets the FAULTB field."]
    #[inline] pub fn set_faultb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Non-Recoverable Fault 0"]
    #[inline] pub fn fault0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FAULT0 != 0"]
    #[inline] pub fn test_fault0(&self) -> bool {
        self.fault0() != 0
    }

    #[doc="Sets the FAULT0 field."]
    #[inline] pub fn set_fault0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Non-Recoverable Fault 1"]
    #[inline] pub fn fault1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FAULT1 != 0"]
    #[inline] pub fn test_fault1(&self) -> bool {
        self.fault1() != 0
    }

    #[doc="Sets the FAULT1 field."]
    #[inline] pub fn set_fault1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Match or Capture 0"]
    #[inline] pub fn mc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Match or Capture 1"]
    #[inline] pub fn mc1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Match or Capture 2"]
    #[inline] pub fn mc2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if MC2 != 0"]
    #[inline] pub fn test_mc2(&self) -> bool {
        self.mc2() != 0
    }

    #[doc="Sets the MC2 field."]
    #[inline] pub fn set_mc2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Match or Capture 3"]
    #[inline] pub fn mc3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if MC3 != 0"]
    #[inline] pub fn test_mc3(&self) -> bool {
        self.mc3() != 0
    }

    #[doc="Sets the MC3 field."]
    #[inline] pub fn set_mc3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Match or Capture 4"]
    #[inline] pub fn mc4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if MC4 != 0"]
    #[inline] pub fn test_mc4(&self) -> bool {
        self.mc4() != 0
    }

    #[doc="Sets the MC4 field."]
    #[inline] pub fn set_mc4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Match or Capture 5"]
    #[inline] pub fn mc5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if MC5 != 0"]
    #[inline] pub fn test_mc5(&self) -> bool {
        self.mc5() != 0
    }

    #[doc="Sets the MC5 field."]
    #[inline] pub fn set_mc5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
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
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.trg() != 0 { try!(write!(f, " trg"))}
        if self.cnt() != 0 { try!(write!(f, " cnt"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.ufs() != 0 { try!(write!(f, " ufs"))}
        if self.dfs() != 0 { try!(write!(f, " dfs"))}
        if self.faulta() != 0 { try!(write!(f, " faulta"))}
        if self.faultb() != 0 { try!(write!(f, " faultb"))}
        if self.fault0() != 0 { try!(write!(f, " fault0"))}
        if self.fault1() != 0 { try!(write!(f, " fault1"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        if self.mc2() != 0 { try!(write!(f, " mc2"))}
        if self.mc3() != 0 { try!(write!(f, " mc3"))}
        if self.mc4() != 0 { try!(write!(f, " mc4"))}
        if self.mc5() != 0 { try!(write!(f, " mc5"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="Stop"]
    #[inline] pub fn stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Ramp"]
    #[inline] pub fn idx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IDX != 0"]
    #[inline] pub fn test_idx(&self) -> bool {
        self.idx() != 0
    }

    #[doc="Sets the IDX field."]
    #[inline] pub fn set_idx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Non-recoverable Update Fault State"]
    #[inline] pub fn ufs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if UFS != 0"]
    #[inline] pub fn test_ufs(&self) -> bool {
        self.ufs() != 0
    }

    #[doc="Sets the UFS field."]
    #[inline] pub fn set_ufs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Non-Recoverable Debug Fault State"]
    #[inline] pub fn dfs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DFS != 0"]
    #[inline] pub fn test_dfs(&self) -> bool {
        self.dfs() != 0
    }

    #[doc="Sets the DFS field."]
    #[inline] pub fn set_dfs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Slave"]
    #[inline] pub fn slave(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SLAVE != 0"]
    #[inline] pub fn test_slave(&self) -> bool {
        self.slave() != 0
    }

    #[doc="Sets the SLAVE field."]
    #[inline] pub fn set_slave<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pattern Buffer Valid"]
    #[inline] pub fn pattbufv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PATTBUFV != 0"]
    #[inline] pub fn test_pattbufv(&self) -> bool {
        self.pattbufv() != 0
    }

    #[doc="Sets the PATTBUFV field."]
    #[inline] pub fn set_pattbufv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Period Buffer Valid"]
    #[inline] pub fn perbufv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PERBUFV != 0"]
    #[inline] pub fn test_perbufv(&self) -> bool {
        self.perbufv() != 0
    }

    #[doc="Sets the PERBUFV field."]
    #[inline] pub fn set_perbufv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Recoverable Fault A Input"]
    #[inline] pub fn faultain(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FAULTAIN != 0"]
    #[inline] pub fn test_faultain(&self) -> bool {
        self.faultain() != 0
    }

    #[doc="Sets the FAULTAIN field."]
    #[inline] pub fn set_faultain<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Recoverable Fault B Input"]
    #[inline] pub fn faultbin(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FAULTBIN != 0"]
    #[inline] pub fn test_faultbin(&self) -> bool {
        self.faultbin() != 0
    }

    #[doc="Sets the FAULTBIN field."]
    #[inline] pub fn set_faultbin<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Non-Recoverable Fault0 Input"]
    #[inline] pub fn fault0in(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FAULT0IN != 0"]
    #[inline] pub fn test_fault0in(&self) -> bool {
        self.fault0in() != 0
    }

    #[doc="Sets the FAULT0IN field."]
    #[inline] pub fn set_fault0in<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Non-Recoverable Fault1 Input"]
    #[inline] pub fn fault1in(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FAULT1IN != 0"]
    #[inline] pub fn test_fault1in(&self) -> bool {
        self.fault1in() != 0
    }

    #[doc="Sets the FAULT1IN field."]
    #[inline] pub fn set_fault1in<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Recoverable Fault A State"]
    #[inline] pub fn faulta(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FAULTA != 0"]
    #[inline] pub fn test_faulta(&self) -> bool {
        self.faulta() != 0
    }

    #[doc="Sets the FAULTA field."]
    #[inline] pub fn set_faulta<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Recoverable Fault B State"]
    #[inline] pub fn faultb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FAULTB != 0"]
    #[inline] pub fn test_faultb(&self) -> bool {
        self.faultb() != 0
    }

    #[doc="Sets the FAULTB field."]
    #[inline] pub fn set_faultb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Non-Recoverable Fault 0 State"]
    #[inline] pub fn fault0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FAULT0 != 0"]
    #[inline] pub fn test_fault0(&self) -> bool {
        self.fault0() != 0
    }

    #[doc="Sets the FAULT0 field."]
    #[inline] pub fn set_fault0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Non-Recoverable Fault 1 State"]
    #[inline] pub fn fault1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FAULT1 != 0"]
    #[inline] pub fn test_fault1(&self) -> bool {
        self.fault1() != 0
    }

    #[doc="Sets the FAULT1 field."]
    #[inline] pub fn set_fault1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Compare Channel 0 Buffer Valid"]
    #[inline] pub fn ccbufv0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CCBUFV0 != 0"]
    #[inline] pub fn test_ccbufv0(&self) -> bool {
        self.ccbufv0() != 0
    }

    #[doc="Sets the CCBUFV0 field."]
    #[inline] pub fn set_ccbufv0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Compare Channel 1 Buffer Valid"]
    #[inline] pub fn ccbufv1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CCBUFV1 != 0"]
    #[inline] pub fn test_ccbufv1(&self) -> bool {
        self.ccbufv1() != 0
    }

    #[doc="Sets the CCBUFV1 field."]
    #[inline] pub fn set_ccbufv1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Compare Channel 2 Buffer Valid"]
    #[inline] pub fn ccbufv2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CCBUFV2 != 0"]
    #[inline] pub fn test_ccbufv2(&self) -> bool {
        self.ccbufv2() != 0
    }

    #[doc="Sets the CCBUFV2 field."]
    #[inline] pub fn set_ccbufv2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Compare Channel 3 Buffer Valid"]
    #[inline] pub fn ccbufv3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CCBUFV3 != 0"]
    #[inline] pub fn test_ccbufv3(&self) -> bool {
        self.ccbufv3() != 0
    }

    #[doc="Sets the CCBUFV3 field."]
    #[inline] pub fn set_ccbufv3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Compare Channel 4 Buffer Valid"]
    #[inline] pub fn ccbufv4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CCBUFV4 != 0"]
    #[inline] pub fn test_ccbufv4(&self) -> bool {
        self.ccbufv4() != 0
    }

    #[doc="Sets the CCBUFV4 field."]
    #[inline] pub fn set_ccbufv4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Compare Channel 5 Buffer Valid"]
    #[inline] pub fn ccbufv5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if CCBUFV5 != 0"]
    #[inline] pub fn test_ccbufv5(&self) -> bool {
        self.ccbufv5() != 0
    }

    #[doc="Sets the CCBUFV5 field."]
    #[inline] pub fn set_ccbufv5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Compare Channel 0 Value"]
    #[inline] pub fn cmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CMP0 != 0"]
    #[inline] pub fn test_cmp0(&self) -> bool {
        self.cmp0() != 0
    }

    #[doc="Sets the CMP0 field."]
    #[inline] pub fn set_cmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Compare Channel 1 Value"]
    #[inline] pub fn cmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CMP1 != 0"]
    #[inline] pub fn test_cmp1(&self) -> bool {
        self.cmp1() != 0
    }

    #[doc="Sets the CMP1 field."]
    #[inline] pub fn set_cmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Compare Channel 2 Value"]
    #[inline] pub fn cmp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CMP2 != 0"]
    #[inline] pub fn test_cmp2(&self) -> bool {
        self.cmp2() != 0
    }

    #[doc="Sets the CMP2 field."]
    #[inline] pub fn set_cmp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Compare Channel 3 Value"]
    #[inline] pub fn cmp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if CMP3 != 0"]
    #[inline] pub fn test_cmp3(&self) -> bool {
        self.cmp3() != 0
    }

    #[doc="Sets the CMP3 field."]
    #[inline] pub fn set_cmp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Compare Channel 4 Value"]
    #[inline] pub fn cmp4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if CMP4 != 0"]
    #[inline] pub fn test_cmp4(&self) -> bool {
        self.cmp4() != 0
    }

    #[doc="Sets the CMP4 field."]
    #[inline] pub fn set_cmp4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Compare Channel 5 Value"]
    #[inline] pub fn cmp5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if CMP5 != 0"]
    #[inline] pub fn test_cmp5(&self) -> bool {
        self.cmp5() != 0
    }

    #[doc="Sets the CMP5 field."]
    #[inline] pub fn set_cmp5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
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
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.idx() != 0 { try!(write!(f, " idx"))}
        if self.ufs() != 0 { try!(write!(f, " ufs"))}
        if self.dfs() != 0 { try!(write!(f, " dfs"))}
        if self.slave() != 0 { try!(write!(f, " slave"))}
        if self.pattbufv() != 0 { try!(write!(f, " pattbufv"))}
        if self.perbufv() != 0 { try!(write!(f, " perbufv"))}
        if self.faultain() != 0 { try!(write!(f, " faultain"))}
        if self.faultbin() != 0 { try!(write!(f, " faultbin"))}
        if self.fault0in() != 0 { try!(write!(f, " fault0in"))}
        if self.fault1in() != 0 { try!(write!(f, " fault1in"))}
        if self.faulta() != 0 { try!(write!(f, " faulta"))}
        if self.faultb() != 0 { try!(write!(f, " faultb"))}
        if self.fault0() != 0 { try!(write!(f, " fault0"))}
        if self.fault1() != 0 { try!(write!(f, " fault1"))}
        if self.ccbufv0() != 0 { try!(write!(f, " ccbufv0"))}
        if self.ccbufv1() != 0 { try!(write!(f, " ccbufv1"))}
        if self.ccbufv2() != 0 { try!(write!(f, " ccbufv2"))}
        if self.ccbufv3() != 0 { try!(write!(f, " ccbufv3"))}
        if self.ccbufv4() != 0 { try!(write!(f, " ccbufv4"))}
        if self.ccbufv5() != 0 { try!(write!(f, " ccbufv5"))}
        if self.cmp0() != 0 { try!(write!(f, " cmp0"))}
        if self.cmp1() != 0 { try!(write!(f, " cmp1"))}
        if self.cmp2() != 0 { try!(write!(f, " cmp2"))}
        if self.cmp3() != 0 { try!(write!(f, " cmp3"))}
        if self.cmp4() != 0 { try!(write!(f, " cmp4"))}
        if self.cmp5() != 0 { try!(write!(f, " cmp5"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
    #[doc="Counter Value"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Count {
    #[inline]
    fn from(other: u32) -> Self {
         Count(other)
    }
}

impl ::core::fmt::Display for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CountDith4(pub u32);
impl CountDith4 {
    #[doc="Counter Value"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfffff) as u32) } // [23:4]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U20>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for CountDith4 {
    #[inline]
    fn from(other: u32) -> Self {
         CountDith4(other)
    }
}

impl ::core::fmt::Display for CountDith4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CountDith4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CountDith5(pub u32);
impl CountDith5 {
    #[doc="Counter Value"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7ffff) as u32) } // [23:5]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U19>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for CountDith5 {
    #[inline]
    fn from(other: u32) -> Self {
         CountDith5(other)
    }
}

impl ::core::fmt::Display for CountDith5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CountDith5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CountDith6(pub u32);
impl CountDith6 {
    #[doc="Counter Value"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3ffff) as u32) } // [23:6]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U18>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for CountDith6 {
    #[inline]
    fn from(other: u32) -> Self {
         CountDith6(other)
    }
}

impl ::core::fmt::Display for CountDith6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CountDith6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pattern"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Patt(pub u16);
impl Patt {
    #[doc="Pattern Generator 0 Output Enable"]
    #[inline] pub fn pge0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PGE0 != 0"]
    #[inline] pub fn test_pge0(&self) -> bool {
        self.pge0() != 0
    }

    #[doc="Sets the PGE0 field."]
    #[inline] pub fn set_pge0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Pattern Generator 1 Output Enable"]
    #[inline] pub fn pge1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PGE1 != 0"]
    #[inline] pub fn test_pge1(&self) -> bool {
        self.pge1() != 0
    }

    #[doc="Sets the PGE1 field."]
    #[inline] pub fn set_pge1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pattern Generator 2 Output Enable"]
    #[inline] pub fn pge2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PGE2 != 0"]
    #[inline] pub fn test_pge2(&self) -> bool {
        self.pge2() != 0
    }

    #[doc="Sets the PGE2 field."]
    #[inline] pub fn set_pge2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pattern Generator 3 Output Enable"]
    #[inline] pub fn pge3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PGE3 != 0"]
    #[inline] pub fn test_pge3(&self) -> bool {
        self.pge3() != 0
    }

    #[doc="Sets the PGE3 field."]
    #[inline] pub fn set_pge3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pattern Generator 4 Output Enable"]
    #[inline] pub fn pge4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PGE4 != 0"]
    #[inline] pub fn test_pge4(&self) -> bool {
        self.pge4() != 0
    }

    #[doc="Sets the PGE4 field."]
    #[inline] pub fn set_pge4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pattern Generator 5 Output Enable"]
    #[inline] pub fn pge5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PGE5 != 0"]
    #[inline] pub fn test_pge5(&self) -> bool {
        self.pge5() != 0
    }

    #[doc="Sets the PGE5 field."]
    #[inline] pub fn set_pge5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Pattern Generator 6 Output Enable"]
    #[inline] pub fn pge6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PGE6 != 0"]
    #[inline] pub fn test_pge6(&self) -> bool {
        self.pge6() != 0
    }

    #[doc="Sets the PGE6 field."]
    #[inline] pub fn set_pge6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pattern Generator 7 Output Enable"]
    #[inline] pub fn pge7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PGE7 != 0"]
    #[inline] pub fn test_pge7(&self) -> bool {
        self.pge7() != 0
    }

    #[doc="Sets the PGE7 field."]
    #[inline] pub fn set_pge7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pattern Generator 0 Output Value"]
    #[inline] pub fn pgv0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PGV0 != 0"]
    #[inline] pub fn test_pgv0(&self) -> bool {
        self.pgv0() != 0
    }

    #[doc="Sets the PGV0 field."]
    #[inline] pub fn set_pgv0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pattern Generator 1 Output Value"]
    #[inline] pub fn pgv1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PGV1 != 0"]
    #[inline] pub fn test_pgv1(&self) -> bool {
        self.pgv1() != 0
    }

    #[doc="Sets the PGV1 field."]
    #[inline] pub fn set_pgv1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pattern Generator 2 Output Value"]
    #[inline] pub fn pgv2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PGV2 != 0"]
    #[inline] pub fn test_pgv2(&self) -> bool {
        self.pgv2() != 0
    }

    #[doc="Sets the PGV2 field."]
    #[inline] pub fn set_pgv2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pattern Generator 3 Output Value"]
    #[inline] pub fn pgv3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PGV3 != 0"]
    #[inline] pub fn test_pgv3(&self) -> bool {
        self.pgv3() != 0
    }

    #[doc="Sets the PGV3 field."]
    #[inline] pub fn set_pgv3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pattern Generator 4 Output Value"]
    #[inline] pub fn pgv4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PGV4 != 0"]
    #[inline] pub fn test_pgv4(&self) -> bool {
        self.pgv4() != 0
    }

    #[doc="Sets the PGV4 field."]
    #[inline] pub fn set_pgv4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pattern Generator 5 Output Value"]
    #[inline] pub fn pgv5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PGV5 != 0"]
    #[inline] pub fn test_pgv5(&self) -> bool {
        self.pgv5() != 0
    }

    #[doc="Sets the PGV5 field."]
    #[inline] pub fn set_pgv5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Pattern Generator 6 Output Value"]
    #[inline] pub fn pgv6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PGV6 != 0"]
    #[inline] pub fn test_pgv6(&self) -> bool {
        self.pgv6() != 0
    }

    #[doc="Sets the PGV6 field."]
    #[inline] pub fn set_pgv6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pattern Generator 7 Output Value"]
    #[inline] pub fn pgv7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PGV7 != 0"]
    #[inline] pub fn test_pgv7(&self) -> bool {
        self.pgv7() != 0
    }

    #[doc="Sets the PGV7 field."]
    #[inline] pub fn set_pgv7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Patt {
    #[inline]
    fn from(other: u16) -> Self {
         Patt(other)
    }
}

impl ::core::fmt::Display for Patt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Patt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pge0() != 0 { try!(write!(f, " pge0"))}
        if self.pge1() != 0 { try!(write!(f, " pge1"))}
        if self.pge2() != 0 { try!(write!(f, " pge2"))}
        if self.pge3() != 0 { try!(write!(f, " pge3"))}
        if self.pge4() != 0 { try!(write!(f, " pge4"))}
        if self.pge5() != 0 { try!(write!(f, " pge5"))}
        if self.pge6() != 0 { try!(write!(f, " pge6"))}
        if self.pge7() != 0 { try!(write!(f, " pge7"))}
        if self.pgv0() != 0 { try!(write!(f, " pgv0"))}
        if self.pgv1() != 0 { try!(write!(f, " pgv1"))}
        if self.pgv2() != 0 { try!(write!(f, " pgv2"))}
        if self.pgv3() != 0 { try!(write!(f, " pgv3"))}
        if self.pgv4() != 0 { try!(write!(f, " pgv4"))}
        if self.pgv5() != 0 { try!(write!(f, " pgv5"))}
        if self.pgv6() != 0 { try!(write!(f, " pgv6"))}
        if self.pgv7() != 0 { try!(write!(f, " pgv7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Waveform Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wave(pub u32);
impl Wave {
    #[doc="Waveform Generation"]
    #[inline] pub fn wavegen(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if WAVEGEN != 0"]
    #[inline] pub fn test_wavegen(&self) -> bool {
        self.wavegen() != 0
    }

    #[doc="Sets the WAVEGEN field."]
    #[inline] pub fn set_wavegen<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Ramp Mode"]
    #[inline] pub fn ramp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if RAMP != 0"]
    #[inline] pub fn test_ramp(&self) -> bool {
        self.ramp() != 0
    }

    #[doc="Sets the RAMP field."]
    #[inline] pub fn set_ramp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Circular period Enable"]
    #[inline] pub fn ciperen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CIPEREN != 0"]
    #[inline] pub fn test_ciperen(&self) -> bool {
        self.ciperen() != 0
    }

    #[doc="Sets the CIPEREN field."]
    #[inline] pub fn set_ciperen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Circular Channel 0 Enable"]
    #[inline] pub fn ciccen0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CICCEN0 != 0"]
    #[inline] pub fn test_ciccen0(&self) -> bool {
        self.ciccen0() != 0
    }

    #[doc="Sets the CICCEN0 field."]
    #[inline] pub fn set_ciccen0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Circular Channel 1 Enable"]
    #[inline] pub fn ciccen1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CICCEN1 != 0"]
    #[inline] pub fn test_ciccen1(&self) -> bool {
        self.ciccen1() != 0
    }

    #[doc="Sets the CICCEN1 field."]
    #[inline] pub fn set_ciccen1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Circular Channel 2 Enable"]
    #[inline] pub fn ciccen2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CICCEN2 != 0"]
    #[inline] pub fn test_ciccen2(&self) -> bool {
        self.ciccen2() != 0
    }

    #[doc="Sets the CICCEN2 field."]
    #[inline] pub fn set_ciccen2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Circular Channel 3 Enable"]
    #[inline] pub fn ciccen3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CICCEN3 != 0"]
    #[inline] pub fn test_ciccen3(&self) -> bool {
        self.ciccen3() != 0
    }

    #[doc="Sets the CICCEN3 field."]
    #[inline] pub fn set_ciccen3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Channel 0 Polarity"]
    #[inline] pub fn pol0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if POL0 != 0"]
    #[inline] pub fn test_pol0(&self) -> bool {
        self.pol0() != 0
    }

    #[doc="Sets the POL0 field."]
    #[inline] pub fn set_pol0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Channel 1 Polarity"]
    #[inline] pub fn pol1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if POL1 != 0"]
    #[inline] pub fn test_pol1(&self) -> bool {
        self.pol1() != 0
    }

    #[doc="Sets the POL1 field."]
    #[inline] pub fn set_pol1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Channel 2 Polarity"]
    #[inline] pub fn pol2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if POL2 != 0"]
    #[inline] pub fn test_pol2(&self) -> bool {
        self.pol2() != 0
    }

    #[doc="Sets the POL2 field."]
    #[inline] pub fn set_pol2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Channel 3 Polarity"]
    #[inline] pub fn pol3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if POL3 != 0"]
    #[inline] pub fn test_pol3(&self) -> bool {
        self.pol3() != 0
    }

    #[doc="Sets the POL3 field."]
    #[inline] pub fn set_pol3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Channel 4 Polarity"]
    #[inline] pub fn pol4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if POL4 != 0"]
    #[inline] pub fn test_pol4(&self) -> bool {
        self.pol4() != 0
    }

    #[doc="Sets the POL4 field."]
    #[inline] pub fn set_pol4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Channel 5 Polarity"]
    #[inline] pub fn pol5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if POL5 != 0"]
    #[inline] pub fn test_pol5(&self) -> bool {
        self.pol5() != 0
    }

    #[doc="Sets the POL5 field."]
    #[inline] pub fn set_pol5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Swap DTI Output Pair 0"]
    #[inline] pub fn swap0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SWAP0 != 0"]
    #[inline] pub fn test_swap0(&self) -> bool {
        self.swap0() != 0
    }

    #[doc="Sets the SWAP0 field."]
    #[inline] pub fn set_swap0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Swap DTI Output Pair 1"]
    #[inline] pub fn swap1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if SWAP1 != 0"]
    #[inline] pub fn test_swap1(&self) -> bool {
        self.swap1() != 0
    }

    #[doc="Sets the SWAP1 field."]
    #[inline] pub fn set_swap1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Swap DTI Output Pair 2"]
    #[inline] pub fn swap2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if SWAP2 != 0"]
    #[inline] pub fn test_swap2(&self) -> bool {
        self.swap2() != 0
    }

    #[doc="Sets the SWAP2 field."]
    #[inline] pub fn set_swap2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Swap DTI Output Pair 3"]
    #[inline] pub fn swap3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SWAP3 != 0"]
    #[inline] pub fn test_swap3(&self) -> bool {
        self.swap3() != 0
    }

    #[doc="Sets the SWAP3 field."]
    #[inline] pub fn set_swap3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Wave {
    #[inline]
    fn from(other: u32) -> Self {
         Wave(other)
    }
}

impl ::core::fmt::Display for Wave {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wave {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
        if self.ramp() != 0 { try!(write!(f, " ramp=0x{:x}", self.ramp()))}
        if self.ciperen() != 0 { try!(write!(f, " ciperen"))}
        if self.ciccen0() != 0 { try!(write!(f, " ciccen0"))}
        if self.ciccen1() != 0 { try!(write!(f, " ciccen1"))}
        if self.ciccen2() != 0 { try!(write!(f, " ciccen2"))}
        if self.ciccen3() != 0 { try!(write!(f, " ciccen3"))}
        if self.pol0() != 0 { try!(write!(f, " pol0"))}
        if self.pol1() != 0 { try!(write!(f, " pol1"))}
        if self.pol2() != 0 { try!(write!(f, " pol2"))}
        if self.pol3() != 0 { try!(write!(f, " pol3"))}
        if self.pol4() != 0 { try!(write!(f, " pol4"))}
        if self.pol5() != 0 { try!(write!(f, " pol5"))}
        if self.swap0() != 0 { try!(write!(f, " swap0"))}
        if self.swap1() != 0 { try!(write!(f, " swap1"))}
        if self.swap2() != 0 { try!(write!(f, " swap2"))}
        if self.swap3() != 0 { try!(write!(f, " swap3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Period"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Per(pub u32);
impl Per {
    #[doc="Period Value"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Per {
    #[inline]
    fn from(other: u32) -> Self {
         Per(other)
    }
}

impl ::core::fmt::Display for Per {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Per {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Period"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PerDith4(pub u32);
impl PerDith4 {
    #[doc="Dithering Cycle Number"]
    #[inline] pub fn dither(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DITHER != 0"]
    #[inline] pub fn test_dither(&self) -> bool {
        self.dither() != 0
    }

    #[doc="Sets the DITHER field."]
    #[inline] pub fn set_dither<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Period Value"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfffff) as u32) } // [23:4]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U20>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for PerDith4 {
    #[inline]
    fn from(other: u32) -> Self {
         PerDith4(other)
    }
}

impl ::core::fmt::Display for PerDith4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PerDith4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dither() != 0 { try!(write!(f, " dither=0x{:x}", self.dither()))}
        if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Period"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PerDith5(pub u32);
impl PerDith5 {
    #[doc="Dithering Cycle Number"]
    #[inline] pub fn dither(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if DITHER != 0"]
    #[inline] pub fn test_dither(&self) -> bool {
        self.dither() != 0
    }

    #[doc="Sets the DITHER field."]
    #[inline] pub fn set_dither<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Period Value"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7ffff) as u32) } // [23:5]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U19>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for PerDith5 {
    #[inline]
    fn from(other: u32) -> Self {
         PerDith5(other)
    }
}

impl ::core::fmt::Display for PerDith5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PerDith5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dither() != 0 { try!(write!(f, " dither=0x{:x}", self.dither()))}
        if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Period"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PerDith6(pub u32);
impl PerDith6 {
    #[doc="Dithering Cycle Number"]
    #[inline] pub fn dither(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if DITHER != 0"]
    #[inline] pub fn test_dither(&self) -> bool {
        self.dither() != 0
    }

    #[doc="Sets the DITHER field."]
    #[inline] pub fn set_dither<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Period Value"]
    #[inline] pub fn per(&self) -> ::bobbin_bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3ffff) as u32) } // [23:6]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<::bobbin_bits::U18>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for PerDith6 {
    #[inline]
    fn from(other: u32) -> Self {
         PerDith6(other)
    }
}

impl ::core::fmt::Display for PerDith6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PerDith6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dither() != 0 { try!(write!(f, " dither=0x{:x}", self.dither()))}
        if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare and Capture"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc="Channel Compare/Capture Value"]
    #[inline] pub fn cc(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if CC != 0"]
    #[inline] pub fn test_cc(&self) -> bool {
        self.cc() != 0
    }

    #[doc="Sets the CC field."]
    #[inline] pub fn set_cc<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cc {
    #[inline]
    fn from(other: u32) -> Self {
         Cc(other)
    }
}

impl ::core::fmt::Display for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare and Capture"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CcDith4(pub u32);
impl CcDith4 {
    #[doc="Dithering Cycle Number"]
    #[inline] pub fn dither(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DITHER != 0"]
    #[inline] pub fn test_dither(&self) -> bool {
        self.dither() != 0
    }

    #[doc="Sets the DITHER field."]
    #[inline] pub fn set_dither<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Compare/Capture Value"]
    #[inline] pub fn cc(&self) -> ::bobbin_bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfffff) as u32) } // [23:4]
    }

    #[doc="Returns true if CC != 0"]
    #[inline] pub fn test_cc(&self) -> bool {
        self.cc() != 0
    }

    #[doc="Sets the CC field."]
    #[inline] pub fn set_cc<V: Into<::bobbin_bits::U20>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for CcDith4 {
    #[inline]
    fn from(other: u32) -> Self {
         CcDith4(other)
    }
}

impl ::core::fmt::Display for CcDith4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CcDith4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dither() != 0 { try!(write!(f, " dither=0x{:x}", self.dither()))}
        if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare and Capture"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CcDith5(pub u32);
impl CcDith5 {
    #[doc="Dithering Cycle Number"]
    #[inline] pub fn dither(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if DITHER != 0"]
    #[inline] pub fn test_dither(&self) -> bool {
        self.dither() != 0
    }

    #[doc="Sets the DITHER field."]
    #[inline] pub fn set_dither<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Compare/Capture Value"]
    #[inline] pub fn cc(&self) -> ::bobbin_bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7ffff) as u32) } // [23:5]
    }

    #[doc="Returns true if CC != 0"]
    #[inline] pub fn test_cc(&self) -> bool {
        self.cc() != 0
    }

    #[doc="Sets the CC field."]
    #[inline] pub fn set_cc<V: Into<::bobbin_bits::U19>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for CcDith5 {
    #[inline]
    fn from(other: u32) -> Self {
         CcDith5(other)
    }
}

impl ::core::fmt::Display for CcDith5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CcDith5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dither() != 0 { try!(write!(f, " dither=0x{:x}", self.dither()))}
        if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare and Capture"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CcDith6(pub u32);
impl CcDith6 {
    #[doc="Dithering Cycle Number"]
    #[inline] pub fn dither(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if DITHER != 0"]
    #[inline] pub fn test_dither(&self) -> bool {
        self.dither() != 0
    }

    #[doc="Sets the DITHER field."]
    #[inline] pub fn set_dither<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Compare/Capture Value"]
    #[inline] pub fn cc(&self) -> ::bobbin_bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3ffff) as u32) } // [23:6]
    }

    #[doc="Returns true if CC != 0"]
    #[inline] pub fn test_cc(&self) -> bool {
        self.cc() != 0
    }

    #[doc="Sets the CC field."]
    #[inline] pub fn set_cc<V: Into<::bobbin_bits::U18>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for CcDith6 {
    #[inline]
    fn from(other: u32) -> Self {
         CcDith6(other)
    }
}

impl ::core::fmt::Display for CcDith6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CcDith6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dither() != 0 { try!(write!(f, " dither=0x{:x}", self.dither()))}
        if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pattern Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pattbuf(pub u16);
impl Pattbuf {
    #[doc="Pattern Generator 0 Output Enable Buffer"]
    #[inline] pub fn pgeb0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PGEB0 != 0"]
    #[inline] pub fn test_pgeb0(&self) -> bool {
        self.pgeb0() != 0
    }

    #[doc="Sets the PGEB0 field."]
    #[inline] pub fn set_pgeb0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Pattern Generator 1 Output Enable Buffer"]
    #[inline] pub fn pgeb1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PGEB1 != 0"]
    #[inline] pub fn test_pgeb1(&self) -> bool {
        self.pgeb1() != 0
    }

    #[doc="Sets the PGEB1 field."]
    #[inline] pub fn set_pgeb1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pattern Generator 2 Output Enable Buffer"]
    #[inline] pub fn pgeb2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PGEB2 != 0"]
    #[inline] pub fn test_pgeb2(&self) -> bool {
        self.pgeb2() != 0
    }

    #[doc="Sets the PGEB2 field."]
    #[inline] pub fn set_pgeb2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pattern Generator 3 Output Enable Buffer"]
    #[inline] pub fn pgeb3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PGEB3 != 0"]
    #[inline] pub fn test_pgeb3(&self) -> bool {
        self.pgeb3() != 0
    }

    #[doc="Sets the PGEB3 field."]
    #[inline] pub fn set_pgeb3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pattern Generator 4 Output Enable Buffer"]
    #[inline] pub fn pgeb4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PGEB4 != 0"]
    #[inline] pub fn test_pgeb4(&self) -> bool {
        self.pgeb4() != 0
    }

    #[doc="Sets the PGEB4 field."]
    #[inline] pub fn set_pgeb4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pattern Generator 5 Output Enable Buffer"]
    #[inline] pub fn pgeb5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PGEB5 != 0"]
    #[inline] pub fn test_pgeb5(&self) -> bool {
        self.pgeb5() != 0
    }

    #[doc="Sets the PGEB5 field."]
    #[inline] pub fn set_pgeb5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Pattern Generator 6 Output Enable Buffer"]
    #[inline] pub fn pgeb6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PGEB6 != 0"]
    #[inline] pub fn test_pgeb6(&self) -> bool {
        self.pgeb6() != 0
    }

    #[doc="Sets the PGEB6 field."]
    #[inline] pub fn set_pgeb6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pattern Generator 7 Output Enable Buffer"]
    #[inline] pub fn pgeb7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PGEB7 != 0"]
    #[inline] pub fn test_pgeb7(&self) -> bool {
        self.pgeb7() != 0
    }

    #[doc="Sets the PGEB7 field."]
    #[inline] pub fn set_pgeb7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pattern Generator 0 Output Enable"]
    #[inline] pub fn pgvb0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PGVB0 != 0"]
    #[inline] pub fn test_pgvb0(&self) -> bool {
        self.pgvb0() != 0
    }

    #[doc="Sets the PGVB0 field."]
    #[inline] pub fn set_pgvb0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pattern Generator 1 Output Enable"]
    #[inline] pub fn pgvb1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PGVB1 != 0"]
    #[inline] pub fn test_pgvb1(&self) -> bool {
        self.pgvb1() != 0
    }

    #[doc="Sets the PGVB1 field."]
    #[inline] pub fn set_pgvb1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pattern Generator 2 Output Enable"]
    #[inline] pub fn pgvb2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PGVB2 != 0"]
    #[inline] pub fn test_pgvb2(&self) -> bool {
        self.pgvb2() != 0
    }

    #[doc="Sets the PGVB2 field."]
    #[inline] pub fn set_pgvb2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pattern Generator 3 Output Enable"]
    #[inline] pub fn pgvb3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PGVB3 != 0"]
    #[inline] pub fn test_pgvb3(&self) -> bool {
        self.pgvb3() != 0
    }

    #[doc="Sets the PGVB3 field."]
    #[inline] pub fn set_pgvb3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pattern Generator 4 Output Enable"]
    #[inline] pub fn pgvb4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PGVB4 != 0"]
    #[inline] pub fn test_pgvb4(&self) -> bool {
        self.pgvb4() != 0
    }

    #[doc="Sets the PGVB4 field."]
    #[inline] pub fn set_pgvb4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pattern Generator 5 Output Enable"]
    #[inline] pub fn pgvb5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PGVB5 != 0"]
    #[inline] pub fn test_pgvb5(&self) -> bool {
        self.pgvb5() != 0
    }

    #[doc="Sets the PGVB5 field."]
    #[inline] pub fn set_pgvb5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Pattern Generator 6 Output Enable"]
    #[inline] pub fn pgvb6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PGVB6 != 0"]
    #[inline] pub fn test_pgvb6(&self) -> bool {
        self.pgvb6() != 0
    }

    #[doc="Sets the PGVB6 field."]
    #[inline] pub fn set_pgvb6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pattern Generator 7 Output Enable"]
    #[inline] pub fn pgvb7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PGVB7 != 0"]
    #[inline] pub fn test_pgvb7(&self) -> bool {
        self.pgvb7() != 0
    }

    #[doc="Sets the PGVB7 field."]
    #[inline] pub fn set_pgvb7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Pattbuf {
    #[inline]
    fn from(other: u16) -> Self {
         Pattbuf(other)
    }
}

impl ::core::fmt::Display for Pattbuf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pattbuf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pgeb0() != 0 { try!(write!(f, " pgeb0"))}
        if self.pgeb1() != 0 { try!(write!(f, " pgeb1"))}
        if self.pgeb2() != 0 { try!(write!(f, " pgeb2"))}
        if self.pgeb3() != 0 { try!(write!(f, " pgeb3"))}
        if self.pgeb4() != 0 { try!(write!(f, " pgeb4"))}
        if self.pgeb5() != 0 { try!(write!(f, " pgeb5"))}
        if self.pgeb6() != 0 { try!(write!(f, " pgeb6"))}
        if self.pgeb7() != 0 { try!(write!(f, " pgeb7"))}
        if self.pgvb0() != 0 { try!(write!(f, " pgvb0"))}
        if self.pgvb1() != 0 { try!(write!(f, " pgvb1"))}
        if self.pgvb2() != 0 { try!(write!(f, " pgvb2"))}
        if self.pgvb3() != 0 { try!(write!(f, " pgvb3"))}
        if self.pgvb4() != 0 { try!(write!(f, " pgvb4"))}
        if self.pgvb5() != 0 { try!(write!(f, " pgvb5"))}
        if self.pgvb6() != 0 { try!(write!(f, " pgvb6"))}
        if self.pgvb7() != 0 { try!(write!(f, " pgvb7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Period Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Perbuf(pub u32);
impl Perbuf {
    #[doc="Period Buffer Value"]
    #[inline] pub fn perbuf(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if PERBUF != 0"]
    #[inline] pub fn test_perbuf(&self) -> bool {
        self.perbuf() != 0
    }

    #[doc="Sets the PERBUF field."]
    #[inline] pub fn set_perbuf<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Perbuf {
    #[inline]
    fn from(other: u32) -> Self {
         Perbuf(other)
    }
}

impl ::core::fmt::Display for Perbuf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Perbuf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.perbuf() != 0 { try!(write!(f, " perbuf=0x{:x}", self.perbuf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Period Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PerbufDith4(pub u32);
impl PerbufDith4 {
    #[doc="Dithering Buffer Cycle Number"]
    #[inline] pub fn ditherbuf(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DITHERBUF != 0"]
    #[inline] pub fn test_ditherbuf(&self) -> bool {
        self.ditherbuf() != 0
    }

    #[doc="Sets the DITHERBUF field."]
    #[inline] pub fn set_ditherbuf<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Period Buffer Value"]
    #[inline] pub fn perbuf(&self) -> ::bobbin_bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfffff) as u32) } // [23:4]
    }

    #[doc="Returns true if PERBUF != 0"]
    #[inline] pub fn test_perbuf(&self) -> bool {
        self.perbuf() != 0
    }

    #[doc="Sets the PERBUF field."]
    #[inline] pub fn set_perbuf<V: Into<::bobbin_bits::U20>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for PerbufDith4 {
    #[inline]
    fn from(other: u32) -> Self {
         PerbufDith4(other)
    }
}

impl ::core::fmt::Display for PerbufDith4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PerbufDith4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ditherbuf() != 0 { try!(write!(f, " ditherbuf=0x{:x}", self.ditherbuf()))}
        if self.perbuf() != 0 { try!(write!(f, " perbuf=0x{:x}", self.perbuf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Period Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PerbufDith5(pub u32);
impl PerbufDith5 {
    #[doc="Dithering Buffer Cycle Number"]
    #[inline] pub fn ditherbuf(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if DITHERBUF != 0"]
    #[inline] pub fn test_ditherbuf(&self) -> bool {
        self.ditherbuf() != 0
    }

    #[doc="Sets the DITHERBUF field."]
    #[inline] pub fn set_ditherbuf<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Period Buffer Value"]
    #[inline] pub fn perbuf(&self) -> ::bobbin_bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7ffff) as u32) } // [23:5]
    }

    #[doc="Returns true if PERBUF != 0"]
    #[inline] pub fn test_perbuf(&self) -> bool {
        self.perbuf() != 0
    }

    #[doc="Sets the PERBUF field."]
    #[inline] pub fn set_perbuf<V: Into<::bobbin_bits::U19>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for PerbufDith5 {
    #[inline]
    fn from(other: u32) -> Self {
         PerbufDith5(other)
    }
}

impl ::core::fmt::Display for PerbufDith5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PerbufDith5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ditherbuf() != 0 { try!(write!(f, " ditherbuf=0x{:x}", self.ditherbuf()))}
        if self.perbuf() != 0 { try!(write!(f, " perbuf=0x{:x}", self.perbuf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Period Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PerbufDith6(pub u32);
impl PerbufDith6 {
    #[doc="Dithering Buffer Cycle Number"]
    #[inline] pub fn ditherbuf(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if DITHERBUF != 0"]
    #[inline] pub fn test_ditherbuf(&self) -> bool {
        self.ditherbuf() != 0
    }

    #[doc="Sets the DITHERBUF field."]
    #[inline] pub fn set_ditherbuf<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Period Buffer Value"]
    #[inline] pub fn perbuf(&self) -> ::bobbin_bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3ffff) as u32) } // [23:6]
    }

    #[doc="Returns true if PERBUF != 0"]
    #[inline] pub fn test_perbuf(&self) -> bool {
        self.perbuf() != 0
    }

    #[doc="Sets the PERBUF field."]
    #[inline] pub fn set_perbuf<V: Into<::bobbin_bits::U18>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for PerbufDith6 {
    #[inline]
    fn from(other: u32) -> Self {
         PerbufDith6(other)
    }
}

impl ::core::fmt::Display for PerbufDith6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for PerbufDith6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ditherbuf() != 0 { try!(write!(f, " ditherbuf=0x{:x}", self.ditherbuf()))}
        if self.perbuf() != 0 { try!(write!(f, " perbuf=0x{:x}", self.perbuf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare and Capture Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccbuf(pub u32);
impl Ccbuf {
    #[doc="Channel Compare/Capture Buffer Value"]
    #[inline] pub fn ccbuf(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if CCBUF != 0"]
    #[inline] pub fn test_ccbuf(&self) -> bool {
        self.ccbuf() != 0
    }

    #[doc="Sets the CCBUF field."]
    #[inline] pub fn set_ccbuf<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccbuf {
    #[inline]
    fn from(other: u32) -> Self {
         Ccbuf(other)
    }
}

impl ::core::fmt::Display for Ccbuf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccbuf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccbuf() != 0 { try!(write!(f, " ccbuf=0x{:x}", self.ccbuf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare and Capture Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CcbufDith4(pub u32);
impl CcbufDith4 {
    #[doc="Channel Compare/Capture Buffer Value"]
    #[inline] pub fn ccbuf(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CCBUF != 0"]
    #[inline] pub fn test_ccbuf(&self) -> bool {
        self.ccbuf() != 0
    }

    #[doc="Sets the CCBUF field."]
    #[inline] pub fn set_ccbuf<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Dithering Buffer Cycle Number"]
    #[inline] pub fn ditherbuf(&self) -> ::bobbin_bits::U20 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfffff) as u32) } // [23:4]
    }

    #[doc="Returns true if DITHERBUF != 0"]
    #[inline] pub fn test_ditherbuf(&self) -> bool {
        self.ditherbuf() != 0
    }

    #[doc="Sets the DITHERBUF field."]
    #[inline] pub fn set_ditherbuf<V: Into<::bobbin_bits::U20>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U20 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfffff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for CcbufDith4 {
    #[inline]
    fn from(other: u32) -> Self {
         CcbufDith4(other)
    }
}

impl ::core::fmt::Display for CcbufDith4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CcbufDith4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccbuf() != 0 { try!(write!(f, " ccbuf=0x{:x}", self.ccbuf()))}
        if self.ditherbuf() != 0 { try!(write!(f, " ditherbuf=0x{:x}", self.ditherbuf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare and Capture Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CcbufDith5(pub u32);
impl CcbufDith5 {
    #[doc="Dithering Buffer Cycle Number"]
    #[inline] pub fn ditherbuf(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if DITHERBUF != 0"]
    #[inline] pub fn test_ditherbuf(&self) -> bool {
        self.ditherbuf() != 0
    }

    #[doc="Sets the DITHERBUF field."]
    #[inline] pub fn set_ditherbuf<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Compare/Capture Buffer Value"]
    #[inline] pub fn ccbuf(&self) -> ::bobbin_bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7ffff) as u32) } // [23:5]
    }

    #[doc="Returns true if CCBUF != 0"]
    #[inline] pub fn test_ccbuf(&self) -> bool {
        self.ccbuf() != 0
    }

    #[doc="Sets the CCBUF field."]
    #[inline] pub fn set_ccbuf<V: Into<::bobbin_bits::U19>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for CcbufDith5 {
    #[inline]
    fn from(other: u32) -> Self {
         CcbufDith5(other)
    }
}

impl ::core::fmt::Display for CcbufDith5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CcbufDith5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ditherbuf() != 0 { try!(write!(f, " ditherbuf=0x{:x}", self.ditherbuf()))}
        if self.ccbuf() != 0 { try!(write!(f, " ccbuf=0x{:x}", self.ccbuf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare and Capture Buffer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CcbufDith6(pub u32);
impl CcbufDith6 {
    #[doc="Dithering Buffer Cycle Number"]
    #[inline] pub fn ditherbuf(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if DITHERBUF != 0"]
    #[inline] pub fn test_ditherbuf(&self) -> bool {
        self.ditherbuf() != 0
    }

    #[doc="Sets the DITHERBUF field."]
    #[inline] pub fn set_ditherbuf<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Compare/Capture Buffer Value"]
    #[inline] pub fn ccbuf(&self) -> ::bobbin_bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3ffff) as u32) } // [23:6]
    }

    #[doc="Returns true if CCBUF != 0"]
    #[inline] pub fn test_ccbuf(&self) -> bool {
        self.ccbuf() != 0
    }

    #[doc="Sets the CCBUF field."]
    #[inline] pub fn set_ccbuf<V: Into<::bobbin_bits::U18>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for CcbufDith6 {
    #[inline]
    fn from(other: u32) -> Self {
         CcbufDith6(other)
    }
}

impl ::core::fmt::Display for CcbufDith6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for CcbufDith6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ditherbuf() != 0 { try!(write!(f, " ditherbuf=0x{:x}", self.ditherbuf()))}
        if self.ccbuf() != 0 { try!(write!(f, " ccbuf=0x{:x}", self.ccbuf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

