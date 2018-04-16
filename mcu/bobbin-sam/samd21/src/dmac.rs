::bobbin_mcu::periph!( DMAC, Dmac, DMAC_PERIPH, DmacPeriph, DMAC_OWNED, DMAC_REF_COUNT, 0x41004800, 0x00, 0x06);

::bobbin_mcu::channel!(DMAC_CH0, DmacCh0, dmac_ch0, DMAC, Dmac, DMAC_CH0_CH, DmacCh, DMAC_PERIPH, DMAC_CH0_OWNED, DMAC_CH0_REF_COUNT, 0);
::bobbin_mcu::channel!(DMAC_CH1, DmacCh1, dmac_ch1, DMAC, Dmac, DMAC_CH1_CH, DmacCh, DMAC_PERIPH, DMAC_CH1_OWNED, DMAC_CH1_REF_COUNT, 1);
::bobbin_mcu::channel!(DMAC_CH2, DmacCh2, dmac_ch2, DMAC, Dmac, DMAC_CH2_CH, DmacCh, DMAC_PERIPH, DMAC_CH2_OWNED, DMAC_CH2_REF_COUNT, 2);
::bobbin_mcu::channel!(DMAC_CH3, DmacCh3, dmac_ch3, DMAC, Dmac, DMAC_CH3_CH, DmacCh, DMAC_PERIPH, DMAC_CH3_OWNED, DMAC_CH3_REF_COUNT, 3);
::bobbin_mcu::channel!(DMAC_CH4, DmacCh4, dmac_ch4, DMAC, Dmac, DMAC_CH4_CH, DmacCh, DMAC_PERIPH, DMAC_CH4_OWNED, DMAC_CH4_REF_COUNT, 4);
::bobbin_mcu::channel!(DMAC_CH5, DmacCh5, dmac_ch5, DMAC, Dmac, DMAC_CH5_CH, DmacCh, DMAC_PERIPH, DMAC_CH5_OWNED, DMAC_CH5_REF_COUNT, 5);
::bobbin_mcu::channel!(DMAC_CH6, DmacCh6, dmac_ch6, DMAC, Dmac, DMAC_CH6_CH, DmacCh, DMAC_PERIPH, DMAC_CH6_OWNED, DMAC_CH6_REF_COUNT, 6);
::bobbin_mcu::channel!(DMAC_CH7, DmacCh7, dmac_ch7, DMAC, Dmac, DMAC_CH7_CH, DmacCh, DMAC_PERIPH, DMAC_CH7_OWNED, DMAC_CH7_REF_COUNT, 7);
::bobbin_mcu::channel!(DMAC_CH8, DmacCh8, dmac_ch8, DMAC, Dmac, DMAC_CH8_CH, DmacCh, DMAC_PERIPH, DMAC_CH8_OWNED, DMAC_CH8_REF_COUNT, 8);
::bobbin_mcu::channel!(DMAC_CH9, DmacCh9, dmac_ch9, DMAC, Dmac, DMAC_CH9_CH, DmacCh, DMAC_PERIPH, DMAC_CH9_OWNED, DMAC_CH9_REF_COUNT, 9);
::bobbin_mcu::channel!(DMAC_CH10, DmacCh10, dmac_ch10, DMAC, Dmac, DMAC_CH10_CH, DmacCh, DMAC_PERIPH, DMAC_CH10_OWNED, DMAC_CH10_REF_COUNT, 10);
::bobbin_mcu::channel!(DMAC_CH11, DmacCh11, dmac_ch11, DMAC, Dmac, DMAC_CH11_CH, DmacCh, DMAC_PERIPH, DMAC_CH11_OWNED, DMAC_CH11_REF_COUNT, 11);
// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("AHBMASK"), field: Some("DMAC"), description: None }
impl ::bobbin_mcu::gate::GateEn for Dmac {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::pm::PM.ahbmask().dmac() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_ahbmask(|r| r.set_dmac(value));
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMAC Peripheral"]
pub struct DmacPeriph(pub usize); 

pub struct DmacCh { pub periph: DmacPeriph, pub index: usize }

impl DmacPeriph {
    #[doc="Get the ACTIVE Register."]
    #[inline] pub fn active_reg(&self) -> ::bobbin_mcu::register::Register<Active> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Active, 0x30)
    }

