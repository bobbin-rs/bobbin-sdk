pub const SCB: Scb = Scb(0xe000e000);

#[doc="Nested Vectored Interrupt Controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scb(pub u32);
impl Scb {
#[doc="Get the *const pointer for the ACTLR register."]
  #[inline] pub fn actlr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the ACTLR register."]
  #[inline] pub fn actlr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the ACTLR register."]
  #[inline] pub fn actlr(&self) -> Actlr { 
     unsafe {
        Actlr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the ACTLR register."]
  #[inline] pub fn set_actlr(&self, value: Actlr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ACTLR register."]
  #[inline] pub fn with_actlr<F: FnOnce(Actlr) -> Actlr>(&self, f: F) -> &Self {
     let tmp = self.actlr();
     self.set_actlr(f(tmp))
  }

#[doc="Get the *const pointer for the CPUID register."]
  #[inline] pub fn cpuid_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd00) as *const u32
  }
#[doc="Get the *mut pointer for the CPUID register."]
  #[inline] pub fn cpuid_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd00) as *mut u32
  }
#[doc="Read the CPUID register."]
  #[inline] pub fn cpuid(&self) -> Cpuid { 
     unsafe {
        Cpuid(::core::ptr::read_volatile(((self.0 as usize) + 0xd00) as *const u32))
     }
  }
#[doc="Write the CPUID register."]
  #[inline] pub fn set_cpuid(&self, value: Cpuid) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd00) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CPUID register."]
  #[inline] pub fn with_cpuid<F: FnOnce(Cpuid) -> Cpuid>(&self, f: F) -> &Self {
     let tmp = self.cpuid();
     self.set_cpuid(f(tmp))
  }

#[doc="Get the *const pointer for the ICSR register."]
  #[inline] pub fn icsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd04) as *const u32
  }
#[doc="Get the *mut pointer for the ICSR register."]
  #[inline] pub fn icsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd04) as *mut u32
  }
#[doc="Read the ICSR register."]
  #[inline] pub fn icsr(&self) -> Icsr { 
     unsafe {
        Icsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd04) as *const u32))
     }
  }
#[doc="Write the ICSR register."]
  #[inline] pub fn set_icsr(&self, value: Icsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd04) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ICSR register."]
  #[inline] pub fn with_icsr<F: FnOnce(Icsr) -> Icsr>(&self, f: F) -> &Self {
     let tmp = self.icsr();
     self.set_icsr(f(tmp))
  }

#[doc="Get the *const pointer for the VTOR register."]
  #[inline] pub fn vtor_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd08) as *const u32
  }
#[doc="Get the *mut pointer for the VTOR register."]
  #[inline] pub fn vtor_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd08) as *mut u32
  }
#[doc="Read the VTOR register."]
  #[inline] pub fn vtor(&self) -> Vtor { 
     unsafe {
        Vtor(::core::ptr::read_volatile(((self.0 as usize) + 0xd08) as *const u32))
     }
  }
#[doc="Write the VTOR register."]
  #[inline] pub fn set_vtor(&self, value: Vtor) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd08) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the VTOR register."]
  #[inline] pub fn with_vtor<F: FnOnce(Vtor) -> Vtor>(&self, f: F) -> &Self {
     let tmp = self.vtor();
     self.set_vtor(f(tmp))
  }

#[doc="Get the *const pointer for the AIRCR register."]
  #[inline] pub fn aircr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd0c) as *const u32
  }
#[doc="Get the *mut pointer for the AIRCR register."]
  #[inline] pub fn aircr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd0c) as *mut u32
  }
#[doc="Read the AIRCR register."]
  #[inline] pub fn aircr(&self) -> Aircr { 
     unsafe {
        Aircr(::core::ptr::read_volatile(((self.0 as usize) + 0xd0c) as *const u32))
     }
  }
#[doc="Write the AIRCR register."]
  #[inline] pub fn set_aircr(&self, value: Aircr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd0c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AIRCR register."]
  #[inline] pub fn with_aircr<F: FnOnce(Aircr) -> Aircr>(&self, f: F) -> &Self {
     let tmp = self.aircr();
     self.set_aircr(f(tmp))
  }

#[doc="Get the *const pointer for the SCR register."]
  #[inline] pub fn scr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd0c) as *const u32
  }
#[doc="Get the *mut pointer for the SCR register."]
  #[inline] pub fn scr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd0c) as *mut u32
  }
#[doc="Read the SCR register."]
  #[inline] pub fn scr(&self) -> Scr { 
     unsafe {
        Scr(::core::ptr::read_volatile(((self.0 as usize) + 0xd0c) as *const u32))
     }
  }
#[doc="Write the SCR register."]
  #[inline] pub fn set_scr(&self, value: Scr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd0c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SCR register."]
  #[inline] pub fn with_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) -> &Self {
     let tmp = self.scr();
     self.set_scr(f(tmp))
  }

#[doc="Get the *const pointer for the CCR register."]
  #[inline] pub fn ccr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd14) as *const u32
  }
#[doc="Get the *mut pointer for the CCR register."]
  #[inline] pub fn ccr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd14) as *mut u32
  }
#[doc="Read the CCR register."]
  #[inline] pub fn ccr(&self) -> Ccr { 
     unsafe {
        Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0xd14) as *const u32))
     }
  }
#[doc="Write the CCR register."]
  #[inline] pub fn set_ccr(&self, value: Ccr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CCR register."]
  #[inline] pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
     let tmp = self.ccr();
     self.set_ccr(f(tmp))
  }

