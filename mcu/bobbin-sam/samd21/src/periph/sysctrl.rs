#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="System Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SysctrlPeriph(pub usize);
impl SysctrlPeriph {
    #[doc="Get the BOD33 Register."]
    #[inline] pub fn bod33_reg(&self) -> Register<Bod33> { 
        Register::new(self.0 as *mut Bod33, 0x34)
    }

    #[doc="Get the *mut pointer for the BOD33 register."]
    #[inline] pub fn bod33_mut(&self) -> *mut Bod33 { 
        self.bod33_reg().ptr()
    }

    #[doc="Get the *const pointer for the BOD33 register."]
    #[inline] pub fn bod33_ptr(&self) -> *const Bod33 { 
        self.bod33_reg().ptr()
    }

    #[doc="Read the BOD33 register."]
    #[inline] pub fn bod33(&self) -> Bod33 { 
        self.bod33_reg().read()
    }

    #[doc="Write the BOD33 register."]
    #[inline] pub fn write_bod33(&self, value: Bod33) -> &Self { 
        self.bod33_reg().write(value);
        self
    }

    #[doc="Set the BOD33 register."]
    #[inline] pub fn set_bod33<F: FnOnce(Bod33) -> Bod33>(&self, f: F) -> &Self {
        self.bod33_reg().set(f);
        self
    }

    #[doc="Modify the BOD33 register."]
    #[inline] pub fn with_bod33<F: FnOnce(Bod33) -> Bod33>(&self, f: F) -> &Self {
        self.bod33_reg().with(f);
        self
    }

    #[doc="Get the DFLLCTRL Register."]
    #[inline] pub fn dfllctrl_reg(&self) -> Register<Dfllctrl> { 
        Register::new(self.0 as *mut Dfllctrl, 0x24)
    }

    #[doc="Get the *mut pointer for the DFLLCTRL register."]
    #[inline] pub fn dfllctrl_mut(&self) -> *mut Dfllctrl { 
        self.dfllctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the DFLLCTRL register."]
    #[inline] pub fn dfllctrl_ptr(&self) -> *const Dfllctrl { 
        self.dfllctrl_reg().ptr()
    }

    #[doc="Read the DFLLCTRL register."]
    #[inline] pub fn dfllctrl(&self) -> Dfllctrl { 
        self.dfllctrl_reg().read()
    }

    #[doc="Write the DFLLCTRL register."]
    #[inline] pub fn write_dfllctrl(&self, value: Dfllctrl) -> &Self { 
        self.dfllctrl_reg().write(value);
        self
    }

    #[doc="Set the DFLLCTRL register."]
    #[inline] pub fn set_dfllctrl<F: FnOnce(Dfllctrl) -> Dfllctrl>(&self, f: F) -> &Self {
        self.dfllctrl_reg().set(f);
        self
    }

    #[doc="Modify the DFLLCTRL register."]
    #[inline] pub fn with_dfllctrl<F: FnOnce(Dfllctrl) -> Dfllctrl>(&self, f: F) -> &Self {
        self.dfllctrl_reg().with(f);
        self
    }

    #[doc="Get the DFLLMUL Register."]
    #[inline] pub fn dfllmul_reg(&self) -> Register<Dfllmul> { 
        Register::new(self.0 as *mut Dfllmul, 0x2c)
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
    #[inline] pub fn dfllsync_reg(&self) -> Register<Dfllsync> { 
        Register::new(self.0 as *mut Dfllsync, 0x30)
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

    #[doc="Get the DFLLVAL Register."]
    #[inline] pub fn dfllval_reg(&self) -> Register<Dfllval> { 
        Register::new(self.0 as *mut Dfllval, 0x28)
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

    #[doc="Get the DPLLCTRLA Register."]
    #[inline] pub fn dpllctrla_reg(&self) -> Register<Dpllctrla> { 
        Register::new(self.0 as *mut Dpllctrla, 0x44)
    }

    #[doc="Get the *mut pointer for the DPLLCTRLA register."]
    #[inline] pub fn dpllctrla_mut(&self) -> *mut Dpllctrla { 
        self.dpllctrla_reg().ptr()
    }

    #[doc="Get the *const pointer for the DPLLCTRLA register."]
    #[inline] pub fn dpllctrla_ptr(&self) -> *const Dpllctrla { 
        self.dpllctrla_reg().ptr()
    }

    #[doc="Read the DPLLCTRLA register."]
    #[inline] pub fn dpllctrla(&self) -> Dpllctrla { 
        self.dpllctrla_reg().read()
    }

    #[doc="Write the DPLLCTRLA register."]
    #[inline] pub fn write_dpllctrla(&self, value: Dpllctrla) -> &Self { 
        self.dpllctrla_reg().write(value);
        self
    }

    #[doc="Set the DPLLCTRLA register."]
    #[inline] pub fn set_dpllctrla<F: FnOnce(Dpllctrla) -> Dpllctrla>(&self, f: F) -> &Self {
        self.dpllctrla_reg().set(f);
        self
    }

    #[doc="Modify the DPLLCTRLA register."]
    #[inline] pub fn with_dpllctrla<F: FnOnce(Dpllctrla) -> Dpllctrla>(&self, f: F) -> &Self {
        self.dpllctrla_reg().with(f);
        self
    }

    #[doc="Get the DPLLCTRLB Register."]
    #[inline] pub fn dpllctrlb_reg(&self) -> Register<Dpllctrlb> { 
        Register::new(self.0 as *mut Dpllctrlb, 0x4c)
    }

    #[doc="Get the *mut pointer for the DPLLCTRLB register."]
    #[inline] pub fn dpllctrlb_mut(&self) -> *mut Dpllctrlb { 
        self.dpllctrlb_reg().ptr()
    }

    #[doc="Get the *const pointer for the DPLLCTRLB register."]
    #[inline] pub fn dpllctrlb_ptr(&self) -> *const Dpllctrlb { 
        self.dpllctrlb_reg().ptr()
    }

    #[doc="Read the DPLLCTRLB register."]
    #[inline] pub fn dpllctrlb(&self) -> Dpllctrlb { 
        self.dpllctrlb_reg().read()
    }

    #[doc="Write the DPLLCTRLB register."]
    #[inline] pub fn write_dpllctrlb(&self, value: Dpllctrlb) -> &Self { 
        self.dpllctrlb_reg().write(value);
        self
    }

    #[doc="Set the DPLLCTRLB register."]
    #[inline] pub fn set_dpllctrlb<F: FnOnce(Dpllctrlb) -> Dpllctrlb>(&self, f: F) -> &Self {
        self.dpllctrlb_reg().set(f);
        self
    }

    #[doc="Modify the DPLLCTRLB register."]
    #[inline] pub fn with_dpllctrlb<F: FnOnce(Dpllctrlb) -> Dpllctrlb>(&self, f: F) -> &Self {
        self.dpllctrlb_reg().with(f);
        self
    }

    #[doc="Get the DPLLRATIO Register."]
    #[inline] pub fn dpllratio_reg(&self) -> Register<Dpllratio> { 
        Register::new(self.0 as *mut Dpllratio, 0x48)
    }

    #[doc="Get the *mut pointer for the DPLLRATIO register."]
    #[inline] pub fn dpllratio_mut(&self) -> *mut Dpllratio { 
        self.dpllratio_reg().ptr()
    }

    #[doc="Get the *const pointer for the DPLLRATIO register."]
    #[inline] pub fn dpllratio_ptr(&self) -> *const Dpllratio { 
        self.dpllratio_reg().ptr()
    }

    #[doc="Read the DPLLRATIO register."]
    #[inline] pub fn dpllratio(&self) -> Dpllratio { 
        self.dpllratio_reg().read()
    }

    #[doc="Write the DPLLRATIO register."]
    #[inline] pub fn write_dpllratio(&self, value: Dpllratio) -> &Self { 
        self.dpllratio_reg().write(value);
        self
    }

    #[doc="Set the DPLLRATIO register."]
    #[inline] pub fn set_dpllratio<F: FnOnce(Dpllratio) -> Dpllratio>(&self, f: F) -> &Self {
        self.dpllratio_reg().set(f);
        self
    }

    #[doc="Modify the DPLLRATIO register."]
    #[inline] pub fn with_dpllratio<F: FnOnce(Dpllratio) -> Dpllratio>(&self, f: F) -> &Self {
        self.dpllratio_reg().with(f);
        self
    }

    #[doc="Get the DPLLSTATUS Register."]
    #[inline] pub fn dpllstatus_reg(&self) -> Register<Dpllstatus> { 
        Register::new(self.0 as *mut Dpllstatus, 0x50)
    }

    #[doc="Get the *mut pointer for the DPLLSTATUS register."]
    #[inline] pub fn dpllstatus_mut(&self) -> *mut Dpllstatus { 
        self.dpllstatus_reg().ptr()
    }

    #[doc="Get the *const pointer for the DPLLSTATUS register."]
    #[inline] pub fn dpllstatus_ptr(&self) -> *const Dpllstatus { 
        self.dpllstatus_reg().ptr()
    }

    #[doc="Read the DPLLSTATUS register."]
    #[inline] pub fn dpllstatus(&self) -> Dpllstatus { 
        self.dpllstatus_reg().read()
    }

    #[doc="Get the INTENCLR Register."]
    #[inline] pub fn intenclr_reg(&self) -> Register<Intenclr> { 
        Register::new(self.0 as *mut Intenclr, 0x0)
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
    #[inline] pub fn intenset_reg(&self) -> Register<Intenset> { 
        Register::new(self.0 as *mut Intenset, 0x4)
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
    #[inline] pub fn intflag_reg(&self) -> Register<Intflag> { 
        Register::new(self.0 as *mut Intflag, 0x8)
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

    #[doc="Get the OSCULP32K Register."]
    #[inline] pub fn osculp32k_reg(&self) -> Register<Osculp32k> { 
        Register::new(self.0 as *mut Osculp32k, 0x1c)
    }

    #[doc="Get the *mut pointer for the OSCULP32K register."]
    #[inline] pub fn osculp32k_mut(&self) -> *mut Osculp32k { 
        self.osculp32k_reg().ptr()
    }

    #[doc="Get the *const pointer for the OSCULP32K register."]
    #[inline] pub fn osculp32k_ptr(&self) -> *const Osculp32k { 
        self.osculp32k_reg().ptr()
    }

    #[doc="Read the OSCULP32K register."]
    #[inline] pub fn osculp32k(&self) -> Osculp32k { 
        self.osculp32k_reg().read()
    }

    #[doc="Write the OSCULP32K register."]
    #[inline] pub fn write_osculp32k(&self, value: Osculp32k) -> &Self { 
        self.osculp32k_reg().write(value);
        self
    }

    #[doc="Set the OSCULP32K register."]
    #[inline] pub fn set_osculp32k<F: FnOnce(Osculp32k) -> Osculp32k>(&self, f: F) -> &Self {
        self.osculp32k_reg().set(f);
        self
    }

    #[doc="Modify the OSCULP32K register."]
    #[inline] pub fn with_osculp32k<F: FnOnce(Osculp32k) -> Osculp32k>(&self, f: F) -> &Self {
        self.osculp32k_reg().with(f);
        self
    }

    #[doc="Get the OSC8M Register."]
    #[inline] pub fn osc8m_reg(&self) -> Register<Osc8m> { 
        Register::new(self.0 as *mut Osc8m, 0x20)
    }

    #[doc="Get the *mut pointer for the OSC8M register."]
    #[inline] pub fn osc8m_mut(&self) -> *mut Osc8m { 
        self.osc8m_reg().ptr()
    }

    #[doc="Get the *const pointer for the OSC8M register."]
    #[inline] pub fn osc8m_ptr(&self) -> *const Osc8m { 
        self.osc8m_reg().ptr()
    }

    #[doc="Read the OSC8M register."]
    #[inline] pub fn osc8m(&self) -> Osc8m { 
        self.osc8m_reg().read()
    }

    #[doc="Write the OSC8M register."]
    #[inline] pub fn write_osc8m(&self, value: Osc8m) -> &Self { 
        self.osc8m_reg().write(value);
        self
    }

    #[doc="Set the OSC8M register."]
    #[inline] pub fn set_osc8m<F: FnOnce(Osc8m) -> Osc8m>(&self, f: F) -> &Self {
        self.osc8m_reg().set(f);
        self
    }

    #[doc="Modify the OSC8M register."]
    #[inline] pub fn with_osc8m<F: FnOnce(Osc8m) -> Osc8m>(&self, f: F) -> &Self {
        self.osc8m_reg().with(f);
        self
    }

    #[doc="Get the OSC32K Register."]
    #[inline] pub fn osc32k_reg(&self) -> Register<Osc32k> { 
        Register::new(self.0 as *mut Osc32k, 0x18)
    }

    #[doc="Get the *mut pointer for the OSC32K register."]
    #[inline] pub fn osc32k_mut(&self) -> *mut Osc32k { 
        self.osc32k_reg().ptr()
    }

    #[doc="Get the *const pointer for the OSC32K register."]
    #[inline] pub fn osc32k_ptr(&self) -> *const Osc32k { 
        self.osc32k_reg().ptr()
    }

    #[doc="Read the OSC32K register."]
    #[inline] pub fn osc32k(&self) -> Osc32k { 
        self.osc32k_reg().read()
    }

    #[doc="Write the OSC32K register."]
    #[inline] pub fn write_osc32k(&self, value: Osc32k) -> &Self { 
        self.osc32k_reg().write(value);
        self
    }

    #[doc="Set the OSC32K register."]
    #[inline] pub fn set_osc32k<F: FnOnce(Osc32k) -> Osc32k>(&self, f: F) -> &Self {
        self.osc32k_reg().set(f);
        self
    }

    #[doc="Modify the OSC32K register."]
    #[inline] pub fn with_osc32k<F: FnOnce(Osc32k) -> Osc32k>(&self, f: F) -> &Self {
        self.osc32k_reg().with(f);
        self
    }

    #[doc="Get the PCLKSR Register."]
    #[inline] pub fn pclksr_reg(&self) -> Register<Pclksr> { 
        Register::new(self.0 as *mut Pclksr, 0xc)
    }

    #[doc="Get the *mut pointer for the PCLKSR register."]
    #[inline] pub fn pclksr_mut(&self) -> *mut Pclksr { 
        self.pclksr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCLKSR register."]
    #[inline] pub fn pclksr_ptr(&self) -> *const Pclksr { 
        self.pclksr_reg().ptr()
    }

    #[doc="Read the PCLKSR register."]
    #[inline] pub fn pclksr(&self) -> Pclksr { 
        self.pclksr_reg().read()
    }

    #[doc="Get the VREF Register."]
    #[inline] pub fn vref_reg(&self) -> Register<Vref> { 
        Register::new(self.0 as *mut Vref, 0x40)
    }

    #[doc="Get the *mut pointer for the VREF register."]
    #[inline] pub fn vref_mut(&self) -> *mut Vref { 
        self.vref_reg().ptr()
    }

    #[doc="Get the *const pointer for the VREF register."]
    #[inline] pub fn vref_ptr(&self) -> *const Vref { 
        self.vref_reg().ptr()
    }

    #[doc="Read the VREF register."]
    #[inline] pub fn vref(&self) -> Vref { 
        self.vref_reg().read()
    }

    #[doc="Write the VREF register."]
    #[inline] pub fn write_vref(&self, value: Vref) -> &Self { 
        self.vref_reg().write(value);
        self
    }

    #[doc="Set the VREF register."]
    #[inline] pub fn set_vref<F: FnOnce(Vref) -> Vref>(&self, f: F) -> &Self {
        self.vref_reg().set(f);
        self
    }

    #[doc="Modify the VREF register."]
    #[inline] pub fn with_vref<F: FnOnce(Vref) -> Vref>(&self, f: F) -> &Self {
        self.vref_reg().with(f);
        self
    }

    #[doc="Get the XOSC Register."]
    #[inline] pub fn xosc_reg(&self) -> Register<Xosc> { 
        Register::new(self.0 as *mut Xosc, 0x10)
    }

    #[doc="Get the *mut pointer for the XOSC register."]
    #[inline] pub fn xosc_mut(&self) -> *mut Xosc { 
        self.xosc_reg().ptr()
    }

    #[doc="Get the *const pointer for the XOSC register."]
    #[inline] pub fn xosc_ptr(&self) -> *const Xosc { 
        self.xosc_reg().ptr()
    }

    #[doc="Read the XOSC register."]
    #[inline] pub fn xosc(&self) -> Xosc { 
        self.xosc_reg().read()
    }

    #[doc="Write the XOSC register."]
    #[inline] pub fn write_xosc(&self, value: Xosc) -> &Self { 
        self.xosc_reg().write(value);
        self
    }

    #[doc="Set the XOSC register."]
    #[inline] pub fn set_xosc<F: FnOnce(Xosc) -> Xosc>(&self, f: F) -> &Self {
        self.xosc_reg().set(f);
        self
    }

    #[doc="Modify the XOSC register."]
    #[inline] pub fn with_xosc<F: FnOnce(Xosc) -> Xosc>(&self, f: F) -> &Self {
        self.xosc_reg().with(f);
        self
    }

    #[doc="Get the XOSC32K Register."]
    #[inline] pub fn xosc32k_reg(&self) -> Register<Xosc32k> { 
        Register::new(self.0 as *mut Xosc32k, 0x14)
    }

    #[doc="Get the *mut pointer for the XOSC32K register."]
    #[inline] pub fn xosc32k_mut(&self) -> *mut Xosc32k { 
        self.xosc32k_reg().ptr()
    }

    #[doc="Get the *const pointer for the XOSC32K register."]
    #[inline] pub fn xosc32k_ptr(&self) -> *const Xosc32k { 
        self.xosc32k_reg().ptr()
    }

    #[doc="Read the XOSC32K register."]
    #[inline] pub fn xosc32k(&self) -> Xosc32k { 
        self.xosc32k_reg().read()
    }

    #[doc="Write the XOSC32K register."]
    #[inline] pub fn write_xosc32k(&self, value: Xosc32k) -> &Self { 
        self.xosc32k_reg().write(value);
        self
    }

    #[doc="Set the XOSC32K register."]
    #[inline] pub fn set_xosc32k<F: FnOnce(Xosc32k) -> Xosc32k>(&self, f: F) -> &Self {
        self.xosc32k_reg().set(f);
        self
    }

    #[doc="Modify the XOSC32K register."]
    #[inline] pub fn with_xosc32k<F: FnOnce(Xosc32k) -> Xosc32k>(&self, f: F) -> &Self {
        self.xosc32k_reg().with(f);
        self
    }

}

