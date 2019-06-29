::bobbin_mcu::periph!( USB, Usb, USB_PERIPH, UsbPeriph, USB_OWNED, USB_REF_COUNT, 0x41000000, 0x00, 0x31);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USB Peripheral"]
pub struct UsbPeriph(pub usize); 

impl UsbPeriph {
    #[doc="Get USB is Device Peripheral"]
    #[inline] pub fn device(&self) -> device::Device {
        device::Device(self.0 + 0x0)
    }
    #[doc="Get USB is Host Peripheral"]
    #[inline] pub fn host(&self) -> host::Host {
        host::Host(self.0 + 0x0)
    }
}

#[doc="USB is Device Cluster"]
pub mod device {
    #[allow(unused_imports)] use super::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="USB is Device Peripheral"]
    pub struct Device(pub usize);
impl Device {
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x2)
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

    #[doc="Get the QOSCTRL Register."]
    #[inline] pub fn qosctrl_reg(&self) -> ::bobbin_mcu::register::Register<Qosctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Qosctrl, 0x3)
    }

    #[doc="Get the *mut pointer for the QOSCTRL register."]
    #[inline] pub fn qosctrl_mut(&self) -> *mut Qosctrl { 
        self.qosctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the QOSCTRL register."]
    #[inline] pub fn qosctrl_ptr(&self) -> *const Qosctrl { 
        self.qosctrl_reg().ptr()
    }

    #[doc="Read the QOSCTRL register."]
    #[inline] pub fn qosctrl(&self) -> Qosctrl { 
        self.qosctrl_reg().read()
    }

    #[doc="Write the QOSCTRL register."]
    #[inline] pub fn write_qosctrl(&self, value: Qosctrl) -> &Self { 
        self.qosctrl_reg().write(value);
        self
    }

    #[doc="Set the QOSCTRL register."]
    #[inline] pub fn set_qosctrl<F: FnOnce(Qosctrl) -> Qosctrl>(&self, f: F) -> &Self {
        self.qosctrl_reg().set(f);
        self
    }

    #[doc="Modify the QOSCTRL register."]
    #[inline] pub fn with_qosctrl<F: FnOnce(Qosctrl) -> Qosctrl>(&self, f: F) -> &Self {
        self.qosctrl_reg().with(f);
        self
    }

    #[doc="Get the CTRLB Register."]
    #[inline] pub fn ctrlb_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlb, 0x8)
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

    #[doc="Get the DADD Register."]
    #[inline] pub fn dadd_reg(&self) -> ::bobbin_mcu::register::Register<Dadd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dadd, 0xa)
    }

    #[doc="Get the *mut pointer for the DADD register."]
    #[inline] pub fn dadd_mut(&self) -> *mut Dadd { 
        self.dadd_reg().ptr()
    }

    #[doc="Get the *const pointer for the DADD register."]
    #[inline] pub fn dadd_ptr(&self) -> *const Dadd { 
        self.dadd_reg().ptr()
    }

    #[doc="Read the DADD register."]
    #[inline] pub fn dadd(&self) -> Dadd { 
        self.dadd_reg().read()
    }

    #[doc="Write the DADD register."]
    #[inline] pub fn write_dadd(&self, value: Dadd) -> &Self { 
        self.dadd_reg().write(value);
        self
    }

    #[doc="Set the DADD register."]
    #[inline] pub fn set_dadd<F: FnOnce(Dadd) -> Dadd>(&self, f: F) -> &Self {
        self.dadd_reg().set(f);
        self
    }

    #[doc="Modify the DADD register."]
    #[inline] pub fn with_dadd<F: FnOnce(Dadd) -> Dadd>(&self, f: F) -> &Self {
        self.dadd_reg().with(f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0xc)
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

    #[doc="Get the FSMSTATUS Register."]
    #[inline] pub fn fsmstatus_reg(&self) -> ::bobbin_mcu::register::Register<Fsmstatus> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fsmstatus, 0xd)
    }

    #[doc="Get the *mut pointer for the FSMSTATUS register."]
    #[inline] pub fn fsmstatus_mut(&self) -> *mut Fsmstatus { 
        self.fsmstatus_reg().ptr()
    }

    #[doc="Get the *const pointer for the FSMSTATUS register."]
    #[inline] pub fn fsmstatus_ptr(&self) -> *const Fsmstatus { 
        self.fsmstatus_reg().ptr()
    }

    #[doc="Read the FSMSTATUS register."]
    #[inline] pub fn fsmstatus(&self) -> Fsmstatus { 
        self.fsmstatus_reg().read()
    }

    #[doc="Get the FNUM Register."]
    #[inline] pub fn fnum_reg(&self) -> ::bobbin_mcu::register::Register<Fnum> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fnum, 0x10)
    }

    #[doc="Get the *mut pointer for the FNUM register."]
    #[inline] pub fn fnum_mut(&self) -> *mut Fnum { 
        self.fnum_reg().ptr()
    }

    #[doc="Get the *const pointer for the FNUM register."]
    #[inline] pub fn fnum_ptr(&self) -> *const Fnum { 
        self.fnum_reg().ptr()
    }

    #[doc="Read the FNUM register."]
    #[inline] pub fn fnum(&self) -> Fnum { 
        self.fnum_reg().read()
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x14)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x18)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x1c)
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

    #[doc="Get the EPINTSMRY Register."]
    #[inline] pub fn epintsmry_reg(&self) -> ::bobbin_mcu::register::Register<Epintsmry> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Epintsmry, 0x20)
    }

    #[doc="Get the *mut pointer for the EPINTSMRY register."]
    #[inline] pub fn epintsmry_mut(&self) -> *mut Epintsmry { 
        self.epintsmry_reg().ptr()
    }

    #[doc="Get the *const pointer for the EPINTSMRY register."]
    #[inline] pub fn epintsmry_ptr(&self) -> *const Epintsmry { 
        self.epintsmry_reg().ptr()
    }

    #[doc="Read the EPINTSMRY register."]
    #[inline] pub fn epintsmry(&self) -> Epintsmry { 
        self.epintsmry_reg().read()
    }

    #[doc="Get the DESCADD Register."]
    #[inline] pub fn descadd_reg(&self) -> ::bobbin_mcu::register::Register<Descadd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Descadd, 0x24)
    }

    #[doc="Get the *mut pointer for the DESCADD register."]
    #[inline] pub fn descadd_mut(&self) -> *mut Descadd { 
        self.descadd_reg().ptr()
    }

    #[doc="Get the *const pointer for the DESCADD register."]
    #[inline] pub fn descadd_ptr(&self) -> *const Descadd { 
        self.descadd_reg().ptr()
    }

    #[doc="Read the DESCADD register."]
    #[inline] pub fn descadd(&self) -> Descadd { 
        self.descadd_reg().read()
    }

    #[doc="Write the DESCADD register."]
    #[inline] pub fn write_descadd(&self, value: Descadd) -> &Self { 
        self.descadd_reg().write(value);
        self
    }

    #[doc="Set the DESCADD register."]
    #[inline] pub fn set_descadd<F: FnOnce(Descadd) -> Descadd>(&self, f: F) -> &Self {
        self.descadd_reg().set(f);
        self
    }

    #[doc="Modify the DESCADD register."]
    #[inline] pub fn with_descadd<F: FnOnce(Descadd) -> Descadd>(&self, f: F) -> &Self {
        self.descadd_reg().with(f);
        self
    }

    #[doc="Get the PADCAL Register."]
    #[inline] pub fn padcal_reg(&self) -> ::bobbin_mcu::register::Register<Padcal> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Padcal, 0x28)
    }

    #[doc="Get the *mut pointer for the PADCAL register."]
    #[inline] pub fn padcal_mut(&self) -> *mut Padcal { 
        self.padcal_reg().ptr()
    }

    #[doc="Get the *const pointer for the PADCAL register."]
    #[inline] pub fn padcal_ptr(&self) -> *const Padcal { 
        self.padcal_reg().ptr()
    }

    #[doc="Read the PADCAL register."]
    #[inline] pub fn padcal(&self) -> Padcal { 
        self.padcal_reg().read()
    }

    #[doc="Write the PADCAL register."]
    #[inline] pub fn write_padcal(&self, value: Padcal) -> &Self { 
        self.padcal_reg().write(value);
        self
    }

    #[doc="Set the PADCAL register."]
    #[inline] pub fn set_padcal<F: FnOnce(Padcal) -> Padcal>(&self, f: F) -> &Self {
        self.padcal_reg().set(f);
        self
    }

    #[doc="Modify the PADCAL register."]
    #[inline] pub fn with_padcal<F: FnOnce(Padcal) -> Padcal>(&self, f: F) -> &Self {
        self.padcal_reg().with(f);
        self
    }

    #[doc="Get the EPCFG Register."]
    #[inline] pub fn epcfg_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Epcfg, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Epcfg, 0x100, 0x20)
    }

    #[doc="Get the *mut pointer for the EPCFG register."]
    #[inline] pub fn epcfg_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Epcfg { 
        self.epcfg_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the EPCFG register."]
    #[inline] pub fn epcfg_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Epcfg { 
        self.epcfg_reg().ptr(index.into())
    }

    #[doc="Read the EPCFG register."]
    #[inline] pub fn epcfg<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Epcfg { 
        self.epcfg_reg().read(index.into())
    }

    #[doc="Write the EPCFG register."]
    #[inline] pub fn write_epcfg<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Epcfg) -> &Self {
        self.epcfg_reg().write(index.into(), value);
        self
    }

    #[doc="Set the EPCFG register."]
    #[inline] pub fn set_epcfg<I: Into<::bobbin_bits::R8>, F: FnOnce(Epcfg) -> Epcfg>(&self, index: I, f: F) -> &Self {
        self.epcfg_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the EPCFG register."]
    #[inline] pub fn with_epcfg<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Epcfg) -> Epcfg>(&self, index: I, f: F) -> &Self {
        self.epcfg_reg().with(index.into(), f);
        self
    }

    #[doc="Get the EPSTATUSCLR Register."]
    #[inline] pub fn epstatusclr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Epstatusclr, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Epstatusclr, 0x104, 0x20)
    }

    #[doc="Get the *mut pointer for the EPSTATUSCLR register."]
    #[inline] pub fn epstatusclr_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Epstatusclr { 
        self.epstatusclr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the EPSTATUSCLR register."]
    #[inline] pub fn epstatusclr_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Epstatusclr { 
        self.epstatusclr_reg().ptr(index.into())
    }

    #[doc="Write the EPSTATUSCLR register."]
    #[inline] pub fn write_epstatusclr<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Epstatusclr) -> &Self {
        self.epstatusclr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the EPSTATUSCLR register."]
    #[inline] pub fn set_epstatusclr<I: Into<::bobbin_bits::R8>, F: FnOnce(Epstatusclr) -> Epstatusclr>(&self, index: I, f: F) -> &Self {
        self.epstatusclr_reg().set(index.into(), f);
        self
    }

    #[doc="Get the EPSTATUSSET Register."]
    #[inline] pub fn epstatusset_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Epstatusset, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Epstatusset, 0x105, 0x20)
    }

    #[doc="Get the *mut pointer for the EPSTATUSSET register."]
    #[inline] pub fn epstatusset_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Epstatusset { 
        self.epstatusset_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the EPSTATUSSET register."]
    #[inline] pub fn epstatusset_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Epstatusset { 
        self.epstatusset_reg().ptr(index.into())
    }

    #[doc="Write the EPSTATUSSET register."]
    #[inline] pub fn write_epstatusset<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Epstatusset) -> &Self {
        self.epstatusset_reg().write(index.into(), value);
        self
    }

    #[doc="Set the EPSTATUSSET register."]
    #[inline] pub fn set_epstatusset<I: Into<::bobbin_bits::R8>, F: FnOnce(Epstatusset) -> Epstatusset>(&self, index: I, f: F) -> &Self {
        self.epstatusset_reg().set(index.into(), f);
        self
    }

    #[doc="Get the EPSTATUS Register."]
    #[inline] pub fn epstatus_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Epstatus, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Epstatus, 0x106, 0x20)
    }

    #[doc="Get the *mut pointer for the EPSTATUS register."]
    #[inline] pub fn epstatus_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Epstatus { 
        self.epstatus_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the EPSTATUS register."]
    #[inline] pub fn epstatus_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Epstatus { 
        self.epstatus_reg().ptr(index.into())
    }

    #[doc="Read the EPSTATUS register."]
    #[inline] pub fn epstatus<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Epstatus { 
        self.epstatus_reg().read(index.into())
    }

    #[doc="Get the EPINTFLAG Register."]
    #[inline] pub fn epintflag_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Epintflag, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Epintflag, 0x107, 0x20)
    }

    #[doc="Get the *mut pointer for the EPINTFLAG register."]
    #[inline] pub fn epintflag_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Epintflag { 
        self.epintflag_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the EPINTFLAG register."]
    #[inline] pub fn epintflag_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Epintflag { 
        self.epintflag_reg().ptr(index.into())
    }

    #[doc="Read the EPINTFLAG register."]
    #[inline] pub fn epintflag<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Epintflag { 
        self.epintflag_reg().read(index.into())
    }

    #[doc="Write the EPINTFLAG register."]
    #[inline] pub fn write_epintflag<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Epintflag) -> &Self {
        self.epintflag_reg().write(index.into(), value);
        self
    }

    #[doc="Set the EPINTFLAG register."]
    #[inline] pub fn set_epintflag<I: Into<::bobbin_bits::R8>, F: FnOnce(Epintflag) -> Epintflag>(&self, index: I, f: F) -> &Self {
        self.epintflag_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the EPINTFLAG register."]
    #[inline] pub fn with_epintflag<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Epintflag) -> Epintflag>(&self, index: I, f: F) -> &Self {
        self.epintflag_reg().with(index.into(), f);
        self
    }

    #[doc="Get the EPINTENCLR Register."]
    #[inline] pub fn epintenclr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Epintenclr, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Epintenclr, 0x108, 0x20)
    }

    #[doc="Get the *mut pointer for the EPINTENCLR register."]
    #[inline] pub fn epintenclr_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Epintenclr { 
        self.epintenclr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the EPINTENCLR register."]
    #[inline] pub fn epintenclr_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Epintenclr { 
        self.epintenclr_reg().ptr(index.into())
    }

    #[doc="Read the EPINTENCLR register."]
    #[inline] pub fn epintenclr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Epintenclr { 
        self.epintenclr_reg().read(index.into())
    }

    #[doc="Write the EPINTENCLR register."]
    #[inline] pub fn write_epintenclr<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Epintenclr) -> &Self {
        self.epintenclr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the EPINTENCLR register."]
    #[inline] pub fn set_epintenclr<I: Into<::bobbin_bits::R8>, F: FnOnce(Epintenclr) -> Epintenclr>(&self, index: I, f: F) -> &Self {
        self.epintenclr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the EPINTENCLR register."]
    #[inline] pub fn with_epintenclr<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Epintenclr) -> Epintenclr>(&self, index: I, f: F) -> &Self {
        self.epintenclr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the EPINTENSET Register."]
    #[inline] pub fn epintenset_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Epintenset, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Epintenset, 0x109, 0x20)
    }

    #[doc="Get the *mut pointer for the EPINTENSET register."]
    #[inline] pub fn epintenset_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Epintenset { 
        self.epintenset_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the EPINTENSET register."]
    #[inline] pub fn epintenset_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Epintenset { 
        self.epintenset_reg().ptr(index.into())
    }

    #[doc="Read the EPINTENSET register."]
    #[inline] pub fn epintenset<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Epintenset { 
        self.epintenset_reg().read(index.into())
    }

    #[doc="Write the EPINTENSET register."]
    #[inline] pub fn write_epintenset<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Epintenset) -> &Self {
        self.epintenset_reg().write(index.into(), value);
        self
    }

    #[doc="Set the EPINTENSET register."]
    #[inline] pub fn set_epintenset<I: Into<::bobbin_bits::R8>, F: FnOnce(Epintenset) -> Epintenset>(&self, index: I, f: F) -> &Self {
        self.epintenset_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the EPINTENSET register."]
    #[inline] pub fn with_epintenset<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Epintenset) -> Epintenset>(&self, index: I, f: F) -> &Self {
        self.epintenset_reg().with(index.into(), f);
        self
    }

}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u8);
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
        let value: u8 = value.into();
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Run in Standby Mode"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Operating Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Ctrla {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.mode() != 0 { try!(write!(f, " mode"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u8);
impl Syncbusy {
    #[doc="Software Reset Synchronization Busy"]
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

