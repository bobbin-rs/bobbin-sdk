::bobbin_mcu::periph!( PORT, Port, PORT_PERIPH, PortPeriph, PORT_OWNED, PORT_REF_COUNT, 0x41008000, 0x00, 0x18);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PORT Peripheral"]
pub struct PortPeriph(pub usize); 

impl PortPeriph {
    #[doc="Get the DIR Register."]
    #[inline] pub fn dir_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dir, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dir, 0x0, 0x80)
    }

    #[doc="Get the *mut pointer for the DIR register."]
    #[inline] pub fn dir_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dir { 
        self.dir_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DIR register."]
    #[inline] pub fn dir_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dir { 
        self.dir_reg().ptr(index.into())
    }

    #[doc="Read the DIR register."]
    #[inline] pub fn dir<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dir { 
        self.dir_reg().read(index.into())
    }

    #[doc="Write the DIR register."]
    #[inline] pub fn write_dir<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dir) -> &Self {
        self.dir_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DIR register."]
    #[inline] pub fn set_dir<I: Into<::bobbin_bits::R2>, F: FnOnce(Dir) -> Dir>(&self, index: I, f: F) -> &Self {
        self.dir_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DIR register."]
    #[inline] pub fn with_dir<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dir) -> Dir>(&self, index: I, f: F) -> &Self {
        self.dir_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DIRCLR Register."]
    #[inline] pub fn dirclr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dirclr, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dirclr, 0x4, 0x80)
    }

    #[doc="Get the *mut pointer for the DIRCLR register."]
    #[inline] pub fn dirclr_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dirclr { 
        self.dirclr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DIRCLR register."]
    #[inline] pub fn dirclr_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dirclr { 
        self.dirclr_reg().ptr(index.into())
    }

    #[doc="Read the DIRCLR register."]
    #[inline] pub fn dirclr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dirclr { 
        self.dirclr_reg().read(index.into())
    }

    #[doc="Write the DIRCLR register."]
    #[inline] pub fn write_dirclr<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dirclr) -> &Self {
        self.dirclr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DIRCLR register."]
    #[inline] pub fn set_dirclr<I: Into<::bobbin_bits::R2>, F: FnOnce(Dirclr) -> Dirclr>(&self, index: I, f: F) -> &Self {
        self.dirclr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DIRCLR register."]
    #[inline] pub fn with_dirclr<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dirclr) -> Dirclr>(&self, index: I, f: F) -> &Self {
        self.dirclr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DIRSET Register."]
    #[inline] pub fn dirset_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dirset, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dirset, 0x8, 0x80)
    }

    #[doc="Get the *mut pointer for the DIRSET register."]
    #[inline] pub fn dirset_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dirset { 
        self.dirset_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DIRSET register."]
    #[inline] pub fn dirset_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dirset { 
        self.dirset_reg().ptr(index.into())
    }

    #[doc="Read the DIRSET register."]
    #[inline] pub fn dirset<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dirset { 
        self.dirset_reg().read(index.into())
    }

    #[doc="Write the DIRSET register."]
    #[inline] pub fn write_dirset<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dirset) -> &Self {
        self.dirset_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DIRSET register."]
    #[inline] pub fn set_dirset<I: Into<::bobbin_bits::R2>, F: FnOnce(Dirset) -> Dirset>(&self, index: I, f: F) -> &Self {
        self.dirset_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DIRSET register."]
    #[inline] pub fn with_dirset<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dirset) -> Dirset>(&self, index: I, f: F) -> &Self {
        self.dirset_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DIRTGL Register."]
    #[inline] pub fn dirtgl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dirtgl, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dirtgl, 0xc, 0x80)
    }

    #[doc="Get the *mut pointer for the DIRTGL register."]
    #[inline] pub fn dirtgl_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dirtgl { 
        self.dirtgl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DIRTGL register."]
    #[inline] pub fn dirtgl_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dirtgl { 
        self.dirtgl_reg().ptr(index.into())
    }

    #[doc="Read the DIRTGL register."]
    #[inline] pub fn dirtgl<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dirtgl { 
        self.dirtgl_reg().read(index.into())
    }

    #[doc="Write the DIRTGL register."]
    #[inline] pub fn write_dirtgl<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dirtgl) -> &Self {
        self.dirtgl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DIRTGL register."]
    #[inline] pub fn set_dirtgl<I: Into<::bobbin_bits::R2>, F: FnOnce(Dirtgl) -> Dirtgl>(&self, index: I, f: F) -> &Self {
        self.dirtgl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DIRTGL register."]
    #[inline] pub fn with_dirtgl<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dirtgl) -> Dirtgl>(&self, index: I, f: F) -> &Self {
        self.dirtgl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the OUT Register."]
    #[inline] pub fn out_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Out, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Out, 0x10, 0x80)
    }

    #[doc="Get the *mut pointer for the OUT register."]
    #[inline] pub fn out_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Out { 
        self.out_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the OUT register."]
    #[inline] pub fn out_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Out { 
        self.out_reg().ptr(index.into())
    }

    #[doc="Read the OUT register."]
    #[inline] pub fn out<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Out { 
        self.out_reg().read(index.into())
    }

    #[doc="Write the OUT register."]
    #[inline] pub fn write_out<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Out) -> &Self {
        self.out_reg().write(index.into(), value);
        self
    }

    #[doc="Set the OUT register."]
    #[inline] pub fn set_out<I: Into<::bobbin_bits::R2>, F: FnOnce(Out) -> Out>(&self, index: I, f: F) -> &Self {
        self.out_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the OUT register."]
    #[inline] pub fn with_out<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Out) -> Out>(&self, index: I, f: F) -> &Self {
        self.out_reg().with(index.into(), f);
        self
    }

    #[doc="Get the OUTCLR Register."]
    #[inline] pub fn outclr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Outclr, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Outclr, 0x14, 0x80)
    }

    #[doc="Get the *mut pointer for the OUTCLR register."]
    #[inline] pub fn outclr_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Outclr { 
        self.outclr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the OUTCLR register."]
    #[inline] pub fn outclr_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Outclr { 
        self.outclr_reg().ptr(index.into())
    }

    #[doc="Read the OUTCLR register."]
    #[inline] pub fn outclr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Outclr { 
        self.outclr_reg().read(index.into())
    }

    #[doc="Write the OUTCLR register."]
    #[inline] pub fn write_outclr<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Outclr) -> &Self {
        self.outclr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the OUTCLR register."]
    #[inline] pub fn set_outclr<I: Into<::bobbin_bits::R2>, F: FnOnce(Outclr) -> Outclr>(&self, index: I, f: F) -> &Self {
        self.outclr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the OUTCLR register."]
    #[inline] pub fn with_outclr<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Outclr) -> Outclr>(&self, index: I, f: F) -> &Self {
        self.outclr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the OUTSET Register."]
    #[inline] pub fn outset_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Outset, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Outset, 0x18, 0x80)
    }

    #[doc="Get the *mut pointer for the OUTSET register."]
    #[inline] pub fn outset_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Outset { 
        self.outset_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the OUTSET register."]
    #[inline] pub fn outset_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Outset { 
        self.outset_reg().ptr(index.into())
    }

    #[doc="Read the OUTSET register."]
    #[inline] pub fn outset<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Outset { 
        self.outset_reg().read(index.into())
    }

    #[doc="Write the OUTSET register."]
    #[inline] pub fn write_outset<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Outset) -> &Self {
        self.outset_reg().write(index.into(), value);
        self
    }

    #[doc="Set the OUTSET register."]
    #[inline] pub fn set_outset<I: Into<::bobbin_bits::R2>, F: FnOnce(Outset) -> Outset>(&self, index: I, f: F) -> &Self {
        self.outset_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the OUTSET register."]
    #[inline] pub fn with_outset<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Outset) -> Outset>(&self, index: I, f: F) -> &Self {
        self.outset_reg().with(index.into(), f);
        self
    }

    #[doc="Get the OUTTGL Register."]
    #[inline] pub fn outtgl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Outtgl, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Outtgl, 0x1c, 0x80)
    }

    #[doc="Get the *mut pointer for the OUTTGL register."]
    #[inline] pub fn outtgl_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Outtgl { 
        self.outtgl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the OUTTGL register."]
    #[inline] pub fn outtgl_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Outtgl { 
        self.outtgl_reg().ptr(index.into())
    }

    #[doc="Read the OUTTGL register."]
    #[inline] pub fn outtgl<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Outtgl { 
        self.outtgl_reg().read(index.into())
    }

    #[doc="Write the OUTTGL register."]
    #[inline] pub fn write_outtgl<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Outtgl) -> &Self {
        self.outtgl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the OUTTGL register."]
    #[inline] pub fn set_outtgl<I: Into<::bobbin_bits::R2>, F: FnOnce(Outtgl) -> Outtgl>(&self, index: I, f: F) -> &Self {
        self.outtgl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the OUTTGL register."]
    #[inline] pub fn with_outtgl<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Outtgl) -> Outtgl>(&self, index: I, f: F) -> &Self {
        self.outtgl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the IN Register."]
    #[inline] pub fn in_reg(&self) -> ::bobbin_mcu::register::RegisterArray<In, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut In, 0x20, 0x80)
    }

    #[doc="Get the *mut pointer for the IN register."]
    #[inline] pub fn in_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut In { 
        self.in_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the IN register."]
    #[inline] pub fn in_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const In { 
        self.in_reg().ptr(index.into())
    }

    #[doc="Read the IN register."]
    #[inline] pub fn _in<I: Into<::bobbin_bits::R2>>(&self, index: I) -> In { 
        self.in_reg().read(index.into())
    }

    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Ctrl, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Ctrl, 0x24, 0x80)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Ctrl { 
        self.ctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Ctrl { 
        self.ctrl_reg().ptr(index.into())
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Ctrl { 
        self.ctrl_reg().read(index.into())
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn write_ctrl<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Ctrl) -> &Self {
        self.ctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CTRL register."]
    #[inline] pub fn set_ctrl<I: Into<::bobbin_bits::R2>, F: FnOnce(Ctrl) -> Ctrl>(&self, index: I, f: F) -> &Self {
        self.ctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Ctrl) -> Ctrl>(&self, index: I, f: F) -> &Self {
        self.ctrl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the WRCONFIG Register."]
    #[inline] pub fn wrconfig_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Wrconfig, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Wrconfig, 0x28, 0x80)
    }

    #[doc="Get the *mut pointer for the WRCONFIG register."]
    #[inline] pub fn wrconfig_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Wrconfig { 
        self.wrconfig_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the WRCONFIG register."]
    #[inline] pub fn wrconfig_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Wrconfig { 
        self.wrconfig_reg().ptr(index.into())
    }

    #[doc="Write the WRCONFIG register."]
    #[inline] pub fn write_wrconfig<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Wrconfig) -> &Self {
        self.wrconfig_reg().write(index.into(), value);
        self
    }

    #[doc="Set the WRCONFIG register."]
    #[inline] pub fn set_wrconfig<I: Into<::bobbin_bits::R2>, F: FnOnce(Wrconfig) -> Wrconfig>(&self, index: I, f: F) -> &Self {
        self.wrconfig_reg().set(index.into(), f);
        self
    }

    #[doc="Get the EVCTRL Register."]
    #[inline] pub fn evctrl_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Evctrl, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Evctrl, 0x2c, 0x80)
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Evctrl { 
        self.evctrl_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Evctrl { 
        self.evctrl_reg().ptr(index.into())
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Evctrl { 
        self.evctrl_reg().read(index.into())
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn write_evctrl<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Evctrl) -> &Self {
        self.evctrl_reg().write(index.into(), value);
        self
    }

    #[doc="Set the EVCTRL register."]
    #[inline] pub fn set_evctrl<I: Into<::bobbin_bits::R2>, F: FnOnce(Evctrl) -> Evctrl>(&self, index: I, f: F) -> &Self {
        self.evctrl_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Evctrl) -> Evctrl>(&self, index: I, f: F) -> &Self {
        self.evctrl_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PMUX0 Register."]
    #[inline] pub fn pmux0_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pmux0, ::bobbin_bits::R16> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pmux0, 0x30, 0x1)
    }

    #[doc="Get the *mut pointer for the PMUX0 register."]
    #[inline] pub fn pmux0_mut<I: Into<::bobbin_bits::R16>>(&self, index: I) -> *mut Pmux0 { 
        self.pmux0_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PMUX0 register."]
    #[inline] pub fn pmux0_ptr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> *const Pmux0 { 
        self.pmux0_reg().ptr(index.into())
    }

    #[doc="Read the PMUX0 register."]
    #[inline] pub fn pmux0<I: Into<::bobbin_bits::R16>>(&self, index: I) -> Pmux0 { 
        self.pmux0_reg().read(index.into())
    }

    #[doc="Write the PMUX0 register."]
    #[inline] pub fn write_pmux0<I: Into<::bobbin_bits::R16>>(&self, index: I, value: Pmux0) -> &Self {
        self.pmux0_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PMUX0 register."]
    #[inline] pub fn set_pmux0<I: Into<::bobbin_bits::R16>, F: FnOnce(Pmux0) -> Pmux0>(&self, index: I, f: F) -> &Self {
        self.pmux0_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PMUX0 register."]
    #[inline] pub fn with_pmux0<I: Into<::bobbin_bits::R16> + Copy, F: FnOnce(Pmux0) -> Pmux0>(&self, index: I, f: F) -> &Self {
        self.pmux0_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PMUX1 Register."]
    #[inline] pub fn pmux1_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pmux1, ::bobbin_bits::R16> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pmux1, 0xb0, 0x1)
    }

    #[doc="Get the *mut pointer for the PMUX1 register."]
    #[inline] pub fn pmux1_mut<I: Into<::bobbin_bits::R16>>(&self, index: I) -> *mut Pmux1 { 
        self.pmux1_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PMUX1 register."]
    #[inline] pub fn pmux1_ptr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> *const Pmux1 { 
        self.pmux1_reg().ptr(index.into())
    }

    #[doc="Read the PMUX1 register."]
    #[inline] pub fn pmux1<I: Into<::bobbin_bits::R16>>(&self, index: I) -> Pmux1 { 
        self.pmux1_reg().read(index.into())
    }

    #[doc="Write the PMUX1 register."]
    #[inline] pub fn write_pmux1<I: Into<::bobbin_bits::R16>>(&self, index: I, value: Pmux1) -> &Self {
        self.pmux1_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PMUX1 register."]
    #[inline] pub fn set_pmux1<I: Into<::bobbin_bits::R16>, F: FnOnce(Pmux1) -> Pmux1>(&self, index: I, f: F) -> &Self {
        self.pmux1_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PMUX1 register."]
    #[inline] pub fn with_pmux1<I: Into<::bobbin_bits::R16> + Copy, F: FnOnce(Pmux1) -> Pmux1>(&self, index: I, f: F) -> &Self {
        self.pmux1_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PINCFG0 Register."]
    #[inline] pub fn pincfg0_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pincfg0, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pincfg0, 0x40, 0x1)
    }

    #[doc="Get the *mut pointer for the PINCFG0 register."]
    #[inline] pub fn pincfg0_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Pincfg0 { 
        self.pincfg0_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PINCFG0 register."]
    #[inline] pub fn pincfg0_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Pincfg0 { 
        self.pincfg0_reg().ptr(index.into())
    }

    #[doc="Read the PINCFG0 register."]
    #[inline] pub fn pincfg0<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Pincfg0 { 
        self.pincfg0_reg().read(index.into())
    }

    #[doc="Write the PINCFG0 register."]
    #[inline] pub fn write_pincfg0<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Pincfg0) -> &Self {
        self.pincfg0_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PINCFG0 register."]
    #[inline] pub fn set_pincfg0<I: Into<::bobbin_bits::R32>, F: FnOnce(Pincfg0) -> Pincfg0>(&self, index: I, f: F) -> &Self {
        self.pincfg0_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PINCFG0 register."]
    #[inline] pub fn with_pincfg0<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Pincfg0) -> Pincfg0>(&self, index: I, f: F) -> &Self {
        self.pincfg0_reg().with(index.into(), f);
        self
    }

    #[doc="Get the PINCFG1 Register."]
    #[inline] pub fn pincfg1_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Pincfg1, ::bobbin_bits::R32> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Pincfg1, 0xc0, 0x1)
    }

    #[doc="Get the *mut pointer for the PINCFG1 register."]
    #[inline] pub fn pincfg1_mut<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *mut Pincfg1 { 
        self.pincfg1_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PINCFG1 register."]
    #[inline] pub fn pincfg1_ptr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> *const Pincfg1 { 
        self.pincfg1_reg().ptr(index.into())
    }

    #[doc="Read the PINCFG1 register."]
    #[inline] pub fn pincfg1<I: Into<::bobbin_bits::R32>>(&self, index: I) -> Pincfg1 { 
        self.pincfg1_reg().read(index.into())
    }

    #[doc="Write the PINCFG1 register."]
    #[inline] pub fn write_pincfg1<I: Into<::bobbin_bits::R32>>(&self, index: I, value: Pincfg1) -> &Self {
        self.pincfg1_reg().write(index.into(), value);
        self
    }

    #[doc="Set the PINCFG1 register."]
    #[inline] pub fn set_pincfg1<I: Into<::bobbin_bits::R32>, F: FnOnce(Pincfg1) -> Pincfg1>(&self, index: I, f: F) -> &Self {
        self.pincfg1_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the PINCFG1 register."]
    #[inline] pub fn with_pincfg1<I: Into<::bobbin_bits::R32> + Copy, F: FnOnce(Pincfg1) -> Pincfg1>(&self, index: I, f: F) -> &Self {
        self.pincfg1_reg().with(index.into(), f);
        self
    }

}