#[doc="3.3V Brown-Out Detector (BOD33) Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bod33(pub u32);
impl Bod33 {
    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Hysteresis"]
    #[inline] pub fn hyst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HYST != 0"]
    #[inline] pub fn test_hyst(&self) -> bool {
        self.hyst() != 0
    }

    #[doc="Sets the HYST field."]
    #[inline] pub fn set_hyst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="BOD33 Action"]
    #[inline] pub fn action(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if ACTION != 0"]
    #[inline] pub fn test_action(&self) -> bool {
        self.action() != 0
    }

    #[doc="Sets the ACTION field."]
    #[inline] pub fn set_action<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Operation Mode"]
    #[inline] pub fn mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock Enable"]
    #[inline] pub fn cen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CEN != 0"]
    #[inline] pub fn test_cen(&self) -> bool {
        self.cen() != 0
    }

    #[doc="Sets the CEN field."]
    #[inline] pub fn set_cen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Prescaler Select"]
    #[inline] pub fn psel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if PSEL != 0"]
    #[inline] pub fn test_psel(&self) -> bool {
        self.psel() != 0
    }

    #[doc="Sets the PSEL field."]
    #[inline] pub fn set_psel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="BOD33 Threshold Level"]
    #[inline] pub fn level(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if LEVEL != 0"]
    #[inline] pub fn test_level(&self) -> bool {
        self.level() != 0
    }

    #[doc="Sets the LEVEL field."]
    #[inline] pub fn set_level<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Bod33 {
    #[inline]
    fn from(other: u32) -> Self {
         Bod33(other)
    }
}