    #[doc="Enable Synchronization Busy"]
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

impl From<u8> for Syncbusy {
    #[inline]
    fn from(other: u8) -> Self {
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB Quality Of Service"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Qosctrl(pub u8);
impl Qosctrl {
    #[doc="Configuration Quality of Service"]
    #[inline] pub fn cqos(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CQOS != 0"]
    #[inline] pub fn test_cqos(&self) -> bool {
        self.cqos() != 0
    }

    #[doc="Sets the CQOS field."]
    #[inline] pub fn set_cqos<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Quality of Service"]
    #[inline] pub fn dqos(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if DQOS != 0"]
    #[inline] pub fn test_dqos(&self) -> bool {
        self.dqos() != 0
    }

    #[doc="Sets the DQOS field."]
    #[inline] pub fn set_dqos<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Qosctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Qosctrl(other)
    }
}

impl ::core::fmt::Display for Qosctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Qosctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cqos() != 0 { try!(write!(f, " cqos=0x{:x}", self.cqos()))}
        if self.dqos() != 0 { try!(write!(f, " dqos=0x{:x}", self.dqos()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u16);
impl Ctrlb {
    #[doc="Detach"]
    #[inline] pub fn detach(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DETACH != 0"]
    #[inline] pub fn test_detach(&self) -> bool {
        self.detach() != 0
    }

    #[doc="Sets the DETACH field."]
    #[inline] pub fn set_detach<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Upstream Resume"]
    #[inline] pub fn uprsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if UPRSM != 0"]
    #[inline] pub fn test_uprsm(&self) -> bool {
        self.uprsm() != 0
    }

    #[doc="Sets the UPRSM field."]
    #[inline] pub fn set_uprsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Speed Configuration"]
    #[inline] pub fn spdconf(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPDCONF != 0"]
    #[inline] pub fn test_spdconf(&self) -> bool {
        self.spdconf() != 0
    }

    #[doc="Sets the SPDCONF field."]
    #[inline] pub fn set_spdconf<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="No Reply"]
    #[inline] pub fn nreply(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NREPLY != 0"]
    #[inline] pub fn test_nreply(&self) -> bool {
        self.nreply() != 0
    }

    #[doc="Sets the NREPLY field."]
    #[inline] pub fn set_nreply<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Test mode J"]
    #[inline] pub fn tstj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TSTJ != 0"]
    #[inline] pub fn test_tstj(&self) -> bool {
        self.tstj() != 0
    }

    #[doc="Sets the TSTJ field."]
    #[inline] pub fn set_tstj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Test mode K"]
    #[inline] pub fn tstk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TSTK != 0"]
    #[inline] pub fn test_tstk(&self) -> bool {
        self.tstk() != 0
    }

    #[doc="Sets the TSTK field."]
    #[inline] pub fn set_tstk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Test packet mode"]
    #[inline] pub fn tstpckt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TSTPCKT != 0"]
    #[inline] pub fn test_tstpckt(&self) -> bool {
        self.tstpckt() != 0
    }

    #[doc="Sets the TSTPCKT field."]
    #[inline] pub fn set_tstpckt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Specific Operational Mode"]
    #[inline] pub fn opmode2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OPMODE2 != 0"]
    #[inline] pub fn test_opmode2(&self) -> bool {
        self.opmode2() != 0
    }

    #[doc="Sets the OPMODE2 field."]
    #[inline] pub fn set_opmode2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Global NAK"]
    #[inline] pub fn gnak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GNAK != 0"]
    #[inline] pub fn test_gnak(&self) -> bool {
        self.gnak() != 0
    }

    #[doc="Sets the GNAK field."]
    #[inline] pub fn set_gnak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Link Power Management Handshake"]
    #[inline] pub fn lpmhdsk(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if LPMHDSK != 0"]
    #[inline] pub fn test_lpmhdsk(&self) -> bool {
        self.lpmhdsk() != 0
    }

    #[doc="Sets the LPMHDSK field."]
    #[inline] pub fn set_lpmhdsk<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
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
        if self.detach() != 0 { try!(write!(f, " detach"))}
        if self.uprsm() != 0 { try!(write!(f, " uprsm"))}
        if self.spdconf() != 0 { try!(write!(f, " spdconf=0x{:x}", self.spdconf()))}
        if self.nreply() != 0 { try!(write!(f, " nreply"))}
        if self.tstj() != 0 { try!(write!(f, " tstj"))}
        if self.tstk() != 0 { try!(write!(f, " tstk"))}
        if self.tstpckt() != 0 { try!(write!(f, " tstpckt"))}
        if self.opmode2() != 0 { try!(write!(f, " opmode2"))}
        if self.gnak() != 0 { try!(write!(f, " gnak"))}
        if self.lpmhdsk() != 0 { try!(write!(f, " lpmhdsk=0x{:x}", self.lpmhdsk()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE Device Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dadd(pub u8);
impl Dadd {
    #[doc="Device Address"]
    #[inline] pub fn dadd(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if DADD != 0"]
    #[inline] pub fn test_dadd(&self) -> bool {
        self.dadd() != 0
    }

    #[doc="Sets the DADD field."]
    #[inline] pub fn set_dadd<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Device Address Enable"]
    #[inline] pub fn adden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADDEN != 0"]
    #[inline] pub fn test_adden(&self) -> bool {
        self.adden() != 0
    }

    #[doc="Sets the ADDEN field."]
    #[inline] pub fn set_adden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Dadd {
    #[inline]
    fn from(other: u8) -> Self {
         Dadd(other)
    }
}

impl ::core::fmt::Display for Dadd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dadd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dadd() != 0 { try!(write!(f, " dadd=0x{:x}", self.dadd()))}
        if self.adden() != 0 { try!(write!(f, " adden"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Speed Status"]
    #[inline] pub fn speed(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPEED != 0"]
    #[inline] pub fn test_speed(&self) -> bool {
        self.speed() != 0
    }

    #[doc="Sets the SPEED field."]
    #[inline] pub fn set_speed<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="USB Line State Status"]
    #[inline] pub fn linestate(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if LINESTATE != 0"]
    #[inline] pub fn test_linestate(&self) -> bool {
        self.linestate() != 0
    }

    #[doc="Sets the LINESTATE field."]
    #[inline] pub fn set_linestate<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
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
        if self.speed() != 0 { try!(write!(f, " speed=0x{:x}", self.speed()))}
        if self.linestate() != 0 { try!(write!(f, " linestate=0x{:x}", self.linestate()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Finite State Machine Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fsmstatus(pub u8);
impl Fsmstatus {
    #[doc="Fine State Machine Status"]
    #[inline] pub fn fsmstate(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if FSMSTATE != 0"]
    #[inline] pub fn test_fsmstate(&self) -> bool {
        self.fsmstate() != 0
    }

    #[doc="Sets the FSMSTATE field."]
    #[inline] pub fn set_fsmstate<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fsmstatus {
    #[inline]
    fn from(other: u8) -> Self {
         Fsmstatus(other)
    }
}

impl ::core::fmt::Display for Fsmstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fsmstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsmstate() != 0 { try!(write!(f, " fsmstate=0x{:x}", self.fsmstate()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE Device Frame Number"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fnum(pub u16);
impl Fnum {
    #[doc="Micro Frame Number"]
    #[inline] pub fn mfnum(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if MFNUM != 0"]
    #[inline] pub fn test_mfnum(&self) -> bool {
        self.mfnum() != 0
    }

    #[doc="Sets the MFNUM field."]
    #[inline] pub fn set_mfnum<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Frame Number"]
    #[inline] pub fn fnum(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7ff) as u16) } // [13:3]
    }

    #[doc="Returns true if FNUM != 0"]
    #[inline] pub fn test_fnum(&self) -> bool {
        self.fnum() != 0
    }

    #[doc="Sets the FNUM field."]
    #[inline] pub fn set_fnum<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7ff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Frame Number CRC Error"]
    #[inline] pub fn fncerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FNCERR != 0"]
    #[inline] pub fn test_fncerr(&self) -> bool {
        self.fncerr() != 0
    }

    #[doc="Sets the FNCERR field."]
    #[inline] pub fn set_fncerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Fnum {
    #[inline]
    fn from(other: u16) -> Self {
         Fnum(other)
    }
}

impl ::core::fmt::Display for Fnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mfnum() != 0 { try!(write!(f, " mfnum=0x{:x}", self.mfnum()))}
        if self.fnum() != 0 { try!(write!(f, " fnum=0x{:x}", self.fnum()))}
        if self.fncerr() != 0 { try!(write!(f, " fncerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE Device Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u16);
impl Intenclr {
    #[doc="Suspend Interrupt Enable"]
    #[inline] pub fn suspend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SUSPEND != 0"]
    #[inline] pub fn test_suspend(&self) -> bool {
        self.suspend() != 0
    }

    #[doc="Sets the SUSPEND field."]
    #[inline] pub fn set_suspend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Micro Start of Frame Interrupt Enable in High Speed Mode"]
    #[inline] pub fn msof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MSOF != 0"]
    #[inline] pub fn test_msof(&self) -> bool {
        self.msof() != 0
    }

    #[doc="Sets the MSOF field."]
    #[inline] pub fn set_msof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Start Of Frame Interrupt Enable"]
    #[inline] pub fn sof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SOF != 0"]
    #[inline] pub fn test_sof(&self) -> bool {
        self.sof() != 0
    }

    #[doc="Sets the SOF field."]
    #[inline] pub fn set_sof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End of Reset Interrupt Enable"]
    #[inline] pub fn eorst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EORST != 0"]
    #[inline] pub fn test_eorst(&self) -> bool {
        self.eorst() != 0
    }

    #[doc="Sets the EORST field."]
    #[inline] pub fn set_eorst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wake Up Interrupt Enable"]
    #[inline] pub fn wakeup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WAKEUP != 0"]
    #[inline] pub fn test_wakeup(&self) -> bool {
        self.wakeup() != 0
    }

    #[doc="Sets the WAKEUP field."]
    #[inline] pub fn set_wakeup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="End Of Resume Interrupt Enable"]
    #[inline] pub fn eorsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if EORSM != 0"]
    #[inline] pub fn test_eorsm(&self) -> bool {
        self.eorsm() != 0
    }

    #[doc="Sets the EORSM field."]
    #[inline] pub fn set_eorsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Upstream Resume Interrupt Enable"]
    #[inline] pub fn uprsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if UPRSM != 0"]
    #[inline] pub fn test_uprsm(&self) -> bool {
        self.uprsm() != 0
    }

    #[doc="Sets the UPRSM field."]
    #[inline] pub fn set_uprsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Ram Access Interrupt Enable"]
    #[inline] pub fn ramacer(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RAMACER != 0"]
    #[inline] pub fn test_ramacer(&self) -> bool {
        self.ramacer() != 0
    }

    #[doc="Sets the RAMACER field."]
    #[inline] pub fn set_ramacer<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Link Power Management Not Yet Interrupt Enable"]
    #[inline] pub fn lpmnyet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LPMNYET != 0"]
    #[inline] pub fn test_lpmnyet(&self) -> bool {
        self.lpmnyet() != 0
    }

    #[doc="Sets the LPMNYET field."]
    #[inline] pub fn set_lpmnyet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Link Power Management Suspend Interrupt Enable"]
    #[inline] pub fn lpmsusp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LPMSUSP != 0"]
    #[inline] pub fn test_lpmsusp(&self) -> bool {
        self.lpmsusp() != 0
    }

    #[doc="Sets the LPMSUSP field."]
    #[inline] pub fn set_lpmsusp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
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
        if self.suspend() != 0 { try!(write!(f, " suspend"))}
        if self.msof() != 0 { try!(write!(f, " msof"))}
        if self.sof() != 0 { try!(write!(f, " sof"))}
        if self.eorst() != 0 { try!(write!(f, " eorst"))}
        if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
        if self.eorsm() != 0 { try!(write!(f, " eorsm"))}
        if self.uprsm() != 0 { try!(write!(f, " uprsm"))}
        if self.ramacer() != 0 { try!(write!(f, " ramacer"))}
        if self.lpmnyet() != 0 { try!(write!(f, " lpmnyet"))}
        if self.lpmsusp() != 0 { try!(write!(f, " lpmsusp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE Device Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u16);
impl Intenset {
    #[doc="Suspend Interrupt Enable"]
    #[inline] pub fn suspend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SUSPEND != 0"]
    #[inline] pub fn test_suspend(&self) -> bool {
        self.suspend() != 0
    }

    #[doc="Sets the SUSPEND field."]
    #[inline] pub fn set_suspend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Micro Start of Frame Interrupt Enable in High Speed Mode"]
    #[inline] pub fn msof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MSOF != 0"]
    #[inline] pub fn test_msof(&self) -> bool {
        self.msof() != 0
    }

    #[doc="Sets the MSOF field."]
    #[inline] pub fn set_msof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Start Of Frame Interrupt Enable"]
    #[inline] pub fn sof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SOF != 0"]
    #[inline] pub fn test_sof(&self) -> bool {
        self.sof() != 0
    }

    #[doc="Sets the SOF field."]
    #[inline] pub fn set_sof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End of Reset Interrupt Enable"]
    #[inline] pub fn eorst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EORST != 0"]
    #[inline] pub fn test_eorst(&self) -> bool {
        self.eorst() != 0
    }

    #[doc="Sets the EORST field."]
    #[inline] pub fn set_eorst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wake Up Interrupt Enable"]
    #[inline] pub fn wakeup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WAKEUP != 0"]
    #[inline] pub fn test_wakeup(&self) -> bool {
        self.wakeup() != 0
    }

    #[doc="Sets the WAKEUP field."]
    #[inline] pub fn set_wakeup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="End Of Resume Interrupt Enable"]
    #[inline] pub fn eorsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if EORSM != 0"]
    #[inline] pub fn test_eorsm(&self) -> bool {
        self.eorsm() != 0
    }

    #[doc="Sets the EORSM field."]
    #[inline] pub fn set_eorsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Upstream Resume Interrupt Enable"]
    #[inline] pub fn uprsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if UPRSM != 0"]
    #[inline] pub fn test_uprsm(&self) -> bool {
        self.uprsm() != 0
    }

    #[doc="Sets the UPRSM field."]
    #[inline] pub fn set_uprsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Ram Access Interrupt Enable"]
    #[inline] pub fn ramacer(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RAMACER != 0"]
    #[inline] pub fn test_ramacer(&self) -> bool {
        self.ramacer() != 0
    }

    #[doc="Sets the RAMACER field."]
    #[inline] pub fn set_ramacer<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Link Power Management Not Yet Interrupt Enable"]
    #[inline] pub fn lpmnyet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LPMNYET != 0"]
    #[inline] pub fn test_lpmnyet(&self) -> bool {
        self.lpmnyet() != 0
    }

    #[doc="Sets the LPMNYET field."]
    #[inline] pub fn set_lpmnyet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Link Power Management Suspend Interrupt Enable"]
    #[inline] pub fn lpmsusp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LPMSUSP != 0"]
    #[inline] pub fn test_lpmsusp(&self) -> bool {
        self.lpmsusp() != 0
    }

    #[doc="Sets the LPMSUSP field."]
    #[inline] pub fn set_lpmsusp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
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
        if self.suspend() != 0 { try!(write!(f, " suspend"))}
        if self.msof() != 0 { try!(write!(f, " msof"))}
        if self.sof() != 0 { try!(write!(f, " sof"))}
        if self.eorst() != 0 { try!(write!(f, " eorst"))}
        if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
        if self.eorsm() != 0 { try!(write!(f, " eorsm"))}
        if self.uprsm() != 0 { try!(write!(f, " uprsm"))}
        if self.ramacer() != 0 { try!(write!(f, " ramacer"))}
        if self.lpmnyet() != 0 { try!(write!(f, " lpmnyet"))}
        if self.lpmsusp() != 0 { try!(write!(f, " lpmsusp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE Device Interrupt Flag"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u16);
impl Intflag {
    #[doc="Suspend"]
    #[inline] pub fn suspend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SUSPEND != 0"]
    #[inline] pub fn test_suspend(&self) -> bool {
        self.suspend() != 0
    }

    #[doc="Sets the SUSPEND field."]
    #[inline] pub fn set_suspend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Micro Start of Frame in High Speed Mode"]
    #[inline] pub fn msof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MSOF != 0"]
    #[inline] pub fn test_msof(&self) -> bool {
        self.msof() != 0
    }

    #[doc="Sets the MSOF field."]
    #[inline] pub fn set_msof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Start Of Frame"]
    #[inline] pub fn sof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SOF != 0"]
    #[inline] pub fn test_sof(&self) -> bool {
        self.sof() != 0
    }

    #[doc="Sets the SOF field."]
    #[inline] pub fn set_sof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End of Reset"]
    #[inline] pub fn eorst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EORST != 0"]
    #[inline] pub fn test_eorst(&self) -> bool {
        self.eorst() != 0
    }

    #[doc="Sets the EORST field."]
    #[inline] pub fn set_eorst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wake Up"]
    #[inline] pub fn wakeup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WAKEUP != 0"]
    #[inline] pub fn test_wakeup(&self) -> bool {
        self.wakeup() != 0
    }

    #[doc="Sets the WAKEUP field."]
    #[inline] pub fn set_wakeup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="End Of Resume"]
    #[inline] pub fn eorsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if EORSM != 0"]
    #[inline] pub fn test_eorsm(&self) -> bool {
        self.eorsm() != 0
    }

    #[doc="Sets the EORSM field."]
    #[inline] pub fn set_eorsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Upstream Resume"]
    #[inline] pub fn uprsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if UPRSM != 0"]
    #[inline] pub fn test_uprsm(&self) -> bool {
        self.uprsm() != 0
    }

    #[doc="Sets the UPRSM field."]
    #[inline] pub fn set_uprsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Ram Access"]
    #[inline] pub fn ramacer(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RAMACER != 0"]
    #[inline] pub fn test_ramacer(&self) -> bool {
        self.ramacer() != 0
    }

    #[doc="Sets the RAMACER field."]
    #[inline] pub fn set_ramacer<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Link Power Management Not Yet"]
    #[inline] pub fn lpmnyet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LPMNYET != 0"]
    #[inline] pub fn test_lpmnyet(&self) -> bool {
        self.lpmnyet() != 0
    }

    #[doc="Sets the LPMNYET field."]
    #[inline] pub fn set_lpmnyet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Link Power Management Suspend"]
    #[inline] pub fn lpmsusp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LPMSUSP != 0"]
    #[inline] pub fn test_lpmsusp(&self) -> bool {
        self.lpmsusp() != 0
    }

    #[doc="Sets the LPMSUSP field."]
    #[inline] pub fn set_lpmsusp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
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
        if self.suspend() != 0 { try!(write!(f, " suspend"))}
        if self.msof() != 0 { try!(write!(f, " msof"))}
        if self.sof() != 0 { try!(write!(f, " sof"))}
        if self.eorst() != 0 { try!(write!(f, " eorst"))}
        if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
        if self.eorsm() != 0 { try!(write!(f, " eorsm"))}
        if self.uprsm() != 0 { try!(write!(f, " uprsm"))}
        if self.ramacer() != 0 { try!(write!(f, " ramacer"))}
        if self.lpmnyet() != 0 { try!(write!(f, " lpmnyet"))}
        if self.lpmsusp() != 0 { try!(write!(f, " lpmsusp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE End Point Interrupt Summary"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Epintsmry(pub u16);
impl Epintsmry {
    #[doc="End Point 0 Interrupt"]
    #[inline] pub fn epint0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EPINT0 != 0"]
    #[inline] pub fn test_epint0(&self) -> bool {
        self.epint0() != 0
    }

    #[doc="Sets the EPINT0 field."]
    #[inline] pub fn set_epint0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="End Point 1 Interrupt"]
    #[inline] pub fn epint1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPINT1 != 0"]
    #[inline] pub fn test_epint1(&self) -> bool {
        self.epint1() != 0
    }

    #[doc="Sets the EPINT1 field."]
    #[inline] pub fn set_epint1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End Point 2 Interrupt"]
    #[inline] pub fn epint2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EPINT2 != 0"]
    #[inline] pub fn test_epint2(&self) -> bool {
        self.epint2() != 0
    }

    #[doc="Sets the EPINT2 field."]
    #[inline] pub fn set_epint2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End Point 3 Interrupt"]
    #[inline] pub fn epint3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EPINT3 != 0"]
    #[inline] pub fn test_epint3(&self) -> bool {
        self.epint3() != 0
    }

    #[doc="Sets the EPINT3 field."]
    #[inline] pub fn set_epint3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="End Point 4 Interrupt"]
    #[inline] pub fn epint4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EPINT4 != 0"]
    #[inline] pub fn test_epint4(&self) -> bool {
        self.epint4() != 0
    }

    #[doc="Sets the EPINT4 field."]
    #[inline] pub fn set_epint4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="End Point 5 Interrupt"]
    #[inline] pub fn epint5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if EPINT5 != 0"]
    #[inline] pub fn test_epint5(&self) -> bool {
        self.epint5() != 0
    }

    #[doc="Sets the EPINT5 field."]
    #[inline] pub fn set_epint5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="End Point 6 Interrupt"]
    #[inline] pub fn epint6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EPINT6 != 0"]
    #[inline] pub fn test_epint6(&self) -> bool {
        self.epint6() != 0
    }

    #[doc="Sets the EPINT6 field."]
    #[inline] pub fn set_epint6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="End Point 7 Interrupt"]
    #[inline] pub fn epint7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EPINT7 != 0"]
    #[inline] pub fn test_epint7(&self) -> bool {
        self.epint7() != 0
    }

    #[doc="Sets the EPINT7 field."]
    #[inline] pub fn set_epint7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u16> for Epintsmry {
    #[inline]
    fn from(other: u16) -> Self {
         Epintsmry(other)
    }
}