#[doc="Get the *const pointer for the SHPR1 register."]
  #[inline] pub fn shpr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd18) as *const u32
  }
#[doc="Get the *mut pointer for the SHPR1 register."]
  #[inline] pub fn shpr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd18) as *mut u32
  }
#[doc="Read the SHPR1 register."]
  #[inline] pub fn shpr1(&self) -> Shpr1 { 
     unsafe {
        Shpr1(::core::ptr::read_volatile(((self.0 as usize) + 0xd18) as *const u32))
     }
  }
#[doc="Write the SHPR1 register."]
  #[inline] pub fn set_shpr1(&self, value: Shpr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SHPR1 register."]
  #[inline] pub fn with_shpr1<F: FnOnce(Shpr1) -> Shpr1>(&self, f: F) -> &Self {
     let tmp = self.shpr1();
     self.set_shpr1(f(tmp))
  }

#[doc="Get the *const pointer for the SHPR2 register."]
  #[inline] pub fn shpr2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd1c) as *const u32
  }
#[doc="Get the *mut pointer for the SHPR2 register."]
  #[inline] pub fn shpr2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd1c) as *mut u32
  }
#[doc="Read the SHPR2 register."]
  #[inline] pub fn shpr2(&self) -> Shpr2 { 
     unsafe {
        Shpr2(::core::ptr::read_volatile(((self.0 as usize) + 0xd1c) as *const u32))
     }
  }
#[doc="Write the SHPR2 register."]
  #[inline] pub fn set_shpr2(&self, value: Shpr2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SHPR2 register."]
  #[inline] pub fn with_shpr2<F: FnOnce(Shpr2) -> Shpr2>(&self, f: F) -> &Self {
     let tmp = self.shpr2();
     self.set_shpr2(f(tmp))
  }

#[doc="Get the *const pointer for the SHPR3 register."]
  #[inline] pub fn shpr3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd20) as *const u32
  }
#[doc="Get the *mut pointer for the SHPR3 register."]
  #[inline] pub fn shpr3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd20) as *mut u32
  }
#[doc="Read the SHPR3 register."]
  #[inline] pub fn shpr3(&self) -> Shpr3 { 
     unsafe {
        Shpr3(::core::ptr::read_volatile(((self.0 as usize) + 0xd20) as *const u32))
     }
  }
#[doc="Write the SHPR3 register."]
  #[inline] pub fn set_shpr3(&self, value: Shpr3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SHPR3 register."]
  #[inline] pub fn with_shpr3<F: FnOnce(Shpr3) -> Shpr3>(&self, f: F) -> &Self {
     let tmp = self.shpr3();
     self.set_shpr3(f(tmp))
  }

#[doc="Get the *const pointer for the SHCSR register."]
  #[inline] pub fn shcsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd24) as *const u32
  }
#[doc="Get the *mut pointer for the SHCSR register."]
  #[inline] pub fn shcsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd24) as *mut u32
  }
#[doc="Read the SHCSR register."]
  #[inline] pub fn shcsr(&self) -> Shcsr { 
     unsafe {
        Shcsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd24) as *const u32))
     }
  }
#[doc="Write the SHCSR register."]
  #[inline] pub fn set_shcsr(&self, value: Shcsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SHCSR register."]
  #[inline] pub fn with_shcsr<F: FnOnce(Shcsr) -> Shcsr>(&self, f: F) -> &Self {
     let tmp = self.shcsr();
     self.set_shcsr(f(tmp))
  }

#[doc="Get the *const pointer for the CFSR register."]
  #[inline] pub fn cfsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd28) as *const u32
  }
#[doc="Get the *mut pointer for the CFSR register."]
  #[inline] pub fn cfsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd28) as *mut u32
  }
#[doc="Read the CFSR register."]
  #[inline] pub fn cfsr(&self) -> Cfsr { 
     unsafe {
        Cfsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd28) as *const u32))
     }
  }
#[doc="Write the CFSR register."]
  #[inline] pub fn set_cfsr(&self, value: Cfsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd28) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CFSR register."]
  #[inline] pub fn with_cfsr<F: FnOnce(Cfsr) -> Cfsr>(&self, f: F) -> &Self {
     let tmp = self.cfsr();
     self.set_cfsr(f(tmp))
  }

#[doc="Get the *const pointer for the MMFSR register."]
  #[inline] pub fn mmfsr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd28) as *const u8
  }
#[doc="Get the *mut pointer for the MMFSR register."]
  #[inline] pub fn mmfsr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd28) as *mut u8
  }
#[doc="Read the MMFSR register."]
  #[inline] pub fn mmfsr(&self) -> Mmfsr { 
     unsafe {
        Mmfsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd28) as *const u8))
     }
  }
#[doc="Write the MMFSR register."]
  #[inline] pub fn set_mmfsr(&self, value: Mmfsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd28) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the MMFSR register."]
  #[inline] pub fn with_mmfsr<F: FnOnce(Mmfsr) -> Mmfsr>(&self, f: F) -> &Self {
     let tmp = self.mmfsr();
     self.set_mmfsr(f(tmp))
  }

#[doc="Get the *const pointer for the BFSR register."]
  #[inline] pub fn bfsr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0xd29) as *const u8
  }
#[doc="Get the *mut pointer for the BFSR register."]
  #[inline] pub fn bfsr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0xd29) as *mut u8
  }
