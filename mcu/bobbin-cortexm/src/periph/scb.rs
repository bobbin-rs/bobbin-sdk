#[allow(unused_imports)] use bobbin_common::*;


periph!( SCB, Scb, SCB_PERIPH, ScbPeriph, SCB_OWNED, 0xe000e000, 0x00, 0x01);


#[allow(unused_imports)] use bobbin_common::*;

#[doc="System Control Block"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ScbPeriph(pub usize);
impl ScbPeriph {
    #[doc="Get the *mut pointer for the ACTLR register."]
    #[inline] pub fn actlr_mut(&self) -> *mut Actlr { 
        (self.0 + 0x8) as *mut Actlr
    }

    #[doc="Get the *const pointer for the ACTLR register."]
    #[inline] pub fn actlr_ptr(&self) -> *const Actlr { 
           self.actlr_mut()
    }

    #[doc="Read the ACTLR register."]
    #[inline] pub fn actlr(&self) -> Actlr { 
        unsafe {
            read_volatile(self.actlr_ptr())
        }
    }

    #[doc="Write the ACTLR register."]
    #[inline] pub fn set_actlr<F: FnOnce(Actlr) -> Actlr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.actlr_mut(), f(Actlr(0)));
        }
        self
    }

    #[doc="Modify the ACTLR register."]
    #[inline] pub fn with_actlr<F: FnOnce(Actlr) -> Actlr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.actlr_mut(), f(self.actlr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CPUID register."]
    #[inline] pub fn cpuid_mut(&self) -> *mut Cpuid { 
        (self.0 + 0xd00) as *mut Cpuid
    }

    #[doc="Get the *const pointer for the CPUID register."]
    #[inline] pub fn cpuid_ptr(&self) -> *const Cpuid { 
           self.cpuid_mut()
    }

    #[doc="Read the CPUID register."]
    #[inline] pub fn cpuid(&self) -> Cpuid { 
        unsafe {
            read_volatile(self.cpuid_ptr())
        }
    }

    #[doc="Write the CPUID register."]
    #[inline] pub fn set_cpuid<F: FnOnce(Cpuid) -> Cpuid>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cpuid_mut(), f(Cpuid(0)));
        }
        self
    }

    #[doc="Modify the CPUID register."]
    #[inline] pub fn with_cpuid<F: FnOnce(Cpuid) -> Cpuid>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cpuid_mut(), f(self.cpuid()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICSR register."]
    #[inline] pub fn icsr_mut(&self) -> *mut Icsr { 
        (self.0 + 0xd04) as *mut Icsr
    }

    #[doc="Get the *const pointer for the ICSR register."]
    #[inline] pub fn icsr_ptr(&self) -> *const Icsr { 
           self.icsr_mut()
    }

    #[doc="Read the ICSR register."]
    #[inline] pub fn icsr(&self) -> Icsr { 
        unsafe {
            read_volatile(self.icsr_ptr())
        }
    }

    #[doc="Write the ICSR register."]
    #[inline] pub fn set_icsr<F: FnOnce(Icsr) -> Icsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icsr_mut(), f(Icsr(0)));
        }
        self
    }

    #[doc="Modify the ICSR register."]
    #[inline] pub fn with_icsr<F: FnOnce(Icsr) -> Icsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icsr_mut(), f(self.icsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the VTOR register."]
    #[inline] pub fn vtor_mut(&self) -> *mut Vtor { 
        (self.0 + 0xd08) as *mut Vtor
    }

    #[doc="Get the *const pointer for the VTOR register."]
    #[inline] pub fn vtor_ptr(&self) -> *const Vtor { 
           self.vtor_mut()
    }

    #[doc="Read the VTOR register."]
    #[inline] pub fn vtor(&self) -> Vtor { 
        unsafe {
            read_volatile(self.vtor_ptr())
        }
    }

    #[doc="Write the VTOR register."]
    #[inline] pub fn set_vtor<F: FnOnce(Vtor) -> Vtor>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vtor_mut(), f(Vtor(0)));
        }
        self
    }

    #[doc="Modify the VTOR register."]
    #[inline] pub fn with_vtor<F: FnOnce(Vtor) -> Vtor>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.vtor_mut(), f(self.vtor()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AIRCR register."]
    #[inline] pub fn aircr_mut(&self) -> *mut Aircr { 
        (self.0 + 0xd0c) as *mut Aircr
    }

    #[doc="Get the *const pointer for the AIRCR register."]
    #[inline] pub fn aircr_ptr(&self) -> *const Aircr { 
           self.aircr_mut()
    }

    #[doc="Read the AIRCR register."]
    #[inline] pub fn aircr(&self) -> Aircr { 
        unsafe {
            read_volatile(self.aircr_ptr())
        }
    }

    #[doc="Write the AIRCR register."]
    #[inline] pub fn set_aircr<F: FnOnce(Aircr) -> Aircr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.aircr_mut(), f(Aircr(0)));
        }
        self
    }

    #[doc="Modify the AIRCR register."]
    #[inline] pub fn with_aircr<F: FnOnce(Aircr) -> Aircr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.aircr_mut(), f(self.aircr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCR register."]
    #[inline] pub fn scr_mut(&self) -> *mut Scr { 
        (self.0 + 0xd10) as *mut Scr
    }

    #[doc="Get the *const pointer for the SCR register."]
    #[inline] pub fn scr_ptr(&self) -> *const Scr { 
           self.scr_mut()
    }

    #[doc="Read the SCR register."]
    #[inline] pub fn scr(&self) -> Scr { 
        unsafe {
            read_volatile(self.scr_ptr())
        }
    }

    #[doc="Write the SCR register."]
    #[inline] pub fn set_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scr_mut(), f(Scr(0)));
        }
        self
    }

    #[doc="Modify the SCR register."]
    #[inline] pub fn with_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scr_mut(), f(self.scr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CCR register."]
    #[inline] pub fn ccr_mut(&self) -> *mut Ccr { 
        (self.0 + 0xd14) as *mut Ccr
    }

    #[doc="Get the *const pointer for the CCR register."]
    #[inline] pub fn ccr_ptr(&self) -> *const Ccr { 
           self.ccr_mut()
    }

    #[doc="Read the CCR register."]
    #[inline] pub fn ccr(&self) -> Ccr { 
        unsafe {
            read_volatile(self.ccr_ptr())
        }
    }

    #[doc="Write the CCR register."]
    #[inline] pub fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccr_mut(), f(Ccr(0)));
        }
        self
    }

    #[doc="Modify the CCR register."]
    #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccr_mut(), f(self.ccr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SHPR1 register."]
    #[inline] pub fn shpr1_mut(&self) -> *mut Shpr1 { 
        (self.0 + 0xd18) as *mut Shpr1
    }

    #[doc="Get the *const pointer for the SHPR1 register."]
    #[inline] pub fn shpr1_ptr(&self) -> *const Shpr1 { 
           self.shpr1_mut()
    }

    #[doc="Read the SHPR1 register."]
    #[inline] pub fn shpr1(&self) -> Shpr1 { 
        unsafe {
            read_volatile(self.shpr1_ptr())
        }
    }

    #[doc="Write the SHPR1 register."]
    #[inline] pub fn set_shpr1<F: FnOnce(Shpr1) -> Shpr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.shpr1_mut(), f(Shpr1(0)));
        }
        self
    }

    #[doc="Modify the SHPR1 register."]
    #[inline] pub fn with_shpr1<F: FnOnce(Shpr1) -> Shpr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.shpr1_mut(), f(self.shpr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SHPR2 register."]
    #[inline] pub fn shpr2_mut(&self) -> *mut Shpr2 { 
        (self.0 + 0xd1c) as *mut Shpr2
    }

    #[doc="Get the *const pointer for the SHPR2 register."]
    #[inline] pub fn shpr2_ptr(&self) -> *const Shpr2 { 
           self.shpr2_mut()
    }

    #[doc="Read the SHPR2 register."]
    #[inline] pub fn shpr2(&self) -> Shpr2 { 
        unsafe {
            read_volatile(self.shpr2_ptr())
        }
    }

    #[doc="Write the SHPR2 register."]
    #[inline] pub fn set_shpr2<F: FnOnce(Shpr2) -> Shpr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.shpr2_mut(), f(Shpr2(0)));
        }
        self
    }

    #[doc="Modify the SHPR2 register."]
    #[inline] pub fn with_shpr2<F: FnOnce(Shpr2) -> Shpr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.shpr2_mut(), f(self.shpr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SHPR3 register."]
    #[inline] pub fn shpr3_mut(&self) -> *mut Shpr3 { 
        (self.0 + 0xd20) as *mut Shpr3
    }

    #[doc="Get the *const pointer for the SHPR3 register."]
    #[inline] pub fn shpr3_ptr(&self) -> *const Shpr3 { 
           self.shpr3_mut()
    }

    #[doc="Read the SHPR3 register."]
    #[inline] pub fn shpr3(&self) -> Shpr3 { 
        unsafe {
            read_volatile(self.shpr3_ptr())
        }
    }

    #[doc="Write the SHPR3 register."]
    #[inline] pub fn set_shpr3<F: FnOnce(Shpr3) -> Shpr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.shpr3_mut(), f(Shpr3(0)));
        }
        self
    }

    #[doc="Modify the SHPR3 register."]
    #[inline] pub fn with_shpr3<F: FnOnce(Shpr3) -> Shpr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.shpr3_mut(), f(self.shpr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SHCSR register."]
    #[inline] pub fn shcsr_mut(&self) -> *mut Shcsr { 
        (self.0 + 0xd24) as *mut Shcsr
    }

    #[doc="Get the *const pointer for the SHCSR register."]
    #[inline] pub fn shcsr_ptr(&self) -> *const Shcsr { 
           self.shcsr_mut()
    }

    #[doc="Read the SHCSR register."]
    #[inline] pub fn shcsr(&self) -> Shcsr { 
        unsafe {
            read_volatile(self.shcsr_ptr())
        }
    }

    #[doc="Write the SHCSR register."]
    #[inline] pub fn set_shcsr<F: FnOnce(Shcsr) -> Shcsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.shcsr_mut(), f(Shcsr(0)));
        }
        self
    }

    #[doc="Modify the SHCSR register."]
    #[inline] pub fn with_shcsr<F: FnOnce(Shcsr) -> Shcsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.shcsr_mut(), f(self.shcsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFSR register."]
    #[inline] pub fn cfsr_mut(&self) -> *mut Cfsr { 
        (self.0 + 0xd28) as *mut Cfsr
    }

    #[doc="Get the *const pointer for the CFSR register."]
    #[inline] pub fn cfsr_ptr(&self) -> *const Cfsr { 
           self.cfsr_mut()
    }

    #[doc="Read the CFSR register."]
    #[inline] pub fn cfsr(&self) -> Cfsr { 
        unsafe {
            read_volatile(self.cfsr_ptr())
        }
    }

    #[doc="Write the CFSR register."]
    #[inline] pub fn set_cfsr<F: FnOnce(Cfsr) -> Cfsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfsr_mut(), f(Cfsr(0)));
        }
        self
    }

    #[doc="Modify the CFSR register."]
    #[inline] pub fn with_cfsr<F: FnOnce(Cfsr) -> Cfsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfsr_mut(), f(self.cfsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MMFSR register."]
    #[inline] pub fn mmfsr_mut(&self) -> *mut Mmfsr { 
        (self.0 + 0xd28) as *mut Mmfsr
    }

    #[doc="Get the *const pointer for the MMFSR register."]
    #[inline] pub fn mmfsr_ptr(&self) -> *const Mmfsr { 
           self.mmfsr_mut()
    }

    #[doc="Read the MMFSR register."]
    #[inline] pub fn mmfsr(&self) -> Mmfsr { 
        unsafe {
            read_volatile(self.mmfsr_ptr())
        }
    }

    #[doc="Write the MMFSR register."]
    #[inline] pub fn set_mmfsr<F: FnOnce(Mmfsr) -> Mmfsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmfsr_mut(), f(Mmfsr(0)));
        }
        self
    }

    #[doc="Modify the MMFSR register."]
    #[inline] pub fn with_mmfsr<F: FnOnce(Mmfsr) -> Mmfsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmfsr_mut(), f(self.mmfsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BFSR register."]
    #[inline] pub fn bfsr_mut(&self) -> *mut Bfsr { 
        (self.0 + 0xd29) as *mut Bfsr
    }

    #[doc="Get the *const pointer for the BFSR register."]
    #[inline] pub fn bfsr_ptr(&self) -> *const Bfsr { 
           self.bfsr_mut()
    }

    #[doc="Read the BFSR register."]
    #[inline] pub fn bfsr(&self) -> Bfsr { 
        unsafe {
            read_volatile(self.bfsr_ptr())
        }
    }

    #[doc="Write the BFSR register."]
    #[inline] pub fn set_bfsr<F: FnOnce(Bfsr) -> Bfsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bfsr_mut(), f(Bfsr(0)));
        }
        self
    }

    #[doc="Modify the BFSR register."]
    #[inline] pub fn with_bfsr<F: FnOnce(Bfsr) -> Bfsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bfsr_mut(), f(self.bfsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the UFSR register."]
    #[inline] pub fn ufsr_mut(&self) -> *mut Ufsr { 
        (self.0 + 0xd2a) as *mut Ufsr
    }

    #[doc="Get the *const pointer for the UFSR register."]
    #[inline] pub fn ufsr_ptr(&self) -> *const Ufsr { 
           self.ufsr_mut()
    }

    #[doc="Read the UFSR register."]
    #[inline] pub fn ufsr(&self) -> Ufsr { 
        unsafe {
            read_volatile(self.ufsr_ptr())
        }
    }

    #[doc="Write the UFSR register."]
    #[inline] pub fn set_ufsr<F: FnOnce(Ufsr) -> Ufsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ufsr_mut(), f(Ufsr(0)));
        }
        self
    }

    #[doc="Modify the UFSR register."]
    #[inline] pub fn with_ufsr<F: FnOnce(Ufsr) -> Ufsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ufsr_mut(), f(self.ufsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the HFSR register."]
    #[inline] pub fn hfsr_mut(&self) -> *mut Hfsr { 
        (self.0 + 0xd2c) as *mut Hfsr
    }

    #[doc="Get the *const pointer for the HFSR register."]
    #[inline] pub fn hfsr_ptr(&self) -> *const Hfsr { 
           self.hfsr_mut()
    }

    #[doc="Read the HFSR register."]
    #[inline] pub fn hfsr(&self) -> Hfsr { 
        unsafe {
            read_volatile(self.hfsr_ptr())
        }
    }

    #[doc="Write the HFSR register."]
    #[inline] pub fn set_hfsr<F: FnOnce(Hfsr) -> Hfsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfsr_mut(), f(Hfsr(0)));
        }
        self
    }

    #[doc="Modify the HFSR register."]
    #[inline] pub fn with_hfsr<F: FnOnce(Hfsr) -> Hfsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.hfsr_mut(), f(self.hfsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MMFAR register."]
    #[inline] pub fn mmfar_mut(&self) -> *mut Mmfar { 
        (self.0 + 0xd34) as *mut Mmfar
    }

    #[doc="Get the *const pointer for the MMFAR register."]
    #[inline] pub fn mmfar_ptr(&self) -> *const Mmfar { 
           self.mmfar_mut()
    }

    #[doc="Read the MMFAR register."]
    #[inline] pub fn mmfar(&self) -> Mmfar { 
        unsafe {
            read_volatile(self.mmfar_ptr())
        }
    }

    #[doc="Write the MMFAR register."]
    #[inline] pub fn set_mmfar<F: FnOnce(Mmfar) -> Mmfar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmfar_mut(), f(Mmfar(0)));
        }
        self
    }

    #[doc="Modify the MMFAR register."]
    #[inline] pub fn with_mmfar<F: FnOnce(Mmfar) -> Mmfar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmfar_mut(), f(self.mmfar()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BFAR register."]
    #[inline] pub fn bfar_mut(&self) -> *mut Bfar { 
        (self.0 + 0xd38) as *mut Bfar
    }

    #[doc="Get the *const pointer for the BFAR register."]
    #[inline] pub fn bfar_ptr(&self) -> *const Bfar { 
           self.bfar_mut()
    }

    #[doc="Read the BFAR register."]
    #[inline] pub fn bfar(&self) -> Bfar { 
        unsafe {
            read_volatile(self.bfar_ptr())
        }
    }

    #[doc="Write the BFAR register."]
    #[inline] pub fn set_bfar<F: FnOnce(Bfar) -> Bfar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bfar_mut(), f(Bfar(0)));
        }
        self
    }

    #[doc="Modify the BFAR register."]
    #[inline] pub fn with_bfar<F: FnOnce(Bfar) -> Bfar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bfar_mut(), f(self.bfar()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AFSR register."]
    #[inline] pub fn afsr_mut(&self) -> *mut Afsr { 
        (self.0 + 0xd3c) as *mut Afsr
    }

    #[doc="Get the *const pointer for the AFSR register."]
    #[inline] pub fn afsr_ptr(&self) -> *const Afsr { 
           self.afsr_mut()
    }

    #[doc="Read the AFSR register."]
    #[inline] pub fn afsr(&self) -> Afsr { 
        unsafe {
            read_volatile(self.afsr_ptr())
        }
    }

    #[doc="Write the AFSR register."]
    #[inline] pub fn set_afsr<F: FnOnce(Afsr) -> Afsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.afsr_mut(), f(Afsr(0)));
        }
        self
    }

    #[doc="Modify the AFSR register."]
    #[inline] pub fn with_afsr<F: FnOnce(Afsr) -> Afsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.afsr_mut(), f(self.afsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLIDR register."]
    #[inline] pub fn clidr_mut(&self) -> *mut Clidr { 
        (self.0 + 0xd78) as *mut Clidr
    }

    #[doc="Get the *const pointer for the CLIDR register."]
    #[inline] pub fn clidr_ptr(&self) -> *const Clidr { 
           self.clidr_mut()
    }

    #[doc="Read the CLIDR register."]
    #[inline] pub fn clidr(&self) -> Clidr { 
        unsafe {
            read_volatile(self.clidr_ptr())
        }
    }

    #[doc="Write the CLIDR register."]
    #[inline] pub fn set_clidr<F: FnOnce(Clidr) -> Clidr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clidr_mut(), f(Clidr(0)));
        }
        self
    }

    #[doc="Modify the CLIDR register."]
    #[inline] pub fn with_clidr<F: FnOnce(Clidr) -> Clidr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clidr_mut(), f(self.clidr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTR register."]
    #[inline] pub fn ctr_mut(&self) -> *mut Ctr { 
        (self.0 + 0xd7c) as *mut Ctr
    }

    #[doc="Get the *const pointer for the CTR register."]
    #[inline] pub fn ctr_ptr(&self) -> *const Ctr { 
           self.ctr_mut()
    }

    #[doc="Read the CTR register."]
    #[inline] pub fn ctr(&self) -> Ctr { 
        unsafe {
            read_volatile(self.ctr_ptr())
        }
    }

    #[doc="Write the CTR register."]
    #[inline] pub fn set_ctr<F: FnOnce(Ctr) -> Ctr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctr_mut(), f(Ctr(0)));
        }
        self
    }

    #[doc="Modify the CTR register."]
    #[inline] pub fn with_ctr<F: FnOnce(Ctr) -> Ctr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctr_mut(), f(self.ctr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CCSIDR register."]
    #[inline] pub fn ccsidr_mut(&self) -> *mut Ccsidr { 
        (self.0 + 0xd80) as *mut Ccsidr
    }

    #[doc="Get the *const pointer for the CCSIDR register."]
    #[inline] pub fn ccsidr_ptr(&self) -> *const Ccsidr { 
           self.ccsidr_mut()
    }

    #[doc="Read the CCSIDR register."]
    #[inline] pub fn ccsidr(&self) -> Ccsidr { 
        unsafe {
            read_volatile(self.ccsidr_ptr())
        }
    }

    #[doc="Write the CCSIDR register."]
    #[inline] pub fn set_ccsidr<F: FnOnce(Ccsidr) -> Ccsidr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccsidr_mut(), f(Ccsidr(0)));
        }
        self
    }

    #[doc="Modify the CCSIDR register."]
    #[inline] pub fn with_ccsidr<F: FnOnce(Ccsidr) -> Ccsidr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccsidr_mut(), f(self.ccsidr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CCSELR register."]
    #[inline] pub fn ccselr_mut(&self) -> *mut Ccselr { 
        (self.0 + 0xd84) as *mut Ccselr
    }

    #[doc="Get the *const pointer for the CCSELR register."]
    #[inline] pub fn ccselr_ptr(&self) -> *const Ccselr { 
           self.ccselr_mut()
    }

    #[doc="Read the CCSELR register."]
    #[inline] pub fn ccselr(&self) -> Ccselr { 
        unsafe {
            read_volatile(self.ccselr_ptr())
        }
    }

    #[doc="Write the CCSELR register."]
    #[inline] pub fn set_ccselr<F: FnOnce(Ccselr) -> Ccselr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccselr_mut(), f(Ccselr(0)));
        }
        self
    }

    #[doc="Modify the CCSELR register."]
    #[inline] pub fn with_ccselr<F: FnOnce(Ccselr) -> Ccselr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ccselr_mut(), f(self.ccselr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CPACR register."]
    #[inline] pub fn cpacr_mut(&self) -> *mut Cpacr { 
        (self.0 + 0xd88) as *mut Cpacr
    }

    #[doc="Get the *const pointer for the CPACR register."]
    #[inline] pub fn cpacr_ptr(&self) -> *const Cpacr { 
           self.cpacr_mut()
    }

    #[doc="Read the CPACR register."]
    #[inline] pub fn cpacr(&self) -> Cpacr { 
        unsafe {
            read_volatile(self.cpacr_ptr())
        }
    }

    #[doc="Write the CPACR register."]
    #[inline] pub fn set_cpacr<F: FnOnce(Cpacr) -> Cpacr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cpacr_mut(), f(Cpacr(0)));
        }
        self
    }

    #[doc="Modify the CPACR register."]
    #[inline] pub fn with_cpacr<F: FnOnce(Cpacr) -> Cpacr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cpacr_mut(), f(self.cpacr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICIALLU register."]
    #[inline] pub fn iciallu_mut(&self) -> *mut Iciallu { 
        (self.0 + 0xf50) as *mut Iciallu
    }

    #[doc="Get the *const pointer for the ICIALLU register."]
    #[inline] pub fn iciallu_ptr(&self) -> *const Iciallu { 
           self.iciallu_mut()
    }

    #[doc="Read the ICIALLU register."]
    #[inline] pub fn iciallu(&self) -> Iciallu { 
        unsafe {
            read_volatile(self.iciallu_ptr())
        }
    }

    #[doc="Write the ICIALLU register."]
    #[inline] pub fn set_iciallu<F: FnOnce(Iciallu) -> Iciallu>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iciallu_mut(), f(Iciallu(0)));
        }
        self
    }

    #[doc="Modify the ICIALLU register."]
    #[inline] pub fn with_iciallu<F: FnOnce(Iciallu) -> Iciallu>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.iciallu_mut(), f(self.iciallu()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICIMVAU register."]
    #[inline] pub fn icimvau_mut(&self) -> *mut Icimvau { 
        (self.0 + 0xf58) as *mut Icimvau
    }

    #[doc="Get the *const pointer for the ICIMVAU register."]
    #[inline] pub fn icimvau_ptr(&self) -> *const Icimvau { 
           self.icimvau_mut()
    }

    #[doc="Read the ICIMVAU register."]
    #[inline] pub fn icimvau(&self) -> Icimvau { 
        unsafe {
            read_volatile(self.icimvau_ptr())
        }
    }

    #[doc="Write the ICIMVAU register."]
    #[inline] pub fn set_icimvau<F: FnOnce(Icimvau) -> Icimvau>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icimvau_mut(), f(Icimvau(0)));
        }
        self
    }

    #[doc="Modify the ICIMVAU register."]
    #[inline] pub fn with_icimvau<F: FnOnce(Icimvau) -> Icimvau>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icimvau_mut(), f(self.icimvau()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCIMVAC register."]
    #[inline] pub fn dcimvac_mut(&self) -> *mut Dcimvac { 
        (self.0 + 0xf5c) as *mut Dcimvac
    }

    #[doc="Get the *const pointer for the DCIMVAC register."]
    #[inline] pub fn dcimvac_ptr(&self) -> *const Dcimvac { 
           self.dcimvac_mut()
    }

    #[doc="Read the DCIMVAC register."]
    #[inline] pub fn dcimvac(&self) -> Dcimvac { 
        unsafe {
            read_volatile(self.dcimvac_ptr())
        }
    }

    #[doc="Write the DCIMVAC register."]
    #[inline] pub fn set_dcimvac<F: FnOnce(Dcimvac) -> Dcimvac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcimvac_mut(), f(Dcimvac(0)));
        }
        self
    }

    #[doc="Modify the DCIMVAC register."]
    #[inline] pub fn with_dcimvac<F: FnOnce(Dcimvac) -> Dcimvac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcimvac_mut(), f(self.dcimvac()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCISW register."]
    #[inline] pub fn dcisw_mut(&self) -> *mut Dcisw { 
        (self.0 + 0xf60) as *mut Dcisw
    }

    #[doc="Get the *const pointer for the DCISW register."]
    #[inline] pub fn dcisw_ptr(&self) -> *const Dcisw { 
           self.dcisw_mut()
    }

    #[doc="Read the DCISW register."]
    #[inline] pub fn dcisw(&self) -> Dcisw { 
        unsafe {
            read_volatile(self.dcisw_ptr())
        }
    }

    #[doc="Write the DCISW register."]
    #[inline] pub fn set_dcisw<F: FnOnce(Dcisw) -> Dcisw>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcisw_mut(), f(Dcisw(0)));
        }
        self
    }

    #[doc="Modify the DCISW register."]
    #[inline] pub fn with_dcisw<F: FnOnce(Dcisw) -> Dcisw>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcisw_mut(), f(self.dcisw()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCCMVAU register."]
    #[inline] pub fn dccmvau_mut(&self) -> *mut Dccmvau { 
        (self.0 + 0xf64) as *mut Dccmvau
    }

    #[doc="Get the *const pointer for the DCCMVAU register."]
    #[inline] pub fn dccmvau_ptr(&self) -> *const Dccmvau { 
           self.dccmvau_mut()
    }

    #[doc="Read the DCCMVAU register."]
    #[inline] pub fn dccmvau(&self) -> Dccmvau { 
        unsafe {
            read_volatile(self.dccmvau_ptr())
        }
    }

    #[doc="Write the DCCMVAU register."]
    #[inline] pub fn set_dccmvau<F: FnOnce(Dccmvau) -> Dccmvau>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccmvau_mut(), f(Dccmvau(0)));
        }
        self
    }

    #[doc="Modify the DCCMVAU register."]
    #[inline] pub fn with_dccmvau<F: FnOnce(Dccmvau) -> Dccmvau>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccmvau_mut(), f(self.dccmvau()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCCMVAC register."]
    #[inline] pub fn dccmvac_mut(&self) -> *mut Dccmvac { 
        (self.0 + 0xf68) as *mut Dccmvac
    }

    #[doc="Get the *const pointer for the DCCMVAC register."]
    #[inline] pub fn dccmvac_ptr(&self) -> *const Dccmvac { 
           self.dccmvac_mut()
    }

    #[doc="Read the DCCMVAC register."]
    #[inline] pub fn dccmvac(&self) -> Dccmvac { 
        unsafe {
            read_volatile(self.dccmvac_ptr())
        }
    }

    #[doc="Write the DCCMVAC register."]
    #[inline] pub fn set_dccmvac<F: FnOnce(Dccmvac) -> Dccmvac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccmvac_mut(), f(Dccmvac(0)));
        }
        self
    }

    #[doc="Modify the DCCMVAC register."]
    #[inline] pub fn with_dccmvac<F: FnOnce(Dccmvac) -> Dccmvac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccmvac_mut(), f(self.dccmvac()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCCSW register."]
    #[inline] pub fn dccsw_mut(&self) -> *mut Dccsw { 
        (self.0 + 0xf6c) as *mut Dccsw
    }

    #[doc="Get the *const pointer for the DCCSW register."]
    #[inline] pub fn dccsw_ptr(&self) -> *const Dccsw { 
           self.dccsw_mut()
    }

    #[doc="Read the DCCSW register."]
    #[inline] pub fn dccsw(&self) -> Dccsw { 
        unsafe {
            read_volatile(self.dccsw_ptr())
        }
    }

    #[doc="Write the DCCSW register."]
    #[inline] pub fn set_dccsw<F: FnOnce(Dccsw) -> Dccsw>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccsw_mut(), f(Dccsw(0)));
        }
        self
    }

    #[doc="Modify the DCCSW register."]
    #[inline] pub fn with_dccsw<F: FnOnce(Dccsw) -> Dccsw>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccsw_mut(), f(self.dccsw()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCCIMVAC register."]
    #[inline] pub fn dccimvac_mut(&self) -> *mut Dccimvac { 
        (self.0 + 0xf70) as *mut Dccimvac
    }

    #[doc="Get the *const pointer for the DCCIMVAC register."]
    #[inline] pub fn dccimvac_ptr(&self) -> *const Dccimvac { 
           self.dccimvac_mut()
    }

    #[doc="Read the DCCIMVAC register."]
    #[inline] pub fn dccimvac(&self) -> Dccimvac { 
        unsafe {
            read_volatile(self.dccimvac_ptr())
        }
    }

    #[doc="Write the DCCIMVAC register."]
    #[inline] pub fn set_dccimvac<F: FnOnce(Dccimvac) -> Dccimvac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccimvac_mut(), f(Dccimvac(0)));
        }
        self
    }

    #[doc="Modify the DCCIMVAC register."]
    #[inline] pub fn with_dccimvac<F: FnOnce(Dccimvac) -> Dccimvac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccimvac_mut(), f(self.dccimvac()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCCISW register."]
    #[inline] pub fn dccisw_mut(&self) -> *mut Dccisw { 
        (self.0 + 0xf74) as *mut Dccisw
    }

    #[doc="Get the *const pointer for the DCCISW register."]
    #[inline] pub fn dccisw_ptr(&self) -> *const Dccisw { 
           self.dccisw_mut()
    }

    #[doc="Read the DCCISW register."]
    #[inline] pub fn dccisw(&self) -> Dccisw { 
        unsafe {
            read_volatile(self.dccisw_ptr())
        }
    }

    #[doc="Write the DCCISW register."]
    #[inline] pub fn set_dccisw<F: FnOnce(Dccisw) -> Dccisw>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccisw_mut(), f(Dccisw(0)));
        }
        self
    }

    #[doc="Modify the DCCISW register."]
    #[inline] pub fn with_dccisw<F: FnOnce(Dccisw) -> Dccisw>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccisw_mut(), f(self.dccisw()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ITCMCR register."]
    #[inline] pub fn itcmcr_mut(&self) -> *mut Itcmcr { 
        (self.0 + 0xf90) as *mut Itcmcr
    }

    #[doc="Get the *const pointer for the ITCMCR register."]
    #[inline] pub fn itcmcr_ptr(&self) -> *const Itcmcr { 
           self.itcmcr_mut()
    }

    #[doc="Read the ITCMCR register."]
    #[inline] pub fn itcmcr(&self) -> Itcmcr { 
        unsafe {
            read_volatile(self.itcmcr_ptr())
        }
    }

    #[doc="Write the ITCMCR register."]
    #[inline] pub fn set_itcmcr<F: FnOnce(Itcmcr) -> Itcmcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.itcmcr_mut(), f(Itcmcr(0)));
        }
        self
    }

    #[doc="Modify the ITCMCR register."]
    #[inline] pub fn with_itcmcr<F: FnOnce(Itcmcr) -> Itcmcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.itcmcr_mut(), f(self.itcmcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DTCMCR register."]
    #[inline] pub fn dtcmcr_mut(&self) -> *mut Dtcmcr { 
        (self.0 + 0xf94) as *mut Dtcmcr
    }

    #[doc="Get the *const pointer for the DTCMCR register."]
    #[inline] pub fn dtcmcr_ptr(&self) -> *const Dtcmcr { 
           self.dtcmcr_mut()
    }

    #[doc="Read the DTCMCR register."]
    #[inline] pub fn dtcmcr(&self) -> Dtcmcr { 
        unsafe {
            read_volatile(self.dtcmcr_ptr())
        }
    }

    #[doc="Write the DTCMCR register."]
    #[inline] pub fn set_dtcmcr<F: FnOnce(Dtcmcr) -> Dtcmcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dtcmcr_mut(), f(Dtcmcr(0)));
        }
        self
    }

    #[doc="Modify the DTCMCR register."]
    #[inline] pub fn with_dtcmcr<F: FnOnce(Dtcmcr) -> Dtcmcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dtcmcr_mut(), f(self.dtcmcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHBPCR register."]
    #[inline] pub fn ahbpcr_mut(&self) -> *mut Ahbpcr { 
        (self.0 + 0xf98) as *mut Ahbpcr
    }

    #[doc="Get the *const pointer for the AHBPCR register."]
    #[inline] pub fn ahbpcr_ptr(&self) -> *const Ahbpcr { 
           self.ahbpcr_mut()
    }

    #[doc="Read the AHBPCR register."]
    #[inline] pub fn ahbpcr(&self) -> Ahbpcr { 
        unsafe {
            read_volatile(self.ahbpcr_ptr())
        }
    }

    #[doc="Write the AHBPCR register."]
    #[inline] pub fn set_ahbpcr<F: FnOnce(Ahbpcr) -> Ahbpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbpcr_mut(), f(Ahbpcr(0)));
        }
        self
    }

    #[doc="Modify the AHBPCR register."]
    #[inline] pub fn with_ahbpcr<F: FnOnce(Ahbpcr) -> Ahbpcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbpcr_mut(), f(self.ahbpcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CACR register."]
    #[inline] pub fn cacr_mut(&self) -> *mut Cacr { 
        (self.0 + 0xf9c) as *mut Cacr
    }

    #[doc="Get the *const pointer for the CACR register."]
    #[inline] pub fn cacr_ptr(&self) -> *const Cacr { 
           self.cacr_mut()
    }

    #[doc="Read the CACR register."]
    #[inline] pub fn cacr(&self) -> Cacr { 
        unsafe {
            read_volatile(self.cacr_ptr())
        }
    }

    #[doc="Write the CACR register."]
    #[inline] pub fn set_cacr<F: FnOnce(Cacr) -> Cacr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cacr_mut(), f(Cacr(0)));
        }
        self
    }

    #[doc="Modify the CACR register."]
    #[inline] pub fn with_cacr<F: FnOnce(Cacr) -> Cacr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cacr_mut(), f(self.cacr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHBSCR register."]
    #[inline] pub fn ahbscr_mut(&self) -> *mut Ahbscr { 
        (self.0 + 0xf9c) as *mut Ahbscr
    }

    #[doc="Get the *const pointer for the AHBSCR register."]
    #[inline] pub fn ahbscr_ptr(&self) -> *const Ahbscr { 
           self.ahbscr_mut()
    }

    #[doc="Read the AHBSCR register."]
    #[inline] pub fn ahbscr(&self) -> Ahbscr { 
        unsafe {
            read_volatile(self.ahbscr_ptr())
        }
    }

    #[doc="Write the AHBSCR register."]
    #[inline] pub fn set_ahbscr<F: FnOnce(Ahbscr) -> Ahbscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbscr_mut(), f(Ahbscr(0)));
        }
        self
    }

    #[doc="Modify the AHBSCR register."]
    #[inline] pub fn with_ahbscr<F: FnOnce(Ahbscr) -> Ahbscr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbscr_mut(), f(self.ahbscr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ABFSR register."]
    #[inline] pub fn abfsr_mut(&self) -> *mut Abfsr { 
        (self.0 + 0xf9c) as *mut Abfsr
    }

    #[doc="Get the *const pointer for the ABFSR register."]
    #[inline] pub fn abfsr_ptr(&self) -> *const Abfsr { 
           self.abfsr_mut()
    }

    #[doc="Read the ABFSR register."]
    #[inline] pub fn abfsr(&self) -> Abfsr { 
        unsafe {
            read_volatile(self.abfsr_ptr())
        }
    }

    #[doc="Write the ABFSR register."]
    #[inline] pub fn set_abfsr<F: FnOnce(Abfsr) -> Abfsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.abfsr_mut(), f(Abfsr(0)));
        }
        self
    }

    #[doc="Modify the ABFSR register."]
    #[inline] pub fn with_abfsr<F: FnOnce(Abfsr) -> Abfsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.abfsr_mut(), f(self.abfsr()));
        }
        self
    }

}

