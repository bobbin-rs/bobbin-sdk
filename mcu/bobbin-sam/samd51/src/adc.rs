::bobbin_mcu::periph!( ADC0, Adc0, ADC0_PERIPH, AdcPeriph, ADC0_OWNED, ADC0_REF_COUNT, 0x43001c00, 0x00, 0x01);
::bobbin_mcu::periph!( ADC1, Adc1, ADC1_PERIPH, AdcPeriph, ADC1_OWNED, ADC1_REF_COUNT, 0x43002000, 0x01, 0x02);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 

impl AdcPeriph {
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

    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::Register<Evctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Evctrl, 0x2)
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

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0x3)
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

    #[doc="Get the INPUTCTRL Register."]
    #[inline] pub fn inputctrl_reg(&self) -> ::bobbin_mcu::register::Register<Inputctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Inputctrl, 0x4)
    }

    #[doc="Get the *mut pointer for the INPUTCTRL register."]
    #[inline] pub fn inputctrl_mut(&self) -> *mut Inputctrl { 
        self.inputctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the INPUTCTRL register."]
    #[inline] pub fn inputctrl_ptr(&self) -> *const Inputctrl { 
        self.inputctrl_reg().ptr()
    }

    #[doc="Read the INPUTCTRL register."]
    #[inline] pub fn inputctrl(&self) -> Inputctrl { 
        self.inputctrl_reg().read()
    }

    #[doc="Write the INPUTCTRL register."]
    #[inline] pub fn write_inputctrl(&self, value: Inputctrl) -> &Self { 
        self.inputctrl_reg().write(value);
        self
    }

    #[doc="Set the INPUTCTRL register."]
    #[inline] pub fn set_inputctrl<F: FnOnce(Inputctrl) -> Inputctrl>(&self, f: F) -> &Self {
        self.inputctrl_reg().set(f);
        self
    }

    #[doc="Modify the INPUTCTRL register."]
    #[inline] pub fn with_inputctrl<F: FnOnce(Inputctrl) -> Inputctrl>(&self, f: F) -> &Self {
        self.inputctrl_reg().with(f);
        self
    }

    #[doc="Get the CTRLB Register."]
    #[inline] pub fn ctrlb_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlb, 0x6)
    }

    #[doc="Get the *mut pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_mut(&self) -> *mut Ctrlb { 
        self.ctrlb_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLB register."]
    #[inline] pub fn ctrlb_ptr(&self) -> *const Ctrlb { 
        self.ctrlb_reg().ptr()
    }

    #[doc="Read the CTRLB register."]
    #[inline] pub fn ctrlb(&self) -> Ctrlb { 
        self.ctrlb_reg().read()
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

    #[doc="Modify the CTRLB register."]
    #[inline] pub fn with_ctrlb<F: FnOnce(Ctrlb) -> Ctrlb>(&self, f: F) -> &Self {
        self.ctrlb_reg().with(f);
        self
    }

    #[doc="Get the REFCTRL Register."]
    #[inline] pub fn refctrl_reg(&self) -> ::bobbin_mcu::register::Register<Refctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Refctrl, 0x8)
    }

    #[doc="Get the *mut pointer for the REFCTRL register."]
    #[inline] pub fn refctrl_mut(&self) -> *mut Refctrl { 
        self.refctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the REFCTRL register."]
    #[inline] pub fn refctrl_ptr(&self) -> *const Refctrl { 
        self.refctrl_reg().ptr()
    }

    #[doc="Read the REFCTRL register."]
    #[inline] pub fn refctrl(&self) -> Refctrl { 
        self.refctrl_reg().read()
    }

    #[doc="Write the REFCTRL register."]
    #[inline] pub fn write_refctrl(&self, value: Refctrl) -> &Self { 
        self.refctrl_reg().write(value);
        self
    }

    #[doc="Set the REFCTRL register."]
    #[inline] pub fn set_refctrl<F: FnOnce(Refctrl) -> Refctrl>(&self, f: F) -> &Self {
        self.refctrl_reg().set(f);
        self
    }

    #[doc="Modify the REFCTRL register."]
    #[inline] pub fn with_refctrl<F: FnOnce(Refctrl) -> Refctrl>(&self, f: F) -> &Self {
        self.refctrl_reg().with(f);
        self
    }

    #[doc="Get the AVGCTRL Register."]
    #[inline] pub fn avgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Avgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Avgctrl, 0xa)
    }

    #[doc="Get the *mut pointer for the AVGCTRL register."]
    #[inline] pub fn avgctrl_mut(&self) -> *mut Avgctrl { 
        self.avgctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the AVGCTRL register."]
    #[inline] pub fn avgctrl_ptr(&self) -> *const Avgctrl { 
        self.avgctrl_reg().ptr()
    }

    #[doc="Read the AVGCTRL register."]
    #[inline] pub fn avgctrl(&self) -> Avgctrl { 
        self.avgctrl_reg().read()
    }

    #[doc="Write the AVGCTRL register."]
    #[inline] pub fn write_avgctrl(&self, value: Avgctrl) -> &Self { 
        self.avgctrl_reg().write(value);
        self
    }

    #[doc="Set the AVGCTRL register."]
    #[inline] pub fn set_avgctrl<F: FnOnce(Avgctrl) -> Avgctrl>(&self, f: F) -> &Self {
        self.avgctrl_reg().set(f);
        self
    }

    #[doc="Modify the AVGCTRL register."]
    #[inline] pub fn with_avgctrl<F: FnOnce(Avgctrl) -> Avgctrl>(&self, f: F) -> &Self {
        self.avgctrl_reg().with(f);
        self
    }

    #[doc="Get the SAMPCTRL Register."]
    #[inline] pub fn sampctrl_reg(&self) -> ::bobbin_mcu::register::Register<Sampctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sampctrl, 0xb)
    }

    #[doc="Get the *mut pointer for the SAMPCTRL register."]
    #[inline] pub fn sampctrl_mut(&self) -> *mut Sampctrl { 
        self.sampctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the SAMPCTRL register."]
    #[inline] pub fn sampctrl_ptr(&self) -> *const Sampctrl { 
        self.sampctrl_reg().ptr()
    }

    #[doc="Read the SAMPCTRL register."]
    #[inline] pub fn sampctrl(&self) -> Sampctrl { 
        self.sampctrl_reg().read()
    }

    #[doc="Write the SAMPCTRL register."]
    #[inline] pub fn write_sampctrl(&self, value: Sampctrl) -> &Self { 
        self.sampctrl_reg().write(value);
        self
    }

    #[doc="Set the SAMPCTRL register."]
    #[inline] pub fn set_sampctrl<F: FnOnce(Sampctrl) -> Sampctrl>(&self, f: F) -> &Self {
        self.sampctrl_reg().set(f);
        self
    }

    #[doc="Modify the SAMPCTRL register."]
    #[inline] pub fn with_sampctrl<F: FnOnce(Sampctrl) -> Sampctrl>(&self, f: F) -> &Self {
        self.sampctrl_reg().with(f);
        self
    }

    #[doc="Get the WINLT Register."]
    #[inline] pub fn winlt_reg(&self) -> ::bobbin_mcu::register::Register<Winlt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Winlt, 0xc)
    }

    #[doc="Get the *mut pointer for the WINLT register."]
    #[inline] pub fn winlt_mut(&self) -> *mut Winlt { 
        self.winlt_reg().ptr()
    }

    #[doc="Get the *const pointer for the WINLT register."]
    #[inline] pub fn winlt_ptr(&self) -> *const Winlt { 
        self.winlt_reg().ptr()
    }

    #[doc="Read the WINLT register."]
    #[inline] pub fn winlt(&self) -> Winlt { 
        self.winlt_reg().read()
    }

    #[doc="Write the WINLT register."]
    #[inline] pub fn write_winlt(&self, value: Winlt) -> &Self { 
        self.winlt_reg().write(value);
        self
    }

    #[doc="Set the WINLT register."]
    #[inline] pub fn set_winlt<F: FnOnce(Winlt) -> Winlt>(&self, f: F) -> &Self {
        self.winlt_reg().set(f);
        self
    }

    #[doc="Modify the WINLT register."]
    #[inline] pub fn with_winlt<F: FnOnce(Winlt) -> Winlt>(&self, f: F) -> &Self {
        self.winlt_reg().with(f);
        self
    }

    #[doc="Get the WINUT Register."]
    #[inline] pub fn winut_reg(&self) -> ::bobbin_mcu::register::Register<Winut> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Winut, 0xe)
    }

    #[doc="Get the *mut pointer for the WINUT register."]
    #[inline] pub fn winut_mut(&self) -> *mut Winut { 
        self.winut_reg().ptr()
    }

    #[doc="Get the *const pointer for the WINUT register."]
    #[inline] pub fn winut_ptr(&self) -> *const Winut { 
        self.winut_reg().ptr()
    }

    #[doc="Read the WINUT register."]
    #[inline] pub fn winut(&self) -> Winut { 
        self.winut_reg().read()
    }

    #[doc="Write the WINUT register."]
    #[inline] pub fn write_winut(&self, value: Winut) -> &Self { 
        self.winut_reg().write(value);
        self
    }

    #[doc="Set the WINUT register."]
    #[inline] pub fn set_winut<F: FnOnce(Winut) -> Winut>(&self, f: F) -> &Self {
        self.winut_reg().set(f);
        self
    }

    #[doc="Modify the WINUT register."]
    #[inline] pub fn with_winut<F: FnOnce(Winut) -> Winut>(&self, f: F) -> &Self {
        self.winut_reg().with(f);
        self
    }

    #[doc="Get the GAINCORR Register."]
    #[inline] pub fn gaincorr_reg(&self) -> ::bobbin_mcu::register::Register<Gaincorr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gaincorr, 0x10)
    }

    #[doc="Get the *mut pointer for the GAINCORR register."]
    #[inline] pub fn gaincorr_mut(&self) -> *mut Gaincorr { 
        self.gaincorr_reg().ptr()
    }

    #[doc="Get the *const pointer for the GAINCORR register."]
    #[inline] pub fn gaincorr_ptr(&self) -> *const Gaincorr { 
        self.gaincorr_reg().ptr()
    }

    #[doc="Read the GAINCORR register."]
    #[inline] pub fn gaincorr(&self) -> Gaincorr { 
        self.gaincorr_reg().read()
    }

    #[doc="Write the GAINCORR register."]
    #[inline] pub fn write_gaincorr(&self, value: Gaincorr) -> &Self { 
        self.gaincorr_reg().write(value);
        self
    }

    #[doc="Set the GAINCORR register."]
    #[inline] pub fn set_gaincorr<F: FnOnce(Gaincorr) -> Gaincorr>(&self, f: F) -> &Self {
        self.gaincorr_reg().set(f);
        self
    }

    #[doc="Modify the GAINCORR register."]
    #[inline] pub fn with_gaincorr<F: FnOnce(Gaincorr) -> Gaincorr>(&self, f: F) -> &Self {
        self.gaincorr_reg().with(f);
        self
    }

    #[doc="Get the OFFSETCORR Register."]
    #[inline] pub fn offsetcorr_reg(&self) -> ::bobbin_mcu::register::Register<Offsetcorr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Offsetcorr, 0x12)
    }

    #[doc="Get the *mut pointer for the OFFSETCORR register."]
    #[inline] pub fn offsetcorr_mut(&self) -> *mut Offsetcorr { 
        self.offsetcorr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OFFSETCORR register."]
    #[inline] pub fn offsetcorr_ptr(&self) -> *const Offsetcorr { 
        self.offsetcorr_reg().ptr()
    }

    #[doc="Read the OFFSETCORR register."]
    #[inline] pub fn offsetcorr(&self) -> Offsetcorr { 
        self.offsetcorr_reg().read()
    }

    #[doc="Write the OFFSETCORR register."]
    #[inline] pub fn write_offsetcorr(&self, value: Offsetcorr) -> &Self { 
        self.offsetcorr_reg().write(value);
        self
    }

    #[doc="Set the OFFSETCORR register."]
    #[inline] pub fn set_offsetcorr<F: FnOnce(Offsetcorr) -> Offsetcorr>(&self, f: F) -> &Self {
        self.offsetcorr_reg().set(f);
        self
    }

    #[doc="Modify the OFFSETCORR register."]
    #[inline] pub fn with_offsetcorr<F: FnOnce(Offsetcorr) -> Offsetcorr>(&self, f: F) -> &Self {
        self.offsetcorr_reg().with(f);
        self
    }

    #[doc="Get the SWTRIG Register."]
    #[inline] pub fn swtrig_reg(&self) -> ::bobbin_mcu::register::Register<Swtrig> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Swtrig, 0x14)
    }

    #[doc="Get the *mut pointer for the SWTRIG register."]
    #[inline] pub fn swtrig_mut(&self) -> *mut Swtrig { 
        self.swtrig_reg().ptr()
    }

    #[doc="Get the *const pointer for the SWTRIG register."]
    #[inline] pub fn swtrig_ptr(&self) -> *const Swtrig { 
        self.swtrig_reg().ptr()
    }

    #[doc="Read the SWTRIG register."]
    #[inline] pub fn swtrig(&self) -> Swtrig { 
        self.swtrig_reg().read()
    }

    #[doc="Write the SWTRIG register."]
    #[inline] pub fn write_swtrig(&self, value: Swtrig) -> &Self { 
        self.swtrig_reg().write(value);
        self
    }

    #[doc="Set the SWTRIG register."]
    #[inline] pub fn set_swtrig<F: FnOnce(Swtrig) -> Swtrig>(&self, f: F) -> &Self {
        self.swtrig_reg().set(f);
        self
    }

    #[doc="Modify the SWTRIG register."]
    #[inline] pub fn with_swtrig<F: FnOnce(Swtrig) -> Swtrig>(&self, f: F) -> &Self {
        self.swtrig_reg().with(f);
        self
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x2c)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x2d)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x2e)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x2f)
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x30)
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

    #[doc="Get the DSEQDATA Register."]
    #[inline] pub fn dseqdata_reg(&self) -> ::bobbin_mcu::register::Register<Dseqdata> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dseqdata, 0x34)
    }

    #[doc="Get the *mut pointer for the DSEQDATA register."]
    #[inline] pub fn dseqdata_mut(&self) -> *mut Dseqdata { 
        self.dseqdata_reg().ptr()
    }

    #[doc="Get the *const pointer for the DSEQDATA register."]
    #[inline] pub fn dseqdata_ptr(&self) -> *const Dseqdata { 
        self.dseqdata_reg().ptr()
    }

    #[doc="Write the DSEQDATA register."]
    #[inline] pub fn write_dseqdata(&self, value: Dseqdata) -> &Self { 
        self.dseqdata_reg().write(value);
        self
    }

    #[doc="Set the DSEQDATA register."]
    #[inline] pub fn set_dseqdata<F: FnOnce(Dseqdata) -> Dseqdata>(&self, f: F) -> &Self {
        self.dseqdata_reg().set(f);
        self
    }

    #[doc="Get the DSEQCTRL Register."]
    #[inline] pub fn dseqctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dseqctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dseqctrl, 0x38)
    }

    #[doc="Get the *mut pointer for the DSEQCTRL register."]
    #[inline] pub fn dseqctrl_mut(&self) -> *mut Dseqctrl { 
        self.dseqctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the DSEQCTRL register."]
    #[inline] pub fn dseqctrl_ptr(&self) -> *const Dseqctrl { 
        self.dseqctrl_reg().ptr()
    }

    #[doc="Read the DSEQCTRL register."]
    #[inline] pub fn dseqctrl(&self) -> Dseqctrl { 
        self.dseqctrl_reg().read()
    }

    #[doc="Write the DSEQCTRL register."]
    #[inline] pub fn write_dseqctrl(&self, value: Dseqctrl) -> &Self { 
        self.dseqctrl_reg().write(value);
        self
    }

    #[doc="Set the DSEQCTRL register."]
    #[inline] pub fn set_dseqctrl<F: FnOnce(Dseqctrl) -> Dseqctrl>(&self, f: F) -> &Self {
        self.dseqctrl_reg().set(f);
        self
    }

    #[doc="Modify the DSEQCTRL register."]
    #[inline] pub fn with_dseqctrl<F: FnOnce(Dseqctrl) -> Dseqctrl>(&self, f: F) -> &Self {
        self.dseqctrl_reg().with(f);
        self
    }

    #[doc="Get the DSEQSTAT Register."]
    #[inline] pub fn dseqstat_reg(&self) -> ::bobbin_mcu::register::Register<Dseqstat> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dseqstat, 0x3c)
    }

    #[doc="Get the *mut pointer for the DSEQSTAT register."]
    #[inline] pub fn dseqstat_mut(&self) -> *mut Dseqstat { 
        self.dseqstat_reg().ptr()
    }

    #[doc="Get the *const pointer for the DSEQSTAT register."]
    #[inline] pub fn dseqstat_ptr(&self) -> *const Dseqstat { 
        self.dseqstat_reg().ptr()
    }

    #[doc="Read the DSEQSTAT register."]
    #[inline] pub fn dseqstat(&self) -> Dseqstat { 
        self.dseqstat_reg().read()
    }

    #[doc="Get the RESULT Register."]
    #[inline] pub fn result_reg(&self) -> ::bobbin_mcu::register::Register<Result> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Result, 0x40)
    }

    #[doc="Get the *mut pointer for the RESULT register."]
    #[inline] pub fn result_mut(&self) -> *mut Result { 
        self.result_reg().ptr()
    }

    #[doc="Get the *const pointer for the RESULT register."]
    #[inline] pub fn result_ptr(&self) -> *const Result { 
        self.result_reg().ptr()
    }

    #[doc="Read the RESULT register."]
    #[inline] pub fn result(&self) -> Result { 
        self.result_reg().read()
    }

    #[doc="Get the RESS Register."]
    #[inline] pub fn ress_reg(&self) -> ::bobbin_mcu::register::Register<Ress> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ress, 0x44)
    }

    #[doc="Get the *mut pointer for the RESS register."]
    #[inline] pub fn ress_mut(&self) -> *mut Ress { 
        self.ress_reg().ptr()
    }

    #[doc="Get the *const pointer for the RESS register."]
    #[inline] pub fn ress_ptr(&self) -> *const Ress { 
        self.ress_reg().ptr()
    }

    #[doc="Read the RESS register."]
    #[inline] pub fn ress(&self) -> Ress { 
        self.ress_reg().read()
    }

    #[doc="Get the CALIB Register."]
    #[inline] pub fn calib_reg(&self) -> ::bobbin_mcu::register::Register<Calib> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Calib, 0x48)
    }

    #[doc="Get the *mut pointer for the CALIB register."]
    #[inline] pub fn calib_mut(&self) -> *mut Calib { 
        self.calib_reg().ptr()
    }

    #[doc="Get the *const pointer for the CALIB register."]
    #[inline] pub fn calib_ptr(&self) -> *const Calib { 
        self.calib_reg().ptr()
    }

    #[doc="Read the CALIB register."]
    #[inline] pub fn calib(&self) -> Calib { 
        self.calib_reg().read()
    }

    #[doc="Write the CALIB register."]
    #[inline] pub fn write_calib(&self, value: Calib) -> &Self { 
        self.calib_reg().write(value);
        self
    }

    #[doc="Set the CALIB register."]
    #[inline] pub fn set_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        self.calib_reg().set(f);
        self
    }

    #[doc="Modify the CALIB register."]
    #[inline] pub fn with_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        self.calib_reg().with(f);
        self
    }

}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Dual Mode Trigger Selection"]
    #[inline] pub fn dualsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if DUALSEL != 0"]
    #[inline] pub fn test_dualsel(&self) -> bool {
        self.dualsel() != 0
    }

    #[doc="Sets the DUALSEL field."]
    #[inline] pub fn set_dualsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Slave Enable"]
    #[inline] pub fn slaveen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SLAVEEN != 0"]
    #[inline] pub fn test_slaveen(&self) -> bool {
        self.slaveen() != 0
    }

    #[doc="Sets the SLAVEEN field."]
    #[inline] pub fn set_slaveen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
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
        let value: u16 = value.into();
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
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Prescaler Configuration"]
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
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Rail to Rail Operation Enable"]
    #[inline] pub fn r2r(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if R2R != 0"]
    #[inline] pub fn test_r2r(&self) -> bool {
        self.r2r() != 0
    }

    #[doc="Sets the R2R field."]
    #[inline] pub fn set_r2r<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.dualsel() != 0 { try!(write!(f, " dualsel=0x{:x}", self.dualsel()))}
        if self.slaveen() != 0 { try!(write!(f, " slaveen"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        if self.r2r() != 0 { try!(write!(f, " r2r"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u8);
impl Evctrl {
    #[doc="Flush Event Input Enable"]
    #[inline] pub fn flushei(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FLUSHEI != 0"]
    #[inline] pub fn test_flushei(&self) -> bool {
        self.flushei() != 0
    }

    #[doc="Sets the FLUSHEI field."]
    #[inline] pub fn set_flushei<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start Conversion Event Input Enable"]
    #[inline] pub fn startei(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if STARTEI != 0"]
    #[inline] pub fn test_startei(&self) -> bool {
        self.startei() != 0
    }

    #[doc="Sets the STARTEI field."]
    #[inline] pub fn set_startei<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Flush Event Invert Enable"]
    #[inline] pub fn flushinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FLUSHINV != 0"]
    #[inline] pub fn test_flushinv(&self) -> bool {
        self.flushinv() != 0
    }

    #[doc="Sets the FLUSHINV field."]
    #[inline] pub fn set_flushinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Start Conversion Event Invert Enable"]
    #[inline] pub fn startinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STARTINV != 0"]
    #[inline] pub fn test_startinv(&self) -> bool {
        self.startinv() != 0
    }

    #[doc="Sets the STARTINV field."]
    #[inline] pub fn set_startinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Result Ready Event Out"]
    #[inline] pub fn resrdyeo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RESRDYEO != 0"]
    #[inline] pub fn test_resrdyeo(&self) -> bool {
        self.resrdyeo() != 0
    }

    #[doc="Sets the RESRDYEO field."]
    #[inline] pub fn set_resrdyeo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Window Monitor Event Out"]
    #[inline] pub fn winmoneo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WINMONEO != 0"]
    #[inline] pub fn test_winmoneo(&self) -> bool {
        self.winmoneo() != 0
    }

    #[doc="Sets the WINMONEO field."]
    #[inline] pub fn set_winmoneo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
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
        if self.flushei() != 0 { try!(write!(f, " flushei"))}
        if self.startei() != 0 { try!(write!(f, " startei"))}
        if self.flushinv() != 0 { try!(write!(f, " flushinv"))}
        if self.startinv() != 0 { try!(write!(f, " startinv"))}
        if self.resrdyeo() != 0 { try!(write!(f, " resrdyeo"))}
        if self.winmoneo() != 0 { try!(write!(f, " winmoneo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Run"]
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Input Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Inputctrl(pub u16);
impl Inputctrl {
    #[doc="Positive Mux Input Selection"]
    #[inline] pub fn muxpos(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if MUXPOS != 0"]
    #[inline] pub fn test_muxpos(&self) -> bool {
        self.muxpos() != 0
    }

    #[doc="Sets the MUXPOS field."]
    #[inline] pub fn set_muxpos<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Differential Mode"]
    #[inline] pub fn diffmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DIFFMODE != 0"]
    #[inline] pub fn test_diffmode(&self) -> bool {
        self.diffmode() != 0
    }

    #[doc="Sets the DIFFMODE field."]
    #[inline] pub fn set_diffmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Negative Mux Input Selection"]
    #[inline] pub fn muxneg(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if MUXNEG != 0"]
    #[inline] pub fn test_muxneg(&self) -> bool {
        self.muxneg() != 0
    }

    #[doc="Sets the MUXNEG field."]
    #[inline] pub fn set_muxneg<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Stop DMA Sequencing"]
    #[inline] pub fn dseqstop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DSEQSTOP != 0"]
    #[inline] pub fn test_dseqstop(&self) -> bool {
        self.dseqstop() != 0
    }

    #[doc="Sets the DSEQSTOP field."]
    #[inline] pub fn set_dseqstop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Inputctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Inputctrl(other)
    }
}