impl ::core::fmt::Display for Epintsmry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Epintsmry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epint0() != 0 { try!(write!(f, " epint0"))}
        if self.epint1() != 0 { try!(write!(f, " epint1"))}
        if self.epint2() != 0 { try!(write!(f, " epint2"))}
        if self.epint3() != 0 { try!(write!(f, " epint3"))}
        if self.epint4() != 0 { try!(write!(f, " epint4"))}
        if self.epint5() != 0 { try!(write!(f, " epint5"))}
        if self.epint6() != 0 { try!(write!(f, " epint6"))}
        if self.epint7() != 0 { try!(write!(f, " epint7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Descriptor Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Descadd(pub u32);
impl Descadd {
    #[doc="Descriptor Address Value"]
    #[inline] pub fn descadd(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DESCADD != 0"]
    #[inline] pub fn test_descadd(&self) -> bool {
        self.descadd() != 0
    }

    #[doc="Sets the DESCADD field."]
    #[inline] pub fn set_descadd<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Descadd {
    #[inline]
    fn from(other: u32) -> Self {
         Descadd(other)
    }
}

impl ::core::fmt::Display for Descadd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Descadd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB PAD Calibration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padcal(pub u16);
impl Padcal {
    #[doc="USB Pad Transp calibration"]
    #[inline] pub fn transp(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if TRANSP != 0"]
    #[inline] pub fn test_transp(&self) -> bool {
        self.transp() != 0
    }

    #[doc="Sets the TRANSP field."]
    #[inline] pub fn set_transp<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="USB Pad Transn calibration"]
    #[inline] pub fn transn(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
    }

    #[doc="Returns true if TRANSN != 0"]
    #[inline] pub fn test_transn(&self) -> bool {
        self.transn() != 0
    }

    #[doc="Sets the TRANSN field."]
    #[inline] pub fn set_transn<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="USB Pad Trim calibration"]
    #[inline] pub fn trim(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if TRIM != 0"]
    #[inline] pub fn test_trim(&self) -> bool {
        self.trim() != 0
    }

    #[doc="Sets the TRIM field."]
    #[inline] pub fn set_trim<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Padcal {
    #[inline]
    fn from(other: u16) -> Self {
         Padcal(other)
    }
}

impl ::core::fmt::Display for Padcal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padcal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.transp() != 0 { try!(write!(f, " transp=0x{:x}", self.transp()))}
        if self.transn() != 0 { try!(write!(f, " transn=0x{:x}", self.transn()))}
        if self.trim() != 0 { try!(write!(f, " trim=0x{:x}", self.trim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE End Point Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Epcfg(pub u8);
impl Epcfg {
    #[doc="End Point Type0"]
    #[inline] pub fn eptype0(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EPTYPE0 != 0"]
    #[inline] pub fn test_eptype0(&self) -> bool {
        self.eptype0() != 0
    }

    #[doc="Sets the EPTYPE0 field."]
    #[inline] pub fn set_eptype0<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="End Point Type1"]
    #[inline] pub fn eptype1(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if EPTYPE1 != 0"]
    #[inline] pub fn test_eptype1(&self) -> bool {
        self.eptype1() != 0
    }

    #[doc="Sets the EPTYPE1 field."]
    #[inline] pub fn set_eptype1<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="NYET Token Disable"]
    #[inline] pub fn nyetdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if NYETDIS != 0"]
    #[inline] pub fn test_nyetdis(&self) -> bool {
        self.nyetdis() != 0
    }

    #[doc="Sets the NYETDIS field."]
    #[inline] pub fn set_nyetdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Epcfg {
    #[inline]
    fn from(other: u8) -> Self {
         Epcfg(other)
    }
}