#[doc="Auxiliary Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Actlr(pub u32);
impl Actlr {
    #[doc="When set to 1, disables IT folding."]
    #[inline] pub fn disfold(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DISFOLD != 0"]
    #[inline] pub fn test_disfold(&self) -> bool {
        self.disfold() != 0
    }

    #[doc="Sets the DISFOLD field."]
    #[inline] pub fn set_disfold<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="When set to 1, disables write buffer use during default memory map accesses."]
    #[inline] pub fn disdefwbuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DISDEFWBUF != 0"]
    #[inline] pub fn test_disdefwbuf(&self) -> bool {
        self.disdefwbuf() != 0
    }

    #[doc="Sets the DISDEFWBUF field."]
    #[inline] pub fn set_disdefwbuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="When set to 1, disables interruption of load multiple and store multiple instructions."]
    #[inline] pub fn dismcycint(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DISMCYCINT != 0"]
    #[inline] pub fn test_dismcycint(&self) -> bool {
        self.dismcycint() != 0
    }

    #[doc="Sets the DISMCYCINT field."]
    #[inline] pub fn set_dismcycint<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Actlr {
    #[inline]
    fn from(other: u32) -> Self {
         Actlr(other)
    }
}

impl ::core::fmt::Display for Actlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Actlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.disfold() != 0 { try!(write!(f, " disfold"))}
        if self.disdefwbuf() != 0 { try!(write!(f, " disdefwbuf"))}
        if self.dismcycint() != 0 { try!(write!(f, " dismcycint"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPUID Base Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpuid(pub u32);
impl Cpuid {
    #[doc="Implementer Code"]
    #[inline] pub fn implementer(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if IMPLEMENTER != 0"]
    #[inline] pub fn test_implementer(&self) -> bool {
        self.implementer() != 0
    }

    #[doc="Sets the IMPLEMENTER field."]
    #[inline] pub fn set_implementer<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Variant number, the r value in the rnpn product revision identifier"]
    #[inline] pub fn variant(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if VARIANT != 0"]
    #[inline] pub fn test_variant(&self) -> bool {
        self.variant() != 0
    }

    #[doc="Sets the VARIANT field."]
    #[inline] pub fn set_variant<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Reads as 0xF"]
    #[inline] pub fn constant(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if CONSTANT != 0"]
    #[inline] pub fn test_constant(&self) -> bool {
        self.constant() != 0
    }

    #[doc="Sets the CONSTANT field."]
    #[inline] pub fn set_constant<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Part number of the processor"]
    #[inline] pub fn partno(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
    }

    #[doc="Returns true if PARTNO != 0"]
    #[inline] pub fn test_partno(&self) -> bool {
        self.partno() != 0
    }

    #[doc="Sets the PARTNO field."]
    #[inline] pub fn set_partno<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Revision number, the p value in the rnpn product revision identifier"]
    #[inline] pub fn revision(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if REVISION != 0"]
    #[inline] pub fn test_revision(&self) -> bool {
        self.revision() != 0
    }

    #[doc="Sets the REVISION field."]
    #[inline] pub fn set_revision<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cpuid {
    #[inline]
    fn from(other: u32) -> Self {
         Cpuid(other)
    }
}