#[doc="Read the BFSR register."]
  #[inline] pub fn bfsr(&self) -> Bfsr { 
     unsafe {
        Bfsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd29) as *const u8))
     }
  }
#[doc="Write the BFSR register."]
  #[inline] pub fn set_bfsr(&self, value: Bfsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd29) as *mut u8, value.0);
     }
     self
  }
#[doc="Modify the BFSR register."]
  #[inline] pub fn with_bfsr<F: FnOnce(Bfsr) -> Bfsr>(&self, f: F) -> &Self {
     let tmp = self.bfsr();
     self.set_bfsr(f(tmp))
  }

#[doc="Get the *const pointer for the UFSR register."]
  #[inline] pub fn ufsr_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xd2a) as *const u16
  }
#[doc="Get the *mut pointer for the UFSR register."]
  #[inline] pub fn ufsr_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xd2a) as *mut u16
  }
#[doc="Read the UFSR register."]
  #[inline] pub fn ufsr(&self) -> Ufsr { 
     unsafe {
        Ufsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd2a) as *const u16))
     }
  }
#[doc="Write the UFSR register."]
  #[inline] pub fn set_ufsr(&self, value: Ufsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd2a) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the UFSR register."]
  #[inline] pub fn with_ufsr<F: FnOnce(Ufsr) -> Ufsr>(&self, f: F) -> &Self {
     let tmp = self.ufsr();
     self.set_ufsr(f(tmp))
  }

#[doc="Get the *const pointer for the HFSR register."]
  #[inline] pub fn hfsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd2c) as *const u32
  }
#[doc="Get the *mut pointer for the HFSR register."]
  #[inline] pub fn hfsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd2c) as *mut u32
  }
#[doc="Read the HFSR register."]
  #[inline] pub fn hfsr(&self) -> Hfsr { 
     unsafe {
        Hfsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd2c) as *const u32))
     }
  }
#[doc="Write the HFSR register."]
  #[inline] pub fn set_hfsr(&self, value: Hfsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd2c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the HFSR register."]
  #[inline] pub fn with_hfsr<F: FnOnce(Hfsr) -> Hfsr>(&self, f: F) -> &Self {
     let tmp = self.hfsr();
     self.set_hfsr(f(tmp))
  }

#[doc="Get the *const pointer for the MMFAR register."]
  #[inline] pub fn mmfar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd34) as *const u32
  }
#[doc="Get the *mut pointer for the MMFAR register."]
  #[inline] pub fn mmfar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd34) as *mut u32
  }
#[doc="Read the MMFAR register."]
  #[inline] pub fn mmfar(&self) -> Mmfar { 
     unsafe {
        Mmfar(::core::ptr::read_volatile(((self.0 as usize) + 0xd34) as *const u32))
     }
  }
#[doc="Write the MMFAR register."]
  #[inline] pub fn set_mmfar(&self, value: Mmfar) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd34) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MMFAR register."]
  #[inline] pub fn with_mmfar<F: FnOnce(Mmfar) -> Mmfar>(&self, f: F) -> &Self {
     let tmp = self.mmfar();
     self.set_mmfar(f(tmp))
  }

#[doc="Get the *const pointer for the BFAR register."]
  #[inline] pub fn bfar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd38) as *const u32
  }
#[doc="Get the *mut pointer for the BFAR register."]
  #[inline] pub fn bfar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd38) as *mut u32
  }
#[doc="Read the BFAR register."]
  #[inline] pub fn bfar(&self) -> Bfar { 
     unsafe {
        Bfar(::core::ptr::read_volatile(((self.0 as usize) + 0xd38) as *const u32))
     }
  }
#[doc="Write the BFAR register."]
  #[inline] pub fn set_bfar(&self, value: Bfar) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd38) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BFAR register."]
  #[inline] pub fn with_bfar<F: FnOnce(Bfar) -> Bfar>(&self, f: F) -> &Self {
     let tmp = self.bfar();
     self.set_bfar(f(tmp))
  }

#[doc="Get the *const pointer for the AFSR register."]
  #[inline] pub fn afsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd3c) as *const u32
  }
#[doc="Get the *mut pointer for the AFSR register."]
  #[inline] pub fn afsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd3c) as *mut u32
  }
#[doc="Read the AFSR register."]
  #[inline] pub fn afsr(&self) -> Afsr { 
     unsafe {
        Afsr(::core::ptr::read_volatile(((self.0 as usize) + 0xd3c) as *const u32))
     }
  }
#[doc="Write the AFSR register."]
  #[inline] pub fn set_afsr(&self, value: Afsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd3c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the AFSR register."]
  #[inline] pub fn with_afsr<F: FnOnce(Afsr) -> Afsr>(&self, f: F) -> &Self {
     let tmp = self.afsr();
     self.set_afsr(f(tmp))
  }

}

