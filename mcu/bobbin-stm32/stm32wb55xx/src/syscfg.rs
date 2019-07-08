
::bobbin_mcu::periph!( SYSCFG, Syscfg, SYSCFG_PERIPH, SyscfgPeriph, SYSCFG_OWNED, SYSCFG_REF_COUNT, 0x40010100, 0x00, 0x03);


#[doc="System configuration controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SyscfgPeriph(pub usize);
impl SyscfgPeriph {
    #[doc="Get the MEMRMP Register."]
    #[inline] pub fn memrmp_reg(&self) -> ::bobbin_mcu::register::Register<Memrmp> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Memrmp, 0x0)
    }

    #[doc="Get the *mut pointer for the MEMRMP register."]
    #[inline] pub fn memrmp_mut(&self) -> *mut Memrmp { 
        self.memrmp_reg().ptr()
    }

    #[doc="Get the *const pointer for the MEMRMP register."]
    #[inline] pub fn memrmp_ptr(&self) -> *const Memrmp { 
        self.memrmp_reg().ptr()
    }

    #[doc="Read the MEMRMP register."]
    #[inline] pub fn memrmp(&self) -> Memrmp { 
        self.memrmp_reg().read()
    }

    #[doc="Write the MEMRMP register."]
    #[inline] pub fn write_memrmp(&self, value: Memrmp) -> &Self { 
        self.memrmp_reg().write(value);
        self
    }

    #[doc="Set the MEMRMP register."]
    #[inline] pub fn set_memrmp<F: FnOnce(Memrmp) -> Memrmp>(&self, f: F) -> &Self {
        self.memrmp_reg().set(f);
        self
    }

    #[doc="Modify the MEMRMP register."]
    #[inline] pub fn with_memrmp<F: FnOnce(Memrmp) -> Memrmp>(&self, f: F) -> &Self {
        self.memrmp_reg().with(f);
        self
    }

    #[doc="Get the CFGR1 Register."]
    #[inline] pub fn cfgr1_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr1, 0x4)
    }

    #[doc="Get the *mut pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_mut(&self) -> *mut Cfgr1 { 
        self.cfgr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_ptr(&self) -> *const Cfgr1 { 
        self.cfgr1_reg().ptr()
    }

    #[doc="Read the CFGR1 register."]
    #[inline] pub fn cfgr1(&self) -> Cfgr1 { 
        self.cfgr1_reg().read()
    }

    #[doc="Write the CFGR1 register."]
    #[inline] pub fn write_cfgr1(&self, value: Cfgr1) -> &Self { 
        self.cfgr1_reg().write(value);
        self
    }

    #[doc="Set the CFGR1 register."]
    #[inline] pub fn set_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        self.cfgr1_reg().set(f);
        self
    }

    #[doc="Modify the CFGR1 register."]
    #[inline] pub fn with_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        self.cfgr1_reg().with(f);
        self
    }

    #[doc="Get the EXTICR1 Register."]
    #[inline] pub fn exticr1_reg(&self) -> ::bobbin_mcu::register::Register<Exticr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr1, 0x8)
    }

    #[doc="Get the *mut pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_mut(&self) -> *mut Exticr1 { 
        self.exticr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_ptr(&self) -> *const Exticr1 { 
        self.exticr1_reg().ptr()
    }

    #[doc="Read the EXTICR1 register."]
    #[inline] pub fn exticr1(&self) -> Exticr1 { 
        self.exticr1_reg().read()
    }

    #[doc="Write the EXTICR1 register."]
    #[inline] pub fn write_exticr1(&self, value: Exticr1) -> &Self { 
        self.exticr1_reg().write(value);
        self
    }

    #[doc="Set the EXTICR1 register."]
    #[inline] pub fn set_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        self.exticr1_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR1 register."]
    #[inline] pub fn with_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        self.exticr1_reg().with(f);
        self
    }

    #[doc="Get the EXTICR2 Register."]
    #[inline] pub fn exticr2_reg(&self) -> ::bobbin_mcu::register::Register<Exticr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr2, 0xc)
    }

    #[doc="Get the *mut pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_mut(&self) -> *mut Exticr2 { 
        self.exticr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_ptr(&self) -> *const Exticr2 { 
        self.exticr2_reg().ptr()
    }

    #[doc="Read the EXTICR2 register."]
    #[inline] pub fn exticr2(&self) -> Exticr2 { 
        self.exticr2_reg().read()
    }

    #[doc="Write the EXTICR2 register."]
    #[inline] pub fn write_exticr2(&self, value: Exticr2) -> &Self { 
        self.exticr2_reg().write(value);
        self
    }

    #[doc="Set the EXTICR2 register."]
    #[inline] pub fn set_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        self.exticr2_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR2 register."]
    #[inline] pub fn with_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        self.exticr2_reg().with(f);
        self
    }

    #[doc="Get the EXTICR3 Register."]
    #[inline] pub fn exticr3_reg(&self) -> ::bobbin_mcu::register::Register<Exticr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr3, 0x10)
    }

    #[doc="Get the *mut pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_mut(&self) -> *mut Exticr3 { 
        self.exticr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_ptr(&self) -> *const Exticr3 { 
        self.exticr3_reg().ptr()
    }

    #[doc="Read the EXTICR3 register."]
    #[inline] pub fn exticr3(&self) -> Exticr3 { 
        self.exticr3_reg().read()
    }

    #[doc="Write the EXTICR3 register."]
    #[inline] pub fn write_exticr3(&self, value: Exticr3) -> &Self { 
        self.exticr3_reg().write(value);
        self
    }

    #[doc="Set the EXTICR3 register."]
    #[inline] pub fn set_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        self.exticr3_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR3 register."]
    #[inline] pub fn with_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        self.exticr3_reg().with(f);
        self
    }

    #[doc="Get the EXTICR4 Register."]
    #[inline] pub fn exticr4_reg(&self) -> ::bobbin_mcu::register::Register<Exticr4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exticr4, 0x14)
    }

    #[doc="Get the *mut pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_mut(&self) -> *mut Exticr4 { 
        self.exticr4_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_ptr(&self) -> *const Exticr4 { 
        self.exticr4_reg().ptr()
    }

    #[doc="Read the EXTICR4 register."]
    #[inline] pub fn exticr4(&self) -> Exticr4 { 
        self.exticr4_reg().read()
    }

    #[doc="Write the EXTICR4 register."]
    #[inline] pub fn write_exticr4(&self, value: Exticr4) -> &Self { 
        self.exticr4_reg().write(value);
        self
    }

    #[doc="Set the EXTICR4 register."]
    #[inline] pub fn set_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        self.exticr4_reg().set(f);
        self
    }

    #[doc="Modify the EXTICR4 register."]
    #[inline] pub fn with_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        self.exticr4_reg().with(f);
        self
    }

    #[doc="Get the SCSR Register."]
    #[inline] pub fn scsr_reg(&self) -> ::bobbin_mcu::register::Register<Scsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Scsr, 0x18)
    }

    #[doc="Get the *mut pointer for the SCSR register."]
    #[inline] pub fn scsr_mut(&self) -> *mut Scsr { 
        self.scsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SCSR register."]
    #[inline] pub fn scsr_ptr(&self) -> *const Scsr { 
        self.scsr_reg().ptr()
    }

    #[doc="Read the SCSR register."]
    #[inline] pub fn scsr(&self) -> Scsr { 
        self.scsr_reg().read()
    }

    #[doc="Write the SCSR register."]
    #[inline] pub fn write_scsr(&self, value: Scsr) -> &Self { 
        self.scsr_reg().write(value);
        self
    }

    #[doc="Set the SCSR register."]
    #[inline] pub fn set_scsr<F: FnOnce(Scsr) -> Scsr>(&self, f: F) -> &Self {
        self.scsr_reg().set(f);
        self
    }

    #[doc="Modify the SCSR register."]
    #[inline] pub fn with_scsr<F: FnOnce(Scsr) -> Scsr>(&self, f: F) -> &Self {
        self.scsr_reg().with(f);
        self
    }

    #[doc="Get the CFGR2 Register."]
    #[inline] pub fn cfgr2_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr2, 0x1c)
    }

    #[doc="Get the *mut pointer for the CFGR2 register."]
    #[inline] pub fn cfgr2_mut(&self) -> *mut Cfgr2 { 
        self.cfgr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFGR2 register."]
    #[inline] pub fn cfgr2_ptr(&self) -> *const Cfgr2 { 
        self.cfgr2_reg().ptr()
    }

    #[doc="Read the CFGR2 register."]
    #[inline] pub fn cfgr2(&self) -> Cfgr2 { 
        self.cfgr2_reg().read()
    }

    #[doc="Write the CFGR2 register."]
    #[inline] pub fn write_cfgr2(&self, value: Cfgr2) -> &Self { 
        self.cfgr2_reg().write(value);
        self
    }

    #[doc="Set the CFGR2 register."]
    #[inline] pub fn set_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
        self.cfgr2_reg().set(f);
        self
    }

    #[doc="Modify the CFGR2 register."]
    #[inline] pub fn with_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
        self.cfgr2_reg().with(f);
        self
    }

    #[doc="Get the SWPR Register."]
    #[inline] pub fn swpr_reg(&self) -> ::bobbin_mcu::register::Register<Swpr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Swpr, 0x20)
    }

    #[doc="Get the *mut pointer for the SWPR register."]
    #[inline] pub fn swpr_mut(&self) -> *mut Swpr { 
        self.swpr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SWPR register."]
    #[inline] pub fn swpr_ptr(&self) -> *const Swpr { 
        self.swpr_reg().ptr()
    }

    #[doc="Write the SWPR register."]
    #[inline] pub fn write_swpr(&self, value: Swpr) -> &Self { 
        self.swpr_reg().write(value);
        self
    }

    #[doc="Set the SWPR register."]
    #[inline] pub fn set_swpr<F: FnOnce(Swpr) -> Swpr>(&self, f: F) -> &Self {
        self.swpr_reg().set(f);
        self
    }

    #[doc="Get the SKR Register."]
    #[inline] pub fn skr_reg(&self) -> ::bobbin_mcu::register::Register<Skr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Skr, 0x24)
    }

    #[doc="Get the *mut pointer for the SKR register."]
    #[inline] pub fn skr_mut(&self) -> *mut Skr { 
        self.skr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SKR register."]
    #[inline] pub fn skr_ptr(&self) -> *const Skr { 
        self.skr_reg().ptr()
    }

    #[doc="Write the SKR register."]
    #[inline] pub fn write_skr(&self, value: Skr) -> &Self { 
        self.skr_reg().write(value);
        self
    }

    #[doc="Set the SKR register."]
    #[inline] pub fn set_skr<F: FnOnce(Skr) -> Skr>(&self, f: F) -> &Self {
        self.skr_reg().set(f);
        self
    }

    #[doc="Get the SWPR2 Register."]
    #[inline] pub fn swpr2_reg(&self) -> ::bobbin_mcu::register::Register<Swpr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Swpr2, 0x28)
    }

    #[doc="Get the *mut pointer for the SWPR2 register."]
    #[inline] pub fn swpr2_mut(&self) -> *mut Swpr2 { 
        self.swpr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SWPR2 register."]
    #[inline] pub fn swpr2_ptr(&self) -> *const Swpr2 { 
        self.swpr2_reg().ptr()
    }

    #[doc="Write the SWPR2 register."]
    #[inline] pub fn write_swpr2(&self, value: Swpr2) -> &Self { 
        self.swpr2_reg().write(value);
        self
    }

    #[doc="Set the SWPR2 register."]
    #[inline] pub fn set_swpr2<F: FnOnce(Swpr2) -> Swpr2>(&self, f: F) -> &Self {
        self.swpr2_reg().set(f);
        self
    }

    #[doc="Get the IMR1 Register."]
    #[inline] pub fn imr1_reg(&self) -> ::bobbin_mcu::register::Register<Imr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Imr1, 0x2c)
    }

    #[doc="Get the *mut pointer for the IMR1 register."]
    #[inline] pub fn imr1_mut(&self) -> *mut Imr1 { 
        self.imr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the IMR1 register."]
    #[inline] pub fn imr1_ptr(&self) -> *const Imr1 { 
        self.imr1_reg().ptr()
    }

    #[doc="Read the IMR1 register."]
    #[inline] pub fn imr1(&self) -> Imr1 { 
        self.imr1_reg().read()
    }

    #[doc="Write the IMR1 register."]
    #[inline] pub fn write_imr1(&self, value: Imr1) -> &Self { 
        self.imr1_reg().write(value);
        self
    }

    #[doc="Set the IMR1 register."]
    #[inline] pub fn set_imr1<F: FnOnce(Imr1) -> Imr1>(&self, f: F) -> &Self {
        self.imr1_reg().set(f);
        self
    }

    #[doc="Modify the IMR1 register."]
    #[inline] pub fn with_imr1<F: FnOnce(Imr1) -> Imr1>(&self, f: F) -> &Self {
        self.imr1_reg().with(f);
        self
    }

    #[doc="Get the IMR2 Register."]
    #[inline] pub fn imr2_reg(&self) -> ::bobbin_mcu::register::Register<Imr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Imr2, 0x30)
    }

    #[doc="Get the *mut pointer for the IMR2 register."]
    #[inline] pub fn imr2_mut(&self) -> *mut Imr2 { 
        self.imr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the IMR2 register."]
    #[inline] pub fn imr2_ptr(&self) -> *const Imr2 { 
        self.imr2_reg().ptr()
    }

    #[doc="Read the IMR2 register."]
    #[inline] pub fn imr2(&self) -> Imr2 { 
        self.imr2_reg().read()
    }

    #[doc="Write the IMR2 register."]
    #[inline] pub fn write_imr2(&self, value: Imr2) -> &Self { 
        self.imr2_reg().write(value);
        self
    }

    #[doc="Set the IMR2 register."]
    #[inline] pub fn set_imr2<F: FnOnce(Imr2) -> Imr2>(&self, f: F) -> &Self {
        self.imr2_reg().set(f);
        self
    }

    #[doc="Modify the IMR2 register."]
    #[inline] pub fn with_imr2<F: FnOnce(Imr2) -> Imr2>(&self, f: F) -> &Self {
        self.imr2_reg().with(f);
        self
    }

    #[doc="Get the C2IMR1 Register."]
    #[inline] pub fn c2imr1_reg(&self) -> ::bobbin_mcu::register::Register<C2imr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2imr1, 0x34)
    }

    #[doc="Get the *mut pointer for the C2IMR1 register."]
    #[inline] pub fn c2imr1_mut(&self) -> *mut C2imr1 { 
        self.c2imr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2IMR1 register."]
    #[inline] pub fn c2imr1_ptr(&self) -> *const C2imr1 { 
        self.c2imr1_reg().ptr()
    }

    #[doc="Read the C2IMR1 register."]
    #[inline] pub fn c2imr1(&self) -> C2imr1 { 
        self.c2imr1_reg().read()
    }

    #[doc="Write the C2IMR1 register."]
    #[inline] pub fn write_c2imr1(&self, value: C2imr1) -> &Self { 
        self.c2imr1_reg().write(value);
        self
    }

    #[doc="Set the C2IMR1 register."]
    #[inline] pub fn set_c2imr1<F: FnOnce(C2imr1) -> C2imr1>(&self, f: F) -> &Self {
        self.c2imr1_reg().set(f);
        self
    }

    #[doc="Modify the C2IMR1 register."]
    #[inline] pub fn with_c2imr1<F: FnOnce(C2imr1) -> C2imr1>(&self, f: F) -> &Self {
        self.c2imr1_reg().with(f);
        self
    }

    #[doc="Get the C2IMR2 Register."]
    #[inline] pub fn c2imr2_reg(&self) -> ::bobbin_mcu::register::Register<C2imr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2imr2, 0x38)
    }

    #[doc="Get the *mut pointer for the C2IMR2 register."]
    #[inline] pub fn c2imr2_mut(&self) -> *mut C2imr2 { 
        self.c2imr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2IMR2 register."]
    #[inline] pub fn c2imr2_ptr(&self) -> *const C2imr2 { 
        self.c2imr2_reg().ptr()
    }

    #[doc="Read the C2IMR2 register."]
    #[inline] pub fn c2imr2(&self) -> C2imr2 { 
        self.c2imr2_reg().read()
    }

    #[doc="Write the C2IMR2 register."]
    #[inline] pub fn write_c2imr2(&self, value: C2imr2) -> &Self { 
        self.c2imr2_reg().write(value);
        self
    }

    #[doc="Set the C2IMR2 register."]
    #[inline] pub fn set_c2imr2<F: FnOnce(C2imr2) -> C2imr2>(&self, f: F) -> &Self {
        self.c2imr2_reg().set(f);
        self
    }

    #[doc="Modify the C2IMR2 register."]
    #[inline] pub fn with_c2imr2<F: FnOnce(C2imr2) -> C2imr2>(&self, f: F) -> &Self {
        self.c2imr2_reg().with(f);
        self
    }

    #[doc="Get the SIPCR Register."]
    #[inline] pub fn sipcr_reg(&self) -> ::bobbin_mcu::register::Register<Sipcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sipcr, 0x3c)
    }

    #[doc="Get the *mut pointer for the SIPCR register."]
    #[inline] pub fn sipcr_mut(&self) -> *mut Sipcr { 
        self.sipcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SIPCR register."]
    #[inline] pub fn sipcr_ptr(&self) -> *const Sipcr { 
        self.sipcr_reg().ptr()
    }

    #[doc="Read the SIPCR register."]
    #[inline] pub fn sipcr(&self) -> Sipcr { 
        self.sipcr_reg().read()
    }

    #[doc="Write the SIPCR register."]
    #[inline] pub fn write_sipcr(&self, value: Sipcr) -> &Self { 
        self.sipcr_reg().write(value);
        self
    }

    #[doc="Set the SIPCR register."]
    #[inline] pub fn set_sipcr<F: FnOnce(Sipcr) -> Sipcr>(&self, f: F) -> &Self {
        self.sipcr_reg().set(f);
        self
    }

    #[doc="Modify the SIPCR register."]
    #[inline] pub fn with_sipcr<F: FnOnce(Sipcr) -> Sipcr>(&self, f: F) -> &Self {
        self.sipcr_reg().with(f);
        self
    }

}