impl ::core::fmt::Display for Epcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Epcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eptype0() != 0 { try!(write!(f, " eptype0=0x{:x}", self.eptype0()))}
        if self.eptype1() != 0 { try!(write!(f, " eptype1=0x{:x}", self.eptype1()))}
        if self.nyetdis() != 0 { try!(write!(f, " nyetdis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE End Point Pipe Status Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Epstatusclr(pub u8);
impl Epstatusclr {
    #[doc="Data Toggle OUT Clear"]
    #[inline] pub fn dtglout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DTGLOUT != 0"]
    #[inline] pub fn test_dtglout(&self) -> bool {
        self.dtglout() != 0
    }

    #[doc="Sets the DTGLOUT field."]
    #[inline] pub fn set_dtglout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Toggle IN Clear"]
    #[inline] pub fn dtglin(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DTGLIN != 0"]
    #[inline] pub fn test_dtglin(&self) -> bool {
        self.dtglin() != 0
    }

    #[doc="Sets the DTGLIN field."]
    #[inline] pub fn set_dtglin<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Current Bank Clear"]
    #[inline] pub fn curbk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CURBK != 0"]
    #[inline] pub fn test_curbk(&self) -> bool {
        self.curbk() != 0
    }

    #[doc="Sets the CURBK field."]
    #[inline] pub fn set_curbk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Stall 0 Request Clear"]
    #[inline] pub fn stallrq0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if STALLRQ0 != 0"]
    #[inline] pub fn test_stallrq0(&self) -> bool {
        self.stallrq0() != 0
    }

    #[doc="Sets the STALLRQ0 field."]
    #[inline] pub fn set_stallrq0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Stall 1 Request Clear"]
    #[inline] pub fn stallrq1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STALLRQ1 != 0"]
    #[inline] pub fn test_stallrq1(&self) -> bool {
        self.stallrq1() != 0
    }

    #[doc="Sets the STALLRQ1 field."]
    #[inline] pub fn set_stallrq1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Bank 0 Ready Clear"]
    #[inline] pub fn bk0rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BK0RDY != 0"]
    #[inline] pub fn test_bk0rdy(&self) -> bool {
        self.bk0rdy() != 0
    }

    #[doc="Sets the BK0RDY field."]
    #[inline] pub fn set_bk0rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Bank 1 Ready Clear"]
    #[inline] pub fn bk1rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BK1RDY != 0"]
    #[inline] pub fn test_bk1rdy(&self) -> bool {
        self.bk1rdy() != 0
    }

    #[doc="Sets the BK1RDY field."]
    #[inline] pub fn set_bk1rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Epstatusclr {
    #[inline]
    fn from(other: u8) -> Self {
         Epstatusclr(other)
    }
}

impl ::core::fmt::Display for Epstatusclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Epstatusclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dtglout() != 0 { try!(write!(f, " dtglout"))}
        if self.dtglin() != 0 { try!(write!(f, " dtglin"))}
        if self.curbk() != 0 { try!(write!(f, " curbk"))}
        if self.stallrq0() != 0 { try!(write!(f, " stallrq0"))}
        if self.stallrq1() != 0 { try!(write!(f, " stallrq1"))}
        if self.bk0rdy() != 0 { try!(write!(f, " bk0rdy"))}
        if self.bk1rdy() != 0 { try!(write!(f, " bk1rdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE End Point Pipe Status Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Epstatusset(pub u8);
impl Epstatusset {
    #[doc="Data Toggle OUT Set"]
    #[inline] pub fn dtglout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DTGLOUT != 0"]
    #[inline] pub fn test_dtglout(&self) -> bool {
        self.dtglout() != 0
    }

    #[doc="Sets the DTGLOUT field."]
    #[inline] pub fn set_dtglout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Toggle IN Set"]
    #[inline] pub fn dtglin(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DTGLIN != 0"]
    #[inline] pub fn test_dtglin(&self) -> bool {
        self.dtglin() != 0
    }

    #[doc="Sets the DTGLIN field."]
    #[inline] pub fn set_dtglin<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Current Bank Set"]
    #[inline] pub fn curbk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CURBK != 0"]
    #[inline] pub fn test_curbk(&self) -> bool {
        self.curbk() != 0
    }

    #[doc="Sets the CURBK field."]
    #[inline] pub fn set_curbk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Stall 0 Request Set"]
    #[inline] pub fn stallrq0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if STALLRQ0 != 0"]
    #[inline] pub fn test_stallrq0(&self) -> bool {
        self.stallrq0() != 0
    }

    #[doc="Sets the STALLRQ0 field."]
    #[inline] pub fn set_stallrq0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Stall 1 Request Set"]
    #[inline] pub fn stallrq1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STALLRQ1 != 0"]
    #[inline] pub fn test_stallrq1(&self) -> bool {
        self.stallrq1() != 0
    }

    #[doc="Sets the STALLRQ1 field."]
    #[inline] pub fn set_stallrq1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Bank 0 Ready Set"]
    #[inline] pub fn bk0rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BK0RDY != 0"]
    #[inline] pub fn test_bk0rdy(&self) -> bool {
        self.bk0rdy() != 0
    }

    #[doc="Sets the BK0RDY field."]
    #[inline] pub fn set_bk0rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Bank 1 Ready Set"]
    #[inline] pub fn bk1rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BK1RDY != 0"]
    #[inline] pub fn test_bk1rdy(&self) -> bool {
        self.bk1rdy() != 0
    }

    #[doc="Sets the BK1RDY field."]
    #[inline] pub fn set_bk1rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Epstatusset {
    #[inline]
    fn from(other: u8) -> Self {
         Epstatusset(other)
    }
}

impl ::core::fmt::Display for Epstatusset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Epstatusset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dtglout() != 0 { try!(write!(f, " dtglout"))}
        if self.dtglin() != 0 { try!(write!(f, " dtglin"))}
        if self.curbk() != 0 { try!(write!(f, " curbk"))}
        if self.stallrq0() != 0 { try!(write!(f, " stallrq0"))}
        if self.stallrq1() != 0 { try!(write!(f, " stallrq1"))}
        if self.bk0rdy() != 0 { try!(write!(f, " bk0rdy"))}
        if self.bk1rdy() != 0 { try!(write!(f, " bk1rdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE End Point Pipe Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Epstatus(pub u8);
impl Epstatus {
    #[doc="Data Toggle Out"]
    #[inline] pub fn dtglout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DTGLOUT != 0"]
    #[inline] pub fn test_dtglout(&self) -> bool {
        self.dtglout() != 0
    }

    #[doc="Sets the DTGLOUT field."]
    #[inline] pub fn set_dtglout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Toggle In"]
    #[inline] pub fn dtglin(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DTGLIN != 0"]
    #[inline] pub fn test_dtglin(&self) -> bool {
        self.dtglin() != 0
    }

    #[doc="Sets the DTGLIN field."]
    #[inline] pub fn set_dtglin<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Current Bank"]
    #[inline] pub fn curbk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CURBK != 0"]
    #[inline] pub fn test_curbk(&self) -> bool {
        self.curbk() != 0
    }

    #[doc="Sets the CURBK field."]
    #[inline] pub fn set_curbk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Stall 0 Request"]
    #[inline] pub fn stallrq0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if STALLRQ0 != 0"]
    #[inline] pub fn test_stallrq0(&self) -> bool {
        self.stallrq0() != 0
    }

    #[doc="Sets the STALLRQ0 field."]
    #[inline] pub fn set_stallrq0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Stall 1 Request"]
    #[inline] pub fn stallrq1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STALLRQ1 != 0"]
    #[inline] pub fn test_stallrq1(&self) -> bool {
        self.stallrq1() != 0
    }

    #[doc="Sets the STALLRQ1 field."]
    #[inline] pub fn set_stallrq1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Bank 0 ready"]
    #[inline] pub fn bk0rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BK0RDY != 0"]
    #[inline] pub fn test_bk0rdy(&self) -> bool {
        self.bk0rdy() != 0
    }

    #[doc="Sets the BK0RDY field."]
    #[inline] pub fn set_bk0rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Bank 1 ready"]
    #[inline] pub fn bk1rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BK1RDY != 0"]
    #[inline] pub fn test_bk1rdy(&self) -> bool {
        self.bk1rdy() != 0
    }

    #[doc="Sets the BK1RDY field."]
    #[inline] pub fn set_bk1rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Epstatus {
    #[inline]
    fn from(other: u8) -> Self {
         Epstatus(other)
    }
}

impl ::core::fmt::Display for Epstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Epstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dtglout() != 0 { try!(write!(f, " dtglout"))}
        if self.dtglin() != 0 { try!(write!(f, " dtglin"))}
        if self.curbk() != 0 { try!(write!(f, " curbk"))}
        if self.stallrq0() != 0 { try!(write!(f, " stallrq0"))}
        if self.stallrq1() != 0 { try!(write!(f, " stallrq1"))}
        if self.bk0rdy() != 0 { try!(write!(f, " bk0rdy"))}
        if self.bk1rdy() != 0 { try!(write!(f, " bk1rdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE End Point Interrupt Flag"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Epintflag(pub u8);
impl Epintflag {
    #[doc="Transfer Complete 0"]
    #[inline] pub fn trcpt0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TRCPT0 != 0"]
    #[inline] pub fn test_trcpt0(&self) -> bool {
        self.trcpt0() != 0
    }

    #[doc="Sets the TRCPT0 field."]
    #[inline] pub fn set_trcpt0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete 1"]
    #[inline] pub fn trcpt1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRCPT1 != 0"]
    #[inline] pub fn test_trcpt1(&self) -> bool {
        self.trcpt1() != 0
    }

    #[doc="Sets the TRCPT1 field."]
    #[inline] pub fn set_trcpt1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error Flow 0"]
    #[inline] pub fn trfail0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TRFAIL0 != 0"]
    #[inline] pub fn test_trfail0(&self) -> bool {
        self.trfail0() != 0
    }

    #[doc="Sets the TRFAIL0 field."]
    #[inline] pub fn set_trfail0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Error Flow 1"]
    #[inline] pub fn trfail1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TRFAIL1 != 0"]
    #[inline] pub fn test_trfail1(&self) -> bool {
        self.trfail1() != 0
    }

    #[doc="Sets the TRFAIL1 field."]
    #[inline] pub fn set_trfail1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Received Setup"]
    #[inline] pub fn rxstp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXSTP != 0"]
    #[inline] pub fn test_rxstp(&self) -> bool {
        self.rxstp() != 0
    }

    #[doc="Sets the RXSTP field."]
    #[inline] pub fn set_rxstp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Stall 0 In/out"]
    #[inline] pub fn stall0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STALL0 != 0"]
    #[inline] pub fn test_stall0(&self) -> bool {
        self.stall0() != 0
    }

    #[doc="Sets the STALL0 field."]
    #[inline] pub fn set_stall0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Stall 1 In/out"]
    #[inline] pub fn stall1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STALL1 != 0"]
    #[inline] pub fn test_stall1(&self) -> bool {
        self.stall1() != 0
    }

    #[doc="Sets the STALL1 field."]
    #[inline] pub fn set_stall1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Epintflag {
    #[inline]
    fn from(other: u8) -> Self {
         Epintflag(other)
    }
}

impl ::core::fmt::Display for Epintflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Epintflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trcpt0() != 0 { try!(write!(f, " trcpt0"))}
        if self.trcpt1() != 0 { try!(write!(f, " trcpt1"))}
        if self.trfail0() != 0 { try!(write!(f, " trfail0"))}
        if self.trfail1() != 0 { try!(write!(f, " trfail1"))}
        if self.rxstp() != 0 { try!(write!(f, " rxstp"))}
        if self.stall0() != 0 { try!(write!(f, " stall0"))}
        if self.stall1() != 0 { try!(write!(f, " stall1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE End Point Interrupt Clear Flag"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Epintenclr(pub u8);
impl Epintenclr {
    #[doc="Transfer Complete 0 Interrupt Disable"]
    #[inline] pub fn trcpt0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TRCPT0 != 0"]
    #[inline] pub fn test_trcpt0(&self) -> bool {
        self.trcpt0() != 0
    }

    #[doc="Sets the TRCPT0 field."]
    #[inline] pub fn set_trcpt0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete 1 Interrupt Disable"]
    #[inline] pub fn trcpt1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRCPT1 != 0"]
    #[inline] pub fn test_trcpt1(&self) -> bool {
        self.trcpt1() != 0
    }

    #[doc="Sets the TRCPT1 field."]
    #[inline] pub fn set_trcpt1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error Flow 0 Interrupt Disable"]
    #[inline] pub fn trfail0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TRFAIL0 != 0"]
    #[inline] pub fn test_trfail0(&self) -> bool {
        self.trfail0() != 0
    }

    #[doc="Sets the TRFAIL0 field."]
    #[inline] pub fn set_trfail0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Error Flow 1 Interrupt Disable"]
    #[inline] pub fn trfail1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TRFAIL1 != 0"]
    #[inline] pub fn test_trfail1(&self) -> bool {
        self.trfail1() != 0
    }

    #[doc="Sets the TRFAIL1 field."]
    #[inline] pub fn set_trfail1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Received Setup Interrupt Disable"]
    #[inline] pub fn rxstp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXSTP != 0"]
    #[inline] pub fn test_rxstp(&self) -> bool {
        self.rxstp() != 0
    }

    #[doc="Sets the RXSTP field."]
    #[inline] pub fn set_rxstp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Stall 0 In/Out Interrupt Disable"]
    #[inline] pub fn stall0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STALL0 != 0"]
    #[inline] pub fn test_stall0(&self) -> bool {
        self.stall0() != 0
    }

    #[doc="Sets the STALL0 field."]
    #[inline] pub fn set_stall0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Stall 1 In/Out Interrupt Disable"]
    #[inline] pub fn stall1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STALL1 != 0"]
    #[inline] pub fn test_stall1(&self) -> bool {
        self.stall1() != 0
    }

    #[doc="Sets the STALL1 field."]
    #[inline] pub fn set_stall1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Epintenclr {
    #[inline]
    fn from(other: u8) -> Self {
         Epintenclr(other)
    }
}

