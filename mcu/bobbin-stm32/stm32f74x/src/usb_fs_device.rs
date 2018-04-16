::bobbin_mcu::periph!( USB_FS_DEVICE, UsbFsDevice, USB_FS_DEVICE_PERIPH, UsbFsDevicePeriph, USB_FS_DEVICE_OWNED, USB_FS_DEVICE_REF_COUNT, 0x50000800, 0x00, 0x16);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("OTGFSRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for UsbFsDevice {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2rstr().otgfsrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_otgfsrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("OTGFSEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for UsbFsDevice {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2enr().otgfsen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_otgfsen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2LPENR"), field: Some("OTGFSLPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for UsbFsDevice {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2lpenr().otgfslpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2lpenr(|r| r.set_otgfslpen(value));
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USB_FS_DEVICE Peripheral"]
pub struct UsbFsDevicePeriph(pub usize); 

impl UsbFsDevicePeriph {
    #[doc="Get the DCFG Register."]
    #[inline] pub fn dcfg_reg(&self) -> ::bobbin_mcu::register::Register<Dcfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dcfg, 0x0)
    }

    #[doc="Get the *mut pointer for the DCFG register."]
    #[inline] pub fn dcfg_mut(&self) -> *mut Dcfg { 
        self.dcfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the DCFG register."]
    #[inline] pub fn dcfg_ptr(&self) -> *const Dcfg { 
        self.dcfg_reg().ptr()
    }

    #[doc="Read the DCFG register."]
    #[inline] pub fn dcfg(&self) -> Dcfg { 
        self.dcfg_reg().read()
    }

    #[doc="Write the DCFG register."]
    #[inline] pub fn write_dcfg(&self, value: Dcfg) -> &Self { 
        self.dcfg_reg().write(value);
        self
    }

    #[doc="Set the DCFG register."]
    #[inline] pub fn set_dcfg<F: FnOnce(Dcfg) -> Dcfg>(&self, f: F) -> &Self {
        self.dcfg_reg().set(f);
        self
    }

    #[doc="Modify the DCFG register."]
    #[inline] pub fn with_dcfg<F: FnOnce(Dcfg) -> Dcfg>(&self, f: F) -> &Self {
        self.dcfg_reg().with(f);
        self
    }

    #[doc="Get the DCTL Register."]
    #[inline] pub fn dctl_reg(&self) -> ::bobbin_mcu::register::Register<Dctl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dctl, 0x4)
    }

    #[doc="Get the *mut pointer for the DCTL register."]
    #[inline] pub fn dctl_mut(&self) -> *mut Dctl { 
        self.dctl_reg().ptr()
    }

    #[doc="Get the *const pointer for the DCTL register."]
    #[inline] pub fn dctl_ptr(&self) -> *const Dctl { 
        self.dctl_reg().ptr()
    }

    #[doc="Read the DCTL register."]
    #[inline] pub fn dctl(&self) -> Dctl { 
        self.dctl_reg().read()
    }

    #[doc="Write the DCTL register."]
    #[inline] pub fn write_dctl(&self, value: Dctl) -> &Self { 
        self.dctl_reg().write(value);
        self
    }

    #[doc="Set the DCTL register."]
    #[inline] pub fn set_dctl<F: FnOnce(Dctl) -> Dctl>(&self, f: F) -> &Self {
        self.dctl_reg().set(f);
        self
    }

    #[doc="Modify the DCTL register."]
    #[inline] pub fn with_dctl<F: FnOnce(Dctl) -> Dctl>(&self, f: F) -> &Self {
        self.dctl_reg().with(f);
        self
    }

    #[doc="Get the DSTS Register."]
    #[inline] pub fn dsts_reg(&self) -> ::bobbin_mcu::register::Register<Dsts> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dsts, 0x8)
    }

    #[doc="Get the *mut pointer for the DSTS register."]
    #[inline] pub fn dsts_mut(&self) -> *mut Dsts { 
        self.dsts_reg().ptr()
    }

    #[doc="Get the *const pointer for the DSTS register."]
    #[inline] pub fn dsts_ptr(&self) -> *const Dsts { 
        self.dsts_reg().ptr()
    }

    #[doc="Read the DSTS register."]
    #[inline] pub fn dsts(&self) -> Dsts { 
        self.dsts_reg().read()
    }

    #[doc="Get the DIEPMSK Register."]
    #[inline] pub fn diepmsk_reg(&self) -> ::bobbin_mcu::register::Register<Diepmsk> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Diepmsk, 0x10)
    }

    #[doc="Get the *mut pointer for the DIEPMSK register."]
    #[inline] pub fn diepmsk_mut(&self) -> *mut Diepmsk { 
        self.diepmsk_reg().ptr()
    }

    #[doc="Get the *const pointer for the DIEPMSK register."]
    #[inline] pub fn diepmsk_ptr(&self) -> *const Diepmsk { 
        self.diepmsk_reg().ptr()
    }

    #[doc="Read the DIEPMSK register."]
    #[inline] pub fn diepmsk(&self) -> Diepmsk { 
        self.diepmsk_reg().read()
    }

    #[doc="Write the DIEPMSK register."]
    #[inline] pub fn write_diepmsk(&self, value: Diepmsk) -> &Self { 
        self.diepmsk_reg().write(value);
        self
    }

    #[doc="Set the DIEPMSK register."]
    #[inline] pub fn set_diepmsk<F: FnOnce(Diepmsk) -> Diepmsk>(&self, f: F) -> &Self {
        self.diepmsk_reg().set(f);
        self
    }

    #[doc="Modify the DIEPMSK register."]
    #[inline] pub fn with_diepmsk<F: FnOnce(Diepmsk) -> Diepmsk>(&self, f: F) -> &Self {
        self.diepmsk_reg().with(f);
        self
    }

    #[doc="Get the DOEPMSK Register."]
    #[inline] pub fn doepmsk_reg(&self) -> ::bobbin_mcu::register::Register<Doepmsk> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Doepmsk, 0x14)
    }

    #[doc="Get the *mut pointer for the DOEPMSK register."]
    #[inline] pub fn doepmsk_mut(&self) -> *mut Doepmsk { 
        self.doepmsk_reg().ptr()
    }

    #[doc="Get the *const pointer for the DOEPMSK register."]
    #[inline] pub fn doepmsk_ptr(&self) -> *const Doepmsk { 
        self.doepmsk_reg().ptr()
    }

    #[doc="Read the DOEPMSK register."]
    #[inline] pub fn doepmsk(&self) -> Doepmsk { 
        self.doepmsk_reg().read()
    }

    #[doc="Write the DOEPMSK register."]
    #[inline] pub fn write_doepmsk(&self, value: Doepmsk) -> &Self { 
        self.doepmsk_reg().write(value);
        self
    }

    #[doc="Set the DOEPMSK register."]
    #[inline] pub fn set_doepmsk<F: FnOnce(Doepmsk) -> Doepmsk>(&self, f: F) -> &Self {
        self.doepmsk_reg().set(f);
        self
    }

    #[doc="Modify the DOEPMSK register."]
    #[inline] pub fn with_doepmsk<F: FnOnce(Doepmsk) -> Doepmsk>(&self, f: F) -> &Self {
        self.doepmsk_reg().with(f);
        self
    }

    #[doc="Get the DAINT Register."]
    #[inline] pub fn daint_reg(&self) -> ::bobbin_mcu::register::Register<Daint> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Daint, 0x18)
    }

    #[doc="Get the *mut pointer for the DAINT register."]
    #[inline] pub fn daint_mut(&self) -> *mut Daint { 
        self.daint_reg().ptr()
    }

    #[doc="Get the *const pointer for the DAINT register."]
    #[inline] pub fn daint_ptr(&self) -> *const Daint { 
        self.daint_reg().ptr()
    }

    #[doc="Read the DAINT register."]
    #[inline] pub fn daint(&self) -> Daint { 
        self.daint_reg().read()
    }

    #[doc="Get the DAINTMSK Register."]
    #[inline] pub fn daintmsk_reg(&self) -> ::bobbin_mcu::register::Register<Daintmsk> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Daintmsk, 0x1c)
    }

    #[doc="Get the *mut pointer for the DAINTMSK register."]
    #[inline] pub fn daintmsk_mut(&self) -> *mut Daintmsk { 
        self.daintmsk_reg().ptr()
    }

    #[doc="Get the *const pointer for the DAINTMSK register."]
    #[inline] pub fn daintmsk_ptr(&self) -> *const Daintmsk { 
        self.daintmsk_reg().ptr()
    }

    #[doc="Read the DAINTMSK register."]
    #[inline] pub fn daintmsk(&self) -> Daintmsk { 
        self.daintmsk_reg().read()
    }

    #[doc="Write the DAINTMSK register."]
    #[inline] pub fn write_daintmsk(&self, value: Daintmsk) -> &Self { 
        self.daintmsk_reg().write(value);
        self
    }

    #[doc="Set the DAINTMSK register."]
    #[inline] pub fn set_daintmsk<F: FnOnce(Daintmsk) -> Daintmsk>(&self, f: F) -> &Self {
        self.daintmsk_reg().set(f);
        self
    }

    #[doc="Modify the DAINTMSK register."]
    #[inline] pub fn with_daintmsk<F: FnOnce(Daintmsk) -> Daintmsk>(&self, f: F) -> &Self {
        self.daintmsk_reg().with(f);
        self
    }

    #[doc="Get the DVBUSDIS Register."]
    #[inline] pub fn dvbusdis_reg(&self) -> ::bobbin_mcu::register::Register<Dvbusdis> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dvbusdis, 0x28)
    }

    #[doc="Get the *mut pointer for the DVBUSDIS register."]
    #[inline] pub fn dvbusdis_mut(&self) -> *mut Dvbusdis { 
        self.dvbusdis_reg().ptr()
    }

    #[doc="Get the *const pointer for the DVBUSDIS register."]
    #[inline] pub fn dvbusdis_ptr(&self) -> *const Dvbusdis { 
        self.dvbusdis_reg().ptr()
    }

    #[doc="Read the DVBUSDIS register."]
    #[inline] pub fn dvbusdis(&self) -> Dvbusdis { 
        self.dvbusdis_reg().read()
    }

    #[doc="Write the DVBUSDIS register."]
    #[inline] pub fn write_dvbusdis(&self, value: Dvbusdis) -> &Self { 
        self.dvbusdis_reg().write(value);
        self
    }

    #[doc="Set the DVBUSDIS register."]
    #[inline] pub fn set_dvbusdis<F: FnOnce(Dvbusdis) -> Dvbusdis>(&self, f: F) -> &Self {
        self.dvbusdis_reg().set(f);
        self
    }

    #[doc="Modify the DVBUSDIS register."]
    #[inline] pub fn with_dvbusdis<F: FnOnce(Dvbusdis) -> Dvbusdis>(&self, f: F) -> &Self {
        self.dvbusdis_reg().with(f);
        self
    }

    #[doc="Get the DVBUSPULSE Register."]
    #[inline] pub fn dvbuspulse_reg(&self) -> ::bobbin_mcu::register::Register<Dvbuspulse> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dvbuspulse, 0x2c)
    }

    #[doc="Get the *mut pointer for the DVBUSPULSE register."]
    #[inline] pub fn dvbuspulse_mut(&self) -> *mut Dvbuspulse { 
        self.dvbuspulse_reg().ptr()
    }

    #[doc="Get the *const pointer for the DVBUSPULSE register."]
    #[inline] pub fn dvbuspulse_ptr(&self) -> *const Dvbuspulse { 
        self.dvbuspulse_reg().ptr()
    }

    #[doc="Read the DVBUSPULSE register."]
    #[inline] pub fn dvbuspulse(&self) -> Dvbuspulse { 
        self.dvbuspulse_reg().read()
    }

    #[doc="Write the DVBUSPULSE register."]
    #[inline] pub fn write_dvbuspulse(&self, value: Dvbuspulse) -> &Self { 
        self.dvbuspulse_reg().write(value);
        self
    }

    #[doc="Set the DVBUSPULSE register."]
    #[inline] pub fn set_dvbuspulse<F: FnOnce(Dvbuspulse) -> Dvbuspulse>(&self, f: F) -> &Self {
        self.dvbuspulse_reg().set(f);
        self
    }

    #[doc="Modify the DVBUSPULSE register."]
    #[inline] pub fn with_dvbuspulse<F: FnOnce(Dvbuspulse) -> Dvbuspulse>(&self, f: F) -> &Self {
        self.dvbuspulse_reg().with(f);
        self
    }

    #[doc="Get the DIEPEMPMSK Register."]
    #[inline] pub fn diepempmsk_reg(&self) -> ::bobbin_mcu::register::Register<Diepempmsk> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Diepempmsk, 0x34)
    }

    #[doc="Get the *mut pointer for the DIEPEMPMSK register."]
    #[inline] pub fn diepempmsk_mut(&self) -> *mut Diepempmsk { 
        self.diepempmsk_reg().ptr()
    }

    #[doc="Get the *const pointer for the DIEPEMPMSK register."]
    #[inline] pub fn diepempmsk_ptr(&self) -> *const Diepempmsk { 
        self.diepempmsk_reg().ptr()
    }

    #[doc="Read the DIEPEMPMSK register."]
    #[inline] pub fn diepempmsk(&self) -> Diepempmsk { 
        self.diepempmsk_reg().read()
    }

    #[doc="Write the DIEPEMPMSK register."]
    #[inline] pub fn write_diepempmsk(&self, value: Diepempmsk) -> &Self { 
        self.diepempmsk_reg().write(value);
        self
    }

    #[doc="Set the DIEPEMPMSK register."]
    #[inline] pub fn set_diepempmsk<F: FnOnce(Diepempmsk) -> Diepempmsk>(&self, f: F) -> &Self {
        self.diepempmsk_reg().set(f);
        self
    }

    #[doc="Modify the DIEPEMPMSK register."]
    #[inline] pub fn with_diepempmsk<F: FnOnce(Diepempmsk) -> Diepempmsk>(&self, f: F) -> &Self {
        self.diepempmsk_reg().with(f);
        self
    }

    #[doc="Get the DIEPCTL0 Register."]
    #[inline] pub fn diepctl0_reg(&self) -> ::bobbin_mcu::register::Register<Diepctl0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Diepctl0, 0x100)
    }

    #[doc="Get the *mut pointer for the DIEPCTL0 register."]
    #[inline] pub fn diepctl0_mut(&self) -> *mut Diepctl0 { 
        self.diepctl0_reg().ptr()
    }

    #[doc="Get the *const pointer for the DIEPCTL0 register."]
    #[inline] pub fn diepctl0_ptr(&self) -> *const Diepctl0 { 
        self.diepctl0_reg().ptr()
    }

    #[doc="Read the DIEPCTL0 register."]
    #[inline] pub fn diepctl0(&self) -> Diepctl0 { 
        self.diepctl0_reg().read()
    }

    #[doc="Write the DIEPCTL0 register."]
    #[inline] pub fn write_diepctl0(&self, value: Diepctl0) -> &Self { 
        self.diepctl0_reg().write(value);
        self
    }

    #[doc="Set the DIEPCTL0 register."]
    #[inline] pub fn set_diepctl0<F: FnOnce(Diepctl0) -> Diepctl0>(&self, f: F) -> &Self {
        self.diepctl0_reg().set(f);
        self
    }

    #[doc="Modify the DIEPCTL0 register."]
    #[inline] pub fn with_diepctl0<F: FnOnce(Diepctl0) -> Diepctl0>(&self, f: F) -> &Self {
        self.diepctl0_reg().with(f);
        self
    }

    #[doc="Get the DIEPCTL Register."]
    #[inline] pub fn diepctl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Diepctl, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Diepctl, 0x100, 0x20)
    }

    #[doc="Get the *mut pointer for the DIEPCTL register."]
    #[inline] pub fn diepctl_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Diepctl { 
        self.diepctl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DIEPCTL register."]
    #[inline] pub fn diepctl_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Diepctl { 
        self.diepctl_reg().ptr(index.into())
    }

    #[doc="Read the DIEPCTL register."]
    #[inline] pub fn diepctl<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Diepctl { 
        self.diepctl_reg().read(index.into())
    }

    #[doc="Write the DIEPCTL register."]
    #[inline] pub fn write_diepctl<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Diepctl) -> &Self {
        self.diepctl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DIEPCTL register."]
    #[inline] pub fn set_diepctl<I: Into<::bobbin_bits::R6>, F: FnOnce(Diepctl) -> Diepctl>(&self, index: I, f: F) -> &Self {
        self.diepctl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DIEPCTL register."]
    #[inline] pub fn with_diepctl<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Diepctl) -> Diepctl>(&self, index: I, f: F) -> &Self {
        self.diepctl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DOEPCTL0 Register."]
    #[inline] pub fn doepctl0_reg(&self) -> ::bobbin_mcu::register::Register<Doepctl0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Doepctl0, 0x300)
    }

    #[doc="Get the *mut pointer for the DOEPCTL0 register."]
    #[inline] pub fn doepctl0_mut(&self) -> *mut Doepctl0 { 
        self.doepctl0_reg().ptr()
    }

    #[doc="Get the *const pointer for the DOEPCTL0 register."]
    #[inline] pub fn doepctl0_ptr(&self) -> *const Doepctl0 { 
        self.doepctl0_reg().ptr()
    }

    #[doc="Read the DOEPCTL0 register."]
    #[inline] pub fn doepctl0(&self) -> Doepctl0 { 
        self.doepctl0_reg().read()
    }

    #[doc="Write the DOEPCTL0 register."]
    #[inline] pub fn write_doepctl0(&self, value: Doepctl0) -> &Self { 
        self.doepctl0_reg().write(value);
        self
    }

    #[doc="Set the DOEPCTL0 register."]
    #[inline] pub fn set_doepctl0<F: FnOnce(Doepctl0) -> Doepctl0>(&self, f: F) -> &Self {
        self.doepctl0_reg().set(f);
        self
    }

    #[doc="Modify the DOEPCTL0 register."]
    #[inline] pub fn with_doepctl0<F: FnOnce(Doepctl0) -> Doepctl0>(&self, f: F) -> &Self {
        self.doepctl0_reg().with(f);
        self
    }

    #[doc="Get the DOEPCTL Register."]
    #[inline] pub fn doepctl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Doepctl, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Doepctl, 0x300, 0x20)
    }

    #[doc="Get the *mut pointer for the DOEPCTL register."]
    #[inline] pub fn doepctl_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Doepctl { 
        self.doepctl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DOEPCTL register."]
    #[inline] pub fn doepctl_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Doepctl { 
        self.doepctl_reg().ptr(index.into())
    }

    #[doc="Read the DOEPCTL register."]
    #[inline] pub fn doepctl<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Doepctl { 
        self.doepctl_reg().read(index.into())
    }

    #[doc="Write the DOEPCTL register."]
    #[inline] pub fn write_doepctl<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Doepctl) -> &Self {
        self.doepctl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DOEPCTL register."]
    #[inline] pub fn set_doepctl<I: Into<::bobbin_bits::R6>, F: FnOnce(Doepctl) -> Doepctl>(&self, index: I, f: F) -> &Self {
        self.doepctl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DOEPCTL register."]
    #[inline] pub fn with_doepctl<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Doepctl) -> Doepctl>(&self, index: I, f: F) -> &Self {
        self.doepctl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DIEPINT Register."]
    #[inline] pub fn diepint_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Diepint, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Diepint, 0x108, 0x20)
    }

    #[doc="Get the *mut pointer for the DIEPINT register."]
    #[inline] pub fn diepint_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Diepint { 
        self.diepint_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DIEPINT register."]
    #[inline] pub fn diepint_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Diepint { 
        self.diepint_reg().ptr(index.into())
    }

    #[doc="Read the DIEPINT register."]
    #[inline] pub fn diepint<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Diepint { 
        self.diepint_reg().read(index.into())
    }

    #[doc="Write the DIEPINT register."]
    #[inline] pub fn write_diepint<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Diepint) -> &Self {
        self.diepint_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DIEPINT register."]
    #[inline] pub fn set_diepint<I: Into<::bobbin_bits::R6>, F: FnOnce(Diepint) -> Diepint>(&self, index: I, f: F) -> &Self {
        self.diepint_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DIEPINT register."]
    #[inline] pub fn with_diepint<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Diepint) -> Diepint>(&self, index: I, f: F) -> &Self {
        self.diepint_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DOEPINT Register."]
    #[inline] pub fn doepint_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Doepint, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Doepint, 0x308, 0x20)
    }

    #[doc="Get the *mut pointer for the DOEPINT register."]
    #[inline] pub fn doepint_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Doepint { 
        self.doepint_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DOEPINT register."]
    #[inline] pub fn doepint_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Doepint { 
        self.doepint_reg().ptr(index.into())
    }

    #[doc="Read the DOEPINT register."]
    #[inline] pub fn doepint<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Doepint { 
        self.doepint_reg().read(index.into())
    }

    #[doc="Write the DOEPINT register."]
    #[inline] pub fn write_doepint<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Doepint) -> &Self {
        self.doepint_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DOEPINT register."]
    #[inline] pub fn set_doepint<I: Into<::bobbin_bits::R6>, F: FnOnce(Doepint) -> Doepint>(&self, index: I, f: F) -> &Self {
        self.doepint_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DOEPINT register."]
    #[inline] pub fn with_doepint<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Doepint) -> Doepint>(&self, index: I, f: F) -> &Self {
        self.doepint_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DIEPTSIZ0 Register."]
    #[inline] pub fn dieptsiz0_reg(&self) -> ::bobbin_mcu::register::Register<Dieptsiz0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dieptsiz0, 0x110)
    }

    #[doc="Get the *mut pointer for the DIEPTSIZ0 register."]
    #[inline] pub fn dieptsiz0_mut(&self) -> *mut Dieptsiz0 { 
        self.dieptsiz0_reg().ptr()
    }

    #[doc="Get the *const pointer for the DIEPTSIZ0 register."]
    #[inline] pub fn dieptsiz0_ptr(&self) -> *const Dieptsiz0 { 
        self.dieptsiz0_reg().ptr()
    }

    #[doc="Read the DIEPTSIZ0 register."]
    #[inline] pub fn dieptsiz0(&self) -> Dieptsiz0 { 
        self.dieptsiz0_reg().read()
    }

    #[doc="Write the DIEPTSIZ0 register."]
    #[inline] pub fn write_dieptsiz0(&self, value: Dieptsiz0) -> &Self { 
        self.dieptsiz0_reg().write(value);
        self
    }

    #[doc="Set the DIEPTSIZ0 register."]
    #[inline] pub fn set_dieptsiz0<F: FnOnce(Dieptsiz0) -> Dieptsiz0>(&self, f: F) -> &Self {
        self.dieptsiz0_reg().set(f);
        self
    }

    #[doc="Modify the DIEPTSIZ0 register."]
    #[inline] pub fn with_dieptsiz0<F: FnOnce(Dieptsiz0) -> Dieptsiz0>(&self, f: F) -> &Self {
        self.dieptsiz0_reg().with(f);
        self
    }

    #[doc="Get the DOEPTSIZ0 Register."]
    #[inline] pub fn doeptsiz0_reg(&self) -> ::bobbin_mcu::register::Register<Doeptsiz0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Doeptsiz0, 0x310)
    }

    #[doc="Get the *mut pointer for the DOEPTSIZ0 register."]
    #[inline] pub fn doeptsiz0_mut(&self) -> *mut Doeptsiz0 { 
        self.doeptsiz0_reg().ptr()
    }

    #[doc="Get the *const pointer for the DOEPTSIZ0 register."]
    #[inline] pub fn doeptsiz0_ptr(&self) -> *const Doeptsiz0 { 
        self.doeptsiz0_reg().ptr()
    }

    #[doc="Read the DOEPTSIZ0 register."]
    #[inline] pub fn doeptsiz0(&self) -> Doeptsiz0 { 
        self.doeptsiz0_reg().read()
    }

    #[doc="Write the DOEPTSIZ0 register."]
    #[inline] pub fn write_doeptsiz0(&self, value: Doeptsiz0) -> &Self { 
        self.doeptsiz0_reg().write(value);
        self
    }

    #[doc="Set the DOEPTSIZ0 register."]
    #[inline] pub fn set_doeptsiz0<F: FnOnce(Doeptsiz0) -> Doeptsiz0>(&self, f: F) -> &Self {
        self.doeptsiz0_reg().set(f);
        self
    }

    #[doc="Modify the DOEPTSIZ0 register."]
    #[inline] pub fn with_doeptsiz0<F: FnOnce(Doeptsiz0) -> Doeptsiz0>(&self, f: F) -> &Self {
        self.doeptsiz0_reg().with(f);
        self
    }

    #[doc="Get the DIEPTSIZ Register."]
    #[inline] pub fn dieptsiz_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dieptsiz, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dieptsiz, 0x110, 0x20)
    }

    #[doc="Get the *mut pointer for the DIEPTSIZ register."]
    #[inline] pub fn dieptsiz_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Dieptsiz { 
        self.dieptsiz_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DIEPTSIZ register."]
    #[inline] pub fn dieptsiz_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Dieptsiz { 
        self.dieptsiz_reg().ptr(index.into())
    }

    #[doc="Read the DIEPTSIZ register."]
    #[inline] pub fn dieptsiz<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Dieptsiz { 
        self.dieptsiz_reg().read(index.into())
    }

    #[doc="Write the DIEPTSIZ register."]
    #[inline] pub fn write_dieptsiz<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Dieptsiz) -> &Self {
        self.dieptsiz_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DIEPTSIZ register."]
    #[inline] pub fn set_dieptsiz<I: Into<::bobbin_bits::R6>, F: FnOnce(Dieptsiz) -> Dieptsiz>(&self, index: I, f: F) -> &Self {
        self.dieptsiz_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DIEPTSIZ register."]
    #[inline] pub fn with_dieptsiz<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Dieptsiz) -> Dieptsiz>(&self, index: I, f: F) -> &Self {
        self.dieptsiz_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DTXFSTS Register."]
    #[inline] pub fn dtxfsts_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dtxfsts, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dtxfsts, 0x118, 0x20)
    }

    #[doc="Get the *mut pointer for the DTXFSTS register."]
    #[inline] pub fn dtxfsts_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Dtxfsts { 
        self.dtxfsts_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DTXFSTS register."]
    #[inline] pub fn dtxfsts_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Dtxfsts { 
        self.dtxfsts_reg().ptr(index.into())
    }

    #[doc="Read the DTXFSTS register."]
    #[inline] pub fn dtxfsts<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Dtxfsts { 
        self.dtxfsts_reg().read(index.into())
    }

    #[doc="Get the DOEPTSIZ Register."]
    #[inline] pub fn doeptsiz_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Doeptsiz, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Doeptsiz, 0x310, 0x20)
    }

    #[doc="Get the *mut pointer for the DOEPTSIZ register."]
    #[inline] pub fn doeptsiz_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Doeptsiz { 
        self.doeptsiz_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DOEPTSIZ register."]
    #[inline] pub fn doeptsiz_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Doeptsiz { 
        self.doeptsiz_reg().ptr(index.into())
    }

    #[doc="Read the DOEPTSIZ register."]
    #[inline] pub fn doeptsiz<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Doeptsiz { 
        self.doeptsiz_reg().read(index.into())
    }

    #[doc="Write the DOEPTSIZ register."]
    #[inline] pub fn write_doeptsiz<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Doeptsiz) -> &Self {
        self.doeptsiz_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DOEPTSIZ register."]
    #[inline] pub fn set_doeptsiz<I: Into<::bobbin_bits::R6>, F: FnOnce(Doeptsiz) -> Doeptsiz>(&self, index: I, f: F) -> &Self {
        self.doeptsiz_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DOEPTSIZ register."]
    #[inline] pub fn with_doeptsiz<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Doeptsiz) -> Doeptsiz>(&self, index: I, f: F) -> &Self {
        self.doeptsiz_reg().with(index.into(), f);
        self
    }

}