impl ::core::fmt::Display for Inputctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Inputctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.muxpos() != 0 { try!(write!(f, " muxpos=0x{:x}", self.muxpos()))}
        if self.diffmode() != 0 { try!(write!(f, " diffmode"))}
        if self.muxneg() != 0 { try!(write!(f, " muxneg=0x{:x}", self.muxneg()))}
        if self.dseqstop() != 0 { try!(write!(f, " dseqstop"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u16);
impl Ctrlb {
    #[doc="Left-Adjusted Result"]
    #[inline] pub fn leftadj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LEFTADJ != 0"]
    #[inline] pub fn test_leftadj(&self) -> bool {
        self.leftadj() != 0
    }

    #[doc="Sets the LEFTADJ field."]
    #[inline] pub fn set_leftadj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Free Running Mode"]
    #[inline] pub fn freerun(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FREERUN != 0"]
    #[inline] pub fn test_freerun(&self) -> bool {
        self.freerun() != 0
    }

    #[doc="Sets the FREERUN field."]
    #[inline] pub fn set_freerun<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Digital Correction Logic Enable"]
    #[inline] pub fn corren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CORREN != 0"]
    #[inline] pub fn test_corren(&self) -> bool {
        self.corren() != 0
    }

    #[doc="Sets the CORREN field."]
    #[inline] pub fn set_corren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Conversion Result Resolution"]
    #[inline] pub fn ressel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if RESSEL != 0"]
    #[inline] pub fn test_ressel(&self) -> bool {
        self.ressel() != 0
    }

    #[doc="Sets the RESSEL field."]
    #[inline] pub fn set_ressel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Window Monitor Mode"]
    #[inline] pub fn winmode(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if WINMODE != 0"]
    #[inline] pub fn test_winmode(&self) -> bool {
        self.winmode() != 0
    }

    #[doc="Sets the WINMODE field."]
    #[inline] pub fn set_winmode<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Window Single Sample"]
    #[inline] pub fn winss(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WINSS != 0"]
    #[inline] pub fn test_winss(&self) -> bool {
        self.winss() != 0
    }

    #[doc="Sets the WINSS field."]
    #[inline] pub fn set_winss<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
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
        if self.leftadj() != 0 { try!(write!(f, " leftadj"))}
        if self.freerun() != 0 { try!(write!(f, " freerun"))}
        if self.corren() != 0 { try!(write!(f, " corren"))}
        if self.ressel() != 0 { try!(write!(f, " ressel=0x{:x}", self.ressel()))}
        if self.winmode() != 0 { try!(write!(f, " winmode=0x{:x}", self.winmode()))}
        if self.winss() != 0 { try!(write!(f, " winss"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reference Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Refctrl(pub u8);
impl Refctrl {
    #[doc="Reference Selection"]
    #[inline] pub fn refsel(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if REFSEL != 0"]
    #[inline] pub fn test_refsel(&self) -> bool {
        self.refsel() != 0
    }

    #[doc="Sets the REFSEL field."]
    #[inline] pub fn set_refsel<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Reference Buffer Offset Compensation Enable"]
    #[inline] pub fn refcomp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if REFCOMP != 0"]
    #[inline] pub fn test_refcomp(&self) -> bool {
        self.refcomp() != 0
    }

    #[doc="Sets the REFCOMP field."]
    #[inline] pub fn set_refcomp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Refctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Refctrl(other)
    }
}