#[doc="memory remap register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Memrmp(pub u32);
impl Memrmp {
    #[doc="Memory mapping selection"]
    #[inline] pub fn mem_mode(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if MEM_MODE != 0"]
    #[inline] pub fn test_mem_mode(&self) -> bool {
        self.mem_mode() != 0
    }

    #[doc="Sets the MEM_MODE field."]
    #[inline] pub fn set_mem_mode<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Memrmp {
    #[inline]
    fn from(other: u32) -> Self {
         Memrmp(other)
    }
}

impl ::core::fmt::Display for Memrmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Memrmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mem_mode() != 0 { try!(write!(f, " mem_mode=0x{:x}", self.mem_mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="configuration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
    #[doc="Floating Point Unit interrupts enable bits"]
    #[inline] pub fn fpu_ie(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3f) as u8) } // [31:26]
    }

    #[doc="Returns true if FPU_IE != 0"]
    #[inline] pub fn test_fpu_ie(&self) -> bool {
        self.fpu_ie() != 0
    }

    #[doc="Sets the FPU_IE field."]
    #[inline] pub fn set_fpu_ie<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="I2C3 Fast-mode Plus driving capability activation"]
    #[inline] pub fn i2c3_fmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C3_FMP != 0"]
    #[inline] pub fn test_i2c3_fmp(&self) -> bool {
        self.i2c3_fmp() != 0
    }

    #[doc="Sets the I2C3_FMP field."]
    #[inline] pub fn set_i2c3_fmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I2C1 Fast-mode Plus driving capability activation"]
    #[inline] pub fn i2c1_fmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if I2C1_FMP != 0"]
    #[inline] pub fn test_i2c1_fmp(&self) -> bool {
        self.i2c1_fmp() != 0
    }

    #[doc="Sets the I2C1_FMP field."]
    #[inline] pub fn set_i2c1_fmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline] pub fn i2c_pb9_fmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if I2C_PB9_FMP != 0"]
    #[inline] pub fn test_i2c_pb9_fmp(&self) -> bool {
        self.i2c_pb9_fmp() != 0
    }

    #[doc="Sets the I2C_PB9_FMP field."]
    #[inline] pub fn set_i2c_pb9_fmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline] pub fn i2c_pb8_fmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if I2C_PB8_FMP != 0"]
    #[inline] pub fn test_i2c_pb8_fmp(&self) -> bool {
        self.i2c_pb8_fmp() != 0
    }

    #[doc="Sets the I2C_PB8_FMP field."]
    #[inline] pub fn set_i2c_pb8_fmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline] pub fn i2c_pb7_fmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if I2C_PB7_FMP != 0"]
    #[inline] pub fn test_i2c_pb7_fmp(&self) -> bool {
        self.i2c_pb7_fmp() != 0
    }

    #[doc="Sets the I2C_PB7_FMP field."]
    #[inline] pub fn set_i2c_pb7_fmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline] pub fn i2c_pb6_fmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if I2C_PB6_FMP != 0"]
    #[inline] pub fn test_i2c_pb6_fmp(&self) -> bool {
        self.i2c_pb6_fmp() != 0
    }

    #[doc="Sets the I2C_PB6_FMP field."]
    #[inline] pub fn set_i2c_pb6_fmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="I/O analog switch voltage booster enable"]
    #[inline] pub fn boosten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if BOOSTEN != 0"]
    #[inline] pub fn test_boosten(&self) -> bool {
        self.boosten() != 0
    }

    #[doc="Sets the BOOSTEN field."]
    #[inline] pub fn set_boosten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Cfgr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr1(other)
    }
}

