::bobbin_mcu::periph!( DMAC, Dmac, DMAC_PERIPH, DmacPeriph, DMAC_OWNED, DMAC_REF_COUNT, 0x4100a000, 0x00, 0x07);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMAC Peripheral"]
pub struct DmacPeriph(pub usize); 

impl DmacPeriph {
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

    #[doc="Get the PRICTRL0 Register."]
    #[inline] pub fn prictrl0_reg(&self) -> ::bobbin_mcu::register::Register<Prictrl0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Prictrl0, 0x14)
    }

    #[doc="Get the *mut pointer for the PRICTRL0 register."]
    #[inline] pub fn prictrl0_mut(&self) -> *mut Prictrl0 { 
        self.prictrl0_reg().ptr()
    }

    #[doc="Get the *const pointer for the PRICTRL0 register."]
    #[inline] pub fn prictrl0_ptr(&self) -> *const Prictrl0 { 
        self.prictrl0_reg().ptr()
    }

    #[doc="Read the PRICTRL0 register."]
    #[inline] pub fn prictrl0(&self) -> Prictrl0 { 
        self.prictrl0_reg().read()
    }

    #[doc="Write the PRICTRL0 register."]
    #[inline] pub fn write_prictrl0(&self, value: Prictrl0) -> &Self { 
        self.prictrl0_reg().write(value);
        self
    }

    #[doc="Set the PRICTRL0 register."]
    #[inline] pub fn set_prictrl0<F: FnOnce(Prictrl0) -> Prictrl0>(&self, f: F) -> &Self {
        self.prictrl0_reg().set(f);
        self
    }

    #[doc="Modify the PRICTRL0 register."]
    #[inline] pub fn with_prictrl0<F: FnOnce(Prictrl0) -> Prictrl0>(&self, f: F) -> &Self {
        self.prictrl0_reg().with(f);
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

    #[doc="Get the CHCTRLA Register."]
    #[inline] pub fn chctrla_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chctrla, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chctrla, 0x40, 0x10)
    }

    #[doc="Get the *mut pointer for the CHCTRLA register."]
    #[inline] pub fn chctrla_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chctrla { 
        self.chctrla_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHCTRLA register."]
    #[inline] pub fn chctrla_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chctrla { 
        self.chctrla_reg().ptr(index.into())
    }

    #[doc="Read the CHCTRLA register."]
    #[inline] pub fn chctrla<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chctrla { 
        self.chctrla_reg().read(index.into())
    }

    #[doc="Write the CHCTRLA register."]
    #[inline] pub fn write_chctrla<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chctrla) -> &Self {
        self.chctrla_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHCTRLA register."]
    #[inline] pub fn set_chctrla<I: Into<::bobbin_bits::R32>, F: FnOnce(Chctrla) -> Chctrla>(&self, index: I, f: F) -> &Self {
        self.chctrla_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHCTRLA register."]
    #[inline] pub fn with_chctrla<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chctrla) -> Chctrla>(&self, index: I, f: F) -> &Self {
        self.chctrla_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHCTRLB Register."]
    #[inline] pub fn chctrlb_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chctrlb, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chctrlb, 0x44, 0x10)
    }

    #[doc="Get the *mut pointer for the CHCTRLB register."]
    #[inline] pub fn chctrlb_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chctrlb { 
        self.chctrlb_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHCTRLB register."]
    #[inline] pub fn chctrlb_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chctrlb { 
        self.chctrlb_reg().ptr(index.into())
    }

    #[doc="Read the CHCTRLB register."]
    #[inline] pub fn chctrlb<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chctrlb { 
        self.chctrlb_reg().read(index.into())
    }

    #[doc="Write the CHCTRLB register."]
    #[inline] pub fn write_chctrlb<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chctrlb) -> &Self {
        self.chctrlb_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHCTRLB register."]
    #[inline] pub fn set_chctrlb<I: Into<::bobbin_bits::R32>, F: FnOnce(Chctrlb) -> Chctrlb>(&self, index: I, f: F) -> &Self {
        self.chctrlb_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHCTRLB register."]
    #[inline] pub fn with_chctrlb<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chctrlb) -> Chctrlb>(&self, index: I, f: F) -> &Self {
        self.chctrlb_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHPRILVL Register."]
    #[inline] pub fn chprilvl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chprilvl, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chprilvl, 0x45, 0x10)
    }

    #[doc="Get the *mut pointer for the CHPRILVL register."]
    #[inline] pub fn chprilvl_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chprilvl { 
        self.chprilvl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHPRILVL register."]
    #[inline] pub fn chprilvl_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chprilvl { 
        self.chprilvl_reg().ptr(index.into())
    }

    #[doc="Read the CHPRILVL register."]
    #[inline] pub fn chprilvl<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chprilvl { 
        self.chprilvl_reg().read(index.into())
    }

    #[doc="Write the CHPRILVL register."]
    #[inline] pub fn write_chprilvl<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chprilvl) -> &Self {
        self.chprilvl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHPRILVL register."]
    #[inline] pub fn set_chprilvl<I: Into<::bobbin_bits::R32>, F: FnOnce(Chprilvl) -> Chprilvl>(&self, index: I, f: F) -> &Self {
        self.chprilvl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHPRILVL register."]
    #[inline] pub fn with_chprilvl<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chprilvl) -> Chprilvl>(&self, index: I, f: F) -> &Self {
        self.chprilvl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHEVCTRL Register."]
    #[inline] pub fn chevctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chevctrl, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chevctrl, 0x46, 0x10)
    }

    #[doc="Get the *mut pointer for the CHEVCTRL register."]
    #[inline] pub fn chevctrl_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chevctrl { 
        self.chevctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHEVCTRL register."]
    #[inline] pub fn chevctrl_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chevctrl { 
        self.chevctrl_reg().ptr(index.into())
    }

    #[doc="Read the CHEVCTRL register."]
    #[inline] pub fn chevctrl<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chevctrl { 
        self.chevctrl_reg().read(index.into())
    }

    #[doc="Write the CHEVCTRL register."]
    #[inline] pub fn write_chevctrl<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chevctrl) -> &Self {
        self.chevctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHEVCTRL register."]
    #[inline] pub fn set_chevctrl<I: Into<::bobbin_bits::R32>, F: FnOnce(Chevctrl) -> Chevctrl>(&self, index: I, f: F) -> &Self {
        self.chevctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHEVCTRL register."]
    #[inline] pub fn with_chevctrl<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chevctrl) -> Chevctrl>(&self, index: I, f: F) -> &Self {
        self.chevctrl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHINTENCLR Register."]
    #[inline] pub fn chintenclr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chintenclr, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chintenclr, 0x4c, 0x10)
    }

    #[doc="Get the *mut pointer for the CHINTENCLR register."]
    #[inline] pub fn chintenclr_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chintenclr { 
        self.chintenclr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHINTENCLR register."]
    #[inline] pub fn chintenclr_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chintenclr { 
        self.chintenclr_reg().ptr(index.into())
    }

    #[doc="Read the CHINTENCLR register."]
    #[inline] pub fn chintenclr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chintenclr { 
        self.chintenclr_reg().read(index.into())
    }

    #[doc="Write the CHINTENCLR register."]
    #[inline] pub fn write_chintenclr<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chintenclr) -> &Self {
        self.chintenclr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHINTENCLR register."]
    #[inline] pub fn set_chintenclr<I: Into<::bobbin_bits::R32>, F: FnOnce(Chintenclr) -> Chintenclr>(&self, index: I, f: F) -> &Self {
        self.chintenclr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHINTENCLR register."]
    #[inline] pub fn with_chintenclr<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chintenclr) -> Chintenclr>(&self, index: I, f: F) -> &Self {
        self.chintenclr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHINTENSET Register."]
    #[inline] pub fn chintenset_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chintenset, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chintenset, 0x4d, 0x10)
    }

    #[doc="Get the *mut pointer for the CHINTENSET register."]
    #[inline] pub fn chintenset_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chintenset { 
        self.chintenset_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHINTENSET register."]
    #[inline] pub fn chintenset_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chintenset { 
        self.chintenset_reg().ptr(index.into())
    }

    #[doc="Read the CHINTENSET register."]
    #[inline] pub fn chintenset<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chintenset { 
        self.chintenset_reg().read(index.into())
    }

    #[doc="Write the CHINTENSET register."]
    #[inline] pub fn write_chintenset<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chintenset) -> &Self {
        self.chintenset_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHINTENSET register."]
    #[inline] pub fn set_chintenset<I: Into<::bobbin_bits::R32>, F: FnOnce(Chintenset) -> Chintenset>(&self, index: I, f: F) -> &Self {
        self.chintenset_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHINTENSET register."]
    #[inline] pub fn with_chintenset<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chintenset) -> Chintenset>(&self, index: I, f: F) -> &Self {
        self.chintenset_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHINTFLAG Register."]
    #[inline] pub fn chintflag_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chintflag, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chintflag, 0x4e, 0x10)
    }

    #[doc="Get the *mut pointer for the CHINTFLAG register."]
    #[inline] pub fn chintflag_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chintflag { 
        self.chintflag_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHINTFLAG register."]
    #[inline] pub fn chintflag_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chintflag { 
        self.chintflag_reg().ptr(index.into())
    }

    #[doc="Read the CHINTFLAG register."]
    #[inline] pub fn chintflag<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chintflag { 
        self.chintflag_reg().read(index.into())
    }

    #[doc="Write the CHINTFLAG register."]
    #[inline] pub fn write_chintflag<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chintflag) -> &Self {
        self.chintflag_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHINTFLAG register."]
    #[inline] pub fn set_chintflag<I: Into<::bobbin_bits::R32>, F: FnOnce(Chintflag) -> Chintflag>(&self, index: I, f: F) -> &Self {
        self.chintflag_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHINTFLAG register."]
    #[inline] pub fn with_chintflag<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chintflag) -> Chintflag>(&self, index: I, f: F) -> &Self {
        self.chintflag_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CHSTATUS Register."]
    #[inline] pub fn chstatus_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chstatus, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chstatus, 0x4f, 0x10)
    }

    #[doc="Get the *mut pointer for the CHSTATUS register."]
    #[inline] pub fn chstatus_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Chstatus { 
        self.chstatus_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHSTATUS register."]
    #[inline] pub fn chstatus_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Chstatus { 
        self.chstatus_reg().ptr(index.into())
    }

    #[doc="Read the CHSTATUS register."]
    #[inline] pub fn chstatus<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Chstatus { 
        self.chstatus_reg().read(index.into())
    }

    #[doc="Write the CHSTATUS register."]
    #[inline] pub fn write_chstatus<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Chstatus) -> &Self {
        self.chstatus_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHSTATUS register."]
    #[inline] pub fn set_chstatus<I: Into<::bobbin_bits::R32>, F: FnOnce(Chstatus) -> Chstatus>(&self, index: I, f: F) -> &Self {
        self.chstatus_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHSTATUS register."]
    #[inline] pub fn with_chstatus<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Chstatus) -> Chstatus>(&self, index: I, f: F) -> &Self {
        self.chstatus_reg().with(index.into(), f);
        self
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

    #[doc="Priority Level 0 Enable"]
    #[inline] pub fn lvlen0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LVLEN0 != 0"]
    #[inline] pub fn test_lvlen0(&self) -> bool {
        self.lvlen0() != 0
    }

    #[doc="Sets the LVLEN0 field."]
    #[inline] pub fn set_lvlen0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Priority Level 1 Enable"]
    #[inline] pub fn lvlen1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LVLEN1 != 0"]
    #[inline] pub fn test_lvlen1(&self) -> bool {
        self.lvlen1() != 0
    }

    #[doc="Sets the LVLEN1 field."]
    #[inline] pub fn set_lvlen1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Priority Level 2 Enable"]
    #[inline] pub fn lvlen2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if LVLEN2 != 0"]
    #[inline] pub fn test_lvlen2(&self) -> bool {
        self.lvlen2() != 0
    }

    #[doc="Sets the LVLEN2 field."]
    #[inline] pub fn set_lvlen2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Priority Level 3 Enable"]
    #[inline] pub fn lvlen3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if LVLEN3 != 0"]
    #[inline] pub fn test_lvlen3(&self) -> bool {
        self.lvlen3() != 0
    }

    #[doc="Sets the LVLEN3 field."]
    #[inline] pub fn set_lvlen3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
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
        if self.lvlen0() != 0 { try!(write!(f, " lvlen0"))}
        if self.lvlen1() != 0 { try!(write!(f, " lvlen1"))}
        if self.lvlen2() != 0 { try!(write!(f, " lvlen2"))}
        if self.lvlen3() != 0 { try!(write!(f, " lvlen3"))}
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

    #[doc="CRC Operating Mode"]
    #[inline] pub fn crcmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if CRCMODE != 0"]
    #[inline] pub fn test_crcmode(&self) -> bool {
        self.crcmode() != 0
    }

    #[doc="Sets the CRCMODE field."]
    #[inline] pub fn set_crcmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
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
        if self.crcmode() != 0 { try!(write!(f, " crcmode=0x{:x}", self.crcmode()))}
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

    #[doc="CRC Error"]
    #[inline] pub fn crcerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CRCERR != 0"]
    #[inline] pub fn test_crcerr(&self) -> bool {
        self.crcerr() != 0
    }

    #[doc="Sets the CRCERR field."]
    #[inline] pub fn set_crcerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
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
        if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
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