impl ::core::fmt::Display for Refctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Refctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.refsel() != 0 { try!(write!(f, " refsel=0x{:x}", self.refsel()))}
        if self.refcomp() != 0 { try!(write!(f, " refcomp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Average Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Avgctrl(pub u8);
impl Avgctrl {
    #[doc="Number of Samples to be Collected"]
    #[inline] pub fn samplenum(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SAMPLENUM != 0"]
    #[inline] pub fn test_samplenum(&self) -> bool {
        self.samplenum() != 0
    }

    #[doc="Sets the SAMPLENUM field."]
    #[inline] pub fn set_samplenum<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Adjusting Result / Division Coefficient"]
    #[inline] pub fn adjres(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if ADJRES != 0"]
    #[inline] pub fn test_adjres(&self) -> bool {
        self.adjres() != 0
    }

    #[doc="Sets the ADJRES field."]
    #[inline] pub fn set_adjres<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Avgctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Avgctrl(other)
    }
}

impl ::core::fmt::Display for Avgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Avgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.samplenum() != 0 { try!(write!(f, " samplenum=0x{:x}", self.samplenum()))}
        if self.adjres() != 0 { try!(write!(f, " adjres=0x{:x}", self.adjres()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Sample Time Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sampctrl(pub u8);
impl Sampctrl {
    #[doc="Sampling Time Length"]
    #[inline] pub fn samplen(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if SAMPLEN != 0"]
    #[inline] pub fn test_samplen(&self) -> bool {
        self.samplen() != 0
    }

    #[doc="Sets the SAMPLEN field."]
    #[inline] pub fn set_samplen<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Comparator Offset Compensation Enable"]
    #[inline] pub fn offcomp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OFFCOMP != 0"]
    #[inline] pub fn test_offcomp(&self) -> bool {
        self.offcomp() != 0
    }

    #[doc="Sets the OFFCOMP field."]
    #[inline] pub fn set_offcomp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Sampctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Sampctrl(other)
    }
}