impl ::core::fmt::Display for Bod33 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bod33 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.hyst() != 0 { try!(write!(f, " hyst"))}
        if self.action() != 0 { try!(write!(f, " action=0x{:x}", self.action()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.mode() != 0 { try!(write!(f, " mode"))}
        if self.cen() != 0 { try!(write!(f, " cen"))}
        if self.psel() != 0 { try!(write!(f, " psel=0x{:x}", self.psel()))}
        if self.level() != 0 { try!(write!(f, " level=0x{:x}", self.level()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DFLL48M Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfllctrl(pub u16);
impl Dfllctrl {
    #[doc="DFLL Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Operating Mode Selection"]
    #[inline] pub fn mode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Stable DFLL Frequency"]
    #[inline] pub fn stable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STABLE != 0"]
    #[inline] pub fn test_stable(&self) -> bool {
        self.stable() != 0
    }

    #[doc="Sets the STABLE field."]
    #[inline] pub fn set_stable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Lose Lock After Wake"]
    #[inline] pub fn llaw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LLAW != 0"]
    #[inline] pub fn test_llaw(&self) -> bool {
        self.llaw() != 0
    }

    #[doc="Sets the LLAW field."]
    #[inline] pub fn set_llaw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="USB Clock Recovery Mode"]
    #[inline] pub fn usbcrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if USBCRM != 0"]
    #[inline] pub fn test_usbcrm(&self) -> bool {
        self.usbcrm() != 0
    }

    #[doc="Sets the USBCRM field."]
    #[inline] pub fn set_usbcrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="On Demand Control"]
    #[inline] pub fn ondemand(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Chill Cycle Disable"]
    #[inline] pub fn ccdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CCDIS != 0"]
    #[inline] pub fn test_ccdis(&self) -> bool {
        self.ccdis() != 0
    }

    #[doc="Sets the CCDIS field."]
    #[inline] pub fn set_ccdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Quick Lock Disable"]
    #[inline] pub fn qldis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if QLDIS != 0"]
    #[inline] pub fn test_qldis(&self) -> bool {
        self.qldis() != 0
    }

    #[doc="Sets the QLDIS field."]
    #[inline] pub fn set_qldis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Bypass Coarse Lock"]
    #[inline] pub fn bplckc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BPLCKC != 0"]
    #[inline] pub fn test_bplckc(&self) -> bool {
        self.bplckc() != 0
    }

    #[doc="Sets the BPLCKC field."]
    #[inline] pub fn set_bplckc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Wait Lock"]
    #[inline] pub fn waitlock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WAITLOCK != 0"]
    #[inline] pub fn test_waitlock(&self) -> bool {
        self.waitlock() != 0
    }

    #[doc="Sets the WAITLOCK field."]
    #[inline] pub fn set_waitlock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u16> for Dfllctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Dfllctrl(other)
    }
}

