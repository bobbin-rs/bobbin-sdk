
periph!( FPU, Fpu, FPU_PERIPH, FpuPeriph, FPU_OWNED, FPU_REF_COUNT, 0xe000e000, 0x00, 0x04);


#[doc="FPU"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FpuPeriph(pub usize);
impl FpuPeriph {
    #[doc="Get the CPACR Register."]
    #[inline] pub fn cpacr_reg(&self) -> ::bobbin_mcu::register::Register<Cpacr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cpacr, 0xd88)
    }

    #[doc="Get the *mut pointer for the CPACR register."]
    #[inline] pub fn cpacr_mut(&self) -> *mut Cpacr { 
        self.cpacr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CPACR register."]
    #[inline] pub fn cpacr_ptr(&self) -> *const Cpacr { 
        self.cpacr_reg().ptr()
    }

    #[doc="Read the CPACR register."]
    #[inline] pub fn cpacr(&self) -> Cpacr { 
        self.cpacr_reg().read()
    }

    #[doc="Write the CPACR register."]
    #[inline] pub fn write_cpacr(&self, value: Cpacr) -> &Self { 
        self.cpacr_reg().write(value);
        self
    }

    #[doc="Set the CPACR register."]
    #[inline] pub fn set_cpacr<F: FnOnce(Cpacr) -> Cpacr>(&self, f: F) -> &Self {
        self.cpacr_reg().set(f);
        self
    }

    #[doc="Modify the CPACR register."]
    #[inline] pub fn with_cpacr<F: FnOnce(Cpacr) -> Cpacr>(&self, f: F) -> &Self {
        self.cpacr_reg().with(f);
        self
    }

    #[doc="Get the FPCCR Register."]
    #[inline] pub fn fpccr_reg(&self) -> ::bobbin_mcu::register::Register<Fpccr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fpccr, 0xf34)
    }

    #[doc="Get the *mut pointer for the FPCCR register."]
    #[inline] pub fn fpccr_mut(&self) -> *mut Fpccr { 
        self.fpccr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FPCCR register."]
    #[inline] pub fn fpccr_ptr(&self) -> *const Fpccr { 
        self.fpccr_reg().ptr()
    }

    #[doc="Read the FPCCR register."]
    #[inline] pub fn fpccr(&self) -> Fpccr { 
        self.fpccr_reg().read()
    }

    #[doc="Write the FPCCR register."]
    #[inline] pub fn write_fpccr(&self, value: Fpccr) -> &Self { 
        self.fpccr_reg().write(value);
        self
    }

    #[doc="Set the FPCCR register."]
    #[inline] pub fn set_fpccr<F: FnOnce(Fpccr) -> Fpccr>(&self, f: F) -> &Self {
        self.fpccr_reg().set(f);
        self
    }

    #[doc="Modify the FPCCR register."]
    #[inline] pub fn with_fpccr<F: FnOnce(Fpccr) -> Fpccr>(&self, f: F) -> &Self {
        self.fpccr_reg().with(f);
        self
    }

    #[doc="Get the FPCAR Register."]
    #[inline] pub fn fpcar_reg(&self) -> ::bobbin_mcu::register::Register<Fpcar> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fpcar, 0xf38)
    }

    #[doc="Get the *mut pointer for the FPCAR register."]
    #[inline] pub fn fpcar_mut(&self) -> *mut Fpcar { 
        self.fpcar_reg().ptr()
    }

    #[doc="Get the *const pointer for the FPCAR register."]
    #[inline] pub fn fpcar_ptr(&self) -> *const Fpcar { 
        self.fpcar_reg().ptr()
    }

    #[doc="Read the FPCAR register."]
    #[inline] pub fn fpcar(&self) -> Fpcar { 
        self.fpcar_reg().read()
    }

    #[doc="Write the FPCAR register."]
    #[inline] pub fn write_fpcar(&self, value: Fpcar) -> &Self { 
        self.fpcar_reg().write(value);
        self
    }

    #[doc="Set the FPCAR register."]
    #[inline] pub fn set_fpcar<F: FnOnce(Fpcar) -> Fpcar>(&self, f: F) -> &Self {
        self.fpcar_reg().set(f);
        self
    }

    #[doc="Modify the FPCAR register."]
    #[inline] pub fn with_fpcar<F: FnOnce(Fpcar) -> Fpcar>(&self, f: F) -> &Self {
        self.fpcar_reg().with(f);
        self
    }

    #[doc="Get the FPDSCR Register."]
    #[inline] pub fn fpdscr_reg(&self) -> ::bobbin_mcu::register::Register<Fpdscr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fpdscr, 0xf3c)
    }

    #[doc="Get the *mut pointer for the FPDSCR register."]
    #[inline] pub fn fpdscr_mut(&self) -> *mut Fpdscr { 
        self.fpdscr_reg().ptr()
    }

    #[doc="Get the *const pointer for the FPDSCR register."]
    #[inline] pub fn fpdscr_ptr(&self) -> *const Fpdscr { 
        self.fpdscr_reg().ptr()
    }

    #[doc="Read the FPDSCR register."]
    #[inline] pub fn fpdscr(&self) -> Fpdscr { 
        self.fpdscr_reg().read()
    }

    #[doc="Write the FPDSCR register."]
    #[inline] pub fn write_fpdscr(&self, value: Fpdscr) -> &Self { 
        self.fpdscr_reg().write(value);
        self
    }

    #[doc="Set the FPDSCR register."]
    #[inline] pub fn set_fpdscr<F: FnOnce(Fpdscr) -> Fpdscr>(&self, f: F) -> &Self {
        self.fpdscr_reg().set(f);
        self
    }

    #[doc="Modify the FPDSCR register."]
    #[inline] pub fn with_fpdscr<F: FnOnce(Fpdscr) -> Fpdscr>(&self, f: F) -> &Self {
        self.fpdscr_reg().with(f);
        self
    }

}