#[doc="OTG_FS device configuration register (OTG_FS_DCFG)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcfg(pub u32);
impl Dcfg {
    #[doc="Device speed"]
    #[inline] pub fn dspd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if DSPD != 0"]
    #[inline] pub fn test_dspd(&self) -> bool {
        self.dspd() != 0
    }

    #[doc="Sets the DSPD field."]
    #[inline] pub fn set_dspd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Non-zero-length status OUT handshake"]
    #[inline] pub fn nzlsohsk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NZLSOHSK != 0"]
    #[inline] pub fn test_nzlsohsk(&self) -> bool {
        self.nzlsohsk() != 0
    }

    #[doc="Sets the NZLSOHSK field."]
    #[inline] pub fn set_nzlsohsk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Device address"]
    #[inline] pub fn dad(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7f) as u8) } // [10:4]
    }

    #[doc="Returns true if DAD != 0"]
    #[inline] pub fn test_dad(&self) -> bool {
        self.dad() != 0
    }

    #[doc="Sets the DAD field."]
    #[inline] pub fn set_dad<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Periodic frame interval"]
    #[inline] pub fn pfivl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if PFIVL != 0"]
    #[inline] pub fn test_pfivl(&self) -> bool {
        self.pfivl() != 0
    }

    #[doc="Sets the PFIVL field."]
    #[inline] pub fn set_pfivl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Dcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Dcfg(other)
    }
}

