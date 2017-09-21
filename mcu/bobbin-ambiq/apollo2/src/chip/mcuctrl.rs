//! MCU Miscellaneous Control Logic
#[allow(unused_imports)] use bobbin_common::*;

periph!(MCUCTRL, Mcuctrl, 0x40020000);

#[doc="MCU Miscellaneous Control Logic"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mcuctrl(pub usize);
impl Mcuctrl {
    #[doc="Get the *mut pointer for the CHIP_INFO register."]
    #[inline] pub fn chip_info_mut(&self) -> *mut ChipInfo { 
        (self.0 + 0x0) as *mut ChipInfo
    }

    #[doc="Get the *const pointer for the CHIP_INFO register."]
    #[inline] pub fn chip_info_ptr(&self) -> *const ChipInfo { 
           self.chip_info_mut()
    }

    #[doc="Read the CHIP_INFO register."]
    #[inline] pub fn chip_info(&self) -> ChipInfo { 
        unsafe {
            read_volatile(self.chip_info_ptr())
        }
    }

    #[doc="Write the CHIP_INFO register."]
    #[inline] pub fn set_chip_info<F: FnOnce(ChipInfo) -> ChipInfo>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chip_info_mut(), f(ChipInfo(0)));
        }
        self
    }

    #[doc="Modify the CHIP_INFO register."]
    #[inline] pub fn with_chip_info<F: FnOnce(ChipInfo) -> ChipInfo>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chip_info_mut(), f(self.chip_info()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHIPID0 register."]
    #[inline] pub fn chipid0_mut(&self) -> *mut Chipid0 { 
        (self.0 + 0x4) as *mut Chipid0
    }

    #[doc="Get the *const pointer for the CHIPID0 register."]
    #[inline] pub fn chipid0_ptr(&self) -> *const Chipid0 { 
           self.chipid0_mut()
    }

    #[doc="Read the CHIPID0 register."]
    #[inline] pub fn chipid0(&self) -> Chipid0 { 
        unsafe {
            read_volatile(self.chipid0_ptr())
        }
    }

    #[doc="Write the CHIPID0 register."]
    #[inline] pub fn set_chipid0<F: FnOnce(Chipid0) -> Chipid0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chipid0_mut(), f(Chipid0(0)));
        }
        self
    }

    #[doc="Modify the CHIPID0 register."]
    #[inline] pub fn with_chipid0<F: FnOnce(Chipid0) -> Chipid0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chipid0_mut(), f(self.chipid0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHIPID1 register."]
    #[inline] pub fn chipid1_mut(&self) -> *mut Chipid1 { 
        (self.0 + 0x8) as *mut Chipid1
    }

    #[doc="Get the *const pointer for the CHIPID1 register."]
    #[inline] pub fn chipid1_ptr(&self) -> *const Chipid1 { 
           self.chipid1_mut()
    }

    #[doc="Read the CHIPID1 register."]
    #[inline] pub fn chipid1(&self) -> Chipid1 { 
        unsafe {
            read_volatile(self.chipid1_ptr())
        }
    }

    #[doc="Write the CHIPID1 register."]
    #[inline] pub fn set_chipid1<F: FnOnce(Chipid1) -> Chipid1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chipid1_mut(), f(Chipid1(0)));
        }
        self
    }

    #[doc="Modify the CHIPID1 register."]
    #[inline] pub fn with_chipid1<F: FnOnce(Chipid1) -> Chipid1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chipid1_mut(), f(self.chipid1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CHIPREV register."]
    #[inline] pub fn chiprev_mut(&self) -> *mut Chiprev { 
        (self.0 + 0xc) as *mut Chiprev
    }

    #[doc="Get the *const pointer for the CHIPREV register."]
    #[inline] pub fn chiprev_ptr(&self) -> *const Chiprev { 
           self.chiprev_mut()
    }

    #[doc="Read the CHIPREV register."]
    #[inline] pub fn chiprev(&self) -> Chiprev { 
        unsafe {
            read_volatile(self.chiprev_ptr())
        }
    }

    #[doc="Write the CHIPREV register."]
    #[inline] pub fn set_chiprev<F: FnOnce(Chiprev) -> Chiprev>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chiprev_mut(), f(Chiprev(0)));
        }
        self
    }

    #[doc="Modify the CHIPREV register."]
    #[inline] pub fn with_chiprev<F: FnOnce(Chiprev) -> Chiprev>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.chiprev_mut(), f(self.chiprev()));
        }
        self
    }

    #[doc="Get the *mut pointer for the VENDORID register."]
    #[inline] pub fn vendorid_mut(&self) -> *mut Vendorid { 
        (self.0 + 0x10) as *mut Vendorid
    }

    #[doc="Get the *const pointer for the VENDORID register."]
    #[inline] pub fn vendorid_ptr(&self) -> *const Vendorid { 
           self.vendorid_mut()
    }

    #[doc="Read the VENDORID register."]
    #[inline] pub fn vendorid(&self) -> Vendorid { 
        unsafe {
            read_volatile(self.vendorid_ptr())
        }
    }

    #[doc="Write the VENDORID register."]
    #[inline] pub fn set_vendorid<F: FnOnce(Vendorid) -> Vendorid>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vendorid_mut(), f(Vendorid(0)));
        }
        self
    }

    #[doc="Modify the VENDORID register."]
    #[inline] pub fn with_vendorid<F: FnOnce(Vendorid) -> Vendorid>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vendorid_mut(), f(self.vendorid()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DEBUGGER register."]
    #[inline] pub fn debugger_mut(&self) -> *mut Debugger { 
        (self.0 + 0x14) as *mut Debugger
    }

    #[doc="Get the *const pointer for the DEBUGGER register."]
    #[inline] pub fn debugger_ptr(&self) -> *const Debugger { 
           self.debugger_mut()
    }

    #[doc="Read the DEBUGGER register."]
    #[inline] pub fn debugger(&self) -> Debugger { 
        unsafe {
            read_volatile(self.debugger_ptr())
        }
    }

    #[doc="Write the DEBUGGER register."]
    #[inline] pub fn set_debugger<F: FnOnce(Debugger) -> Debugger>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.debugger_mut(), f(Debugger(0)));
        }
        self
    }

    #[doc="Modify the DEBUGGER register."]
    #[inline] pub fn with_debugger<F: FnOnce(Debugger) -> Debugger>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.debugger_mut(), f(self.debugger()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BUCK register."]
    #[inline] pub fn buck_mut(&self) -> *mut Buck { 
        (self.0 + 0x60) as *mut Buck
    }

    #[doc="Get the *const pointer for the BUCK register."]
    #[inline] pub fn buck_ptr(&self) -> *const Buck { 
           self.buck_mut()
    }

    #[doc="Read the BUCK register."]
    #[inline] pub fn buck(&self) -> Buck { 
        unsafe {
            read_volatile(self.buck_ptr())
        }
    }

    #[doc="Write the BUCK register."]
    #[inline] pub fn set_buck<F: FnOnce(Buck) -> Buck>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.buck_mut(), f(Buck(0)));
        }
        self
    }

    #[doc="Modify the BUCK register."]
    #[inline] pub fn with_buck<F: FnOnce(Buck) -> Buck>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.buck_mut(), f(self.buck()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BUCK2 register."]
    #[inline] pub fn buck2_mut(&self) -> *mut Buck2 { 
        (self.0 + 0x64) as *mut Buck2
    }

    #[doc="Get the *const pointer for the BUCK2 register."]
    #[inline] pub fn buck2_ptr(&self) -> *const Buck2 { 
           self.buck2_mut()
    }

    #[doc="Read the BUCK2 register."]
    #[inline] pub fn buck2(&self) -> Buck2 { 
        unsafe {
            read_volatile(self.buck2_ptr())
        }
    }

    #[doc="Write the BUCK2 register."]
    #[inline] pub fn set_buck2<F: FnOnce(Buck2) -> Buck2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.buck2_mut(), f(Buck2(0)));
        }
        self
    }

    #[doc="Modify the BUCK2 register."]
    #[inline] pub fn with_buck2<F: FnOnce(Buck2) -> Buck2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.buck2_mut(), f(self.buck2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BUCK3 register."]
    #[inline] pub fn buck3_mut(&self) -> *mut Buck3 { 
        (self.0 + 0x68) as *mut Buck3
    }

    #[doc="Get the *const pointer for the BUCK3 register."]
    #[inline] pub fn buck3_ptr(&self) -> *const Buck3 { 
           self.buck3_mut()
    }

    #[doc="Read the BUCK3 register."]
    #[inline] pub fn buck3(&self) -> Buck3 { 
        unsafe {
            read_volatile(self.buck3_ptr())
        }
    }

    #[doc="Write the BUCK3 register."]
    #[inline] pub fn set_buck3<F: FnOnce(Buck3) -> Buck3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.buck3_mut(), f(Buck3(0)));
        }
        self
    }

    #[doc="Modify the BUCK3 register."]
    #[inline] pub fn with_buck3<F: FnOnce(Buck3) -> Buck3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.buck3_mut(), f(self.buck3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LDOREG1 register."]
    #[inline] pub fn ldoreg1_mut(&self) -> *mut Ldoreg1 { 
        (self.0 + 0x80) as *mut Ldoreg1
    }

    #[doc="Get the *const pointer for the LDOREG1 register."]
    #[inline] pub fn ldoreg1_ptr(&self) -> *const Ldoreg1 { 
           self.ldoreg1_mut()
    }

    #[doc="Read the LDOREG1 register."]
    #[inline] pub fn ldoreg1(&self) -> Ldoreg1 { 
        unsafe {
            read_volatile(self.ldoreg1_ptr())
        }
    }

    #[doc="Write the LDOREG1 register."]
    #[inline] pub fn set_ldoreg1<F: FnOnce(Ldoreg1) -> Ldoreg1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ldoreg1_mut(), f(Ldoreg1(0)));
        }
        self
    }

    #[doc="Modify the LDOREG1 register."]
    #[inline] pub fn with_ldoreg1<F: FnOnce(Ldoreg1) -> Ldoreg1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ldoreg1_mut(), f(self.ldoreg1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LDOREG2 register."]
    #[inline] pub fn ldoreg2_mut(&self) -> *mut Ldoreg2 { 
        (self.0 + 0x84) as *mut Ldoreg2
    }

    #[doc="Get the *const pointer for the LDOREG2 register."]
    #[inline] pub fn ldoreg2_ptr(&self) -> *const Ldoreg2 { 
           self.ldoreg2_mut()
    }

    #[doc="Read the LDOREG2 register."]
    #[inline] pub fn ldoreg2(&self) -> Ldoreg2 { 
        unsafe {
            read_volatile(self.ldoreg2_ptr())
        }
    }

    #[doc="Write the LDOREG2 register."]
    #[inline] pub fn set_ldoreg2<F: FnOnce(Ldoreg2) -> Ldoreg2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ldoreg2_mut(), f(Ldoreg2(0)));
        }
        self
    }

    #[doc="Modify the LDOREG2 register."]
    #[inline] pub fn with_ldoreg2<F: FnOnce(Ldoreg2) -> Ldoreg2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ldoreg2_mut(), f(self.ldoreg2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LDOREG3 register."]
    #[inline] pub fn ldoreg3_mut(&self) -> *mut Ldoreg3 { 
        (self.0 + 0x88) as *mut Ldoreg3
    }

    #[doc="Get the *const pointer for the LDOREG3 register."]
    #[inline] pub fn ldoreg3_ptr(&self) -> *const Ldoreg3 { 
           self.ldoreg3_mut()
    }

    #[doc="Read the LDOREG3 register."]
    #[inline] pub fn ldoreg3(&self) -> Ldoreg3 { 
        unsafe {
            read_volatile(self.ldoreg3_ptr())
        }
    }

    #[doc="Write the LDOREG3 register."]
    #[inline] pub fn set_ldoreg3<F: FnOnce(Ldoreg3) -> Ldoreg3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ldoreg3_mut(), f(Ldoreg3(0)));
        }
        self
    }

    #[doc="Modify the LDOREG3 register."]
    #[inline] pub fn with_ldoreg3<F: FnOnce(Ldoreg3) -> Ldoreg3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ldoreg3_mut(), f(self.ldoreg3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BODPORCTRL register."]
    #[inline] pub fn bodporctrl_mut(&self) -> *mut Bodporctrl { 
        (self.0 + 0x100) as *mut Bodporctrl
    }

    #[doc="Get the *const pointer for the BODPORCTRL register."]
    #[inline] pub fn bodporctrl_ptr(&self) -> *const Bodporctrl { 
           self.bodporctrl_mut()
    }

    #[doc="Read the BODPORCTRL register."]
    #[inline] pub fn bodporctrl(&self) -> Bodporctrl { 
        unsafe {
            read_volatile(self.bodporctrl_ptr())
        }
    }

    #[doc="Write the BODPORCTRL register."]
    #[inline] pub fn set_bodporctrl<F: FnOnce(Bodporctrl) -> Bodporctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bodporctrl_mut(), f(Bodporctrl(0)));
        }
        self
    }

    #[doc="Modify the BODPORCTRL register."]
    #[inline] pub fn with_bodporctrl<F: FnOnce(Bodporctrl) -> Bodporctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bodporctrl_mut(), f(self.bodporctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADCPWRDLY register."]
    #[inline] pub fn adcpwrdly_mut(&self) -> *mut Adcpwrdly { 
        (self.0 + 0x104) as *mut Adcpwrdly
    }

    #[doc="Get the *const pointer for the ADCPWRDLY register."]
    #[inline] pub fn adcpwrdly_ptr(&self) -> *const Adcpwrdly { 
           self.adcpwrdly_mut()
    }

    #[doc="Read the ADCPWRDLY register."]
    #[inline] pub fn adcpwrdly(&self) -> Adcpwrdly { 
        unsafe {
            read_volatile(self.adcpwrdly_ptr())
        }
    }

    #[doc="Write the ADCPWRDLY register."]
    #[inline] pub fn set_adcpwrdly<F: FnOnce(Adcpwrdly) -> Adcpwrdly>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcpwrdly_mut(), f(Adcpwrdly(0)));
        }
        self
    }

    #[doc="Modify the ADCPWRDLY register."]
    #[inline] pub fn with_adcpwrdly<F: FnOnce(Adcpwrdly) -> Adcpwrdly>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcpwrdly_mut(), f(self.adcpwrdly()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADCCAL register."]
    #[inline] pub fn adccal_mut(&self) -> *mut Adccal { 
        (self.0 + 0x10c) as *mut Adccal
    }

    #[doc="Get the *const pointer for the ADCCAL register."]
    #[inline] pub fn adccal_ptr(&self) -> *const Adccal { 
           self.adccal_mut()
    }

    #[doc="Read the ADCCAL register."]
    #[inline] pub fn adccal(&self) -> Adccal { 
        unsafe {
            read_volatile(self.adccal_ptr())
        }
    }

    #[doc="Write the ADCCAL register."]
    #[inline] pub fn set_adccal<F: FnOnce(Adccal) -> Adccal>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adccal_mut(), f(Adccal(0)));
        }
        self
    }

    #[doc="Modify the ADCCAL register."]
    #[inline] pub fn with_adccal<F: FnOnce(Adccal) -> Adccal>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adccal_mut(), f(self.adccal()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADCBATTLOAD register."]
    #[inline] pub fn adcbattload_mut(&self) -> *mut Adcbattload { 
        (self.0 + 0x110) as *mut Adcbattload
    }

    #[doc="Get the *const pointer for the ADCBATTLOAD register."]
    #[inline] pub fn adcbattload_ptr(&self) -> *const Adcbattload { 
           self.adcbattload_mut()
    }

    #[doc="Read the ADCBATTLOAD register."]
    #[inline] pub fn adcbattload(&self) -> Adcbattload { 
        unsafe {
            read_volatile(self.adcbattload_ptr())
        }
    }

    #[doc="Write the ADCBATTLOAD register."]
    #[inline] pub fn set_adcbattload<F: FnOnce(Adcbattload) -> Adcbattload>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcbattload_mut(), f(Adcbattload(0)));
        }
        self
    }

    #[doc="Modify the ADCBATTLOAD register."]
    #[inline] pub fn with_adcbattload<F: FnOnce(Adcbattload) -> Adcbattload>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcbattload_mut(), f(self.adcbattload()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BUCKTRIM register."]
    #[inline] pub fn bucktrim_mut(&self) -> *mut Bucktrim { 
        (self.0 + 0x114) as *mut Bucktrim
    }

    #[doc="Get the *const pointer for the BUCKTRIM register."]
    #[inline] pub fn bucktrim_ptr(&self) -> *const Bucktrim { 
           self.bucktrim_mut()
    }

    #[doc="Read the BUCKTRIM register."]
    #[inline] pub fn bucktrim(&self) -> Bucktrim { 
        unsafe {
            read_volatile(self.bucktrim_ptr())
        }
    }

    #[doc="Write the BUCKTRIM register."]
    #[inline] pub fn set_bucktrim<F: FnOnce(Bucktrim) -> Bucktrim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bucktrim_mut(), f(Bucktrim(0)));
        }
        self
    }

    #[doc="Modify the BUCKTRIM register."]
    #[inline] pub fn with_bucktrim<F: FnOnce(Bucktrim) -> Bucktrim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bucktrim_mut(), f(self.bucktrim()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADCTRIM register."]
    #[inline] pub fn adctrim_mut(&self) -> *mut Adctrim { 
        (self.0 + 0x118) as *mut Adctrim
    }

    #[doc="Get the *const pointer for the ADCTRIM register."]
    #[inline] pub fn adctrim_ptr(&self) -> *const Adctrim { 
           self.adctrim_mut()
    }

    #[doc="Read the ADCTRIM register."]
    #[inline] pub fn adctrim(&self) -> Adctrim { 
        unsafe {
            read_volatile(self.adctrim_ptr())
        }
    }

    #[doc="Write the ADCTRIM register."]
    #[inline] pub fn set_adctrim<F: FnOnce(Adctrim) -> Adctrim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adctrim_mut(), f(Adctrim(0)));
        }
        self
    }

    #[doc="Modify the ADCTRIM register."]
    #[inline] pub fn with_adctrim<F: FnOnce(Adctrim) -> Adctrim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adctrim_mut(), f(self.adctrim()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADCREFCOMP register."]
    #[inline] pub fn adcrefcomp_mut(&self) -> *mut Adcrefcomp { 
        (self.0 + 0x11c) as *mut Adcrefcomp
    }

    #[doc="Get the *const pointer for the ADCREFCOMP register."]
    #[inline] pub fn adcrefcomp_ptr(&self) -> *const Adcrefcomp { 
           self.adcrefcomp_mut()
    }

    #[doc="Read the ADCREFCOMP register."]
    #[inline] pub fn adcrefcomp(&self) -> Adcrefcomp { 
        unsafe {
            read_volatile(self.adcrefcomp_ptr())
        }
    }

    #[doc="Write the ADCREFCOMP register."]
    #[inline] pub fn set_adcrefcomp<F: FnOnce(Adcrefcomp) -> Adcrefcomp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcrefcomp_mut(), f(Adcrefcomp(0)));
        }
        self
    }

    #[doc="Modify the ADCREFCOMP register."]
    #[inline] pub fn with_adcrefcomp<F: FnOnce(Adcrefcomp) -> Adcrefcomp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcrefcomp_mut(), f(self.adcrefcomp()));
        }
        self
    }

    #[doc="Get the *mut pointer for the XTALGENCTRL register."]
    #[inline] pub fn xtalgenctrl_mut(&self) -> *mut Xtalgenctrl { 
        (self.0 + 0x124) as *mut Xtalgenctrl
    }

    #[doc="Get the *const pointer for the XTALGENCTRL register."]
    #[inline] pub fn xtalgenctrl_ptr(&self) -> *const Xtalgenctrl { 
           self.xtalgenctrl_mut()
    }

    #[doc="Read the XTALGENCTRL register."]
    #[inline] pub fn xtalgenctrl(&self) -> Xtalgenctrl { 
        unsafe {
            read_volatile(self.xtalgenctrl_ptr())
        }
    }

    #[doc="Write the XTALGENCTRL register."]
    #[inline] pub fn set_xtalgenctrl<F: FnOnce(Xtalgenctrl) -> Xtalgenctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.xtalgenctrl_mut(), f(Xtalgenctrl(0)));
        }
        self
    }

    #[doc="Modify the XTALGENCTRL register."]
    #[inline] pub fn with_xtalgenctrl<F: FnOnce(Xtalgenctrl) -> Xtalgenctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.xtalgenctrl_mut(), f(self.xtalgenctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTCLKSEL register."]
    #[inline] pub fn extclksel_mut(&self) -> *mut Extclksel { 
        (self.0 + 0x160) as *mut Extclksel
    }

    #[doc="Get the *const pointer for the EXTCLKSEL register."]
    #[inline] pub fn extclksel_ptr(&self) -> *const Extclksel { 
           self.extclksel_mut()
    }

    #[doc="Read the EXTCLKSEL register."]
    #[inline] pub fn extclksel(&self) -> Extclksel { 
        unsafe {
            read_volatile(self.extclksel_ptr())
        }
    }

    #[doc="Write the EXTCLKSEL register."]
    #[inline] pub fn set_extclksel<F: FnOnce(Extclksel) -> Extclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extclksel_mut(), f(Extclksel(0)));
        }
        self
    }

    #[doc="Modify the EXTCLKSEL register."]
    #[inline] pub fn with_extclksel<F: FnOnce(Extclksel) -> Extclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.extclksel_mut(), f(self.extclksel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BOOTLOADERLOW register."]
    #[inline] pub fn bootloaderlow_mut(&self) -> *mut Bootloaderlow { 
        (self.0 + 0x1a0) as *mut Bootloaderlow
    }

    #[doc="Get the *const pointer for the BOOTLOADERLOW register."]
    #[inline] pub fn bootloaderlow_ptr(&self) -> *const Bootloaderlow { 
           self.bootloaderlow_mut()
    }

    #[doc="Read the BOOTLOADERLOW register."]
    #[inline] pub fn bootloaderlow(&self) -> Bootloaderlow { 
        unsafe {
            read_volatile(self.bootloaderlow_ptr())
        }
    }

    #[doc="Write the BOOTLOADERLOW register."]
    #[inline] pub fn set_bootloaderlow<F: FnOnce(Bootloaderlow) -> Bootloaderlow>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bootloaderlow_mut(), f(Bootloaderlow(0)));
        }
        self
    }

    #[doc="Modify the BOOTLOADERLOW register."]
    #[inline] pub fn with_bootloaderlow<F: FnOnce(Bootloaderlow) -> Bootloaderlow>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bootloaderlow_mut(), f(self.bootloaderlow()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SHADOWVALID register."]
    #[inline] pub fn shadowvalid_mut(&self) -> *mut Shadowvalid { 
        (self.0 + 0x1a4) as *mut Shadowvalid
    }

    #[doc="Get the *const pointer for the SHADOWVALID register."]
    #[inline] pub fn shadowvalid_ptr(&self) -> *const Shadowvalid { 
           self.shadowvalid_mut()
    }

    #[doc="Read the SHADOWVALID register."]
    #[inline] pub fn shadowvalid(&self) -> Shadowvalid { 
        unsafe {
            read_volatile(self.shadowvalid_ptr())
        }
    }

    #[doc="Write the SHADOWVALID register."]
    #[inline] pub fn set_shadowvalid<F: FnOnce(Shadowvalid) -> Shadowvalid>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.shadowvalid_mut(), f(Shadowvalid(0)));
        }
        self
    }

    #[doc="Modify the SHADOWVALID register."]
    #[inline] pub fn with_shadowvalid<F: FnOnce(Shadowvalid) -> Shadowvalid>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.shadowvalid_mut(), f(self.shadowvalid()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICODEFAULTADDR register."]
    #[inline] pub fn icodefaultaddr_mut(&self) -> *mut Icodefaultaddr { 
        (self.0 + 0x1c0) as *mut Icodefaultaddr
    }

    #[doc="Get the *const pointer for the ICODEFAULTADDR register."]
    #[inline] pub fn icodefaultaddr_ptr(&self) -> *const Icodefaultaddr { 
           self.icodefaultaddr_mut()
    }

    #[doc="Read the ICODEFAULTADDR register."]
    #[inline] pub fn icodefaultaddr(&self) -> Icodefaultaddr { 
        unsafe {
            read_volatile(self.icodefaultaddr_ptr())
        }
    }

    #[doc="Write the ICODEFAULTADDR register."]
    #[inline] pub fn set_icodefaultaddr<F: FnOnce(Icodefaultaddr) -> Icodefaultaddr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icodefaultaddr_mut(), f(Icodefaultaddr(0)));
        }
        self
    }

    #[doc="Modify the ICODEFAULTADDR register."]
    #[inline] pub fn with_icodefaultaddr<F: FnOnce(Icodefaultaddr) -> Icodefaultaddr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icodefaultaddr_mut(), f(self.icodefaultaddr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCODEFAULTADDR register."]
    #[inline] pub fn dcodefaultaddr_mut(&self) -> *mut Dcodefaultaddr { 
        (self.0 + 0x1c4) as *mut Dcodefaultaddr
    }

    #[doc="Get the *const pointer for the DCODEFAULTADDR register."]
    #[inline] pub fn dcodefaultaddr_ptr(&self) -> *const Dcodefaultaddr { 
           self.dcodefaultaddr_mut()
    }

    #[doc="Read the DCODEFAULTADDR register."]
    #[inline] pub fn dcodefaultaddr(&self) -> Dcodefaultaddr { 
        unsafe {
            read_volatile(self.dcodefaultaddr_ptr())
        }
    }

    #[doc="Write the DCODEFAULTADDR register."]
    #[inline] pub fn set_dcodefaultaddr<F: FnOnce(Dcodefaultaddr) -> Dcodefaultaddr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcodefaultaddr_mut(), f(Dcodefaultaddr(0)));
        }
        self
    }

    #[doc="Modify the DCODEFAULTADDR register."]
    #[inline] pub fn with_dcodefaultaddr<F: FnOnce(Dcodefaultaddr) -> Dcodefaultaddr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcodefaultaddr_mut(), f(self.dcodefaultaddr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SYSFAULTADDR register."]
    #[inline] pub fn sysfaultaddr_mut(&self) -> *mut Sysfaultaddr { 
        (self.0 + 0x1c8) as *mut Sysfaultaddr
    }

    #[doc="Get the *const pointer for the SYSFAULTADDR register."]
    #[inline] pub fn sysfaultaddr_ptr(&self) -> *const Sysfaultaddr { 
           self.sysfaultaddr_mut()
    }

    #[doc="Read the SYSFAULTADDR register."]
    #[inline] pub fn sysfaultaddr(&self) -> Sysfaultaddr { 
        unsafe {
            read_volatile(self.sysfaultaddr_ptr())
        }
    }

    #[doc="Write the SYSFAULTADDR register."]
    #[inline] pub fn set_sysfaultaddr<F: FnOnce(Sysfaultaddr) -> Sysfaultaddr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sysfaultaddr_mut(), f(Sysfaultaddr(0)));
        }
        self
    }

    #[doc="Modify the SYSFAULTADDR register."]
    #[inline] pub fn with_sysfaultaddr<F: FnOnce(Sysfaultaddr) -> Sysfaultaddr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sysfaultaddr_mut(), f(self.sysfaultaddr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FAULTSTATUS register."]
    #[inline] pub fn faultstatus_mut(&self) -> *mut Faultstatus { 
        (self.0 + 0x1cc) as *mut Faultstatus
    }

    #[doc="Get the *const pointer for the FAULTSTATUS register."]
    #[inline] pub fn faultstatus_ptr(&self) -> *const Faultstatus { 
           self.faultstatus_mut()
    }

    #[doc="Read the FAULTSTATUS register."]
    #[inline] pub fn faultstatus(&self) -> Faultstatus { 
        unsafe {
            read_volatile(self.faultstatus_ptr())
        }
    }

    #[doc="Write the FAULTSTATUS register."]
    #[inline] pub fn set_faultstatus<F: FnOnce(Faultstatus) -> Faultstatus>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.faultstatus_mut(), f(Faultstatus(0)));
        }
        self
    }

    #[doc="Modify the FAULTSTATUS register."]
    #[inline] pub fn with_faultstatus<F: FnOnce(Faultstatus) -> Faultstatus>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.faultstatus_mut(), f(self.faultstatus()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FAULTCAPTUREEN register."]
    #[inline] pub fn faultcaptureen_mut(&self) -> *mut Faultcaptureen { 
        (self.0 + 0x1d0) as *mut Faultcaptureen
    }

    #[doc="Get the *const pointer for the FAULTCAPTUREEN register."]
    #[inline] pub fn faultcaptureen_ptr(&self) -> *const Faultcaptureen { 
           self.faultcaptureen_mut()
    }

    #[doc="Read the FAULTCAPTUREEN register."]
    #[inline] pub fn faultcaptureen(&self) -> Faultcaptureen { 
        unsafe {
            read_volatile(self.faultcaptureen_ptr())
        }
    }

    #[doc="Write the FAULTCAPTUREEN register."]
    #[inline] pub fn set_faultcaptureen<F: FnOnce(Faultcaptureen) -> Faultcaptureen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.faultcaptureen_mut(), f(Faultcaptureen(0)));
        }
        self
    }

    #[doc="Modify the FAULTCAPTUREEN register."]
    #[inline] pub fn with_faultcaptureen<F: FnOnce(Faultcaptureen) -> Faultcaptureen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.faultcaptureen_mut(), f(self.faultcaptureen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DBGR1 register."]
    #[inline] pub fn dbgr1_mut(&self) -> *mut Dbgr1 { 
        (self.0 + 0x200) as *mut Dbgr1
    }

    #[doc="Get the *const pointer for the DBGR1 register."]
    #[inline] pub fn dbgr1_ptr(&self) -> *const Dbgr1 { 
           self.dbgr1_mut()
    }

    #[doc="Read the DBGR1 register."]
    #[inline] pub fn dbgr1(&self) -> Dbgr1 { 
        unsafe {
            read_volatile(self.dbgr1_ptr())
        }
    }

    #[doc="Write the DBGR1 register."]
    #[inline] pub fn set_dbgr1<F: FnOnce(Dbgr1) -> Dbgr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgr1_mut(), f(Dbgr1(0)));
        }
        self
    }

    #[doc="Modify the DBGR1 register."]
    #[inline] pub fn with_dbgr1<F: FnOnce(Dbgr1) -> Dbgr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgr1_mut(), f(self.dbgr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DBGR2 register."]
    #[inline] pub fn dbgr2_mut(&self) -> *mut Dbgr2 { 
        (self.0 + 0x204) as *mut Dbgr2
    }

    #[doc="Get the *const pointer for the DBGR2 register."]
    #[inline] pub fn dbgr2_ptr(&self) -> *const Dbgr2 { 
           self.dbgr2_mut()
    }

    #[doc="Read the DBGR2 register."]
    #[inline] pub fn dbgr2(&self) -> Dbgr2 { 
        unsafe {
            read_volatile(self.dbgr2_ptr())
        }
    }

    #[doc="Write the DBGR2 register."]
    #[inline] pub fn set_dbgr2<F: FnOnce(Dbgr2) -> Dbgr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgr2_mut(), f(Dbgr2(0)));
        }
        self
    }

    #[doc="Modify the DBGR2 register."]
    #[inline] pub fn with_dbgr2<F: FnOnce(Dbgr2) -> Dbgr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgr2_mut(), f(self.dbgr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PMUENABLE register."]
    #[inline] pub fn pmuenable_mut(&self) -> *mut Pmuenable { 
        (self.0 + 0x220) as *mut Pmuenable
    }

    #[doc="Get the *const pointer for the PMUENABLE register."]
    #[inline] pub fn pmuenable_ptr(&self) -> *const Pmuenable { 
           self.pmuenable_mut()
    }

    #[doc="Read the PMUENABLE register."]
    #[inline] pub fn pmuenable(&self) -> Pmuenable { 
        unsafe {
            read_volatile(self.pmuenable_ptr())
        }
    }

    #[doc="Write the PMUENABLE register."]
    #[inline] pub fn set_pmuenable<F: FnOnce(Pmuenable) -> Pmuenable>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pmuenable_mut(), f(Pmuenable(0)));
        }
        self
    }

    #[doc="Modify the PMUENABLE register."]
    #[inline] pub fn with_pmuenable<F: FnOnce(Pmuenable) -> Pmuenable>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pmuenable_mut(), f(self.pmuenable()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TPIUCTRL register."]
    #[inline] pub fn tpiuctrl_mut(&self) -> *mut Tpiuctrl { 
        (self.0 + 0x250) as *mut Tpiuctrl
    }

    #[doc="Get the *const pointer for the TPIUCTRL register."]
    #[inline] pub fn tpiuctrl_ptr(&self) -> *const Tpiuctrl { 
           self.tpiuctrl_mut()
    }

    #[doc="Read the TPIUCTRL register."]
    #[inline] pub fn tpiuctrl(&self) -> Tpiuctrl { 
        unsafe {
            read_volatile(self.tpiuctrl_ptr())
        }
    }

    #[doc="Write the TPIUCTRL register."]
    #[inline] pub fn set_tpiuctrl<F: FnOnce(Tpiuctrl) -> Tpiuctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tpiuctrl_mut(), f(Tpiuctrl(0)));
        }
        self
    }

    #[doc="Modify the TPIUCTRL register."]
    #[inline] pub fn with_tpiuctrl<F: FnOnce(Tpiuctrl) -> Tpiuctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tpiuctrl_mut(), f(self.tpiuctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the KEXTCLKSEL register."]
    #[inline] pub fn kextclksel_mut(&self) -> *mut Kextclksel { 
        (self.0 + 0x348) as *mut Kextclksel
    }

    #[doc="Get the *const pointer for the KEXTCLKSEL register."]
    #[inline] pub fn kextclksel_ptr(&self) -> *const Kextclksel { 
           self.kextclksel_mut()
    }

    #[doc="Read the KEXTCLKSEL register."]
    #[inline] pub fn kextclksel(&self) -> Kextclksel { 
        unsafe {
            read_volatile(self.kextclksel_ptr())
        }
    }

    #[doc="Write the KEXTCLKSEL register."]
    #[inline] pub fn set_kextclksel<F: FnOnce(Kextclksel) -> Kextclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.kextclksel_mut(), f(Kextclksel(0)));
        }
        self
    }

    #[doc="Modify the KEXTCLKSEL register."]
    #[inline] pub fn with_kextclksel<F: FnOnce(Kextclksel) -> Kextclksel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.kextclksel_mut(), f(self.kextclksel()));
        }
        self
    }

}