#[doc="Coprocessor Access Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpacr(pub u32);
impl Cpacr {
    #[doc="CP10 Access privileges"]
    #[inline] pub fn cp10(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if CP10 != 0"]
    #[inline] pub fn test_cp10(&self) -> bool {
        self.cp10() != 0
    }

    #[doc="Sets the CP10 field."]
    #[inline] pub fn set_cp10<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="CP11 Access privileges"]
    #[inline] pub fn cp11(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if CP11 != 0"]
    #[inline] pub fn test_cp11(&self) -> bool {
        self.cp11() != 0
    }

    #[doc="Sets the CP11 field."]
    #[inline] pub fn set_cp11<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

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
        if self.cp10() != 0 { try!(write!(f, " cp10=0x{:x}", self.cp10()))}
        if self.cp11() != 0 { try!(write!(f, " cp11=0x{:x}", self.cp11()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Floating-point Context Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fpccr(pub u32);
impl Fpccr {
    #[doc="Gets the ASPEN field."]
    #[inline] pub fn aspen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if ASPEN != 0"]
    #[inline] pub fn test_aspen(&self) -> bool {
        self.aspen() != 0
    }

    #[doc="Sets the ASPEN field."]
    #[inline] pub fn set_aspen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Gets the LSPEN field."]
    #[inline] pub fn lspen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if LSPEN != 0"]
    #[inline] pub fn test_lspen(&self) -> bool {
        self.lspen() != 0
    }

    #[doc="Sets the LSPEN field."]
    #[inline] pub fn set_lspen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Gets the MONRDY field."]
    #[inline] pub fn monrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if MONRDY != 0"]
    #[inline] pub fn test_monrdy(&self) -> bool {
        self.monrdy() != 0
    }

    #[doc="Sets the MONRDY field."]
    #[inline] pub fn set_monrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Gets the BFRDY field."]
    #[inline] pub fn bfrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BFRDY != 0"]
    #[inline] pub fn test_bfrdy(&self) -> bool {
        self.bfrdy() != 0
    }

    #[doc="Sets the BFRDY field."]
    #[inline] pub fn set_bfrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Gets the MMRDY field."]
    #[inline] pub fn mmrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MMRDY != 0"]
    #[inline] pub fn test_mmrdy(&self) -> bool {
        self.mmrdy() != 0
    }

    #[doc="Sets the MMRDY field."]
    #[inline] pub fn set_mmrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Gets the HFRDY field."]
    #[inline] pub fn hfrdy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HFRDY != 0"]
    #[inline] pub fn test_hfrdy(&self) -> bool {
        self.hfrdy() != 0
    }

    #[doc="Sets the HFRDY field."]
    #[inline] pub fn set_hfrdy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Gets the THREAD field."]
    #[inline] pub fn thread(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if THREAD != 0"]
    #[inline] pub fn test_thread(&self) -> bool {
        self.thread() != 0
    }

    #[doc="Sets the THREAD field."]
    #[inline] pub fn set_thread<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Gets the USER field."]
    #[inline] pub fn user(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if USER != 0"]
    #[inline] pub fn test_user(&self) -> bool {
        self.user() != 0
    }

    #[doc="Sets the USER field."]
    #[inline] pub fn set_user<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Gets the LSPACT field."]
    #[inline] pub fn lspact(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSPACT != 0"]
    #[inline] pub fn test_lspact(&self) -> bool {
        self.lspact() != 0
    }

    #[doc="Sets the LSPACT field."]
    #[inline] pub fn set_lspact<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fpccr {
    #[inline]
    fn from(other: u32) -> Self {
         Fpccr(other)
    }
}