impl ::core::fmt::Display for Epintenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Epintenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trcpt0() != 0 { try!(write!(f, " trcpt0"))}
        if self.trcpt1() != 0 { try!(write!(f, " trcpt1"))}
        if self.trfail0() != 0 { try!(write!(f, " trfail0"))}
        if self.trfail1() != 0 { try!(write!(f, " trfail1"))}
        if self.rxstp() != 0 { try!(write!(f, " rxstp"))}
        if self.stall0() != 0 { try!(write!(f, " stall0"))}
        if self.stall1() != 0 { try!(write!(f, " stall1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DEVICE End Point Interrupt Set Flag"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Epintenset(pub u8);
impl Epintenset {
    #[doc="Transfer Complete 0 Interrupt Enable"]
    #[inline] pub fn trcpt0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TRCPT0 != 0"]
    #[inline] pub fn test_trcpt0(&self) -> bool {
        self.trcpt0() != 0
    }

    #[doc="Sets the TRCPT0 field."]
    #[inline] pub fn set_trcpt0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete 1 Interrupt Enable"]
    #[inline] pub fn trcpt1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRCPT1 != 0"]
    #[inline] pub fn test_trcpt1(&self) -> bool {
        self.trcpt1() != 0
    }

    #[doc="Sets the TRCPT1 field."]
    #[inline] pub fn set_trcpt1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error Flow 0 Interrupt Enable"]
    #[inline] pub fn trfail0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TRFAIL0 != 0"]
    #[inline] pub fn test_trfail0(&self) -> bool {
        self.trfail0() != 0
    }

    #[doc="Sets the TRFAIL0 field."]
    #[inline] pub fn set_trfail0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Error Flow 1 Interrupt Enable"]
    #[inline] pub fn trfail1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TRFAIL1 != 0"]
    #[inline] pub fn test_trfail1(&self) -> bool {
        self.trfail1() != 0
    }

    #[doc="Sets the TRFAIL1 field."]
    #[inline] pub fn set_trfail1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Received Setup Interrupt Enable"]
    #[inline] pub fn rxstp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXSTP != 0"]
    #[inline] pub fn test_rxstp(&self) -> bool {
        self.rxstp() != 0
    }

    #[doc="Sets the RXSTP field."]
    #[inline] pub fn set_rxstp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Stall 0 In/out Interrupt enable"]
    #[inline] pub fn stall0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STALL0 != 0"]
    #[inline] pub fn test_stall0(&self) -> bool {
        self.stall0() != 0
    }

    #[doc="Sets the STALL0 field."]
    #[inline] pub fn set_stall0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Stall 1 In/out Interrupt enable"]
    #[inline] pub fn stall1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STALL1 != 0"]
    #[inline] pub fn test_stall1(&self) -> bool {
        self.stall1() != 0
    }

    #[doc="Sets the STALL1 field."]
    #[inline] pub fn set_stall1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Epintenset {
    #[inline]
    fn from(other: u8) -> Self {
         Epintenset(other)
    }
}