impl ::core::fmt::Display for Dfllctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dfllctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.mode() != 0 { try!(write!(f, " mode"))}
        if self.stable() != 0 { try!(write!(f, " stable"))}
        if self.llaw() != 0 { try!(write!(f, " llaw"))}
        if self.usbcrm() != 0 { try!(write!(f, " usbcrm"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        if self.ccdis() != 0 { try!(write!(f, " ccdis"))}
        if self.qldis() != 0 { try!(write!(f, " qldis"))}
        if self.bplckc() != 0 { try!(write!(f, " bplckc"))}
        if self.waitlock() != 0 { try!(write!(f, " waitlock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DFLL48M Multiplier"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfllmul(pub u32);
impl Dfllmul {
    #[doc="DFLL Multiply Factor"]
    #[inline] pub fn mul(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MUL != 0"]
    #[inline] pub fn test_mul(&self) -> bool {
        self.mul() != 0
    }

    #[doc="Sets the MUL field."]
    #[inline] pub fn set_mul<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fine Maximum Step"]
    #[inline] pub fn fstep(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3ff) as u16) } // [25:16]
    }

    #[doc="Returns true if FSTEP != 0"]
    #[inline] pub fn test_fstep(&self) -> bool {
        self.fstep() != 0
    }

    #[doc="Sets the FSTEP field."]
    #[inline] pub fn set_fstep<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Coarse Maximum Step"]
    #[inline] pub fn cstep(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
    }

    #[doc="Returns true if CSTEP != 0"]
    #[inline] pub fn test_cstep(&self) -> bool {
        self.cstep() != 0
    }

    #[doc="Sets the CSTEP field."]
    #[inline] pub fn set_cstep<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
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
    #[doc="Read Request"]
    #[inline] pub fn readreq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if READREQ != 0"]
    #[inline] pub fn test_readreq(&self) -> bool {
        self.readreq() != 0
    }

    #[doc="Sets the READREQ field."]
    #[inline] pub fn set_readreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.readreq() != 0 { try!(write!(f, " readreq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DFLL48M Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dfllval(pub u32);
impl Dfllval {
    #[doc="Fine Value"]
    #[inline] pub fn fine(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if FINE != 0"]
    #[inline] pub fn test_fine(&self) -> bool {
        self.fine() != 0
    }

    #[doc="Sets the FINE field."]
    #[inline] pub fn set_fine<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Coarse Value"]
    #[inline] pub fn coarse(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3f) as u8) } // [15:10]
    }

    #[doc="Returns true if COARSE != 0"]
    #[inline] pub fn test_coarse(&self) -> bool {
        self.coarse() != 0
    }

    #[doc="Sets the COARSE field."]
    #[inline] pub fn set_coarse<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Multiplication Ratio Difference"]
    #[inline] pub fn diff(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if DIFF != 0"]
    #[inline] pub fn test_diff(&self) -> bool {
        self.diff() != 0
    }

    #[doc="Sets the DIFF field."]
    #[inline] pub fn set_diff<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
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

#[doc="DPLL Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllctrla(pub u8);
impl Dpllctrla {
    #[doc="DPLL Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="On Demand Clock Activation"]
    #[inline] pub fn ondemand(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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

#[doc="DPLL Control B"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllctrlb(pub u32);
impl Dpllctrlb {
    #[doc="Proportional Integral Filter Selection"]
    #[inline] pub fn filter(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if FILTER != 0"]
    #[inline] pub fn test_filter(&self) -> bool {
        self.filter() != 0
    }

    #[doc="Sets the FILTER field."]
    #[inline] pub fn set_filter<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low-Power Enable"]
    #[inline] pub fn lpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LPEN != 0"]
    #[inline] pub fn test_lpen(&self) -> bool {
        self.lpen() != 0
    }

    #[doc="Sets the LPEN field."]
    #[inline] pub fn set_lpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Wake Up Fast"]
    #[inline] pub fn wuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WUF != 0"]
    #[inline] pub fn test_wuf(&self) -> bool {
        self.wuf() != 0
    }

    #[doc="Sets the WUF field."]
    #[inline] pub fn set_wuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Reference Clock Selection"]
    #[inline] pub fn refclk(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if REFCLK != 0"]
    #[inline] pub fn test_refclk(&self) -> bool {
        self.refclk() != 0
    }

    #[doc="Sets the REFCLK field."]
    #[inline] pub fn set_refclk<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Lock Time"]
    #[inline] pub fn ltime(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if LTIME != 0"]
    #[inline] pub fn test_ltime(&self) -> bool {
        self.ltime() != 0
    }

    #[doc="Sets the LTIME field."]
    #[inline] pub fn set_ltime<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Lock Bypass"]
    #[inline] pub fn lbypass(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if LBYPASS != 0"]
    #[inline] pub fn test_lbypass(&self) -> bool {
        self.lbypass() != 0
    }

    #[doc="Sets the LBYPASS field."]
    #[inline] pub fn set_lbypass<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Clock Divider"]
    #[inline] pub fn div(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7ff) as u16) } // [26:16]
    }

    #[doc="Returns true if DIV != 0"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Sets the DIV field."]
    #[inline] pub fn set_div<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
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
        if self.lpen() != 0 { try!(write!(f, " lpen"))}
        if self.wuf() != 0 { try!(write!(f, " wuf"))}
        if self.refclk() != 0 { try!(write!(f, " refclk=0x{:x}", self.refclk()))}
        if self.ltime() != 0 { try!(write!(f, " ltime=0x{:x}", self.ltime()))}
        if self.lbypass() != 0 { try!(write!(f, " lbypass"))}
        if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DPLL Ratio Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllratio(pub u32);
impl Dpllratio {
    #[doc="Loop Divider Ratio"]
    #[inline] pub fn ldr(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if LDR != 0"]
    #[inline] pub fn test_ldr(&self) -> bool {
        self.ldr() != 0
    }

    #[doc="Sets the LDR field."]
    #[inline] pub fn set_ldr<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Loop Divider Ratio Fractional Part"]
    #[inline] pub fn ldrfrac(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if LDRFRAC != 0"]
    #[inline] pub fn test_ldrfrac(&self) -> bool {
        self.ldrfrac() != 0
    }

    #[doc="Sets the LDRFRAC field."]
    #[inline] pub fn set_ldrfrac<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
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