impl ::core::fmt::Display for Cfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fpu_ie() != 0 { try!(write!(f, " fpu_ie=0x{:x}", self.fpu_ie()))}
        if self.i2c3_fmp() != 0 { try!(write!(f, " i2c3_fmp"))}
        if self.i2c1_fmp() != 0 { try!(write!(f, " i2c1_fmp"))}
        if self.i2c_pb9_fmp() != 0 { try!(write!(f, " i2c_pb9_fmp"))}
        if self.i2c_pb8_fmp() != 0 { try!(write!(f, " i2c_pb8_fmp"))}
        if self.i2c_pb7_fmp() != 0 { try!(write!(f, " i2c_pb7_fmp"))}
        if self.i2c_pb6_fmp() != 0 { try!(write!(f, " i2c_pb6_fmp"))}
        if self.boosten() != 0 { try!(write!(f, " boosten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
    #[doc="EXTI 3 configuration bits"]
    #[inline] pub fn exti3(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if EXTI3 != 0"]
    #[inline] pub fn test_exti3(&self) -> bool {
        self.exti3() != 0
    }

    #[doc="Sets the EXTI3 field."]
    #[inline] pub fn set_exti3<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI 2 configuration bits"]
    #[inline] pub fn exti2(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if EXTI2 != 0"]
    #[inline] pub fn test_exti2(&self) -> bool {
        self.exti2() != 0
    }

    #[doc="Sets the EXTI2 field."]
    #[inline] pub fn set_exti2<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI 1 configuration bits"]
    #[inline] pub fn exti1(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if EXTI1 != 0"]
    #[inline] pub fn test_exti1(&self) -> bool {
        self.exti1() != 0
    }

    #[doc="Sets the EXTI1 field."]
    #[inline] pub fn set_exti1<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI 0 configuration bits"]
    #[inline] pub fn exti0(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXTI0 != 0"]
    #[inline] pub fn test_exti0(&self) -> bool {
        self.exti0() != 0
    }

    #[doc="Sets the EXTI0 field."]
    #[inline] pub fn set_exti0<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr1(other)
    }
}

impl ::core::fmt::Display for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti3() != 0 { try!(write!(f, " exti3=0x{:x}", self.exti3()))}
        if self.exti2() != 0 { try!(write!(f, " exti2=0x{:x}", self.exti2()))}
        if self.exti1() != 0 { try!(write!(f, " exti1=0x{:x}", self.exti1()))}
        if self.exti0() != 0 { try!(write!(f, " exti0=0x{:x}", self.exti0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr2(pub u32);
impl Exticr2 {
    #[doc="EXTI 7 configuration bits"]
    #[inline] pub fn exti7(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if EXTI7 != 0"]
    #[inline] pub fn test_exti7(&self) -> bool {
        self.exti7() != 0
    }

    #[doc="Sets the EXTI7 field."]
    #[inline] pub fn set_exti7<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI 6 configuration bits"]
    #[inline] pub fn exti6(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if EXTI6 != 0"]
    #[inline] pub fn test_exti6(&self) -> bool {
        self.exti6() != 0
    }

    #[doc="Sets the EXTI6 field."]
    #[inline] pub fn set_exti6<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI 5 configuration bits"]
    #[inline] pub fn exti5(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if EXTI5 != 0"]
    #[inline] pub fn test_exti5(&self) -> bool {
        self.exti5() != 0
    }

    #[doc="Sets the EXTI5 field."]
    #[inline] pub fn set_exti5<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI 4 configuration bits"]
    #[inline] pub fn exti4(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXTI4 != 0"]
    #[inline] pub fn test_exti4(&self) -> bool {
        self.exti4() != 0
    }

    #[doc="Sets the EXTI4 field."]
    #[inline] pub fn set_exti4<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr2(other)
    }
}

impl ::core::fmt::Display for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti7() != 0 { try!(write!(f, " exti7=0x{:x}", self.exti7()))}
        if self.exti6() != 0 { try!(write!(f, " exti6=0x{:x}", self.exti6()))}
        if self.exti5() != 0 { try!(write!(f, " exti5=0x{:x}", self.exti5()))}
        if self.exti4() != 0 { try!(write!(f, " exti4=0x{:x}", self.exti4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr3(pub u32);
impl Exticr3 {
    #[doc="EXTI 11 configuration bits"]
    #[inline] pub fn exti11(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if EXTI11 != 0"]
    #[inline] pub fn test_exti11(&self) -> bool {
        self.exti11() != 0
    }

    #[doc="Sets the EXTI11 field."]
    #[inline] pub fn set_exti11<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI 10 configuration bits"]
    #[inline] pub fn exti10(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if EXTI10 != 0"]
    #[inline] pub fn test_exti10(&self) -> bool {
        self.exti10() != 0
    }

    #[doc="Sets the EXTI10 field."]
    #[inline] pub fn set_exti10<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI 9 configuration bits"]
    #[inline] pub fn exti9(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if EXTI9 != 0"]
    #[inline] pub fn test_exti9(&self) -> bool {
        self.exti9() != 0
    }

    #[doc="Sets the EXTI9 field."]
    #[inline] pub fn set_exti9<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI 8 configuration bits"]
    #[inline] pub fn exti8(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXTI8 != 0"]
    #[inline] pub fn test_exti8(&self) -> bool {
        self.exti8() != 0
    }

    #[doc="Sets the EXTI8 field."]
    #[inline] pub fn set_exti8<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr3(other)
    }
}

impl ::core::fmt::Display for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti11() != 0 { try!(write!(f, " exti11=0x{:x}", self.exti11()))}
        if self.exti10() != 0 { try!(write!(f, " exti10=0x{:x}", self.exti10()))}
        if self.exti9() != 0 { try!(write!(f, " exti9=0x{:x}", self.exti9()))}
        if self.exti8() != 0 { try!(write!(f, " exti8=0x{:x}", self.exti8()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr4(pub u32);
impl Exticr4 {
    #[doc="EXTI15 configuration bits"]
    #[inline] pub fn exti15(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x7) as u8) } // [14:12]
    }

    #[doc="Returns true if EXTI15 != 0"]
    #[inline] pub fn test_exti15(&self) -> bool {
        self.exti15() != 0
    }

    #[doc="Sets the EXTI15 field."]
    #[inline] pub fn set_exti15<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI14 configuration bits"]
    #[inline] pub fn exti14(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if EXTI14 != 0"]
    #[inline] pub fn test_exti14(&self) -> bool {
        self.exti14() != 0
    }

    #[doc="Sets the EXTI14 field."]
    #[inline] pub fn set_exti14<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI13 configuration bits"]
    #[inline] pub fn exti13(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if EXTI13 != 0"]
    #[inline] pub fn test_exti13(&self) -> bool {
        self.exti13() != 0
    }

    #[doc="Sets the EXTI13 field."]
    #[inline] pub fn set_exti13<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI12 configuration bits"]
    #[inline] pub fn exti12(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EXTI12 != 0"]
    #[inline] pub fn test_exti12(&self) -> bool {
        self.exti12() != 0
    }

    #[doc="Sets the EXTI12 field."]
    #[inline] pub fn set_exti12<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr4(other)
    }
}