impl ::core::fmt::Display for Sampctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sampctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.samplen() != 0 { try!(write!(f, " samplen=0x{:x}", self.samplen()))}
        if self.offcomp() != 0 { try!(write!(f, " offcomp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Window Monitor Lower Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winlt(pub u16);
impl Winlt {
    #[doc="Window Lower Threshold"]
    #[inline] pub fn winlt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WINLT != 0"]
    #[inline] pub fn test_winlt(&self) -> bool {
        self.winlt() != 0
    }

    #[doc="Sets the WINLT field."]
    #[inline] pub fn set_winlt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Winlt {
    #[inline]
    fn from(other: u16) -> Self {
         Winlt(other)
    }
}

impl ::core::fmt::Display for Winlt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winlt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.winlt() != 0 { try!(write!(f, " winlt=0x{:x}", self.winlt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Window Monitor Upper Threshold"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winut(pub u16);
impl Winut {
    #[doc="Window Upper Threshold"]
    #[inline] pub fn winut(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if WINUT != 0"]
    #[inline] pub fn test_winut(&self) -> bool {
        self.winut() != 0
    }

    #[doc="Sets the WINUT field."]
    #[inline] pub fn set_winut<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Winut {
    #[inline]
    fn from(other: u16) -> Self {
         Winut(other)
    }
}

impl ::core::fmt::Display for Winut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.winut() != 0 { try!(write!(f, " winut=0x{:x}", self.winut()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Gain Correction"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gaincorr(pub u16);
impl Gaincorr {
    #[doc="Gain Correction Value"]
    #[inline] pub fn gaincorr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if GAINCORR != 0"]
    #[inline] pub fn test_gaincorr(&self) -> bool {
        self.gaincorr() != 0
    }

    #[doc="Sets the GAINCORR field."]
    #[inline] pub fn set_gaincorr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Gaincorr {
    #[inline]
    fn from(other: u16) -> Self {
         Gaincorr(other)
    }
}

impl ::core::fmt::Display for Gaincorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gaincorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gaincorr() != 0 { try!(write!(f, " gaincorr=0x{:x}", self.gaincorr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Offset Correction"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Offsetcorr(pub u16);
impl Offsetcorr {
    #[doc="Offset Correction Value"]
    #[inline] pub fn offsetcorr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if OFFSETCORR != 0"]
    #[inline] pub fn test_offsetcorr(&self) -> bool {
        self.offsetcorr() != 0
    }

    #[doc="Sets the OFFSETCORR field."]
    #[inline] pub fn set_offsetcorr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Offsetcorr {
    #[inline]
    fn from(other: u16) -> Self {
         Offsetcorr(other)
    }
}

impl ::core::fmt::Display for Offsetcorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Offsetcorr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.offsetcorr() != 0 { try!(write!(f, " offsetcorr=0x{:x}", self.offsetcorr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software Trigger"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swtrig(pub u8);
impl Swtrig {
    #[doc="ADC Conversion Flush"]
    #[inline] pub fn flush(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FLUSH != 0"]
    #[inline] pub fn test_flush(&self) -> bool {
        self.flush() != 0
    }

    #[doc="Sets the FLUSH field."]
    #[inline] pub fn set_flush<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start ADC Conversion"]
    #[inline] pub fn start(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Swtrig {
    #[inline]
    fn from(other: u8) -> Self {
         Swtrig(other)
    }
}

impl ::core::fmt::Display for Swtrig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swtrig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flush() != 0 { try!(write!(f, " flush"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Result Ready Interrupt Disable"]
    #[inline] pub fn resrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RESRDY != 0"]
    #[inline] pub fn test_resrdy(&self) -> bool {
        self.resrdy() != 0
    }

    #[doc="Sets the RESRDY field."]
    #[inline] pub fn set_resrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overrun Interrupt Disable"]
    #[inline] pub fn overrun(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVERRUN != 0"]
    #[inline] pub fn test_overrun(&self) -> bool {
        self.overrun() != 0
    }

    #[doc="Sets the OVERRUN field."]
    #[inline] pub fn set_overrun<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window Monitor Interrupt Disable"]
    #[inline] pub fn winmon(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WINMON != 0"]
    #[inline] pub fn test_winmon(&self) -> bool {
        self.winmon() != 0
    }

    #[doc="Sets the WINMON field."]
    #[inline] pub fn set_winmon<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Intenclr {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.resrdy() != 0 { try!(write!(f, " resrdy"))}
        if self.overrun() != 0 { try!(write!(f, " overrun"))}
        if self.winmon() != 0 { try!(write!(f, " winmon"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Result Ready Interrupt Enable"]
    #[inline] pub fn resrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RESRDY != 0"]
    #[inline] pub fn test_resrdy(&self) -> bool {
        self.resrdy() != 0
    }

    #[doc="Sets the RESRDY field."]
    #[inline] pub fn set_resrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn overrun(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVERRUN != 0"]
    #[inline] pub fn test_overrun(&self) -> bool {
        self.overrun() != 0
    }

    #[doc="Sets the OVERRUN field."]
    #[inline] pub fn set_overrun<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window Monitor Interrupt Enable"]
    #[inline] pub fn winmon(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WINMON != 0"]
    #[inline] pub fn test_winmon(&self) -> bool {
        self.winmon() != 0
    }

    #[doc="Sets the WINMON field."]
    #[inline] pub fn set_winmon<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Intenset {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.resrdy() != 0 { try!(write!(f, " resrdy"))}
        if self.overrun() != 0 { try!(write!(f, " overrun"))}
        if self.winmon() != 0 { try!(write!(f, " winmon"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Result Ready Interrupt Flag"]
    #[inline] pub fn resrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RESRDY != 0"]
    #[inline] pub fn test_resrdy(&self) -> bool {
        self.resrdy() != 0
    }

    #[doc="Sets the RESRDY field."]
    #[inline] pub fn set_resrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Overrun Interrupt Flag"]
    #[inline] pub fn overrun(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OVERRUN != 0"]
    #[inline] pub fn test_overrun(&self) -> bool {
        self.overrun() != 0
    }

    #[doc="Sets the OVERRUN field."]
    #[inline] pub fn set_overrun<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Window Monitor Interrupt Flag"]
    #[inline] pub fn winmon(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WINMON != 0"]
    #[inline] pub fn test_winmon(&self) -> bool {
        self.winmon() != 0
    }

    #[doc="Sets the WINMON field."]
    #[inline] pub fn set_winmon<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Intflag {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.resrdy() != 0 { try!(write!(f, " resrdy"))}
        if self.overrun() != 0 { try!(write!(f, " overrun"))}
        if self.winmon() != 0 { try!(write!(f, " winmon"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="ADC Busy Status"]
    #[inline] pub fn adcbusy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADCBUSY != 0"]
    #[inline] pub fn test_adcbusy(&self) -> bool {
        self.adcbusy() != 0
    }

    #[doc="Sets the ADCBUSY field."]
    #[inline] pub fn set_adcbusy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Window Comparator Counter"]
    #[inline] pub fn wcc(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3f) as u8) } // [7:2]
    }

    #[doc="Returns true if WCC != 0"]
    #[inline] pub fn test_wcc(&self) -> bool {
        self.wcc() != 0
    }

    #[doc="Sets the WCC field."]
    #[inline] pub fn set_wcc<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.adcbusy() != 0 { try!(write!(f, " adcbusy"))}
        if self.wcc() != 0 { try!(write!(f, " wcc=0x{:x}", self.wcc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u32);
impl Syncbusy {
    #[doc="SWRST Synchronization Busy"]
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
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Input Control Synchronization Busy"]
    #[inline] pub fn inputctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if INPUTCTRL != 0"]
    #[inline] pub fn test_inputctrl(&self) -> bool {
        self.inputctrl() != 0
    }

    #[doc="Sets the INPUTCTRL field."]
    #[inline] pub fn set_inputctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Control B Synchronization Busy"]
    #[inline] pub fn ctrlb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CTRLB != 0"]
    #[inline] pub fn test_ctrlb(&self) -> bool {
        self.ctrlb() != 0
    }

    #[doc="Sets the CTRLB field."]
    #[inline] pub fn set_ctrlb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Reference Control Synchronization Busy"]
    #[inline] pub fn refctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if REFCTRL != 0"]
    #[inline] pub fn test_refctrl(&self) -> bool {
        self.refctrl() != 0
    }

    #[doc="Sets the REFCTRL field."]
    #[inline] pub fn set_refctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Average Control Synchronization Busy"]
    #[inline] pub fn avgctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AVGCTRL != 0"]
    #[inline] pub fn test_avgctrl(&self) -> bool {
        self.avgctrl() != 0
    }

    #[doc="Sets the AVGCTRL field."]
    #[inline] pub fn set_avgctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Sampling Time Control Synchronization Busy"]
    #[inline] pub fn sampctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SAMPCTRL != 0"]
    #[inline] pub fn test_sampctrl(&self) -> bool {
        self.sampctrl() != 0
    }

    #[doc="Sets the SAMPCTRL field."]
    #[inline] pub fn set_sampctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Window Monitor Lower Threshold Synchronization Busy"]
    #[inline] pub fn winlt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if WINLT != 0"]
    #[inline] pub fn test_winlt(&self) -> bool {
        self.winlt() != 0
    }

    #[doc="Sets the WINLT field."]
    #[inline] pub fn set_winlt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Window Monitor Upper Threshold Synchronization Busy"]
    #[inline] pub fn winut(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if WINUT != 0"]
    #[inline] pub fn test_winut(&self) -> bool {
        self.winut() != 0
    }

    #[doc="Sets the WINUT field."]
    #[inline] pub fn set_winut<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Gain Correction Synchronization Busy"]
    #[inline] pub fn gaincorr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GAINCORR != 0"]
    #[inline] pub fn test_gaincorr(&self) -> bool {
        self.gaincorr() != 0
    }

    #[doc="Sets the GAINCORR field."]
    #[inline] pub fn set_gaincorr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Offset Correction Synchronization Busy"]
    #[inline] pub fn offsetcorr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OFFSETCORR != 0"]
    #[inline] pub fn test_offsetcorr(&self) -> bool {
        self.offsetcorr() != 0
    }

    #[doc="Sets the OFFSETCORR field."]
    #[inline] pub fn set_offsetcorr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Software Trigger Synchronization Busy"]
    #[inline] pub fn swtrig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SWTRIG != 0"]
    #[inline] pub fn test_swtrig(&self) -> bool {
        self.swtrig() != 0
    }

    #[doc="Sets the SWTRIG field."]
    #[inline] pub fn set_swtrig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
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
        if self.inputctrl() != 0 { try!(write!(f, " inputctrl"))}
        if self.ctrlb() != 0 { try!(write!(f, " ctrlb"))}
        if self.refctrl() != 0 { try!(write!(f, " refctrl"))}
        if self.avgctrl() != 0 { try!(write!(f, " avgctrl"))}
        if self.sampctrl() != 0 { try!(write!(f, " sampctrl"))}
        if self.winlt() != 0 { try!(write!(f, " winlt"))}
        if self.winut() != 0 { try!(write!(f, " winut"))}
        if self.gaincorr() != 0 { try!(write!(f, " gaincorr"))}
        if self.offsetcorr() != 0 { try!(write!(f, " offsetcorr"))}
        if self.swtrig() != 0 { try!(write!(f, " swtrig"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Sequencial Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dseqdata(pub u32);
impl Dseqdata {
    #[doc="DMA Sequential Data"]
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

impl From<u32> for Dseqdata {
    #[inline]
    fn from(other: u32) -> Self {
         Dseqdata(other)
    }
}

impl ::core::fmt::Display for Dseqdata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dseqdata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Sequential Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dseqctrl(pub u32);
impl Dseqctrl {
    #[doc="Input Control"]
    #[inline] pub fn inputctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INPUTCTRL != 0"]
    #[inline] pub fn test_inputctrl(&self) -> bool {
        self.inputctrl() != 0
    }

    #[doc="Sets the INPUTCTRL field."]
    #[inline] pub fn set_inputctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Control B"]
    #[inline] pub fn ctrlb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTRLB != 0"]
    #[inline] pub fn test_ctrlb(&self) -> bool {
        self.ctrlb() != 0
    }

    #[doc="Sets the CTRLB field."]
    #[inline] pub fn set_ctrlb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Reference Control"]
    #[inline] pub fn refctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if REFCTRL != 0"]
    #[inline] pub fn test_refctrl(&self) -> bool {
        self.refctrl() != 0
    }

    #[doc="Sets the REFCTRL field."]
    #[inline] pub fn set_refctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Average Control"]
    #[inline] pub fn avgctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if AVGCTRL != 0"]
    #[inline] pub fn test_avgctrl(&self) -> bool {
        self.avgctrl() != 0
    }

    #[doc="Sets the AVGCTRL field."]
    #[inline] pub fn set_avgctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Sampling Time Control"]
    #[inline] pub fn sampctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SAMPCTRL != 0"]
    #[inline] pub fn test_sampctrl(&self) -> bool {
        self.sampctrl() != 0
    }

    #[doc="Sets the SAMPCTRL field."]
    #[inline] pub fn set_sampctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Window Monitor Lower Threshold"]
    #[inline] pub fn winlt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WINLT != 0"]
    #[inline] pub fn test_winlt(&self) -> bool {
        self.winlt() != 0
    }

    #[doc="Sets the WINLT field."]
    #[inline] pub fn set_winlt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Window Monitor Upper Threshold"]
    #[inline] pub fn winut(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if WINUT != 0"]
    #[inline] pub fn test_winut(&self) -> bool {
        self.winut() != 0
    }

    #[doc="Sets the WINUT field."]
    #[inline] pub fn set_winut<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Gain Correction"]
    #[inline] pub fn gaincorr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GAINCORR != 0"]
    #[inline] pub fn test_gaincorr(&self) -> bool {
        self.gaincorr() != 0
    }

    #[doc="Sets the GAINCORR field."]
    #[inline] pub fn set_gaincorr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Offset Correction"]
    #[inline] pub fn offsetcorr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OFFSETCORR != 0"]
    #[inline] pub fn test_offsetcorr(&self) -> bool {
        self.offsetcorr() != 0
    }

    #[doc="Sets the OFFSETCORR field."]
    #[inline] pub fn set_offsetcorr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADC Auto-Start Conversion"]
    #[inline] pub fn autostart(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if AUTOSTART != 0"]
    #[inline] pub fn test_autostart(&self) -> bool {
        self.autostart() != 0
    }

    #[doc="Sets the AUTOSTART field."]
    #[inline] pub fn set_autostart<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Dseqctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Dseqctrl(other)
    }
}