impl ::core::fmt::Display for Dcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dspd() != 0 { try!(write!(f, " dspd=0x{:x}", self.dspd()))}
        if self.nzlsohsk() != 0 { try!(write!(f, " nzlsohsk"))}
        if self.dad() != 0 { try!(write!(f, " dad=0x{:x}", self.dad()))}
        if self.pfivl() != 0 { try!(write!(f, " pfivl=0x{:x}", self.pfivl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device control register (OTG_FS_DCTL)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dctl(pub u32);
impl Dctl {
    #[doc="Remote wakeup signaling"]
    #[inline] pub fn rwusig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RWUSIG != 0"]
    #[inline] pub fn test_rwusig(&self) -> bool {
        self.rwusig() != 0
    }

    #[doc="Sets the RWUSIG field."]
    #[inline] pub fn set_rwusig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Soft disconnect"]
    #[inline] pub fn sdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SDIS != 0"]
    #[inline] pub fn test_sdis(&self) -> bool {
        self.sdis() != 0
    }

    #[doc="Sets the SDIS field."]
    #[inline] pub fn set_sdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Global IN NAK status"]
    #[inline] pub fn ginsts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if GINSTS != 0"]
    #[inline] pub fn test_ginsts(&self) -> bool {
        self.ginsts() != 0
    }

    #[doc="Sets the GINSTS field."]
    #[inline] pub fn set_ginsts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Global OUT NAK status"]
    #[inline] pub fn gonsts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if GONSTS != 0"]
    #[inline] pub fn test_gonsts(&self) -> bool {
        self.gonsts() != 0
    }

    #[doc="Sets the GONSTS field."]
    #[inline] pub fn set_gonsts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Test control"]
    #[inline] pub fn tctl(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if TCTL != 0"]
    #[inline] pub fn test_tctl(&self) -> bool {
        self.tctl() != 0
    }

    #[doc="Sets the TCTL field."]
    #[inline] pub fn set_tctl<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Set global IN NAK"]
    #[inline] pub fn sginak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SGINAK != 0"]
    #[inline] pub fn test_sginak(&self) -> bool {
        self.sginak() != 0
    }

    #[doc="Sets the SGINAK field."]
    #[inline] pub fn set_sginak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Clear global IN NAK"]
    #[inline] pub fn cginak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CGINAK != 0"]
    #[inline] pub fn test_cginak(&self) -> bool {
        self.cginak() != 0
    }

    #[doc="Sets the CGINAK field."]
    #[inline] pub fn set_cginak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Set global OUT NAK"]
    #[inline] pub fn sgonak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SGONAK != 0"]
    #[inline] pub fn test_sgonak(&self) -> bool {
        self.sgonak() != 0
    }

    #[doc="Sets the SGONAK field."]
    #[inline] pub fn set_sgonak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Clear global OUT NAK"]
    #[inline] pub fn cgonak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CGONAK != 0"]
    #[inline] pub fn test_cgonak(&self) -> bool {
        self.cgonak() != 0
    }

    #[doc="Sets the CGONAK field."]
    #[inline] pub fn set_cgonak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Power-on programming done"]
    #[inline] pub fn poprgdne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if POPRGDNE != 0"]
    #[inline] pub fn test_poprgdne(&self) -> bool {
        self.poprgdne() != 0
    }

    #[doc="Sets the POPRGDNE field."]
    #[inline] pub fn set_poprgdne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Dctl {
    #[inline]
    fn from(other: u32) -> Self {
         Dctl(other)
    }
}