impl ::core::fmt::Display for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti15() != 0 { try!(write!(f, " exti15=0x{:x}", self.exti15()))}
        if self.exti14() != 0 { try!(write!(f, " exti14=0x{:x}", self.exti14()))}
        if self.exti13() != 0 { try!(write!(f, " exti13=0x{:x}", self.exti13()))}
        if self.exti12() != 0 { try!(write!(f, " exti12=0x{:x}", self.exti12()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SCSR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scsr(pub u32);
impl Scsr {
    #[doc="SRAM2 busy by erase operation"]
    #[inline] pub fn sram2bsy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SRAM2BSY != 0"]
    #[inline] pub fn test_sram2bsy(&self) -> bool {
        self.sram2bsy() != 0
    }

    #[doc="Sets the SRAM2BSY field."]
    #[inline] pub fn set_sram2bsy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SRAM2 Erase"]
    #[inline] pub fn sram2er(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SRAM2ER != 0"]
    #[inline] pub fn test_sram2er(&self) -> bool {
        self.sram2er() != 0
    }

    #[doc="Sets the SRAM2ER field."]
    #[inline] pub fn set_sram2er<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CPU2 SRAM fetch (execution) disable."]
    #[inline] pub fn c2rfd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if C2RFD != 0"]
    #[inline] pub fn test_c2rfd(&self) -> bool {
        self.c2rfd() != 0
    }

    #[doc="Sets the C2RFD field."]
    #[inline] pub fn set_c2rfd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Scsr {
    #[inline]
    fn from(other: u32) -> Self {
         Scsr(other)
    }
}

impl ::core::fmt::Display for Scsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sram2bsy() != 0 { try!(write!(f, " sram2bsy"))}
        if self.sram2er() != 0 { try!(write!(f, " sram2er"))}
        if self.c2rfd() != 0 { try!(write!(f, " c2rfd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CFGR2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
    #[doc="SRAM2 parity error flag"]
    #[inline] pub fn spf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SPF != 0"]
    #[inline] pub fn test_spf(&self) -> bool {
        self.spf() != 0
    }

    #[doc="Sets the SPF field."]
    #[inline] pub fn set_spf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ECC Lock"]
    #[inline] pub fn eccl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ECCL != 0"]
    #[inline] pub fn test_eccl(&self) -> bool {
        self.eccl() != 0
    }

    #[doc="Sets the ECCL field."]
    #[inline] pub fn set_eccl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PVD lock enable bit"]
    #[inline] pub fn pvdl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PVDL != 0"]
    #[inline] pub fn test_pvdl(&self) -> bool {
        self.pvdl() != 0
    }

    #[doc="Sets the PVDL field."]
    #[inline] pub fn set_pvdl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SRAM2 parity lock bit"]
    #[inline] pub fn spl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SPL != 0"]
    #[inline] pub fn test_spl(&self) -> bool {
        self.spl() != 0
    }

    #[doc="Sets the SPL field."]
    #[inline] pub fn set_spl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Cortex-M4 LOCKUP (Hardfault) output enable bit"]
    #[inline] pub fn cll(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CLL != 0"]
    #[inline] pub fn test_cll(&self) -> bool {
        self.cll() != 0
    }

    #[doc="Sets the CLL field."]
    #[inline] pub fn set_cll<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr2(other)
    }
}

impl ::core::fmt::Display for Cfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.spf() != 0 { try!(write!(f, " spf"))}
        if self.eccl() != 0 { try!(write!(f, " eccl"))}
        if self.pvdl() != 0 { try!(write!(f, " pvdl"))}
        if self.spl() != 0 { try!(write!(f, " spl"))}
        if self.cll() != 0 { try!(write!(f, " cll"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM2 write protection register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swpr(pub u32);
impl Swpr {
    #[doc="SRAM2 page 31 write protection"]
    #[inline] pub fn p31wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if P31WP != 0"]
    #[inline] pub fn test_p31wp(&self) -> bool {
        self.p31wp() != 0
    }

    #[doc="Sets the P31WP field."]
    #[inline] pub fn set_p31wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="P30WP"]
    #[inline] pub fn p30wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if P30WP != 0"]
    #[inline] pub fn test_p30wp(&self) -> bool {
        self.p30wp() != 0
    }

    #[doc="Sets the P30WP field."]
    #[inline] pub fn set_p30wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="P29WP"]
    #[inline] pub fn p29wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if P29WP != 0"]
    #[inline] pub fn test_p29wp(&self) -> bool {
        self.p29wp() != 0
    }

    #[doc="Sets the P29WP field."]
    #[inline] pub fn set_p29wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="P28WP"]
    #[inline] pub fn p28wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if P28WP != 0"]
    #[inline] pub fn test_p28wp(&self) -> bool {
        self.p28wp() != 0
    }

    #[doc="Sets the P28WP field."]
    #[inline] pub fn set_p28wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="P27WP"]
    #[inline] pub fn p27wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if P27WP != 0"]
    #[inline] pub fn test_p27wp(&self) -> bool {
        self.p27wp() != 0
    }

    #[doc="Sets the P27WP field."]
    #[inline] pub fn set_p27wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="P26WP"]
    #[inline] pub fn p26wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if P26WP != 0"]
    #[inline] pub fn test_p26wp(&self) -> bool {
        self.p26wp() != 0
    }

    #[doc="Sets the P26WP field."]
    #[inline] pub fn set_p26wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="P25WP"]
    #[inline] pub fn p25wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if P25WP != 0"]
    #[inline] pub fn test_p25wp(&self) -> bool {
        self.p25wp() != 0
    }

    #[doc="Sets the P25WP field."]
    #[inline] pub fn set_p25wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="P24WP"]
    #[inline] pub fn p24wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if P24WP != 0"]
    #[inline] pub fn test_p24wp(&self) -> bool {
        self.p24wp() != 0
    }

    #[doc="Sets the P24WP field."]
    #[inline] pub fn set_p24wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="P23WP"]
    #[inline] pub fn p23wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if P23WP != 0"]
    #[inline] pub fn test_p23wp(&self) -> bool {
        self.p23wp() != 0
    }

    #[doc="Sets the P23WP field."]
    #[inline] pub fn set_p23wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="P22WP"]
    #[inline] pub fn p22wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if P22WP != 0"]
    #[inline] pub fn test_p22wp(&self) -> bool {
        self.p22wp() != 0
    }

    #[doc="Sets the P22WP field."]
    #[inline] pub fn set_p22wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="P21WP"]
    #[inline] pub fn p21wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if P21WP != 0"]
    #[inline] pub fn test_p21wp(&self) -> bool {
        self.p21wp() != 0
    }

    #[doc="Sets the P21WP field."]
    #[inline] pub fn set_p21wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="P20WP"]
    #[inline] pub fn p20wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if P20WP != 0"]
    #[inline] pub fn test_p20wp(&self) -> bool {
        self.p20wp() != 0
    }

    #[doc="Sets the P20WP field."]
    #[inline] pub fn set_p20wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="P19WP"]
    #[inline] pub fn p19wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if P19WP != 0"]
    #[inline] pub fn test_p19wp(&self) -> bool {
        self.p19wp() != 0
    }

    #[doc="Sets the P19WP field."]
    #[inline] pub fn set_p19wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="P18WP"]
    #[inline] pub fn p18wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if P18WP != 0"]
    #[inline] pub fn test_p18wp(&self) -> bool {
        self.p18wp() != 0
    }

    #[doc="Sets the P18WP field."]
    #[inline] pub fn set_p18wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="P17WP"]
    #[inline] pub fn p17wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if P17WP != 0"]
    #[inline] pub fn test_p17wp(&self) -> bool {
        self.p17wp() != 0
    }

    #[doc="Sets the P17WP field."]
    #[inline] pub fn set_p17wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="P16WP"]
    #[inline] pub fn p16wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if P16WP != 0"]
    #[inline] pub fn test_p16wp(&self) -> bool {
        self.p16wp() != 0
    }

    #[doc="Sets the P16WP field."]
    #[inline] pub fn set_p16wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="P15WP"]
    #[inline] pub fn p15wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if P15WP != 0"]
    #[inline] pub fn test_p15wp(&self) -> bool {
        self.p15wp() != 0
    }

    #[doc="Sets the P15WP field."]
    #[inline] pub fn set_p15wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="P14WP"]
    #[inline] pub fn p14wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if P14WP != 0"]
    #[inline] pub fn test_p14wp(&self) -> bool {
        self.p14wp() != 0
    }

    #[doc="Sets the P14WP field."]
    #[inline] pub fn set_p14wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="P13WP"]
    #[inline] pub fn p13wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if P13WP != 0"]
    #[inline] pub fn test_p13wp(&self) -> bool {
        self.p13wp() != 0
    }

    #[doc="Sets the P13WP field."]
    #[inline] pub fn set_p13wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="P12WP"]
    #[inline] pub fn p12wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if P12WP != 0"]
    #[inline] pub fn test_p12wp(&self) -> bool {
        self.p12wp() != 0
    }

    #[doc="Sets the P12WP field."]
    #[inline] pub fn set_p12wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="P11WP"]
    #[inline] pub fn p11wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if P11WP != 0"]
    #[inline] pub fn test_p11wp(&self) -> bool {
        self.p11wp() != 0
    }

    #[doc="Sets the P11WP field."]
    #[inline] pub fn set_p11wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="P10WP"]
    #[inline] pub fn p10wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if P10WP != 0"]
    #[inline] pub fn test_p10wp(&self) -> bool {
        self.p10wp() != 0
    }

    #[doc="Sets the P10WP field."]
    #[inline] pub fn set_p10wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="P9WP"]
    #[inline] pub fn p9wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if P9WP != 0"]
    #[inline] pub fn test_p9wp(&self) -> bool {
        self.p9wp() != 0
    }

    #[doc="Sets the P9WP field."]
    #[inline] pub fn set_p9wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="P8WP"]
    #[inline] pub fn p8wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if P8WP != 0"]
    #[inline] pub fn test_p8wp(&self) -> bool {
        self.p8wp() != 0
    }

    #[doc="Sets the P8WP field."]
    #[inline] pub fn set_p8wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="P7WP"]
    #[inline] pub fn p7wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if P7WP != 0"]
    #[inline] pub fn test_p7wp(&self) -> bool {
        self.p7wp() != 0
    }

    #[doc="Sets the P7WP field."]
    #[inline] pub fn set_p7wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="P6WP"]
    #[inline] pub fn p6wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if P6WP != 0"]
    #[inline] pub fn test_p6wp(&self) -> bool {
        self.p6wp() != 0
    }

    #[doc="Sets the P6WP field."]
    #[inline] pub fn set_p6wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="P5WP"]
    #[inline] pub fn p5wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if P5WP != 0"]
    #[inline] pub fn test_p5wp(&self) -> bool {
        self.p5wp() != 0
    }

    #[doc="Sets the P5WP field."]
    #[inline] pub fn set_p5wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="P4WP"]
    #[inline] pub fn p4wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if P4WP != 0"]
    #[inline] pub fn test_p4wp(&self) -> bool {
        self.p4wp() != 0
    }

    #[doc="Sets the P4WP field."]
    #[inline] pub fn set_p4wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="P3WP"]
    #[inline] pub fn p3wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if P3WP != 0"]
    #[inline] pub fn test_p3wp(&self) -> bool {
        self.p3wp() != 0
    }

    #[doc="Sets the P3WP field."]
    #[inline] pub fn set_p3wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="P2WP"]
    #[inline] pub fn p2wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if P2WP != 0"]
    #[inline] pub fn test_p2wp(&self) -> bool {
        self.p2wp() != 0
    }

    #[doc="Sets the P2WP field."]
    #[inline] pub fn set_p2wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="P1WP"]
    #[inline] pub fn p1wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if P1WP != 0"]
    #[inline] pub fn test_p1wp(&self) -> bool {
        self.p1wp() != 0
    }

    #[doc="Sets the P1WP field."]
    #[inline] pub fn set_p1wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="P0WP"]
    #[inline] pub fn p0wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if P0WP != 0"]
    #[inline] pub fn test_p0wp(&self) -> bool {
        self.p0wp() != 0
    }

    #[doc="Sets the P0WP field."]
    #[inline] pub fn set_p0wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Swpr {
    #[inline]
    fn from(other: u32) -> Self {
         Swpr(other)
    }
}