impl ::core::fmt::Display for Cpuid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cpuid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.implementer() != 0 { try!(write!(f, " implementer=0x{:x}", self.implementer()))}
        if self.variant() != 0 { try!(write!(f, " variant=0x{:x}", self.variant()))}
        if self.constant() != 0 { try!(write!(f, " constant=0x{:x}", self.constant()))}
        if self.partno() != 0 { try!(write!(f, " partno=0x{:x}", self.partno()))}
        if self.revision() != 0 { try!(write!(f, " revision=0x{:x}", self.revision()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Control and State Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icsr(pub u32);
impl Icsr {
    #[doc="NMI set-pending bit"]
    #[inline] pub fn nmipendset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if NMIPENDSET != 0"]
    #[inline] pub fn test_nmipendset(&self) -> bool {
        self.nmipendset() != 0
    }

    #[doc="Sets the NMIPENDSET field."]
    #[inline] pub fn set_nmipendset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="PendSV set-pending bit"]
    #[inline] pub fn pendsvset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PENDSVSET != 0"]
    #[inline] pub fn test_pendsvset(&self) -> bool {
        self.pendsvset() != 0
    }

    #[doc="Sets the PENDSVSET field."]
    #[inline] pub fn set_pendsvset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="PendSV clear-pending bit"]
    #[inline] pub fn pendsvclr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PENDSVCLR != 0"]
    #[inline] pub fn test_pendsvclr(&self) -> bool {
        self.pendsvclr() != 0
    }

    #[doc="Sets the PENDSVCLR field."]
    #[inline] pub fn set_pendsvclr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Systick exception set-pending bit"]
    #[inline] pub fn pendstset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PENDSTSET != 0"]
    #[inline] pub fn test_pendstset(&self) -> bool {
        self.pendstset() != 0
    }

    #[doc="Sets the PENDSTSET field."]
    #[inline] pub fn set_pendstset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Systick clear-pending bit"]
    #[inline] pub fn pendstclr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PENDSTCLR != 0"]
    #[inline] pub fn test_pendstclr(&self) -> bool {
        self.pendstclr() != 0
    }

    #[doc="Sets the PENDSTCLR field."]
    #[inline] pub fn set_pendstclr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Interrupt pending flag, excluding NMI and Faults"]
    #[inline] pub fn isrpending(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if ISRPENDING != 0"]
    #[inline] pub fn test_isrpending(&self) -> bool {
        self.isrpending() != 0
    }

    #[doc="Sets the ISRPENDING field."]
    #[inline] pub fn set_isrpending<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Indicates the exception number of the highest priority pending enabled exception"]
    #[inline] pub fn vectpending(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3f) as u8) } // [17:12]
    }

    #[doc="Returns true if VECTPENDING != 0"]
    #[inline] pub fn test_vectpending(&self) -> bool {
        self.vectpending() != 0
    }

    #[doc="Sets the VECTPENDING field."]
    #[inline] pub fn set_vectpending<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Indicates the exception number of the highest priority pending enabled exception"]
    #[inline] pub fn rettobase(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RETTOBASE != 0"]
    #[inline] pub fn test_rettobase(&self) -> bool {
        self.rettobase() != 0
    }

    #[doc="Sets the RETTOBASE field."]
    #[inline] pub fn set_rettobase<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Contains the active exception number. Subtract 16 from this value to obtain the CMSIS IRQ number required to index into the Interrupt Clear-Enable, Set-Enable, Clear-Pending, Set-Pending, or Priority Registers"]
    #[inline] pub fn vectactive(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if VECTACTIVE != 0"]
    #[inline] pub fn test_vectactive(&self) -> bool {
        self.vectactive() != 0
    }

    #[doc="Sets the VECTACTIVE field."]
    #[inline] pub fn set_vectactive<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icsr {
    #[inline]
    fn from(other: u32) -> Self {
         Icsr(other)
    }
}