#[doc="Software Trigger Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swtrigctrl(pub u32);
impl Swtrigctrl {
    #[doc="Channel 0 Software Trigger"]
    #[inline] pub fn swtrig0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWTRIG0 != 0"]
    #[inline] pub fn test_swtrig0(&self) -> bool {
        self.swtrig0() != 0
    }

    #[doc="Sets the SWTRIG0 field."]
    #[inline] pub fn set_swtrig0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Software Trigger"]
    #[inline] pub fn swtrig1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SWTRIG1 != 0"]
    #[inline] pub fn test_swtrig1(&self) -> bool {
        self.swtrig1() != 0
    }

    #[doc="Sets the SWTRIG1 field."]
    #[inline] pub fn set_swtrig1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 2 Software Trigger"]
    #[inline] pub fn swtrig2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SWTRIG2 != 0"]
    #[inline] pub fn test_swtrig2(&self) -> bool {
        self.swtrig2() != 0
    }

    #[doc="Sets the SWTRIG2 field."]
    #[inline] pub fn set_swtrig2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 3 Software Trigger"]
    #[inline] pub fn swtrig3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SWTRIG3 != 0"]
    #[inline] pub fn test_swtrig3(&self) -> bool {
        self.swtrig3() != 0
    }

    #[doc="Sets the SWTRIG3 field."]
    #[inline] pub fn set_swtrig3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 4 Software Trigger"]
    #[inline] pub fn swtrig4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SWTRIG4 != 0"]
    #[inline] pub fn test_swtrig4(&self) -> bool {
        self.swtrig4() != 0
    }

    #[doc="Sets the SWTRIG4 field."]
    #[inline] pub fn set_swtrig4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 5 Software Trigger"]
    #[inline] pub fn swtrig5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SWTRIG5 != 0"]
    #[inline] pub fn test_swtrig5(&self) -> bool {
        self.swtrig5() != 0
    }

    #[doc="Sets the SWTRIG5 field."]
    #[inline] pub fn set_swtrig5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel 6 Software Trigger"]
    #[inline] pub fn swtrig6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SWTRIG6 != 0"]
    #[inline] pub fn test_swtrig6(&self) -> bool {
        self.swtrig6() != 0
    }

    #[doc="Sets the SWTRIG6 field."]
    #[inline] pub fn set_swtrig6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel 7 Software Trigger"]
    #[inline] pub fn swtrig7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SWTRIG7 != 0"]
    #[inline] pub fn test_swtrig7(&self) -> bool {
        self.swtrig7() != 0
    }

    #[doc="Sets the SWTRIG7 field."]
    #[inline] pub fn set_swtrig7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Channel 8 Software Trigger"]
    #[inline] pub fn swtrig8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SWTRIG8 != 0"]
    #[inline] pub fn test_swtrig8(&self) -> bool {
        self.swtrig8() != 0
    }

    #[doc="Sets the SWTRIG8 field."]
    #[inline] pub fn set_swtrig8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Channel 9 Software Trigger"]
    #[inline] pub fn swtrig9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SWTRIG9 != 0"]
    #[inline] pub fn test_swtrig9(&self) -> bool {
        self.swtrig9() != 0
    }

    #[doc="Sets the SWTRIG9 field."]
    #[inline] pub fn set_swtrig9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Channel 10 Software Trigger"]
    #[inline] pub fn swtrig10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SWTRIG10 != 0"]
    #[inline] pub fn test_swtrig10(&self) -> bool {
        self.swtrig10() != 0
    }

    #[doc="Sets the SWTRIG10 field."]
    #[inline] pub fn set_swtrig10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Channel 11 Software Trigger"]
    #[inline] pub fn swtrig11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SWTRIG11 != 0"]
    #[inline] pub fn test_swtrig11(&self) -> bool {
        self.swtrig11() != 0
    }

    #[doc="Sets the SWTRIG11 field."]
    #[inline] pub fn set_swtrig11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Channel 12 Software Trigger"]
    #[inline] pub fn swtrig12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SWTRIG12 != 0"]
    #[inline] pub fn test_swtrig12(&self) -> bool {
        self.swtrig12() != 0
    }

    #[doc="Sets the SWTRIG12 field."]
    #[inline] pub fn set_swtrig12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Channel 13 Software Trigger"]
    #[inline] pub fn swtrig13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SWTRIG13 != 0"]
    #[inline] pub fn test_swtrig13(&self) -> bool {
        self.swtrig13() != 0
    }

    #[doc="Sets the SWTRIG13 field."]
    #[inline] pub fn set_swtrig13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Channel 14 Software Trigger"]
    #[inline] pub fn swtrig14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SWTRIG14 != 0"]
    #[inline] pub fn test_swtrig14(&self) -> bool {
        self.swtrig14() != 0
    }

    #[doc="Sets the SWTRIG14 field."]
    #[inline] pub fn set_swtrig14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Channel 15 Software Trigger"]
    #[inline] pub fn swtrig15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SWTRIG15 != 0"]
    #[inline] pub fn test_swtrig15(&self) -> bool {
        self.swtrig15() != 0
    }

    #[doc="Sets the SWTRIG15 field."]
    #[inline] pub fn set_swtrig15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Channel 16 Software Trigger"]
    #[inline] pub fn swtrig16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SWTRIG16 != 0"]
    #[inline] pub fn test_swtrig16(&self) -> bool {
        self.swtrig16() != 0
    }

    #[doc="Sets the SWTRIG16 field."]
    #[inline] pub fn set_swtrig16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Channel 17 Software Trigger"]
    #[inline] pub fn swtrig17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SWTRIG17 != 0"]
    #[inline] pub fn test_swtrig17(&self) -> bool {
        self.swtrig17() != 0
    }

    #[doc="Sets the SWTRIG17 field."]
    #[inline] pub fn set_swtrig17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Channel 18 Software Trigger"]
    #[inline] pub fn swtrig18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if SWTRIG18 != 0"]
    #[inline] pub fn test_swtrig18(&self) -> bool {
        self.swtrig18() != 0
    }

    #[doc="Sets the SWTRIG18 field."]
    #[inline] pub fn set_swtrig18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Channel 19 Software Trigger"]
    #[inline] pub fn swtrig19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if SWTRIG19 != 0"]
    #[inline] pub fn test_swtrig19(&self) -> bool {
        self.swtrig19() != 0
    }

    #[doc="Sets the SWTRIG19 field."]
    #[inline] pub fn set_swtrig19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Channel 20 Software Trigger"]
    #[inline] pub fn swtrig20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SWTRIG20 != 0"]
    #[inline] pub fn test_swtrig20(&self) -> bool {
        self.swtrig20() != 0
    }

    #[doc="Sets the SWTRIG20 field."]
    #[inline] pub fn set_swtrig20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Channel 21 Software Trigger"]
    #[inline] pub fn swtrig21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SWTRIG21 != 0"]
    #[inline] pub fn test_swtrig21(&self) -> bool {
        self.swtrig21() != 0
    }

    #[doc="Sets the SWTRIG21 field."]
    #[inline] pub fn set_swtrig21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Channel 22 Software Trigger"]
    #[inline] pub fn swtrig22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SWTRIG22 != 0"]
    #[inline] pub fn test_swtrig22(&self) -> bool {
        self.swtrig22() != 0
    }

    #[doc="Sets the SWTRIG22 field."]
    #[inline] pub fn set_swtrig22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Channel 23 Software Trigger"]
    #[inline] pub fn swtrig23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if SWTRIG23 != 0"]
    #[inline] pub fn test_swtrig23(&self) -> bool {
        self.swtrig23() != 0
    }

    #[doc="Sets the SWTRIG23 field."]
    #[inline] pub fn set_swtrig23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Channel 24 Software Trigger"]
    #[inline] pub fn swtrig24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SWTRIG24 != 0"]
    #[inline] pub fn test_swtrig24(&self) -> bool {
        self.swtrig24() != 0
    }

    #[doc="Sets the SWTRIG24 field."]
    #[inline] pub fn set_swtrig24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Channel 25 Software Trigger"]
    #[inline] pub fn swtrig25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if SWTRIG25 != 0"]
    #[inline] pub fn test_swtrig25(&self) -> bool {
        self.swtrig25() != 0
    }

    #[doc="Sets the SWTRIG25 field."]
    #[inline] pub fn set_swtrig25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Channel 26 Software Trigger"]
    #[inline] pub fn swtrig26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if SWTRIG26 != 0"]
    #[inline] pub fn test_swtrig26(&self) -> bool {
        self.swtrig26() != 0
    }

    #[doc="Sets the SWTRIG26 field."]
    #[inline] pub fn set_swtrig26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Channel 27 Software Trigger"]
    #[inline] pub fn swtrig27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SWTRIG27 != 0"]
    #[inline] pub fn test_swtrig27(&self) -> bool {
        self.swtrig27() != 0
    }

    #[doc="Sets the SWTRIG27 field."]
    #[inline] pub fn set_swtrig27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Channel 28 Software Trigger"]
    #[inline] pub fn swtrig28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SWTRIG28 != 0"]
    #[inline] pub fn test_swtrig28(&self) -> bool {
        self.swtrig28() != 0
    }

    #[doc="Sets the SWTRIG28 field."]
    #[inline] pub fn set_swtrig28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Channel 29 Software Trigger"]
    #[inline] pub fn swtrig29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if SWTRIG29 != 0"]
    #[inline] pub fn test_swtrig29(&self) -> bool {
        self.swtrig29() != 0
    }

    #[doc="Sets the SWTRIG29 field."]
    #[inline] pub fn set_swtrig29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel 30 Software Trigger"]
    #[inline] pub fn swtrig30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if SWTRIG30 != 0"]
    #[inline] pub fn test_swtrig30(&self) -> bool {
        self.swtrig30() != 0
    }

    #[doc="Sets the SWTRIG30 field."]
    #[inline] pub fn set_swtrig30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel 31 Software Trigger"]
    #[inline] pub fn swtrig31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if SWTRIG31 != 0"]
    #[inline] pub fn test_swtrig31(&self) -> bool {
        self.swtrig31() != 0
    }

    #[doc="Sets the SWTRIG31 field."]
    #[inline] pub fn set_swtrig31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.swtrig0() != 0 { try!(write!(f, " swtrig0"))}
        if self.swtrig1() != 0 { try!(write!(f, " swtrig1"))}
        if self.swtrig2() != 0 { try!(write!(f, " swtrig2"))}
        if self.swtrig3() != 0 { try!(write!(f, " swtrig3"))}
        if self.swtrig4() != 0 { try!(write!(f, " swtrig4"))}
        if self.swtrig5() != 0 { try!(write!(f, " swtrig5"))}
        if self.swtrig6() != 0 { try!(write!(f, " swtrig6"))}
        if self.swtrig7() != 0 { try!(write!(f, " swtrig7"))}
        if self.swtrig8() != 0 { try!(write!(f, " swtrig8"))}
        if self.swtrig9() != 0 { try!(write!(f, " swtrig9"))}
        if self.swtrig10() != 0 { try!(write!(f, " swtrig10"))}
        if self.swtrig11() != 0 { try!(write!(f, " swtrig11"))}
        if self.swtrig12() != 0 { try!(write!(f, " swtrig12"))}
        if self.swtrig13() != 0 { try!(write!(f, " swtrig13"))}
        if self.swtrig14() != 0 { try!(write!(f, " swtrig14"))}
        if self.swtrig15() != 0 { try!(write!(f, " swtrig15"))}
        if self.swtrig16() != 0 { try!(write!(f, " swtrig16"))}
        if self.swtrig17() != 0 { try!(write!(f, " swtrig17"))}
        if self.swtrig18() != 0 { try!(write!(f, " swtrig18"))}
        if self.swtrig19() != 0 { try!(write!(f, " swtrig19"))}
        if self.swtrig20() != 0 { try!(write!(f, " swtrig20"))}
        if self.swtrig21() != 0 { try!(write!(f, " swtrig21"))}
        if self.swtrig22() != 0 { try!(write!(f, " swtrig22"))}
        if self.swtrig23() != 0 { try!(write!(f, " swtrig23"))}
        if self.swtrig24() != 0 { try!(write!(f, " swtrig24"))}
        if self.swtrig25() != 0 { try!(write!(f, " swtrig25"))}
        if self.swtrig26() != 0 { try!(write!(f, " swtrig26"))}
        if self.swtrig27() != 0 { try!(write!(f, " swtrig27"))}
        if self.swtrig28() != 0 { try!(write!(f, " swtrig28"))}
        if self.swtrig29() != 0 { try!(write!(f, " swtrig29"))}
        if self.swtrig30() != 0 { try!(write!(f, " swtrig30"))}
        if self.swtrig31() != 0 { try!(write!(f, " swtrig31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Priority Control 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Prictrl0(pub u32);
impl Prictrl0 {
    #[doc="Level 0 Channel Priority Number"]
    #[inline] pub fn lvlpri0(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if LVLPRI0 != 0"]
    #[inline] pub fn test_lvlpri0(&self) -> bool {
        self.lvlpri0() != 0
    }

    #[doc="Sets the LVLPRI0 field."]
    #[inline] pub fn set_lvlpri0<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Level 0 Quality of Service"]
    #[inline] pub fn qos0(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if QOS0 != 0"]
    #[inline] pub fn test_qos0(&self) -> bool {
        self.qos0() != 0
    }

    #[doc="Sets the QOS0 field."]
    #[inline] pub fn set_qos0<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Level 0 Round-Robin Scheduling Enable"]
    #[inline] pub fn rrlvlen0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RRLVLEN0 != 0"]
    #[inline] pub fn test_rrlvlen0(&self) -> bool {
        self.rrlvlen0() != 0
    }

    #[doc="Sets the RRLVLEN0 field."]
    #[inline] pub fn set_rrlvlen0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Level 1 Channel Priority Number"]
    #[inline] pub fn lvlpri1(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if LVLPRI1 != 0"]
    #[inline] pub fn test_lvlpri1(&self) -> bool {
        self.lvlpri1() != 0
    }

    #[doc="Sets the LVLPRI1 field."]
    #[inline] pub fn set_lvlpri1<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Level 1 Quality of Service"]
    #[inline] pub fn qos1(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if QOS1 != 0"]
    #[inline] pub fn test_qos1(&self) -> bool {
        self.qos1() != 0
    }

    #[doc="Sets the QOS1 field."]
    #[inline] pub fn set_qos1<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Level 1 Round-Robin Scheduling Enable"]
    #[inline] pub fn rrlvlen1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RRLVLEN1 != 0"]
    #[inline] pub fn test_rrlvlen1(&self) -> bool {
        self.rrlvlen1() != 0
    }

    #[doc="Sets the RRLVLEN1 field."]
    #[inline] pub fn set_rrlvlen1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Level 2 Channel Priority Number"]
    #[inline] pub fn lvlpri2(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if LVLPRI2 != 0"]
    #[inline] pub fn test_lvlpri2(&self) -> bool {
        self.lvlpri2() != 0
    }

    #[doc="Sets the LVLPRI2 field."]
    #[inline] pub fn set_lvlpri2<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Level 2 Quality of Service"]
    #[inline] pub fn qos2(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if QOS2 != 0"]
    #[inline] pub fn test_qos2(&self) -> bool {
        self.qos2() != 0
    }

    #[doc="Sets the QOS2 field."]
    #[inline] pub fn set_qos2<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Level 2 Round-Robin Scheduling Enable"]
    #[inline] pub fn rrlvlen2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if RRLVLEN2 != 0"]
    #[inline] pub fn test_rrlvlen2(&self) -> bool {
        self.rrlvlen2() != 0
    }

    #[doc="Sets the RRLVLEN2 field."]
    #[inline] pub fn set_rrlvlen2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Level 3 Channel Priority Number"]
    #[inline] pub fn lvlpri3(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if LVLPRI3 != 0"]
    #[inline] pub fn test_lvlpri3(&self) -> bool {
        self.lvlpri3() != 0
    }

    #[doc="Sets the LVLPRI3 field."]
    #[inline] pub fn set_lvlpri3<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Level 3 Quality of Service"]
    #[inline] pub fn qos3(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if QOS3 != 0"]
    #[inline] pub fn test_qos3(&self) -> bool {
        self.qos3() != 0
    }

    #[doc="Sets the QOS3 field."]
    #[inline] pub fn set_qos3<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Level 3 Round-Robin Scheduling Enable"]
    #[inline] pub fn rrlvlen3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if RRLVLEN3 != 0"]
    #[inline] pub fn test_rrlvlen3(&self) -> bool {
        self.rrlvlen3() != 0
    }

    #[doc="Sets the RRLVLEN3 field."]
    #[inline] pub fn set_rrlvlen3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Prictrl0 {
    #[inline]
    fn from(other: u32) -> Self {
         Prictrl0(other)
    }
}