#[doc="Chip Information Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ChipInfo(pub u32);
impl ChipInfo {
    #[doc="BCD part number."]
    #[inline] pub fn partnum(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PARTNUM != 0"]
    #[inline] pub fn test_partnum(&self) -> bool {
        self.partnum() != 0
    }

    #[doc="Sets the PARTNUM field."]
    #[inline] pub fn set_partnum<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for ChipInfo {
    #[inline]
    fn from(other: u32) -> Self {
         ChipInfo(other)
    }
}

impl ::core::fmt::Display for ChipInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ChipInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Unique Chip ID 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chipid0(pub u32);
impl Chipid0 {
    #[doc="Unique chip ID 0."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chipid0 {
    #[inline]
    fn from(other: u32) -> Self {
         Chipid0(other)
    }
}

impl ::core::fmt::Display for Chipid0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chipid0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Unique Chip ID 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chipid1(pub u32);
impl Chipid1 {
    #[doc="Unique chip ID 1."]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chipid1 {
    #[inline]
    fn from(other: u32) -> Self {
         Chipid1(other)
    }
}

impl ::core::fmt::Display for Chipid1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chipid1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Chip Revision"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chiprev(pub u32);
impl Chiprev {
    #[doc="Major Revision ID."]
    #[inline] pub fn revmaj(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if REVMAJ != 0"]
    #[inline] pub fn test_revmaj(&self) -> bool {
        self.revmaj() != 0
    }

    #[doc="Sets the REVMAJ field."]
    #[inline] pub fn set_revmaj<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Minor Revision ID."]
    #[inline] pub fn revmin(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if REVMIN != 0"]
    #[inline] pub fn test_revmin(&self) -> bool {
        self.revmin() != 0
    }

    #[doc="Sets the REVMIN field."]
    #[inline] pub fn set_revmin<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Chiprev {
    #[inline]
    fn from(other: u32) -> Self {
         Chiprev(other)
    }
}