#[doc="DPLL Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dpllstatus(pub u8);
impl Dpllstatus {
    #[doc="DPLL Lock Status"]
    #[inline] pub fn lock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Output Clock Ready"]
    #[inline] pub fn clkrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CLKRDY != 0"]
    #[inline] pub fn test_clkrdy(&self) -> bool {
        self.clkrdy() != 0
    }

    #[doc="Sets the CLKRDY field."]
    #[inline] pub fn set_clkrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DPLL Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Divider Enable"]
    #[inline] pub fn div(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DIV != 0"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Sets the DIV field."]
    #[inline] pub fn set_div<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Dpllstatus {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.div() != 0 { try!(write!(f, " div"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc="XOSC Ready Interrupt Enable"]
    #[inline] pub fn xoscrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSCRDY != 0"]
    #[inline] pub fn test_xoscrdy(&self) -> bool {
        self.xoscrdy() != 0
    }

    #[doc="Sets the XOSCRDY field."]
    #[inline] pub fn set_xoscrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC32K Ready Interrupt Enable"]
    #[inline] pub fn xosc32krdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if XOSC32KRDY != 0"]
    #[inline] pub fn test_xosc32krdy(&self) -> bool {
        self.xosc32krdy() != 0
    }

    #[doc="Sets the XOSC32KRDY field."]
    #[inline] pub fn set_xosc32krdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="OSC32K Ready Interrupt Enable"]
    #[inline] pub fn osc32krdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OSC32KRDY != 0"]
    #[inline] pub fn test_osc32krdy(&self) -> bool {
        self.osc32krdy() != 0
    }

    #[doc="Sets the OSC32KRDY field."]
    #[inline] pub fn set_osc32krdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="OSC8M Ready Interrupt Enable"]
    #[inline] pub fn osc8mrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OSC8MRDY != 0"]
    #[inline] pub fn test_osc8mrdy(&self) -> bool {
        self.osc8mrdy() != 0
    }

    #[doc="Sets the OSC8MRDY field."]
    #[inline] pub fn set_osc8mrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DFLL Ready Interrupt Enable"]
    #[inline] pub fn dfllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DFLLRDY != 0"]
    #[inline] pub fn test_dfllrdy(&self) -> bool {
        self.dfllrdy() != 0
    }

    #[doc="Sets the DFLLRDY field."]
    #[inline] pub fn set_dfllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DFLL Out Of Bounds Interrupt Enable"]
    #[inline] pub fn dflloob(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DFLLOOB != 0"]
    #[inline] pub fn test_dflloob(&self) -> bool {
        self.dflloob() != 0
    }

    #[doc="Sets the DFLLOOB field."]
    #[inline] pub fn set_dflloob<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="DFLL Lock Fine Interrupt Enable"]
    #[inline] pub fn dflllckf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DFLLLCKF != 0"]
    #[inline] pub fn test_dflllckf(&self) -> bool {
        self.dflllckf() != 0
    }

    #[doc="Sets the DFLLLCKF field."]
    #[inline] pub fn set_dflllckf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DFLL Lock Coarse Interrupt Enable"]
    #[inline] pub fn dflllckc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DFLLLCKC != 0"]
    #[inline] pub fn test_dflllckc(&self) -> bool {
        self.dflllckc() != 0
    }

    #[doc="Sets the DFLLLCKC field."]
    #[inline] pub fn set_dflllckc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline] pub fn dfllrcs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DFLLRCS != 0"]
    #[inline] pub fn test_dfllrcs(&self) -> bool {
        self.dfllrcs() != 0
    }

    #[doc="Sets the DFLLRCS field."]
    #[inline] pub fn set_dfllrcs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BOD33 Ready Interrupt Enable"]
    #[inline] pub fn bod33rdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BOD33RDY != 0"]
    #[inline] pub fn test_bod33rdy(&self) -> bool {
        self.bod33rdy() != 0
    }

    #[doc="Sets the BOD33RDY field."]
    #[inline] pub fn set_bod33rdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BOD33 Detection Interrupt Enable"]
    #[inline] pub fn bod33det(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BOD33DET != 0"]
    #[inline] pub fn test_bod33det(&self) -> bool {
        self.bod33det() != 0
    }

    #[doc="Sets the BOD33DET field."]
    #[inline] pub fn set_bod33det<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BOD33 Synchronization Ready Interrupt Enable"]
    #[inline] pub fn b33srdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if B33SRDY != 0"]
    #[inline] pub fn test_b33srdy(&self) -> bool {
        self.b33srdy() != 0
    }

    #[doc="Sets the B33SRDY field."]
    #[inline] pub fn set_b33srdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="DPLL Lock Rise Interrupt Enable"]
    #[inline] pub fn dplllckr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DPLLLCKR != 0"]
    #[inline] pub fn test_dplllckr(&self) -> bool {
        self.dplllckr() != 0
    }

    #[doc="Sets the DPLLLCKR field."]
    #[inline] pub fn set_dplllckr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="DPLL Lock Fall Interrupt Enable"]
    #[inline] pub fn dplllckf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLLLCKF != 0"]
    #[inline] pub fn test_dplllckf(&self) -> bool {
        self.dplllckf() != 0
    }

    #[doc="Sets the DPLLLCKF field."]
    #[inline] pub fn set_dplllckf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DPLL Lock Timeout Interrupt Enable"]
    #[inline] pub fn dplllto(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLLLTO != 0"]
    #[inline] pub fn test_dplllto(&self) -> bool {
        self.dplllto() != 0
    }

    #[doc="Sets the DPLLLTO field."]
    #[inline] pub fn set_dplllto<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
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
        if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
        if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
        if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
        if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
        if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
        if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
        if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
        if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
        if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
        if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
        if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
        if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
        if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
        if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
        if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc="XOSC Ready Interrupt Enable"]
    #[inline] pub fn xoscrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSCRDY != 0"]
    #[inline] pub fn test_xoscrdy(&self) -> bool {
        self.xoscrdy() != 0
    }

    #[doc="Sets the XOSCRDY field."]
    #[inline] pub fn set_xoscrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC32K Ready Interrupt Enable"]
    #[inline] pub fn xosc32krdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if XOSC32KRDY != 0"]
    #[inline] pub fn test_xosc32krdy(&self) -> bool {
        self.xosc32krdy() != 0
    }

    #[doc="Sets the XOSC32KRDY field."]
    #[inline] pub fn set_xosc32krdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="OSC32K Ready Interrupt Enable"]
    #[inline] pub fn osc32krdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OSC32KRDY != 0"]
    #[inline] pub fn test_osc32krdy(&self) -> bool {
        self.osc32krdy() != 0
    }

    #[doc="Sets the OSC32KRDY field."]
    #[inline] pub fn set_osc32krdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="OSC8M Ready Interrupt Enable"]
    #[inline] pub fn osc8mrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OSC8MRDY != 0"]
    #[inline] pub fn test_osc8mrdy(&self) -> bool {
        self.osc8mrdy() != 0
    }

    #[doc="Sets the OSC8MRDY field."]
    #[inline] pub fn set_osc8mrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DFLL Ready Interrupt Enable"]
    #[inline] pub fn dfllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DFLLRDY != 0"]
    #[inline] pub fn test_dfllrdy(&self) -> bool {
        self.dfllrdy() != 0
    }

    #[doc="Sets the DFLLRDY field."]
    #[inline] pub fn set_dfllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DFLL Out Of Bounds Interrupt Enable"]
    #[inline] pub fn dflloob(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DFLLOOB != 0"]
    #[inline] pub fn test_dflloob(&self) -> bool {
        self.dflloob() != 0
    }

    #[doc="Sets the DFLLOOB field."]
    #[inline] pub fn set_dflloob<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="DFLL Lock Fine Interrupt Enable"]
    #[inline] pub fn dflllckf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DFLLLCKF != 0"]
    #[inline] pub fn test_dflllckf(&self) -> bool {
        self.dflllckf() != 0
    }

    #[doc="Sets the DFLLLCKF field."]
    #[inline] pub fn set_dflllckf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DFLL Lock Coarse Interrupt Enable"]
    #[inline] pub fn dflllckc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DFLLLCKC != 0"]
    #[inline] pub fn test_dflllckc(&self) -> bool {
        self.dflllckc() != 0
    }

    #[doc="Sets the DFLLLCKC field."]
    #[inline] pub fn set_dflllckc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline] pub fn dfllrcs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DFLLRCS != 0"]
    #[inline] pub fn test_dfllrcs(&self) -> bool {
        self.dfllrcs() != 0
    }

    #[doc="Sets the DFLLRCS field."]
    #[inline] pub fn set_dfllrcs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BOD33 Ready Interrupt Enable"]
    #[inline] pub fn bod33rdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BOD33RDY != 0"]
    #[inline] pub fn test_bod33rdy(&self) -> bool {
        self.bod33rdy() != 0
    }

    #[doc="Sets the BOD33RDY field."]
    #[inline] pub fn set_bod33rdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BOD33 Detection Interrupt Enable"]
    #[inline] pub fn bod33det(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BOD33DET != 0"]
    #[inline] pub fn test_bod33det(&self) -> bool {
        self.bod33det() != 0
    }

    #[doc="Sets the BOD33DET field."]
    #[inline] pub fn set_bod33det<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BOD33 Synchronization Ready Interrupt Enable"]
    #[inline] pub fn b33srdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if B33SRDY != 0"]
    #[inline] pub fn test_b33srdy(&self) -> bool {
        self.b33srdy() != 0
    }

    #[doc="Sets the B33SRDY field."]
    #[inline] pub fn set_b33srdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="DPLL Lock Rise Interrupt Enable"]
    #[inline] pub fn dplllckr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DPLLLCKR != 0"]
    #[inline] pub fn test_dplllckr(&self) -> bool {
        self.dplllckr() != 0
    }

    #[doc="Sets the DPLLLCKR field."]
    #[inline] pub fn set_dplllckr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="DPLL Lock Fall Interrupt Enable"]
    #[inline] pub fn dplllckf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLLLCKF != 0"]
    #[inline] pub fn test_dplllckf(&self) -> bool {
        self.dplllckf() != 0
    }

    #[doc="Sets the DPLLLCKF field."]
    #[inline] pub fn set_dplllckf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DPLL Lock Timeout Interrupt Enable"]
    #[inline] pub fn dplllto(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLLLTO != 0"]
    #[inline] pub fn test_dplllto(&self) -> bool {
        self.dplllto() != 0
    }

    #[doc="Sets the DPLLLTO field."]
    #[inline] pub fn set_dplllto<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
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
        if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
        if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
        if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
        if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
        if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
        if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
        if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
        if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
        if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
        if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
        if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
        if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
        if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
        if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
        if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u32);