impl ::core::fmt::Display for Epintenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Epintenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trcpt0() != 0 { try!(write!(f, " trcpt0"))}
        if self.trcpt1() != 0 { try!(write!(f, " trcpt1"))}
        if self.trfail0() != 0 { try!(write!(f, " trfail0"))}
        if self.trfail1() != 0 { try!(write!(f, " trfail1"))}
        if self.rxstp() != 0 { try!(write!(f, " rxstp"))}
        if self.stall0() != 0 { try!(write!(f, " stall0"))}
        if self.stall1() != 0 { try!(write!(f, " stall1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of device

#[doc="USB is Host Cluster"]
pub mod host {
    #[allow(unused_imports)] use super::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="USB is Host Peripheral"]
    pub struct Host(pub usize);
impl Host {
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

    #[doc="Get the SYNCBUSY Register."]
    #[inline] pub fn syncbusy_reg(&self) -> ::bobbin_mcu::register::Register<Syncbusy> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Syncbusy, 0x2)
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

    #[doc="Get the QOSCTRL Register."]
    #[inline] pub fn qosctrl_reg(&self) -> ::bobbin_mcu::register::Register<Qosctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Qosctrl, 0x3)
    }

    #[doc="Get the *mut pointer for the QOSCTRL register."]
    #[inline] pub fn qosctrl_mut(&self) -> *mut Qosctrl { 
        self.qosctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the QOSCTRL register."]
    #[inline] pub fn qosctrl_ptr(&self) -> *const Qosctrl { 
        self.qosctrl_reg().ptr()
    }

    #[doc="Read the QOSCTRL register."]
    #[inline] pub fn qosctrl(&self) -> Qosctrl { 
        self.qosctrl_reg().read()
    }

    #[doc="Write the QOSCTRL register."]
    #[inline] pub fn write_qosctrl(&self, value: Qosctrl) -> &Self { 
        self.qosctrl_reg().write(value);
        self
    }

    #[doc="Set the QOSCTRL register."]
    #[inline] pub fn set_qosctrl<F: FnOnce(Qosctrl) -> Qosctrl>(&self, f: F) -> &Self {
        self.qosctrl_reg().set(f);
        self
    }

    #[doc="Modify the QOSCTRL register."]
    #[inline] pub fn with_qosctrl<F: FnOnce(Qosctrl) -> Qosctrl>(&self, f: F) -> &Self {
        self.qosctrl_reg().with(f);
        self
    }

    #[doc="Get the CTRLB Register."]
    #[inline] pub fn ctrlb_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlb> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlb, 0x8)
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

    #[doc="Get the HSOFC Register."]
    #[inline] pub fn hsofc_reg(&self) -> ::bobbin_mcu::register::Register<Hsofc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hsofc, 0xa)
    }

    #[doc="Get the *mut pointer for the HSOFC register."]
    #[inline] pub fn hsofc_mut(&self) -> *mut Hsofc { 
        self.hsofc_reg().ptr()
    }

    #[doc="Get the *const pointer for the HSOFC register."]
    #[inline] pub fn hsofc_ptr(&self) -> *const Hsofc { 
        self.hsofc_reg().ptr()
    }

    #[doc="Read the HSOFC register."]
    #[inline] pub fn hsofc(&self) -> Hsofc { 
        self.hsofc_reg().read()
    }

    #[doc="Write the HSOFC register."]
    #[inline] pub fn write_hsofc(&self, value: Hsofc) -> &Self { 
        self.hsofc_reg().write(value);
        self
    }

    #[doc="Set the HSOFC register."]
    #[inline] pub fn set_hsofc<F: FnOnce(Hsofc) -> Hsofc>(&self, f: F) -> &Self {
        self.hsofc_reg().set(f);
        self
    }

    #[doc="Modify the HSOFC register."]
    #[inline] pub fn with_hsofc<F: FnOnce(Hsofc) -> Hsofc>(&self, f: F) -> &Self {
        self.hsofc_reg().with(f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0xc)
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

    #[doc="Get the FSMSTATUS Register."]
    #[inline] pub fn fsmstatus_reg(&self) -> ::bobbin_mcu::register::Register<Fsmstatus> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fsmstatus, 0xd)
    }

    #[doc="Get the *mut pointer for the FSMSTATUS register."]
    #[inline] pub fn fsmstatus_mut(&self) -> *mut Fsmstatus { 
        self.fsmstatus_reg().ptr()
    }

    #[doc="Get the *const pointer for the FSMSTATUS register."]
    #[inline] pub fn fsmstatus_ptr(&self) -> *const Fsmstatus { 
        self.fsmstatus_reg().ptr()
    }

    #[doc="Read the FSMSTATUS register."]
    #[inline] pub fn fsmstatus(&self) -> Fsmstatus { 
        self.fsmstatus_reg().read()
    }

    #[doc="Get the FNUM Register."]
    #[inline] pub fn fnum_reg(&self) -> ::bobbin_mcu::register::Register<Fnum> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fnum, 0x10)
    }

    #[doc="Get the *mut pointer for the FNUM register."]
    #[inline] pub fn fnum_mut(&self) -> *mut Fnum { 
        self.fnum_reg().ptr()
    }

    #[doc="Get the *const pointer for the FNUM register."]
    #[inline] pub fn fnum_ptr(&self) -> *const Fnum { 
        self.fnum_reg().ptr()
    }

    #[doc="Read the FNUM register."]
    #[inline] pub fn fnum(&self) -> Fnum { 
        self.fnum_reg().read()
    }

    #[doc="Write the FNUM register."]
    #[inline] pub fn write_fnum(&self, value: Fnum) -> &Self { 
        self.fnum_reg().write(value);
        self
    }

    #[doc="Set the FNUM register."]
    #[inline] pub fn set_fnum<F: FnOnce(Fnum) -> Fnum>(&self, f: F) -> &Self {
        self.fnum_reg().set(f);
        self
    }

    #[doc="Modify the FNUM register."]
    #[inline] pub fn with_fnum<F: FnOnce(Fnum) -> Fnum>(&self, f: F) -> &Self {
        self.fnum_reg().with(f);
        self
    }

    #[doc="Get the FLENHIGH Register."]
    #[inline] pub fn flenhigh_reg(&self) -> ::bobbin_mcu::register::Register<Flenhigh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Flenhigh, 0x12)
    }

    #[doc="Get the *mut pointer for the FLENHIGH register."]
    #[inline] pub fn flenhigh_mut(&self) -> *mut Flenhigh { 
        self.flenhigh_reg().ptr()
    }

    #[doc="Get the *const pointer for the FLENHIGH register."]
    #[inline] pub fn flenhigh_ptr(&self) -> *const Flenhigh { 
        self.flenhigh_reg().ptr()
    }

    #[doc="Read the FLENHIGH register."]
    #[inline] pub fn flenhigh(&self) -> Flenhigh { 
        self.flenhigh_reg().read()
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> ::bobbin_mcu::register::Register<Intenclr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenclr, 0x14)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intenset, 0x18)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut Intflag, 0x1c)
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

    #[doc="Get the PINTSMRY Register."]
    #[inline] pub fn pintsmry_reg(&self) -> ::bobbin_mcu::register::Register<Pintsmry> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pintsmry, 0x20)
    }

    #[doc="Get the *mut pointer for the PINTSMRY register."]
    #[inline] pub fn pintsmry_mut(&self) -> *mut Pintsmry { 
        self.pintsmry_reg().ptr()
    }

    #[doc="Get the *const pointer for the PINTSMRY register."]
    #[inline] pub fn pintsmry_ptr(&self) -> *const Pintsmry { 
        self.pintsmry_reg().ptr()
    }

    #[doc="Read the PINTSMRY register."]
    #[inline] pub fn pintsmry(&self) -> Pintsmry { 
        self.pintsmry_reg().read()
    }

    #[doc="Get the DESCADD Register."]
    #[inline] pub fn descadd_reg(&self) -> ::bobbin_mcu::register::Register<Descadd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Descadd, 0x24)
    }

    #[doc="Get the *mut pointer for the DESCADD register."]
    #[inline] pub fn descadd_mut(&self) -> *mut Descadd { 
        self.descadd_reg().ptr()
    }

    #[doc="Get the *const pointer for the DESCADD register."]
    #[inline] pub fn descadd_ptr(&self) -> *const Descadd { 
        self.descadd_reg().ptr()
    }

    #[doc="Read the DESCADD register."]
    #[inline] pub fn descadd(&self) -> Descadd { 
        self.descadd_reg().read()
    }

    #[doc="Write the DESCADD register."]
    #[inline] pub fn write_descadd(&self, value: Descadd) -> &Self { 
        self.descadd_reg().write(value);
        self
    }

    #[doc="Set the DESCADD register."]
    #[inline] pub fn set_descadd<F: FnOnce(Descadd) -> Descadd>(&self, f: F) -> &Self {
        self.descadd_reg().set(f);
        self
    }

    #[doc="Modify the DESCADD register."]
    #[inline] pub fn with_descadd<F: FnOnce(Descadd) -> Descadd>(&self, f: F) -> &Self {
        self.descadd_reg().with(f);
        self
    }

    #[doc="Get the PADCAL Register."]
    #[inline] pub fn padcal_reg(&self) -> ::bobbin_mcu::register::Register<Padcal> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Padcal, 0x28)
    }

    #[doc="Get the *mut pointer for the PADCAL register."]
    #[inline] pub fn padcal_mut(&self) -> *mut Padcal { 
        self.padcal_reg().ptr()
    }

    #[doc="Get the *const pointer for the PADCAL register."]
    #[inline] pub fn padcal_ptr(&self) -> *const Padcal { 
        self.padcal_reg().ptr()
    }

    #[doc="Read the PADCAL register."]
    #[inline] pub fn padcal(&self) -> Padcal { 
        self.padcal_reg().read()
    }

    #[doc="Write the PADCAL register."]
    #[inline] pub fn write_padcal(&self, value: Padcal) -> &Self { 
        self.padcal_reg().write(value);
        self
    }

    #[doc="Set the PADCAL register."]
    #[inline] pub fn set_padcal<F: FnOnce(Padcal) -> Padcal>(&self, f: F) -> &Self {
        self.padcal_reg().set(f);
        self
    }

    #[doc="Modify the PADCAL register."]
    #[inline] pub fn with_padcal<F: FnOnce(Padcal) -> Padcal>(&self, f: F) -> &Self {
        self.padcal_reg().with(f);
        self
    }

    #[doc="Get the PCFG Register."]
    #[inline] pub fn pcfg_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pcfg, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pcfg, 0x100, 0x20)
    }

    #[doc="Get the *mut pointer for the PCFG register."]
    #[inline] pub fn pcfg_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Pcfg { 
        self.pcfg_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PCFG register."]
    #[inline] pub fn pcfg_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Pcfg { 
        self.pcfg_reg().ptr(index.into())
    }

    #[doc="Read the PCFG register."]
    #[inline] pub fn pcfg<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Pcfg { 
        self.pcfg_reg().read(index.into())
    }

    #[doc="Write the PCFG register."]
    #[inline] pub fn write_pcfg<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Pcfg) -> &Self {
        self.pcfg_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PCFG register."]
    #[inline] pub fn set_pcfg<I: Into<::bobbin_bits::R8>, F: FnOnce(Pcfg) -> Pcfg>(&self, index: I, f: F) -> &Self {
        self.pcfg_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PCFG register."]
    #[inline] pub fn with_pcfg<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Pcfg) -> Pcfg>(&self, index: I, f: F) -> &Self {
        self.pcfg_reg().with(index.into(), f);
        self
    }

    #[doc="Get the BINTERVAL Register."]
    #[inline] pub fn binterval_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Binterval, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Binterval, 0x103, 0x20)
    }

    #[doc="Get the *mut pointer for the BINTERVAL register."]
    #[inline] pub fn binterval_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Binterval { 
        self.binterval_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the BINTERVAL register."]
    #[inline] pub fn binterval_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Binterval { 
        self.binterval_reg().ptr(index.into())
    }

    #[doc="Read the BINTERVAL register."]
    #[inline] pub fn binterval<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Binterval { 
        self.binterval_reg().read(index.into())
    }

    #[doc="Write the BINTERVAL register."]
    #[inline] pub fn write_binterval<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Binterval) -> &Self {
        self.binterval_reg().write(index.into(), value);
        self
    }

    #[doc="Set the BINTERVAL register."]
    #[inline] pub fn set_binterval<I: Into<::bobbin_bits::R8>, F: FnOnce(Binterval) -> Binterval>(&self, index: I, f: F) -> &Self {
        self.binterval_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the BINTERVAL register."]
    #[inline] pub fn with_binterval<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Binterval) -> Binterval>(&self, index: I, f: F) -> &Self {
        self.binterval_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PSTATUSCLR Register."]
    #[inline] pub fn pstatusclr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pstatusclr, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pstatusclr, 0x104, 0x20)
    }

    #[doc="Get the *mut pointer for the PSTATUSCLR register."]
    #[inline] pub fn pstatusclr_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Pstatusclr { 
        self.pstatusclr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PSTATUSCLR register."]
    #[inline] pub fn pstatusclr_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Pstatusclr { 
        self.pstatusclr_reg().ptr(index.into())
    }

    #[doc="Write the PSTATUSCLR register."]
    #[inline] pub fn write_pstatusclr<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Pstatusclr) -> &Self {
        self.pstatusclr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PSTATUSCLR register."]
    #[inline] pub fn set_pstatusclr<I: Into<::bobbin_bits::R8>, F: FnOnce(Pstatusclr) -> Pstatusclr>(&self, index: I, f: F) -> &Self {
        self.pstatusclr_reg().set(index.into(), f);
        self
    }

    #[doc="Get the PSTATUSSET Register."]
    #[inline] pub fn pstatusset_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pstatusset, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pstatusset, 0x105, 0x20)
    }

    #[doc="Get the *mut pointer for the PSTATUSSET register."]
    #[inline] pub fn pstatusset_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Pstatusset { 
        self.pstatusset_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PSTATUSSET register."]
    #[inline] pub fn pstatusset_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Pstatusset { 
        self.pstatusset_reg().ptr(index.into())
    }

    #[doc="Write the PSTATUSSET register."]
    #[inline] pub fn write_pstatusset<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Pstatusset) -> &Self {
        self.pstatusset_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PSTATUSSET register."]
    #[inline] pub fn set_pstatusset<I: Into<::bobbin_bits::R8>, F: FnOnce(Pstatusset) -> Pstatusset>(&self, index: I, f: F) -> &Self {
        self.pstatusset_reg().set(index.into(), f);
        self
    }

    #[doc="Get the PSTATUS Register."]
    #[inline] pub fn pstatus_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pstatus, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pstatus, 0x106, 0x20)
    }

    #[doc="Get the *mut pointer for the PSTATUS register."]
    #[inline] pub fn pstatus_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Pstatus { 
        self.pstatus_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PSTATUS register."]
    #[inline] pub fn pstatus_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Pstatus { 
        self.pstatus_reg().ptr(index.into())
    }

    #[doc="Read the PSTATUS register."]
    #[inline] pub fn pstatus<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Pstatus { 
        self.pstatus_reg().read(index.into())
    }

    #[doc="Get the PINTFLAG Register."]
    #[inline] pub fn pintflag_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pintflag, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pintflag, 0x107, 0x20)
    }

    #[doc="Get the *mut pointer for the PINTFLAG register."]
    #[inline] pub fn pintflag_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Pintflag { 
        self.pintflag_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PINTFLAG register."]
    #[inline] pub fn pintflag_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Pintflag { 
        self.pintflag_reg().ptr(index.into())
    }

    #[doc="Read the PINTFLAG register."]
    #[inline] pub fn pintflag<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Pintflag { 
        self.pintflag_reg().read(index.into())
    }

    #[doc="Write the PINTFLAG register."]
    #[inline] pub fn write_pintflag<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Pintflag) -> &Self {
        self.pintflag_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PINTFLAG register."]
    #[inline] pub fn set_pintflag<I: Into<::bobbin_bits::R8>, F: FnOnce(Pintflag) -> Pintflag>(&self, index: I, f: F) -> &Self {
        self.pintflag_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PINTFLAG register."]
    #[inline] pub fn with_pintflag<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Pintflag) -> Pintflag>(&self, index: I, f: F) -> &Self {
        self.pintflag_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PINTENCLR Register."]
    #[inline] pub fn pintenclr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pintenclr, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pintenclr, 0x108, 0x20)
    }

    #[doc="Get the *mut pointer for the PINTENCLR register."]
    #[inline] pub fn pintenclr_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Pintenclr { 
        self.pintenclr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PINTENCLR register."]
    #[inline] pub fn pintenclr_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Pintenclr { 
        self.pintenclr_reg().ptr(index.into())
    }

    #[doc="Read the PINTENCLR register."]
    #[inline] pub fn pintenclr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Pintenclr { 
        self.pintenclr_reg().read(index.into())
    }

    #[doc="Write the PINTENCLR register."]
    #[inline] pub fn write_pintenclr<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Pintenclr) -> &Self {
        self.pintenclr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PINTENCLR register."]
    #[inline] pub fn set_pintenclr<I: Into<::bobbin_bits::R8>, F: FnOnce(Pintenclr) -> Pintenclr>(&self, index: I, f: F) -> &Self {
        self.pintenclr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PINTENCLR register."]
    #[inline] pub fn with_pintenclr<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Pintenclr) -> Pintenclr>(&self, index: I, f: F) -> &Self {
        self.pintenclr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PINTENSET Register."]
    #[inline] pub fn pintenset_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pintenset, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pintenset, 0x109, 0x20)
    }

    #[doc="Get the *mut pointer for the PINTENSET register."]
    #[inline] pub fn pintenset_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Pintenset { 
        self.pintenset_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PINTENSET register."]
    #[inline] pub fn pintenset_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Pintenset { 
        self.pintenset_reg().ptr(index.into())
    }

    #[doc="Read the PINTENSET register."]
    #[inline] pub fn pintenset<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Pintenset { 
        self.pintenset_reg().read(index.into())
    }

    #[doc="Write the PINTENSET register."]
    #[inline] pub fn write_pintenset<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Pintenset) -> &Self {
        self.pintenset_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PINTENSET register."]
    #[inline] pub fn set_pintenset<I: Into<::bobbin_bits::R8>, F: FnOnce(Pintenset) -> Pintenset>(&self, index: I, f: F) -> &Self {
        self.pintenset_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PINTENSET register."]
    #[inline] pub fn with_pintenset<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Pintenset) -> Pintenset>(&self, index: I, f: F) -> &Self {
        self.pintenset_reg().with(index.into(), f);
        self
    }

}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u8);
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
        let value: u8 = value.into();
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
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Run in Standby Mode"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Operating Mode"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Ctrla {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.mode() != 0 { try!(write!(f, " mode"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Busy"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Syncbusy(pub u8);
impl Syncbusy {
    #[doc="Software Reset Synchronization Busy"]
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

    #[doc="Enable Synchronization Busy"]
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

impl From<u8> for Syncbusy {
    #[inline]
    fn from(other: u8) -> Self {
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
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB Quality Of Service"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Qosctrl(pub u8);
impl Qosctrl {
    #[doc="Configuration Quality of Service"]
    #[inline] pub fn cqos(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CQOS != 0"]
    #[inline] pub fn test_cqos(&self) -> bool {
        self.cqos() != 0
    }

    #[doc="Sets the CQOS field."]
    #[inline] pub fn set_cqos<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Quality of Service"]
    #[inline] pub fn dqos(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if DQOS != 0"]
    #[inline] pub fn test_dqos(&self) -> bool {
        self.dqos() != 0
    }

    #[doc="Sets the DQOS field."]
    #[inline] pub fn set_dqos<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u8> for Qosctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Qosctrl(other)
    }
}

impl ::core::fmt::Display for Qosctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Qosctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cqos() != 0 { try!(write!(f, " cqos=0x{:x}", self.cqos()))}
        if self.dqos() != 0 { try!(write!(f, " dqos=0x{:x}", self.dqos()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlb(pub u16);
impl Ctrlb {
    #[doc="Send USB Resume"]
    #[inline] pub fn resume(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RESUME != 0"]
    #[inline] pub fn test_resume(&self) -> bool {
        self.resume() != 0
    }

    #[doc="Sets the RESUME field."]
    #[inline] pub fn set_resume<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Speed Configuration for Host"]
    #[inline] pub fn spdconf(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPDCONF != 0"]
    #[inline] pub fn test_spdconf(&self) -> bool {
        self.spdconf() != 0
    }

    #[doc="Sets the SPDCONF field."]
    #[inline] pub fn set_spdconf<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Auto Resume Enable"]
    #[inline] pub fn autoresume(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AUTORESUME != 0"]
    #[inline] pub fn test_autoresume(&self) -> bool {
        self.autoresume() != 0
    }

    #[doc="Sets the AUTORESUME field."]
    #[inline] pub fn set_autoresume<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Test mode J"]
    #[inline] pub fn tstj(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TSTJ != 0"]
    #[inline] pub fn test_tstj(&self) -> bool {
        self.tstj() != 0
    }

    #[doc="Sets the TSTJ field."]
    #[inline] pub fn set_tstj<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Test mode K"]
    #[inline] pub fn tstk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TSTK != 0"]
    #[inline] pub fn test_tstk(&self) -> bool {
        self.tstk() != 0
    }

    #[doc="Sets the TSTK field."]
    #[inline] pub fn set_tstk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Start of Frame Generation Enable"]
    #[inline] pub fn sofe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SOFE != 0"]
    #[inline] pub fn test_sofe(&self) -> bool {
        self.sofe() != 0
    }

    #[doc="Sets the SOFE field."]
    #[inline] pub fn set_sofe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Send USB Reset"]
    #[inline] pub fn busreset(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BUSRESET != 0"]
    #[inline] pub fn test_busreset(&self) -> bool {
        self.busreset() != 0
    }

    #[doc="Sets the BUSRESET field."]
    #[inline] pub fn set_busreset<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="VBUS is OK"]
    #[inline] pub fn vbusok(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VBUSOK != 0"]
    #[inline] pub fn test_vbusok(&self) -> bool {
        self.vbusok() != 0
    }

    #[doc="Sets the VBUSOK field."]
    #[inline] pub fn set_vbusok<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Send L1 Resume"]
    #[inline] pub fn l1resume(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if L1RESUME != 0"]
    #[inline] pub fn test_l1resume(&self) -> bool {
        self.l1resume() != 0
    }

    #[doc="Sets the L1RESUME field."]
    #[inline] pub fn set_l1resume<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.resume() != 0 { try!(write!(f, " resume"))}
        if self.spdconf() != 0 { try!(write!(f, " spdconf=0x{:x}", self.spdconf()))}
        if self.autoresume() != 0 { try!(write!(f, " autoresume"))}
        if self.tstj() != 0 { try!(write!(f, " tstj"))}
        if self.tstk() != 0 { try!(write!(f, " tstk"))}
        if self.sofe() != 0 { try!(write!(f, " sofe"))}
        if self.busreset() != 0 { try!(write!(f, " busreset"))}
        if self.vbusok() != 0 { try!(write!(f, " vbusok"))}
        if self.l1resume() != 0 { try!(write!(f, " l1resume"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Host Start Of Frame Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hsofc(pub u8);
impl Hsofc {
    #[doc="Frame Length Control"]
    #[inline] pub fn flenc(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if FLENC != 0"]
    #[inline] pub fn test_flenc(&self) -> bool {
        self.flenc() != 0
    }

    #[doc="Sets the FLENC field."]
    #[inline] pub fn set_flenc<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Frame Length Control Enable"]
    #[inline] pub fn flence(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FLENCE != 0"]
    #[inline] pub fn test_flence(&self) -> bool {
        self.flence() != 0
    }

    #[doc="Sets the FLENCE field."]
    #[inline] pub fn set_flence<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Hsofc {
    #[inline]
    fn from(other: u8) -> Self {
         Hsofc(other)
    }
}

impl ::core::fmt::Display for Hsofc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hsofc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flenc() != 0 { try!(write!(f, " flenc=0x{:x}", self.flenc()))}
        if self.flence() != 0 { try!(write!(f, " flence"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Speed Status"]
    #[inline] pub fn speed(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SPEED != 0"]
    #[inline] pub fn test_speed(&self) -> bool {
        self.speed() != 0
    }

    #[doc="Sets the SPEED field."]
    #[inline] pub fn set_speed<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="USB Line State Status"]
    #[inline] pub fn linestate(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if LINESTATE != 0"]
    #[inline] pub fn test_linestate(&self) -> bool {
        self.linestate() != 0
    }

    #[doc="Sets the LINESTATE field."]
    #[inline] pub fn set_linestate<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
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
        if self.speed() != 0 { try!(write!(f, " speed=0x{:x}", self.speed()))}
        if self.linestate() != 0 { try!(write!(f, " linestate=0x{:x}", self.linestate()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Finite State Machine Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fsmstatus(pub u8);
impl Fsmstatus {
    #[doc="Fine State Machine Status"]
    #[inline] pub fn fsmstate(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if FSMSTATE != 0"]
    #[inline] pub fn test_fsmstate(&self) -> bool {
        self.fsmstate() != 0
    }

    #[doc="Sets the FSMSTATE field."]
    #[inline] pub fn set_fsmstate<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fsmstatus {
    #[inline]
    fn from(other: u8) -> Self {
         Fsmstatus(other)
    }
}

impl ::core::fmt::Display for Fsmstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fsmstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsmstate() != 0 { try!(write!(f, " fsmstate=0x{:x}", self.fsmstate()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Host Frame Number"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fnum(pub u16);
impl Fnum {
    #[doc="Micro Frame Number"]
    #[inline] pub fn mfnum(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if MFNUM != 0"]
    #[inline] pub fn test_mfnum(&self) -> bool {
        self.mfnum() != 0
    }

    #[doc="Sets the MFNUM field."]
    #[inline] pub fn set_mfnum<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Frame Number"]
    #[inline] pub fn fnum(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7ff) as u16) } // [13:3]
    }

    #[doc="Returns true if FNUM != 0"]
    #[inline] pub fn test_fnum(&self) -> bool {
        self.fnum() != 0
    }

    #[doc="Sets the FNUM field."]
    #[inline] pub fn set_fnum<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7ff << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u16> for Fnum {
    #[inline]
    fn from(other: u16) -> Self {
         Fnum(other)
    }
}

impl ::core::fmt::Display for Fnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mfnum() != 0 { try!(write!(f, " mfnum=0x{:x}", self.mfnum()))}
        if self.fnum() != 0 { try!(write!(f, " fnum=0x{:x}", self.fnum()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Host Frame Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flenhigh(pub u8);
impl Flenhigh {
    #[doc="Frame Length"]
    #[inline] pub fn flenhigh(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FLENHIGH != 0"]
    #[inline] pub fn test_flenhigh(&self) -> bool {
        self.flenhigh() != 0
    }

    #[doc="Sets the FLENHIGH field."]
    #[inline] pub fn set_flenhigh<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Flenhigh {
    #[inline]
    fn from(other: u8) -> Self {
         Flenhigh(other)
    }
}

impl ::core::fmt::Display for Flenhigh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Flenhigh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flenhigh() != 0 { try!(write!(f, " flenhigh=0x{:x}", self.flenhigh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Host Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u16);
impl Intenclr {
    #[doc="Host Start Of Frame Interrupt Disable"]
    #[inline] pub fn hsof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSOF != 0"]
    #[inline] pub fn test_hsof(&self) -> bool {
        self.hsof() != 0
    }

    #[doc="Sets the HSOF field."]
    #[inline] pub fn set_hsof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="BUS Reset Interrupt Disable"]
    #[inline] pub fn rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RST != 0"]
    #[inline] pub fn test_rst(&self) -> bool {
        self.rst() != 0
    }

    #[doc="Sets the RST field."]
    #[inline] pub fn set_rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wake Up Interrupt Disable"]
    #[inline] pub fn wakeup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WAKEUP != 0"]
    #[inline] pub fn test_wakeup(&self) -> bool {
        self.wakeup() != 0
    }

    #[doc="Sets the WAKEUP field."]
    #[inline] pub fn set_wakeup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DownStream to Device Interrupt Disable"]
    #[inline] pub fn dnrsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DNRSM != 0"]
    #[inline] pub fn test_dnrsm(&self) -> bool {
        self.dnrsm() != 0
    }

    #[doc="Sets the DNRSM field."]
    #[inline] pub fn set_dnrsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Upstream Resume from Device Interrupt Disable"]
    #[inline] pub fn uprsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if UPRSM != 0"]
    #[inline] pub fn test_uprsm(&self) -> bool {
        self.uprsm() != 0
    }

    #[doc="Sets the UPRSM field."]
    #[inline] pub fn set_uprsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Ram Access Interrupt Disable"]
    #[inline] pub fn ramacer(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RAMACER != 0"]
    #[inline] pub fn test_ramacer(&self) -> bool {
        self.ramacer() != 0
    }

    #[doc="Sets the RAMACER field."]
    #[inline] pub fn set_ramacer<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Device Connection Interrupt Disable"]
    #[inline] pub fn dconn(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DCONN != 0"]
    #[inline] pub fn test_dconn(&self) -> bool {
        self.dconn() != 0
    }

    #[doc="Sets the DCONN field."]
    #[inline] pub fn set_dconn<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Device Disconnection Interrupt Disable"]
    #[inline] pub fn ddisc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DDISC != 0"]
    #[inline] pub fn test_ddisc(&self) -> bool {
        self.ddisc() != 0
    }

    #[doc="Sets the DDISC field."]
    #[inline] pub fn set_ddisc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
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
        if self.hsof() != 0 { try!(write!(f, " hsof"))}
        if self.rst() != 0 { try!(write!(f, " rst"))}
        if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
        if self.dnrsm() != 0 { try!(write!(f, " dnrsm"))}
        if self.uprsm() != 0 { try!(write!(f, " uprsm"))}
        if self.ramacer() != 0 { try!(write!(f, " ramacer"))}
        if self.dconn() != 0 { try!(write!(f, " dconn"))}
        if self.ddisc() != 0 { try!(write!(f, " ddisc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Host Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u16);
impl Intenset {
    #[doc="Host Start Of Frame Interrupt Enable"]
    #[inline] pub fn hsof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSOF != 0"]
    #[inline] pub fn test_hsof(&self) -> bool {
        self.hsof() != 0
    }

    #[doc="Sets the HSOF field."]
    #[inline] pub fn set_hsof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Bus Reset Interrupt Enable"]
    #[inline] pub fn rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RST != 0"]
    #[inline] pub fn test_rst(&self) -> bool {
        self.rst() != 0
    }

    #[doc="Sets the RST field."]
    #[inline] pub fn set_rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wake Up Interrupt Enable"]
    #[inline] pub fn wakeup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WAKEUP != 0"]
    #[inline] pub fn test_wakeup(&self) -> bool {
        self.wakeup() != 0
    }

    #[doc="Sets the WAKEUP field."]
    #[inline] pub fn set_wakeup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DownStream to the Device Interrupt Enable"]
    #[inline] pub fn dnrsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DNRSM != 0"]
    #[inline] pub fn test_dnrsm(&self) -> bool {
        self.dnrsm() != 0
    }

    #[doc="Sets the DNRSM field."]
    #[inline] pub fn set_dnrsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Upstream Resume fromthe device Interrupt Enable"]
    #[inline] pub fn uprsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if UPRSM != 0"]
    #[inline] pub fn test_uprsm(&self) -> bool {
        self.uprsm() != 0
    }

    #[doc="Sets the UPRSM field."]
    #[inline] pub fn set_uprsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Ram Access Interrupt Enable"]
    #[inline] pub fn ramacer(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RAMACER != 0"]
    #[inline] pub fn test_ramacer(&self) -> bool {
        self.ramacer() != 0
    }

    #[doc="Sets the RAMACER field."]
    #[inline] pub fn set_ramacer<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Link Power Management Interrupt Enable"]
    #[inline] pub fn dconn(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DCONN != 0"]
    #[inline] pub fn test_dconn(&self) -> bool {
        self.dconn() != 0
    }

    #[doc="Sets the DCONN field."]
    #[inline] pub fn set_dconn<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Device Disconnection Interrupt Enable"]
    #[inline] pub fn ddisc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DDISC != 0"]
    #[inline] pub fn test_ddisc(&self) -> bool {
        self.ddisc() != 0
    }

    #[doc="Sets the DDISC field."]
    #[inline] pub fn set_ddisc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
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
        if self.hsof() != 0 { try!(write!(f, " hsof"))}
        if self.rst() != 0 { try!(write!(f, " rst"))}
        if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
        if self.dnrsm() != 0 { try!(write!(f, " dnrsm"))}
        if self.uprsm() != 0 { try!(write!(f, " uprsm"))}
        if self.ramacer() != 0 { try!(write!(f, " ramacer"))}
        if self.dconn() != 0 { try!(write!(f, " dconn"))}
        if self.ddisc() != 0 { try!(write!(f, " ddisc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Host Interrupt Flag"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u16);
impl Intflag {
    #[doc="Host Start Of Frame"]
    #[inline] pub fn hsof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSOF != 0"]
    #[inline] pub fn test_hsof(&self) -> bool {
        self.hsof() != 0
    }

    #[doc="Sets the HSOF field."]
    #[inline] pub fn set_hsof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Bus Reset"]
    #[inline] pub fn rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RST != 0"]
    #[inline] pub fn test_rst(&self) -> bool {
        self.rst() != 0
    }

    #[doc="Sets the RST field."]
    #[inline] pub fn set_rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wake Up"]
    #[inline] pub fn wakeup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WAKEUP != 0"]
    #[inline] pub fn test_wakeup(&self) -> bool {
        self.wakeup() != 0
    }

    #[doc="Sets the WAKEUP field."]
    #[inline] pub fn set_wakeup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Downstream"]
    #[inline] pub fn dnrsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DNRSM != 0"]
    #[inline] pub fn test_dnrsm(&self) -> bool {
        self.dnrsm() != 0
    }

    #[doc="Sets the DNRSM field."]
    #[inline] pub fn set_dnrsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Upstream Resume from the Device"]
    #[inline] pub fn uprsm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if UPRSM != 0"]
    #[inline] pub fn test_uprsm(&self) -> bool {
        self.uprsm() != 0
    }

    #[doc="Sets the UPRSM field."]
    #[inline] pub fn set_uprsm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Ram Access"]
    #[inline] pub fn ramacer(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RAMACER != 0"]
    #[inline] pub fn test_ramacer(&self) -> bool {
        self.ramacer() != 0
    }

    #[doc="Sets the RAMACER field."]
    #[inline] pub fn set_ramacer<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Device Connection"]
    #[inline] pub fn dconn(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DCONN != 0"]
    #[inline] pub fn test_dconn(&self) -> bool {
        self.dconn() != 0
    }

    #[doc="Sets the DCONN field."]
    #[inline] pub fn set_dconn<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Device Disconnection"]
    #[inline] pub fn ddisc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DDISC != 0"]
    #[inline] pub fn test_ddisc(&self) -> bool {
        self.ddisc() != 0
    }

    #[doc="Sets the DDISC field."]
    #[inline] pub fn set_ddisc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
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
        if self.hsof() != 0 { try!(write!(f, " hsof"))}
        if self.rst() != 0 { try!(write!(f, " rst"))}
        if self.wakeup() != 0 { try!(write!(f, " wakeup"))}
        if self.dnrsm() != 0 { try!(write!(f, " dnrsm"))}
        if self.uprsm() != 0 { try!(write!(f, " uprsm"))}
        if self.ramacer() != 0 { try!(write!(f, " ramacer"))}
        if self.dconn() != 0 { try!(write!(f, " dconn"))}
        if self.ddisc() != 0 { try!(write!(f, " ddisc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Pipe Interrupt Summary"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pintsmry(pub u16);
impl Pintsmry {
    #[doc="Pipe 0 Interrupt"]
    #[inline] pub fn epint0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EPINT0 != 0"]
    #[inline] pub fn test_epint0(&self) -> bool {
        self.epint0() != 0
    }

    #[doc="Sets the EPINT0 field."]
    #[inline] pub fn set_epint0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Pipe 1 Interrupt"]
    #[inline] pub fn epint1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPINT1 != 0"]
    #[inline] pub fn test_epint1(&self) -> bool {
        self.epint1() != 0
    }

    #[doc="Sets the EPINT1 field."]
    #[inline] pub fn set_epint1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pipe 2 Interrupt"]
    #[inline] pub fn epint2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EPINT2 != 0"]
    #[inline] pub fn test_epint2(&self) -> bool {
        self.epint2() != 0
    }

    #[doc="Sets the EPINT2 field."]
    #[inline] pub fn set_epint2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pipe 3 Interrupt"]
    #[inline] pub fn epint3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EPINT3 != 0"]
    #[inline] pub fn test_epint3(&self) -> bool {
        self.epint3() != 0
    }

    #[doc="Sets the EPINT3 field."]
    #[inline] pub fn set_epint3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pipe 4 Interrupt"]
    #[inline] pub fn epint4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EPINT4 != 0"]
    #[inline] pub fn test_epint4(&self) -> bool {
        self.epint4() != 0
    }

    #[doc="Sets the EPINT4 field."]
    #[inline] pub fn set_epint4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pipe 5 Interrupt"]
    #[inline] pub fn epint5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if EPINT5 != 0"]
    #[inline] pub fn test_epint5(&self) -> bool {
        self.epint5() != 0
    }

    #[doc="Sets the EPINT5 field."]
    #[inline] pub fn set_epint5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Pipe 6 Interrupt"]
    #[inline] pub fn epint6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EPINT6 != 0"]
    #[inline] pub fn test_epint6(&self) -> bool {
        self.epint6() != 0
    }

    #[doc="Sets the EPINT6 field."]
    #[inline] pub fn set_epint6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pipe 7 Interrupt"]
    #[inline] pub fn epint7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EPINT7 != 0"]
    #[inline] pub fn test_epint7(&self) -> bool {
        self.epint7() != 0
    }

    #[doc="Sets the EPINT7 field."]
    #[inline] pub fn set_epint7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u16> for Pintsmry {
    #[inline]
    fn from(other: u16) -> Self {
         Pintsmry(other)
    }
}