impl ::core::fmt::Display for Chiprev {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chiprev {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.revmaj() != 0 { try!(write!(f, " revmaj=0x{:x}", self.revmaj()))}
        if self.revmin() != 0 { try!(write!(f, " revmin=0x{:x}", self.revmin()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Unique Vendor ID"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vendorid(pub u32);
impl Vendorid {
    #[doc="Unique Vendor ID"]
    #[inline] pub fn value(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Vendorid {
    #[inline]
    fn from(other: u32) -> Self {
         Vendorid(other)
    }
}

impl ::core::fmt::Display for Vendorid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vendorid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debugger Access Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Debugger(pub u32);
impl Debugger {
    #[doc="Lockout of debugger (SWD)."]
    #[inline] pub fn lockout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LOCKOUT != 0"]
    #[inline] pub fn test_lockout(&self) -> bool {
        self.lockout() != 0
    }

    #[doc="Sets the LOCKOUT field."]
    #[inline] pub fn set_lockout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Debugger {
    #[inline]
    fn from(other: u32) -> Self {
         Debugger(other)
    }
}

impl ::core::fmt::Display for Debugger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Debugger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lockout() != 0 { try!(write!(f, " lockout"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog Buck Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Buck(pub u32);
impl Buck {
    #[doc="Reset control override for Mem Buck; 0=enabled, 1=reset; Value is propagated only when the BUCKSWE bit is active, otherwise contrl is from the power control module."]
    #[inline] pub fn membuckrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MEMBUCKRST != 0"]
    #[inline] pub fn test_membuckrst(&self) -> bool {
        self.membuckrst() != 0
    }

    #[doc="Sets the MEMBUCKRST field."]
    #[inline] pub fn set_membuckrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Reset control override for Core Buck; 0=enabled, 1=reset; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module."]
    #[inline] pub fn corebuckrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COREBUCKRST != 0"]
    #[inline] pub fn test_corebuckrst(&self) -> bool {
        self.corebuckrst() != 0
    }

    #[doc="Sets the COREBUCKRST field."]
    #[inline] pub fn set_corebuckrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Not used. Additional control of buck is available in the power control module"]
    #[inline] pub fn bypbuckmem(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BYPBUCKMEM != 0"]
    #[inline] pub fn test_bypbuckmem(&self) -> bool {
        self.bypbuckmem() != 0
    }

    #[doc="Sets the BYPBUCKMEM field."]
    #[inline] pub fn set_bypbuckmem<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Memory buck power down override. 1=Powered Down; 0=Enabled; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module."]
    #[inline] pub fn membuckpwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MEMBUCKPWD != 0"]
    #[inline] pub fn test_membuckpwd(&self) -> bool {
        self.membuckpwd() != 0
    }

    #[doc="Sets the MEMBUCKPWD field."]
    #[inline] pub fn set_membuckpwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="HFRC clkgen bit 0 override. When set, this will override to 0 bit 0 of the hfrc_freq_clkgen internal bus (see internal Shelby-1473)"]
    #[inline] pub fn sleepbuckana(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SLEEPBUCKANA != 0"]
    #[inline] pub fn test_sleepbuckana(&self) -> bool {
        self.sleepbuckana() != 0
    }

    #[doc="Sets the SLEEPBUCKANA field."]
    #[inline] pub fn set_sleepbuckana<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Core buck power down override. 1=Powered Down; 0=Enabled; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module."]
    #[inline] pub fn corebuckpwd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if COREBUCKPWD != 0"]
    #[inline] pub fn test_corebuckpwd(&self) -> bool {
        self.corebuckpwd() != 0
    }