impl ::core::fmt::Display for Icsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nmipendset() != 0 { try!(write!(f, " nmipendset"))}
        if self.pendsvset() != 0 { try!(write!(f, " pendsvset"))}
        if self.pendsvclr() != 0 { try!(write!(f, " pendsvclr"))}
        if self.pendstset() != 0 { try!(write!(f, " pendstset"))}
        if self.pendstclr() != 0 { try!(write!(f, " pendstclr"))}
        if self.isrpending() != 0 { try!(write!(f, " isrpending"))}
        if self.vectpending() != 0 { try!(write!(f, " vectpending=0x{:x}", self.vectpending()))}
        if self.rettobase() != 0 { try!(write!(f, " rettobase"))}
        if self.vectactive() != 0 { try!(write!(f, " vectactive=0x{:x}", self.vectactive()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Vector Table Offset Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vtor(pub u32);
impl Vtor {
    #[doc="Vector table base offset field. It contains bits[29:7] of the offset of the table base from the bottom of the memory map."]
    #[inline] pub fn tbloff(&self) -> bits::U25 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1ffffff) as u32) } // [31:7]
    }

    #[doc="Returns true if TBLOFF != 0"]
    #[inline] pub fn test_tbloff(&self) -> bool {
        self.tbloff() != 0
    }

    #[doc="Sets the TBLOFF field."]
    #[inline] pub fn set_tbloff<V: Into<bits::U25>>(mut self, value: V) -> Self {
        let value: bits::U25 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ffffff << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Vtor {
    #[inline]
    fn from(other: u32) -> Self {
         Vtor(other)
    }
}

impl ::core::fmt::Display for Vtor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Vtor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tbloff() != 0 { try!(write!(f, " tbloff=0x{:x}", self.tbloff()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Application Interrupt and Reset Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Aircr(pub u32);
impl Aircr {
    #[doc="Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored."]
    #[inline] pub fn vectkey(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if VECTKEY != 0"]
    #[inline] pub fn test_vectkey(&self) -> bool {
        self.vectkey() != 0
    }

    #[doc="Sets the VECTKEY field."]
    #[inline] pub fn set_vectkey<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data endianness bit is implementation defined: 0 = Little-endian, 1 = Big-endian."]
    #[inline] pub fn endianness(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ENDIANNESS != 0"]
    #[inline] pub fn test_endianness(&self) -> bool {
        self.endianness() != 0
    }

    #[doc="Sets the ENDIANNESS field."]
    #[inline] pub fn set_endianness<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Interrupt priority grouping field is implementation defined. This field determines the split of group priority from subpriority, see Binary point."]
    #[inline] pub fn prigroup(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PRIGROUP != 0"]
    #[inline] pub fn test_prigroup(&self) -> bool {
        self.prigroup() != 0
    }

    #[doc="Sets the PRIGROUP field."]
    #[inline] pub fn set_prigroup<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="System reset request bit is implementation defined: 0 = no system reset request, 1 = asserts a signal to the outer system that requests a reset. This is intended to force a large system reset of all major components except for debug."]
    #[inline] pub fn sysresetreq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SYSRESETREQ != 0"]
    #[inline] pub fn test_sysresetreq(&self) -> bool {
        self.sysresetreq() != 0
    }

    #[doc="Sets the SYSRESETREQ field."]
    #[inline] pub fn set_sysresetreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
    #[inline] pub fn vectclractive(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if VECTCLRACTIVE != 0"]
    #[inline] pub fn test_vectclractive(&self) -> bool {
        self.vectclractive() != 0
    }

    #[doc="Sets the VECTCLRACTIVE field."]
    #[inline] pub fn set_vectclractive<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
    #[inline] pub fn vectreset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if VECTRESET != 0"]
    #[inline] pub fn test_vectreset(&self) -> bool {
        self.vectreset() != 0
    }

    #[doc="Sets the VECTRESET field."]
    #[inline] pub fn set_vectreset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Aircr {
    #[inline]
    fn from(other: u32) -> Self {
         Aircr(other)
    }
}