impl ::core::fmt::Display for Prictrl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Prictrl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lvlpri0() != 0 { try!(write!(f, " lvlpri0=0x{:x}", self.lvlpri0()))}
        if self.qos0() != 0 { try!(write!(f, " qos0=0x{:x}", self.qos0()))}
        if self.rrlvlen0() != 0 { try!(write!(f, " rrlvlen0"))}
        if self.lvlpri1() != 0 { try!(write!(f, " lvlpri1=0x{:x}", self.lvlpri1()))}
        if self.qos1() != 0 { try!(write!(f, " qos1=0x{:x}", self.qos1()))}
        if self.rrlvlen1() != 0 { try!(write!(f, " rrlvlen1"))}
        if self.lvlpri2() != 0 { try!(write!(f, " lvlpri2=0x{:x}", self.lvlpri2()))}
        if self.qos2() != 0 { try!(write!(f, " qos2=0x{:x}", self.qos2()))}
        if self.rrlvlen2() != 0 { try!(write!(f, " rrlvlen2"))}
        if self.lvlpri3() != 0 { try!(write!(f, " lvlpri3=0x{:x}", self.lvlpri3()))}
        if self.qos3() != 0 { try!(write!(f, " qos3=0x{:x}", self.qos3()))}
        if self.rrlvlen3() != 0 { try!(write!(f, " rrlvlen3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Pending"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intpend(pub u16);
impl Intpend {
    #[doc="Channel ID"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 0);
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

    #[doc="CRC Error"]
    #[inline] pub fn crcerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCERR != 0"]
    #[inline] pub fn test_crcerr(&self) -> bool {
        self.crcerr() != 0
    }

    #[doc="Sets the CRCERR field."]
    #[inline] pub fn set_crcerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
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
        if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
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
    #[doc="Channel 0 Pending Interrupt"]
    #[inline] pub fn chint0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHINT0 != 0"]
    #[inline] pub fn test_chint0(&self) -> bool {
        self.chint0() != 0
    }

    #[doc="Sets the CHINT0 field."]
    #[inline] pub fn set_chint0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Pending Interrupt"]
    #[inline] pub fn chint1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CHINT1 != 0"]
    #[inline] pub fn test_chint1(&self) -> bool {
        self.chint1() != 0
    }

    #[doc="Sets the CHINT1 field."]
    #[inline] pub fn set_chint1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 2 Pending Interrupt"]
    #[inline] pub fn chint2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CHINT2 != 0"]
    #[inline] pub fn test_chint2(&self) -> bool {
        self.chint2() != 0
    }

    #[doc="Sets the CHINT2 field."]
    #[inline] pub fn set_chint2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 3 Pending Interrupt"]
    #[inline] pub fn chint3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CHINT3 != 0"]
    #[inline] pub fn test_chint3(&self) -> bool {
        self.chint3() != 0
    }

    #[doc="Sets the CHINT3 field."]
    #[inline] pub fn set_chint3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 4 Pending Interrupt"]
    #[inline] pub fn chint4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CHINT4 != 0"]
    #[inline] pub fn test_chint4(&self) -> bool {
        self.chint4() != 0
    }

    #[doc="Sets the CHINT4 field."]
    #[inline] pub fn set_chint4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 5 Pending Interrupt"]
    #[inline] pub fn chint5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CHINT5 != 0"]
    #[inline] pub fn test_chint5(&self) -> bool {
        self.chint5() != 0
    }

    #[doc="Sets the CHINT5 field."]
    #[inline] pub fn set_chint5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel 6 Pending Interrupt"]
    #[inline] pub fn chint6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CHINT6 != 0"]
    #[inline] pub fn test_chint6(&self) -> bool {
        self.chint6() != 0
    }

    #[doc="Sets the CHINT6 field."]
    #[inline] pub fn set_chint6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel 7 Pending Interrupt"]
    #[inline] pub fn chint7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHINT7 != 0"]
    #[inline] pub fn test_chint7(&self) -> bool {
        self.chint7() != 0
    }

    #[doc="Sets the CHINT7 field."]
    #[inline] pub fn set_chint7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Channel 8 Pending Interrupt"]
    #[inline] pub fn chint8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHINT8 != 0"]
    #[inline] pub fn test_chint8(&self) -> bool {
        self.chint8() != 0
    }

    #[doc="Sets the CHINT8 field."]
    #[inline] pub fn set_chint8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Channel 9 Pending Interrupt"]
    #[inline] pub fn chint9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CHINT9 != 0"]
    #[inline] pub fn test_chint9(&self) -> bool {
        self.chint9() != 0
    }

    #[doc="Sets the CHINT9 field."]
    #[inline] pub fn set_chint9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Channel 10 Pending Interrupt"]
    #[inline] pub fn chint10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CHINT10 != 0"]
    #[inline] pub fn test_chint10(&self) -> bool {
        self.chint10() != 0
    }

    #[doc="Sets the CHINT10 field."]
    #[inline] pub fn set_chint10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Channel 11 Pending Interrupt"]
    #[inline] pub fn chint11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CHINT11 != 0"]
    #[inline] pub fn test_chint11(&self) -> bool {
        self.chint11() != 0
    }

    #[doc="Sets the CHINT11 field."]
    #[inline] pub fn set_chint11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Channel 12 Pending Interrupt"]
    #[inline] pub fn chint12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CHINT12 != 0"]
    #[inline] pub fn test_chint12(&self) -> bool {
        self.chint12() != 0
    }

    #[doc="Sets the CHINT12 field."]
    #[inline] pub fn set_chint12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Channel 13 Pending Interrupt"]
    #[inline] pub fn chint13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CHINT13 != 0"]
    #[inline] pub fn test_chint13(&self) -> bool {
        self.chint13() != 0
    }

    #[doc="Sets the CHINT13 field."]
    #[inline] pub fn set_chint13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Channel 14 Pending Interrupt"]
    #[inline] pub fn chint14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CHINT14 != 0"]
    #[inline] pub fn test_chint14(&self) -> bool {
        self.chint14() != 0
    }

    #[doc="Sets the CHINT14 field."]
    #[inline] pub fn set_chint14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Channel 15 Pending Interrupt"]
    #[inline] pub fn chint15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CHINT15 != 0"]
    #[inline] pub fn test_chint15(&self) -> bool {
        self.chint15() != 0
    }

    #[doc="Sets the CHINT15 field."]
    #[inline] pub fn set_chint15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Channel 16 Pending Interrupt"]
    #[inline] pub fn chint16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CHINT16 != 0"]
    #[inline] pub fn test_chint16(&self) -> bool {
        self.chint16() != 0
    }

    #[doc="Sets the CHINT16 field."]
    #[inline] pub fn set_chint16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Channel 17 Pending Interrupt"]
    #[inline] pub fn chint17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CHINT17 != 0"]
    #[inline] pub fn test_chint17(&self) -> bool {
        self.chint17() != 0
    }

    #[doc="Sets the CHINT17 field."]
    #[inline] pub fn set_chint17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Channel 18 Pending Interrupt"]
    #[inline] pub fn chint18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CHINT18 != 0"]
    #[inline] pub fn test_chint18(&self) -> bool {
        self.chint18() != 0
    }

    #[doc="Sets the CHINT18 field."]
    #[inline] pub fn set_chint18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Channel 19 Pending Interrupt"]
    #[inline] pub fn chint19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CHINT19 != 0"]
    #[inline] pub fn test_chint19(&self) -> bool {
        self.chint19() != 0
    }

    #[doc="Sets the CHINT19 field."]
    #[inline] pub fn set_chint19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Channel 20 Pending Interrupt"]
    #[inline] pub fn chint20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CHINT20 != 0"]
    #[inline] pub fn test_chint20(&self) -> bool {
        self.chint20() != 0
    }

    #[doc="Sets the CHINT20 field."]
    #[inline] pub fn set_chint20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Channel 21 Pending Interrupt"]
    #[inline] pub fn chint21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if CHINT21 != 0"]
    #[inline] pub fn test_chint21(&self) -> bool {
        self.chint21() != 0
    }

    #[doc="Sets the CHINT21 field."]
    #[inline] pub fn set_chint21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Channel 22 Pending Interrupt"]
    #[inline] pub fn chint22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if CHINT22 != 0"]
    #[inline] pub fn test_chint22(&self) -> bool {
        self.chint22() != 0
    }

    #[doc="Sets the CHINT22 field."]
    #[inline] pub fn set_chint22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Channel 23 Pending Interrupt"]
    #[inline] pub fn chint23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if CHINT23 != 0"]
    #[inline] pub fn test_chint23(&self) -> bool {
        self.chint23() != 0
    }

    #[doc="Sets the CHINT23 field."]
    #[inline] pub fn set_chint23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Channel 24 Pending Interrupt"]
    #[inline] pub fn chint24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if CHINT24 != 0"]
    #[inline] pub fn test_chint24(&self) -> bool {
        self.chint24() != 0
    }

    #[doc="Sets the CHINT24 field."]
    #[inline] pub fn set_chint24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Channel 25 Pending Interrupt"]
    #[inline] pub fn chint25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CHINT25 != 0"]
    #[inline] pub fn test_chint25(&self) -> bool {
        self.chint25() != 0
    }

    #[doc="Sets the CHINT25 field."]
    #[inline] pub fn set_chint25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Channel 26 Pending Interrupt"]
    #[inline] pub fn chint26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CHINT26 != 0"]
    #[inline] pub fn test_chint26(&self) -> bool {
        self.chint26() != 0
    }

    #[doc="Sets the CHINT26 field."]
    #[inline] pub fn set_chint26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Channel 27 Pending Interrupt"]
    #[inline] pub fn chint27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if CHINT27 != 0"]
    #[inline] pub fn test_chint27(&self) -> bool {
        self.chint27() != 0
    }

    #[doc="Sets the CHINT27 field."]
    #[inline] pub fn set_chint27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Channel 28 Pending Interrupt"]
    #[inline] pub fn chint28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if CHINT28 != 0"]
    #[inline] pub fn test_chint28(&self) -> bool {
        self.chint28() != 0
    }

    #[doc="Sets the CHINT28 field."]
    #[inline] pub fn set_chint28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Channel 29 Pending Interrupt"]
    #[inline] pub fn chint29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if CHINT29 != 0"]
    #[inline] pub fn test_chint29(&self) -> bool {
        self.chint29() != 0
    }

    #[doc="Sets the CHINT29 field."]
    #[inline] pub fn set_chint29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Channel 30 Pending Interrupt"]
    #[inline] pub fn chint30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CHINT30 != 0"]
    #[inline] pub fn test_chint30(&self) -> bool {
        self.chint30() != 0
    }

    #[doc="Sets the CHINT30 field."]
    #[inline] pub fn set_chint30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Channel 31 Pending Interrupt"]
    #[inline] pub fn chint31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if CHINT31 != 0"]
    #[inline] pub fn test_chint31(&self) -> bool {
        self.chint31() != 0
    }

    #[doc="Sets the CHINT31 field."]
    #[inline] pub fn set_chint31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.chint0() != 0 { try!(write!(f, " chint0"))}
        if self.chint1() != 0 { try!(write!(f, " chint1"))}
        if self.chint2() != 0 { try!(write!(f, " chint2"))}
        if self.chint3() != 0 { try!(write!(f, " chint3"))}
        if self.chint4() != 0 { try!(write!(f, " chint4"))}
        if self.chint5() != 0 { try!(write!(f, " chint5"))}
        if self.chint6() != 0 { try!(write!(f, " chint6"))}
        if self.chint7() != 0 { try!(write!(f, " chint7"))}
        if self.chint8() != 0 { try!(write!(f, " chint8"))}
        if self.chint9() != 0 { try!(write!(f, " chint9"))}
        if self.chint10() != 0 { try!(write!(f, " chint10"))}
        if self.chint11() != 0 { try!(write!(f, " chint11"))}
        if self.chint12() != 0 { try!(write!(f, " chint12"))}
        if self.chint13() != 0 { try!(write!(f, " chint13"))}
        if self.chint14() != 0 { try!(write!(f, " chint14"))}
        if self.chint15() != 0 { try!(write!(f, " chint15"))}
        if self.chint16() != 0 { try!(write!(f, " chint16"))}
        if self.chint17() != 0 { try!(write!(f, " chint17"))}
        if self.chint18() != 0 { try!(write!(f, " chint18"))}
        if self.chint19() != 0 { try!(write!(f, " chint19"))}
        if self.chint20() != 0 { try!(write!(f, " chint20"))}
        if self.chint21() != 0 { try!(write!(f, " chint21"))}
        if self.chint22() != 0 { try!(write!(f, " chint22"))}
        if self.chint23() != 0 { try!(write!(f, " chint23"))}
        if self.chint24() != 0 { try!(write!(f, " chint24"))}
        if self.chint25() != 0 { try!(write!(f, " chint25"))}
        if self.chint26() != 0 { try!(write!(f, " chint26"))}
        if self.chint27() != 0 { try!(write!(f, " chint27"))}
        if self.chint28() != 0 { try!(write!(f, " chint28"))}
        if self.chint29() != 0 { try!(write!(f, " chint29"))}
        if self.chint30() != 0 { try!(write!(f, " chint30"))}
        if self.chint31() != 0 { try!(write!(f, " chint31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Busy Channels"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Busych(pub u32);
impl Busych {
    #[doc="Busy Channel 0"]
    #[inline] pub fn busych0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BUSYCH0 != 0"]
    #[inline] pub fn test_busych0(&self) -> bool {
        self.busych0() != 0
    }

    #[doc="Sets the BUSYCH0 field."]
    #[inline] pub fn set_busych0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Busy Channel 1"]
    #[inline] pub fn busych1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BUSYCH1 != 0"]
    #[inline] pub fn test_busych1(&self) -> bool {
        self.busych1() != 0
    }

    #[doc="Sets the BUSYCH1 field."]
    #[inline] pub fn set_busych1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Busy Channel 2"]
    #[inline] pub fn busych2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BUSYCH2 != 0"]
    #[inline] pub fn test_busych2(&self) -> bool {
        self.busych2() != 0
    }

    #[doc="Sets the BUSYCH2 field."]
    #[inline] pub fn set_busych2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Busy Channel 3"]
    #[inline] pub fn busych3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BUSYCH3 != 0"]
    #[inline] pub fn test_busych3(&self) -> bool {
        self.busych3() != 0
    }

    #[doc="Sets the BUSYCH3 field."]
    #[inline] pub fn set_busych3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Busy Channel 4"]
    #[inline] pub fn busych4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BUSYCH4 != 0"]
    #[inline] pub fn test_busych4(&self) -> bool {
        self.busych4() != 0
    }

    #[doc="Sets the BUSYCH4 field."]
    #[inline] pub fn set_busych4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Busy Channel 5"]
    #[inline] pub fn busych5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BUSYCH5 != 0"]
    #[inline] pub fn test_busych5(&self) -> bool {
        self.busych5() != 0
    }

    #[doc="Sets the BUSYCH5 field."]
    #[inline] pub fn set_busych5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Busy Channel 6"]
    #[inline] pub fn busych6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BUSYCH6 != 0"]
    #[inline] pub fn test_busych6(&self) -> bool {
        self.busych6() != 0
    }

    #[doc="Sets the BUSYCH6 field."]
    #[inline] pub fn set_busych6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Busy Channel 7"]
    #[inline] pub fn busych7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BUSYCH7 != 0"]
    #[inline] pub fn test_busych7(&self) -> bool {
        self.busych7() != 0
    }

    #[doc="Sets the BUSYCH7 field."]
    #[inline] pub fn set_busych7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Busy Channel 8"]
    #[inline] pub fn busych8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BUSYCH8 != 0"]
    #[inline] pub fn test_busych8(&self) -> bool {
        self.busych8() != 0
    }

    #[doc="Sets the BUSYCH8 field."]
    #[inline] pub fn set_busych8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Busy Channel 9"]
    #[inline] pub fn busych9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BUSYCH9 != 0"]
    #[inline] pub fn test_busych9(&self) -> bool {
        self.busych9() != 0
    }

    #[doc="Sets the BUSYCH9 field."]
    #[inline] pub fn set_busych9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Busy Channel 10"]
    #[inline] pub fn busych10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BUSYCH10 != 0"]
    #[inline] pub fn test_busych10(&self) -> bool {
        self.busych10() != 0
    }

    #[doc="Sets the BUSYCH10 field."]
    #[inline] pub fn set_busych10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Busy Channel 11"]
    #[inline] pub fn busych11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if BUSYCH11 != 0"]
    #[inline] pub fn test_busych11(&self) -> bool {
        self.busych11() != 0
    }

    #[doc="Sets the BUSYCH11 field."]
    #[inline] pub fn set_busych11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Busy Channel 12"]
    #[inline] pub fn busych12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if BUSYCH12 != 0"]
    #[inline] pub fn test_busych12(&self) -> bool {
        self.busych12() != 0
    }

    #[doc="Sets the BUSYCH12 field."]
    #[inline] pub fn set_busych12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Busy Channel 13"]
    #[inline] pub fn busych13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if BUSYCH13 != 0"]
    #[inline] pub fn test_busych13(&self) -> bool {
        self.busych13() != 0
    }

    #[doc="Sets the BUSYCH13 field."]
    #[inline] pub fn set_busych13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Busy Channel 14"]
    #[inline] pub fn busych14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if BUSYCH14 != 0"]
    #[inline] pub fn test_busych14(&self) -> bool {
        self.busych14() != 0
    }

    #[doc="Sets the BUSYCH14 field."]
    #[inline] pub fn set_busych14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Busy Channel 15"]
    #[inline] pub fn busych15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BUSYCH15 != 0"]
    #[inline] pub fn test_busych15(&self) -> bool {
        self.busych15() != 0
    }

    #[doc="Sets the BUSYCH15 field."]
    #[inline] pub fn set_busych15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Busy Channel 16"]
    #[inline] pub fn busych16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BUSYCH16 != 0"]
    #[inline] pub fn test_busych16(&self) -> bool {
        self.busych16() != 0
    }

    #[doc="Sets the BUSYCH16 field."]
    #[inline] pub fn set_busych16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Busy Channel 17"]
    #[inline] pub fn busych17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if BUSYCH17 != 0"]
    #[inline] pub fn test_busych17(&self) -> bool {
        self.busych17() != 0
    }

    #[doc="Sets the BUSYCH17 field."]
    #[inline] pub fn set_busych17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Busy Channel 18"]
    #[inline] pub fn busych18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if BUSYCH18 != 0"]
    #[inline] pub fn test_busych18(&self) -> bool {
        self.busych18() != 0
    }

    #[doc="Sets the BUSYCH18 field."]
    #[inline] pub fn set_busych18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Busy Channel 19"]
    #[inline] pub fn busych19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if BUSYCH19 != 0"]
    #[inline] pub fn test_busych19(&self) -> bool {
        self.busych19() != 0
    }

    #[doc="Sets the BUSYCH19 field."]
    #[inline] pub fn set_busych19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Busy Channel 20"]
    #[inline] pub fn busych20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if BUSYCH20 != 0"]
    #[inline] pub fn test_busych20(&self) -> bool {
        self.busych20() != 0
    }

    #[doc="Sets the BUSYCH20 field."]
    #[inline] pub fn set_busych20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Busy Channel 21"]
    #[inline] pub fn busych21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if BUSYCH21 != 0"]
    #[inline] pub fn test_busych21(&self) -> bool {
        self.busych21() != 0
    }

    #[doc="Sets the BUSYCH21 field."]
    #[inline] pub fn set_busych21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Busy Channel 22"]
    #[inline] pub fn busych22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if BUSYCH22 != 0"]
    #[inline] pub fn test_busych22(&self) -> bool {
        self.busych22() != 0
    }

    #[doc="Sets the BUSYCH22 field."]
    #[inline] pub fn set_busych22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Busy Channel 23"]
    #[inline] pub fn busych23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if BUSYCH23 != 0"]
    #[inline] pub fn test_busych23(&self) -> bool {
        self.busych23() != 0
    }

    #[doc="Sets the BUSYCH23 field."]
    #[inline] pub fn set_busych23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Busy Channel 24"]
    #[inline] pub fn busych24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if BUSYCH24 != 0"]
    #[inline] pub fn test_busych24(&self) -> bool {
        self.busych24() != 0
    }

    #[doc="Sets the BUSYCH24 field."]
    #[inline] pub fn set_busych24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Busy Channel 25"]
    #[inline] pub fn busych25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if BUSYCH25 != 0"]
    #[inline] pub fn test_busych25(&self) -> bool {
        self.busych25() != 0
    }

    #[doc="Sets the BUSYCH25 field."]
    #[inline] pub fn set_busych25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Busy Channel 26"]
    #[inline] pub fn busych26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if BUSYCH26 != 0"]
    #[inline] pub fn test_busych26(&self) -> bool {
        self.busych26() != 0
    }

    #[doc="Sets the BUSYCH26 field."]
    #[inline] pub fn set_busych26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Busy Channel 27"]
    #[inline] pub fn busych27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if BUSYCH27 != 0"]
    #[inline] pub fn test_busych27(&self) -> bool {
        self.busych27() != 0
    }

    #[doc="Sets the BUSYCH27 field."]
    #[inline] pub fn set_busych27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Busy Channel 28"]
    #[inline] pub fn busych28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if BUSYCH28 != 0"]
    #[inline] pub fn test_busych28(&self) -> bool {
        self.busych28() != 0
    }

    #[doc="Sets the BUSYCH28 field."]
    #[inline] pub fn set_busych28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Busy Channel 29"]
    #[inline] pub fn busych29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if BUSYCH29 != 0"]
    #[inline] pub fn test_busych29(&self) -> bool {
        self.busych29() != 0
    }

    #[doc="Sets the BUSYCH29 field."]
    #[inline] pub fn set_busych29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Busy Channel 30"]
    #[inline] pub fn busych30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if BUSYCH30 != 0"]
    #[inline] pub fn test_busych30(&self) -> bool {
        self.busych30() != 0
    }

    #[doc="Sets the BUSYCH30 field."]
    #[inline] pub fn set_busych30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Busy Channel 31"]
    #[inline] pub fn busych31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if BUSYCH31 != 0"]
    #[inline] pub fn test_busych31(&self) -> bool {
        self.busych31() != 0
    }

    #[doc="Sets the BUSYCH31 field."]
    #[inline] pub fn set_busych31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.busych0() != 0 { try!(write!(f, " busych0"))}
        if self.busych1() != 0 { try!(write!(f, " busych1"))}
        if self.busych2() != 0 { try!(write!(f, " busych2"))}
        if self.busych3() != 0 { try!(write!(f, " busych3"))}
        if self.busych4() != 0 { try!(write!(f, " busych4"))}
        if self.busych5() != 0 { try!(write!(f, " busych5"))}
        if self.busych6() != 0 { try!(write!(f, " busych6"))}
        if self.busych7() != 0 { try!(write!(f, " busych7"))}
        if self.busych8() != 0 { try!(write!(f, " busych8"))}
        if self.busych9() != 0 { try!(write!(f, " busych9"))}
        if self.busych10() != 0 { try!(write!(f, " busych10"))}
        if self.busych11() != 0 { try!(write!(f, " busych11"))}
        if self.busych12() != 0 { try!(write!(f, " busych12"))}
        if self.busych13() != 0 { try!(write!(f, " busych13"))}
        if self.busych14() != 0 { try!(write!(f, " busych14"))}
        if self.busych15() != 0 { try!(write!(f, " busych15"))}
        if self.busych16() != 0 { try!(write!(f, " busych16"))}
        if self.busych17() != 0 { try!(write!(f, " busych17"))}
        if self.busych18() != 0 { try!(write!(f, " busych18"))}
        if self.busych19() != 0 { try!(write!(f, " busych19"))}
        if self.busych20() != 0 { try!(write!(f, " busych20"))}
        if self.busych21() != 0 { try!(write!(f, " busych21"))}
        if self.busych22() != 0 { try!(write!(f, " busych22"))}
        if self.busych23() != 0 { try!(write!(f, " busych23"))}
        if self.busych24() != 0 { try!(write!(f, " busych24"))}
        if self.busych25() != 0 { try!(write!(f, " busych25"))}
        if self.busych26() != 0 { try!(write!(f, " busych26"))}
        if self.busych27() != 0 { try!(write!(f, " busych27"))}
        if self.busych28() != 0 { try!(write!(f, " busych28"))}
        if self.busych29() != 0 { try!(write!(f, " busych29"))}
        if self.busych30() != 0 { try!(write!(f, " busych30"))}
        if self.busych31() != 0 { try!(write!(f, " busych31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pending Channels"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pendch(pub u32);
impl Pendch {
    #[doc="Pending Channel 0"]
    #[inline] pub fn pendch0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PENDCH0 != 0"]
    #[inline] pub fn test_pendch0(&self) -> bool {
        self.pendch0() != 0
    }

    #[doc="Sets the PENDCH0 field."]
    #[inline] pub fn set_pendch0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Pending Channel 1"]
    #[inline] pub fn pendch1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PENDCH1 != 0"]
    #[inline] pub fn test_pendch1(&self) -> bool {
        self.pendch1() != 0
    }

    #[doc="Sets the PENDCH1 field."]
    #[inline] pub fn set_pendch1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pending Channel 2"]
    #[inline] pub fn pendch2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PENDCH2 != 0"]
    #[inline] pub fn test_pendch2(&self) -> bool {
        self.pendch2() != 0
    }

    #[doc="Sets the PENDCH2 field."]
    #[inline] pub fn set_pendch2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pending Channel 3"]
    #[inline] pub fn pendch3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PENDCH3 != 0"]
    #[inline] pub fn test_pendch3(&self) -> bool {
        self.pendch3() != 0
    }

    #[doc="Sets the PENDCH3 field."]
    #[inline] pub fn set_pendch3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pending Channel 4"]
    #[inline] pub fn pendch4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PENDCH4 != 0"]
    #[inline] pub fn test_pendch4(&self) -> bool {
        self.pendch4() != 0
    }

    #[doc="Sets the PENDCH4 field."]
    #[inline] pub fn set_pendch4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pending Channel 5"]
    #[inline] pub fn pendch5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PENDCH5 != 0"]
    #[inline] pub fn test_pendch5(&self) -> bool {
        self.pendch5() != 0
    }

    #[doc="Sets the PENDCH5 field."]
    #[inline] pub fn set_pendch5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Pending Channel 6"]
    #[inline] pub fn pendch6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PENDCH6 != 0"]
    #[inline] pub fn test_pendch6(&self) -> bool {
        self.pendch6() != 0
    }

    #[doc="Sets the PENDCH6 field."]
    #[inline] pub fn set_pendch6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pending Channel 7"]
    #[inline] pub fn pendch7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PENDCH7 != 0"]
    #[inline] pub fn test_pendch7(&self) -> bool {
        self.pendch7() != 0
    }

    #[doc="Sets the PENDCH7 field."]
    #[inline] pub fn set_pendch7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pending Channel 8"]
    #[inline] pub fn pendch8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PENDCH8 != 0"]
    #[inline] pub fn test_pendch8(&self) -> bool {
        self.pendch8() != 0
    }

    #[doc="Sets the PENDCH8 field."]
    #[inline] pub fn set_pendch8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pending Channel 9"]
    #[inline] pub fn pendch9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PENDCH9 != 0"]
    #[inline] pub fn test_pendch9(&self) -> bool {
        self.pendch9() != 0
    }

    #[doc="Sets the PENDCH9 field."]
    #[inline] pub fn set_pendch9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pending Channel 10"]
    #[inline] pub fn pendch10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PENDCH10 != 0"]
    #[inline] pub fn test_pendch10(&self) -> bool {
        self.pendch10() != 0
    }

    #[doc="Sets the PENDCH10 field."]
    #[inline] pub fn set_pendch10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pending Channel 11"]
    #[inline] pub fn pendch11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PENDCH11 != 0"]
    #[inline] pub fn test_pendch11(&self) -> bool {
        self.pendch11() != 0
    }

    #[doc="Sets the PENDCH11 field."]
    #[inline] pub fn set_pendch11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pending Channel 12"]
    #[inline] pub fn pendch12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PENDCH12 != 0"]
    #[inline] pub fn test_pendch12(&self) -> bool {
        self.pendch12() != 0
    }

    #[doc="Sets the PENDCH12 field."]
    #[inline] pub fn set_pendch12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pending Channel 13"]
    #[inline] pub fn pendch13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PENDCH13 != 0"]
    #[inline] pub fn test_pendch13(&self) -> bool {
        self.pendch13() != 0
    }

    #[doc="Sets the PENDCH13 field."]
    #[inline] pub fn set_pendch13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Pending Channel 14"]
    #[inline] pub fn pendch14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PENDCH14 != 0"]
    #[inline] pub fn test_pendch14(&self) -> bool {
        self.pendch14() != 0
    }

    #[doc="Sets the PENDCH14 field."]
    #[inline] pub fn set_pendch14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pending Channel 15"]
    #[inline] pub fn pendch15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PENDCH15 != 0"]
    #[inline] pub fn test_pendch15(&self) -> bool {
        self.pendch15() != 0
    }

    #[doc="Sets the PENDCH15 field."]
    #[inline] pub fn set_pendch15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Pending Channel 16"]
    #[inline] pub fn pendch16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PENDCH16 != 0"]
    #[inline] pub fn test_pendch16(&self) -> bool {
        self.pendch16() != 0
    }

    #[doc="Sets the PENDCH16 field."]
    #[inline] pub fn set_pendch16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pending Channel 17"]
    #[inline] pub fn pendch17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PENDCH17 != 0"]
    #[inline] pub fn test_pendch17(&self) -> bool {
        self.pendch17() != 0
    }

    #[doc="Sets the PENDCH17 field."]
    #[inline] pub fn set_pendch17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pending Channel 18"]
    #[inline] pub fn pendch18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PENDCH18 != 0"]
    #[inline] pub fn test_pendch18(&self) -> bool {
        self.pendch18() != 0
    }

    #[doc="Sets the PENDCH18 field."]
    #[inline] pub fn set_pendch18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pending Channel 19"]
    #[inline] pub fn pendch19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PENDCH19 != 0"]
    #[inline] pub fn test_pendch19(&self) -> bool {
        self.pendch19() != 0
    }

    #[doc="Sets the PENDCH19 field."]
    #[inline] pub fn set_pendch19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pending Channel 20"]
    #[inline] pub fn pendch20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PENDCH20 != 0"]
    #[inline] pub fn test_pendch20(&self) -> bool {
        self.pendch20() != 0
    }

    #[doc="Sets the PENDCH20 field."]
    #[inline] pub fn set_pendch20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pending Channel 21"]
    #[inline] pub fn pendch21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PENDCH21 != 0"]
    #[inline] pub fn test_pendch21(&self) -> bool {
        self.pendch21() != 0
    }

    #[doc="Sets the PENDCH21 field."]
    #[inline] pub fn set_pendch21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Pending Channel 22"]
    #[inline] pub fn pendch22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PENDCH22 != 0"]
    #[inline] pub fn test_pendch22(&self) -> bool {
        self.pendch22() != 0
    }

    #[doc="Sets the PENDCH22 field."]
    #[inline] pub fn set_pendch22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Pending Channel 23"]
    #[inline] pub fn pendch23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PENDCH23 != 0"]
    #[inline] pub fn test_pendch23(&self) -> bool {
        self.pendch23() != 0
    }

    #[doc="Sets the PENDCH23 field."]
    #[inline] pub fn set_pendch23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Pending Channel 24"]
    #[inline] pub fn pendch24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PENDCH24 != 0"]
    #[inline] pub fn test_pendch24(&self) -> bool {
        self.pendch24() != 0
    }

    #[doc="Sets the PENDCH24 field."]
    #[inline] pub fn set_pendch24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pending Channel 25"]
    #[inline] pub fn pendch25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PENDCH25 != 0"]
    #[inline] pub fn test_pendch25(&self) -> bool {
        self.pendch25() != 0
    }

    #[doc="Sets the PENDCH25 field."]
    #[inline] pub fn set_pendch25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pending Channel 26"]
    #[inline] pub fn pendch26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PENDCH26 != 0"]
    #[inline] pub fn test_pendch26(&self) -> bool {
        self.pendch26() != 0
    }

    #[doc="Sets the PENDCH26 field."]
    #[inline] pub fn set_pendch26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pending Channel 27"]
    #[inline] pub fn pendch27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PENDCH27 != 0"]
    #[inline] pub fn test_pendch27(&self) -> bool {
        self.pendch27() != 0
    }

    #[doc="Sets the PENDCH27 field."]
    #[inline] pub fn set_pendch27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pending Channel 28"]
    #[inline] pub fn pendch28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PENDCH28 != 0"]
    #[inline] pub fn test_pendch28(&self) -> bool {
        self.pendch28() != 0
    }

    #[doc="Sets the PENDCH28 field."]
    #[inline] pub fn set_pendch28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pending Channel 29"]
    #[inline] pub fn pendch29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if PENDCH29 != 0"]
    #[inline] pub fn test_pendch29(&self) -> bool {
        self.pendch29() != 0
    }

    #[doc="Sets the PENDCH29 field."]
    #[inline] pub fn set_pendch29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Pending Channel 30"]
    #[inline] pub fn pendch30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PENDCH30 != 0"]
    #[inline] pub fn test_pendch30(&self) -> bool {
        self.pendch30() != 0
    }

    #[doc="Sets the PENDCH30 field."]
    #[inline] pub fn set_pendch30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Pending Channel 31"]
    #[inline] pub fn pendch31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PENDCH31 != 0"]
    #[inline] pub fn test_pendch31(&self) -> bool {
        self.pendch31() != 0
    }

    #[doc="Sets the PENDCH31 field."]
    #[inline] pub fn set_pendch31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.pendch0() != 0 { try!(write!(f, " pendch0"))}
        if self.pendch1() != 0 { try!(write!(f, " pendch1"))}
        if self.pendch2() != 0 { try!(write!(f, " pendch2"))}
        if self.pendch3() != 0 { try!(write!(f, " pendch3"))}
        if self.pendch4() != 0 { try!(write!(f, " pendch4"))}
        if self.pendch5() != 0 { try!(write!(f, " pendch5"))}
        if self.pendch6() != 0 { try!(write!(f, " pendch6"))}
        if self.pendch7() != 0 { try!(write!(f, " pendch7"))}
        if self.pendch8() != 0 { try!(write!(f, " pendch8"))}
        if self.pendch9() != 0 { try!(write!(f, " pendch9"))}
        if self.pendch10() != 0 { try!(write!(f, " pendch10"))}
        if self.pendch11() != 0 { try!(write!(f, " pendch11"))}
        if self.pendch12() != 0 { try!(write!(f, " pendch12"))}
        if self.pendch13() != 0 { try!(write!(f, " pendch13"))}
        if self.pendch14() != 0 { try!(write!(f, " pendch14"))}
        if self.pendch15() != 0 { try!(write!(f, " pendch15"))}
        if self.pendch16() != 0 { try!(write!(f, " pendch16"))}
        if self.pendch17() != 0 { try!(write!(f, " pendch17"))}
        if self.pendch18() != 0 { try!(write!(f, " pendch18"))}
        if self.pendch19() != 0 { try!(write!(f, " pendch19"))}
        if self.pendch20() != 0 { try!(write!(f, " pendch20"))}
        if self.pendch21() != 0 { try!(write!(f, " pendch21"))}
        if self.pendch22() != 0 { try!(write!(f, " pendch22"))}
        if self.pendch23() != 0 { try!(write!(f, " pendch23"))}
        if self.pendch24() != 0 { try!(write!(f, " pendch24"))}
        if self.pendch25() != 0 { try!(write!(f, " pendch25"))}
        if self.pendch26() != 0 { try!(write!(f, " pendch26"))}
        if self.pendch27() != 0 { try!(write!(f, " pendch27"))}
        if self.pendch28() != 0 { try!(write!(f, " pendch28"))}
        if self.pendch29() != 0 { try!(write!(f, " pendch29"))}
        if self.pendch30() != 0 { try!(write!(f, " pendch30"))}
        if self.pendch31() != 0 { try!(write!(f, " pendch31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Active Channel and Levels"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Active(pub u32);
impl Active {
    #[doc="Level 0 Channel Trigger Request Executing"]
    #[inline] pub fn lvlex0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LVLEX0 != 0"]
    #[inline] pub fn test_lvlex0(&self) -> bool {
        self.lvlex0() != 0
    }

    #[doc="Sets the LVLEX0 field."]
    #[inline] pub fn set_lvlex0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Level 1 Channel Trigger Request Executing"]
    #[inline] pub fn lvlex1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LVLEX1 != 0"]
    #[inline] pub fn test_lvlex1(&self) -> bool {
        self.lvlex1() != 0
    }

    #[doc="Sets the LVLEX1 field."]
    #[inline] pub fn set_lvlex1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Level 2 Channel Trigger Request Executing"]
    #[inline] pub fn lvlex2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LVLEX2 != 0"]
    #[inline] pub fn test_lvlex2(&self) -> bool {
        self.lvlex2() != 0
    }

    #[doc="Sets the LVLEX2 field."]
    #[inline] pub fn set_lvlex2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Level 3 Channel Trigger Request Executing"]
    #[inline] pub fn lvlex3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LVLEX3 != 0"]
    #[inline] pub fn test_lvlex3(&self) -> bool {
        self.lvlex3() != 0
    }

    #[doc="Sets the LVLEX3 field."]
    #[inline] pub fn set_lvlex3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
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
        if self.lvlex0() != 0 { try!(write!(f, " lvlex0"))}
        if self.lvlex1() != 0 { try!(write!(f, " lvlex1"))}
        if self.lvlex2() != 0 { try!(write!(f, " lvlex2"))}
        if self.lvlex3() != 0 { try!(write!(f, " lvlex3"))}
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