    #[doc="Sets the COREBUCKPWD field."]
    #[inline] pub fn set_corebuckpwd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Not used. Additional control of buck is available in the power control module"]
    #[inline] pub fn bypbuckcore(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BYPBUCKCORE != 0"]
    #[inline] pub fn test_bypbuckcore(&self) -> bool {
        self.bypbuckcore() != 0
    }

    #[doc="Sets the BYPBUCKCORE field."]
    #[inline] pub fn set_bypbuckcore<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Buck Register Software Override Enable. This will enable the override values for MEMBUCKPWD, COREBUCKPWD, COREBUCKRST, MEMBUCKRST, all to be propagated to the control logic, instead of the normal power control module signal. Note - Must take care to have correct value for ALL the register bits when this SWE is enabled."]
    #[inline] pub fn buckswe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BUCKSWE != 0"]
    #[inline] pub fn test_buckswe(&self) -> bool {
        self.buckswe() != 0
    }

    #[doc="Sets the BUCKSWE field."]
    #[inline] pub fn set_buckswe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Buck {
    #[inline]
    fn from(other: u32) -> Self {
         Buck(other)
    }
}

impl ::core::fmt::Display for Buck {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Buck {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.membuckrst() != 0 { try!(write!(f, " membuckrst"))}
        if self.corebuckrst() != 0 { try!(write!(f, " corebuckrst"))}
        if self.bypbuckmem() != 0 { try!(write!(f, " bypbuckmem"))}
        if self.membuckpwd() != 0 { try!(write!(f, " membuckpwd"))}
        if self.sleepbuckana() != 0 { try!(write!(f, " sleepbuckana"))}
        if self.corebuckpwd() != 0 { try!(write!(f, " corebuckpwd"))}
        if self.bypbuckcore() != 0 { try!(write!(f, " bypbuckcore"))}
        if self.buckswe() != 0 { try!(write!(f, " buckswe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Buck Control Reg2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Buck2(pub u32);
impl Buck2 {
    #[doc="Buck clkgen divider trim. 00 = 1.5MHz; 01 = 750kHz; 10 = 375kHz; 11 = 187.5kHz"]
    #[inline] pub fn bucklfclksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if BUCKLFCLKSEL != 0"]
    #[inline] pub fn test_bucklfclksel(&self) -> bool {
        self.bucklfclksel() != 0
    }

    #[doc="Sets the BUCKLFCLKSEL field."]
    #[inline] pub fn set_bucklfclksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Enable/disable hysteresis on core buck converters internal comparators."]
    #[inline] pub fn hystbuckcore(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HYSTBUCKCORE != 0"]
    #[inline] pub fn test_hystbuckcore(&self) -> bool {
        self.hystbuckcore() != 0
    }

    #[doc="Sets the HYSTBUCKCORE field."]
    #[inline] pub fn set_hystbuckcore<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enable/disable hysteresis on memory buck converters internal comparators."]
    #[inline] pub fn hystbuckmem(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HYSTBUCKMEM != 0"]
    #[inline] pub fn test_hystbuckmem(&self) -> bool {
        self.hystbuckmem() != 0
    }

    #[doc="Sets the HYSTBUCKMEM field."]
    #[inline] pub fn set_hystbuckmem<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Flash Buck high turn on trim"]
    #[inline] pub fn bmemtonsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if BMEMTONSEL != 0"]
    #[inline] pub fn test_bmemtonsel(&self) -> bool {
        self.bmemtonsel() != 0
    }