impl ::core::fmt::Display for Aircr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Aircr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vectkey() != 0 { try!(write!(f, " vectkey=0x{:x}", self.vectkey()))}
        if self.endianness() != 0 { try!(write!(f, " endianness"))}
        if self.prigroup() != 0 { try!(write!(f, " prigroup=0x{:x}", self.prigroup()))}
        if self.sysresetreq() != 0 { try!(write!(f, " sysresetreq"))}
        if self.vectclractive() != 0 { try!(write!(f, " vectclractive"))}
        if self.vectreset() != 0 { try!(write!(f, " vectreset"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc="Send Event on Pending bit"]
    #[inline] pub fn sevonpend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SEVONPEND != 0"]
    #[inline] pub fn test_sevonpend(&self) -> bool {
        self.sevonpend() != 0
    }

    #[doc="Sets the SEVONPEND field."]
    #[inline] pub fn set_sevonpend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline] pub fn sleepdeep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SLEEPDEEP != 0"]
    #[inline] pub fn test_sleepdeep(&self) -> bool {
        self.sleepdeep() != 0
    }

    #[doc="Sets the SLEEPDEEP field."]
    #[inline] pub fn set_sleepdeep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Indicates sleep-on-exit when returning from Handler mode to Thread mode:"]
    #[inline] pub fn sleeponexit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SLEEPONEXIT != 0"]
    #[inline] pub fn test_sleeponexit(&self) -> bool {
        self.sleeponexit() != 0
    }

    #[doc="Sets the SLEEPONEXIT field."]
    #[inline] pub fn set_sleeponexit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Scr {
    #[inline]
    fn from(other: u32) -> Self {
         Scr(other)
    }
}

impl ::core::fmt::Display for Scr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sevonpend() != 0 { try!(write!(f, " sevonpend"))}
        if self.sleepdeep() != 0 { try!(write!(f, " sleepdeep"))}
        if self.sleeponexit() != 0 { try!(write!(f, " sleeponexit"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration and Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc="Always reads-as-one. It indicates branch prediction is enabled."]
    #[inline] pub fn bp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if BP != 0"]
    #[inline] pub fn test_bp(&self) -> bool {
        self.bp() != 0
    }

    #[doc="Sets the BP field."]
    #[inline] pub fn set_bp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Enables L1 instruction cache. 0 = L1 instruction cache disabled. 1 = L1 instruction cache enabled."]
    #[inline] pub fn ic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if IC != 0"]
    #[inline] pub fn test_ic(&self) -> bool {
        self.ic() != 0
    }

    #[doc="Sets the IC field."]
    #[inline] pub fn set_ic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Enables L1 data cache. 0 = L1 data cache disabled. 1 = L1 data cache enabled."]
    #[inline] pub fn dc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DC != 0"]
    #[inline] pub fn test_dc(&self) -> bool {
        self.dc() != 0
    }

    #[doc="Sets the DC field."]
    #[inline] pub fn set_dc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Indicates stack alignment on exception entry: 0 = 4-byte aligned1 = 8-byte aligned. On exception entry, the processor uses bit[9] of the stacked PSR to indicate the stack alignment. On return from the exception it uses this stacked bit to restore the correct stack alignment."]
    #[inline] pub fn stkalign(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if STKALIGN != 0"]
    #[inline] pub fn test_stkalign(&self) -> bool {
        self.stkalign() != 0
    }

    #[doc="Sets the STKALIGN field."]
    #[inline] pub fn set_stkalign<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the hard fault, NMI, and FAULTMASK escalated handlers: 0 = data bus faults caused by load and store instructions cause a lock-up, 1 = handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect control path problems and fix them."]
    #[inline] pub fn bfhfnmign(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BFHFNMIGN != 0"]
    #[inline] pub fn test_bfhfnmign(&self) -> bool {
        self.bfhfnmign() != 0
    }

    #[doc="Sets the BFHFNMIGN field."]
    #[inline] pub fn set_bfhfnmign<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0 = do not trap divide by 0, 1 = trap divide by 0. When this bit is set to 0, a divide by zero returns a quotient of 0."]
    #[inline] pub fn div_0_trp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DIV_0_TRP != 0"]
    #[inline] pub fn test_div_0_trp(&self) -> bool {
        self.div_0_trp() != 0
    }

    #[doc="Sets the DIV_0_TRP field."]
    #[inline] pub fn set_div_0_trp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Enables unaligned access traps: 0 = do not trap unaligned halfword and word accesses1 = trap unaligned halfword and word accesses. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault irrespective of whether UNALIGN_TRP is set to 1."]
    #[inline] pub fn unalign_trp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if UNALIGN_TRP != 0"]
    #[inline] pub fn test_unalign_trp(&self) -> bool {
        self.unalign_trp() != 0
    }

    #[doc="Sets the UNALIGN_TRP field."]
    #[inline] pub fn set_unalign_trp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Enables unprivileged software access to the STIR, see Software Trigger Interrupt Register: 0 = disable, 1 = enable."]
    #[inline] pub fn usersetmpend(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if USERSETMPEND != 0"]
    #[inline] pub fn test_usersetmpend(&self) -> bool {
        self.usersetmpend() != 0
    }

    #[doc="Sets the USERSETMPEND field."]
    #[inline] pub fn set_usersetmpend<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Indicates how the processor enters Thread mode: 0 = processor can enter Thread mode only when no exception is active, 1 = processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception return."]
    #[inline] pub fn nonbasethrdena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if NONBASETHRDENA != 0"]
    #[inline] pub fn test_nonbasethrdena(&self) -> bool {
        self.nonbasethrdena() != 0
    }

    #[doc="Sets the NONBASETHRDENA field."]
    #[inline] pub fn set_nonbasethrdena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccr {
    #[inline]
    fn from(other: u32) -> Self {
         Ccr(other)
    }
}

impl ::core::fmt::Display for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bp() != 0 { try!(write!(f, " bp"))}
        if self.ic() != 0 { try!(write!(f, " ic"))}
        if self.dc() != 0 { try!(write!(f, " dc"))}
        if self.stkalign() != 0 { try!(write!(f, " stkalign"))}
        if self.bfhfnmign() != 0 { try!(write!(f, " bfhfnmign"))}
        if self.div_0_trp() != 0 { try!(write!(f, " div_0_trp"))}
        if self.unalign_trp() != 0 { try!(write!(f, " unalign_trp"))}
        if self.usersetmpend() != 0 { try!(write!(f, " usersetmpend"))}
        if self.nonbasethrdena() != 0 { try!(write!(f, " nonbasethrdena"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Handler Priority Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Shpr1(pub u32);
impl Shpr1 {
    #[doc="Priority of system handler 6, UsageFault"]
    #[inline] pub fn pri_6(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if PRI_6 != 0"]
    #[inline] pub fn test_pri_6(&self) -> bool {
        self.pri_6() != 0
    }

    #[doc="Sets the PRI_6 field."]
    #[inline] pub fn set_pri_6<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Priority of system handler 5, BusFault"]
    #[inline] pub fn pri_5(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if PRI_5 != 0"]
    #[inline] pub fn test_pri_5(&self) -> bool {
        self.pri_5() != 0
    }

    #[doc="Sets the PRI_5 field."]
    #[inline] pub fn set_pri_5<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Priority of system handler 4, MemManage"]
    #[inline] pub fn pri_4(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PRI_4 != 0"]
    #[inline] pub fn test_pri_4(&self) -> bool {
        self.pri_4() != 0
    }

    #[doc="Sets the PRI_4 field."]
    #[inline] pub fn set_pri_4<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Shpr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Shpr1(other)
    }
}