impl ::core::fmt::Display for Dctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rwusig() != 0 { try!(write!(f, " rwusig"))}
        if self.sdis() != 0 { try!(write!(f, " sdis"))}
        if self.ginsts() != 0 { try!(write!(f, " ginsts"))}
        if self.gonsts() != 0 { try!(write!(f, " gonsts"))}
        if self.tctl() != 0 { try!(write!(f, " tctl=0x{:x}", self.tctl()))}
        if self.sginak() != 0 { try!(write!(f, " sginak"))}
        if self.cginak() != 0 { try!(write!(f, " cginak"))}
        if self.sgonak() != 0 { try!(write!(f, " sgonak"))}
        if self.cgonak() != 0 { try!(write!(f, " cgonak"))}
        if self.poprgdne() != 0 { try!(write!(f, " poprgdne"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device status register (OTG_FS_DSTS)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dsts(pub u32);
impl Dsts {
    #[doc="Suspend status"]
    #[inline] pub fn suspsts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SUSPSTS != 0"]
    #[inline] pub fn test_suspsts(&self) -> bool {
        self.suspsts() != 0
    }

    #[doc="Sets the SUSPSTS field."]
    #[inline] pub fn set_suspsts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enumerated speed"]
    #[inline] pub fn enumspd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if ENUMSPD != 0"]
    #[inline] pub fn test_enumspd(&self) -> bool {
        self.enumspd() != 0
    }

    #[doc="Sets the ENUMSPD field."]
    #[inline] pub fn set_enumspd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Erratic error"]
    #[inline] pub fn eerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EERR != 0"]
    #[inline] pub fn test_eerr(&self) -> bool {
        self.eerr() != 0
    }

    #[doc="Sets the EERR field."]
    #[inline] pub fn set_eerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Frame number of the received SOF"]
    #[inline] pub fn fnsof(&self) -> ::bobbin_bits::U14 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3fff) as u16) } // [21:8]
    }

    #[doc="Returns true if FNSOF != 0"]
    #[inline] pub fn test_fnsof(&self) -> bool {
        self.fnsof() != 0
    }

    #[doc="Sets the FNSOF field."]
    #[inline] pub fn set_fnsof<V: Into<::bobbin_bits::U14>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U14 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3fff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Dsts {
    #[inline]
    fn from(other: u32) -> Self {
         Dsts(other)
    }
}