    #[doc="Sets the BMEMTONSEL field."]
    #[inline] pub fn set_bmemtonsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Core Buck low turn on trim"]
    #[inline] pub fn bcoretonsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if BCORETONSEL != 0"]
    #[inline] pub fn test_bcoretonsel(&self) -> bool {
        self.bcoretonsel() != 0
    }

    #[doc="Sets the BCORETONSEL field."]
    #[inline] pub fn set_bcoretonsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Buck2 {
    #[inline]
    fn from(other: u32) -> Self {
         Buck2(other)
    }
}

impl ::core::fmt::Display for Buck2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Buck2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bucklfclksel() != 0 { try!(write!(f, " bucklfclksel=0x{:x}", self.bucklfclksel()))}
        if self.hystbuckcore() != 0 { try!(write!(f, " hystbuckcore"))}
        if self.hystbuckmem() != 0 { try!(write!(f, " hystbuckmem"))}
        if self.bmemtonsel() != 0 { try!(write!(f, " bmemtonsel=0x{:x}", self.bmemtonsel()))}
        if self.bcoretonsel() != 0 { try!(write!(f, " bcoretonsel=0x{:x}", self.bcoretonsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Buck control reg 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Buck3(pub u32);
impl Buck3 {
    #[doc="MEM Buck low TON trim value"]
    #[inline] pub fn membuckloton(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0xf) as u8) } // [21:18]
    }

    #[doc="Returns true if MEMBUCKLOTON != 0"]
    #[inline] pub fn test_membuckloton(&self) -> bool {
        self.membuckloton() != 0
    }

    #[doc="Sets the MEMBUCKLOTON field."]
    #[inline] pub fn set_membuckloton<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="MEM Buck burst enable 0=disable, 0=disabled, 1=enable."]
    #[inline] pub fn membuckbursten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if MEMBUCKBURSTEN != 0"]
    #[inline] pub fn test_membuckbursten(&self) -> bool {
        self.membuckbursten() != 0
    }

    #[doc="Sets the MEMBUCKBURSTEN field."]
    #[inline] pub fn set_membuckbursten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Memory buck zero crossing trim value"]
    #[inline] pub fn membuckzxtrim(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0xf) as u8) } // [16:13]
    }

    #[doc="Returns true if MEMBUCKZXTRIM != 0"]
    #[inline] pub fn test_membuckzxtrim(&self) -> bool {
        self.membuckzxtrim() != 0
    }

    #[doc="Sets the MEMBUCKZXTRIM field."]
    #[inline] pub fn set_membuckzxtrim<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Hysterisis trim for mem buck"]
    #[inline] pub fn membuckhysttrim(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if MEMBUCKHYSTTRIM != 0"]
    #[inline] pub fn test_membuckhysttrim(&self) -> bool {
        self.membuckhysttrim() != 0
    }

    #[doc="Sets the MEMBUCKHYSTTRIM field."]
    #[inline] pub fn set_membuckhysttrim<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Core Buck low TON trim value"]
    #[inline] pub fn corebuckloton(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0xf) as u8) } // [10:7]
    }

    #[doc="Returns true if COREBUCKLOTON != 0"]
    #[inline] pub fn test_corebuckloton(&self) -> bool {
        self.corebuckloton() != 0
    }

    #[doc="Sets the COREBUCKLOTON field."]
    #[inline] pub fn set_corebuckloton<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Core Buck burst enable. 0=disabled, 1=enabled"]
    #[inline] pub fn corebuckbursten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if COREBUCKBURSTEN != 0"]
    #[inline] pub fn test_corebuckbursten(&self) -> bool {
        self.corebuckbursten() != 0
    }

    #[doc="Sets the COREBUCKBURSTEN field."]
    #[inline] pub fn set_corebuckbursten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Core buck zero crossing trim value"]
    #[inline] pub fn corebuckzxtrim(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0xf) as u8) } // [5:2]
    }

    #[doc="Returns true if COREBUCKZXTRIM != 0"]
    #[inline] pub fn test_corebuckzxtrim(&self) -> bool {
        self.corebuckzxtrim() != 0
    }

    #[doc="Sets the COREBUCKZXTRIM field."]
    #[inline] pub fn set_corebuckzxtrim<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Hysterisis trim for core buck"]
    #[inline] pub fn corebuckhysttrim(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if COREBUCKHYSTTRIM != 0"]
    #[inline] pub fn test_corebuckhysttrim(&self) -> bool {
        self.corebuckhysttrim() != 0
    }

    #[doc="Sets the COREBUCKHYSTTRIM field."]
    #[inline] pub fn set_corebuckhysttrim<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Buck3 {
    #[inline]
    fn from(other: u32) -> Self {
         Buck3(other)
    }
}

impl ::core::fmt::Display for Buck3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Buck3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.membuckloton() != 0 { try!(write!(f, " membuckloton=0x{:x}", self.membuckloton()))}
        if self.membuckbursten() != 0 { try!(write!(f, " membuckbursten"))}
        if self.membuckzxtrim() != 0 { try!(write!(f, " membuckzxtrim=0x{:x}", self.membuckzxtrim()))}
        if self.membuckhysttrim() != 0 { try!(write!(f, " membuckhysttrim=0x{:x}", self.membuckhysttrim()))}
        if self.corebuckloton() != 0 { try!(write!(f, " corebuckloton=0x{:x}", self.corebuckloton()))}
        if self.corebuckbursten() != 0 { try!(write!(f, " corebuckbursten"))}
        if self.corebuckzxtrim() != 0 { try!(write!(f, " corebuckzxtrim=0x{:x}", self.corebuckzxtrim()))}
        if self.corebuckhysttrim() != 0 { try!(write!(f, " corebuckhysttrim=0x{:x}", self.corebuckhysttrim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Analog LDO Reg 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ldoreg1(pub u32);
impl Ldoreg1 {
    #[doc="CORE LDO IBIAS Trim"]
    #[inline] pub fn coreldoibstrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if CORELDOIBSTRM != 0"]
    #[inline] pub fn test_coreldoibstrm(&self) -> bool {
        self.coreldoibstrm() != 0
    }

    #[doc="Sets the CORELDOIBSTRM field."]
    #[inline] pub fn set_coreldoibstrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="CORE LDO Low Power Trim"]
    #[inline] pub fn coreldolptrim(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3f) as u8) } // [19:14]
    }

    #[doc="Returns true if CORELDOLPTRIM != 0"]
    #[inline] pub fn test_coreldolptrim(&self) -> bool {
        self.coreldolptrim() != 0
    }

    #[doc="Sets the CORELDOLPTRIM field."]
    #[inline] pub fn set_coreldolptrim<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="CORE LDO tempco trim (R3)."]
    #[inline] pub fn trimcoreldor3(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0xf) as u8) } // [13:10]
    }

    #[doc="Returns true if TRIMCORELDOR3 != 0"]
    #[inline] pub fn test_trimcoreldor3(&self) -> bool {
        self.trimcoreldor3() != 0
    }

    #[doc="Sets the TRIMCORELDOR3 field."]
    #[inline] pub fn set_trimcoreldor3<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CORE LDO Active mode ouput trim (R1)."]
    #[inline] pub fn trimcoreldor1(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if TRIMCORELDOR1 != 0"]
    #[inline] pub fn test_trimcoreldor1(&self) -> bool {
        self.trimcoreldor1() != 0
    }

    #[doc="Sets the TRIMCORELDOR1 field."]
    #[inline] pub fn set_trimcoreldor1<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ldoreg1 {
    #[inline]
    fn from(other: u32) -> Self {
         Ldoreg1(other)
    }
}