impl ::core::fmt::Display for Shpr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Shpr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pri_6() != 0 { try!(write!(f, " pri_6=0x{:x}", self.pri_6()))}
        if self.pri_5() != 0 { try!(write!(f, " pri_5=0x{:x}", self.pri_5()))}
        if self.pri_4() != 0 { try!(write!(f, " pri_4=0x{:x}", self.pri_4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Handler Priority Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Shpr2(pub u32);
impl Shpr2 {
    #[doc="Priority of system handler 11, SVCall"]
    #[inline] pub fn pri_11(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if PRI_11 != 0"]
    #[inline] pub fn test_pri_11(&self) -> bool {
        self.pri_11() != 0
    }

    #[doc="Sets the PRI_11 field."]
    #[inline] pub fn set_pri_11<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Shpr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Shpr2(other)
    }
}

impl ::core::fmt::Display for Shpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Shpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pri_11() != 0 { try!(write!(f, " pri_11=0x{:x}", self.pri_11()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Handler Priority Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Shpr3(pub u32);
impl Shpr3 {
    #[doc="Priority of system handler 15, SysTick exception"]
    #[inline] pub fn pri_15(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if PRI_15 != 0"]
    #[inline] pub fn test_pri_15(&self) -> bool {
        self.pri_15() != 0
    }

    #[doc="Sets the PRI_15 field."]
    #[inline] pub fn set_pri_15<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Priority of system handler 14, PendSV"]
    #[inline] pub fn pri_14(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if PRI_14 != 0"]
    #[inline] pub fn test_pri_14(&self) -> bool {
        self.pri_14() != 0
    }

    #[doc="Sets the PRI_14 field."]
    #[inline] pub fn set_pri_14<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Shpr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Shpr3(other)
    }
}

impl ::core::fmt::Display for Shpr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Shpr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pri_15() != 0 { try!(write!(f, " pri_15=0x{:x}", self.pri_15()))}
        if self.pri_14() != 0 { try!(write!(f, " pri_14=0x{:x}", self.pri_14()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Handler Control and State Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Shcsr(pub u32);
impl Shcsr {
    #[doc="UsageFault enable bit, set to 1 to enable"]
    #[inline] pub fn usgfaultena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USGFAULTENA != 0"]
    #[inline] pub fn test_usgfaultena(&self) -> bool {
        self.usgfaultena() != 0
    }

    #[doc="Sets the USGFAULTENA field."]
    #[inline] pub fn set_usgfaultena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="BusFault enable bit, set to 1 to enable"]
    #[inline] pub fn busfaultena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if BUSFAULTENA != 0"]
    #[inline] pub fn test_busfaultena(&self) -> bool {
        self.busfaultena() != 0
    }

    #[doc="Sets the BUSFAULTENA field."]
    #[inline] pub fn set_busfaultena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="MemManage enable bit, set to 1 to enable"]
    #[inline] pub fn memfaultena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if MEMFAULTENA != 0"]
    #[inline] pub fn test_memfaultena(&self) -> bool {
        self.memfaultena() != 0
    }

    #[doc="Sets the MEMFAULTENA field."]
    #[inline] pub fn set_memfaultena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="SVCall pending bit, reads as 1 if exception is pending"]
    #[inline] pub fn svcallpended(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SVCALLPENDED != 0"]
    #[inline] pub fn test_svcallpended(&self) -> bool {
        self.svcallpended() != 0
    }

    #[doc="Sets the SVCALLPENDED field."]
    #[inline] pub fn set_svcallpended<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="BusFault exception pending bit, reads as 1 if exception is pending"]
    #[inline] pub fn busfaultpended(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if BUSFAULTPENDED != 0"]
    #[inline] pub fn test_busfaultpended(&self) -> bool {
        self.busfaultpended() != 0
    }

    #[doc="Sets the BUSFAULTPENDED field."]
    #[inline] pub fn set_busfaultpended<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="MemManage exception pending bit, reads as 1 if exception is pending"]
    #[inline] pub fn memfaultpended(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if MEMFAULTPENDED != 0"]
    #[inline] pub fn test_memfaultpended(&self) -> bool {
        self.memfaultpended() != 0
    }

    #[doc="Sets the MEMFAULTPENDED field."]
    #[inline] pub fn set_memfaultpended<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="UsageFault exception pending bit, reads as 1 if exception is pending"]
    #[inline] pub fn usgfaultpended(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if USGFAULTPENDED != 0"]
    #[inline] pub fn test_usgfaultpended(&self) -> bool {
        self.usgfaultpended() != 0
    }

    #[doc="Sets the USGFAULTPENDED field."]
    #[inline] pub fn set_usgfaultpended<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SysTick exception active bit, reads as 1 if exception is active"]
    #[inline] pub fn systickact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SYSTICKACT != 0"]
    #[inline] pub fn test_systickact(&self) -> bool {
        self.systickact() != 0
    }

    #[doc="Sets the SYSTICKACT field."]
    #[inline] pub fn set_systickact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PendSV exception active bit, reads as 1 if exception is active"]
    #[inline] pub fn pendsvact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PENDSVACT != 0"]
    #[inline] pub fn test_pendsvact(&self) -> bool {
        self.pendsvact() != 0
    }

    #[doc="Sets the PENDSVACT field."]
    #[inline] pub fn set_pendsvact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Debug monitor active bit, reads as 1 if Debug monitor is active"]
    #[inline] pub fn monitoract(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MONITORACT != 0"]
    #[inline] pub fn test_monitoract(&self) -> bool {
        self.monitoract() != 0
    }

    #[doc="Sets the MONITORACT field."]
    #[inline] pub fn set_monitoract<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="SVCall active bit, reads as 1 if SVC call is active"]
    #[inline] pub fn svcallact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SVCALLACT != 0"]
    #[inline] pub fn test_svcallact(&self) -> bool {
        self.svcallact() != 0
    }

    #[doc="Sets the SVCALLACT field."]
    #[inline] pub fn set_svcallact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="UsageFault exception active bit, reads as 1 if exception is active"]
    #[inline] pub fn usgfaultact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if USGFAULTACT != 0"]
    #[inline] pub fn test_usgfaultact(&self) -> bool {
        self.usgfaultact() != 0
    }

    #[doc="Sets the USGFAULTACT field."]
    #[inline] pub fn set_usgfaultact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="BusFault exception active bit, reads as 1 if exception is active"]
    #[inline] pub fn busfaultact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BUSFAULTACT != 0"]
    #[inline] pub fn test_busfaultact(&self) -> bool {
        self.busfaultact() != 0
    }

    #[doc="Sets the BUSFAULTACT field."]
    #[inline] pub fn set_busfaultact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MemManage exception active bit, reads as 1 if exception is active"]
    #[inline] pub fn memfaultact(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MEMFAULTACT != 0"]
    #[inline] pub fn test_memfaultact(&self) -> bool {
        self.memfaultact() != 0
    }

    #[doc="Sets the MEMFAULTACT field."]
    #[inline] pub fn set_memfaultact<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Shcsr {
    #[inline]
    fn from(other: u32) -> Self {
         Shcsr(other)
    }
}

impl ::core::fmt::Display for Shcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Shcsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usgfaultena() != 0 { try!(write!(f, " usgfaultena"))}
        if self.busfaultena() != 0 { try!(write!(f, " busfaultena"))}
        if self.memfaultena() != 0 { try!(write!(f, " memfaultena"))}
        if self.svcallpended() != 0 { try!(write!(f, " svcallpended"))}
        if self.busfaultpended() != 0 { try!(write!(f, " busfaultpended"))}
        if self.memfaultpended() != 0 { try!(write!(f, " memfaultpended"))}
        if self.usgfaultpended() != 0 { try!(write!(f, " usgfaultpended"))}
        if self.systickact() != 0 { try!(write!(f, " systickact"))}
        if self.pendsvact() != 0 { try!(write!(f, " pendsvact"))}
        if self.monitoract() != 0 { try!(write!(f, " monitoract"))}
        if self.svcallact() != 0 { try!(write!(f, " svcallact"))}
        if self.usgfaultact() != 0 { try!(write!(f, " usgfaultact"))}
        if self.busfaultact() != 0 { try!(write!(f, " busfaultact"))}
        if self.memfaultact() != 0 { try!(write!(f, " memfaultact"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configurable Fault Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfsr(pub u32);
impl Cfsr {
}

impl From<u32> for Cfsr {
    #[inline]
    fn from(other: u32) -> Self {
         Cfsr(other)
    }
}

impl ::core::fmt::Display for Cfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MemManage Fault Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmfsr(pub u8);
impl Mmfsr {
    #[doc="MemManage Fault Address Register (MMFAR) valid flag: 0 = value in MMAR is not a valid fault address, 1 = MMAR holds a valid fault address. If a MemManage fault occurs and is escalated to a HardFault because of priority, the HardFault handler must set this bit to 0. This prevents problems on return to a stacked active MemManage fault handler whose MMAR value has been overwritten."]
    #[inline] pub fn mmarvalid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if MMARVALID != 0"]
    #[inline] pub fn test_mmarvalid(&self) -> bool {
        self.mmarvalid() != 0
    }

    #[doc="Sets the MMARVALID field."]
    #[inline] pub fn set_mmarvalid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="MemManage fault on stacking for exception entry: 0 = no stacking fault, 1 = stacking for an exception entry has caused one or more access violations. When this bit is 1, the SP is still adjusted but the values in the context area on the stack might be incorrect. The processor has not written a fault address to the MMAR."]
    #[inline] pub fn mstkerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MSTKERR != 0"]
    #[inline] pub fn test_mstkerr(&self) -> bool {
        self.mstkerr() != 0
    }

    #[doc="Sets the MSTKERR field."]
    #[inline] pub fn set_mstkerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="MemManage fault on unstacking for a return from exception: 0 = no unstacking fault, 1 = unstack for an exception return has caused one or more access violations. This fault is chained to the handler. This means that when this bit is 1, the original return stack is still present. The processor has not adjusted the SP from the failing return, and has not performed a new save. The processor has not written a fault address to the MMAR."]
    #[inline] pub fn munstkerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MUNSTKERR != 0"]
    #[inline] pub fn test_munstkerr(&self) -> bool {
        self.munstkerr() != 0
    }

    #[doc="Sets the MUNSTKERR field."]
    #[inline] pub fn set_munstkerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data access violation flag: 0 = no data access violation fault, 1 = the processor attempted a load or store at a location that does not permit the operation. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has loaded the MMAR with the address of the attempted access."]
    #[inline] pub fn daccviol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DACCVIOL != 0"]
    #[inline] pub fn test_daccviol(&self) -> bool {
        self.daccviol() != 0
    }

    #[doc="Sets the DACCVIOL field."]
    #[inline] pub fn set_daccviol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Instruction access violation flag: 0 = no instruction access violation fault, 1 = the processor attempted an instruction fetch from a location that does not permit execution. This fault occurs on any access to an XN region, even when the MPU is disabled or not present. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has not written a fault address to the MMAR."]
    #[inline] pub fn iaccviol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IACCVIOL != 0"]
    #[inline] pub fn test_iaccviol(&self) -> bool {
        self.iaccviol() != 0
    }

    #[doc="Sets the IACCVIOL field."]
    #[inline] pub fn set_iaccviol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Mmfsr {
    #[inline]
    fn from(other: u8) -> Self {
         Mmfsr(other)
    }
}