#[doc="Auxiliary Control Register"]
#[derive(PartialEq, Eq)]
pub struct Actlr(pub u32);
impl Actlr {
#[doc="When set to 1, disables IT folding."]
  #[inline] pub fn disfold(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="When set to 1, disables IT folding."]
  #[inline] pub fn set_disfold(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="When set to 1, disables write buffer use during default memory map accesses."]
  #[inline] pub fn disdefwbuf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="When set to 1, disables write buffer use during default memory map accesses."]
  #[inline] pub fn set_disdefwbuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="When set to 1, disables interruption of load multiple and store multiple instructions."]
  #[inline] pub fn dismcycint(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="When set to 1, disables interruption of load multiple and store multiple instructions."]
  #[inline] pub fn set_dismcycint(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Cpuid(pub u32);
impl Cpuid {
#[doc="Implementer Code"]
  #[inline] pub fn implementer(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
#[doc="Implementer Code"]
  #[inline] pub fn set_implementer(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Variant number, the r value in the rnpn product revision identifier"]
  #[inline] pub fn variant(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
#[doc="Variant number, the r value in the rnpn product revision identifier"]
  #[inline] pub fn set_variant(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Reads as 0xF"]
  #[inline] pub fn constant(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="Reads as 0xF"]
  #[inline] pub fn set_constant(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Part number of the processor"]
  #[inline] pub fn partno(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xfff // [15:4]
  }
#[doc="Part number of the processor"]
  #[inline] pub fn set_partno(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Revision number, the p value in the rnpn product revision identifier"]
  #[inline] pub fn revision(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Revision number, the p value in the rnpn product revision identifier"]
  #[inline] pub fn set_revision(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Icsr(pub u32);
impl Icsr {
#[doc="NMI set-pending bit"]
  #[inline] pub fn nmipendset(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="NMI set-pending bit"]
  #[inline] pub fn set_nmipendset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="PendSV set-pending bit"]
  #[inline] pub fn pendsvset(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
#[doc="PendSV set-pending bit"]
  #[inline] pub fn set_pendsvset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="PendSV clear-pending bit"]
  #[inline] pub fn pendsvclr(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
#[doc="PendSV clear-pending bit"]
  #[inline] pub fn set_pendsvclr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

#[doc="Systick exception set-pending bit"]
  #[inline] pub fn pendstset(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
#[doc="Systick exception set-pending bit"]
  #[inline] pub fn set_pendstset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Systick clear-pending bit"]
  #[inline] pub fn pendstclr(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
#[doc="Systick clear-pending bit"]
  #[inline] pub fn set_pendstclr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

#[doc="Interrupt pending flag, excluding NMI and Faults"]
  #[inline] pub fn isrpending(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="Interrupt pending flag, excluding NMI and Faults"]
  #[inline] pub fn set_isrpending(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Indicates the exception number of the highest priority pending enabled exception"]
  #[inline] pub fn vectpending(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3f // [17:12]
  }
#[doc="Indicates the exception number of the highest priority pending enabled exception"]
  #[inline] pub fn set_vectpending(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Indicates the exception number of the highest priority pending enabled exception"]
  #[inline] pub fn rettobase(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Indicates the exception number of the highest priority pending enabled exception"]
  #[inline] pub fn set_rettobase(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Contains the active exception number. Subtract 16 from this value to obtain the CMSIS IRQ number required to index into the Interrupt Clear-Enable, Set-Enable, Clear-Pending, Set-Pending, or Priority Registers"]
  #[inline] pub fn vectactive(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Contains the active exception number. Subtract 16 from this value to obtain the CMSIS IRQ number required to index into the Interrupt Clear-Enable, Set-Enable, Clear-Pending, Set-Pending, or Priority Registers"]
  #[inline] pub fn set_vectactive(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Vtor(pub u32);
impl Vtor {
#[doc="Vector table base offset field. It contains bits[29:7] of the offset of the table base from the bottom of the memory map."]
  #[inline] pub fn tbloff(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1ffffff // [31:7]
  }
#[doc="Vector table base offset field. It contains bits[29:7] of the offset of the table base from the bottom of the memory map."]
  #[inline] pub fn set_tbloff(mut self, value: u32) -> Self {
     assert!((value & !0x1ffffff) == 0);
     self.0 &= !(0x1ffffff << 7);
     self.0 |= value << 7;
     self
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
#[derive(PartialEq, Eq)]
pub struct Aircr(pub u32);
impl Aircr {
#[doc="Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored."]
  #[inline] pub fn vectkey(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
#[doc="Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored."]
  #[inline] pub fn set_vectkey(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Data endianness bit is implementation defined: 0 = Little-endian, 1 = Big-endian."]
  #[inline] pub fn endianness(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Data endianness bit is implementation defined: 0 = Little-endian, 1 = Big-endian."]
  #[inline] pub fn set_endianness(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Interrupt priority grouping field is implementation defined. This field determines the split of group priority from subpriority, see Binary point."]
  #[inline] pub fn prigroup(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
#[doc="Interrupt priority grouping field is implementation defined. This field determines the split of group priority from subpriority, see Binary point."]
  #[inline] pub fn set_prigroup(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="System reset request bit is implementation defined: 0 = no system reset request, 1 = asserts a signal to the outer system that requests a reset. This is intended to force a large system reset of all major components except for debug."]
  #[inline] pub fn sysresetreq(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="System reset request bit is implementation defined: 0 = no system reset request, 1 = asserts a signal to the outer system that requests a reset. This is intended to force a large system reset of all major components except for debug."]
  #[inline] pub fn set_sysresetreq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
  #[inline] pub fn vectclractive(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
  #[inline] pub fn set_vectclractive(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
  #[inline] pub fn vectreset(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
  #[inline] pub fn set_vectreset(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Scr(pub u32);
impl Scr {
#[doc="Send Event on Pending bit"]
  #[inline] pub fn sevonpend(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Send Event on Pending bit"]
  #[inline] pub fn set_sevonpend(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Controls whether the processor uses sleep or deep sleep as its low power mode"]
  #[inline] pub fn sleepdeep(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Controls whether the processor uses sleep or deep sleep as its low power mode"]
  #[inline] pub fn set_sleepdeep(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Indicates sleep-on-exit when returning from Handler mode to Thread mode:"]
  #[inline] pub fn sleeponexit(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Indicates sleep-on-exit when returning from Handler mode to Thread mode:"]
  #[inline] pub fn set_sleeponexit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="Indicates stack alignment on exception entry: 0 = 4-byte aligned1 = 8-byte aligned. On exception entry, the processor uses bit[9] of the stacked PSR to indicate the stack alignment. On return from the exception it uses this stacked bit to restore the correct stack alignment."]
  #[inline] pub fn stkalign(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Indicates stack alignment on exception entry: 0 = 4-byte aligned1 = 8-byte aligned. On exception entry, the processor uses bit[9] of the stacked PSR to indicate the stack alignment. On return from the exception it uses this stacked bit to restore the correct stack alignment."]
  #[inline] pub fn set_stkalign(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the hard fault, NMI, and FAULTMASK escalated handlers: 0 = data bus faults caused by load and store instructions cause a lock-up, 1 = handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect control path problems and fix them."]
  #[inline] pub fn bfhfnmign(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the hard fault, NMI, and FAULTMASK escalated handlers: 0 = data bus faults caused by load and store instructions cause a lock-up, 1 = handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect control path problems and fix them."]
  #[inline] pub fn set_bfhfnmign(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0 = do not trap divide by 0, 1 = trap divide by 0. When this bit is set to 0, a divide by zero returns a quotient of 0."]
  #[inline] pub fn div_0_trp(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0 = do not trap divide by 0, 1 = trap divide by 0. When this bit is set to 0, a divide by zero returns a quotient of 0."]
  #[inline] pub fn set_div_0_trp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Enables unaligned access traps: 0 = do not trap unaligned halfword and word accesses1 = trap unaligned halfword and word accesses. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault irrespective of whether UNALIGN_TRP is set to 1."]
  #[inline] pub fn unalign_trp(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Enables unaligned access traps: 0 = do not trap unaligned halfword and word accesses1 = trap unaligned halfword and word accesses. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault irrespective of whether UNALIGN_TRP is set to 1."]
  #[inline] pub fn set_unalign_trp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Enables unprivileged software access to the STIR, see Software Trigger Interrupt Register: 0 = disable, 1 = enable."]
  #[inline] pub fn usersetmpend(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Enables unprivileged software access to the STIR, see Software Trigger Interrupt Register: 0 = disable, 1 = enable."]
  #[inline] pub fn set_usersetmpend(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Indicates how the processor enters Thread mode: 0 = processor can enter Thread mode only when no exception is active, 1 = processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception return."]
  #[inline] pub fn nonbasethrdena(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Indicates how the processor enters Thread mode: 0 = processor can enter Thread mode only when no exception is active, 1 = processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception return."]
  #[inline] pub fn set_nonbasethrdena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Shpr1(pub u32);
impl Shpr1 {
#[doc="Priority of system handler 6, UsageFault"]
  #[inline] pub fn pri_6(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
#[doc="Priority of system handler 6, UsageFault"]
  #[inline] pub fn set_pri_6(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Priority of system handler 5, BusFault"]
  #[inline] pub fn pri_5(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xff // [15:8]
  }
#[doc="Priority of system handler 5, BusFault"]
  #[inline] pub fn set_pri_5(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Priority of system handler 4, MemManage"]
  #[inline] pub fn pri_4(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Priority of system handler 4, MemManage"]
  #[inline] pub fn set_pri_4(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Shpr2(pub u32);
impl Shpr2 {
#[doc="Priority of system handler 11, SVCall"]
  #[inline] pub fn pri_11(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
#[doc="Priority of system handler 11, SVCall"]
  #[inline] pub fn set_pri_11(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
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
#[derive(PartialEq, Eq)]
pub struct Shpr3(pub u32);
impl Shpr3 {
#[doc="Priority of system handler 15, SysTick exception"]
  #[inline] pub fn pri_15(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xff // [31:24]
  }
#[doc="Priority of system handler 15, SysTick exception"]
  #[inline] pub fn set_pri_15(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Priority of system handler 14, PendSV"]
  #[inline] pub fn pri_14(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xff // [23:16]
  }
#[doc="Priority of system handler 14, PendSV"]
  #[inline] pub fn set_pri_14(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 16);
     self.0 |= value << 16;
     self
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
#[derive(PartialEq, Eq)]
pub struct Shcsr(pub u32);
impl Shcsr {
#[doc="UsageFault enable bit, set to 1 to enable"]
  #[inline] pub fn usgfaultena(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="UsageFault enable bit, set to 1 to enable"]
  #[inline] pub fn set_usgfaultena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="BusFault enable bit, set to 1 to enable"]
  #[inline] pub fn busfaultena(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="BusFault enable bit, set to 1 to enable"]
  #[inline] pub fn set_busfaultena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="MemManage enable bit, set to 1 to enable"]
  #[inline] pub fn memfaultena(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="MemManage enable bit, set to 1 to enable"]
  #[inline] pub fn set_memfaultena(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="SVCall pending bit, reads as 1 if exception is pending"]
  #[inline] pub fn svcallpended(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="SVCall pending bit, reads as 1 if exception is pending"]
  #[inline] pub fn set_svcallpended(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="BusFault exception pending bit, reads as 1 if exception is pending"]
  #[inline] pub fn busfaultpended(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="BusFault exception pending bit, reads as 1 if exception is pending"]
  #[inline] pub fn set_busfaultpended(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="MemManage exception pending bit, reads as 1 if exception is pending"]
  #[inline] pub fn memfaultpended(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="MemManage exception pending bit, reads as 1 if exception is pending"]
  #[inline] pub fn set_memfaultpended(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="UsageFault exception pending bit, reads as 1 if exception is pending"]
  #[inline] pub fn usgfaultpended(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="UsageFault exception pending bit, reads as 1 if exception is pending"]
  #[inline] pub fn set_usgfaultpended(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="SysTick exception active bit, reads as 1 if exception is active"]
  #[inline] pub fn systickact(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="SysTick exception active bit, reads as 1 if exception is active"]
  #[inline] pub fn set_systickact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="PendSV exception active bit, reads as 1 if exception is active"]
  #[inline] pub fn pendsvact(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="PendSV exception active bit, reads as 1 if exception is active"]
  #[inline] pub fn set_pendsvact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Debug monitor active bit, reads as 1 if Debug monitor is active"]
  #[inline] pub fn monitoract(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Debug monitor active bit, reads as 1 if Debug monitor is active"]
  #[inline] pub fn set_monitoract(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="SVCall active bit, reads as 1 if SVC call is active"]
  #[inline] pub fn svcallact(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="SVCall active bit, reads as 1 if SVC call is active"]
  #[inline] pub fn set_svcallact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="UsageFault exception active bit, reads as 1 if exception is active"]
  #[inline] pub fn usgfaultact(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="UsageFault exception active bit, reads as 1 if exception is active"]
  #[inline] pub fn set_usgfaultact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="BusFault exception active bit, reads as 1 if exception is active"]
  #[inline] pub fn busfaultact(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="BusFault exception active bit, reads as 1 if exception is active"]
  #[inline] pub fn set_busfaultact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="MemManage exception active bit, reads as 1 if exception is active"]
  #[inline] pub fn memfaultact(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="MemManage exception active bit, reads as 1 if exception is active"]
  #[inline] pub fn set_memfaultact(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Cfsr(pub u32);
impl Cfsr {
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
#[derive(PartialEq, Eq)]
pub struct Mmfsr(pub u8);
impl Mmfsr {
#[doc="MemManage Fault Address Register (MMFAR) valid flag: 0 = value in MMAR is not a valid fault address, 1 = MMAR holds a valid fault address. If a MemManage fault occurs and is escalated to a HardFault because of priority, the HardFault handler must set this bit to 0. This prevents problems on return to a stacked active MemManage fault handler whose MMAR value has been overwritten."]
  #[inline] pub fn mmarvalid(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="MemManage Fault Address Register (MMFAR) valid flag: 0 = value in MMAR is not a valid fault address, 1 = MMAR holds a valid fault address. If a MemManage fault occurs and is escalated to a HardFault because of priority, the HardFault handler must set this bit to 0. This prevents problems on return to a stacked active MemManage fault handler whose MMAR value has been overwritten."]
  #[inline] pub fn set_mmarvalid(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="MemManage fault on stacking for exception entry: 0 = no stacking fault, 1 = stacking for an exception entry has caused one or more access violations. When this bit is 1, the SP is still adjusted but the values in the context area on the stack might be incorrect. The processor has not written a fault address to the MMAR."]
  #[inline] pub fn mstkerr(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="MemManage fault on stacking for exception entry: 0 = no stacking fault, 1 = stacking for an exception entry has caused one or more access violations. When this bit is 1, the SP is still adjusted but the values in the context area on the stack might be incorrect. The processor has not written a fault address to the MMAR."]
  #[inline] pub fn set_mstkerr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="MemManage fault on unstacking for a return from exception: 0 = no unstacking fault, 1 = unstack for an exception return has caused one or more access violations. This fault is chained to the handler. This means that when this bit is 1, the original return stack is still present. The processor has not adjusted the SP from the failing return, and has not performed a new save. The processor has not written a fault address to the MMAR."]
  #[inline] pub fn munstkerr(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="MemManage fault on unstacking for a return from exception: 0 = no unstacking fault, 1 = unstack for an exception return has caused one or more access violations. This fault is chained to the handler. This means that when this bit is 1, the original return stack is still present. The processor has not adjusted the SP from the failing return, and has not performed a new save. The processor has not written a fault address to the MMAR."]
  #[inline] pub fn set_munstkerr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Data access violation flag: 0 = no data access violation fault, 1 = the processor attempted a load or store at a location that does not permit the operation. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has loaded the MMAR with the address of the attempted access."]
  #[inline] pub fn daccviol(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Data access violation flag: 0 = no data access violation fault, 1 = the processor attempted a load or store at a location that does not permit the operation. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has loaded the MMAR with the address of the attempted access."]
  #[inline] pub fn set_daccviol(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Instruction access violation flag: 0 = no instruction access violation fault, 1 = the processor attempted an instruction fetch from a location that does not permit execution. This fault occurs on any access to an XN region, even when the MPU is disabled or not present. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has not written a fault address to the MMAR."]
  #[inline] pub fn iaccviol(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Instruction access violation flag: 0 = no instruction access violation fault, 1 = the processor attempted an instruction fetch from a location that does not permit execution. This fault occurs on any access to an XN region, even when the MPU is disabled or not present. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has not written a fault address to the MMAR."]
  #[inline] pub fn set_iaccviol(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Bfsr(pub u8);
impl Bfsr {
#[doc="BusFault Address Register (BFAR) valid flag: 0 = value in BFAR is not a valid fault address, 1 = BFAR holds a valid fault address. The processor sets this bit to 1 after a BusFault where the address is known. Other faults can set this bit to 0, such as a MemManage fault occurring later. If a BusFault occurs and is escalated to a hard fault because of priority, the hard fault handler must set this bit to 0. This prevents problems if returning to a stacked active BusFault handler whose BFAR value has been overwritten."]
  #[inline] pub fn bfarvalid(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
#[doc="BusFault Address Register (BFAR) valid flag: 0 = value in BFAR is not a valid fault address, 1 = BFAR holds a valid fault address. The processor sets this bit to 1 after a BusFault where the address is known. Other faults can set this bit to 0, such as a MemManage fault occurring later. If a BusFault occurs and is escalated to a hard fault because of priority, the hard fault handler must set this bit to 0. This prevents problems if returning to a stacked active BusFault handler whose BFAR value has been overwritten."]
  #[inline] pub fn set_bfarvalid(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="BusFault on stacking for exception entry: 0 = no stacking fault, 1 = stacking for an exception entry has caused one or more BusFaults. When the processor sets this bit to 1, the SP is still adjusted but the values in the context area on the stack might be incorrect. The processor does not write a fault address to the BFAR."]
  #[inline] pub fn stkerr(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0x1 // [4]
  }
#[doc="BusFault on stacking for exception entry: 0 = no stacking fault, 1 = stacking for an exception entry has caused one or more BusFaults. When the processor sets this bit to 1, the SP is still adjusted but the values in the context area on the stack might be incorrect. The processor does not write a fault address to the BFAR."]
  #[inline] pub fn set_stkerr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="BusFault on unstacking for a return from exception: 0 = no unstacking fault, 1 = unstack for an exception return has caused one or more BusFaults. This fault is chained to the handler. This means that when the processor sets this bit to 1, the original return stack is still present. The processor does not adjust the SP from the failing return, does not performed a new save, and does not write a fault address to the BFAR."]
  #[inline] pub fn unstkerr(&self) -> u8 {
     ((self.0 as u8) >> 3) & 0x1 // [3]
  }
#[doc="BusFault on unstacking for a return from exception: 0 = no unstacking fault, 1 = unstack for an exception return has caused one or more BusFaults. This fault is chained to the handler. This means that when the processor sets this bit to 1, the original return stack is still present. The processor does not adjust the SP from the failing return, does not performed a new save, and does not write a fault address to the BFAR."]
  #[inline] pub fn set_unstkerr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Imprecise data bus error: 0 = no imprecise data bus error, 1 = a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error. When the processor sets this bit to 1, it does not write a fault address to the BFAR. This is an asynchronous fault. Therefore, if it is detected when the priority of the current process is higher than the BusFault priority, the BusFault becomes pending and becomes active only when the processor returns from all higher priority processes. If a precise fault occurs before the processor enters the handler for the imprecise BusFault, the handler detects both IMPRECISERR set to 1 and one of the precise fault status bits set to 1."]
  #[inline] pub fn impreciserr(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
#[doc="Imprecise data bus error: 0 = no imprecise data bus error, 1 = a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error. When the processor sets this bit to 1, it does not write a fault address to the BFAR. This is an asynchronous fault. Therefore, if it is detected when the priority of the current process is higher than the BusFault priority, the BusFault becomes pending and becomes active only when the processor returns from all higher priority processes. If a precise fault occurs before the processor enters the handler for the imprecise BusFault, the handler detects both IMPRECISERR set to 1 and one of the precise fault status bits set to 1."]
  #[inline] pub fn set_impreciserr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Precise data bus error: 0 = no precise data bus error, 1 = a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault. When the processor sets this bit is 1, it writes the faulting address to the BFAR."]
  #[inline] pub fn preciserr(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
#[doc="Precise data bus error: 0 = no precise data bus error, 1 = a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault. When the processor sets this bit is 1, it writes the faulting address to the BFAR."]
  #[inline] pub fn set_preciserr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Instruction bus error: 0 = no instruction bus error, 1 = instruction bus error. The processor detects the instruction bus error on prefetching an instruction, but it sets the IBUSERR flag to 1 only if it attempts to issue the faulting instruction. When the processor sets this bit is 1, it does not write a fault address to the BFAR."]
  #[inline] pub fn ibuserr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
#[doc="Instruction bus error: 0 = no instruction bus error, 1 = instruction bus error. The processor detects the instruction bus error on prefetching an instruction, but it sets the IBUSERR flag to 1 only if it attempts to issue the faulting instruction. When the processor sets this bit is 1, it does not write a fault address to the BFAR."]
  #[inline] pub fn set_ibuserr(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Ufsr(pub u16);
impl Ufsr {
#[doc="Divide by zero UsageFault: 0 = no divide by zero fault, or divide by zero trapping not enabled, 1 = the processor has executed an SDIV or UDIV instruction with a divisor of 0. When the processor sets this bit to 1, the PC value stacked for the exception return points to the instruction that performed the divide by zero. Enable trapping of divide by zero by setting the DIV_0_TRP bit in the CCR to 1, see Configuration and Control Register."]
  #[inline] pub fn divbyzero(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0x1 // [9]
  }
#[doc="Divide by zero UsageFault: 0 = no divide by zero fault, or divide by zero trapping not enabled, 1 = the processor has executed an SDIV or UDIV instruction with a divisor of 0. When the processor sets this bit to 1, the PC value stacked for the exception return points to the instruction that performed the divide by zero. Enable trapping of divide by zero by setting the DIV_0_TRP bit in the CCR to 1, see Configuration and Control Register."]
  #[inline] pub fn set_divbyzero(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Unaligned access UsageFault: 0 = no unaligned access fault, or unaligned access trapping not enabled, 1 = the processor has made an unaligned memory access. Enable trapping of unaligned accesses by setting the UNALIGN_TRP bit in the CCR to 1, see Configuration and Control Register. Unaligned LDM, STM, LDRD, and STRD instructions always fault irrespective of the setting of UNALIGN_TRP."]
  #[inline] pub fn unaligned(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x1 // [8]
  }
#[doc="Unaligned access UsageFault: 0 = no unaligned access fault, or unaligned access trapping not enabled, 1 = the processor has made an unaligned memory access. Enable trapping of unaligned accesses by setting the UNALIGN_TRP bit in the CCR to 1, see Configuration and Control Register. Unaligned LDM, STM, LDRD, and STRD instructions always fault irrespective of the setting of UNALIGN_TRP."]
  #[inline] pub fn set_unaligned(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="No coprocessor UsageFault. The processor does not support coprocessor instructions: 0 = no UsageFault caused by attempting to access a coprocessor, 1 = the processor has attempted to access a coprocessor."]
  #[inline] pub fn nocp(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
#[doc="No coprocessor UsageFault. The processor does not support coprocessor instructions: 0 = no UsageFault caused by attempting to access a coprocessor, 1 = the processor has attempted to access a coprocessor."]
  #[inline] pub fn set_nocp(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN: 0 = no invalid PC load UsageFault, 1 = the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that tried to perform the illegal load of the PC."]
  #[inline] pub fn invpc(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN: 0 = no invalid PC load UsageFault, 1 = the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that tried to perform the illegal load of the PC."]
  #[inline] pub fn set_invpc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Invalid state UsageFault: 0 = no invalid state UsageFault, 1 = the processor has attempted to execute an instruction that makes illegal use of the EPSR. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that attempted the illegal use of the EPSR. This bit is not set to 1 if an undefined instruction uses the EPSR."]
  #[inline] pub fn invstate(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Invalid state UsageFault: 0 = no invalid state UsageFault, 1 = the processor has attempted to execute an instruction that makes illegal use of the EPSR. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that attempted the illegal use of the EPSR. This bit is not set to 1 if an undefined instruction uses the EPSR."]
  #[inline] pub fn set_invstate(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Undefined instruction UsageFault: 0 = no undefined instruction UsageFault, 1 = the processor has attempted to execute an undefined instruction. When this bit is set to 1, the PC value stacked for the exception return points to the undefined instruction. An undefined instruction is an instruction that the processor cannot decode."]
  #[inline] pub fn undefinstr(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Undefined instruction UsageFault: 0 = no undefined instruction UsageFault, 1 = the processor has attempted to execute an undefined instruction. When this bit is set to 1, the PC value stacked for the exception return points to the undefined instruction. An undefined instruction is an instruction that the processor cannot decode."]
  #[inline] pub fn set_undefinstr(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Hfsr(pub u32);
impl Hfsr {
#[doc="Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
  #[inline] pub fn debugevt(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
  #[inline] pub fn set_debugevt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled: 0 = no forced HardFault, 1 = forced HardFault. When this bit is set to 1, the HardFault handler must read the other fault status registers to find the cause of the fault."]
  #[inline] pub fn forced(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled: 0 = no forced HardFault, 1 = forced HardFault. When this bit is set to 1, the HardFault handler must read the other fault status registers to find the cause of the fault."]
  #[inline] pub fn set_forced(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Indicates a BusFault on a vector table read during exception processing: 0 = no BusFault on vector table read, 1 = BusFault on vector table read. This error is always handled by the hard fault handler. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that was preempted by the exception."]
  #[inline] pub fn vecttbl(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Indicates a BusFault on a vector table read during exception processing: 0 = no BusFault on vector table read, 1 = BusFault on vector table read. This error is always handled by the hard fault handler. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that was preempted by the exception."]
  #[inline] pub fn set_vecttbl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
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
#[derive(PartialEq, Eq)]
pub struct Mmfar(pub u32);
impl Mmfar {
#[doc="When the MMARVALID bit of the MMFSR is set to 1, this field holds the address of the location that generated the MemManage fault"]
  #[inline] pub fn address(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="When the MMARVALID bit of the MMFSR is set to 1, this field holds the address of the location that generated the MemManage fault"]
  #[inline] pub fn set_address(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Bfar(pub u32);
impl Bfar {
#[doc="When the BFARVALID bit of the BFSR is set to 1, this field holds the address of the location that generated the BusFault"]
  #[inline] pub fn address(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="When the BFARVALID bit of the BFSR is set to 1, this field holds the address of the location that generated the BusFault"]
  #[inline] pub fn set_address(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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
#[derive(PartialEq, Eq)]
pub struct Afsr(pub u32);
impl Afsr {
#[doc="Implementation defined. The bits map to the AUXFAULT input signals."]
  #[inline] pub fn impdef(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="Implementation defined. The bits map to the AUXFAULT input signals."]
  #[inline] pub fn set_impdef(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
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