impl ::core::fmt::Display for Pintsmry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pintsmry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epint0() != 0 { try!(write!(f, " epint0"))}
        if self.epint1() != 0 { try!(write!(f, " epint1"))}
        if self.epint2() != 0 { try!(write!(f, " epint2"))}
        if self.epint3() != 0 { try!(write!(f, " epint3"))}
        if self.epint4() != 0 { try!(write!(f, " epint4"))}
        if self.epint5() != 0 { try!(write!(f, " epint5"))}
        if self.epint6() != 0 { try!(write!(f, " epint6"))}
        if self.epint7() != 0 { try!(write!(f, " epint7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Descriptor Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Descadd(pub u32);
impl Descadd {
    #[doc="Descriptor Address Value"]
    #[inline] pub fn descadd(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DESCADD != 0"]
    #[inline] pub fn test_descadd(&self) -> bool {
        self.descadd() != 0
    }

    #[doc="Sets the DESCADD field."]
    #[inline] pub fn set_descadd<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Descadd {
    #[inline]
    fn from(other: u32) -> Self {
         Descadd(other)
    }
}

impl ::core::fmt::Display for Descadd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Descadd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="USB PAD Calibration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Padcal(pub u16);
impl Padcal {
    #[doc="USB Pad Transp calibration"]
    #[inline] pub fn transp(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if TRANSP != 0"]
    #[inline] pub fn test_transp(&self) -> bool {
        self.transp() != 0
    }

    #[doc="Sets the TRANSP field."]
    #[inline] pub fn set_transp<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="USB Pad Transn calibration"]
    #[inline] pub fn transn(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
    }

    #[doc="Returns true if TRANSN != 0"]
    #[inline] pub fn test_transn(&self) -> bool {
        self.transn() != 0
    }

    #[doc="Sets the TRANSN field."]
    #[inline] pub fn set_transn<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="USB Pad Trim calibration"]
    #[inline] pub fn trim(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if TRIM != 0"]
    #[inline] pub fn test_trim(&self) -> bool {
        self.trim() != 0
    }

    #[doc="Sets the TRIM field."]
    #[inline] pub fn set_trim<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Padcal {
    #[inline]
    fn from(other: u16) -> Self {
         Padcal(other)
    }
}