impl Intflag {
    #[doc="XOSC Ready"]
    #[inline] pub fn xoscrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSCRDY != 0"]
    #[inline] pub fn test_xoscrdy(&self) -> bool {
        self.xoscrdy() != 0
    }

    #[doc="Sets the XOSCRDY field."]
    #[inline] pub fn set_xoscrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC32K Ready"]
    #[inline] pub fn xosc32krdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if XOSC32KRDY != 0"]
    #[inline] pub fn test_xosc32krdy(&self) -> bool {
        self.xosc32krdy() != 0
    }

    #[doc="Sets the XOSC32KRDY field."]
    #[inline] pub fn set_xosc32krdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="OSC32K Ready"]
    #[inline] pub fn osc32krdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OSC32KRDY != 0"]
    #[inline] pub fn test_osc32krdy(&self) -> bool {
        self.osc32krdy() != 0
    }

    #[doc="Sets the OSC32KRDY field."]
    #[inline] pub fn set_osc32krdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="OSC8M Ready"]
    #[inline] pub fn osc8mrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OSC8MRDY != 0"]
    #[inline] pub fn test_osc8mrdy(&self) -> bool {
        self.osc8mrdy() != 0
    }

    #[doc="Sets the OSC8MRDY field."]
    #[inline] pub fn set_osc8mrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DFLL Ready"]
    #[inline] pub fn dfllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DFLLRDY != 0"]
    #[inline] pub fn test_dfllrdy(&self) -> bool {
        self.dfllrdy() != 0
    }

    #[doc="Sets the DFLLRDY field."]
    #[inline] pub fn set_dfllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DFLL Out Of Bounds"]
    #[inline] pub fn dflloob(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DFLLOOB != 0"]
    #[inline] pub fn test_dflloob(&self) -> bool {
        self.dflloob() != 0
    }

    #[doc="Sets the DFLLOOB field."]
    #[inline] pub fn set_dflloob<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="DFLL Lock Fine"]
    #[inline] pub fn dflllckf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DFLLLCKF != 0"]
    #[inline] pub fn test_dflllckf(&self) -> bool {
        self.dflllckf() != 0
    }

    #[doc="Sets the DFLLLCKF field."]
    #[inline] pub fn set_dflllckf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DFLL Lock Coarse"]
    #[inline] pub fn dflllckc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DFLLLCKC != 0"]
    #[inline] pub fn test_dflllckc(&self) -> bool {
        self.dflllckc() != 0
    }

    #[doc="Sets the DFLLLCKC field."]
    #[inline] pub fn set_dflllckc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DFLL Reference Clock Stopped"]
    #[inline] pub fn dfllrcs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DFLLRCS != 0"]
    #[inline] pub fn test_dfllrcs(&self) -> bool {
        self.dfllrcs() != 0
    }

    #[doc="Sets the DFLLRCS field."]
    #[inline] pub fn set_dfllrcs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BOD33 Ready"]
    #[inline] pub fn bod33rdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BOD33RDY != 0"]
    #[inline] pub fn test_bod33rdy(&self) -> bool {
        self.bod33rdy() != 0
    }

    #[doc="Sets the BOD33RDY field."]
    #[inline] pub fn set_bod33rdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BOD33 Detection"]
    #[inline] pub fn bod33det(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BOD33DET != 0"]
    #[inline] pub fn test_bod33det(&self) -> bool {
        self.bod33det() != 0
    }

    #[doc="Sets the BOD33DET field."]
    #[inline] pub fn set_bod33det<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BOD33 Synchronization Ready"]
    #[inline] pub fn b33srdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if B33SRDY != 0"]
    #[inline] pub fn test_b33srdy(&self) -> bool {
        self.b33srdy() != 0
    }

    #[doc="Sets the B33SRDY field."]
    #[inline] pub fn set_b33srdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="DPLL Lock Rise"]
    #[inline] pub fn dplllckr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DPLLLCKR != 0"]
    #[inline] pub fn test_dplllckr(&self) -> bool {
        self.dplllckr() != 0
    }

    #[doc="Sets the DPLLLCKR field."]
    #[inline] pub fn set_dplllckr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="DPLL Lock Fall"]
    #[inline] pub fn dplllckf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLLLCKF != 0"]
    #[inline] pub fn test_dplllckf(&self) -> bool {
        self.dplllckf() != 0
    }

    #[doc="Sets the DPLLLCKF field."]
    #[inline] pub fn set_dplllckf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DPLL Lock Timeout"]
    #[inline] pub fn dplllto(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLLLTO != 0"]
    #[inline] pub fn test_dplllto(&self) -> bool {
        self.dplllto() != 0
    }

    #[doc="Sets the DPLLLTO field."]
    #[inline] pub fn set_dplllto<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
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
        if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
        if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
        if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
        if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
        if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
        if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
        if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
        if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
        if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
        if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
        if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
        if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
        if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
        if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
        if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Osculp32k(pub u8);
impl Osculp32k {
    #[doc="Oscillator Calibration"]
    #[inline] pub fn calib(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if CALIB != 0"]
    #[inline] pub fn test_calib(&self) -> bool {
        self.calib() != 0
    }

    #[doc="Sets the CALIB field."]
    #[inline] pub fn set_calib<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Write Lock"]
    #[inline] pub fn wrtlock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if WRTLOCK != 0"]
    #[inline] pub fn test_wrtlock(&self) -> bool {
        self.wrtlock() != 0
    }

    #[doc="Sets the WRTLOCK field."]
    #[inline] pub fn set_wrtlock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Osculp32k {
    #[inline]
    fn from(other: u8) -> Self {
         Osculp32k(other)
    }
}