impl ::core::fmt::Display for Swpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p31wp() != 0 { try!(write!(f, " p31wp"))}
        if self.p30wp() != 0 { try!(write!(f, " p30wp"))}
        if self.p29wp() != 0 { try!(write!(f, " p29wp"))}
        if self.p28wp() != 0 { try!(write!(f, " p28wp"))}
        if self.p27wp() != 0 { try!(write!(f, " p27wp"))}
        if self.p26wp() != 0 { try!(write!(f, " p26wp"))}
        if self.p25wp() != 0 { try!(write!(f, " p25wp"))}
        if self.p24wp() != 0 { try!(write!(f, " p24wp"))}
        if self.p23wp() != 0 { try!(write!(f, " p23wp"))}
        if self.p22wp() != 0 { try!(write!(f, " p22wp"))}
        if self.p21wp() != 0 { try!(write!(f, " p21wp"))}
        if self.p20wp() != 0 { try!(write!(f, " p20wp"))}
        if self.p19wp() != 0 { try!(write!(f, " p19wp"))}
        if self.p18wp() != 0 { try!(write!(f, " p18wp"))}
        if self.p17wp() != 0 { try!(write!(f, " p17wp"))}
        if self.p16wp() != 0 { try!(write!(f, " p16wp"))}
        if self.p15wp() != 0 { try!(write!(f, " p15wp"))}
        if self.p14wp() != 0 { try!(write!(f, " p14wp"))}
        if self.p13wp() != 0 { try!(write!(f, " p13wp"))}
        if self.p12wp() != 0 { try!(write!(f, " p12wp"))}
        if self.p11wp() != 0 { try!(write!(f, " p11wp"))}
        if self.p10wp() != 0 { try!(write!(f, " p10wp"))}
        if self.p9wp() != 0 { try!(write!(f, " p9wp"))}
        if self.p8wp() != 0 { try!(write!(f, " p8wp"))}
        if self.p7wp() != 0 { try!(write!(f, " p7wp"))}
        if self.p6wp() != 0 { try!(write!(f, " p6wp"))}
        if self.p5wp() != 0 { try!(write!(f, " p5wp"))}
        if self.p4wp() != 0 { try!(write!(f, " p4wp"))}
        if self.p3wp() != 0 { try!(write!(f, " p3wp"))}
        if self.p2wp() != 0 { try!(write!(f, " p2wp"))}
        if self.p1wp() != 0 { try!(write!(f, " p1wp"))}
        if self.p0wp() != 0 { try!(write!(f, " p0wp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SKR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Skr(pub u32);
impl Skr {
    #[doc="SRAM2 write protection key for software erase"]
    #[inline] pub fn key(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Skr {
    #[inline]
    fn from(other: u32) -> Self {
         Skr(other)
    }
}

impl ::core::fmt::Display for Skr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Skr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SRAM2 write protection register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swpr2(pub u32);
impl Swpr2 {
    #[doc="SRAM2 page 63 write protection"]
    #[inline] pub fn p63wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if P63WP != 0"]
    #[inline] pub fn test_p63wp(&self) -> bool {
        self.p63wp() != 0
    }

    #[doc="Sets the P63WP field."]
    #[inline] pub fn set_p63wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="P62WP"]
    #[inline] pub fn p62wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if P62WP != 0"]
    #[inline] pub fn test_p62wp(&self) -> bool {
        self.p62wp() != 0
    }

    #[doc="Sets the P62WP field."]
    #[inline] pub fn set_p62wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="P61WP"]
    #[inline] pub fn p61wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if P61WP != 0"]
    #[inline] pub fn test_p61wp(&self) -> bool {
        self.p61wp() != 0
    }

    #[doc="Sets the P61WP field."]
    #[inline] pub fn set_p61wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="P60WP"]
    #[inline] pub fn p60wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if P60WP != 0"]
    #[inline] pub fn test_p60wp(&self) -> bool {
        self.p60wp() != 0
    }

    #[doc="Sets the P60WP field."]
    #[inline] pub fn set_p60wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="P59WP"]
    #[inline] pub fn p59wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if P59WP != 0"]
    #[inline] pub fn test_p59wp(&self) -> bool {
        self.p59wp() != 0
    }

    #[doc="Sets the P59WP field."]
    #[inline] pub fn set_p59wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="P58WP"]
    #[inline] pub fn p58wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if P58WP != 0"]
    #[inline] pub fn test_p58wp(&self) -> bool {
        self.p58wp() != 0
    }

    #[doc="Sets the P58WP field."]
    #[inline] pub fn set_p58wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="P57WP"]
    #[inline] pub fn p57wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if P57WP != 0"]
    #[inline] pub fn test_p57wp(&self) -> bool {
        self.p57wp() != 0
    }

    #[doc="Sets the P57WP field."]
    #[inline] pub fn set_p57wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="P56WP"]
    #[inline] pub fn p56wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if P56WP != 0"]
    #[inline] pub fn test_p56wp(&self) -> bool {
        self.p56wp() != 0
    }

    #[doc="Sets the P56WP field."]
    #[inline] pub fn set_p56wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="P55WP"]
    #[inline] pub fn p55wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if P55WP != 0"]
    #[inline] pub fn test_p55wp(&self) -> bool {
        self.p55wp() != 0
    }

    #[doc="Sets the P55WP field."]
    #[inline] pub fn set_p55wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="P54WP"]
    #[inline] pub fn p54wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if P54WP != 0"]
    #[inline] pub fn test_p54wp(&self) -> bool {
        self.p54wp() != 0
    }

    #[doc="Sets the P54WP field."]
    #[inline] pub fn set_p54wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="P53WP"]
    #[inline] pub fn p53wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if P53WP != 0"]
    #[inline] pub fn test_p53wp(&self) -> bool {
        self.p53wp() != 0
    }

    #[doc="Sets the P53WP field."]
    #[inline] pub fn set_p53wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="P52WP"]
    #[inline] pub fn p52wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if P52WP != 0"]
    #[inline] pub fn test_p52wp(&self) -> bool {
        self.p52wp() != 0
    }

    #[doc="Sets the P52WP field."]
    #[inline] pub fn set_p52wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="P51WP"]
    #[inline] pub fn p51wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if P51WP != 0"]
    #[inline] pub fn test_p51wp(&self) -> bool {
        self.p51wp() != 0
    }

    #[doc="Sets the P51WP field."]
    #[inline] pub fn set_p51wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="P50WP"]
    #[inline] pub fn p50wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if P50WP != 0"]
    #[inline] pub fn test_p50wp(&self) -> bool {
        self.p50wp() != 0
    }

    #[doc="Sets the P50WP field."]
    #[inline] pub fn set_p50wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="P49WP"]
    #[inline] pub fn p49wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if P49WP != 0"]
    #[inline] pub fn test_p49wp(&self) -> bool {
        self.p49wp() != 0
    }

    #[doc="Sets the P49WP field."]
    #[inline] pub fn set_p49wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="P48WP"]
    #[inline] pub fn p48wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if P48WP != 0"]
    #[inline] pub fn test_p48wp(&self) -> bool {
        self.p48wp() != 0
    }

    #[doc="Sets the P48WP field."]
    #[inline] pub fn set_p48wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="P47WP"]
    #[inline] pub fn p47wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if P47WP != 0"]
    #[inline] pub fn test_p47wp(&self) -> bool {
        self.p47wp() != 0
    }

    #[doc="Sets the P47WP field."]
    #[inline] pub fn set_p47wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="P46WP"]
    #[inline] pub fn p46wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if P46WP != 0"]
    #[inline] pub fn test_p46wp(&self) -> bool {
        self.p46wp() != 0
    }

    #[doc="Sets the P46WP field."]
    #[inline] pub fn set_p46wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="P45WP"]
    #[inline] pub fn p45wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if P45WP != 0"]
    #[inline] pub fn test_p45wp(&self) -> bool {
        self.p45wp() != 0
    }

    #[doc="Sets the P45WP field."]
    #[inline] pub fn set_p45wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="P44WP"]
    #[inline] pub fn p44wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if P44WP != 0"]
    #[inline] pub fn test_p44wp(&self) -> bool {
        self.p44wp() != 0
    }

    #[doc="Sets the P44WP field."]
    #[inline] pub fn set_p44wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="P43WP"]
    #[inline] pub fn p43wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if P43WP != 0"]
    #[inline] pub fn test_p43wp(&self) -> bool {
        self.p43wp() != 0
    }

    #[doc="Sets the P43WP field."]
    #[inline] pub fn set_p43wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="P42WP"]
    #[inline] pub fn p42wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if P42WP != 0"]
    #[inline] pub fn test_p42wp(&self) -> bool {
        self.p42wp() != 0
    }

    #[doc="Sets the P42WP field."]
    #[inline] pub fn set_p42wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="P41WP"]
    #[inline] pub fn p41wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if P41WP != 0"]
    #[inline] pub fn test_p41wp(&self) -> bool {
        self.p41wp() != 0
    }

    #[doc="Sets the P41WP field."]
    #[inline] pub fn set_p41wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="P40WP"]
    #[inline] pub fn p40wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if P40WP != 0"]
    #[inline] pub fn test_p40wp(&self) -> bool {
        self.p40wp() != 0
    }

    #[doc="Sets the P40WP field."]
    #[inline] pub fn set_p40wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="P39WP"]
    #[inline] pub fn p39wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if P39WP != 0"]
    #[inline] pub fn test_p39wp(&self) -> bool {
        self.p39wp() != 0
    }

    #[doc="Sets the P39WP field."]
    #[inline] pub fn set_p39wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="P38WP"]
    #[inline] pub fn p38wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if P38WP != 0"]
    #[inline] pub fn test_p38wp(&self) -> bool {
        self.p38wp() != 0
    }

    #[doc="Sets the P38WP field."]
    #[inline] pub fn set_p38wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="P37WP"]
    #[inline] pub fn p37wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if P37WP != 0"]
    #[inline] pub fn test_p37wp(&self) -> bool {
        self.p37wp() != 0
    }

    #[doc="Sets the P37WP field."]
    #[inline] pub fn set_p37wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="P36WP"]
    #[inline] pub fn p36wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if P36WP != 0"]
    #[inline] pub fn test_p36wp(&self) -> bool {
        self.p36wp() != 0
    }

    #[doc="Sets the P36WP field."]
    #[inline] pub fn set_p36wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="P35WP"]
    #[inline] pub fn p35wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if P35WP != 0"]
    #[inline] pub fn test_p35wp(&self) -> bool {
        self.p35wp() != 0
    }

    #[doc="Sets the P35WP field."]
    #[inline] pub fn set_p35wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="P34WP"]
    #[inline] pub fn p34wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if P34WP != 0"]
    #[inline] pub fn test_p34wp(&self) -> bool {
        self.p34wp() != 0
    }

    #[doc="Sets the P34WP field."]
    #[inline] pub fn set_p34wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="P33WP"]
    #[inline] pub fn p33wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if P33WP != 0"]
    #[inline] pub fn test_p33wp(&self) -> bool {
        self.p33wp() != 0
    }

    #[doc="Sets the P33WP field."]
    #[inline] pub fn set_p33wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="P32WP"]
    #[inline] pub fn p32wp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if P32WP != 0"]
    #[inline] pub fn test_p32wp(&self) -> bool {
        self.p32wp() != 0
    }

    #[doc="Sets the P32WP field."]
    #[inline] pub fn set_p32wp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Swpr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Swpr2(other)
    }
}