    #[doc="Get the *mut pointer for the ACTIVE register."]
    #[inline] pub fn active_mut(&self) -> *mut Active { 
        self.active_reg().ptr()
    }

    #[doc="Get the *const pointer for the ACTIVE register."]
    #[inline] pub fn active_ptr(&self) -> *const Active { 
        self.active_reg().ptr()
    }

    #[doc="Read the ACTIVE register."]
    #[inline] pub fn active(&self) -> Active { 
        self.active_reg().read()
    }

    #[doc="Get the BASEADDR Register."]
    #[inline] pub fn baseaddr_reg(&self) -> ::bobbin_mcu::register::Register<Baseaddr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Baseaddr, 0x34)
    }

    #[doc="Get the *mut pointer for the BASEADDR register."]
    #[inline] pub fn baseaddr_mut(&self) -> *mut Baseaddr { 
        self.baseaddr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BASEADDR register."]
    #[inline] pub fn baseaddr_ptr(&self) -> *const Baseaddr { 
        self.baseaddr_reg().ptr()
    }

    #[doc="Read the BASEADDR register."]
    #[inline] pub fn baseaddr(&self) -> Baseaddr { 
        self.baseaddr_reg().read()
    }

    #[doc="Write the BASEADDR register."]
    #[inline] pub fn write_baseaddr(&self, value: Baseaddr) -> &Self { 
        self.baseaddr_reg().write(value);
        self
    }

    #[doc="Set the BASEADDR register."]
    #[inline] pub fn set_baseaddr<F: FnOnce(Baseaddr) -> Baseaddr>(&self, f: F) -> &Self {
        self.baseaddr_reg().set(f);
        self
    }

    #[doc="Modify the BASEADDR register."]
    #[inline] pub fn with_baseaddr<F: FnOnce(Baseaddr) -> Baseaddr>(&self, f: F) -> &Self {
        self.baseaddr_reg().with(f);
        self
    }

    #[doc="Get the BUSYCH Register."]
    #[inline] pub fn busych_reg(&self) -> ::bobbin_mcu::register::Register<Busych> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Busych, 0x28)
    }

    #[doc="Get the *mut pointer for the BUSYCH register."]
    #[inline] pub fn busych_mut(&self) -> *mut Busych { 
        self.busych_reg().ptr()
    }

    #[doc="Get the *const pointer for the BUSYCH register."]
    #[inline] pub fn busych_ptr(&self) -> *const Busych { 
        self.busych_reg().ptr()
    }

    #[doc="Read the BUSYCH register."]
    #[inline] pub fn busych(&self) -> Busych { 
        self.busych_reg().read()
    }

    #[doc="Get the CHCTRLA Register."]
    #[inline] pub fn chctrla_reg(&self) -> ::bobbin_mcu::register::Register<Chctrla> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Chctrla, 0x40)
    }

    #[doc="Get the *mut pointer for the CHCTRLA register."]
    #[inline] pub fn chctrla_mut(&self) -> *mut Chctrla { 
        self.chctrla_reg().ptr()
    }

    #[doc="Get the *const pointer for the CHCTRLA register."]
    #[inline] pub fn chctrla_ptr(&self) -> *const Chctrla { 
        self.chctrla_reg().ptr()
    }

    #[doc="Read the CHCTRLA register."]
    #[inline] pub fn chctrla(&self) -> Chctrla { 
        self.chctrla_reg().read()
    }

    #[doc="Write the CHCTRLA register."]
    #[inline] pub fn write_chctrla(&self, value: Chctrla) -> &Self { 
        self.chctrla_reg().write(value);
        self
    }

    #[doc="Set the CHCTRLA register."]
    #[inline] pub fn set_chctrla<F: FnOnce(Chctrla) -> Chctrla>(&self, f: F) -> &Self {
        self.chctrla_reg().set(f);
        self
    }

    #[doc="Modify the CHCTRLA register."]
    #[inline] pub fn with_chctrla<F: FnOnce(Chctrla) -> Chctrla>(&self, f: F) -> &Self {
        self.chctrla_reg().with(f);
        self
    }

    #[doc="Get the CHCTRLB Register."]
    #[inline] pub fn chctrlb_reg(&self) -> ::bobbin_mcu::register::Register<Chctrlb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Chctrlb, 0x44)
    }

    #[doc="Get the *mut pointer for the CHCTRLB register."]
    #[inline] pub fn chctrlb_mut(&self) -> *mut Chctrlb { 
        self.chctrlb_reg().ptr()
    }

    #[doc="Get the *const pointer for the CHCTRLB register."]
    #[inline] pub fn chctrlb_ptr(&self) -> *const Chctrlb { 
        self.chctrlb_reg().ptr()
    }

    #[doc="Read the CHCTRLB register."]
    #[inline] pub fn chctrlb(&self) -> Chctrlb { 
        self.chctrlb_reg().read()
    }

    #[doc="Write the CHCTRLB register."]
    #[inline] pub fn write_chctrlb(&self, value: Chctrlb) -> &Self { 
        self.chctrlb_reg().write(value);
        self
    }

    #[doc="Set the CHCTRLB register."]
    #[inline] pub fn set_chctrlb<F: FnOnce(Chctrlb) -> Chctrlb>(&self, f: F) -> &Self {
        self.chctrlb_reg().set(f);
        self
    }

    #[doc="Modify the CHCTRLB register."]
    #[inline] pub fn with_chctrlb<F: FnOnce(Chctrlb) -> Chctrlb>(&self, f: F) -> &Self {
        self.chctrlb_reg().with(f);
        self
    }

    #[doc="Get the CHID Register."]
    #[inline] pub fn chid_reg(&self) -> ::bobbin_mcu::register::Register<Chid> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Chid, 0x3f)
    }

    #[doc="Get the *mut pointer for the CHID register."]
    #[inline] pub fn chid_mut(&self) -> *mut Chid { 
        self.chid_reg().ptr()
    }

    #[doc="Get the *const pointer for the CHID register."]
    #[inline] pub fn chid_ptr(&self) -> *const Chid { 
        self.chid_reg().ptr()
    }

    #[doc="Read the CHID register."]
    #[inline] pub fn chid(&self) -> Chid { 
        self.chid_reg().read()
    }

    #[doc="Write the CHID register."]
    #[inline] pub fn write_chid(&self, value: Chid) -> &Self { 
        self.chid_reg().write(value);
        self
    }

    #[doc="Set the CHID register."]
    #[inline] pub fn set_chid<F: FnOnce(Chid) -> Chid>(&self, f: F) -> &Self {
        self.chid_reg().set(f);
        self
    }

    #[doc="Modify the CHID register."]
    #[inline] pub fn with_chid<F: FnOnce(Chid) -> Chid>(&self, f: F) -> &Self {
        self.chid_reg().with(f);
        self
    }

    #[doc="Get the CHINTENCLR Register."]
    #[inline] pub fn chintenclr_reg(&self) -> ::bobbin_mcu::register::Register<Chintenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Chintenclr, 0x4c)
    }

    #[doc="Get the *mut pointer for the CHINTENCLR register."]
    #[inline] pub fn chintenclr_mut(&self) -> *mut Chintenclr { 
        self.chintenclr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CHINTENCLR register."]
    #[inline] pub fn chintenclr_ptr(&self) -> *const Chintenclr { 
        self.chintenclr_reg().ptr()
    }

    #[doc="Read the CHINTENCLR register."]
    #[inline] pub fn chintenclr(&self) -> Chintenclr { 
        self.chintenclr_reg().read()
    }

    #[doc="Write the CHINTENCLR register."]
    #[inline] pub fn write_chintenclr(&self, value: Chintenclr) -> &Self { 
        self.chintenclr_reg().write(value);
        self
    }

    #[doc="Set the CHINTENCLR register."]
    #[inline] pub fn set_chintenclr<F: FnOnce(Chintenclr) -> Chintenclr>(&self, f: F) -> &Self {
        self.chintenclr_reg().set(f);
        self
    }

    #[doc="Modify the CHINTENCLR register."]
    #[inline] pub fn with_chintenclr<F: FnOnce(Chintenclr) -> Chintenclr>(&self, f: F) -> &Self {
        self.chintenclr_reg().with(f);
        self
    }

    #[doc="Get the CHINTENSET Register."]
    #[inline] pub fn chintenset_reg(&self) -> ::bobbin_mcu::register::Register<Chintenset> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Chintenset, 0x4d)
    }

    #[doc="Get the *mut pointer for the CHINTENSET register."]
    #[inline] pub fn chintenset_mut(&self) -> *mut Chintenset { 
        self.chintenset_reg().ptr()
    }

    #[doc="Get the *const pointer for the CHINTENSET register."]
    #[inline] pub fn chintenset_ptr(&self) -> *const Chintenset { 
        self.chintenset_reg().ptr()
    }

    #[doc="Read the CHINTENSET register."]
    #[inline] pub fn chintenset(&self) -> Chintenset { 
        self.chintenset_reg().read()
    }

    #[doc="Write the CHINTENSET register."]
    #[inline] pub fn write_chintenset(&self, value: Chintenset) -> &Self { 
        self.chintenset_reg().write(value);
        self
    }

    #[doc="Set the CHINTENSET register."]
    #[inline] pub fn set_chintenset<F: FnOnce(Chintenset) -> Chintenset>(&self, f: F) -> &Self {
        self.chintenset_reg().set(f);
        self
    }

    #[doc="Modify the CHINTENSET register."]
    #[inline] pub fn with_chintenset<F: FnOnce(Chintenset) -> Chintenset>(&self, f: F) -> &Self {
        self.chintenset_reg().with(f);
        self
    }

    #[doc="Get the CHINTFLAG Register."]
    #[inline] pub fn chintflag_reg(&self) -> ::bobbin_mcu::register::Register<Chintflag> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Chintflag, 0x4e)
    }

    #[doc="Get the *mut pointer for the CHINTFLAG register."]
    #[inline] pub fn chintflag_mut(&self) -> *mut Chintflag { 
        self.chintflag_reg().ptr()
    }

    #[doc="Get the *const pointer for the CHINTFLAG register."]
    #[inline] pub fn chintflag_ptr(&self) -> *const Chintflag { 
        self.chintflag_reg().ptr()
    }

    #[doc="Read the CHINTFLAG register."]
    #[inline] pub fn chintflag(&self) -> Chintflag { 
        self.chintflag_reg().read()
    }

    #[doc="Write the CHINTFLAG register."]
    #[inline] pub fn write_chintflag(&self, value: Chintflag) -> &Self { 
        self.chintflag_reg().write(value);
        self
    }

    #[doc="Set the CHINTFLAG register."]
    #[inline] pub fn set_chintflag<F: FnOnce(Chintflag) -> Chintflag>(&self, f: F) -> &Self {
        self.chintflag_reg().set(f);
        self
    }

    #[doc="Modify the CHINTFLAG register."]
    #[inline] pub fn with_chintflag<F: FnOnce(Chintflag) -> Chintflag>(&self, f: F) -> &Self {
        self.chintflag_reg().with(f);
        self
    }

    #[doc="Get the CHSTATUS Register."]
    #[inline] pub fn chstatus_reg(&self) -> ::bobbin_mcu::register::Register<Chstatus> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Chstatus, 0x4f)
    }

    #[doc="Get the *mut pointer for the CHSTATUS register."]
    #[inline] pub fn chstatus_mut(&self) -> *mut Chstatus { 
        self.chstatus_reg().ptr()
    }

    #[doc="Get the *const pointer for the CHSTATUS register."]
    #[inline] pub fn chstatus_ptr(&self) -> *const Chstatus { 
        self.chstatus_reg().ptr()
    }

    #[doc="Read the CHSTATUS register."]
    #[inline] pub fn chstatus(&self) -> Chstatus { 
        self.chstatus_reg().read()
    }

    #[doc="Get the CRCCHKSUM Register."]
    #[inline] pub fn crcchksum_reg(&self) -> ::bobbin_mcu::register::Register<Crcchksum> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Crcchksum, 0x8)
    }

    #[doc="Get the *mut pointer for the CRCCHKSUM register."]
    #[inline] pub fn crcchksum_mut(&self) -> *mut Crcchksum { 
        self.crcchksum_reg().ptr()
    }

    #[doc="Get the *const pointer for the CRCCHKSUM register."]
    #[inline] pub fn crcchksum_ptr(&self) -> *const Crcchksum { 
        self.crcchksum_reg().ptr()
    }

    #[doc="Read the CRCCHKSUM register."]
    #[inline] pub fn crcchksum(&self) -> Crcchksum { 
        self.crcchksum_reg().read()
    }

    #[doc="Write the CRCCHKSUM register."]
    #[inline] pub fn write_crcchksum(&self, value: Crcchksum) -> &Self { 
        self.crcchksum_reg().write(value);
        self
    }

    #[doc="Set the CRCCHKSUM register."]
    #[inline] pub fn set_crcchksum<F: FnOnce(Crcchksum) -> Crcchksum>(&self, f: F) -> &Self {
        self.crcchksum_reg().set(f);
        self
    }

    #[doc="Modify the CRCCHKSUM register."]
    #[inline] pub fn with_crcchksum<F: FnOnce(Crcchksum) -> Crcchksum>(&self, f: F) -> &Self {
        self.crcchksum_reg().with(f);
        self
    }

    #[doc="Get the CRCCTRL Register."]
    #[inline] pub fn crcctrl_reg(&self) -> ::bobbin_mcu::register::Register<Crcctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Crcctrl, 0x2)
    }

    #[doc="Get the *mut pointer for the CRCCTRL register."]
    #[inline] pub fn crcctrl_mut(&self) -> *mut Crcctrl { 
        self.crcctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CRCCTRL register."]
    #[inline] pub fn crcctrl_ptr(&self) -> *const Crcctrl { 
        self.crcctrl_reg().ptr()
    }

    #[doc="Read the CRCCTRL register."]
    #[inline] pub fn crcctrl(&self) -> Crcctrl { 
        self.crcctrl_reg().read()
    }

    #[doc="Write the CRCCTRL register."]
    #[inline] pub fn write_crcctrl(&self, value: Crcctrl) -> &Self { 
        self.crcctrl_reg().write(value);
        self
    }

    #[doc="Set the CRCCTRL register."]
    #[inline] pub fn set_crcctrl<F: FnOnce(Crcctrl) -> Crcctrl>(&self, f: F) -> &Self {
        self.crcctrl_reg().set(f);
        self
    }

    #[doc="Modify the CRCCTRL register."]
    #[inline] pub fn with_crcctrl<F: FnOnce(Crcctrl) -> Crcctrl>(&self, f: F) -> &Self {
        self.crcctrl_reg().with(f);
        self
    }

    #[doc="Get the CRCDATAIN Register."]
    #[inline] pub fn crcdatain_reg(&self) -> ::bobbin_mcu::register::Register<Crcdatain> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Crcdatain, 0x4)
    }

    #[doc="Get the *mut pointer for the CRCDATAIN register."]
    #[inline] pub fn crcdatain_mut(&self) -> *mut Crcdatain { 
        self.crcdatain_reg().ptr()
    }

    #[doc="Get the *const pointer for the CRCDATAIN register."]
    #[inline] pub fn crcdatain_ptr(&self) -> *const Crcdatain { 
        self.crcdatain_reg().ptr()
    }

    #[doc="Read the CRCDATAIN register."]
    #[inline] pub fn crcdatain(&self) -> Crcdatain { 
        self.crcdatain_reg().read()
    }

    #[doc="Write the CRCDATAIN register."]
    #[inline] pub fn write_crcdatain(&self, value: Crcdatain) -> &Self { 
        self.crcdatain_reg().write(value);
        self
    }

    #[doc="Set the CRCDATAIN register."]
    #[inline] pub fn set_crcdatain<F: FnOnce(Crcdatain) -> Crcdatain>(&self, f: F) -> &Self {
        self.crcdatain_reg().set(f);
        self
    }

    #[doc="Modify the CRCDATAIN register."]
    #[inline] pub fn with_crcdatain<F: FnOnce(Crcdatain) -> Crcdatain>(&self, f: F) -> &Self {
        self.crcdatain_reg().with(f);
        self
    }

    #[doc="Get the CRCSTATUS Register."]
    #[inline] pub fn crcstatus_reg(&self) -> ::bobbin_mcu::register::Register<Crcstatus> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Crcstatus, 0xc)
    }

    #[doc="Get the *mut pointer for the CRCSTATUS register."]
    #[inline] pub fn crcstatus_mut(&self) -> *mut Crcstatus { 
        self.crcstatus_reg().ptr()
    }

    #[doc="Get the *const pointer for the CRCSTATUS register."]
    #[inline] pub fn crcstatus_ptr(&self) -> *const Crcstatus { 
        self.crcstatus_reg().ptr()
    }

    #[doc="Read the CRCSTATUS register."]
    #[inline] pub fn crcstatus(&self) -> Crcstatus { 
        self.crcstatus_reg().read()
    }

    #[doc="Write the CRCSTATUS register."]
    #[inline] pub fn write_crcstatus(&self, value: Crcstatus) -> &Self { 
        self.crcstatus_reg().write(value);
        self
    }

    #[doc="Set the CRCSTATUS register."]
    #[inline] pub fn set_crcstatus<F: FnOnce(Crcstatus) -> Crcstatus>(&self, f: F) -> &Self {
        self.crcstatus_reg().set(f);
        self
    }

    #[doc="Modify the CRCSTATUS register."]
    #[inline] pub fn with_crcstatus<F: FnOnce(Crcstatus) -> Crcstatus>(&self, f: F) -> &Self {
        self.crcstatus_reg().with(f);
        self
    }

    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        self.ctrl_reg().read()
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn write_ctrl(&self, value: Ctrl) -> &Self { 
        self.ctrl_reg().write(value);
        self
    }

    #[doc="Set the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().set(f);
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().with(f);
        self
    }

    #[doc="Get the DBGCTRL Register."]
    #[inline] pub fn dbgctrl_reg(&self) -> ::bobbin_mcu::register::Register<Dbgctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dbgctrl, 0xd)
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

    #[doc="Get the INTPEND Register."]
    #[inline] pub fn intpend_reg(&self) -> ::bobbin_mcu::register::Register<Intpend> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intpend, 0x20)
    }

    #[doc="Get the *mut pointer for the INTPEND register."]
    #[inline] pub fn intpend_mut(&self) -> *mut Intpend { 
        self.intpend_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTPEND register."]
    #[inline] pub fn intpend_ptr(&self) -> *const Intpend { 
        self.intpend_reg().ptr()
    }

    #[doc="Read the INTPEND register."]
    #[inline] pub fn intpend(&self) -> Intpend { 
        self.intpend_reg().read()
    }

    #[doc="Write the INTPEND register."]
    #[inline] pub fn write_intpend(&self, value: Intpend) -> &Self { 
        self.intpend_reg().write(value);
        self
    }

    #[doc="Set the INTPEND register."]
    #[inline] pub fn set_intpend<F: FnOnce(Intpend) -> Intpend>(&self, f: F) -> &Self {
        self.intpend_reg().set(f);
        self
    }

    #[doc="Modify the INTPEND register."]
    #[inline] pub fn with_intpend<F: FnOnce(Intpend) -> Intpend>(&self, f: F) -> &Self {
        self.intpend_reg().with(f);
        self
    }

    #[doc="Get the INTSTATUS Register."]
    #[inline] pub fn intstatus_reg(&self) -> ::bobbin_mcu::register::Register<Intstatus> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intstatus, 0x24)
    }

    #[doc="Get the *mut pointer for the INTSTATUS register."]
    #[inline] pub fn intstatus_mut(&self) -> *mut Intstatus { 
        self.intstatus_reg().ptr()
    }

    #[doc="Get the *const pointer for the INTSTATUS register."]
    #[inline] pub fn intstatus_ptr(&self) -> *const Intstatus { 
        self.intstatus_reg().ptr()
    }

    #[doc="Read the INTSTATUS register."]
    #[inline] pub fn intstatus(&self) -> Intstatus { 
        self.intstatus_reg().read()
    }

    #[doc="Get the PENDCH Register."]
    #[inline] pub fn pendch_reg(&self) -> ::bobbin_mcu::register::Register<Pendch> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pendch, 0x2c)
    }

    #[doc="Get the *mut pointer for the PENDCH register."]
    #[inline] pub fn pendch_mut(&self) -> *mut Pendch { 
        self.pendch_reg().ptr()
    }

    #[doc="Get the *const pointer for the PENDCH register."]
    #[inline] pub fn pendch_ptr(&self) -> *const Pendch { 
        self.pendch_reg().ptr()
    }

    #[doc="Read the PENDCH register."]
    #[inline] pub fn pendch(&self) -> Pendch { 
        self.pendch_reg().read()
    }

    #[doc="Get the PRICTRL Register."]
    #[inline] pub fn prictrl_reg(&self) -> ::bobbin_mcu::register::Register<Prictrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Prictrl, 0x14)
    }

    #[doc="Get the *mut pointer for the PRICTRL register."]
    #[inline] pub fn prictrl_mut(&self) -> *mut Prictrl { 
        self.prictrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the PRICTRL register."]
    #[inline] pub fn prictrl_ptr(&self) -> *const Prictrl { 
        self.prictrl_reg().ptr()
    }

    #[doc="Read the PRICTRL register."]
    #[inline] pub fn prictrl(&self) -> Prictrl { 
        self.prictrl_reg().read()
    }

    #[doc="Write the PRICTRL register."]
    #[inline] pub fn write_prictrl(&self, value: Prictrl) -> &Self { 
        self.prictrl_reg().write(value);
        self
    }

    #[doc="Set the PRICTRL register."]
    #[inline] pub fn set_prictrl<F: FnOnce(Prictrl) -> Prictrl>(&self, f: F) -> &Self {
        self.prictrl_reg().set(f);
        self
    }

    #[doc="Modify the PRICTRL register."]
    #[inline] pub fn with_prictrl<F: FnOnce(Prictrl) -> Prictrl>(&self, f: F) -> &Self {
        self.prictrl_reg().with(f);
        self
    }

    #[doc="Get the SWTRIGCTRL Register."]
    #[inline] pub fn swtrigctrl_reg(&self) -> ::bobbin_mcu::register::Register<Swtrigctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Swtrigctrl, 0x10)
    }

    #[doc="Get the *mut pointer for the SWTRIGCTRL register."]
    #[inline] pub fn swtrigctrl_mut(&self) -> *mut Swtrigctrl { 
        self.swtrigctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the SWTRIGCTRL register."]
    #[inline] pub fn swtrigctrl_ptr(&self) -> *const Swtrigctrl { 
        self.swtrigctrl_reg().ptr()
    }

    #[doc="Read the SWTRIGCTRL register."]
    #[inline] pub fn swtrigctrl(&self) -> Swtrigctrl { 
        self.swtrigctrl_reg().read()
    }

    #[doc="Write the SWTRIGCTRL register."]
    #[inline] pub fn write_swtrigctrl(&self, value: Swtrigctrl) -> &Self { 
        self.swtrigctrl_reg().write(value);
        self
    }

    #[doc="Set the SWTRIGCTRL register."]
    #[inline] pub fn set_swtrigctrl<F: FnOnce(Swtrigctrl) -> Swtrigctrl>(&self, f: F) -> &Self {
        self.swtrigctrl_reg().set(f);
        self
    }

    #[doc="Modify the SWTRIGCTRL register."]
    #[inline] pub fn with_swtrigctrl<F: FnOnce(Swtrigctrl) -> Swtrigctrl>(&self, f: F) -> &Self {
        self.swtrigctrl_reg().with(f);
        self
    }

    #[doc="Get the WRBADDR Register."]
    #[inline] pub fn wrbaddr_reg(&self) -> ::bobbin_mcu::register::Register<Wrbaddr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Wrbaddr, 0x38)
    }

    #[doc="Get the *mut pointer for the WRBADDR register."]
    #[inline] pub fn wrbaddr_mut(&self) -> *mut Wrbaddr { 
        self.wrbaddr_reg().ptr()
    }

    #[doc="Get the *const pointer for the WRBADDR register."]
    #[inline] pub fn wrbaddr_ptr(&self) -> *const Wrbaddr { 
        self.wrbaddr_reg().ptr()
    }

    #[doc="Read the WRBADDR register."]
    #[inline] pub fn wrbaddr(&self) -> Wrbaddr { 
        self.wrbaddr_reg().read()
    }

    #[doc="Write the WRBADDR register."]
    #[inline] pub fn write_wrbaddr(&self, value: Wrbaddr) -> &Self { 
        self.wrbaddr_reg().write(value);
        self
    }

    #[doc="Set the WRBADDR register."]
    #[inline] pub fn set_wrbaddr<F: FnOnce(Wrbaddr) -> Wrbaddr>(&self, f: F) -> &Self {
        self.wrbaddr_reg().set(f);
        self
    }

    #[doc="Modify the WRBADDR register."]
    #[inline] pub fn with_wrbaddr<F: FnOnce(Wrbaddr) -> Wrbaddr>(&self, f: F) -> &Self {
        self.wrbaddr_reg().with(f);
        self
    }

}