impl ::core::fmt::Display for Mmfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mmarvalid() != 0 { try!(write!(f, " mmarvalid"))}
        if self.mstkerr() != 0 { try!(write!(f, " mstkerr"))}
        if self.munstkerr() != 0 { try!(write!(f, " munstkerr"))}
        if self.daccviol() != 0 { try!(write!(f, " daccviol"))}
        if self.iaccviol() != 0 { try!(write!(f, " iaccviol"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BusFault Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bfsr(pub u8);
impl Bfsr {
    #[doc="BusFault Address Register (BFAR) valid flag: 0 = value in BFAR is not a valid fault address, 1 = BFAR holds a valid fault address. The processor sets this bit to 1 after a BusFault where the address is known. Other faults can set this bit to 0, such as a MemManage fault occurring later. If a BusFault occurs and is escalated to a hard fault because of priority, the hard fault handler must set this bit to 0. This prevents problems if returning to a stacked active BusFault handler whose BFAR value has been overwritten."]
    #[inline] pub fn bfarvalid(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BFARVALID != 0"]
    #[inline] pub fn test_bfarvalid(&self) -> bool {
        self.bfarvalid() != 0
    }

    #[doc="Sets the BFARVALID field."]
    #[inline] pub fn set_bfarvalid<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="BusFault on stacking for exception entry: 0 = no stacking fault, 1 = stacking for an exception entry has caused one or more BusFaults. When the processor sets this bit to 1, the SP is still adjusted but the values in the context area on the stack might be incorrect. The processor does not write a fault address to the BFAR."]
    #[inline] pub fn stkerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if STKERR != 0"]
    #[inline] pub fn test_stkerr(&self) -> bool {
        self.stkerr() != 0
    }

    #[doc="Sets the STKERR field."]
    #[inline] pub fn set_stkerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="BusFault on unstacking for a return from exception: 0 = no unstacking fault, 1 = unstack for an exception return has caused one or more BusFaults. This fault is chained to the handler. This means that when the processor sets this bit to 1, the original return stack is still present. The processor does not adjust the SP from the failing return, does not performed a new save, and does not write a fault address to the BFAR."]
    #[inline] pub fn unstkerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if UNSTKERR != 0"]
    #[inline] pub fn test_unstkerr(&self) -> bool {
        self.unstkerr() != 0
    }

    #[doc="Sets the UNSTKERR field."]
    #[inline] pub fn set_unstkerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Imprecise data bus error: 0 = no imprecise data bus error, 1 = a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error. When the processor sets this bit to 1, it does not write a fault address to the BFAR. This is an asynchronous fault. Therefore, if it is detected when the priority of the current process is higher than the BusFault priority, the BusFault becomes pending and becomes active only when the processor returns from all higher priority processes. If a precise fault occurs before the processor enters the handler for the imprecise BusFault, the handler detects both IMPRECISERR set to 1 and one of the precise fault status bits set to 1."]
    #[inline] pub fn impreciserr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IMPRECISERR != 0"]
    #[inline] pub fn test_impreciserr(&self) -> bool {
        self.impreciserr() != 0
    }

    #[doc="Sets the IMPRECISERR field."]
    #[inline] pub fn set_impreciserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Precise data bus error: 0 = no precise data bus error, 1 = a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault. When the processor sets this bit is 1, it writes the faulting address to the BFAR."]
    #[inline] pub fn preciserr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PRECISERR != 0"]
    #[inline] pub fn test_preciserr(&self) -> bool {
        self.preciserr() != 0
    }

    #[doc="Sets the PRECISERR field."]
    #[inline] pub fn set_preciserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Instruction bus error: 0 = no instruction bus error, 1 = instruction bus error. The processor detects the instruction bus error on prefetching an instruction, but it sets the IBUSERR flag to 1 only if it attempts to issue the faulting instruction. When the processor sets this bit is 1, it does not write a fault address to the BFAR."]
    #[inline] pub fn ibuserr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IBUSERR != 0"]
    #[inline] pub fn test_ibuserr(&self) -> bool {
        self.ibuserr() != 0
    }

    #[doc="Sets the IBUSERR field."]
    #[inline] pub fn set_ibuserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Bfsr {
    #[inline]
    fn from(other: u8) -> Self {
         Bfsr(other)
    }
}

impl ::core::fmt::Display for Bfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bfarvalid() != 0 { try!(write!(f, " bfarvalid"))}
        if self.stkerr() != 0 { try!(write!(f, " stkerr"))}
        if self.unstkerr() != 0 { try!(write!(f, " unstkerr"))}
        if self.impreciserr() != 0 { try!(write!(f, " impreciserr"))}
        if self.preciserr() != 0 { try!(write!(f, " preciserr"))}
        if self.ibuserr() != 0 { try!(write!(f, " ibuserr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UsageFault Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ufsr(pub u16);
impl Ufsr {
    #[doc="Divide by zero UsageFault: 0 = no divide by zero fault, or divide by zero trapping not enabled, 1 = the processor has executed an SDIV or UDIV instruction with a divisor of 0. When the processor sets this bit to 1, the PC value stacked for the exception return points to the instruction that performed the divide by zero. Enable trapping of divide by zero by setting the DIV_0_TRP bit in the CCR to 1, see Configuration and Control Register."]
    #[inline] pub fn divbyzero(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DIVBYZERO != 0"]
    #[inline] pub fn test_divbyzero(&self) -> bool {
        self.divbyzero() != 0
    }

    #[doc="Sets the DIVBYZERO field."]
    #[inline] pub fn set_divbyzero<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Unaligned access UsageFault: 0 = no unaligned access fault, or unaligned access trapping not enabled, 1 = the processor has made an unaligned memory access. Enable trapping of unaligned accesses by setting the UNALIGN_TRP bit in the CCR to 1, see Configuration and Control Register. Unaligned LDM, STM, LDRD, and STRD instructions always fault irrespective of the setting of UNALIGN_TRP."]
    #[inline] pub fn unaligned(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if UNALIGNED != 0"]
    #[inline] pub fn test_unaligned(&self) -> bool {
        self.unaligned() != 0
    }

    #[doc="Sets the UNALIGNED field."]
    #[inline] pub fn set_unaligned<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="No coprocessor UsageFault. The processor does not support coprocessor instructions: 0 = no UsageFault caused by attempting to access a coprocessor, 1 = the processor has attempted to access a coprocessor."]
    #[inline] pub fn nocp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if NOCP != 0"]
    #[inline] pub fn test_nocp(&self) -> bool {
        self.nocp() != 0
    }

    #[doc="Sets the NOCP field."]
    #[inline] pub fn set_nocp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN: 0 = no invalid PC load UsageFault, 1 = the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that tried to perform the illegal load of the PC."]
    #[inline] pub fn invpc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if INVPC != 0"]
    #[inline] pub fn test_invpc(&self) -> bool {
        self.invpc() != 0
    }

    #[doc="Sets the INVPC field."]
    #[inline] pub fn set_invpc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Invalid state UsageFault: 0 = no invalid state UsageFault, 1 = the processor has attempted to execute an instruction that makes illegal use of the EPSR. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that attempted the illegal use of the EPSR. This bit is not set to 1 if an undefined instruction uses the EPSR."]
    #[inline] pub fn invstate(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INVSTATE != 0"]
    #[inline] pub fn test_invstate(&self) -> bool {
        self.invstate() != 0
    }

    #[doc="Sets the INVSTATE field."]
    #[inline] pub fn set_invstate<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Undefined instruction UsageFault: 0 = no undefined instruction UsageFault, 1 = the processor has attempted to execute an undefined instruction. When this bit is set to 1, the PC value stacked for the exception return points to the undefined instruction. An undefined instruction is an instruction that the processor cannot decode."]
    #[inline] pub fn undefinstr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UNDEFINSTR != 0"]
    #[inline] pub fn test_undefinstr(&self) -> bool {
        self.undefinstr() != 0
    }

    #[doc="Sets the UNDEFINSTR field."]
    #[inline] pub fn set_undefinstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Ufsr {
    #[inline]
    fn from(other: u16) -> Self {
         Ufsr(other)
    }
}

impl ::core::fmt::Display for Ufsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ufsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.divbyzero() != 0 { try!(write!(f, " divbyzero"))}
        if self.unaligned() != 0 { try!(write!(f, " unaligned"))}
        if self.nocp() != 0 { try!(write!(f, " nocp"))}
        if self.invpc() != 0 { try!(write!(f, " invpc"))}
        if self.invstate() != 0 { try!(write!(f, " invstate"))}
        if self.undefinstr() != 0 { try!(write!(f, " undefinstr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="HardFault Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hfsr(pub u32);
impl Hfsr {
    #[doc="Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
    #[inline] pub fn debugevt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if DEBUGEVT != 0"]
    #[inline] pub fn test_debugevt(&self) -> bool {
        self.debugevt() != 0
    }

    #[doc="Sets the DEBUGEVT field."]
    #[inline] pub fn set_debugevt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled: 0 = no forced HardFault, 1 = forced HardFault. When this bit is set to 1, the HardFault handler must read the other fault status registers to find the cause of the fault."]
    #[inline] pub fn forced(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if FORCED != 0"]
    #[inline] pub fn test_forced(&self) -> bool {
        self.forced() != 0
    }

    #[doc="Sets the FORCED field."]
    #[inline] pub fn set_forced<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Indicates a BusFault on a vector table read during exception processing: 0 = no BusFault on vector table read, 1 = BusFault on vector table read. This error is always handled by the hard fault handler. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that was preempted by the exception."]
    #[inline] pub fn vecttbl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if VECTTBL != 0"]
    #[inline] pub fn test_vecttbl(&self) -> bool {
        self.vecttbl() != 0
    }

    #[doc="Sets the VECTTBL field."]
    #[inline] pub fn set_vecttbl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Hfsr {
    #[inline]
    fn from(other: u32) -> Self {
         Hfsr(other)
    }
}

impl ::core::fmt::Display for Hfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.debugevt() != 0 { try!(write!(f, " debugevt"))}
        if self.forced() != 0 { try!(write!(f, " forced"))}
        if self.vecttbl() != 0 { try!(write!(f, " vecttbl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MemManage Fault Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmfar(pub u32);
impl Mmfar {
    #[doc="When the MMARVALID bit of the MMFSR is set to 1, this field holds the address of the location that generated the MemManage fault"]
    #[inline] pub fn address(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ADDRESS != 0"]
    #[inline] pub fn test_address(&self) -> bool {
        self.address() != 0
    }

    #[doc="Sets the ADDRESS field."]
    #[inline] pub fn set_address<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mmfar {
    #[inline]
    fn from(other: u32) -> Self {
         Mmfar(other)
    }
}

impl ::core::fmt::Display for Mmfar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmfar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BusFault Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bfar(pub u32);
impl Bfar {
    #[doc="When the BFARVALID bit of the BFSR is set to 1, this field holds the address of the location that generated the BusFault"]
    #[inline] pub fn address(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ADDRESS != 0"]
    #[inline] pub fn test_address(&self) -> bool {
        self.address() != 0
    }

    #[doc="Sets the ADDRESS field."]
    #[inline] pub fn set_address<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bfar {
    #[inline]
    fn from(other: u32) -> Self {
         Bfar(other)
    }
}

impl ::core::fmt::Display for Bfar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bfar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Auxiliary Fault Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Afsr(pub u32);
impl Afsr {
    #[doc="Implementation defined. The bits map to the AUXFAULT input signals."]
    #[inline] pub fn impdef(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if IMPDEF != 0"]
    #[inline] pub fn test_impdef(&self) -> bool {
        self.impdef() != 0
    }

    #[doc="Sets the IMPDEF field."]
    #[inline] pub fn set_impdef<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Afsr {
    #[inline]
    fn from(other: u32) -> Self {
         Afsr(other)
    }
}