impl ::core::fmt::Display for Fpccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fpccr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.aspen() != 0 { try!(write!(f, " aspen"))}
        if self.lspen() != 0 { try!(write!(f, " lspen"))}
        if self.monrdy() != 0 { try!(write!(f, " monrdy"))}
        if self.bfrdy() != 0 { try!(write!(f, " bfrdy"))}
        if self.mmrdy() != 0 { try!(write!(f, " mmrdy"))}
        if self.hfrdy() != 0 { try!(write!(f, " hfrdy"))}
        if self.thread() != 0 { try!(write!(f, " thread"))}
        if self.user() != 0 { try!(write!(f, " user"))}
        if self.lspact() != 0 { try!(write!(f, " lspact"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Floating-point Context Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fpcar(pub u32);
impl Fpcar {
    #[doc="Gets the ADDRESS field."]
    #[inline] pub fn address(&self) -> ::bobbin_bits::U29 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1fffffff) as u32) } // [31:3]
    }

    #[doc="Returns true if ADDRESS != 0"]
    #[inline] pub fn test_address(&self) -> bool {
        self.address() != 0
    }

    #[doc="Sets the ADDRESS field."]
    #[inline] pub fn set_address<V: Into<::bobbin_bits::U29>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U29 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1fffffff << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Fpcar {
    #[inline]
    fn from(other: u32) -> Self {
         Fpcar(other)
    }
}

impl ::core::fmt::Display for Fpcar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fpcar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.address() != 0 { try!(write!(f, " address=0x{:x}", self.address()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Floating-point Default Status Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fpdscr(pub u32);
impl Fpdscr {
    #[doc="Gets the AHP field."]
    #[inline] pub fn ahp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if AHP != 0"]
    #[inline] pub fn test_ahp(&self) -> bool {
        self.ahp() != 0
    }

    #[doc="Sets the AHP field."]
    #[inline] pub fn set_ahp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Gets the DN field."]
    #[inline] pub fn dn(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if DN != 0"]
    #[inline] pub fn test_dn(&self) -> bool {
        self.dn() != 0
    }

    #[doc="Sets the DN field."]
    #[inline] pub fn set_dn<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Gets the FZ field."]
    #[inline] pub fn fz(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FZ != 0"]
    #[inline] pub fn test_fz(&self) -> bool {
        self.fz() != 0
    }

    #[doc="Sets the FZ field."]
    #[inline] pub fn set_fz<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Gets the RMODE field."]
    #[inline] pub fn rmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if RMODE != 0"]
    #[inline] pub fn test_rmode(&self) -> bool {
        self.rmode() != 0
    }

    #[doc="Sets the RMODE field."]
    #[inline] pub fn set_rmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

}

impl From<u32> for Fpdscr {
    #[inline]
    fn from(other: u32) -> Self {
         Fpdscr(other)
    }
}

impl ::core::fmt::Display for Fpdscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fpdscr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ahp() != 0 { try!(write!(f, " ahp"))}
        if self.dn() != 0 { try!(write!(f, " dn"))}
        if self.fz() != 0 { try!(write!(f, " fz"))}
        if self.rmode() != 0 { try!(write!(f, " rmode=0x{:x}", self.rmode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