#[doc="Active Channel and Levels"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Active(pub u32);
impl Active {
    #[doc="Level n Channel Trigger Request Executing"]
    #[inline] pub fn lvlex<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LVLEX != 0"]
    #[inline] pub fn test_lvlex<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.lvlex(index) != 0
    }

    #[doc="Sets the LVLEX field."]
    #[inline] pub fn set_lvlex<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Active Channel ID"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Active Channel Busy"]
    #[inline] pub fn abusy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ABUSY != 0"]
    #[inline] pub fn test_abusy(&self) -> bool {
        self.abusy() != 0
    }

    #[doc="Sets the ABUSY field."]
    #[inline] pub fn set_abusy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Active Channel Block Transfer Count"]
    #[inline] pub fn btcnt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if BTCNT != 0"]
    #[inline] pub fn test_btcnt(&self) -> bool {
        self.btcnt() != 0
    }

    #[doc="Sets the BTCNT field."]
    #[inline] pub fn set_btcnt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Active {
    #[inline]
    fn from(other: u32) -> Self {
         Active(other)
    }
}

impl ::core::fmt::Display for Active {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Active {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lvlex(0) != 0 { try!(write!(f, " lvlex[0]"))}
        if self.lvlex(1) != 0 { try!(write!(f, " lvlex[1]"))}
        if self.lvlex(2) != 0 { try!(write!(f, " lvlex[2]"))}
        if self.lvlex(3) != 0 { try!(write!(f, " lvlex[3]"))}
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        if self.abusy() != 0 { try!(write!(f, " abusy"))}
        if self.btcnt() != 0 { try!(write!(f, " btcnt=0x{:x}", self.btcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Descriptor Memory Section Base Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Baseaddr(pub u32);
impl Baseaddr {
    #[doc="Descriptor Memory Base Address"]
    #[inline] pub fn baseaddr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if BASEADDR != 0"]
    #[inline] pub fn test_baseaddr(&self) -> bool {
        self.baseaddr() != 0
    }

    #[doc="Sets the BASEADDR field."]
    #[inline] pub fn set_baseaddr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Baseaddr {
    #[inline]
    fn from(other: u32) -> Self {
         Baseaddr(other)
    }
}

impl ::core::fmt::Display for Baseaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Baseaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Busy Channels"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Busych(pub u32);
impl Busych {
    #[doc="Busy Channel n"]
    #[inline] pub fn busych<I: Into<::bobbin_bits::R12>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BUSYCH != 0"]
    #[inline] pub fn test_busych<I: Into<::bobbin_bits::R12>>(&self, index: I) -> bool{
        self.busych(index) != 0
    }

    #[doc="Sets the BUSYCH field."]
    #[inline] pub fn set_busych<I: Into<::bobbin_bits::R12>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Busych {
    #[inline]
    fn from(other: u32) -> Self {
         Busych(other)
    }
}

impl ::core::fmt::Display for Busych {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Busych {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.busych(0) != 0 { try!(write!(f, " busych[0]"))}
        if self.busych(1) != 0 { try!(write!(f, " busych[1]"))}
        if self.busych(2) != 0 { try!(write!(f, " busych[2]"))}
        if self.busych(3) != 0 { try!(write!(f, " busych[3]"))}
        if self.busych(4) != 0 { try!(write!(f, " busych[4]"))}
        if self.busych(5) != 0 { try!(write!(f, " busych[5]"))}
        if self.busych(6) != 0 { try!(write!(f, " busych[6]"))}
        if self.busych(7) != 0 { try!(write!(f, " busych[7]"))}
        if self.busych(8) != 0 { try!(write!(f, " busych[8]"))}
        if self.busych(9) != 0 { try!(write!(f, " busych[9]"))}
        if self.busych(10) != 0 { try!(write!(f, " busych[10]"))}
        if self.busych(11) != 0 { try!(write!(f, " busych[11]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chctrla(pub u8);
impl Chctrla {
    #[doc="Channel Software Reset"]
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Enable"]
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

}

impl From<u8> for Chctrla {
    #[inline]
    fn from(other: u8) -> Self {
         Chctrla(other)
    }
}

impl ::core::fmt::Display for Chctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chctrlb(pub u32);
impl Chctrlb {
    #[doc="Event Input Action"]
    #[inline] pub fn evact(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EVACT != 0"]
    #[inline] pub fn test_evact(&self) -> bool {
        self.evact() != 0
    }

    #[doc="Sets the EVACT field."]
    #[inline] pub fn set_evact<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Event Input Enable"]
    #[inline] pub fn evie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EVIE != 0"]
    #[inline] pub fn test_evie(&self) -> bool {
        self.evie() != 0
    }

    #[doc="Sets the EVIE field."]
    #[inline] pub fn set_evie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel Event Output Enable"]
    #[inline] pub fn evoe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EVOE != 0"]
    #[inline] pub fn test_evoe(&self) -> bool {
        self.evoe() != 0
    }

    #[doc="Sets the EVOE field."]
    #[inline] pub fn set_evoe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel Arbitration Level"]
    #[inline] pub fn lvl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if LVL != 0"]
    #[inline] pub fn test_lvl(&self) -> bool {
        self.lvl() != 0
    }

    #[doc="Sets the LVL field."]
    #[inline] pub fn set_lvl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Peripheral Trigger Source"]
    #[inline] pub fn trigsrc(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if TRIGSRC != 0"]
    #[inline] pub fn test_trigsrc(&self) -> bool {
        self.trigsrc() != 0
    }

    #[doc="Sets the TRIGSRC field."]
    #[inline] pub fn set_trigsrc<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Trigger Action"]
    #[inline] pub fn trigact(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if TRIGACT != 0"]
    #[inline] pub fn test_trigact(&self) -> bool {
        self.trigact() != 0
    }

    #[doc="Sets the TRIGACT field."]
    #[inline] pub fn set_trigact<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Software Command"]
    #[inline] pub fn cmd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Chctrlb {
    #[inline]
    fn from(other: u32) -> Self {
         Chctrlb(other)
    }
}

impl ::core::fmt::Display for Chctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chctrlb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
        if self.evie() != 0 { try!(write!(f, " evie"))}
        if self.evoe() != 0 { try!(write!(f, " evoe"))}
        if self.lvl() != 0 { try!(write!(f, " lvl=0x{:x}", self.lvl()))}
        if self.trigsrc() != 0 { try!(write!(f, " trigsrc=0x{:x}", self.trigsrc()))}
        if self.trigact() != 0 { try!(write!(f, " trigact=0x{:x}", self.trigact()))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel ID"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chid(pub u8);
impl Chid {
    #[doc="Channel ID"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Chid {
    #[inline]
    fn from(other: u8) -> Self {
         Chid(other)
    }
}

impl ::core::fmt::Display for Chid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chintenclr(pub u8);
impl Chintenclr {
    #[doc="Transfer Error Interrupt Enable"]
    #[inline] pub fn terr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TERR != 0"]
    #[inline] pub fn test_terr(&self) -> bool {
        self.terr() != 0
    }

    #[doc="Sets the TERR field."]
    #[inline] pub fn set_terr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete Interrupt Enable"]
    #[inline] pub fn tcmpl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TCMPL != 0"]
    #[inline] pub fn test_tcmpl(&self) -> bool {
        self.tcmpl() != 0
    }