impl ::core::fmt::Display for Swpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swpr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.p63wp() != 0 { try!(write!(f, " p63wp"))}
        if self.p62wp() != 0 { try!(write!(f, " p62wp"))}
        if self.p61wp() != 0 { try!(write!(f, " p61wp"))}
        if self.p60wp() != 0 { try!(write!(f, " p60wp"))}
        if self.p59wp() != 0 { try!(write!(f, " p59wp"))}
        if self.p58wp() != 0 { try!(write!(f, " p58wp"))}
        if self.p57wp() != 0 { try!(write!(f, " p57wp"))}
        if self.p56wp() != 0 { try!(write!(f, " p56wp"))}
        if self.p55wp() != 0 { try!(write!(f, " p55wp"))}
        if self.p54wp() != 0 { try!(write!(f, " p54wp"))}
        if self.p53wp() != 0 { try!(write!(f, " p53wp"))}
        if self.p52wp() != 0 { try!(write!(f, " p52wp"))}
        if self.p51wp() != 0 { try!(write!(f, " p51wp"))}
        if self.p50wp() != 0 { try!(write!(f, " p50wp"))}
        if self.p49wp() != 0 { try!(write!(f, " p49wp"))}
        if self.p48wp() != 0 { try!(write!(f, " p48wp"))}
        if self.p47wp() != 0 { try!(write!(f, " p47wp"))}
        if self.p46wp() != 0 { try!(write!(f, " p46wp"))}
        if self.p45wp() != 0 { try!(write!(f, " p45wp"))}
        if self.p44wp() != 0 { try!(write!(f, " p44wp"))}
        if self.p43wp() != 0 { try!(write!(f, " p43wp"))}
        if self.p42wp() != 0 { try!(write!(f, " p42wp"))}
        if self.p41wp() != 0 { try!(write!(f, " p41wp"))}
        if self.p40wp() != 0 { try!(write!(f, " p40wp"))}
        if self.p39wp() != 0 { try!(write!(f, " p39wp"))}
        if self.p38wp() != 0 { try!(write!(f, " p38wp"))}
        if self.p37wp() != 0 { try!(write!(f, " p37wp"))}
        if self.p36wp() != 0 { try!(write!(f, " p36wp"))}
        if self.p35wp() != 0 { try!(write!(f, " p35wp"))}
        if self.p34wp() != 0 { try!(write!(f, " p34wp"))}
        if self.p33wp() != 0 { try!(write!(f, " p33wp"))}
        if self.p32wp() != 0 { try!(write!(f, " p32wp"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU1 interrupt mask register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imr1(pub u32);
impl Imr1 {
    #[doc="Peripheral TIM1 interrupt mask to CPU1"]
    #[inline] pub fn tim1im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TIM1IM != 0"]
    #[inline] pub fn test_tim1im(&self) -> bool {
        self.tim1im() != 0
    }

    #[doc="Sets the TIM1IM field."]
    #[inline] pub fn set_tim1im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Peripheral TIM16 interrupt mask to CPU1"]
    #[inline] pub fn tim16im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TIM16IM != 0"]
    #[inline] pub fn test_tim16im(&self) -> bool {
        self.tim16im() != 0
    }

    #[doc="Sets the TIM16IM field."]
    #[inline] pub fn set_tim16im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Peripheral TIM17 interrupt mask to CPU1"]
    #[inline] pub fn tim17im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TIM17IM != 0"]
    #[inline] pub fn test_tim17im(&self) -> bool {
        self.tim17im() != 0
    }

    #[doc="Sets the TIM17IM field."]
    #[inline] pub fn set_tim17im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Peripheral EXIT5 interrupt mask to CPU1"]
    #[inline] pub fn exit5im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if EXIT5IM != 0"]
    #[inline] pub fn test_exit5im(&self) -> bool {
        self.exit5im() != 0
    }

    #[doc="Sets the EXIT5IM field."]
    #[inline] pub fn set_exit5im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Peripheral EXIT6 interrupt mask to CPU1"]
    #[inline] pub fn exit6im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if EXIT6IM != 0"]
    #[inline] pub fn test_exit6im(&self) -> bool {
        self.exit6im() != 0
    }

    #[doc="Sets the EXIT6IM field."]
    #[inline] pub fn set_exit6im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Peripheral EXIT7 interrupt mask to CPU1"]
    #[inline] pub fn exit7im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if EXIT7IM != 0"]
    #[inline] pub fn test_exit7im(&self) -> bool {
        self.exit7im() != 0
    }

    #[doc="Sets the EXIT7IM field."]
    #[inline] pub fn set_exit7im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Peripheral EXIT8 interrupt mask to CPU1"]
    #[inline] pub fn exit8im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if EXIT8IM != 0"]
    #[inline] pub fn test_exit8im(&self) -> bool {
        self.exit8im() != 0
    }

    #[doc="Sets the EXIT8IM field."]
    #[inline] pub fn set_exit8im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Peripheral EXIT9 interrupt mask to CPU1"]
    #[inline] pub fn exit9im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if EXIT9IM != 0"]
    #[inline] pub fn test_exit9im(&self) -> bool {
        self.exit9im() != 0
    }

    #[doc="Sets the EXIT9IM field."]
    #[inline] pub fn set_exit9im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Peripheral EXIT10 interrupt mask to CPU1"]
    #[inline] pub fn exit10im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if EXIT10IM != 0"]
    #[inline] pub fn test_exit10im(&self) -> bool {
        self.exit10im() != 0
    }

    #[doc="Sets the EXIT10IM field."]
    #[inline] pub fn set_exit10im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Peripheral EXIT11 interrupt mask to CPU1"]
    #[inline] pub fn exit11im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if EXIT11IM != 0"]
    #[inline] pub fn test_exit11im(&self) -> bool {
        self.exit11im() != 0
    }

    #[doc="Sets the EXIT11IM field."]
    #[inline] pub fn set_exit11im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Peripheral EXIT12 interrupt mask to CPU1"]
    #[inline] pub fn exit12im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if EXIT12IM != 0"]
    #[inline] pub fn test_exit12im(&self) -> bool {
        self.exit12im() != 0
    }

    #[doc="Sets the EXIT12IM field."]
    #[inline] pub fn set_exit12im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Peripheral EXIT13 interrupt mask to CPU1"]
    #[inline] pub fn exit13im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if EXIT13IM != 0"]
    #[inline] pub fn test_exit13im(&self) -> bool {
        self.exit13im() != 0
    }

    #[doc="Sets the EXIT13IM field."]
    #[inline] pub fn set_exit13im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Peripheral EXIT14 interrupt mask to CPU1"]
    #[inline] pub fn exit14im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if EXIT14IM != 0"]
    #[inline] pub fn test_exit14im(&self) -> bool {
        self.exit14im() != 0
    }

    #[doc="Sets the EXIT14IM field."]
    #[inline] pub fn set_exit14im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Peripheral EXIT15 interrupt mask to CPU1"]
    #[inline] pub fn exit15im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EXIT15IM != 0"]
    #[inline] pub fn test_exit15im(&self) -> bool {
        self.exit15im() != 0
    }

    #[doc="Sets the EXIT15IM field."]
    #[inline] pub fn set_exit15im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Imr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Imr1(other)
    }
}