impl ::core::fmt::Display for Dseqctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dseqctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.inputctrl() != 0 { try!(write!(f, " inputctrl"))}
        if self.ctrlb() != 0 { try!(write!(f, " ctrlb"))}
        if self.refctrl() != 0 { try!(write!(f, " refctrl"))}
        if self.avgctrl() != 0 { try!(write!(f, " avgctrl"))}
        if self.sampctrl() != 0 { try!(write!(f, " sampctrl"))}
        if self.winlt() != 0 { try!(write!(f, " winlt"))}
        if self.winut() != 0 { try!(write!(f, " winut"))}
        if self.gaincorr() != 0 { try!(write!(f, " gaincorr"))}
        if self.offsetcorr() != 0 { try!(write!(f, " offsetcorr"))}
        if self.autostart() != 0 { try!(write!(f, " autostart"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Sequencial Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dseqstat(pub u32);
impl Dseqstat {
    #[doc="Input Control"]
    #[inline] pub fn inputctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INPUTCTRL != 0"]
    #[inline] pub fn test_inputctrl(&self) -> bool {
        self.inputctrl() != 0
    }

    #[doc="Sets the INPUTCTRL field."]
    #[inline] pub fn set_inputctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Control B"]
    #[inline] pub fn ctrlb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTRLB != 0"]
    #[inline] pub fn test_ctrlb(&self) -> bool {
        self.ctrlb() != 0
    }

    #[doc="Sets the CTRLB field."]
    #[inline] pub fn set_ctrlb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Reference Control"]
    #[inline] pub fn refctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if REFCTRL != 0"]
    #[inline] pub fn test_refctrl(&self) -> bool {
        self.refctrl() != 0
    }

    #[doc="Sets the REFCTRL field."]
    #[inline] pub fn set_refctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Average Control"]
    #[inline] pub fn avgctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if AVGCTRL != 0"]
    #[inline] pub fn test_avgctrl(&self) -> bool {
        self.avgctrl() != 0
    }

    #[doc="Sets the AVGCTRL field."]
    #[inline] pub fn set_avgctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Sampling Time Control"]
    #[inline] pub fn sampctrl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SAMPCTRL != 0"]
    #[inline] pub fn test_sampctrl(&self) -> bool {
        self.sampctrl() != 0
    }

    #[doc="Sets the SAMPCTRL field."]
    #[inline] pub fn set_sampctrl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Window Monitor Lower Threshold"]
    #[inline] pub fn winlt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WINLT != 0"]
    #[inline] pub fn test_winlt(&self) -> bool {
        self.winlt() != 0
    }

    #[doc="Sets the WINLT field."]
    #[inline] pub fn set_winlt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Window Monitor Upper Threshold"]
    #[inline] pub fn winut(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if WINUT != 0"]
    #[inline] pub fn test_winut(&self) -> bool {
        self.winut() != 0
    }

    #[doc="Sets the WINUT field."]
    #[inline] pub fn set_winut<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Gain Correction"]
    #[inline] pub fn gaincorr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GAINCORR != 0"]
    #[inline] pub fn test_gaincorr(&self) -> bool {
        self.gaincorr() != 0
    }

    #[doc="Sets the GAINCORR field."]
    #[inline] pub fn set_gaincorr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Offset Correction"]
    #[inline] pub fn offsetcorr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OFFSETCORR != 0"]
    #[inline] pub fn test_offsetcorr(&self) -> bool {
        self.offsetcorr() != 0
    }

    #[doc="Sets the OFFSETCORR field."]
    #[inline] pub fn set_offsetcorr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA Sequencing Busy"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Dseqstat {
    #[inline]
    fn from(other: u32) -> Self {
         Dseqstat(other)
    }
}