impl ::core::fmt::Display for Ldoreg1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ldoreg1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.coreldoibstrm() != 0 { try!(write!(f, " coreldoibstrm"))}
        if self.coreldolptrim() != 0 { try!(write!(f, " coreldolptrim=0x{:x}", self.coreldolptrim()))}
        if self.trimcoreldor3() != 0 { try!(write!(f, " trimcoreldor3=0x{:x}", self.trimcoreldor3()))}
        if self.trimcoreldor1() != 0 { try!(write!(f, " trimcoreldor1=0x{:x}", self.trimcoreldor1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LDO Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ldoreg2(pub u32);
impl Ldoreg2 {
    #[doc="Core LDO output enable. 0=Hi-Z, 1=enable. This value is propagated only when LDO2SWE bit is active(1)."]
    #[inline] pub fn coreldovddlen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if CORELDOVDDLEN != 0"]
    #[inline] pub fn test_coreldovddlen(&self) -> bool {
        self.coreldovddlen() != 0
    }

    #[doc="Sets the CORELDOVDDLEN field."]
    #[inline] pub fn set_coreldovddlen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="RAM LDO LP Mode. 0=normal mode, 1=low power mode; This value is propagated only when LDO2SWE bit is active(1)."]
    #[inline] pub fn ramldolpmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RAMLDOLPMODE != 0"]
    #[inline] pub fn test_ramldolpmode(&self) -> bool {
        self.ramldolpmode() != 0
    }

    #[doc="Sets the RAMLDOLPMODE field."]
    #[inline] pub fn set_ramldolpmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="RAM LDO Power Down. 0=powered up, 1=powered down ; This value is propagated only when LDO2SWE bit is active(1)."]
    #[inline] pub fn pwdramldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PWDRAMLDO != 0"]
    #[inline] pub fn test_pwdramldo(&self) -> bool {
        self.pwdramldo() != 0
    }

    #[doc="Sets the PWDRAMLDO field."]
    #[inline] pub fn set_pwdramldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Analog LDO Power Down. This value is propagated only when LDO2SWE bit is active(1)."]
    #[inline] pub fn pwdanaldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PWDANALDO != 0"]
    #[inline] pub fn test_pwdanaldo(&self) -> bool {
        self.pwdanaldo() != 0
    }

    #[doc="Sets the PWDANALDO field."]
    #[inline] pub fn set_pwdanaldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="MEM LDO Power Down. This value is propagated only when LDO2SWE bit is active(1)."]
    #[inline] pub fn pwdmemldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PWDMEMLDO != 0"]
    #[inline] pub fn test_pwdmemldo(&self) -> bool {
        self.pwdmemldo() != 0
    }

    #[doc="Sets the PWDMEMLDO field."]
    #[inline] pub fn set_pwdmemldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="CORE LDO Power Down. This value is propagated only when LDO2SWE bit is active(1)."]
    #[inline] pub fn pwdcoreldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PWDCORELDO != 0"]
    #[inline] pub fn test_pwdcoreldo(&self) -> bool {
        self.pwdcoreldo() != 0
    }

    #[doc="Sets the PWDCORELDO field."]
    #[inline] pub fn set_pwdcoreldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Analog LDO Sleep. This value is propagated only when LDO2SWE bit is active(1)."]
    #[inline] pub fn sleepanaldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SLEEPANALDO != 0"]
    #[inline] pub fn test_sleepanaldo(&self) -> bool {
        self.sleepanaldo() != 0
    }

    #[doc="Sets the SLEEPANALDO field."]
    #[inline] pub fn set_sleepanaldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="FLASH LDO Sleep. This value is propagated only when LDO2SWE bit is active(1)."]
    #[inline] pub fn sleepmemldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SLEEPMEMLDO != 0"]
    #[inline] pub fn test_sleepmemldo(&self) -> bool {
        self.sleepmemldo() != 0
    }

    #[doc="Sets the SLEEPMEMLDO field."]
    #[inline] pub fn set_sleepmemldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="CORE LDO Sleep. This value is propagated only when LDO2SWE bit is active(1)."]
    #[inline] pub fn sleepcoreldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SLEEPCORELDO != 0"]
    #[inline] pub fn test_sleepcoreldo(&self) -> bool {
        self.sleepcoreldo() != 0
    }

    #[doc="Sets the SLEEPCORELDO field."]
    #[inline] pub fn set_sleepcoreldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="CONTROL BIT IS NOT USED. PLEASE TREAT AS RESERVED"]
    #[inline] pub fn vrefselanaldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if VREFSELANALDO != 0"]
    #[inline] pub fn test_vrefselanaldo(&self) -> bool {
        self.vrefselanaldo() != 0
    }

    #[doc="Sets the VREFSELANALDO field."]
    #[inline] pub fn set_vrefselanaldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CONTROL BIT IS NOT USED. PLEASE TREAT AS RESERVED"]
    #[inline] pub fn vrefselsramldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if VREFSELSRAMLDO != 0"]
    #[inline] pub fn test_vrefselsramldo(&self) -> bool {
        self.vrefselsramldo() != 0
    }

    #[doc="Sets the VREFSELSRAMLDO field."]
    #[inline] pub fn set_vrefselsramldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CONTROL BIT IS NOT USED. PLEASE TREAT AS RESERVED"]
    #[inline] pub fn vrefselflashldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if VREFSELFLASHLDO != 0"]
    #[inline] pub fn test_vrefselflashldo(&self) -> bool {
        self.vrefselflashldo() != 0
    }

    #[doc="Sets the VREFSELFLASHLDO field."]
    #[inline] pub fn set_vrefselflashldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CONTROL BIT IS NOT USED. PLEASE TREAT AS RESERVED"]
    #[inline] pub fn vrefselcoreldo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if VREFSELCORELDO != 0"]
    #[inline] pub fn test_vrefselcoreldo(&self) -> bool {
        self.vrefselcoreldo() != 0
    }

    #[doc="Sets the VREFSELCORELDO field."]
    #[inline] pub fn set_vrefselcoreldo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Analog LDO Trim."]
    #[inline] pub fn trimanaldo(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0xf) as u8) } // [9:6]
    }

    #[doc="Returns true if TRIMANALDO != 0"]
    #[inline] pub fn test_trimanaldo(&self) -> bool {
        self.trimanaldo() != 0
    }

    #[doc="Sets the TRIMANALDO field."]
    #[inline] pub fn set_trimanaldo<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="RAM LDO TRIM"]
    #[inline] pub fn ramldotrim(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1f) as u8) } // [5:1]
    }

    #[doc="Returns true if RAMLDOTRIM != 0"]
    #[inline] pub fn test_ramldotrim(&self) -> bool {
        self.ramldotrim() != 0
    }

    #[doc="Sets the RAMLDOTRIM field."]
    #[inline] pub fn set_ramldotrim<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LDO2 Software Override Enable. If enabled (=1), this will enable the override values from this register to be used instead of the normal control signals for the following fields: CORELDOVDDLEN, RAMLDOLPMODE, PWDRAMLDO, PWDANALDO, PWDMEMLDO, PWDCORELDO, SLEEPANALDO, SLEEPMEMLDO, SLEEPCORELDO."]
    #[inline] pub fn ldo2swe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LDO2SWE != 0"]
    #[inline] pub fn test_ldo2swe(&self) -> bool {
        self.ldo2swe() != 0
    }

    #[doc="Sets the LDO2SWE field."]
    #[inline] pub fn set_ldo2swe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ldoreg2 {
    #[inline]
    fn from(other: u32) -> Self {
         Ldoreg2(other)
    }
}

impl ::core::fmt::Display for Ldoreg2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ldoreg2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.coreldovddlen() != 0 { try!(write!(f, " coreldovddlen"))}
        if self.ramldolpmode() != 0 { try!(write!(f, " ramldolpmode"))}
        if self.pwdramldo() != 0 { try!(write!(f, " pwdramldo"))}
        if self.pwdanaldo() != 0 { try!(write!(f, " pwdanaldo"))}
        if self.pwdmemldo() != 0 { try!(write!(f, " pwdmemldo"))}
        if self.pwdcoreldo() != 0 { try!(write!(f, " pwdcoreldo"))}
        if self.sleepanaldo() != 0 { try!(write!(f, " sleepanaldo"))}
        if self.sleepmemldo() != 0 { try!(write!(f, " sleepmemldo"))}
        if self.sleepcoreldo() != 0 { try!(write!(f, " sleepcoreldo"))}
        if self.vrefselanaldo() != 0 { try!(write!(f, " vrefselanaldo"))}
        if self.vrefselsramldo() != 0 { try!(write!(f, " vrefselsramldo"))}
        if self.vrefselflashldo() != 0 { try!(write!(f, " vrefselflashldo"))}
        if self.vrefselcoreldo() != 0 { try!(write!(f, " vrefselcoreldo"))}
        if self.trimanaldo() != 0 { try!(write!(f, " trimanaldo=0x{:x}", self.trimanaldo()))}
        if self.ramldotrim() != 0 { try!(write!(f, " ramldotrim=0x{:x}", self.ramldotrim()))}
        if self.ldo2swe() != 0 { try!(write!(f, " ldo2swe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="LDO Control Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ldoreg3(pub u32);
impl Ldoreg3 {
    #[doc="MEM LDO active mode trim (R1)."]
    #[inline] pub fn trimmemldor1(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3f) as u8) } // [17:12]
    }

    #[doc="Returns true if TRIMMEMLDOR1 != 0"]
    #[inline] pub fn test_trimmemldor1(&self) -> bool {
        self.trimmemldor1() != 0
    }

    #[doc="Sets the TRIMMEMLDOR1 field."]
    #[inline] pub fn set_trimmemldor1<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="MEM LDO TRIM for low power mode with ADC active"]
    #[inline] pub fn memldolpalttrim(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3f) as u8) } // [11:6]
    }

    #[doc="Returns true if MEMLDOLPALTTRIM != 0"]
    #[inline] pub fn test_memldolpalttrim(&self) -> bool {
        self.memldolpalttrim() != 0
    }

    #[doc="Sets the MEMLDOLPALTTRIM field."]
    #[inline] pub fn set_memldolpalttrim<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="MEM LDO TRIM for low power mode with ADC inactive"]
    #[inline] pub fn memldolptrim(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if MEMLDOLPTRIM != 0"]
    #[inline] pub fn test_memldolptrim(&self) -> bool {
        self.memldolptrim() != 0
    }

    #[doc="Sets the MEMLDOLPTRIM field."]
    #[inline] pub fn set_memldolptrim<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ldoreg3 {
    #[inline]
    fn from(other: u32) -> Self {
         Ldoreg3(other)
    }
}