impl ::core::fmt::Display for Osculp32k {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Osculp32k {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
        if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="8MHz Internal Oscillator (OSC8M) Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Osc8m(pub u32);
impl Osc8m {
    #[doc="Oscillator Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="On Demand Control"]
    #[inline] pub fn ondemand(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Oscillator Prescaler"]
    #[inline] pub fn presc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if PRESC != 0"]
    #[inline] pub fn test_presc(&self) -> bool {
        self.presc() != 0
    }

    #[doc="Sets the PRESC field."]
    #[inline] pub fn set_presc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Oscillator Calibration"]
    #[inline] pub fn calib(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
    }

    #[doc="Returns true if CALIB != 0"]
    #[inline] pub fn test_calib(&self) -> bool {
        self.calib() != 0
    }

    #[doc="Sets the CALIB field."]
    #[inline] pub fn set_calib<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Oscillator Frequency Range"]
    #[inline] pub fn frange(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if FRANGE != 0"]
    #[inline] pub fn test_frange(&self) -> bool {
        self.frange() != 0
    }

    #[doc="Sets the FRANGE field."]
    #[inline] pub fn set_frange<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Osc8m {
    #[inline]
    fn from(other: u32) -> Self {
         Osc8m(other)
    }
}

impl ::core::fmt::Display for Osc8m {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Osc8m {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
        if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
        if self.frange() != 0 { try!(write!(f, " frange=0x{:x}", self.frange()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="32kHz Internal Oscillator (OSC32K) Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Osc32k(pub u32);
impl Osc32k {
    #[doc="Oscillator Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="32kHz Output Enable"]
    #[inline] pub fn en32k(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EN32K != 0"]
    #[inline] pub fn test_en32k(&self) -> bool {
        self.en32k() != 0
    }

    #[doc="Sets the EN32K field."]
    #[inline] pub fn set_en32k<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="1kHz Output Enable"]
    #[inline] pub fn en1k(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EN1K != 0"]
    #[inline] pub fn test_en1k(&self) -> bool {
        self.en1k() != 0
    }

    #[doc="Sets the EN1K field."]
    #[inline] pub fn set_en1k<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="On Demand Control"]
    #[inline] pub fn ondemand(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Oscillator Start-Up Time"]
    #[inline] pub fn startup(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if STARTUP != 0"]
    #[inline] pub fn test_startup(&self) -> bool {
        self.startup() != 0
    }

    #[doc="Sets the STARTUP field."]
    #[inline] pub fn set_startup<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Write Lock"]
    #[inline] pub fn wrtlock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WRTLOCK != 0"]
    #[inline] pub fn test_wrtlock(&self) -> bool {
        self.wrtlock() != 0
    }

    #[doc="Sets the WRTLOCK field."]
    #[inline] pub fn set_wrtlock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Oscillator Calibration"]
    #[inline] pub fn calib(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
    }

    #[doc="Returns true if CALIB != 0"]
    #[inline] pub fn test_calib(&self) -> bool {
        self.calib() != 0
    }

    #[doc="Sets the CALIB field."]
    #[inline] pub fn set_calib<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Osc32k {
    #[inline]
    fn from(other: u32) -> Self {
         Osc32k(other)
    }
}

impl ::core::fmt::Display for Osc32k {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Osc32k {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.en32k() != 0 { try!(write!(f, " en32k"))}
        if self.en1k() != 0 { try!(write!(f, " en1k"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
        if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
        if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Power and Clocks Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pclksr(pub u32);
impl Pclksr {
    #[doc="XOSC Ready"]
    #[inline] pub fn xoscrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if XOSCRDY != 0"]
    #[inline] pub fn test_xoscrdy(&self) -> bool {
        self.xoscrdy() != 0
    }

    #[doc="Sets the XOSCRDY field."]
    #[inline] pub fn set_xoscrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="XOSC32K Ready"]
    #[inline] pub fn xosc32krdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if XOSC32KRDY != 0"]
    #[inline] pub fn test_xosc32krdy(&self) -> bool {
        self.xosc32krdy() != 0
    }

    #[doc="Sets the XOSC32KRDY field."]
    #[inline] pub fn set_xosc32krdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="OSC32K Ready"]
    #[inline] pub fn osc32krdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OSC32KRDY != 0"]
    #[inline] pub fn test_osc32krdy(&self) -> bool {
        self.osc32krdy() != 0
    }

    #[doc="Sets the OSC32KRDY field."]
    #[inline] pub fn set_osc32krdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="OSC8M Ready"]
    #[inline] pub fn osc8mrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OSC8MRDY != 0"]
    #[inline] pub fn test_osc8mrdy(&self) -> bool {
        self.osc8mrdy() != 0
    }

    #[doc="Sets the OSC8MRDY field."]
    #[inline] pub fn set_osc8mrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DFLL Ready"]
    #[inline] pub fn dfllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DFLLRDY != 0"]
    #[inline] pub fn test_dfllrdy(&self) -> bool {
        self.dfllrdy() != 0
    }

    #[doc="Sets the DFLLRDY field."]
    #[inline] pub fn set_dfllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="DFLL Out Of Bounds"]
    #[inline] pub fn dflloob(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DFLLOOB != 0"]
    #[inline] pub fn test_dflloob(&self) -> bool {
        self.dflloob() != 0
    }

    #[doc="Sets the DFLLOOB field."]
    #[inline] pub fn set_dflloob<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="DFLL Lock Fine"]
    #[inline] pub fn dflllckf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DFLLLCKF != 0"]
    #[inline] pub fn test_dflllckf(&self) -> bool {
        self.dflllckf() != 0
    }

    #[doc="Sets the DFLLLCKF field."]
    #[inline] pub fn set_dflllckf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DFLL Lock Coarse"]
    #[inline] pub fn dflllckc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DFLLLCKC != 0"]
    #[inline] pub fn test_dflllckc(&self) -> bool {
        self.dflllckc() != 0
    }

    #[doc="Sets the DFLLLCKC field."]
    #[inline] pub fn set_dflllckc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DFLL Reference Clock Stopped"]
    #[inline] pub fn dfllrcs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DFLLRCS != 0"]
    #[inline] pub fn test_dfllrcs(&self) -> bool {
        self.dfllrcs() != 0
    }

    #[doc="Sets the DFLLRCS field."]
    #[inline] pub fn set_dfllrcs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BOD33 Ready"]
    #[inline] pub fn bod33rdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BOD33RDY != 0"]
    #[inline] pub fn test_bod33rdy(&self) -> bool {
        self.bod33rdy() != 0
    }

    #[doc="Sets the BOD33RDY field."]
    #[inline] pub fn set_bod33rdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="BOD33 Detection"]
    #[inline] pub fn bod33det(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BOD33DET != 0"]
    #[inline] pub fn test_bod33det(&self) -> bool {
        self.bod33det() != 0
    }

    #[doc="Sets the BOD33DET field."]
    #[inline] pub fn set_bod33det<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="BOD33 Synchronization Ready"]
    #[inline] pub fn b33srdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if B33SRDY != 0"]
    #[inline] pub fn test_b33srdy(&self) -> bool {
        self.b33srdy() != 0
    }

    #[doc="Sets the B33SRDY field."]
    #[inline] pub fn set_b33srdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="DPLL Lock Rise"]
    #[inline] pub fn dplllckr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DPLLLCKR != 0"]
    #[inline] pub fn test_dplllckr(&self) -> bool {
        self.dplllckr() != 0
    }

    #[doc="Sets the DPLLLCKR field."]
    #[inline] pub fn set_dplllckr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="DPLL Lock Fall"]
    #[inline] pub fn dplllckf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DPLLLCKF != 0"]
    #[inline] pub fn test_dplllckf(&self) -> bool {
        self.dplllckf() != 0
    }

    #[doc="Sets the DPLLLCKF field."]
    #[inline] pub fn set_dplllckf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DPLL Lock Timeout"]
    #[inline] pub fn dplllto(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DPLLLTO != 0"]
    #[inline] pub fn test_dplllto(&self) -> bool {
        self.dplllto() != 0
    }

    #[doc="Sets the DPLLLTO field."]
    #[inline] pub fn set_dplllto<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Pclksr {
    #[inline]
    fn from(other: u32) -> Self {
         Pclksr(other)
    }
}