impl ::core::fmt::Display for Dseqstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dseqstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.inputctrl() != 0 { try!(write!(f, " inputctrl"))}
        if self.ctrlb() != 0 { try!(write!(f, " ctrlb"))}
        if self.refctrl() != 0 { try!(write!(f, " refctrl"))}
        if self.avgctrl() != 0 { try!(write!(f, " avgctrl"))}
        if self.sampctrl() != 0 { try!(write!(f, " sampctrl"))}
        if self.winlt() != 0 { try!(write!(f, " winlt"))}
        if self.winut() != 0 { try!(write!(f, " winut"))}
        if self.gaincorr() != 0 { try!(write!(f, " gaincorr"))}
        if self.offsetcorr() != 0 { try!(write!(f, " offsetcorr"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Result Conversion Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Result(pub u16);
impl Result {
    #[doc="Result Conversion Value"]
    #[inline] pub fn result(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if RESULT != 0"]
    #[inline] pub fn test_result(&self) -> bool {
        self.result() != 0
    }

    #[doc="Sets the RESULT field."]
    #[inline] pub fn set_result<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Result {
    #[inline]
    fn from(other: u16) -> Self {
         Result(other)
    }
}

impl ::core::fmt::Display for Result {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Result {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.result() != 0 { try!(write!(f, " result=0x{:x}", self.result()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Last Sample Result"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ress(pub u16);
impl Ress {
    #[doc="Last ADC conversion result"]
    #[inline] pub fn ress(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if RESS != 0"]
    #[inline] pub fn test_ress(&self) -> bool {
        self.ress() != 0
    }

    #[doc="Sets the RESS field."]
    #[inline] pub fn set_ress<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Ress {
    #[inline]
    fn from(other: u16) -> Self {
         Ress(other)
    }
}

impl ::core::fmt::Display for Ress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ress() != 0 { try!(write!(f, " ress=0x{:x}", self.ress()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Calibration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calib(pub u16);
impl Calib {
    #[doc="Bias Comparator Scaling"]
    #[inline] pub fn biascomp(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if BIASCOMP != 0"]
    #[inline] pub fn test_biascomp(&self) -> bool {
        self.biascomp() != 0
    }

    #[doc="Sets the BIASCOMP field."]
    #[inline] pub fn set_biascomp<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Bias R2R Ampli scaling"]
    #[inline] pub fn biasr2r(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if BIASR2R != 0"]
    #[inline] pub fn test_biasr2r(&self) -> bool {
        self.biasr2r() != 0
    }

    #[doc="Sets the BIASR2R field."]
    #[inline] pub fn set_biasr2r<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Bias Reference Buffer Scaling"]
    #[inline] pub fn biasrefbuf(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if BIASREFBUF != 0"]
    #[inline] pub fn test_biasrefbuf(&self) -> bool {
        self.biasrefbuf() != 0
    }

    #[doc="Sets the BIASREFBUF field."]
    #[inline] pub fn set_biasrefbuf<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Calib {
    #[inline]
    fn from(other: u16) -> Self {
         Calib(other)
    }
}

impl ::core::fmt::Display for Calib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.biascomp() != 0 { try!(write!(f, " biascomp=0x{:x}", self.biascomp()))}
        if self.biasr2r() != 0 { try!(write!(f, " biasr2r=0x{:x}", self.biasr2r()))}
        if self.biasrefbuf() != 0 { try!(write!(f, " biasrefbuf=0x{:x}", self.biasrefbuf()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