impl ::core::fmt::Display for Afsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Afsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Level ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clidr(pub u32);
impl Clidr {
}

impl From<u32> for Clidr {
    #[inline]
    fn from(other: u32) -> Self {
         Clidr(other)
    }
}

impl ::core::fmt::Display for Clidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Type Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctr(pub u32);
impl Ctr {
}

impl From<u32> for Ctr {
    #[inline]
    fn from(other: u32) -> Self {
         Ctr(other)
    }
}

impl ::core::fmt::Display for Ctr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Size ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccsidr(pub u32);
impl Ccsidr {
    #[doc="Indicates support available for Write-Through: 1 - Write-Through support available."]
    #[inline] pub fn wt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if WT != 0"]
    #[inline] pub fn test_wt(&self) -> bool {
        self.wt() != 0
    }

    #[doc="Sets the WT field."]
    #[inline] pub fn set_wt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Indicates support available for Write-Back: 1 - Write-Back support available."]
    #[inline] pub fn wb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if WB != 0"]
    #[inline] pub fn test_wb(&self) -> bool {
        self.wb() != 0
    }

    #[doc="Sets the WB field."]
    #[inline] pub fn set_wb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Indicates support available for read allocation: 1 - read allocation support available."]
    #[inline] pub fn ra(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if RA != 0"]
    #[inline] pub fn test_ra(&self) -> bool {
        self.ra() != 0
    }

    #[doc="Sets the RA field."]
    #[inline] pub fn set_ra<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Indicates support available for write allocation: 1 - write allocation support available."]
    #[inline] pub fn wa(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if WA != 0"]
    #[inline] pub fn test_wa(&self) -> bool {
        self.wa() != 0
    }

    #[doc="Sets the WA field."]
    #[inline] pub fn set_wa<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Indicates the number of sets as: (number of sets) - 1."]
    #[inline] pub fn num_sets(&self) -> bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7fff) as u16) } // [27:13]
    }

    #[doc="Returns true if NUM_SETS != 0"]
    #[inline] pub fn test_num_sets(&self) -> bool {
        self.num_sets() != 0
    }

    #[doc="Sets the NUM_SETS field."]
    #[inline] pub fn set_num_sets<V: Into<bits::U15>>(mut self, value: V) -> Self {
        let value: bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Indicates the number of ways as: (number of ways) - 1. 0x1 Represents two instruction caches. 0x3 Represents four data caches."]
    #[inline] pub fn associativity(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3ff) as u16) } // [12:3]
    }

    #[doc="Returns true if ASSOCIATIVITY != 0"]
    #[inline] pub fn test_associativity(&self) -> bool {
        self.associativity() != 0
    }

    #[doc="Sets the ASSOCIATIVITY field."]
    #[inline] pub fn set_associativity<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Indicates the number of words in each cache line. 0x1 Represents 32 bytes."]
    #[inline] pub fn linesize(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if LINESIZE != 0"]
    #[inline] pub fn test_linesize(&self) -> bool {
        self.linesize() != 0
    }

    #[doc="Sets the LINESIZE field."]
    #[inline] pub fn set_linesize<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ccsidr {
    #[inline]
    fn from(other: u32) -> Self {
         Ccsidr(other)
    }
}

impl ::core::fmt::Display for Ccsidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccsidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wt() != 0 { try!(write!(f, " wt"))}
        if self.wb() != 0 { try!(write!(f, " wb"))}
        if self.ra() != 0 { try!(write!(f, " ra"))}
        if self.wa() != 0 { try!(write!(f, " wa"))}
        if self.num_sets() != 0 { try!(write!(f, " num_sets=0x{:x}", self.num_sets()))}
        if self.associativity() != 0 { try!(write!(f, " associativity=0x{:x}", self.associativity()))}
        if self.linesize() != 0 { try!(write!(f, " linesize=0x{:x}", self.linesize()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Cache Size Selection Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ccselr(pub u32);
impl Ccselr {
}

impl From<u32> for Ccselr {
    #[inline]
    fn from(other: u32) -> Self {
         Ccselr(other)
    }
}

impl ::core::fmt::Display for Ccselr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ccselr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Coprocessor Access Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpacr(pub u32);
impl Cpacr {
}

impl From<u32> for Cpacr {
    #[inline]
    fn from(other: u32) -> Self {
         Cpacr(other)
    }
}

impl ::core::fmt::Display for Cpacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cpacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I-Cache Invalidate All to PoU"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Iciallu(pub u32);
impl Iciallu {
}

impl From<u32> for Iciallu {
    #[inline]
    fn from(other: u32) -> Self {
         Iciallu(other)
    }
}

impl ::core::fmt::Display for Iciallu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Iciallu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I-Cache Invalidate by MVA to PoU"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icimvau(pub u32);
impl Icimvau {
}

impl From<u32> for Icimvau {
    #[inline]
    fn from(other: u32) -> Self {
         Icimvau(other)
    }
}

impl ::core::fmt::Display for Icimvau {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icimvau {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="D-Cache Invalidate by MVA to PoC"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcimvac(pub u32);
impl Dcimvac {
}

impl From<u32> for Dcimvac {
    #[inline]
    fn from(other: u32) -> Self {
         Dcimvac(other)
    }
}

impl ::core::fmt::Display for Dcimvac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcimvac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="D-Cache Invalidate by Set-way"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcisw(pub u32);
impl Dcisw {
    #[doc="Way"]
    #[inline] pub fn way(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if WAY != 0"]
    #[inline] pub fn test_way(&self) -> bool {
        self.way() != 0
    }

    #[doc="Sets the WAY field."]
    #[inline] pub fn set_way<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Set"]
    #[inline] pub fn set(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1ff) as u16) } // [13:5]
    }

    #[doc="Returns true if SET != 0"]
    #[inline] pub fn test_set(&self) -> bool {
        self.set() != 0
    }

    #[doc="Sets the SET field."]
    #[inline] pub fn set_set<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Dcisw {
    #[inline]
    fn from(other: u32) -> Self {
         Dcisw(other)
    }
}

impl ::core::fmt::Display for Dcisw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcisw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.way() != 0 { try!(write!(f, " way=0x{:x}", self.way()))}
        if self.set() != 0 { try!(write!(f, " set=0x{:x}", self.set()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="D-Cache Clean by MVA to PoU"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dccmvau(pub u32);
impl Dccmvau {
}

impl From<u32> for Dccmvau {
    #[inline]
    fn from(other: u32) -> Self {
         Dccmvau(other)
    }
}

impl ::core::fmt::Display for Dccmvau {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dccmvau {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="D-Cache Clean by MVA to PoC"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dccmvac(pub u32);
impl Dccmvac {
}

impl From<u32> for Dccmvac {
    #[inline]
    fn from(other: u32) -> Self {
         Dccmvac(other)
    }
}

impl ::core::fmt::Display for Dccmvac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dccmvac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="D-Cache Clean by Set-way"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dccsw(pub u32);
impl Dccsw {
    #[doc="Way"]
    #[inline] pub fn way(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if WAY != 0"]
    #[inline] pub fn test_way(&self) -> bool {
        self.way() != 0
    }

    #[doc="Sets the WAY field."]
    #[inline] pub fn set_way<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Set"]
    #[inline] pub fn set(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1ff) as u16) } // [13:5]
    }

    #[doc="Returns true if SET != 0"]
    #[inline] pub fn test_set(&self) -> bool {
        self.set() != 0
    }

    #[doc="Sets the SET field."]
    #[inline] pub fn set_set<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Dccsw {
    #[inline]
    fn from(other: u32) -> Self {
         Dccsw(other)
    }
}

impl ::core::fmt::Display for Dccsw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dccsw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.way() != 0 { try!(write!(f, " way=0x{:x}", self.way()))}
        if self.set() != 0 { try!(write!(f, " set=0x{:x}", self.set()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="D-Cache Clean and Invalidate by MVA to PoC"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dccimvac(pub u32);
impl Dccimvac {
}

impl From<u32> for Dccimvac {
    #[inline]
    fn from(other: u32) -> Self {
         Dccimvac(other)
    }
}

impl ::core::fmt::Display for Dccimvac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dccimvac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="D-Cache Clean and Invalidate by Set-way"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dccisw(pub u32);
impl Dccisw {
    #[doc="Way"]
    #[inline] pub fn way(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if WAY != 0"]
    #[inline] pub fn test_way(&self) -> bool {
        self.way() != 0
    }

    #[doc="Sets the WAY field."]
    #[inline] pub fn set_way<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Set"]
    #[inline] pub fn set(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1ff) as u16) } // [13:5]
    }

    #[doc="Returns true if SET != 0"]
    #[inline] pub fn test_set(&self) -> bool {
        self.set() != 0
    }

    #[doc="Sets the SET field."]
    #[inline] pub fn set_set<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Dccisw {
    #[inline]
    fn from(other: u32) -> Self {
         Dccisw(other)
    }
}

impl ::core::fmt::Display for Dccisw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dccisw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.way() != 0 { try!(write!(f, " way=0x{:x}", self.way()))}
        if self.set() != 0 { try!(write!(f, " set=0x{:x}", self.set()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Instruction Tightly-Coupled Memory Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Itcmcr(pub u32);
impl Itcmcr {
}

impl From<u32> for Itcmcr {
    #[inline]
    fn from(other: u32) -> Self {
         Itcmcr(other)
    }
}

impl ::core::fmt::Display for Itcmcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Itcmcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Data Tightly-Coupled Memory Control Registers"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dtcmcr(pub u32);
impl Dtcmcr {
}

impl From<u32> for Dtcmcr {
    #[inline]
    fn from(other: u32) -> Self {
         Dtcmcr(other)
    }
}

impl ::core::fmt::Display for Dtcmcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dtcmcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHBP Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahbpcr(pub u32);
impl Ahbpcr {
}

impl From<u32> for Ahbpcr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahbpcr(other)
    }
}

impl ::core::fmt::Display for Ahbpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahbpcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="L1 Cache Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cacr(pub u32);
impl Cacr {
}

impl From<u32> for Cacr {
    #[inline]
    fn from(other: u32) -> Self {
         Cacr(other)
    }
}

impl ::core::fmt::Display for Cacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cacr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB Slave Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahbscr(pub u32);
impl Ahbscr {
}

impl From<u32> for Ahbscr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahbscr(other)
    }
}

impl ::core::fmt::Display for Ahbscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahbscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Auxiliary Bus Fault Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Abfsr(pub u32);
impl Abfsr {
}

impl From<u32> for Abfsr {
    #[inline]
    fn from(other: u32) -> Self {
         Abfsr(other)
    }
}

impl ::core::fmt::Display for Abfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Abfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