    #[doc="Sets the TCMPL field."]
    #[inline] pub fn set_tcmpl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel Suspend Interrupt Enable"]
    #[inline] pub fn susp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Chintenclr {
    #[inline]
    fn from(other: u8) -> Self {
         Chintenclr(other)
    }
}

impl ::core::fmt::Display for Chintenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chintenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.terr() != 0 { try!(write!(f, " terr"))}
        if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chintenset(pub u8);
impl Chintenset {
    #[doc="Transfer Error Interrupt Enable"]
    #[inline] pub fn terr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TERR != 0"]
    #[inline] pub fn test_terr(&self) -> bool {
        self.terr() != 0
    }

    #[doc="Sets the TERR field."]
    #[inline] pub fn set_terr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete Interrupt Enable"]
    #[inline] pub fn tcmpl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TCMPL != 0"]
    #[inline] pub fn test_tcmpl(&self) -> bool {
        self.tcmpl() != 0
    }

    #[doc="Sets the TCMPL field."]
    #[inline] pub fn set_tcmpl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel Suspend Interrupt Enable"]
    #[inline] pub fn susp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Chintenset {
    #[inline]
    fn from(other: u8) -> Self {
         Chintenset(other)
    }
}

impl ::core::fmt::Display for Chintenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chintenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.terr() != 0 { try!(write!(f, " terr"))}
        if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chintflag(pub u8);