impl ::core::fmt::Display for Dsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.suspsts() != 0 { try!(write!(f, " suspsts"))}
        if self.enumspd() != 0 { try!(write!(f, " enumspd=0x{:x}", self.enumspd()))}
        if self.eerr() != 0 { try!(write!(f, " eerr"))}
        if self.fnsof() != 0 { try!(write!(f, " fnsof=0x{:x}", self.fnsof()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepmsk(pub u32);
impl Diepmsk {
    #[doc="Transfer completed interrupt mask"]
    #[inline] pub fn xfrcm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint disabled interrupt mask"]
    #[inline] pub fn epdm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDM != 0"]
    #[inline] pub fn test_epdm(&self) -> bool {
        self.epdm() != 0
    }

    #[doc="Sets the EPDM field."]
    #[inline] pub fn set_epdm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Timeout condition mask (Non-isochronous endpoints)"]
    #[inline] pub fn tom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TOM != 0"]
    #[inline] pub fn test_tom(&self) -> bool {
        self.tom() != 0
    }

    #[doc="Sets the TOM field."]
    #[inline] pub fn set_tom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IN token received when TxFIFO empty mask"]
    #[inline] pub fn ittxfemsk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ITTXFEMSK != 0"]
    #[inline] pub fn test_ittxfemsk(&self) -> bool {
        self.ittxfemsk() != 0
    }

    #[doc="Sets the ITTXFEMSK field."]
    #[inline] pub fn set_ittxfemsk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="IN token received with EP mismatch mask"]
    #[inline] pub fn inepnmm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if INEPNMM != 0"]
    #[inline] pub fn test_inepnmm(&self) -> bool {
        self.inepnmm() != 0
    }

    #[doc="Sets the INEPNMM field."]
    #[inline] pub fn set_inepnmm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IN endpoint NAK effective mask"]
    #[inline] pub fn inepnem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INEPNEM != 0"]
    #[inline] pub fn test_inepnem(&self) -> bool {
        self.inepnem() != 0
    }

    #[doc="Sets the INEPNEM field."]
    #[inline] pub fn set_inepnem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Diepmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Diepmsk(other)
    }
}

impl ::core::fmt::Display for Diepmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.epdm() != 0 { try!(write!(f, " epdm"))}
        if self.tom() != 0 { try!(write!(f, " tom"))}
        if self.ittxfemsk() != 0 { try!(write!(f, " ittxfemsk"))}
        if self.inepnmm() != 0 { try!(write!(f, " inepnmm"))}
        if self.inepnem() != 0 { try!(write!(f, " inepnem"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepmsk(pub u32);
impl Doepmsk {
    #[doc="Transfer completed interrupt mask"]
    #[inline] pub fn xfrcm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRCM != 0"]
    #[inline] pub fn test_xfrcm(&self) -> bool {
        self.xfrcm() != 0
    }

    #[doc="Sets the XFRCM field."]
    #[inline] pub fn set_xfrcm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint disabled interrupt mask"]
    #[inline] pub fn epdm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDM != 0"]
    #[inline] pub fn test_epdm(&self) -> bool {
        self.epdm() != 0
    }

    #[doc="Sets the EPDM field."]
    #[inline] pub fn set_epdm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SETUP phase done mask"]
    #[inline] pub fn stupm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STUPM != 0"]
    #[inline] pub fn test_stupm(&self) -> bool {
        self.stupm() != 0
    }

    #[doc="Sets the STUPM field."]
    #[inline] pub fn set_stupm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="OUT token received when endpoint disabled mask"]
    #[inline] pub fn otepdm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OTEPDM != 0"]
    #[inline] pub fn test_otepdm(&self) -> bool {
        self.otepdm() != 0
    }

    #[doc="Sets the OTEPDM field."]
    #[inline] pub fn set_otepdm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Doepmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Doepmsk(other)
    }
}