impl ::core::fmt::Display for Ldoreg3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ldoreg3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.trimmemldor1() != 0 { try!(write!(f, " trimmemldor1=0x{:x}", self.trimmemldor1()))}
        if self.memldolpalttrim() != 0 { try!(write!(f, " memldolpalttrim=0x{:x}", self.memldolpalttrim()))}
        if self.memldolptrim() != 0 { try!(write!(f, " memldolptrim=0x{:x}", self.memldolptrim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BOD and PDR control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bodporctrl(pub u32);
impl Bodporctrl {
    #[doc="BOD External Reference Select."]
    #[inline] pub fn bodextrefsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BODEXTREFSEL != 0"]
    #[inline] pub fn test_bodextrefsel(&self) -> bool {
        self.bodextrefsel() != 0
    }

    #[doc="Sets the BODEXTREFSEL field."]
    #[inline] pub fn set_bodextrefsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PDR External Reference Select."]
    #[inline] pub fn pdrextrefsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PDREXTREFSEL != 0"]
    #[inline] pub fn test_pdrextrefsel(&self) -> bool {
        self.pdrextrefsel() != 0
    }

    #[doc="Sets the PDREXTREFSEL field."]
    #[inline] pub fn set_pdrextrefsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="BOD Power Down."]
    #[inline] pub fn pwdbod(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PWDBOD != 0"]
    #[inline] pub fn test_pwdbod(&self) -> bool {
        self.pwdbod() != 0
    }

    #[doc="Sets the PWDBOD field."]
    #[inline] pub fn set_pwdbod<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="PDR Power Down."]
    #[inline] pub fn pwdpdr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PWDPDR != 0"]
    #[inline] pub fn test_pwdpdr(&self) -> bool {
        self.pwdpdr() != 0
    }

    #[doc="Sets the PWDPDR field."]
    #[inline] pub fn set_pwdpdr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bodporctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Bodporctrl(other)
    }
}