impl Chintflag {
    #[doc="Transfer Error"]
    #[inline] pub fn terr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TERR != 0"]
    #[inline] pub fn test_terr(&self) -> bool {
        self.terr() != 0
    }

    #[doc="Sets the TERR field."]
    #[inline] pub fn set_terr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete"]
    #[inline] pub fn tcmpl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TCMPL != 0"]
    #[inline] pub fn test_tcmpl(&self) -> bool {
        self.tcmpl() != 0
    }

    #[doc="Sets the TCMPL field."]
    #[inline] pub fn set_tcmpl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel Suspend"]
    #[inline] pub fn susp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Chintflag {
    #[inline]
    fn from(other: u8) -> Self {
         Chintflag(other)
    }
}

impl ::core::fmt::Display for Chintflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chintflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.terr() != 0 { try!(write!(f, " terr"))}
        if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chstatus(pub u8);
impl Chstatus {
    #[doc="Channel Pending"]
    #[inline] pub fn pend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PEND != 0"]
    #[inline] pub fn test_pend(&self) -> bool {
        self.pend() != 0
    }

    #[doc="Sets the PEND field."]
    #[inline] pub fn set_pend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Busy"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Fetch Error"]
    #[inline] pub fn ferr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Chstatus {
    #[inline]
    fn from(other: u8) -> Self {
         Chstatus(other)
    }
}