impl ::core::fmt::Display for Pclksr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pclksr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xoscrdy() != 0 { try!(write!(f, " xoscrdy"))}
        if self.xosc32krdy() != 0 { try!(write!(f, " xosc32krdy"))}
        if self.osc32krdy() != 0 { try!(write!(f, " osc32krdy"))}
        if self.osc8mrdy() != 0 { try!(write!(f, " osc8mrdy"))}
        if self.dfllrdy() != 0 { try!(write!(f, " dfllrdy"))}
        if self.dflloob() != 0 { try!(write!(f, " dflloob"))}
        if self.dflllckf() != 0 { try!(write!(f, " dflllckf"))}
        if self.dflllckc() != 0 { try!(write!(f, " dflllckc"))}
        if self.dfllrcs() != 0 { try!(write!(f, " dfllrcs"))}
        if self.bod33rdy() != 0 { try!(write!(f, " bod33rdy"))}
        if self.bod33det() != 0 { try!(write!(f, " bod33det"))}
        if self.b33srdy() != 0 { try!(write!(f, " b33srdy"))}
        if self.dplllckr() != 0 { try!(write!(f, " dplllckr"))}
        if self.dplllckf() != 0 { try!(write!(f, " dplllckf"))}
        if self.dplllto() != 0 { try!(write!(f, " dplllto"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Voltage References System (VREF) Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vref(pub u32);
impl Vref {
    #[doc="Temperature Sensor Enable"]
    #[inline] pub fn tsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TSEN != 0"]
    #[inline] pub fn test_tsen(&self) -> bool {
        self.tsen() != 0
    }

    #[doc="Sets the TSEN field."]
    #[inline] pub fn set_tsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Bandgap Output Enable"]
    #[inline] pub fn bgouten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BGOUTEN != 0"]
    #[inline] pub fn test_bgouten(&self) -> bool {
        self.bgouten() != 0
    }

    #[doc="Sets the BGOUTEN field."]
    #[inline] pub fn set_bgouten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Bandgap Voltage Generator Calibration"]
    #[inline] pub fn calib(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7ff) as u16) } // [26:16]
    }

    #[doc="Returns true if CALIB != 0"]
    #[inline] pub fn test_calib(&self) -> bool {
        self.calib() != 0
    }

    #[doc="Sets the CALIB field."]
    #[inline] pub fn set_calib<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Vref {
    #[inline]
    fn from(other: u32) -> Self {
         Vref(other)
    }
}

impl ::core::fmt::Display for Vref {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vref {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsen() != 0 { try!(write!(f, " tsen"))}
        if self.bgouten() != 0 { try!(write!(f, " bgouten"))}
        if self.calib() != 0 { try!(write!(f, " calib=0x{:x}", self.calib()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External Multipurpose Crystal Oscillator (XOSC) Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Xosc(pub u16);
impl Xosc {
    #[doc="Oscillator Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Crystal Oscillator Enable"]
    #[inline] pub fn xtalen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XTALEN != 0"]
    #[inline] pub fn test_xtalen(&self) -> bool {
        self.xtalen() != 0
    }

    #[doc="Sets the XTALEN field."]
    #[inline] pub fn set_xtalen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="On Demand Control"]
    #[inline] pub fn ondemand(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Oscillator Gain"]
    #[inline] pub fn gain(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if GAIN != 0"]
    #[inline] pub fn test_gain(&self) -> bool {
        self.gain() != 0
    }

    #[doc="Sets the GAIN field."]
    #[inline] pub fn set_gain<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Automatic Amplitude Gain Control"]
    #[inline] pub fn ampgc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if AMPGC != 0"]
    #[inline] pub fn test_ampgc(&self) -> bool {
        self.ampgc() != 0
    }

    #[doc="Sets the AMPGC field."]
    #[inline] pub fn set_ampgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Start-Up Time"]
    #[inline] pub fn startup(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if STARTUP != 0"]
    #[inline] pub fn test_startup(&self) -> bool {
        self.startup() != 0
    }

    #[doc="Sets the STARTUP field."]
    #[inline] pub fn set_startup<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Xosc {
    #[inline]
    fn from(other: u16) -> Self {
         Xosc(other)
    }
}

impl ::core::fmt::Display for Xosc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Xosc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.xtalen() != 0 { try!(write!(f, " xtalen"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        if self.gain() != 0 { try!(write!(f, " gain=0x{:x}", self.gain()))}
        if self.ampgc() != 0 { try!(write!(f, " ampgc"))}
        if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="32kHz External Crystal Oscillator (XOSC32K) Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Xosc32k(pub u16);
impl Xosc32k {
    #[doc="Oscillator Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Crystal Oscillator Enable"]
    #[inline] pub fn xtalen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if XTALEN != 0"]
    #[inline] pub fn test_xtalen(&self) -> bool {
        self.xtalen() != 0
    }

    #[doc="Sets the XTALEN field."]
    #[inline] pub fn set_xtalen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="32kHz Output Enable"]
    #[inline] pub fn en32k(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if EN32K != 0"]
    #[inline] pub fn test_en32k(&self) -> bool {
        self.en32k() != 0
    }

    #[doc="Sets the EN32K field."]
    #[inline] pub fn set_en32k<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="1kHz Output Enable"]
    #[inline] pub fn en1k(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EN1K != 0"]
    #[inline] pub fn test_en1k(&self) -> bool {
        self.en1k() != 0
    }

    #[doc="Sets the EN1K field."]
    #[inline] pub fn set_en1k<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Automatic Amplitude Control Enable"]
    #[inline] pub fn aampen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AAMPEN != 0"]
    #[inline] pub fn test_aampen(&self) -> bool {
        self.aampen() != 0
    }

    #[doc="Sets the AAMPEN field."]
    #[inline] pub fn set_aampen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="On Demand Control"]
    #[inline] pub fn ondemand(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ONDEMAND != 0"]
    #[inline] pub fn test_ondemand(&self) -> bool {
        self.ondemand() != 0
    }

    #[doc="Sets the ONDEMAND field."]
    #[inline] pub fn set_ondemand<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Oscillator Start-Up Time"]
    #[inline] pub fn startup(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if STARTUP != 0"]
    #[inline] pub fn test_startup(&self) -> bool {
        self.startup() != 0
    }

    #[doc="Sets the STARTUP field."]
    #[inline] pub fn set_startup<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Write Lock"]
    #[inline] pub fn wrtlock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if WRTLOCK != 0"]
    #[inline] pub fn test_wrtlock(&self) -> bool {
        self.wrtlock() != 0
    }

    #[doc="Sets the WRTLOCK field."]
    #[inline] pub fn set_wrtlock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Xosc32k {
    #[inline]
    fn from(other: u16) -> Self {
         Xosc32k(other)
    }
}

impl ::core::fmt::Display for Xosc32k {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Xosc32k {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.xtalen() != 0 { try!(write!(f, " xtalen"))}
        if self.en32k() != 0 { try!(write!(f, " en32k"))}
        if self.en1k() != 0 { try!(write!(f, " en1k"))}
        if self.aampen() != 0 { try!(write!(f, " aampen"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.ondemand() != 0 { try!(write!(f, " ondemand"))}
        if self.startup() != 0 { try!(write!(f, " startup=0x{:x}", self.startup()))}
        if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