impl ::core::fmt::Display for Bodporctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bodporctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bodextrefsel() != 0 { try!(write!(f, " bodextrefsel"))}
        if self.pdrextrefsel() != 0 { try!(write!(f, " pdrextrefsel"))}
        if self.pwdbod() != 0 { try!(write!(f, " pwdbod"))}
        if self.pwdpdr() != 0 { try!(write!(f, " pwdpdr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Power Up Delay Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adcpwrdly(pub u32);
impl Adcpwrdly {
    #[doc="ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline] pub fn adcpwr1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if ADCPWR1 != 0"]
    #[inline] pub fn test_adcpwr1(&self) -> bool {
        self.adcpwr1() != 0
    }

    #[doc="Sets the ADCPWR1 field."]
    #[inline] pub fn set_adcpwr1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline] pub fn adcpwr0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ADCPWR0 != 0"]
    #[inline] pub fn test_adcpwr0(&self) -> bool {
        self.adcpwr0() != 0
    }

    #[doc="Sets the ADCPWR0 field."]
    #[inline] pub fn set_adcpwr0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Adcpwrdly {
    #[inline]
    fn from(other: u32) -> Self {
         Adcpwrdly(other)
    }
}

impl ::core::fmt::Display for Adcpwrdly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adcpwrdly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adcpwr1() != 0 { try!(write!(f, " adcpwr1=0x{:x}", self.adcpwr1()))}
        if self.adcpwr0() != 0 { try!(write!(f, " adcpwr0=0x{:x}", self.adcpwr0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Calibration Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adccal(pub u32);
impl Adccal {
    #[doc="Status for ADC Calibration"]
    #[inline] pub fn adccalibrated(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ADCCALIBRATED != 0"]
    #[inline] pub fn test_adccalibrated(&self) -> bool {
        self.adccalibrated() != 0
    }

    #[doc="Sets the ADCCALIBRATED field."]
    #[inline] pub fn set_adccalibrated<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Run ADC Calibration on initial power up sequence"]
    #[inline] pub fn calonpwrup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CALONPWRUP != 0"]
    #[inline] pub fn test_calonpwrup(&self) -> bool {
        self.calonpwrup() != 0
    }

    #[doc="Sets the CALONPWRUP field."]
    #[inline] pub fn set_calonpwrup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Adccal {
    #[inline]
    fn from(other: u32) -> Self {
         Adccal(other)
    }
}

impl ::core::fmt::Display for Adccal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adccal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adccalibrated() != 0 { try!(write!(f, " adccalibrated"))}
        if self.calonpwrup() != 0 { try!(write!(f, " calonpwrup"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Battery Load Enable"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adcbattload(pub u32);
impl Adcbattload {
    #[doc="Enable the ADC battery load resistor"]
    #[inline] pub fn battload(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BATTLOAD != 0"]
    #[inline] pub fn test_battload(&self) -> bool {
        self.battload() != 0
    }

    #[doc="Sets the BATTLOAD field."]
    #[inline] pub fn set_battload<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Adcbattload {
    #[inline]
    fn from(other: u32) -> Self {
         Adcbattload(other)
    }
}

impl ::core::fmt::Display for Adcbattload {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adcbattload {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.battload() != 0 { try!(write!(f, " battload"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Trim settings for Core and Mem buck modules"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bucktrim(pub u32);
impl Bucktrim {
    #[doc="RESERVED."]
    #[inline] pub fn rsvd2(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if RSVD2 != 0"]
    #[inline] pub fn test_rsvd2(&self) -> bool {
        self.rsvd2() != 0
    }

    #[doc="Sets the RSVD2 field."]
    #[inline] pub fn set_rsvd2<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Core Buck voltage output trim bits[9:6]. Concatenate with field COREBUCKR1_LO for the full trim value."]
    #[inline] pub fn corebuckr1_hi(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if COREBUCKR1_HI != 0"]
    #[inline] pub fn test_corebuckr1_hi(&self) -> bool {
        self.corebuckr1_hi() != 0
    }

    #[doc="Sets the COREBUCKR1_HI field."]
    #[inline] pub fn set_corebuckr1_hi<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Core Buck voltage output trim bits[5:0], Concatenate with field COREBUCKR1_HI for the full trim value."]
    #[inline] pub fn corebuckr1_lo(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if COREBUCKR1_LO != 0"]
    #[inline] pub fn test_corebuckr1_lo(&self) -> bool {
        self.corebuckr1_lo() != 0
    }

    #[doc="Sets the COREBUCKR1_LO field."]
    #[inline] pub fn set_corebuckr1_lo<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Trim values for BUCK regulator."]
    #[inline] pub fn membuckr1(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if MEMBUCKR1 != 0"]
    #[inline] pub fn test_membuckr1(&self) -> bool {
        self.membuckr1() != 0
    }

    #[doc="Sets the MEMBUCKR1 field."]
    #[inline] pub fn set_membuckr1<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bucktrim {
    #[inline]
    fn from(other: u32) -> Self {
         Bucktrim(other)
    }
}

impl ::core::fmt::Display for Bucktrim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bucktrim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rsvd2() != 0 { try!(write!(f, " rsvd2=0x{:x}", self.rsvd2()))}
        if self.corebuckr1_hi() != 0 { try!(write!(f, " corebuckr1_hi=0x{:x}", self.corebuckr1_hi()))}
        if self.corebuckr1_lo() != 0 { try!(write!(f, " corebuckr1_lo=0x{:x}", self.corebuckr1_lo()))}
        if self.membuckr1() != 0 { try!(write!(f, " membuckr1=0x{:x}", self.membuckr1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Trims"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adctrim(pub u32);
impl Adctrim {
    #[doc="ADC reference buffer input bias trim"]
    #[inline] pub fn adcrfbufibtrim(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if ADCRFBUFIBTRIM != 0"]
    #[inline] pub fn test_adcrfbufibtrim(&self) -> bool {
        self.adcrfbufibtrim() != 0
    }

    #[doc="Sets the ADCRFBUFIBTRIM field."]
    #[inline] pub fn set_adcrfbufibtrim<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ADC Reference buffer trim"]
    #[inline] pub fn adcrefbuftrim(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
    }

    #[doc="Returns true if ADCREFBUFTRIM != 0"]
    #[inline] pub fn test_adcrefbuftrim(&self) -> bool {
        self.adcrefbuftrim() != 0
    }

    #[doc="Sets the ADCREFBUFTRIM field."]
    #[inline] pub fn set_adcrefbuftrim<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="ADC Reference Ibias trim"]
    #[inline] pub fn adcrefkeepibtrim(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if ADCREFKEEPIBTRIM != 0"]
    #[inline] pub fn test_adcrefkeepibtrim(&self) -> bool {
        self.adcrefkeepibtrim() != 0
    }

    #[doc="Sets the ADCREFKEEPIBTRIM field."]
    #[inline] pub fn set_adcrefkeepibtrim<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Adctrim {
    #[inline]
    fn from(other: u32) -> Self {
         Adctrim(other)
    }
}

impl ::core::fmt::Display for Adctrim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adctrim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adcrfbufibtrim() != 0 { try!(write!(f, " adcrfbufibtrim=0x{:x}", self.adcrfbufibtrim()))}
        if self.adcrefbuftrim() != 0 { try!(write!(f, " adcrefbuftrim=0x{:x}", self.adcrefbuftrim()))}
        if self.adcrefkeepibtrim() != 0 { try!(write!(f, " adcrefkeepibtrim=0x{:x}", self.adcrefkeepibtrim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Reference Keeper and Comparator Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adcrefcomp(pub u32);
impl Adcrefcomp {
    #[doc="ADC Reference comparator power down"]
    #[inline] pub fn adcrfcmpen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ADCRFCMPEN != 0"]
    #[inline] pub fn test_adcrfcmpen(&self) -> bool {
        self.adcrfcmpen() != 0
    }

    #[doc="Sets the ADCRFCMPEN field."]
    #[inline] pub fn set_adcrfcmpen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ADC Reference Keeper Trim"]
    #[inline] pub fn adcrefkeeptrim(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if ADCREFKEEPTRIM != 0"]
    #[inline] pub fn test_adcrefkeeptrim(&self) -> bool {
        self.adcrefkeeptrim() != 0
    }

    #[doc="Sets the ADCREFKEEPTRIM field."]
    #[inline] pub fn set_adcrefkeeptrim<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Output of the ADC reference comparator"]
    #[inline] pub fn adc_refcomp_out(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADC_REFCOMP_OUT != 0"]
    #[inline] pub fn test_adc_refcomp_out(&self) -> bool {
        self.adc_refcomp_out() != 0
    }

    #[doc="Sets the ADC_REFCOMP_OUT field."]
    #[inline] pub fn set_adc_refcomp_out<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Adcrefcomp {
    #[inline]
    fn from(other: u32) -> Self {
         Adcrefcomp(other)
    }
}

impl ::core::fmt::Display for Adcrefcomp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adcrefcomp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adcrfcmpen() != 0 { try!(write!(f, " adcrfcmpen"))}
        if self.adcrefkeeptrim() != 0 { try!(write!(f, " adcrefkeeptrim=0x{:x}", self.adcrefkeeptrim()))}
        if self.adc_refcomp_out() != 0 { try!(write!(f, " adc_refcomp_out"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="XTAL Oscillator General Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Xtalgenctrl(pub u32);
impl Xtalgenctrl {
    #[doc="XTAL IBIAS Kick start trim . This trim value is used during the startup process to enable a faster lock and is applied when the kickstart signal is active."]
    #[inline] pub fn xtalksbiastrim(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if XTALKSBIASTRIM != 0"]
    #[inline] pub fn test_xtalksbiastrim(&self) -> bool {
        self.xtalksbiastrim() != 0
    }

    #[doc="Sets the XTALKSBIASTRIM field."]
    #[inline] pub fn set_xtalksbiastrim<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="XTAL IBIAS trim"]
    #[inline] pub fn xtalbiastrim(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3f) as u8) } // [7:2]
    }

    #[doc="Returns true if XTALBIASTRIM != 0"]
    #[inline] pub fn test_xtalbiastrim(&self) -> bool {
        self.xtalbiastrim() != 0
    }

    #[doc="Sets the XTALBIASTRIM field."]
    #[inline] pub fn set_xtalbiastrim<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Auto-calibration delay control"]
    #[inline] pub fn acwarmup(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if ACWARMUP != 0"]
    #[inline] pub fn test_acwarmup(&self) -> bool {
        self.acwarmup() != 0
    }

    #[doc="Sets the ACWARMUP field."]
    #[inline] pub fn set_acwarmup<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Xtalgenctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Xtalgenctrl(other)
    }
}

impl ::core::fmt::Display for Xtalgenctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Xtalgenctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.xtalksbiastrim() != 0 { try!(write!(f, " xtalksbiastrim=0x{:x}", self.xtalksbiastrim()))}
        if self.xtalbiastrim() != 0 { try!(write!(f, " xtalbiastrim=0x{:x}", self.xtalbiastrim()))}
        if self.acwarmup() != 0 { try!(write!(f, " acwarmup=0x{:x}", self.acwarmup()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Source selection of LFRC, HFRC and XTAL clock sources"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Extclksel(pub u32);
impl Extclksel {
    #[doc="HFRC External Clock Source Select."]
    #[inline] pub fn ext_hf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EXT_HF != 0"]
    #[inline] pub fn test_ext_hf(&self) -> bool {
        self.ext_hf() != 0
    }

    #[doc="Sets the EXT_HF field."]
    #[inline] pub fn set_ext_hf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LFRC External Clock Source Select."]
    #[inline] pub fn ext_lf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EXT_LF != 0"]
    #[inline] pub fn test_ext_lf(&self) -> bool {
        self.ext_lf() != 0
    }

    #[doc="Sets the EXT_LF field."]
    #[inline] pub fn set_ext_lf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="XTAL External Clock Source Select."]
    #[inline] pub fn ext_xt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EXT_XT != 0"]
    #[inline] pub fn test_ext_xt(&self) -> bool {
        self.ext_xt() != 0
    }

    #[doc="Sets the EXT_XT field."]
    #[inline] pub fn set_ext_xt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Extclksel {
    #[inline]
    fn from(other: u32) -> Self {
         Extclksel(other)
    }
}

impl ::core::fmt::Display for Extclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Extclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ext_hf() != 0 { try!(write!(f, " ext_hf"))}
        if self.ext_lf() != 0 { try!(write!(f, " ext_lf"))}
        if self.ext_xt() != 0 { try!(write!(f, " ext_xt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Determines whether the bootloader code is visible at address 0x00000000"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bootloaderlow(pub u32);
impl Bootloaderlow {
    #[doc="Determines whether the bootloader code is visible at address 0x00000000 or not."]
    #[inline] pub fn value(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if VALUE != 0"]
    #[inline] pub fn test_value(&self) -> bool {
        self.value() != 0
    }

    #[doc="Sets the VALUE field."]
    #[inline] pub fn set_value<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bootloaderlow {
    #[inline]
    fn from(other: u32) -> Self {
         Bootloaderlow(other)
    }
}

impl ::core::fmt::Display for Bootloaderlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bootloaderlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.value() != 0 { try!(write!(f, " value"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Shadowvalid(pub u32);
impl Shadowvalid {
    #[doc="Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline] pub fn bl_dsleep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BL_DSLEEP != 0"]
    #[inline] pub fn test_bl_dsleep(&self) -> bool {
        self.bl_dsleep() != 0
    }

    #[doc="Sets the BL_DSLEEP field."]
    #[inline] pub fn set_bl_dsleep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline] pub fn valid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if VALID != 0"]
    #[inline] pub fn test_valid(&self) -> bool {
        self.valid() != 0
    }

    #[doc="Sets the VALID field."]
    #[inline] pub fn set_valid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Shadowvalid {
    #[inline]
    fn from(other: u32) -> Self {
         Shadowvalid(other)
    }
}

impl ::core::fmt::Display for Shadowvalid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Shadowvalid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bl_dsleep() != 0 { try!(write!(f, " bl_dsleep"))}
        if self.valid() != 0 { try!(write!(f, " valid"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ICODE bus address which was present when a bus fault occurred."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icodefaultaddr(pub u32);
impl Icodefaultaddr {
    #[doc="The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline] pub fn addr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icodefaultaddr {
    #[inline]
    fn from(other: u32) -> Self {
         Icodefaultaddr(other)
    }
}

impl ::core::fmt::Display for Icodefaultaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icodefaultaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DCODE bus address which was present when a bus fault occurred."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcodefaultaddr(pub u32);
impl Dcodefaultaddr {
    #[doc="The DCODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline] pub fn addr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dcodefaultaddr {
    #[inline]
    fn from(other: u32) -> Self {
         Dcodefaultaddr(other)
    }
}

impl ::core::fmt::Display for Dcodefaultaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcodefaultaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System bus address which was present when a bus fault occurred."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sysfaultaddr(pub u32);
impl Sysfaultaddr {
    #[doc="SYS bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline] pub fn addr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sysfaultaddr {
    #[inline]
    fn from(other: u32) -> Self {
         Sysfaultaddr(other)
    }
}

impl ::core::fmt::Display for Sysfaultaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sysfaultaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reflects the status of the bus decoders\' fault detection. Any write to this register will clear all of the status bits within the register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Faultstatus(pub u32);
impl Faultstatus {
    #[doc="SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline] pub fn sys(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SYS != 0"]
    #[inline] pub fn test_sys(&self) -> bool {
        self.sys() != 0
    }

    #[doc="Sets the SYS field."]
    #[inline] pub fn set_sys<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline] pub fn dcode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DCODE != 0"]
    #[inline] pub fn test_dcode(&self) -> bool {
        self.dcode() != 0
    }

    #[doc="Sets the DCODE field."]
    #[inline] pub fn set_dcode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline] pub fn icode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ICODE != 0"]
    #[inline] pub fn test_icode(&self) -> bool {
        self.icode() != 0
    }

    #[doc="Sets the ICODE field."]
    #[inline] pub fn set_icode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Faultstatus {
    #[inline]
    fn from(other: u32) -> Self {
         Faultstatus(other)
    }
}

impl ::core::fmt::Display for Faultstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Faultstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sys() != 0 { try!(write!(f, " sys"))}
        if self.dcode() != 0 { try!(write!(f, " dcode"))}
        if self.icode() != 0 { try!(write!(f, " icode"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Enable the fault capture registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Faultcaptureen(pub u32);
impl Faultcaptureen {
    #[doc="Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Faultcaptureen {
    #[inline]
    fn from(other: u32) -> Self {
         Faultcaptureen(other)
    }
}

impl ::core::fmt::Display for Faultcaptureen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Faultcaptureen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Read-only debug register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgr1(pub u32);
impl Dbgr1 {
    #[doc="Read-only register for communication validation"]
    #[inline] pub fn oneto8(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ONETO8 != 0"]
    #[inline] pub fn test_oneto8(&self) -> bool {
        self.oneto8() != 0
    }

    #[doc="Sets the ONETO8 field."]
    #[inline] pub fn set_oneto8<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dbgr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dbgr1(other)
    }
}

impl ::core::fmt::Display for Dbgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Read-only debug register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgr2(pub u32);
impl Dbgr2 {
    #[doc="Read-only register for communication validation"]
    #[inline] pub fn coolcode(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if COOLCODE != 0"]
    #[inline] pub fn test_coolcode(&self) -> bool {
        self.coolcode() != 0
    }

    #[doc="Sets the COOLCODE field."]
    #[inline] pub fn set_coolcode<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dbgr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dbgr2(other)
    }
}

impl ::core::fmt::Display for Dbgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control bit to enable/disable the PMU"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pmuenable(pub u32);
impl Pmuenable {
    #[doc="PMU Enable Control bit. When set, the MCU\'s PMU will place the MCU into the lowest power consuming Deep Sleep mode upon execution of a WFI instruction (dependent on the setting of the SLEEPDEEP bit in the ARM SCR register). When cleared, regardless of the requested sleep mode, the PMU will not enter the lowest power Deep Sleep mode, instead entering the Sleep mode."]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pmuenable {
    #[inline]
    fn from(other: u32) -> Self {
         Pmuenable(other)
    }
}

impl ::core::fmt::Display for Pmuenable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pmuenable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enable() != 0 { try!(write!(f, " enable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TPIU Control Register. Determines the clock enable and frequency for the M4\'s TPIU interface."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tpiuctrl(pub u32);
impl Tpiuctrl {
    #[doc="This field selects the frequency of the ARM M4 TPIU port."]
    #[inline] pub fn clksel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if CLKSEL != 0"]
    #[inline] pub fn test_clksel(&self) -> bool {
        self.clksel() != 0
    }

    #[doc="Sets the CLKSEL field."]
    #[inline] pub fn set_clksel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU\'s SWO port using the ARM ITM and TPIU modules."]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tpiuctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Tpiuctrl(other)
    }
}

impl ::core::fmt::Display for Tpiuctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tpiuctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clksel() != 0 { try!(write!(f, " clksel=0x{:x}", self.clksel()))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Key Register to enable the use of external clock selects via the EXTCLKSEL reg"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Kextclksel(pub u32);
impl Kextclksel {
    #[doc="Key register value."]
    #[inline] pub fn kextclksel(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if KEXTCLKSEL != 0"]
    #[inline] pub fn test_kextclksel(&self) -> bool {
        self.kextclksel() != 0
    }

    #[doc="Sets the KEXTCLKSEL field."]
    #[inline] pub fn set_kextclksel<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Kextclksel {
    #[inline]
    fn from(other: u32) -> Self {
         Kextclksel(other)
    }
}

impl ::core::fmt::Display for Kextclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Kextclksel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