impl ::core::fmt::Display for Chstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pend() != 0 { try!(write!(f, " pend"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC Checksum"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crcchksum(pub u32);
impl Crcchksum {
    #[doc="CRC Checksum"]
    #[inline] pub fn crcchksum(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CRCCHKSUM != 0"]
    #[inline] pub fn test_crcchksum(&self) -> bool {
        self.crcchksum() != 0
    }

    #[doc="Sets the CRCCHKSUM field."]
    #[inline] pub fn set_crcchksum<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Crcchksum {
    #[inline]
    fn from(other: u32) -> Self {
         Crcchksum(other)
    }
}

impl ::core::fmt::Display for Crcchksum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crcchksum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crcctrl(pub u16);
impl Crcctrl {
    #[doc="CRC Beat Size"]
    #[inline] pub fn crcbeatsize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CRCBEATSIZE != 0"]
    #[inline] pub fn test_crcbeatsize(&self) -> bool {
        self.crcbeatsize() != 0
    }

    #[doc="Sets the CRCBEATSIZE field."]
    #[inline] pub fn set_crcbeatsize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CRC Polynomial Type"]
    #[inline] pub fn crcpoly(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if CRCPOLY != 0"]
    #[inline] pub fn test_crcpoly(&self) -> bool {
        self.crcpoly() != 0
    }

    #[doc="Sets the CRCPOLY field."]
    #[inline] pub fn set_crcpoly<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="CRC Input Source"]
    #[inline] pub fn crcsrc(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if CRCSRC != 0"]
    #[inline] pub fn test_crcsrc(&self) -> bool {
        self.crcsrc() != 0
    }

    #[doc="Sets the CRCSRC field."]
    #[inline] pub fn set_crcsrc<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u16> for Crcctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Crcctrl(other)
    }
}

impl ::core::fmt::Display for Crcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crcctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.crcbeatsize() != 0 { try!(write!(f, " crcbeatsize=0x{:x}", self.crcbeatsize()))}
        if self.crcpoly() != 0 { try!(write!(f, " crcpoly=0x{:x}", self.crcpoly()))}
        if self.crcsrc() != 0 { try!(write!(f, " crcsrc=0x{:x}", self.crcsrc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC Data Input"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crcdatain(pub u32);
impl Crcdatain {
    #[doc="CRC Data Input"]
    #[inline] pub fn crcdatain(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CRCDATAIN != 0"]
    #[inline] pub fn test_crcdatain(&self) -> bool {
        self.crcdatain() != 0
    }

    #[doc="Sets the CRCDATAIN field."]
    #[inline] pub fn set_crcdatain<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Crcdatain {
    #[inline]
    fn from(other: u32) -> Self {
         Crcdatain(other)
    }
}

impl ::core::fmt::Display for Crcdatain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crcdatain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crcstatus(pub u8);
impl Crcstatus {
    #[doc="CRC Module Busy"]
    #[inline] pub fn crcbusy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CRCBUSY != 0"]
    #[inline] pub fn test_crcbusy(&self) -> bool {
        self.crcbusy() != 0
    }

    #[doc="Sets the CRCBUSY field."]
    #[inline] pub fn set_crcbusy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CRC Zero"]
    #[inline] pub fn crczero(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CRCZERO != 0"]
    #[inline] pub fn test_crczero(&self) -> bool {
        self.crczero() != 0
    }

    #[doc="Sets the CRCZERO field."]
    #[inline] pub fn set_crczero<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Crcstatus {
    #[inline]
    fn from(other: u8) -> Self {
         Crcstatus(other)
    }
}

impl ::core::fmt::Display for Crcstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crcstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.crcbusy() != 0 { try!(write!(f, " crcbusy"))}
        if self.crczero() != 0 { try!(write!(f, " crczero"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u16);
impl Ctrl {
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

    #[doc="DMA Enable"]
    #[inline] pub fn dmaenable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMAENABLE != 0"]
    #[inline] pub fn test_dmaenable(&self) -> bool {
        self.dmaenable() != 0
    }

    #[doc="Sets the DMAENABLE field."]
    #[inline] pub fn set_dmaenable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CRC Enable"]
    #[inline] pub fn crcenable(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CRCENABLE != 0"]
    #[inline] pub fn test_crcenable(&self) -> bool {
        self.crcenable() != 0
    }

    #[doc="Sets the CRCENABLE field."]
    #[inline] pub fn set_crcenable<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Priority Level n Enable"]
    #[inline] pub fn lvlen<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LVLEN != 0"]
    #[inline] pub fn test_lvlen<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.lvlen(index) != 0
    }

    #[doc="Sets the LVLEN field."]
    #[inline] pub fn set_lvlen<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u16> for Ctrl {
    #[inline]
    fn from(other: u16) -> Self {
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
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.dmaenable() != 0 { try!(write!(f, " dmaenable"))}
        if self.crcenable() != 0 { try!(write!(f, " crcenable"))}
        if self.lvlen(0) != 0 { try!(write!(f, " lvlen[0]"))}
        if self.lvlen(1) != 0 { try!(write!(f, " lvlen[1]"))}
        if self.lvlen(2) != 0 { try!(write!(f, " lvlen[2]"))}
        if self.lvlen(3) != 0 { try!(write!(f, " lvlen[3]"))}
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

#[doc="Interrupt Pending"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intpend(pub u16);
impl Intpend {
    #[doc="Channel ID"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Error"]
    #[inline] pub fn terr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TERR != 0"]
    #[inline] pub fn test_terr(&self) -> bool {
        self.terr() != 0
    }

    #[doc="Sets the TERR field."]
    #[inline] pub fn set_terr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transfer Complete"]
    #[inline] pub fn tcmpl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TCMPL != 0"]
    #[inline] pub fn test_tcmpl(&self) -> bool {
        self.tcmpl() != 0
    }

    #[doc="Sets the TCMPL field."]
    #[inline] pub fn set_tcmpl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Channel Suspend"]
    #[inline] pub fn susp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SUSP != 0"]
    #[inline] pub fn test_susp(&self) -> bool {
        self.susp() != 0
    }

    #[doc="Sets the SUSP field."]
    #[inline] pub fn set_susp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Fetch Error"]
    #[inline] pub fn ferr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FERR != 0"]
    #[inline] pub fn test_ferr(&self) -> bool {
        self.ferr() != 0
    }

    #[doc="Sets the FERR field."]
    #[inline] pub fn set_ferr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn busy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pending"]
    #[inline] pub fn pend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PEND != 0"]
    #[inline] pub fn test_pend(&self) -> bool {
        self.pend() != 0
    }

    #[doc="Sets the PEND field."]
    #[inline] pub fn set_pend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Intpend {
    #[inline]
    fn from(other: u16) -> Self {
         Intpend(other)
    }
}