#[doc="Channel n Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chctrla(pub u32);
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
        let value: u32 = value.into();
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
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel Run in Standby"]
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

    #[doc="Trigger Source"]
    #[inline] pub fn trigsrc(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if TRIGSRC != 0"]
    #[inline] pub fn test_trigsrc(&self) -> bool {
        self.trigsrc() != 0
    }

    #[doc="Sets the TRIGSRC field."]
    #[inline] pub fn set_trigsrc<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Trigger Action"]
    #[inline] pub fn trigact(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if TRIGACT != 0"]
    #[inline] pub fn test_trigact(&self) -> bool {
        self.trigact() != 0
    }

    #[doc="Sets the TRIGACT field."]
    #[inline] pub fn set_trigact<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Burst Length"]
    #[inline] pub fn burstlen(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if BURSTLEN != 0"]
    #[inline] pub fn test_burstlen(&self) -> bool {
        self.burstlen() != 0
    }

    #[doc="Sets the BURSTLEN field."]
    #[inline] pub fn set_burstlen<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="FIFO Threshold"]
    #[inline] pub fn threshold(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if THRESHOLD != 0"]
    #[inline] pub fn test_threshold(&self) -> bool {
        self.threshold() != 0
    }

    #[doc="Sets the THRESHOLD field."]
    #[inline] pub fn set_threshold<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Chctrla {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.trigsrc() != 0 { try!(write!(f, " trigsrc=0x{:x}", self.trigsrc()))}
        if self.trigact() != 0 { try!(write!(f, " trigact=0x{:x}", self.trigact()))}
        if self.burstlen() != 0 { try!(write!(f, " burstlen=0x{:x}", self.burstlen()))}
        if self.threshold() != 0 { try!(write!(f, " threshold=0x{:x}", self.threshold()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel n Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chctrlb(pub u8);
impl Chctrlb {
    #[doc="Software Command"]
    #[inline] pub fn cmd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Chctrlb {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel n Priority Level"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chprilvl(pub u8);
impl Chprilvl {
    #[doc="Channel Priority Level"]
    #[inline] pub fn prilvl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if PRILVL != 0"]
    #[inline] pub fn test_prilvl(&self) -> bool {
        self.prilvl() != 0
    }

    #[doc="Sets the PRILVL field."]
    #[inline] pub fn set_prilvl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Chprilvl {
    #[inline]
    fn from(other: u8) -> Self {
         Chprilvl(other)
    }
}

impl ::core::fmt::Display for Chprilvl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chprilvl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prilvl() != 0 { try!(write!(f, " prilvl=0x{:x}", self.prilvl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel n Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chevctrl(pub u8);
impl Chevctrl {
    #[doc="Channel Event Input Action"]
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
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel Event Output Mode"]
    #[inline] pub fn evomode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if EVOMODE != 0"]
    #[inline] pub fn test_evomode(&self) -> bool {
        self.evomode() != 0
    }

    #[doc="Sets the EVOMODE field."]
    #[inline] pub fn set_evomode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel Event Input Enable"]
    #[inline] pub fn evie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EVIE != 0"]
    #[inline] pub fn test_evie(&self) -> bool {
        self.evie() != 0
    }

    #[doc="Sets the EVIE field."]
    #[inline] pub fn set_evie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel Event Output Enable"]
    #[inline] pub fn evoe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EVOE != 0"]
    #[inline] pub fn test_evoe(&self) -> bool {
        self.evoe() != 0
    }

    #[doc="Sets the EVOE field."]
    #[inline] pub fn set_evoe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Chevctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Chevctrl(other)
    }
}

impl ::core::fmt::Display for Chevctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chevctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
        if self.evomode() != 0 { try!(write!(f, " evomode=0x{:x}", self.evomode()))}
        if self.evie() != 0 { try!(write!(f, " evie"))}
        if self.evoe() != 0 { try!(write!(f, " evoe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel n Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chintenclr(pub u8);
impl Chintenclr {
    #[doc="Channel Transfer Error Interrupt Enable"]
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

    #[doc="Channel Transfer Complete Interrupt Enable"]
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

#[doc="Channel n Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chintenset(pub u8);
impl Chintenset {
    #[doc="Channel Transfer Error Interrupt Enable"]
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

    #[doc="Channel Transfer Complete Interrupt Enable"]
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

#[doc="Channel n Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chintflag(pub u8);
impl Chintflag {
    #[doc="Channel Transfer Error"]
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

    #[doc="Channel Transfer Complete"]
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

#[doc="Channel n Status"]
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

    #[doc="Channel Fetch Error"]
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

    #[doc="Channel CRC Error"]
    #[inline] pub fn crcerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CRCERR != 0"]
    #[inline] pub fn test_crcerr(&self) -> bool {
        self.crcerr() != 0
    }

    #[doc="Sets the CRCERR field."]
    #[inline] pub fn set_crcerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
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
        if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