impl ::core::fmt::Display for Imr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tim1im() != 0 { try!(write!(f, " tim1im"))}
        if self.tim16im() != 0 { try!(write!(f, " tim16im"))}
        if self.tim17im() != 0 { try!(write!(f, " tim17im"))}
        if self.exit5im() != 0 { try!(write!(f, " exit5im"))}
        if self.exit6im() != 0 { try!(write!(f, " exit6im"))}
        if self.exit7im() != 0 { try!(write!(f, " exit7im"))}
        if self.exit8im() != 0 { try!(write!(f, " exit8im"))}
        if self.exit9im() != 0 { try!(write!(f, " exit9im"))}
        if self.exit10im() != 0 { try!(write!(f, " exit10im"))}
        if self.exit11im() != 0 { try!(write!(f, " exit11im"))}
        if self.exit12im() != 0 { try!(write!(f, " exit12im"))}
        if self.exit13im() != 0 { try!(write!(f, " exit13im"))}
        if self.exit14im() != 0 { try!(write!(f, " exit14im"))}
        if self.exit15im() != 0 { try!(write!(f, " exit15im"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU1 interrupt mask register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imr2(pub u32);
impl Imr2 {
    #[doc="Peripheral PVM3 interrupt mask to CPU1"]
    #[inline] pub fn pvm3im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PVM3IM != 0"]
    #[inline] pub fn test_pvm3im(&self) -> bool {
        self.pvm3im() != 0
    }

    #[doc="Sets the PVM3IM field."]
    #[inline] pub fn set_pvm3im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Peripheral PVM1 interrupt mask to CPU1"]
    #[inline] pub fn pvm1im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PVM1IM != 0"]
    #[inline] pub fn test_pvm1im(&self) -> bool {
        self.pvm1im() != 0
    }

    #[doc="Sets the PVM1IM field."]
    #[inline] pub fn set_pvm1im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Peripheral PVD interrupt mask to CPU1"]
    #[inline] pub fn pvdim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PVDIM != 0"]
    #[inline] pub fn test_pvdim(&self) -> bool {
        self.pvdim() != 0
    }

    #[doc="Sets the PVDIM field."]
    #[inline] pub fn set_pvdim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl From<u32> for Imr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Imr2(other)
    }
}

impl ::core::fmt::Display for Imr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pvm3im() != 0 { try!(write!(f, " pvm3im"))}
        if self.pvm1im() != 0 { try!(write!(f, " pvm1im"))}
        if self.pvdim() != 0 { try!(write!(f, " pvdim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 interrupt mask register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2imr1(pub u32);
impl C2imr1 {
    #[doc="Peripheral RTCSTAMP interrupt mask to CPU2"]
    #[inline] pub fn rtcstamp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RTCSTAMP != 0"]
    #[inline] pub fn test_rtcstamp(&self) -> bool {
        self.rtcstamp() != 0
    }

    #[doc="Sets the RTCSTAMP field."]
    #[inline] pub fn set_rtcstamp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Peripheral RTCWKUP interrupt mask to CPU2"]
    #[inline] pub fn rtcwkup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RTCWKUP != 0"]
    #[inline] pub fn test_rtcwkup(&self) -> bool {
        self.rtcwkup() != 0
    }

    #[doc="Sets the RTCWKUP field."]
    #[inline] pub fn set_rtcwkup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Peripheral RTCALARM interrupt mask to CPU2"]
    #[inline] pub fn rtcalarm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RTCALARM != 0"]
    #[inline] pub fn test_rtcalarm(&self) -> bool {
        self.rtcalarm() != 0
    }

    #[doc="Sets the RTCALARM field."]
    #[inline] pub fn set_rtcalarm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Peripheral RCC interrupt mask to CPU2"]
    #[inline] pub fn rcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RCC != 0"]
    #[inline] pub fn test_rcc(&self) -> bool {
        self.rcc() != 0
    }

    #[doc="Sets the RCC field."]
    #[inline] pub fn set_rcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Peripheral FLASH interrupt mask to CPU2"]
    #[inline] pub fn flash(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FLASH != 0"]
    #[inline] pub fn test_flash(&self) -> bool {
        self.flash() != 0
    }

    #[doc="Sets the FLASH field."]
    #[inline] pub fn set_flash<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Peripheral PKA interrupt mask to CPU2"]
    #[inline] pub fn pka(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PKA != 0"]
    #[inline] pub fn test_pka(&self) -> bool {
        self.pka() != 0
    }

    #[doc="Sets the PKA field."]
    #[inline] pub fn set_pka<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Peripheral RNG interrupt mask to CPU2"]
    #[inline] pub fn rng(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RNG != 0"]
    #[inline] pub fn test_rng(&self) -> bool {
        self.rng() != 0
    }

    #[doc="Sets the RNG field."]
    #[inline] pub fn set_rng<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Peripheral AES1 interrupt mask to CPU2"]
    #[inline] pub fn aes1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if AES1 != 0"]
    #[inline] pub fn test_aes1(&self) -> bool {
        self.aes1() != 0
    }

    #[doc="Sets the AES1 field."]
    #[inline] pub fn set_aes1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Peripheral COMP interrupt mask to CPU2"]
    #[inline] pub fn comp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if COMP != 0"]
    #[inline] pub fn test_comp(&self) -> bool {
        self.comp() != 0
    }

    #[doc="Sets the COMP field."]
    #[inline] pub fn set_comp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Peripheral ADC interrupt mask to CPU2"]
    #[inline] pub fn adc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if ADC != 0"]
    #[inline] pub fn test_adc(&self) -> bool {
        self.adc() != 0
    }

    #[doc="Sets the ADC field."]
    #[inline] pub fn set_adc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for C2imr1 {
    #[inline]
    fn from(other: u32) -> Self {
         C2imr1(other)
    }
}