impl ::core::fmt::Display for Intpend {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intpend {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        if self.terr() != 0 { try!(write!(f, " terr"))}
        if self.tcmpl() != 0 { try!(write!(f, " tcmpl"))}
        if self.susp() != 0 { try!(write!(f, " susp"))}
        if self.ferr() != 0 { try!(write!(f, " ferr"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.pend() != 0 { try!(write!(f, " pend"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intstatus(pub u32);
impl Intstatus {
    #[doc="Channel n Pending Interrupt"]
    #[inline] pub fn chint<I: Into<::bobbin_bits::R12>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHINT != 0"]
    #[inline] pub fn test_chint<I: Into<::bobbin_bits::R12>>(&self, index: I) -> bool{
        self.chint(index) != 0
    }

    #[doc="Sets the CHINT field."]
    #[inline] pub fn set_chint<I: Into<::bobbin_bits::R12>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Intstatus {
    #[inline]
    fn from(other: u32) -> Self {
         Intstatus(other)
    }
}

impl ::core::fmt::Display for Intstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.chint(0) != 0 { try!(write!(f, " chint[0]"))}
        if self.chint(1) != 0 { try!(write!(f, " chint[1]"))}
        if self.chint(2) != 0 { try!(write!(f, " chint[2]"))}
        if self.chint(3) != 0 { try!(write!(f, " chint[3]"))}
        if self.chint(4) != 0 { try!(write!(f, " chint[4]"))}
        if self.chint(5) != 0 { try!(write!(f, " chint[5]"))}
        if self.chint(6) != 0 { try!(write!(f, " chint[6]"))}
        if self.chint(7) != 0 { try!(write!(f, " chint[7]"))}
        if self.chint(8) != 0 { try!(write!(f, " chint[8]"))}
        if self.chint(9) != 0 { try!(write!(f, " chint[9]"))}
        if self.chint(10) != 0 { try!(write!(f, " chint[10]"))}
        if self.chint(11) != 0 { try!(write!(f, " chint[11]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pending Channels"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pendch(pub u32);
impl Pendch {
    #[doc="Pending Channel n"]
    #[inline] pub fn pendch<I: Into<::bobbin_bits::R12>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PENDCH != 0"]
    #[inline] pub fn test_pendch<I: Into<::bobbin_bits::R12>>(&self, index: I) -> bool{
        self.pendch(index) != 0
    }

    #[doc="Sets the PENDCH field."]
    #[inline] pub fn set_pendch<I: Into<::bobbin_bits::R12>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pendch {
    #[inline]
    fn from(other: u32) -> Self {
         Pendch(other)
    }
}

impl ::core::fmt::Display for Pendch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pendch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pendch(0) != 0 { try!(write!(f, " pendch[0]"))}
        if self.pendch(1) != 0 { try!(write!(f, " pendch[1]"))}
        if self.pendch(2) != 0 { try!(write!(f, " pendch[2]"))}
        if self.pendch(3) != 0 { try!(write!(f, " pendch[3]"))}
        if self.pendch(4) != 0 { try!(write!(f, " pendch[4]"))}
        if self.pendch(5) != 0 { try!(write!(f, " pendch[5]"))}
        if self.pendch(6) != 0 { try!(write!(f, " pendch[6]"))}
        if self.pendch(7) != 0 { try!(write!(f, " pendch[7]"))}
        if self.pendch(8) != 0 { try!(write!(f, " pendch[8]"))}
        if self.pendch(9) != 0 { try!(write!(f, " pendch[9]"))}
        if self.pendch(10) != 0 { try!(write!(f, " pendch[10]"))}
        if self.pendch(11) != 0 { try!(write!(f, " pendch[11]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Priority Control 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prictrl(pub u32);
impl Prictrl {
    #[doc="Level n Channel Priority Number"]
    #[inline] pub fn lvlpri<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if LVLPRI != 0"]
    #[inline] pub fn test_lvlpri<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.lvlpri(index) != 0
    }

    #[doc="Sets the LVLPRI field."]
    #[inline] pub fn set_lvlpri<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Level 0 Round-Robin Scheduling Enable"]
    #[inline] pub fn rrlvlen<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 7 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RRLVLEN != 0"]
    #[inline] pub fn test_rrlvlen<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.rrlvlen(index) != 0
    }

    #[doc="Sets the RRLVLEN field."]
    #[inline] pub fn set_rrlvlen<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 7 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Prictrl {
    #[inline]
    fn from(other: u32) -> Self {
         Prictrl(other)
    }
}

impl ::core::fmt::Display for Prictrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prictrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lvlpri(0) != 0 { try!(write!(f, " lvlpri[0]=0x{:x}", self.lvlpri(0)))}
        if self.lvlpri(1) != 0 { try!(write!(f, " lvlpri[1]=0x{:x}", self.lvlpri(1)))}
        if self.lvlpri(2) != 0 { try!(write!(f, " lvlpri[2]=0x{:x}", self.lvlpri(2)))}
        if self.lvlpri(3) != 0 { try!(write!(f, " lvlpri[3]=0x{:x}", self.lvlpri(3)))}
        if self.rrlvlen(0) != 0 { try!(write!(f, " rrlvlen[0]"))}
        if self.rrlvlen(1) != 0 { try!(write!(f, " rrlvlen[1]"))}
        if self.rrlvlen(2) != 0 { try!(write!(f, " rrlvlen[2]"))}
        if self.rrlvlen(3) != 0 { try!(write!(f, " rrlvlen[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Software Trigger Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swtrigctrl(pub u32);
impl Swtrigctrl {
    #[doc="Channel n Software Trigger"]
    #[inline] pub fn swtrig<I: Into<::bobbin_bits::R12>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWTRIG != 0"]
    #[inline] pub fn test_swtrig<I: Into<::bobbin_bits::R12>>(&self, index: I) -> bool{
        self.swtrig(index) != 0
    }

    #[doc="Sets the SWTRIG field."]
    #[inline] pub fn set_swtrig<I: Into<::bobbin_bits::R12>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Swtrigctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Swtrigctrl(other)
    }
}

impl ::core::fmt::Display for Swtrigctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swtrigctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swtrig(0) != 0 { try!(write!(f, " swtrig[0]"))}
        if self.swtrig(1) != 0 { try!(write!(f, " swtrig[1]"))}
        if self.swtrig(2) != 0 { try!(write!(f, " swtrig[2]"))}
        if self.swtrig(3) != 0 { try!(write!(f, " swtrig[3]"))}
        if self.swtrig(4) != 0 { try!(write!(f, " swtrig[4]"))}
        if self.swtrig(5) != 0 { try!(write!(f, " swtrig[5]"))}
        if self.swtrig(6) != 0 { try!(write!(f, " swtrig[6]"))}
        if self.swtrig(7) != 0 { try!(write!(f, " swtrig[7]"))}
        if self.swtrig(8) != 0 { try!(write!(f, " swtrig[8]"))}
        if self.swtrig(9) != 0 { try!(write!(f, " swtrig[9]"))}
        if self.swtrig(10) != 0 { try!(write!(f, " swtrig[10]"))}
        if self.swtrig(11) != 0 { try!(write!(f, " swtrig[11]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Write-Back Memory Section Base Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrbaddr(pub u32);
impl Wrbaddr {
    #[doc="Write-Back Memory Base Address"]
    #[inline] pub fn wrbaddr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if WRBADDR != 0"]
    #[inline] pub fn test_wrbaddr(&self) -> bool {
        self.wrbaddr() != 0
    }

    #[doc="Sets the WRBADDR field."]
    #[inline] pub fn set_wrbaddr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Wrbaddr {
    #[inline]
    fn from(other: u32) -> Self {
         Wrbaddr(other)
    }
}

impl ::core::fmt::Display for Wrbaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrbaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


#[doc="DMAC Block Transfer Descriptor"]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transfer(pub [u8; 16]);

impl Transfer {
#[doc="Read the BTCTRL register."]
    #[inline] pub fn btctrl(&self) -> Btctrl { 
        unsafe {
            ::core::ptr::read_volatile(self.0.as_ptr().offset(0x0) as *const Btctrl)
        }
    }

#[doc="Write the BTCTRL register."]
    #[inline] pub fn set_btctrl<F: FnOnce(Btctrl) -> Btctrl>(&mut self, f: F) -> &mut Self {
        unsafe {
            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x0) as *mut Btctrl, f(Btctrl(0)));
        }
        self
  }

#[doc="Modfy the BTCTRL register."]
    #[inline] pub fn with_btctrl<F: FnOnce(Btctrl) -> Btctrl>(&mut self, f: F) -> &mut Self {
        unsafe {
            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x0) as *mut Btctrl, f(self.btctrl()));
        }
      self
    }


#[doc="Read the BTCNT register."]
    #[inline] pub fn btcnt(&self) -> Btcnt { 
        unsafe {
            ::core::ptr::read_volatile(self.0.as_ptr().offset(0x2) as *const Btcnt)
        }
    }

#[doc="Write the BTCNT register."]
    #[inline] pub fn set_btcnt<F: FnOnce(Btcnt) -> Btcnt>(&mut self, f: F) -> &mut Self {
        unsafe {
            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x2) as *mut Btcnt, f(Btcnt(0)));
        }
        self
  }

#[doc="Modfy the BTCNT register."]
    #[inline] pub fn with_btcnt<F: FnOnce(Btcnt) -> Btcnt>(&mut self, f: F) -> &mut Self {
        unsafe {
            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x2) as *mut Btcnt, f(self.btcnt()));
        }
      self
    }