#[doc="Data Direction"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dir(pub u32);
impl Dir {
    #[doc="Port Data Direction"]
    #[inline] pub fn dir<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.dir(index) != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dir {
    #[inline]
    fn from(other: u32) -> Self {
         Dir(other)
    }
}

impl ::core::fmt::Display for Dir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir(0) != 0 { try!(write!(f, " dir[0]"))}
        if self.dir(1) != 0 { try!(write!(f, " dir[1]"))}
        if self.dir(2) != 0 { try!(write!(f, " dir[2]"))}
        if self.dir(3) != 0 { try!(write!(f, " dir[3]"))}
        if self.dir(4) != 0 { try!(write!(f, " dir[4]"))}
        if self.dir(5) != 0 { try!(write!(f, " dir[5]"))}
        if self.dir(6) != 0 { try!(write!(f, " dir[6]"))}
        if self.dir(7) != 0 { try!(write!(f, " dir[7]"))}
        if self.dir(8) != 0 { try!(write!(f, " dir[8]"))}
        if self.dir(9) != 0 { try!(write!(f, " dir[9]"))}
        if self.dir(10) != 0 { try!(write!(f, " dir[10]"))}
        if self.dir(11) != 0 { try!(write!(f, " dir[11]"))}
        if self.dir(12) != 0 { try!(write!(f, " dir[12]"))}
        if self.dir(13) != 0 { try!(write!(f, " dir[13]"))}
        if self.dir(14) != 0 { try!(write!(f, " dir[14]"))}
        if self.dir(15) != 0 { try!(write!(f, " dir[15]"))}
        if self.dir(16) != 0 { try!(write!(f, " dir[16]"))}
        if self.dir(17) != 0 { try!(write!(f, " dir[17]"))}
        if self.dir(18) != 0 { try!(write!(f, " dir[18]"))}
        if self.dir(19) != 0 { try!(write!(f, " dir[19]"))}
        if self.dir(20) != 0 { try!(write!(f, " dir[20]"))}
        if self.dir(21) != 0 { try!(write!(f, " dir[21]"))}
        if self.dir(22) != 0 { try!(write!(f, " dir[22]"))}
        if self.dir(23) != 0 { try!(write!(f, " dir[23]"))}
        if self.dir(24) != 0 { try!(write!(f, " dir[24]"))}
        if self.dir(25) != 0 { try!(write!(f, " dir[25]"))}
        if self.dir(26) != 0 { try!(write!(f, " dir[26]"))}
        if self.dir(27) != 0 { try!(write!(f, " dir[27]"))}
        if self.dir(28) != 0 { try!(write!(f, " dir[28]"))}
        if self.dir(29) != 0 { try!(write!(f, " dir[29]"))}
        if self.dir(30) != 0 { try!(write!(f, " dir[30]"))}
        if self.dir(31) != 0 { try!(write!(f, " dir[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Direction Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dirclr(pub u32);
impl Dirclr {
    #[doc="Port Data Direction Clear"]
    #[inline] pub fn dirclr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIRCLR != 0"]
    #[inline] pub fn test_dirclr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.dirclr(index) != 0
    }

    #[doc="Sets the DIRCLR field."]
    #[inline] pub fn set_dirclr<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dirclr {
    #[inline]
    fn from(other: u32) -> Self {
         Dirclr(other)
    }
}

impl ::core::fmt::Display for Dirclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dirclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dirclr(0) != 0 { try!(write!(f, " dirclr[0]"))}
        if self.dirclr(1) != 0 { try!(write!(f, " dirclr[1]"))}
        if self.dirclr(2) != 0 { try!(write!(f, " dirclr[2]"))}
        if self.dirclr(3) != 0 { try!(write!(f, " dirclr[3]"))}
        if self.dirclr(4) != 0 { try!(write!(f, " dirclr[4]"))}
        if self.dirclr(5) != 0 { try!(write!(f, " dirclr[5]"))}
        if self.dirclr(6) != 0 { try!(write!(f, " dirclr[6]"))}
        if self.dirclr(7) != 0 { try!(write!(f, " dirclr[7]"))}
        if self.dirclr(8) != 0 { try!(write!(f, " dirclr[8]"))}
        if self.dirclr(9) != 0 { try!(write!(f, " dirclr[9]"))}
        if self.dirclr(10) != 0 { try!(write!(f, " dirclr[10]"))}
        if self.dirclr(11) != 0 { try!(write!(f, " dirclr[11]"))}
        if self.dirclr(12) != 0 { try!(write!(f, " dirclr[12]"))}
        if self.dirclr(13) != 0 { try!(write!(f, " dirclr[13]"))}
        if self.dirclr(14) != 0 { try!(write!(f, " dirclr[14]"))}
        if self.dirclr(15) != 0 { try!(write!(f, " dirclr[15]"))}
        if self.dirclr(16) != 0 { try!(write!(f, " dirclr[16]"))}
        if self.dirclr(17) != 0 { try!(write!(f, " dirclr[17]"))}
        if self.dirclr(18) != 0 { try!(write!(f, " dirclr[18]"))}
        if self.dirclr(19) != 0 { try!(write!(f, " dirclr[19]"))}
        if self.dirclr(20) != 0 { try!(write!(f, " dirclr[20]"))}
        if self.dirclr(21) != 0 { try!(write!(f, " dirclr[21]"))}
        if self.dirclr(22) != 0 { try!(write!(f, " dirclr[22]"))}
        if self.dirclr(23) != 0 { try!(write!(f, " dirclr[23]"))}
        if self.dirclr(24) != 0 { try!(write!(f, " dirclr[24]"))}
        if self.dirclr(25) != 0 { try!(write!(f, " dirclr[25]"))}
        if self.dirclr(26) != 0 { try!(write!(f, " dirclr[26]"))}
        if self.dirclr(27) != 0 { try!(write!(f, " dirclr[27]"))}
        if self.dirclr(28) != 0 { try!(write!(f, " dirclr[28]"))}
        if self.dirclr(29) != 0 { try!(write!(f, " dirclr[29]"))}
        if self.dirclr(30) != 0 { try!(write!(f, " dirclr[30]"))}
        if self.dirclr(31) != 0 { try!(write!(f, " dirclr[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Direction Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dirset(pub u32);
impl Dirset {
    #[doc="Port Data Direction Set"]
    #[inline] pub fn dirset<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIRSET != 0"]
    #[inline] pub fn test_dirset<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.dirset(index) != 0
    }

    #[doc="Sets the DIRSET field."]
    #[inline] pub fn set_dirset<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dirset {
    #[inline]
    fn from(other: u32) -> Self {
         Dirset(other)
    }
}

impl ::core::fmt::Display for Dirset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dirset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dirset(0) != 0 { try!(write!(f, " dirset[0]"))}
        if self.dirset(1) != 0 { try!(write!(f, " dirset[1]"))}
        if self.dirset(2) != 0 { try!(write!(f, " dirset[2]"))}
        if self.dirset(3) != 0 { try!(write!(f, " dirset[3]"))}
        if self.dirset(4) != 0 { try!(write!(f, " dirset[4]"))}
        if self.dirset(5) != 0 { try!(write!(f, " dirset[5]"))}
        if self.dirset(6) != 0 { try!(write!(f, " dirset[6]"))}
        if self.dirset(7) != 0 { try!(write!(f, " dirset[7]"))}
        if self.dirset(8) != 0 { try!(write!(f, " dirset[8]"))}
        if self.dirset(9) != 0 { try!(write!(f, " dirset[9]"))}
        if self.dirset(10) != 0 { try!(write!(f, " dirset[10]"))}
        if self.dirset(11) != 0 { try!(write!(f, " dirset[11]"))}
        if self.dirset(12) != 0 { try!(write!(f, " dirset[12]"))}
        if self.dirset(13) != 0 { try!(write!(f, " dirset[13]"))}
        if self.dirset(14) != 0 { try!(write!(f, " dirset[14]"))}
        if self.dirset(15) != 0 { try!(write!(f, " dirset[15]"))}
        if self.dirset(16) != 0 { try!(write!(f, " dirset[16]"))}
        if self.dirset(17) != 0 { try!(write!(f, " dirset[17]"))}
        if self.dirset(18) != 0 { try!(write!(f, " dirset[18]"))}
        if self.dirset(19) != 0 { try!(write!(f, " dirset[19]"))}
        if self.dirset(20) != 0 { try!(write!(f, " dirset[20]"))}
        if self.dirset(21) != 0 { try!(write!(f, " dirset[21]"))}
        if self.dirset(22) != 0 { try!(write!(f, " dirset[22]"))}
        if self.dirset(23) != 0 { try!(write!(f, " dirset[23]"))}
        if self.dirset(24) != 0 { try!(write!(f, " dirset[24]"))}
        if self.dirset(25) != 0 { try!(write!(f, " dirset[25]"))}
        if self.dirset(26) != 0 { try!(write!(f, " dirset[26]"))}
        if self.dirset(27) != 0 { try!(write!(f, " dirset[27]"))}
        if self.dirset(28) != 0 { try!(write!(f, " dirset[28]"))}
        if self.dirset(29) != 0 { try!(write!(f, " dirset[29]"))}
        if self.dirset(30) != 0 { try!(write!(f, " dirset[30]"))}
        if self.dirset(31) != 0 { try!(write!(f, " dirset[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Direction Toggle"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dirtgl(pub u32);
impl Dirtgl {
    #[doc="Port Data Direction Toggle"]
    #[inline] pub fn dirtgl<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIRTGL != 0"]
    #[inline] pub fn test_dirtgl<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.dirtgl(index) != 0
    }

    #[doc="Sets the DIRTGL field."]
    #[inline] pub fn set_dirtgl<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dirtgl {
    #[inline]
    fn from(other: u32) -> Self {
         Dirtgl(other)
    }
}

impl ::core::fmt::Display for Dirtgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dirtgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dirtgl(0) != 0 { try!(write!(f, " dirtgl[0]"))}
        if self.dirtgl(1) != 0 { try!(write!(f, " dirtgl[1]"))}
        if self.dirtgl(2) != 0 { try!(write!(f, " dirtgl[2]"))}
        if self.dirtgl(3) != 0 { try!(write!(f, " dirtgl[3]"))}
        if self.dirtgl(4) != 0 { try!(write!(f, " dirtgl[4]"))}
        if self.dirtgl(5) != 0 { try!(write!(f, " dirtgl[5]"))}
        if self.dirtgl(6) != 0 { try!(write!(f, " dirtgl[6]"))}
        if self.dirtgl(7) != 0 { try!(write!(f, " dirtgl[7]"))}
        if self.dirtgl(8) != 0 { try!(write!(f, " dirtgl[8]"))}
        if self.dirtgl(9) != 0 { try!(write!(f, " dirtgl[9]"))}
        if self.dirtgl(10) != 0 { try!(write!(f, " dirtgl[10]"))}
        if self.dirtgl(11) != 0 { try!(write!(f, " dirtgl[11]"))}
        if self.dirtgl(12) != 0 { try!(write!(f, " dirtgl[12]"))}
        if self.dirtgl(13) != 0 { try!(write!(f, " dirtgl[13]"))}
        if self.dirtgl(14) != 0 { try!(write!(f, " dirtgl[14]"))}
        if self.dirtgl(15) != 0 { try!(write!(f, " dirtgl[15]"))}
        if self.dirtgl(16) != 0 { try!(write!(f, " dirtgl[16]"))}
        if self.dirtgl(17) != 0 { try!(write!(f, " dirtgl[17]"))}
        if self.dirtgl(18) != 0 { try!(write!(f, " dirtgl[18]"))}
        if self.dirtgl(19) != 0 { try!(write!(f, " dirtgl[19]"))}
        if self.dirtgl(20) != 0 { try!(write!(f, " dirtgl[20]"))}
        if self.dirtgl(21) != 0 { try!(write!(f, " dirtgl[21]"))}
        if self.dirtgl(22) != 0 { try!(write!(f, " dirtgl[22]"))}
        if self.dirtgl(23) != 0 { try!(write!(f, " dirtgl[23]"))}
        if self.dirtgl(24) != 0 { try!(write!(f, " dirtgl[24]"))}
        if self.dirtgl(25) != 0 { try!(write!(f, " dirtgl[25]"))}
        if self.dirtgl(26) != 0 { try!(write!(f, " dirtgl[26]"))}
        if self.dirtgl(27) != 0 { try!(write!(f, " dirtgl[27]"))}
        if self.dirtgl(28) != 0 { try!(write!(f, " dirtgl[28]"))}
        if self.dirtgl(29) != 0 { try!(write!(f, " dirtgl[29]"))}
        if self.dirtgl(30) != 0 { try!(write!(f, " dirtgl[30]"))}
        if self.dirtgl(31) != 0 { try!(write!(f, " dirtgl[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Output Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Out(pub u32);
impl Out {
    #[doc="PORT Data Output Value"]
    #[inline] pub fn out<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUT != 0"]
    #[inline] pub fn test_out<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.out(index) != 0
    }

    #[doc="Sets the OUT field."]
    #[inline] pub fn set_out<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Out {
    #[inline]
    fn from(other: u32) -> Self {
         Out(other)
    }
}

impl ::core::fmt::Display for Out {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Out {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.out(0) != 0 { try!(write!(f, " out[0]"))}
        if self.out(1) != 0 { try!(write!(f, " out[1]"))}
        if self.out(2) != 0 { try!(write!(f, " out[2]"))}
        if self.out(3) != 0 { try!(write!(f, " out[3]"))}
        if self.out(4) != 0 { try!(write!(f, " out[4]"))}
        if self.out(5) != 0 { try!(write!(f, " out[5]"))}
        if self.out(6) != 0 { try!(write!(f, " out[6]"))}
        if self.out(7) != 0 { try!(write!(f, " out[7]"))}
        if self.out(8) != 0 { try!(write!(f, " out[8]"))}
        if self.out(9) != 0 { try!(write!(f, " out[9]"))}
        if self.out(10) != 0 { try!(write!(f, " out[10]"))}
        if self.out(11) != 0 { try!(write!(f, " out[11]"))}
        if self.out(12) != 0 { try!(write!(f, " out[12]"))}
        if self.out(13) != 0 { try!(write!(f, " out[13]"))}
        if self.out(14) != 0 { try!(write!(f, " out[14]"))}
        if self.out(15) != 0 { try!(write!(f, " out[15]"))}
        if self.out(16) != 0 { try!(write!(f, " out[16]"))}
        if self.out(17) != 0 { try!(write!(f, " out[17]"))}
        if self.out(18) != 0 { try!(write!(f, " out[18]"))}
        if self.out(19) != 0 { try!(write!(f, " out[19]"))}
        if self.out(20) != 0 { try!(write!(f, " out[20]"))}
        if self.out(21) != 0 { try!(write!(f, " out[21]"))}
        if self.out(22) != 0 { try!(write!(f, " out[22]"))}
        if self.out(23) != 0 { try!(write!(f, " out[23]"))}
        if self.out(24) != 0 { try!(write!(f, " out[24]"))}
        if self.out(25) != 0 { try!(write!(f, " out[25]"))}
        if self.out(26) != 0 { try!(write!(f, " out[26]"))}
        if self.out(27) != 0 { try!(write!(f, " out[27]"))}
        if self.out(28) != 0 { try!(write!(f, " out[28]"))}
        if self.out(29) != 0 { try!(write!(f, " out[29]"))}
        if self.out(30) != 0 { try!(write!(f, " out[30]"))}
        if self.out(31) != 0 { try!(write!(f, " out[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Output Value Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Outclr(pub u32);
impl Outclr {
    #[doc="PORT Data Output Value Clear"]
    #[inline] pub fn outclr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUTCLR != 0"]
    #[inline] pub fn test_outclr<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.outclr(index) != 0
    }

    #[doc="Sets the OUTCLR field."]
    #[inline] pub fn set_outclr<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Outclr {
    #[inline]
    fn from(other: u32) -> Self {
         Outclr(other)
    }
}

impl ::core::fmt::Display for Outclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Outclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.outclr(0) != 0 { try!(write!(f, " outclr[0]"))}
        if self.outclr(1) != 0 { try!(write!(f, " outclr[1]"))}
        if self.outclr(2) != 0 { try!(write!(f, " outclr[2]"))}
        if self.outclr(3) != 0 { try!(write!(f, " outclr[3]"))}
        if self.outclr(4) != 0 { try!(write!(f, " outclr[4]"))}
        if self.outclr(5) != 0 { try!(write!(f, " outclr[5]"))}
        if self.outclr(6) != 0 { try!(write!(f, " outclr[6]"))}
        if self.outclr(7) != 0 { try!(write!(f, " outclr[7]"))}
        if self.outclr(8) != 0 { try!(write!(f, " outclr[8]"))}
        if self.outclr(9) != 0 { try!(write!(f, " outclr[9]"))}
        if self.outclr(10) != 0 { try!(write!(f, " outclr[10]"))}
        if self.outclr(11) != 0 { try!(write!(f, " outclr[11]"))}
        if self.outclr(12) != 0 { try!(write!(f, " outclr[12]"))}
        if self.outclr(13) != 0 { try!(write!(f, " outclr[13]"))}
        if self.outclr(14) != 0 { try!(write!(f, " outclr[14]"))}
        if self.outclr(15) != 0 { try!(write!(f, " outclr[15]"))}
        if self.outclr(16) != 0 { try!(write!(f, " outclr[16]"))}
        if self.outclr(17) != 0 { try!(write!(f, " outclr[17]"))}
        if self.outclr(18) != 0 { try!(write!(f, " outclr[18]"))}
        if self.outclr(19) != 0 { try!(write!(f, " outclr[19]"))}
        if self.outclr(20) != 0 { try!(write!(f, " outclr[20]"))}
        if self.outclr(21) != 0 { try!(write!(f, " outclr[21]"))}
        if self.outclr(22) != 0 { try!(write!(f, " outclr[22]"))}
        if self.outclr(23) != 0 { try!(write!(f, " outclr[23]"))}
        if self.outclr(24) != 0 { try!(write!(f, " outclr[24]"))}
        if self.outclr(25) != 0 { try!(write!(f, " outclr[25]"))}
        if self.outclr(26) != 0 { try!(write!(f, " outclr[26]"))}
        if self.outclr(27) != 0 { try!(write!(f, " outclr[27]"))}
        if self.outclr(28) != 0 { try!(write!(f, " outclr[28]"))}
        if self.outclr(29) != 0 { try!(write!(f, " outclr[29]"))}
        if self.outclr(30) != 0 { try!(write!(f, " outclr[30]"))}
        if self.outclr(31) != 0 { try!(write!(f, " outclr[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Output Value Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Outset(pub u32);
impl Outset {
    #[doc="PORT Data Output Value Set"]
    #[inline] pub fn outset<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUTSET != 0"]
    #[inline] pub fn test_outset<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.outset(index) != 0
    }

    #[doc="Sets the OUTSET field."]
    #[inline] pub fn set_outset<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Outset {
    #[inline]
    fn from(other: u32) -> Self {
         Outset(other)
    }
}

impl ::core::fmt::Display for Outset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Outset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.outset(0) != 0 { try!(write!(f, " outset[0]"))}
        if self.outset(1) != 0 { try!(write!(f, " outset[1]"))}
        if self.outset(2) != 0 { try!(write!(f, " outset[2]"))}
        if self.outset(3) != 0 { try!(write!(f, " outset[3]"))}
        if self.outset(4) != 0 { try!(write!(f, " outset[4]"))}
        if self.outset(5) != 0 { try!(write!(f, " outset[5]"))}
        if self.outset(6) != 0 { try!(write!(f, " outset[6]"))}
        if self.outset(7) != 0 { try!(write!(f, " outset[7]"))}
        if self.outset(8) != 0 { try!(write!(f, " outset[8]"))}
        if self.outset(9) != 0 { try!(write!(f, " outset[9]"))}
        if self.outset(10) != 0 { try!(write!(f, " outset[10]"))}
        if self.outset(11) != 0 { try!(write!(f, " outset[11]"))}
        if self.outset(12) != 0 { try!(write!(f, " outset[12]"))}
        if self.outset(13) != 0 { try!(write!(f, " outset[13]"))}
        if self.outset(14) != 0 { try!(write!(f, " outset[14]"))}
        if self.outset(15) != 0 { try!(write!(f, " outset[15]"))}
        if self.outset(16) != 0 { try!(write!(f, " outset[16]"))}
        if self.outset(17) != 0 { try!(write!(f, " outset[17]"))}
        if self.outset(18) != 0 { try!(write!(f, " outset[18]"))}
        if self.outset(19) != 0 { try!(write!(f, " outset[19]"))}
        if self.outset(20) != 0 { try!(write!(f, " outset[20]"))}
        if self.outset(21) != 0 { try!(write!(f, " outset[21]"))}
        if self.outset(22) != 0 { try!(write!(f, " outset[22]"))}
        if self.outset(23) != 0 { try!(write!(f, " outset[23]"))}
        if self.outset(24) != 0 { try!(write!(f, " outset[24]"))}
        if self.outset(25) != 0 { try!(write!(f, " outset[25]"))}
        if self.outset(26) != 0 { try!(write!(f, " outset[26]"))}
        if self.outset(27) != 0 { try!(write!(f, " outset[27]"))}
        if self.outset(28) != 0 { try!(write!(f, " outset[28]"))}
        if self.outset(29) != 0 { try!(write!(f, " outset[29]"))}
        if self.outset(30) != 0 { try!(write!(f, " outset[30]"))}
        if self.outset(31) != 0 { try!(write!(f, " outset[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Output Value Toggle"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Outtgl(pub u32);
impl Outtgl {
    #[doc="PORT Data Output Value Toggle"]
    #[inline] pub fn outtgl<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OUTTGL != 0"]
    #[inline] pub fn test_outtgl<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.outtgl(index) != 0
    }

    #[doc="Sets the OUTTGL field."]
    #[inline] pub fn set_outtgl<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Outtgl {
    #[inline]
    fn from(other: u32) -> Self {
         Outtgl(other)
    }
}

impl ::core::fmt::Display for Outtgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Outtgl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.outtgl(0) != 0 { try!(write!(f, " outtgl[0]"))}
        if self.outtgl(1) != 0 { try!(write!(f, " outtgl[1]"))}
        if self.outtgl(2) != 0 { try!(write!(f, " outtgl[2]"))}
        if self.outtgl(3) != 0 { try!(write!(f, " outtgl[3]"))}
        if self.outtgl(4) != 0 { try!(write!(f, " outtgl[4]"))}
        if self.outtgl(5) != 0 { try!(write!(f, " outtgl[5]"))}
        if self.outtgl(6) != 0 { try!(write!(f, " outtgl[6]"))}
        if self.outtgl(7) != 0 { try!(write!(f, " outtgl[7]"))}
        if self.outtgl(8) != 0 { try!(write!(f, " outtgl[8]"))}
        if self.outtgl(9) != 0 { try!(write!(f, " outtgl[9]"))}
        if self.outtgl(10) != 0 { try!(write!(f, " outtgl[10]"))}
        if self.outtgl(11) != 0 { try!(write!(f, " outtgl[11]"))}
        if self.outtgl(12) != 0 { try!(write!(f, " outtgl[12]"))}
        if self.outtgl(13) != 0 { try!(write!(f, " outtgl[13]"))}
        if self.outtgl(14) != 0 { try!(write!(f, " outtgl[14]"))}
        if self.outtgl(15) != 0 { try!(write!(f, " outtgl[15]"))}
        if self.outtgl(16) != 0 { try!(write!(f, " outtgl[16]"))}
        if self.outtgl(17) != 0 { try!(write!(f, " outtgl[17]"))}
        if self.outtgl(18) != 0 { try!(write!(f, " outtgl[18]"))}
        if self.outtgl(19) != 0 { try!(write!(f, " outtgl[19]"))}
        if self.outtgl(20) != 0 { try!(write!(f, " outtgl[20]"))}
        if self.outtgl(21) != 0 { try!(write!(f, " outtgl[21]"))}
        if self.outtgl(22) != 0 { try!(write!(f, " outtgl[22]"))}
        if self.outtgl(23) != 0 { try!(write!(f, " outtgl[23]"))}
        if self.outtgl(24) != 0 { try!(write!(f, " outtgl[24]"))}
        if self.outtgl(25) != 0 { try!(write!(f, " outtgl[25]"))}
        if self.outtgl(26) != 0 { try!(write!(f, " outtgl[26]"))}
        if self.outtgl(27) != 0 { try!(write!(f, " outtgl[27]"))}
        if self.outtgl(28) != 0 { try!(write!(f, " outtgl[28]"))}
        if self.outtgl(29) != 0 { try!(write!(f, " outtgl[29]"))}
        if self.outtgl(30) != 0 { try!(write!(f, " outtgl[30]"))}
        if self.outtgl(31) != 0 { try!(write!(f, " outtgl[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Input Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct In(pub u32);
impl In {
    #[doc="PORT Data Input Value"]
    #[inline] pub fn _in<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IN != 0"]
    #[inline] pub fn test_in<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self._in(index) != 0
    }

    #[doc="Sets the IN field."]
    #[inline] pub fn set_in<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for In {
    #[inline]
    fn from(other: u32) -> Self {
         In(other)
    }
}

impl ::core::fmt::Display for In {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for In {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._in(0) != 0 { try!(write!(f, " _in[0]"))}
        if self._in(1) != 0 { try!(write!(f, " _in[1]"))}
        if self._in(2) != 0 { try!(write!(f, " _in[2]"))}
        if self._in(3) != 0 { try!(write!(f, " _in[3]"))}
        if self._in(4) != 0 { try!(write!(f, " _in[4]"))}
        if self._in(5) != 0 { try!(write!(f, " _in[5]"))}
        if self._in(6) != 0 { try!(write!(f, " _in[6]"))}
        if self._in(7) != 0 { try!(write!(f, " _in[7]"))}
        if self._in(8) != 0 { try!(write!(f, " _in[8]"))}
        if self._in(9) != 0 { try!(write!(f, " _in[9]"))}
        if self._in(10) != 0 { try!(write!(f, " _in[10]"))}
        if self._in(11) != 0 { try!(write!(f, " _in[11]"))}
        if self._in(12) != 0 { try!(write!(f, " _in[12]"))}
        if self._in(13) != 0 { try!(write!(f, " _in[13]"))}
        if self._in(14) != 0 { try!(write!(f, " _in[14]"))}
        if self._in(15) != 0 { try!(write!(f, " _in[15]"))}
        if self._in(16) != 0 { try!(write!(f, " _in[16]"))}
        if self._in(17) != 0 { try!(write!(f, " _in[17]"))}
        if self._in(18) != 0 { try!(write!(f, " _in[18]"))}
        if self._in(19) != 0 { try!(write!(f, " _in[19]"))}
        if self._in(20) != 0 { try!(write!(f, " _in[20]"))}
        if self._in(21) != 0 { try!(write!(f, " _in[21]"))}
        if self._in(22) != 0 { try!(write!(f, " _in[22]"))}
        if self._in(23) != 0 { try!(write!(f, " _in[23]"))}
        if self._in(24) != 0 { try!(write!(f, " _in[24]"))}
        if self._in(25) != 0 { try!(write!(f, " _in[25]"))}
        if self._in(26) != 0 { try!(write!(f, " _in[26]"))}
        if self._in(27) != 0 { try!(write!(f, " _in[27]"))}
        if self._in(28) != 0 { try!(write!(f, " _in[28]"))}
        if self._in(29) != 0 { try!(write!(f, " _in[29]"))}
        if self._in(30) != 0 { try!(write!(f, " _in[30]"))}
        if self._in(31) != 0 { try!(write!(f, " _in[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Input Sampling Mode"]
    #[inline] pub fn sampling<I: Into<::bobbin_bits::R32>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SAMPLING != 0"]
    #[inline] pub fn test_sampling<I: Into<::bobbin_bits::R32>>(&self, index: I) -> bool{
        self.sampling(index) != 0
    }

    #[doc="Sets the SAMPLING field."]
    #[inline] pub fn set_sampling<I: Into<::bobbin_bits::R32>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ctrl {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.sampling(0) != 0 { try!(write!(f, " sampling[0]"))}
        if self.sampling(1) != 0 { try!(write!(f, " sampling[1]"))}
        if self.sampling(2) != 0 { try!(write!(f, " sampling[2]"))}
        if self.sampling(3) != 0 { try!(write!(f, " sampling[3]"))}
        if self.sampling(4) != 0 { try!(write!(f, " sampling[4]"))}
        if self.sampling(5) != 0 { try!(write!(f, " sampling[5]"))}
        if self.sampling(6) != 0 { try!(write!(f, " sampling[6]"))}
        if self.sampling(7) != 0 { try!(write!(f, " sampling[7]"))}
        if self.sampling(8) != 0 { try!(write!(f, " sampling[8]"))}
        if self.sampling(9) != 0 { try!(write!(f, " sampling[9]"))}
        if self.sampling(10) != 0 { try!(write!(f, " sampling[10]"))}
        if self.sampling(11) != 0 { try!(write!(f, " sampling[11]"))}
        if self.sampling(12) != 0 { try!(write!(f, " sampling[12]"))}
        if self.sampling(13) != 0 { try!(write!(f, " sampling[13]"))}
        if self.sampling(14) != 0 { try!(write!(f, " sampling[14]"))}
        if self.sampling(15) != 0 { try!(write!(f, " sampling[15]"))}
        if self.sampling(16) != 0 { try!(write!(f, " sampling[16]"))}
        if self.sampling(17) != 0 { try!(write!(f, " sampling[17]"))}
        if self.sampling(18) != 0 { try!(write!(f, " sampling[18]"))}
        if self.sampling(19) != 0 { try!(write!(f, " sampling[19]"))}
        if self.sampling(20) != 0 { try!(write!(f, " sampling[20]"))}
        if self.sampling(21) != 0 { try!(write!(f, " sampling[21]"))}
        if self.sampling(22) != 0 { try!(write!(f, " sampling[22]"))}
        if self.sampling(23) != 0 { try!(write!(f, " sampling[23]"))}
        if self.sampling(24) != 0 { try!(write!(f, " sampling[24]"))}
        if self.sampling(25) != 0 { try!(write!(f, " sampling[25]"))}
        if self.sampling(26) != 0 { try!(write!(f, " sampling[26]"))}
        if self.sampling(27) != 0 { try!(write!(f, " sampling[27]"))}
        if self.sampling(28) != 0 { try!(write!(f, " sampling[28]"))}
        if self.sampling(29) != 0 { try!(write!(f, " sampling[29]"))}
        if self.sampling(30) != 0 { try!(write!(f, " sampling[30]"))}
        if self.sampling(31) != 0 { try!(write!(f, " sampling[31]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Write Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Wrconfig(pub u32);
impl Wrconfig {
    #[doc="Pin Mask for Multiple Pin Configuration"]
    #[inline] pub fn pinmask(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PINMASK != 0"]
    #[inline] pub fn test_pinmask(&self) -> bool {
        self.pinmask() != 0
    }

    #[doc="Sets the PINMASK field."]
    #[inline] pub fn set_pinmask<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Peripheral Multiplexer Enable"]
    #[inline] pub fn pmuxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PMUXEN != 0"]
    #[inline] pub fn test_pmuxen(&self) -> bool {
        self.pmuxen() != 0
    }

    #[doc="Sets the PMUXEN field."]
    #[inline] pub fn set_pmuxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Input Enable"]
    #[inline] pub fn inen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if INEN != 0"]
    #[inline] pub fn test_inen(&self) -> bool {
        self.inen() != 0
    }

    #[doc="Sets the INEN field."]
    #[inline] pub fn set_inen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pull Enable"]
    #[inline] pub fn pullen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PULLEN != 0"]
    #[inline] pub fn test_pullen(&self) -> bool {
        self.pullen() != 0
    }

    #[doc="Sets the PULLEN field."]
    #[inline] pub fn set_pullen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Output Driver Strength Selection"]
    #[inline] pub fn drvstr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if DRVSTR != 0"]
    #[inline] pub fn test_drvstr(&self) -> bool {
        self.drvstr() != 0
    }

    #[doc="Sets the DRVSTR field."]
    #[inline] pub fn set_drvstr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Peripheral Multiplexing"]
    #[inline] pub fn pmux(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if PMUX != 0"]
    #[inline] pub fn test_pmux(&self) -> bool {
        self.pmux() != 0
    }

    #[doc="Sets the PMUX field."]
    #[inline] pub fn set_pmux<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Write PMUX"]
    #[inline] pub fn wrpmux(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if WRPMUX != 0"]
    #[inline] pub fn test_wrpmux(&self) -> bool {
        self.wrpmux() != 0
    }

    #[doc="Sets the WRPMUX field."]
    #[inline] pub fn set_wrpmux<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Write PINCFG"]
    #[inline] pub fn wrpincfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if WRPINCFG != 0"]
    #[inline] pub fn test_wrpincfg(&self) -> bool {
        self.wrpincfg() != 0
    }

    #[doc="Sets the WRPINCFG field."]
    #[inline] pub fn set_wrpincfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Half-Word Select"]
    #[inline] pub fn hwsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if HWSEL != 0"]
    #[inline] pub fn test_hwsel(&self) -> bool {
        self.hwsel() != 0
    }

    #[doc="Sets the HWSEL field."]
    #[inline] pub fn set_hwsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Wrconfig {
    #[inline]
    fn from(other: u32) -> Self {
         Wrconfig(other)
    }
}

impl ::core::fmt::Display for Wrconfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Wrconfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pinmask() != 0 { try!(write!(f, " pinmask=0x{:x}", self.pinmask()))}
        if self.pmuxen() != 0 { try!(write!(f, " pmuxen"))}
        if self.inen() != 0 { try!(write!(f, " inen"))}
        if self.pullen() != 0 { try!(write!(f, " pullen"))}
        if self.drvstr() != 0 { try!(write!(f, " drvstr"))}
        if self.pmux() != 0 { try!(write!(f, " pmux=0x{:x}", self.pmux()))}
        if self.wrpmux() != 0 { try!(write!(f, " wrpmux"))}
        if self.wrpincfg() != 0 { try!(write!(f, " wrpincfg"))}
        if self.hwsel() != 0 { try!(write!(f, " hwsel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Input Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u32);
impl Evctrl {
    #[doc="PORT Event Pin Identifier 0"]
    #[inline] pub fn pid0(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if PID0 != 0"]
    #[inline] pub fn test_pid0(&self) -> bool {
        self.pid0() != 0
    }

    #[doc="Sets the PID0 field."]
    #[inline] pub fn set_pid0<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PORT Event Action 0"]
    #[inline] pub fn evact0(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if EVACT0 != 0"]
    #[inline] pub fn test_evact0(&self) -> bool {
        self.evact0() != 0
    }

    #[doc="Sets the EVACT0 field."]
    #[inline] pub fn set_evact0<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="PORT Event Input Enable 0"]
    #[inline] pub fn portei0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PORTEI0 != 0"]
    #[inline] pub fn test_portei0(&self) -> bool {
        self.portei0() != 0
    }

    #[doc="Sets the PORTEI0 field."]
    #[inline] pub fn set_portei0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="PORT Event Pin Identifier 1"]
    #[inline] pub fn pid1(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if PID1 != 0"]
    #[inline] pub fn test_pid1(&self) -> bool {
        self.pid1() != 0
    }

    #[doc="Sets the PID1 field."]
    #[inline] pub fn set_pid1<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PORT Event Action 1"]
    #[inline] pub fn evact1(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if EVACT1 != 0"]
    #[inline] pub fn test_evact1(&self) -> bool {
        self.evact1() != 0
    }

    #[doc="Sets the EVACT1 field."]
    #[inline] pub fn set_evact1<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="PORT Event Input Enable 1"]
    #[inline] pub fn portei1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PORTEI1 != 0"]
    #[inline] pub fn test_portei1(&self) -> bool {
        self.portei1() != 0
    }

    #[doc="Sets the PORTEI1 field."]
    #[inline] pub fn set_portei1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="PORT Event Pin Identifier 2"]
    #[inline] pub fn pid2(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if PID2 != 0"]
    #[inline] pub fn test_pid2(&self) -> bool {
        self.pid2() != 0
    }

    #[doc="Sets the PID2 field."]
    #[inline] pub fn set_pid2<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="PORT Event Action 2"]
    #[inline] pub fn evact2(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if EVACT2 != 0"]
    #[inline] pub fn test_evact2(&self) -> bool {
        self.evact2() != 0
    }

    #[doc="Sets the EVACT2 field."]
    #[inline] pub fn set_evact2<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="PORT Event Input Enable 2"]
    #[inline] pub fn portei2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PORTEI2 != 0"]
    #[inline] pub fn test_portei2(&self) -> bool {
        self.portei2() != 0
    }

    #[doc="Sets the PORTEI2 field."]
    #[inline] pub fn set_portei2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="PORT Event Pin Identifier 3"]
    #[inline] pub fn pid3(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1f) as u8) } // [28:24]
    }

    #[doc="Returns true if PID3 != 0"]
    #[inline] pub fn test_pid3(&self) -> bool {
        self.pid3() != 0
    }

    #[doc="Sets the PID3 field."]
    #[inline] pub fn set_pid3<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="PORT Event Action 3"]
    #[inline] pub fn evact3(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x3) as u8) } // [30:29]
    }

    #[doc="Returns true if EVACT3 != 0"]
    #[inline] pub fn test_evact3(&self) -> bool {
        self.evact3() != 0
    }

    #[doc="Sets the EVACT3 field."]
    #[inline] pub fn set_evact3<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="PORT Event Input Enable 3"]
    #[inline] pub fn portei3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PORTEI3 != 0"]
    #[inline] pub fn test_portei3(&self) -> bool {
        self.portei3() != 0
    }

    #[doc="Sets the PORTEI3 field."]
    #[inline] pub fn set_portei3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.pid0() != 0 { try!(write!(f, " pid0=0x{:x}", self.pid0()))}
        if self.evact0() != 0 { try!(write!(f, " evact0=0x{:x}", self.evact0()))}
        if self.portei0() != 0 { try!(write!(f, " portei0"))}
        if self.pid1() != 0 { try!(write!(f, " pid1=0x{:x}", self.pid1()))}
        if self.evact1() != 0 { try!(write!(f, " evact1=0x{:x}", self.evact1()))}
        if self.portei1() != 0 { try!(write!(f, " portei1"))}
        if self.pid2() != 0 { try!(write!(f, " pid2=0x{:x}", self.pid2()))}
        if self.evact2() != 0 { try!(write!(f, " evact2=0x{:x}", self.evact2()))}
        if self.portei2() != 0 { try!(write!(f, " portei2"))}
        if self.pid3() != 0 { try!(write!(f, " pid3=0x{:x}", self.pid3()))}
        if self.evact3() != 0 { try!(write!(f, " evact3=0x{:x}", self.evact3()))}
        if self.portei3() != 0 { try!(write!(f, " portei3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Multiplexing - Group 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmux0(pub u8);
impl Pmux0 {
    #[doc="Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline] pub fn pmuxe(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PMUXE != 0"]
    #[inline] pub fn test_pmuxe(&self) -> bool {
        self.pmuxe() != 0
    }

    #[doc="Sets the PMUXE field."]
    #[inline] pub fn set_pmuxe<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline] pub fn pmuxo(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if PMUXO != 0"]
    #[inline] pub fn test_pmuxo(&self) -> bool {
        self.pmuxo() != 0
    }

    #[doc="Sets the PMUXO field."]
    #[inline] pub fn set_pmuxo<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Pmux0 {
    #[inline]
    fn from(other: u8) -> Self {
         Pmux0(other)
    }
}

impl ::core::fmt::Display for Pmux0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmux0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pmuxe() != 0 { try!(write!(f, " pmuxe=0x{:x}", self.pmuxe()))}
        if self.pmuxo() != 0 { try!(write!(f, " pmuxo=0x{:x}", self.pmuxo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral Multiplexing - Group 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmux1(pub u32);
impl Pmux1 {
}

impl From<u32> for Pmux1 {
    #[inline]
    fn from(other: u32) -> Self {
         Pmux1(other)
    }
}

impl ::core::fmt::Display for Pmux1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmux1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pin Configuration - Group 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pincfg0(pub u8);
impl Pincfg0 {
    #[doc="Peripheral Multiplexer Enable"]
    #[inline] pub fn pmuxen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PMUXEN != 0"]
    #[inline] pub fn test_pmuxen(&self) -> bool {
        self.pmuxen() != 0
    }

    #[doc="Sets the PMUXEN field."]
    #[inline] pub fn set_pmuxen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Input Enable"]
    #[inline] pub fn inen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INEN != 0"]
    #[inline] pub fn test_inen(&self) -> bool {
        self.inen() != 0
    }

    #[doc="Sets the INEN field."]
    #[inline] pub fn set_inen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pull Enable"]
    #[inline] pub fn pullen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PULLEN != 0"]
    #[inline] pub fn test_pullen(&self) -> bool {
        self.pullen() != 0
    }

    #[doc="Sets the PULLEN field."]
    #[inline] pub fn set_pullen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Output Driver Strength Selection"]
    #[inline] pub fn drvstr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DRVSTR != 0"]
    #[inline] pub fn test_drvstr(&self) -> bool {
        self.drvstr() != 0
    }

    #[doc="Sets the DRVSTR field."]
    #[inline] pub fn set_drvstr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Pincfg0 {
    #[inline]
    fn from(other: u8) -> Self {
         Pincfg0(other)
    }
}

impl ::core::fmt::Display for Pincfg0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pincfg0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pmuxen() != 0 { try!(write!(f, " pmuxen"))}
        if self.inen() != 0 { try!(write!(f, " inen"))}
        if self.pullen() != 0 { try!(write!(f, " pullen"))}
        if self.drvstr() != 0 { try!(write!(f, " drvstr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pin Configuration - Group 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pincfg1(pub u32);
impl Pincfg1 {
}

impl From<u32> for Pincfg1 {
    #[inline]
    fn from(other: u32) -> Self {
         Pincfg1(other)
    }
}

impl ::core::fmt::Display for Pincfg1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pincfg1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