impl ::core::fmt::Display for Padcal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Padcal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.transp() != 0 { try!(write!(f, " transp=0x{:x}", self.transp()))}
        if self.transn() != 0 { try!(write!(f, " transn=0x{:x}", self.transn()))}
        if self.trim() != 0 { try!(write!(f, " trim=0x{:x}", self.trim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST End Point Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcfg(pub u8);
impl Pcfg {
    #[doc="Pipe Token"]
    #[inline] pub fn ptoken(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if PTOKEN != 0"]
    #[inline] pub fn test_ptoken(&self) -> bool {
        self.ptoken() != 0
    }

    #[doc="Sets the PTOKEN field."]
    #[inline] pub fn set_ptoken<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Pipe Bank"]
    #[inline] pub fn bk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BK != 0"]
    #[inline] pub fn test_bk(&self) -> bool {
        self.bk() != 0
    }

    #[doc="Sets the BK field."]
    #[inline] pub fn set_bk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pipe Type"]
    #[inline] pub fn ptype(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if PTYPE != 0"]
    #[inline] pub fn test_ptype(&self) -> bool {
        self.ptype() != 0
    }

    #[doc="Sets the PTYPE field."]
    #[inline] pub fn set_ptype<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Pcfg {
    #[inline]
    fn from(other: u8) -> Self {
         Pcfg(other)
    }
}

impl ::core::fmt::Display for Pcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ptoken() != 0 { try!(write!(f, " ptoken=0x{:x}", self.ptoken()))}
        if self.bk() != 0 { try!(write!(f, " bk"))}
        if self.ptype() != 0 { try!(write!(f, " ptype=0x{:x}", self.ptype()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Bus Access Period of Pipe"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Binterval(pub u8);
impl Binterval {
    #[doc="Bit Interval"]
    #[inline] pub fn bitinterval(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BITINTERVAL != 0"]
    #[inline] pub fn test_bitinterval(&self) -> bool {
        self.bitinterval() != 0
    }

    #[doc="Sets the BITINTERVAL field."]
    #[inline] pub fn set_bitinterval<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Binterval {
    #[inline]
    fn from(other: u8) -> Self {
         Binterval(other)
    }
}

impl ::core::fmt::Display for Binterval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Binterval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bitinterval() != 0 { try!(write!(f, " bitinterval=0x{:x}", self.bitinterval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST End Point Pipe Status Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pstatusclr(pub u8);
impl Pstatusclr {
    #[doc="Data Toggle clear"]
    #[inline] pub fn dtgl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DTGL != 0"]
    #[inline] pub fn test_dtgl(&self) -> bool {
        self.dtgl() != 0
    }

    #[doc="Sets the DTGL field."]
    #[inline] pub fn set_dtgl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Curren Bank clear"]
    #[inline] pub fn curbk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CURBK != 0"]
    #[inline] pub fn test_curbk(&self) -> bool {
        self.curbk() != 0
    }

    #[doc="Sets the CURBK field."]
    #[inline] pub fn set_curbk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pipe Freeze Clear"]
    #[inline] pub fn pfreeze(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PFREEZE != 0"]
    #[inline] pub fn test_pfreeze(&self) -> bool {
        self.pfreeze() != 0
    }

    #[doc="Sets the PFREEZE field."]
    #[inline] pub fn set_pfreeze<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Bank 0 Ready Clear"]
    #[inline] pub fn bk0rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BK0RDY != 0"]
    #[inline] pub fn test_bk0rdy(&self) -> bool {
        self.bk0rdy() != 0
    }

    #[doc="Sets the BK0RDY field."]
    #[inline] pub fn set_bk0rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Bank 1 Ready Clear"]
    #[inline] pub fn bk1rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BK1RDY != 0"]
    #[inline] pub fn test_bk1rdy(&self) -> bool {
        self.bk1rdy() != 0
    }

    #[doc="Sets the BK1RDY field."]
    #[inline] pub fn set_bk1rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Pstatusclr {
    #[inline]
    fn from(other: u8) -> Self {
         Pstatusclr(other)
    }
}

impl ::core::fmt::Display for Pstatusclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pstatusclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dtgl() != 0 { try!(write!(f, " dtgl"))}
        if self.curbk() != 0 { try!(write!(f, " curbk"))}
        if self.pfreeze() != 0 { try!(write!(f, " pfreeze"))}
        if self.bk0rdy() != 0 { try!(write!(f, " bk0rdy"))}
        if self.bk1rdy() != 0 { try!(write!(f, " bk1rdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST End Point Pipe Status Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pstatusset(pub u8);
impl Pstatusset {
    #[doc="Data Toggle Set"]
    #[inline] pub fn dtgl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DTGL != 0"]
    #[inline] pub fn test_dtgl(&self) -> bool {
        self.dtgl() != 0
    }

    #[doc="Sets the DTGL field."]
    #[inline] pub fn set_dtgl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Current Bank Set"]
    #[inline] pub fn curbk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CURBK != 0"]
    #[inline] pub fn test_curbk(&self) -> bool {
        self.curbk() != 0
    }

    #[doc="Sets the CURBK field."]
    #[inline] pub fn set_curbk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pipe Freeze Set"]
    #[inline] pub fn pfreeze(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PFREEZE != 0"]
    #[inline] pub fn test_pfreeze(&self) -> bool {
        self.pfreeze() != 0
    }

    #[doc="Sets the PFREEZE field."]
    #[inline] pub fn set_pfreeze<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Bank 0 Ready Set"]
    #[inline] pub fn bk0rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BK0RDY != 0"]
    #[inline] pub fn test_bk0rdy(&self) -> bool {
        self.bk0rdy() != 0
    }

    #[doc="Sets the BK0RDY field."]
    #[inline] pub fn set_bk0rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Bank 1 Ready Set"]
    #[inline] pub fn bk1rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BK1RDY != 0"]
    #[inline] pub fn test_bk1rdy(&self) -> bool {
        self.bk1rdy() != 0
    }

    #[doc="Sets the BK1RDY field."]
    #[inline] pub fn set_bk1rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Pstatusset {
    #[inline]
    fn from(other: u8) -> Self {
         Pstatusset(other)
    }
}

impl ::core::fmt::Display for Pstatusset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pstatusset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dtgl() != 0 { try!(write!(f, " dtgl"))}
        if self.curbk() != 0 { try!(write!(f, " curbk"))}
        if self.pfreeze() != 0 { try!(write!(f, " pfreeze"))}
        if self.bk0rdy() != 0 { try!(write!(f, " bk0rdy"))}
        if self.bk1rdy() != 0 { try!(write!(f, " bk1rdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST End Point Pipe Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pstatus(pub u8);
impl Pstatus {
    #[doc="Data Toggle"]
    #[inline] pub fn dtgl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DTGL != 0"]
    #[inline] pub fn test_dtgl(&self) -> bool {
        self.dtgl() != 0
    }

    #[doc="Sets the DTGL field."]
    #[inline] pub fn set_dtgl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Current Bank"]
    #[inline] pub fn curbk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CURBK != 0"]
    #[inline] pub fn test_curbk(&self) -> bool {
        self.curbk() != 0
    }

    #[doc="Sets the CURBK field."]
    #[inline] pub fn set_curbk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pipe Freeze"]
    #[inline] pub fn pfreeze(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PFREEZE != 0"]
    #[inline] pub fn test_pfreeze(&self) -> bool {
        self.pfreeze() != 0
    }

    #[doc="Sets the PFREEZE field."]
    #[inline] pub fn set_pfreeze<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Bank 0 ready"]
    #[inline] pub fn bk0rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BK0RDY != 0"]
    #[inline] pub fn test_bk0rdy(&self) -> bool {
        self.bk0rdy() != 0
    }

    #[doc="Sets the BK0RDY field."]
    #[inline] pub fn set_bk0rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Bank 1 ready"]
    #[inline] pub fn bk1rdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BK1RDY != 0"]
    #[inline] pub fn test_bk1rdy(&self) -> bool {
        self.bk1rdy() != 0
    }

    #[doc="Sets the BK1RDY field."]
    #[inline] pub fn set_bk1rdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Pstatus {
    #[inline]
    fn from(other: u8) -> Self {
         Pstatus(other)
    }
}

impl ::core::fmt::Display for Pstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dtgl() != 0 { try!(write!(f, " dtgl"))}
        if self.curbk() != 0 { try!(write!(f, " curbk"))}
        if self.pfreeze() != 0 { try!(write!(f, " pfreeze"))}
        if self.bk0rdy() != 0 { try!(write!(f, " bk0rdy"))}
        if self.bk1rdy() != 0 { try!(write!(f, " bk1rdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Pipe Interrupt Flag"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pintflag(pub u8);
impl Pintflag {
    #[doc="Transfer Complete 0 Interrupt Flag"]
    #[inline] pub fn trcpt0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TRCPT0 != 0"]
    #[inline] pub fn test_trcpt0(&self) -> bool {
        self.trcpt0() != 0
    }

    #[doc="Sets the TRCPT0 field."]
    #[inline] pub fn set_trcpt0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete 1 Interrupt Flag"]
    #[inline] pub fn trcpt1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRCPT1 != 0"]
    #[inline] pub fn test_trcpt1(&self) -> bool {
        self.trcpt1() != 0
    }

    #[doc="Sets the TRCPT1 field."]
    #[inline] pub fn set_trcpt1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error Flow Interrupt Flag"]
    #[inline] pub fn trfail(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TRFAIL != 0"]
    #[inline] pub fn test_trfail(&self) -> bool {
        self.trfail() != 0
    }

    #[doc="Sets the TRFAIL field."]
    #[inline] pub fn set_trfail<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pipe Error Interrupt Flag"]
    #[inline] pub fn perr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit Setup Interrupt Flag"]
    #[inline] pub fn txstp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXSTP != 0"]
    #[inline] pub fn test_txstp(&self) -> bool {
        self.txstp() != 0
    }

    #[doc="Sets the TXSTP field."]
    #[inline] pub fn set_txstp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Stall Interrupt Flag"]
    #[inline] pub fn stall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Pintflag {
    #[inline]
    fn from(other: u8) -> Self {
         Pintflag(other)
    }
}

impl ::core::fmt::Display for Pintflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pintflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trcpt0() != 0 { try!(write!(f, " trcpt0"))}
        if self.trcpt1() != 0 { try!(write!(f, " trcpt1"))}
        if self.trfail() != 0 { try!(write!(f, " trfail"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.txstp() != 0 { try!(write!(f, " txstp"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Pipe Interrupt Flag Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pintenclr(pub u8);
impl Pintenclr {
    #[doc="Transfer Complete 0 Disable"]
    #[inline] pub fn trcpt0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TRCPT0 != 0"]
    #[inline] pub fn test_trcpt0(&self) -> bool {
        self.trcpt0() != 0
    }

    #[doc="Sets the TRCPT0 field."]
    #[inline] pub fn set_trcpt0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete 1 Disable"]
    #[inline] pub fn trcpt1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRCPT1 != 0"]
    #[inline] pub fn test_trcpt1(&self) -> bool {
        self.trcpt1() != 0
    }

    #[doc="Sets the TRCPT1 field."]
    #[inline] pub fn set_trcpt1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error Flow Interrupt Disable"]
    #[inline] pub fn trfail(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TRFAIL != 0"]
    #[inline] pub fn test_trfail(&self) -> bool {
        self.trfail() != 0
    }

    #[doc="Sets the TRFAIL field."]
    #[inline] pub fn set_trfail<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pipe Error Interrupt Disable"]
    #[inline] pub fn perr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit Setup Interrupt Disable"]
    #[inline] pub fn txstp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXSTP != 0"]
    #[inline] pub fn test_txstp(&self) -> bool {
        self.txstp() != 0
    }

    #[doc="Sets the TXSTP field."]
    #[inline] pub fn set_txstp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Stall Inetrrupt Disable"]
    #[inline] pub fn stall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Pintenclr {
    #[inline]
    fn from(other: u8) -> Self {
         Pintenclr(other)
    }
}

impl ::core::fmt::Display for Pintenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pintenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trcpt0() != 0 { try!(write!(f, " trcpt0"))}
        if self.trcpt1() != 0 { try!(write!(f, " trcpt1"))}
        if self.trfail() != 0 { try!(write!(f, " trfail"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.txstp() != 0 { try!(write!(f, " txstp"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HOST Pipe Interrupt Flag Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pintenset(pub u8);
impl Pintenset {
    #[doc="Transfer Complete 0 Interrupt Enable"]
    #[inline] pub fn trcpt0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TRCPT0 != 0"]
    #[inline] pub fn test_trcpt0(&self) -> bool {
        self.trcpt0() != 0
    }

    #[doc="Sets the TRCPT0 field."]
    #[inline] pub fn set_trcpt0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transfer Complete 1 Interrupt Enable"]
    #[inline] pub fn trcpt1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TRCPT1 != 0"]
    #[inline] pub fn test_trcpt1(&self) -> bool {
        self.trcpt1() != 0
    }

    #[doc="Sets the TRCPT1 field."]
    #[inline] pub fn set_trcpt1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error Flow Interrupt Enable"]
    #[inline] pub fn trfail(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TRFAIL != 0"]
    #[inline] pub fn test_trfail(&self) -> bool {
        self.trfail() != 0
    }

    #[doc="Sets the TRFAIL field."]
    #[inline] pub fn set_trfail<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pipe Error Interrupt Enable"]
    #[inline] pub fn perr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PERR != 0"]
    #[inline] pub fn test_perr(&self) -> bool {
        self.perr() != 0
    }

    #[doc="Sets the PERR field."]
    #[inline] pub fn set_perr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit Setup Interrupt Enable"]
    #[inline] pub fn txstp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXSTP != 0"]
    #[inline] pub fn test_txstp(&self) -> bool {
        self.txstp() != 0
    }

    #[doc="Sets the TXSTP field."]
    #[inline] pub fn set_txstp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Stall Interrupt Enable"]
    #[inline] pub fn stall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Pintenset {
    #[inline]
    fn from(other: u8) -> Self {
         Pintenset(other)
    }
}

impl ::core::fmt::Display for Pintenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pintenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trcpt0() != 0 { try!(write!(f, " trcpt0"))}
        if self.trcpt1() != 0 { try!(write!(f, " trcpt1"))}
        if self.trfail() != 0 { try!(write!(f, " trfail"))}
        if self.perr() != 0 { try!(write!(f, " perr"))}
        if self.txstp() != 0 { try!(write!(f, " txstp"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of host