impl ::core::fmt::Display for Doepmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xfrcm() != 0 { try!(write!(f, " xfrcm"))}
        if self.epdm() != 0 { try!(write!(f, " epdm"))}
        if self.stupm() != 0 { try!(write!(f, " stupm"))}
        if self.otepdm() != 0 { try!(write!(f, " otepdm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Daint(pub u32);
impl Daint {
    #[doc="IN endpoint interrupt bits"]
    #[inline] pub fn iepint<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IEPINT != 0"]
    #[inline] pub fn test_iepint<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.iepint(index) != 0
    }

    #[doc="Sets the IEPINT field."]
    #[inline] pub fn set_iepint<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="OUT endpoint interrupt bits"]
    #[inline] pub fn oepint<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if OEPINT != 0"]
    #[inline] pub fn test_oepint<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.oepint(index) != 0
    }

    #[doc="Sets the OEPINT field."]
    #[inline] pub fn set_oepint<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Daint {
    #[inline]
    fn from(other: u32) -> Self {
         Daint(other)
    }
}

impl ::core::fmt::Display for Daint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Daint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iepint(0) != 0 { try!(write!(f, " iepint[0]"))}
        if self.iepint(1) != 0 { try!(write!(f, " iepint[1]"))}
        if self.iepint(2) != 0 { try!(write!(f, " iepint[2]"))}
        if self.iepint(3) != 0 { try!(write!(f, " iepint[3]"))}
        if self.iepint(4) != 0 { try!(write!(f, " iepint[4]"))}
        if self.iepint(5) != 0 { try!(write!(f, " iepint[5]"))}
        if self.iepint(6) != 0 { try!(write!(f, " iepint[6]"))}
        if self.iepint(7) != 0 { try!(write!(f, " iepint[7]"))}
        if self.iepint(8) != 0 { try!(write!(f, " iepint[8]"))}
        if self.iepint(9) != 0 { try!(write!(f, " iepint[9]"))}
        if self.iepint(10) != 0 { try!(write!(f, " iepint[10]"))}
        if self.iepint(11) != 0 { try!(write!(f, " iepint[11]"))}
        if self.iepint(12) != 0 { try!(write!(f, " iepint[12]"))}
        if self.iepint(13) != 0 { try!(write!(f, " iepint[13]"))}
        if self.iepint(14) != 0 { try!(write!(f, " iepint[14]"))}
        if self.iepint(15) != 0 { try!(write!(f, " iepint[15]"))}
        if self.oepint(0) != 0 { try!(write!(f, " oepint[0]"))}
        if self.oepint(1) != 0 { try!(write!(f, " oepint[1]"))}
        if self.oepint(2) != 0 { try!(write!(f, " oepint[2]"))}
        if self.oepint(3) != 0 { try!(write!(f, " oepint[3]"))}
        if self.oepint(4) != 0 { try!(write!(f, " oepint[4]"))}
        if self.oepint(5) != 0 { try!(write!(f, " oepint[5]"))}
        if self.oepint(6) != 0 { try!(write!(f, " oepint[6]"))}
        if self.oepint(7) != 0 { try!(write!(f, " oepint[7]"))}
        if self.oepint(8) != 0 { try!(write!(f, " oepint[8]"))}
        if self.oepint(9) != 0 { try!(write!(f, " oepint[9]"))}
        if self.oepint(10) != 0 { try!(write!(f, " oepint[10]"))}
        if self.oepint(11) != 0 { try!(write!(f, " oepint[11]"))}
        if self.oepint(12) != 0 { try!(write!(f, " oepint[12]"))}
        if self.oepint(13) != 0 { try!(write!(f, " oepint[13]"))}
        if self.oepint(14) != 0 { try!(write!(f, " oepint[14]"))}
        if self.oepint(15) != 0 { try!(write!(f, " oepint[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Daintmsk(pub u32);
impl Daintmsk {
    #[doc="IN EP interrupt mask bits"]
    #[inline] pub fn iepm<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IEPM != 0"]
    #[inline] pub fn test_iepm<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.iepm(index) != 0
    }

    #[doc="Sets the IEPM field."]
    #[inline] pub fn set_iepm<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="OUT EP interrupt mask bits"]
    #[inline] pub fn oepm<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if OEPM != 0"]
    #[inline] pub fn test_oepm<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.oepm(index) != 0
    }

    #[doc="Sets the OEPM field."]
    #[inline] pub fn set_oepm<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Daintmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Daintmsk(other)
    }
}

impl ::core::fmt::Display for Daintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Daintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.iepm(0) != 0 { try!(write!(f, " iepm[0]"))}
        if self.iepm(1) != 0 { try!(write!(f, " iepm[1]"))}
        if self.iepm(2) != 0 { try!(write!(f, " iepm[2]"))}
        if self.iepm(3) != 0 { try!(write!(f, " iepm[3]"))}
        if self.iepm(4) != 0 { try!(write!(f, " iepm[4]"))}
        if self.iepm(5) != 0 { try!(write!(f, " iepm[5]"))}
        if self.iepm(6) != 0 { try!(write!(f, " iepm[6]"))}
        if self.iepm(7) != 0 { try!(write!(f, " iepm[7]"))}
        if self.iepm(8) != 0 { try!(write!(f, " iepm[8]"))}
        if self.iepm(9) != 0 { try!(write!(f, " iepm[9]"))}
        if self.iepm(10) != 0 { try!(write!(f, " iepm[10]"))}
        if self.iepm(11) != 0 { try!(write!(f, " iepm[11]"))}
        if self.iepm(12) != 0 { try!(write!(f, " iepm[12]"))}
        if self.iepm(13) != 0 { try!(write!(f, " iepm[13]"))}
        if self.iepm(14) != 0 { try!(write!(f, " iepm[14]"))}
        if self.iepm(15) != 0 { try!(write!(f, " iepm[15]"))}
        if self.oepm(0) != 0 { try!(write!(f, " oepm[0]"))}
        if self.oepm(1) != 0 { try!(write!(f, " oepm[1]"))}
        if self.oepm(2) != 0 { try!(write!(f, " oepm[2]"))}
        if self.oepm(3) != 0 { try!(write!(f, " oepm[3]"))}
        if self.oepm(4) != 0 { try!(write!(f, " oepm[4]"))}
        if self.oepm(5) != 0 { try!(write!(f, " oepm[5]"))}
        if self.oepm(6) != 0 { try!(write!(f, " oepm[6]"))}
        if self.oepm(7) != 0 { try!(write!(f, " oepm[7]"))}
        if self.oepm(8) != 0 { try!(write!(f, " oepm[8]"))}
        if self.oepm(9) != 0 { try!(write!(f, " oepm[9]"))}
        if self.oepm(10) != 0 { try!(write!(f, " oepm[10]"))}
        if self.oepm(11) != 0 { try!(write!(f, " oepm[11]"))}
        if self.oepm(12) != 0 { try!(write!(f, " oepm[12]"))}
        if self.oepm(13) != 0 { try!(write!(f, " oepm[13]"))}
        if self.oepm(14) != 0 { try!(write!(f, " oepm[14]"))}
        if self.oepm(15) != 0 { try!(write!(f, " oepm[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device VBUS discharge time register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dvbusdis(pub u32);
impl Dvbusdis {
    #[doc="Device VBUS discharge time"]
    #[inline] pub fn vbusdt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if VBUSDT != 0"]
    #[inline] pub fn test_vbusdt(&self) -> bool {
        self.vbusdt() != 0
    }

    #[doc="Sets the VBUSDT field."]
    #[inline] pub fn set_vbusdt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dvbusdis {
    #[inline]
    fn from(other: u32) -> Self {
         Dvbusdis(other)
    }
}

impl ::core::fmt::Display for Dvbusdis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dvbusdis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vbusdt() != 0 { try!(write!(f, " vbusdt=0x{:x}", self.vbusdt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device VBUS pulsing time register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dvbuspulse(pub u32);
impl Dvbuspulse {
    #[doc="Device VBUS pulsing time"]
    #[inline] pub fn dvbusp(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DVBUSP != 0"]
    #[inline] pub fn test_dvbusp(&self) -> bool {
        self.dvbusp() != 0
    }

    #[doc="Sets the DVBUSP field."]
    #[inline] pub fn set_dvbusp<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dvbuspulse {
    #[inline]
    fn from(other: u32) -> Self {
         Dvbuspulse(other)
    }
}

impl ::core::fmt::Display for Dvbuspulse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dvbuspulse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dvbusp() != 0 { try!(write!(f, " dvbusp=0x{:x}", self.dvbusp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint FIFO empty interrupt mask register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepempmsk(pub u32);
impl Diepempmsk {
    #[doc="IN EP Tx FIFO empty interrupt mask bits"]
    #[inline] pub fn ineptxfem<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INEPTXFEM != 0"]
    #[inline] pub fn test_ineptxfem<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.ineptxfem(index) != 0
    }

    #[doc="Sets the INEPTXFEM field."]
    #[inline] pub fn set_ineptxfem<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Diepempmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Diepempmsk(other)
    }
}

impl ::core::fmt::Display for Diepempmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepempmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ineptxfem(0) != 0 { try!(write!(f, " ineptxfem[0]"))}
        if self.ineptxfem(1) != 0 { try!(write!(f, " ineptxfem[1]"))}
        if self.ineptxfem(2) != 0 { try!(write!(f, " ineptxfem[2]"))}
        if self.ineptxfem(3) != 0 { try!(write!(f, " ineptxfem[3]"))}
        if self.ineptxfem(4) != 0 { try!(write!(f, " ineptxfem[4]"))}
        if self.ineptxfem(5) != 0 { try!(write!(f, " ineptxfem[5]"))}
        if self.ineptxfem(6) != 0 { try!(write!(f, " ineptxfem[6]"))}
        if self.ineptxfem(7) != 0 { try!(write!(f, " ineptxfem[7]"))}
        if self.ineptxfem(8) != 0 { try!(write!(f, " ineptxfem[8]"))}
        if self.ineptxfem(9) != 0 { try!(write!(f, " ineptxfem[9]"))}
        if self.ineptxfem(10) != 0 { try!(write!(f, " ineptxfem[10]"))}
        if self.ineptxfem(11) != 0 { try!(write!(f, " ineptxfem[11]"))}
        if self.ineptxfem(12) != 0 { try!(write!(f, " ineptxfem[12]"))}
        if self.ineptxfem(13) != 0 { try!(write!(f, " ineptxfem[13]"))}
        if self.ineptxfem(14) != 0 { try!(write!(f, " ineptxfem[14]"))}
        if self.ineptxfem(15) != 0 { try!(write!(f, " ineptxfem[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepctl0(pub u32);
impl Diepctl0 {
    #[doc="Maximum packet size"]
    #[inline] pub fn mpsiz(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="USB active endpoint"]
    #[inline] pub fn usbaep(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="NAK status"]
    #[inline] pub fn naksts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Endpoint type"]
    #[inline] pub fn eptyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="STALL handshake"]
    #[inline] pub fn stall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if STALL != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the STALL field."]
    #[inline] pub fn set_stall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="TxFIFO number"]
    #[inline] pub fn txfnum(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="Returns true if TXFNUM != 0"]
    #[inline] pub fn test_txfnum(&self) -> bool {
        self.txfnum() != 0
    }

    #[doc="Sets the TXFNUM field."]
    #[inline] pub fn set_txfnum<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Clear NAK"]
    #[inline] pub fn cnak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Set NAK"]
    #[inline] pub fn snak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Endpoint disable"]
    #[inline] pub fn epdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Endpoint enable"]
    #[inline] pub fn epena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Diepctl0 {
    #[inline]
    fn from(other: u32) -> Self {
         Diepctl0(other)
    }
}

impl ::core::fmt::Display for Diepctl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepctl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.txfnum() != 0 { try!(write!(f, " txfnum=0x{:x}", self.txfnum()))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.epena() != 0 { try!(write!(f, " epena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG device endpoint-n control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepctl(pub u32);
impl Diepctl {
    #[doc="EPENA"]
    #[inline] pub fn epena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="EPDIS"]
    #[inline] pub fn epdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SODDFRM/SD1PID"]
    #[inline] pub fn soddfrm_sd1pid(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if SODDFRM_SD1PID != 0"]
    #[inline] pub fn test_soddfrm_sd1pid(&self) -> bool {
        self.soddfrm_sd1pid() != 0
    }

    #[doc="Sets the SODDFRM_SD1PID field."]
    #[inline] pub fn set_soddfrm_sd1pid<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="SD0PID/SEVNFRM"]
    #[inline] pub fn sd0pid_sevnfrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SD0PID_SEVNFRM != 0"]
    #[inline] pub fn test_sd0pid_sevnfrm(&self) -> bool {
        self.sd0pid_sevnfrm() != 0
    }

    #[doc="Sets the SD0PID_SEVNFRM field."]
    #[inline] pub fn set_sd0pid_sevnfrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SNAK"]
    #[inline] pub fn snak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="CNAK"]
    #[inline] pub fn cnak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="TXFNUM"]
    #[inline] pub fn txfnum(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
    }

    #[doc="Returns true if TXFNUM != 0"]
    #[inline] pub fn test_txfnum(&self) -> bool {
        self.txfnum() != 0
    }

    #[doc="Sets the TXFNUM field."]
    #[inline] pub fn set_txfnum<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Stall"]
    #[inline] pub fn stall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if Stall != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the Stall field."]
    #[inline] pub fn set_stall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EPTYP"]
    #[inline] pub fn eptyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="NAKSTS"]
    #[inline] pub fn naksts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="EONUM/DPID"]
    #[inline] pub fn eonum_dpid(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EONUM_DPID != 0"]
    #[inline] pub fn test_eonum_dpid(&self) -> bool {
        self.eonum_dpid() != 0
    }

    #[doc="Sets the EONUM_DPID field."]
    #[inline] pub fn set_eonum_dpid<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USBAEP"]
    #[inline] pub fn usbaep(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MPSIZ"]
    #[inline] pub fn mpsiz(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Diepctl {
    #[inline]
    fn from(other: u32) -> Self {
         Diepctl(other)
    }
}

impl ::core::fmt::Display for Diepctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epena() != 0 { try!(write!(f, " epena"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.soddfrm_sd1pid() != 0 { try!(write!(f, " soddfrm_sd1pid"))}
        if self.sd0pid_sevnfrm() != 0 { try!(write!(f, " sd0pid_sevnfrm"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.txfnum() != 0 { try!(write!(f, " txfnum=0x{:x}", self.txfnum()))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.eonum_dpid() != 0 { try!(write!(f, " eonum_dpid"))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-0 control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepctl0(pub u32);
impl Doepctl0 {
    #[doc="EPENA"]
    #[inline] pub fn epena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="EPDIS"]
    #[inline] pub fn epdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SNAK"]
    #[inline] pub fn snak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="CNAK"]
    #[inline] pub fn cnak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Stall"]
    #[inline] pub fn stall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if Stall != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the Stall field."]
    #[inline] pub fn set_stall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SNPM"]
    #[inline] pub fn snpm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SNPM != 0"]
    #[inline] pub fn test_snpm(&self) -> bool {
        self.snpm() != 0
    }

    #[doc="Sets the SNPM field."]
    #[inline] pub fn set_snpm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="EPTYP"]
    #[inline] pub fn eptyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="NAKSTS"]
    #[inline] pub fn naksts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="USBAEP"]
    #[inline] pub fn usbaep(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MPSIZ"]
    #[inline] pub fn mpsiz(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doepctl0 {
    #[inline]
    fn from(other: u32) -> Self {
         Doepctl0(other)
    }
}

impl ::core::fmt::Display for Doepctl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepctl0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epena() != 0 { try!(write!(f, " epena"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.snpm() != 0 { try!(write!(f, " snpm"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-n control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepctl(pub u32);
impl Doepctl {
    #[doc="EPENA"]
    #[inline] pub fn epena(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EPENA != 0"]
    #[inline] pub fn test_epena(&self) -> bool {
        self.epena() != 0
    }

    #[doc="Sets the EPENA field."]
    #[inline] pub fn set_epena<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="EPDIS"]
    #[inline] pub fn epdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EPDIS != 0"]
    #[inline] pub fn test_epdis(&self) -> bool {
        self.epdis() != 0
    }

    #[doc="Sets the EPDIS field."]
    #[inline] pub fn set_epdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SODDFRM"]
    #[inline] pub fn soddfrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if SODDFRM != 0"]
    #[inline] pub fn test_soddfrm(&self) -> bool {
        self.soddfrm() != 0
    }

    #[doc="Sets the SODDFRM field."]
    #[inline] pub fn set_soddfrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="SD0PID/SEVNFRM"]
    #[inline] pub fn sd0pid_sevnfrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SD0PID_SEVNFRM != 0"]
    #[inline] pub fn test_sd0pid_sevnfrm(&self) -> bool {
        self.sd0pid_sevnfrm() != 0
    }

    #[doc="Sets the SD0PID_SEVNFRM field."]
    #[inline] pub fn set_sd0pid_sevnfrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="SNAK"]
    #[inline] pub fn snak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SNAK != 0"]
    #[inline] pub fn test_snak(&self) -> bool {
        self.snak() != 0
    }

    #[doc="Sets the SNAK field."]
    #[inline] pub fn set_snak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="CNAK"]
    #[inline] pub fn cnak(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if CNAK != 0"]
    #[inline] pub fn test_cnak(&self) -> bool {
        self.cnak() != 0
    }

    #[doc="Sets the CNAK field."]
    #[inline] pub fn set_cnak<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Stall"]
    #[inline] pub fn stall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if Stall != 0"]
    #[inline] pub fn test_stall(&self) -> bool {
        self.stall() != 0
    }

    #[doc="Sets the Stall field."]
    #[inline] pub fn set_stall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="SNPM"]
    #[inline] pub fn snpm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SNPM != 0"]
    #[inline] pub fn test_snpm(&self) -> bool {
        self.snpm() != 0
    }

    #[doc="Sets the SNPM field."]
    #[inline] pub fn set_snpm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="EPTYP"]
    #[inline] pub fn eptyp(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if EPTYP != 0"]
    #[inline] pub fn test_eptyp(&self) -> bool {
        self.eptyp() != 0
    }

    #[doc="Sets the EPTYP field."]
    #[inline] pub fn set_eptyp<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="NAKSTS"]
    #[inline] pub fn naksts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if NAKSTS != 0"]
    #[inline] pub fn test_naksts(&self) -> bool {
        self.naksts() != 0
    }

    #[doc="Sets the NAKSTS field."]
    #[inline] pub fn set_naksts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="EONUM/DPID"]
    #[inline] pub fn eonum_dpid(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EONUM_DPID != 0"]
    #[inline] pub fn test_eonum_dpid(&self) -> bool {
        self.eonum_dpid() != 0
    }

    #[doc="Sets the EONUM_DPID field."]
    #[inline] pub fn set_eonum_dpid<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USBAEP"]
    #[inline] pub fn usbaep(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if USBAEP != 0"]
    #[inline] pub fn test_usbaep(&self) -> bool {
        self.usbaep() != 0
    }

    #[doc="Sets the USBAEP field."]
    #[inline] pub fn set_usbaep<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="MPSIZ"]
    #[inline] pub fn mpsiz(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if MPSIZ != 0"]
    #[inline] pub fn test_mpsiz(&self) -> bool {
        self.mpsiz() != 0
    }

    #[doc="Sets the MPSIZ field."]
    #[inline] pub fn set_mpsiz<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doepctl {
    #[inline]
    fn from(other: u32) -> Self {
         Doepctl(other)
    }
}

impl ::core::fmt::Display for Doepctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epena() != 0 { try!(write!(f, " epena"))}
        if self.epdis() != 0 { try!(write!(f, " epdis"))}
        if self.soddfrm() != 0 { try!(write!(f, " soddfrm"))}
        if self.sd0pid_sevnfrm() != 0 { try!(write!(f, " sd0pid_sevnfrm"))}
        if self.snak() != 0 { try!(write!(f, " snak"))}
        if self.cnak() != 0 { try!(write!(f, " cnak"))}
        if self.stall() != 0 { try!(write!(f, " stall"))}
        if self.snpm() != 0 { try!(write!(f, " snpm"))}
        if self.eptyp() != 0 { try!(write!(f, " eptyp=0x{:x}", self.eptyp()))}
        if self.naksts() != 0 { try!(write!(f, " naksts"))}
        if self.eonum_dpid() != 0 { try!(write!(f, " eonum_dpid"))}
        if self.usbaep() != 0 { try!(write!(f, " usbaep"))}
        if self.mpsiz() != 0 { try!(write!(f, " mpsiz=0x{:x}", self.mpsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-x interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Diepint(pub u32);
impl Diepint {
    #[doc="TXFE"]
    #[inline] pub fn txfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFE != 0"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Sets the TXFE field."]
    #[inline] pub fn set_txfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="INEPNE"]
    #[inline] pub fn inepne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INEPNE != 0"]
    #[inline] pub fn test_inepne(&self) -> bool {
        self.inepne() != 0
    }

    #[doc="Sets the INEPNE field."]
    #[inline] pub fn set_inepne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="ITTXFE"]
    #[inline] pub fn ittxfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ITTXFE != 0"]
    #[inline] pub fn test_ittxfe(&self) -> bool {
        self.ittxfe() != 0
    }

    #[doc="Sets the ITTXFE field."]
    #[inline] pub fn set_ittxfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TOC"]
    #[inline] pub fn toc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TOC != 0"]
    #[inline] pub fn test_toc(&self) -> bool {
        self.toc() != 0
    }

    #[doc="Sets the TOC field."]
    #[inline] pub fn set_toc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EPDISD"]
    #[inline] pub fn epdisd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDISD != 0"]
    #[inline] pub fn test_epdisd(&self) -> bool {
        self.epdisd() != 0
    }

    #[doc="Sets the EPDISD field."]
    #[inline] pub fn set_epdisd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XFRC"]
    #[inline] pub fn xfrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Diepint {
    #[inline]
    fn from(other: u32) -> Self {
         Diepint(other)
    }
}

impl ::core::fmt::Display for Diepint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Diepint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.inepne() != 0 { try!(write!(f, " inepne"))}
        if self.ittxfe() != 0 { try!(write!(f, " ittxfe"))}
        if self.toc() != 0 { try!(write!(f, " toc"))}
        if self.epdisd() != 0 { try!(write!(f, " epdisd"))}
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-n interrupt register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doepint(pub u32);
impl Doepint {
    #[doc="B2BSTUP"]
    #[inline] pub fn b2bstup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if B2BSTUP != 0"]
    #[inline] pub fn test_b2bstup(&self) -> bool {
        self.b2bstup() != 0
    }

    #[doc="Sets the B2BSTUP field."]
    #[inline] pub fn set_b2bstup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="OTEPDIS"]
    #[inline] pub fn otepdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OTEPDIS != 0"]
    #[inline] pub fn test_otepdis(&self) -> bool {
        self.otepdis() != 0
    }

    #[doc="Sets the OTEPDIS field."]
    #[inline] pub fn set_otepdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="STUP"]
    #[inline] pub fn stup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STUP != 0"]
    #[inline] pub fn test_stup(&self) -> bool {
        self.stup() != 0
    }

    #[doc="Sets the STUP field."]
    #[inline] pub fn set_stup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="EPDISD"]
    #[inline] pub fn epdisd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPDISD != 0"]
    #[inline] pub fn test_epdisd(&self) -> bool {
        self.epdisd() != 0
    }

    #[doc="Sets the EPDISD field."]
    #[inline] pub fn set_epdisd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XFRC"]
    #[inline] pub fn xfrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XFRC != 0"]
    #[inline] pub fn test_xfrc(&self) -> bool {
        self.xfrc() != 0
    }

    #[doc="Sets the XFRC field."]
    #[inline] pub fn set_xfrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doepint {
    #[inline]
    fn from(other: u32) -> Self {
         Doepint(other)
    }
}

impl ::core::fmt::Display for Doepint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doepint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.b2bstup() != 0 { try!(write!(f, " b2bstup"))}
        if self.otepdis() != 0 { try!(write!(f, " otepdis"))}
        if self.stup() != 0 { try!(write!(f, " stup"))}
        if self.epdisd() != 0 { try!(write!(f, " epdisd"))}
        if self.xfrc() != 0 { try!(write!(f, " xfrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-0 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dieptsiz0(pub u32);
impl Dieptsiz0 {
    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3) as u8) } // [20:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dieptsiz0 {
    #[inline]
    fn from(other: u32) -> Self {
         Dieptsiz0(other)
    }
}

impl ::core::fmt::Display for Dieptsiz0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dieptsiz0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device OUT endpoint-0 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doeptsiz0(pub u32);
impl Doeptsiz0 {
    #[doc="SETUP packet count"]
    #[inline] pub fn stupcnt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if STUPCNT != 0"]
    #[inline] pub fn test_stupcnt(&self) -> bool {
        self.stupcnt() != 0
    }

    #[doc="Sets the STUPCNT field."]
    #[inline] pub fn set_stupcnt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doeptsiz0 {
    #[inline]
    fn from(other: u32) -> Self {
         Doeptsiz0(other)
    }
}