impl ::core::fmt::Display for C2imr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2imr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rtcstamp() != 0 { try!(write!(f, " rtcstamp"))}
        if self.rtcwkup() != 0 { try!(write!(f, " rtcwkup"))}
        if self.rtcalarm() != 0 { try!(write!(f, " rtcalarm"))}
        if self.rcc() != 0 { try!(write!(f, " rcc"))}
        if self.flash() != 0 { try!(write!(f, " flash"))}
        if self.pka() != 0 { try!(write!(f, " pka"))}
        if self.rng() != 0 { try!(write!(f, " rng"))}
        if self.aes1() != 0 { try!(write!(f, " aes1"))}
        if self.comp() != 0 { try!(write!(f, " comp"))}
        if self.adc() != 0 { try!(write!(f, " adc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CPU2 interrupt mask register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2imr2(pub u32);
impl C2imr2 {
    #[doc="Peripheral DMA1 CH1 interrupt mask to CPU2"]
    #[inline] pub fn dma1_ch1_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA1_CH1_IM != 0"]
    #[inline] pub fn test_dma1_ch1_im(&self) -> bool {
        self.dma1_ch1_im() != 0
    }

    #[doc="Sets the DMA1_CH1_IM field."]
    #[inline] pub fn set_dma1_ch1_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Peripheral DMA1 CH2 interrupt mask to CPU2"]
    #[inline] pub fn dma1_ch2_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA1_CH2_IM != 0"]
    #[inline] pub fn test_dma1_ch2_im(&self) -> bool {
        self.dma1_ch2_im() != 0
    }

    #[doc="Sets the DMA1_CH2_IM field."]
    #[inline] pub fn set_dma1_ch2_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Peripheral DMA1 CH3 interrupt mask to CPU2"]
    #[inline] pub fn dma1_ch3_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMA1_CH3_IM != 0"]
    #[inline] pub fn test_dma1_ch3_im(&self) -> bool {
        self.dma1_ch3_im() != 0
    }

    #[doc="Sets the DMA1_CH3_IM field."]
    #[inline] pub fn set_dma1_ch3_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Peripheral DMA1 CH4 interrupt mask to CPU2"]
    #[inline] pub fn dma1_ch4_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMA1_CH4_IM != 0"]
    #[inline] pub fn test_dma1_ch4_im(&self) -> bool {
        self.dma1_ch4_im() != 0
    }

    #[doc="Sets the DMA1_CH4_IM field."]
    #[inline] pub fn set_dma1_ch4_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Peripheral DMA1 CH5 interrupt mask to CPU2"]
    #[inline] pub fn dma1_ch5_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMA1_CH5_IM != 0"]
    #[inline] pub fn test_dma1_ch5_im(&self) -> bool {
        self.dma1_ch5_im() != 0
    }

    #[doc="Sets the DMA1_CH5_IM field."]
    #[inline] pub fn set_dma1_ch5_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Peripheral DMA1 CH6 interrupt mask to CPU2"]
    #[inline] pub fn dma1_ch6_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMA1_CH6_IM != 0"]
    #[inline] pub fn test_dma1_ch6_im(&self) -> bool {
        self.dma1_ch6_im() != 0
    }

    #[doc="Sets the DMA1_CH6_IM field."]
    #[inline] pub fn set_dma1_ch6_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Peripheral DMA1 CH7 interrupt mask to CPU2"]
    #[inline] pub fn dma1_ch7_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DMA1_CH7_IM != 0"]
    #[inline] pub fn test_dma1_ch7_im(&self) -> bool {
        self.dma1_ch7_im() != 0
    }

    #[doc="Sets the DMA1_CH7_IM field."]
    #[inline] pub fn set_dma1_ch7_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Peripheral DMA2 CH1 interrupt mask to CPU1"]
    #[inline] pub fn dma2_ch1_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DMA2_CH1_IM != 0"]
    #[inline] pub fn test_dma2_ch1_im(&self) -> bool {
        self.dma2_ch1_im() != 0
    }

    #[doc="Sets the DMA2_CH1_IM field."]
    #[inline] pub fn set_dma2_ch1_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Peripheral DMA2 CH2 interrupt mask to CPU1"]
    #[inline] pub fn dma2_ch2_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if DMA2_CH2_IM != 0"]
    #[inline] pub fn test_dma2_ch2_im(&self) -> bool {
        self.dma2_ch2_im() != 0
    }

    #[doc="Sets the DMA2_CH2_IM field."]
    #[inline] pub fn set_dma2_ch2_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Peripheral DMA2 CH3 interrupt mask to CPU1"]
    #[inline] pub fn dma2_ch3_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DMA2_CH3_IM != 0"]
    #[inline] pub fn test_dma2_ch3_im(&self) -> bool {
        self.dma2_ch3_im() != 0
    }

    #[doc="Sets the DMA2_CH3_IM field."]
    #[inline] pub fn set_dma2_ch3_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Peripheral DMA2 CH4 interrupt mask to CPU1"]
    #[inline] pub fn dma2_ch4_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DMA2_CH4_IM != 0"]
    #[inline] pub fn test_dma2_ch4_im(&self) -> bool {
        self.dma2_ch4_im() != 0
    }

    #[doc="Sets the DMA2_CH4_IM field."]
    #[inline] pub fn set_dma2_ch4_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Peripheral DMA2 CH5 interrupt mask to CPU1"]
    #[inline] pub fn dma2_ch5_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DMA2_CH5_IM != 0"]
    #[inline] pub fn test_dma2_ch5_im(&self) -> bool {
        self.dma2_ch5_im() != 0
    }

    #[doc="Sets the DMA2_CH5_IM field."]
    #[inline] pub fn set_dma2_ch5_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Peripheral DMA2 CH6 interrupt mask to CPU1"]
    #[inline] pub fn dma2_ch6_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DMA2_CH6_IM != 0"]
    #[inline] pub fn test_dma2_ch6_im(&self) -> bool {
        self.dma2_ch6_im() != 0
    }

    #[doc="Sets the DMA2_CH6_IM field."]
    #[inline] pub fn set_dma2_ch6_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Peripheral DMA2 CH7 interrupt mask to CPU1"]
    #[inline] pub fn dma2_ch7_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DMA2_CH7_IM != 0"]
    #[inline] pub fn test_dma2_ch7_im(&self) -> bool {
        self.dma2_ch7_im() != 0
    }

    #[doc="Sets the DMA2_CH7_IM field."]
    #[inline] pub fn set_dma2_ch7_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Peripheral DMAM UX1 interrupt mask to CPU1"]
    #[inline] pub fn dmam_ux1_im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DMAM_UX1_IM != 0"]
    #[inline] pub fn test_dmam_ux1_im(&self) -> bool {
        self.dmam_ux1_im() != 0
    }

    #[doc="Sets the DMAM_UX1_IM field."]
    #[inline] pub fn set_dmam_ux1_im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Peripheral PVM1IM interrupt mask to CPU1"]
    #[inline] pub fn pvm1im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PVM1IM != 0"]
    #[inline] pub fn test_pvm1im(&self) -> bool {
        self.pvm1im() != 0
    }

    #[doc="Sets the PVM1IM field."]
    #[inline] pub fn set_pvm1im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Peripheral PVM3IM interrupt mask to CPU1"]
    #[inline] pub fn pvm3im(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PVM3IM != 0"]
    #[inline] pub fn test_pvm3im(&self) -> bool {
        self.pvm3im() != 0
    }

    #[doc="Sets the PVM3IM field."]
    #[inline] pub fn set_pvm3im<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Peripheral PVDIM interrupt mask to CPU1"]
    #[inline] pub fn pvdim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PVDIM != 0"]
    #[inline] pub fn test_pvdim(&self) -> bool {
        self.pvdim() != 0
    }

    #[doc="Sets the PVDIM field."]
    #[inline] pub fn set_pvdim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Peripheral TSCIM interrupt mask to CPU1"]
    #[inline] pub fn tscim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if TSCIM != 0"]
    #[inline] pub fn test_tscim(&self) -> bool {
        self.tscim() != 0
    }

    #[doc="Sets the TSCIM field."]
    #[inline] pub fn set_tscim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Peripheral LCDIM interrupt mask to CPU1"]
    #[inline] pub fn lcdim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if LCDIM != 0"]
    #[inline] pub fn test_lcdim(&self) -> bool {
        self.lcdim() != 0
    }

    #[doc="Sets the LCDIM field."]
    #[inline] pub fn set_lcdim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

}

impl From<u32> for C2imr2 {
    #[inline]
    fn from(other: u32) -> Self {
         C2imr2(other)
    }
}

impl ::core::fmt::Display for C2imr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2imr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dma1_ch1_im() != 0 { try!(write!(f, " dma1_ch1_im"))}
        if self.dma1_ch2_im() != 0 { try!(write!(f, " dma1_ch2_im"))}
        if self.dma1_ch3_im() != 0 { try!(write!(f, " dma1_ch3_im"))}
        if self.dma1_ch4_im() != 0 { try!(write!(f, " dma1_ch4_im"))}
        if self.dma1_ch5_im() != 0 { try!(write!(f, " dma1_ch5_im"))}
        if self.dma1_ch6_im() != 0 { try!(write!(f, " dma1_ch6_im"))}
        if self.dma1_ch7_im() != 0 { try!(write!(f, " dma1_ch7_im"))}
        if self.dma2_ch1_im() != 0 { try!(write!(f, " dma2_ch1_im"))}
        if self.dma2_ch2_im() != 0 { try!(write!(f, " dma2_ch2_im"))}
        if self.dma2_ch3_im() != 0 { try!(write!(f, " dma2_ch3_im"))}
        if self.dma2_ch4_im() != 0 { try!(write!(f, " dma2_ch4_im"))}
        if self.dma2_ch5_im() != 0 { try!(write!(f, " dma2_ch5_im"))}
        if self.dma2_ch6_im() != 0 { try!(write!(f, " dma2_ch6_im"))}
        if self.dma2_ch7_im() != 0 { try!(write!(f, " dma2_ch7_im"))}
        if self.dmam_ux1_im() != 0 { try!(write!(f, " dmam_ux1_im"))}
        if self.pvm1im() != 0 { try!(write!(f, " pvm1im"))}
        if self.pvm3im() != 0 { try!(write!(f, " pvm3im"))}
        if self.pvdim() != 0 { try!(write!(f, " pvdim"))}
        if self.tscim() != 0 { try!(write!(f, " tscim"))}
        if self.lcdim() != 0 { try!(write!(f, " lcdim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="secure IP control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sipcr(pub u32);
impl Sipcr {
    #[doc="Enable AES1 KEY[7:0] security."]
    #[inline] pub fn saes1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SAES1 != 0"]
    #[inline] pub fn test_saes1(&self) -> bool {
        self.saes1() != 0
    }

    #[doc="Sets the SAES1 field."]
    #[inline] pub fn set_saes1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable AES2 security."]
    #[inline] pub fn saes2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SAES2 != 0"]
    #[inline] pub fn test_saes2(&self) -> bool {
        self.saes2() != 0
    }

    #[doc="Sets the SAES2 field."]
    #[inline] pub fn set_saes2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enable PKA security"]
    #[inline] pub fn spka(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SPKA != 0"]
    #[inline] pub fn test_spka(&self) -> bool {
        self.spka() != 0
    }

    #[doc="Sets the SPKA field."]
    #[inline] pub fn set_spka<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enable True RNG security"]
    #[inline] pub fn srng(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SRNG != 0"]
    #[inline] pub fn test_srng(&self) -> bool {
        self.srng() != 0
    }

    #[doc="Sets the SRNG field."]
    #[inline] pub fn set_srng<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Sipcr {
    #[inline]
    fn from(other: u32) -> Self {
         Sipcr(other)
    }
}

impl ::core::fmt::Display for Sipcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sipcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.saes1() != 0 { try!(write!(f, " saes1"))}
        if self.saes2() != 0 { try!(write!(f, " saes2"))}
        if self.spka() != 0 { try!(write!(f, " spka"))}
        if self.srng() != 0 { try!(write!(f, " srng"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