#[doc="Read the SRCADDR register."]
    #[inline] pub fn srcaddr(&self) -> Srcaddr { 
        unsafe {
            ::core::ptr::read_volatile(self.0.as_ptr().offset(0x4) as *const Srcaddr)
        }
    }

#[doc="Write the SRCADDR register."]
    #[inline] pub fn set_srcaddr<F: FnOnce(Srcaddr) -> Srcaddr>(&mut self, f: F) -> &mut Self {
        unsafe {
            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x4) as *mut Srcaddr, f(Srcaddr(0)));
        }
        self
  }

#[doc="Modfy the SRCADDR register."]
    #[inline] pub fn with_srcaddr<F: FnOnce(Srcaddr) -> Srcaddr>(&mut self, f: F) -> &mut Self {
        unsafe {
            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x4) as *mut Srcaddr, f(self.srcaddr()));
        }
      self
    }


#[doc="Read the DSTADDR register."]
    #[inline] pub fn dstaddr(&self) -> Dstaddr { 
        unsafe {
            ::core::ptr::read_volatile(self.0.as_ptr().offset(0x8) as *const Dstaddr)
        }
    }

#[doc="Write the DSTADDR register."]
    #[inline] pub fn set_dstaddr<F: FnOnce(Dstaddr) -> Dstaddr>(&mut self, f: F) -> &mut Self {
        unsafe {
            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x8) as *mut Dstaddr, f(Dstaddr(0)));
        }
        self
  }

#[doc="Modfy the DSTADDR register."]
    #[inline] pub fn with_dstaddr<F: FnOnce(Dstaddr) -> Dstaddr>(&mut self, f: F) -> &mut Self {
        unsafe {
            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x8) as *mut Dstaddr, f(self.dstaddr()));
        }
      self
    }


#[doc="Read the DESCADDR register."]
    #[inline] pub fn descaddr(&self) -> Descaddr { 
        unsafe {
            ::core::ptr::read_volatile(self.0.as_ptr().offset(0xc) as *const Descaddr)
        }
    }

#[doc="Write the DESCADDR register."]
    #[inline] pub fn set_descaddr<F: FnOnce(Descaddr) -> Descaddr>(&mut self, f: F) -> &mut Self {
        unsafe {
            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0xc) as *mut Descaddr, f(Descaddr(0)));
        }
        self
  }

#[doc="Modfy the DESCADDR register."]
    #[inline] pub fn with_descaddr<F: FnOnce(Descaddr) -> Descaddr>(&mut self, f: F) -> &mut Self {
        unsafe {
            ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0xc) as *mut Descaddr, f(self.descaddr()));
        }
      self
    }


}
#[doc="Block Transfer Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Btctrl(pub u16);
impl Btctrl {
    #[doc="Descriptor Valid"]
    #[inline] pub fn valid(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if VALID != 0"]
    #[inline] pub fn test_valid(&self) -> bool {
        self.valid() != 0
    }

    #[doc="Sets the VALID field."]
    #[inline] pub fn set_valid<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Event Output Selection"]
    #[inline] pub fn evosel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if EVOSEL != 0"]
    #[inline] pub fn test_evosel(&self) -> bool {
        self.evosel() != 0
    }

    #[doc="Sets the EVOSEL field."]
    #[inline] pub fn set_evosel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Block Action"]
    #[inline] pub fn blockact(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if BLOCKACT != 0"]
    #[inline] pub fn test_blockact(&self) -> bool {
        self.blockact() != 0
    }

    #[doc="Sets the BLOCKACT field."]
    #[inline] pub fn set_blockact<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Beat Size"]
    #[inline] pub fn beatsize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if BEATSIZE != 0"]
    #[inline] pub fn test_beatsize(&self) -> bool {
        self.beatsize() != 0
    }

    #[doc="Sets the BEATSIZE field."]
    #[inline] pub fn set_beatsize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Source Address Increment Enable"]
    #[inline] pub fn srcinc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SRCINC != 0"]
    #[inline] pub fn test_srcinc(&self) -> bool {
        self.srcinc() != 0
    }

    #[doc="Sets the SRCINC field."]
    #[inline] pub fn set_srcinc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Destination Address Increment Enable"]
    #[inline] pub fn dstinc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DSTINC != 0"]
    #[inline] pub fn test_dstinc(&self) -> bool {
        self.dstinc() != 0
    }

    #[doc="Sets the DSTINC field."]
    #[inline] pub fn set_dstinc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Step Selection"]
    #[inline] pub fn stepsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if STEPSEL != 0"]
    #[inline] pub fn test_stepsel(&self) -> bool {
        self.stepsel() != 0
    }

    #[doc="Sets the STEPSEL field."]
    #[inline] pub fn set_stepsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Address Increment Step Size"]
    #[inline] pub fn stepsize(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if STEPSIZE != 0"]
    #[inline] pub fn test_stepsize(&self) -> bool {
        self.stepsize() != 0
    }

    #[doc="Sets the STEPSIZE field."]
    #[inline] pub fn set_stepsize<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for Btctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Btctrl(other)
    }
}

impl ::core::fmt::Display for Btctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Btctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.valid() != 0 { try!(write!(f, " valid"))}
        if self.evosel() != 0 { try!(write!(f, " evosel=0x{:x}", self.evosel()))}
        if self.blockact() != 0 { try!(write!(f, " blockact=0x{:x}", self.blockact()))}
        if self.beatsize() != 0 { try!(write!(f, " beatsize=0x{:x}", self.beatsize()))}
        if self.srcinc() != 0 { try!(write!(f, " srcinc"))}
        if self.dstinc() != 0 { try!(write!(f, " dstinc"))}
        if self.stepsel() != 0 { try!(write!(f, " stepsel"))}
        if self.stepsize() != 0 { try!(write!(f, " stepsize=0x{:x}", self.stepsize()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Block Transfer Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Btcnt(pub u16);
impl Btcnt {
    #[doc="Block Transfer Count"]
    #[inline] pub fn btcnt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if BTCNT != 0"]
    #[inline] pub fn test_btcnt(&self) -> bool {
        self.btcnt() != 0
    }

    #[doc="Sets the BTCNT field."]
    #[inline] pub fn set_btcnt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Btcnt {
    #[inline]
    fn from(other: u16) -> Self {
         Btcnt(other)
    }
}

impl ::core::fmt::Display for Btcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Btcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.btcnt() != 0 { try!(write!(f, " btcnt=0x{:x}", self.btcnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Block Transfer Source Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srcaddr(pub u32);
impl Srcaddr {
    #[doc="Transfer Source Address"]
    #[inline] pub fn srcaddr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SRCADDR != 0"]
    #[inline] pub fn test_srcaddr(&self) -> bool {
        self.srcaddr() != 0
    }

    #[doc="Sets the SRCADDR field."]
    #[inline] pub fn set_srcaddr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Srcaddr {
    #[inline]
    fn from(other: u32) -> Self {
         Srcaddr(other)
    }
}

impl ::core::fmt::Display for Srcaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srcaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Block Transfer Source Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dstaddr(pub u32);
impl Dstaddr {
    #[doc="Transfer Destination Address"]
    #[inline] pub fn dstaddr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DSTADDR != 0"]
    #[inline] pub fn test_dstaddr(&self) -> bool {
        self.dstaddr() != 0
    }

    #[doc="Sets the DSTADDR field."]
    #[inline] pub fn set_dstaddr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dstaddr {
    #[inline]
    fn from(other: u32) -> Self {
         Dstaddr(other)
    }
}

impl ::core::fmt::Display for Dstaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dstaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Block Transfer Destination Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Descaddr(pub u32);
impl Descaddr {
    #[doc="Next Descriptor Address"]
    #[inline] pub fn descaddr(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DESCADDR != 0"]
    #[inline] pub fn test_descaddr(&self) -> bool {
        self.descaddr() != 0
    }

    #[doc="Sets the DESCADDR field."]
    #[inline] pub fn set_descaddr<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Descaddr {
    #[inline]
    fn from(other: u32) -> Self {
         Descaddr(other)
    }
}

impl ::core::fmt::Display for Descaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Descaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