impl ::core::fmt::Display for Doeptsiz0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doeptsiz0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stupcnt() != 0 { try!(write!(f, " stupcnt=0x{:x}", self.stupcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt"))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device endpoint-1 transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dieptsiz(pub u32);
impl Dieptsiz {
    #[doc="Multi count"]
    #[inline] pub fn mcnt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if MCNT != 0"]
    #[inline] pub fn test_mcnt(&self) -> bool {
        self.mcnt() != 0
    }

    #[doc="Sets the MCNT field."]
    #[inline] pub fn set_mcnt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> ::bobbin_bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<::bobbin_bits::U19>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dieptsiz {
    #[inline]
    fn from(other: u32) -> Self {
         Dieptsiz(other)
    }
}

impl ::core::fmt::Display for Dieptsiz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dieptsiz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcnt() != 0 { try!(write!(f, " mcnt=0x{:x}", self.mcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint transmit FIFO status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dtxfsts(pub u32);
impl Dtxfsts {
    #[doc="IN endpoint TxFIFO space available"]
    #[inline] pub fn ineptfsav(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INEPTFSAV != 0"]
    #[inline] pub fn test_ineptfsav(&self) -> bool {
        self.ineptfsav() != 0
    }

    #[doc="Sets the INEPTFSAV field."]
    #[inline] pub fn set_ineptfsav<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dtxfsts {
    #[inline]
    fn from(other: u32) -> Self {
         Dtxfsts(other)
    }
}

impl ::core::fmt::Display for Dtxfsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dtxfsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ineptfsav() != 0 { try!(write!(f, " ineptfsav=0x{:x}", self.ineptfsav()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="device OUT endpoint-n transfer size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Doeptsiz(pub u32);
impl Doeptsiz {
    #[doc="Received data PID/SETUP packet count"]
    #[inline] pub fn rxdpid_stupcnt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if RXDPID_STUPCNT != 0"]
    #[inline] pub fn test_rxdpid_stupcnt(&self) -> bool {
        self.rxdpid_stupcnt() != 0
    }

    #[doc="Sets the RXDPID_STUPCNT field."]
    #[inline] pub fn set_rxdpid_stupcnt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Packet count"]
    #[inline] pub fn pktcnt(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x3ff) as u16) } // [28:19]
    }

    #[doc="Returns true if PKTCNT != 0"]
    #[inline] pub fn test_pktcnt(&self) -> bool {
        self.pktcnt() != 0
    }

    #[doc="Sets the PKTCNT field."]
    #[inline] pub fn set_pktcnt<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Transfer size"]
    #[inline] pub fn xfrsiz(&self) -> ::bobbin_bits::U19 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ffff) as u32) } // [18:0]
    }

    #[doc="Returns true if XFRSIZ != 0"]
    #[inline] pub fn test_xfrsiz(&self) -> bool {
        self.xfrsiz() != 0
    }

    #[doc="Sets the XFRSIZ field."]
    #[inline] pub fn set_xfrsiz<V: Into<::bobbin_bits::U19>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U19 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Doeptsiz {
    #[inline]
    fn from(other: u32) -> Self {
         Doeptsiz(other)
    }
}

impl ::core::fmt::Display for Doeptsiz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Doeptsiz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdpid_stupcnt() != 0 { try!(write!(f, " rxdpid_stupcnt=0x{:x}", self.rxdpid_stupcnt()))}
        if self.pktcnt() != 0 { try!(write!(f, " pktcnt=0x{:x}", self.pktcnt()))}
        if self.xfrsiz() != 0 { try!(write!(f, " xfrsiz=0x{:x}", self.xfrsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

