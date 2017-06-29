//! PCC
pub const PCC: Pcc = Pcc(0x40065000);

#[doc="PCC"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcc(pub u32);
impl Pcc {
#[doc="Get the *const pointer for the PCCDUMMY0 register."]
  #[inline] pub fn pccdummy0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY0 register."]
  #[inline] pub fn pccdummy0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the PCCDUMMY0 register."]
  #[inline] pub fn pccdummy0(&self) -> Pccdummy0 { 
     unsafe {
        Pccdummy0(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY0 register."]
  #[inline] pub fn set_pccdummy0(&self, value: Pccdummy0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY0 register."]
  #[inline] pub fn with_pccdummy0<F: FnOnce(Pccdummy0) -> Pccdummy0>(&self, f: F) -> &Self {
     let tmp = self.pccdummy0();
     self.set_pccdummy0(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY1 register."]
  #[inline] pub fn pccdummy1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY1 register."]
  #[inline] pub fn pccdummy1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the PCCDUMMY1 register."]
  #[inline] pub fn pccdummy1(&self) -> Pccdummy1 { 
     unsafe {
        Pccdummy1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY1 register."]
  #[inline] pub fn set_pccdummy1(&self, value: Pccdummy1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY1 register."]
  #[inline] pub fn with_pccdummy1<F: FnOnce(Pccdummy1) -> Pccdummy1>(&self, f: F) -> &Self {
     let tmp = self.pccdummy1();
     self.set_pccdummy1(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY2 register."]
  #[inline] pub fn pccdummy2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY2 register."]
  #[inline] pub fn pccdummy2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the PCCDUMMY2 register."]
  #[inline] pub fn pccdummy2(&self) -> Pccdummy2 { 
     unsafe {
        Pccdummy2(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY2 register."]
  #[inline] pub fn set_pccdummy2(&self, value: Pccdummy2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY2 register."]
  #[inline] pub fn with_pccdummy2<F: FnOnce(Pccdummy2) -> Pccdummy2>(&self, f: F) -> &Self {
     let tmp = self.pccdummy2();
     self.set_pccdummy2(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY3 register."]
  #[inline] pub fn pccdummy3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY3 register."]
  #[inline] pub fn pccdummy3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the PCCDUMMY3 register."]
  #[inline] pub fn pccdummy3(&self) -> Pccdummy3 { 
     unsafe {
        Pccdummy3(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY3 register."]
  #[inline] pub fn set_pccdummy3(&self, value: Pccdummy3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY3 register."]
  #[inline] pub fn with_pccdummy3<F: FnOnce(Pccdummy3) -> Pccdummy3>(&self, f: F) -> &Self {
     let tmp = self.pccdummy3();
     self.set_pccdummy3(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY4 register."]
  #[inline] pub fn pccdummy4_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY4 register."]
  #[inline] pub fn pccdummy4_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the PCCDUMMY4 register."]
  #[inline] pub fn pccdummy4(&self) -> Pccdummy4 { 
     unsafe {
        Pccdummy4(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY4 register."]
  #[inline] pub fn set_pccdummy4(&self, value: Pccdummy4) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY4 register."]
  #[inline] pub fn with_pccdummy4<F: FnOnce(Pccdummy4) -> Pccdummy4>(&self, f: F) -> &Self {
     let tmp = self.pccdummy4();
     self.set_pccdummy4(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY5 register."]
  #[inline] pub fn pccdummy5_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY5 register."]
  #[inline] pub fn pccdummy5_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the PCCDUMMY5 register."]
  #[inline] pub fn pccdummy5(&self) -> Pccdummy5 { 
     unsafe {
        Pccdummy5(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY5 register."]
  #[inline] pub fn set_pccdummy5(&self, value: Pccdummy5) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY5 register."]
  #[inline] pub fn with_pccdummy5<F: FnOnce(Pccdummy5) -> Pccdummy5>(&self, f: F) -> &Self {
     let tmp = self.pccdummy5();
     self.set_pccdummy5(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY6 register."]
  #[inline] pub fn pccdummy6_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY6 register."]
  #[inline] pub fn pccdummy6_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the PCCDUMMY6 register."]
  #[inline] pub fn pccdummy6(&self) -> Pccdummy6 { 
     unsafe {
        Pccdummy6(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY6 register."]
  #[inline] pub fn set_pccdummy6(&self, value: Pccdummy6) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY6 register."]
  #[inline] pub fn with_pccdummy6<F: FnOnce(Pccdummy6) -> Pccdummy6>(&self, f: F) -> &Self {
     let tmp = self.pccdummy6();
     self.set_pccdummy6(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY7 register."]
  #[inline] pub fn pccdummy7_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY7 register."]
  #[inline] pub fn pccdummy7_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the PCCDUMMY7 register."]
  #[inline] pub fn pccdummy7(&self) -> Pccdummy7 { 
     unsafe {
        Pccdummy7(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY7 register."]
  #[inline] pub fn set_pccdummy7(&self, value: Pccdummy7) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY7 register."]
  #[inline] pub fn with_pccdummy7<F: FnOnce(Pccdummy7) -> Pccdummy7>(&self, f: F) -> &Self {
     let tmp = self.pccdummy7();
     self.set_pccdummy7(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY8 register."]
  #[inline] pub fn pccdummy8_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY8 register."]
  #[inline] pub fn pccdummy8_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the PCCDUMMY8 register."]
  #[inline] pub fn pccdummy8(&self) -> Pccdummy8 { 
     unsafe {
        Pccdummy8(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY8 register."]
  #[inline] pub fn set_pccdummy8(&self, value: Pccdummy8) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY8 register."]
  #[inline] pub fn with_pccdummy8<F: FnOnce(Pccdummy8) -> Pccdummy8>(&self, f: F) -> &Self {
     let tmp = self.pccdummy8();
     self.set_pccdummy8(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY9 register."]
  #[inline] pub fn pccdummy9_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY9 register."]
  #[inline] pub fn pccdummy9_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the PCCDUMMY9 register."]
  #[inline] pub fn pccdummy9(&self) -> Pccdummy9 { 
     unsafe {
        Pccdummy9(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY9 register."]
  #[inline] pub fn set_pccdummy9(&self, value: Pccdummy9) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY9 register."]
  #[inline] pub fn with_pccdummy9<F: FnOnce(Pccdummy9) -> Pccdummy9>(&self, f: F) -> &Self {
     let tmp = self.pccdummy9();
     self.set_pccdummy9(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY10 register."]
  #[inline] pub fn pccdummy10_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY10 register."]
  #[inline] pub fn pccdummy10_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the PCCDUMMY10 register."]
  #[inline] pub fn pccdummy10(&self) -> Pccdummy10 { 
     unsafe {
        Pccdummy10(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY10 register."]
  #[inline] pub fn set_pccdummy10(&self, value: Pccdummy10) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY10 register."]
  #[inline] pub fn with_pccdummy10<F: FnOnce(Pccdummy10) -> Pccdummy10>(&self, f: F) -> &Self {
     let tmp = self.pccdummy10();
     self.set_pccdummy10(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY11 register."]
  #[inline] pub fn pccdummy11_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY11 register."]
  #[inline] pub fn pccdummy11_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
#[doc="Read the PCCDUMMY11 register."]
  #[inline] pub fn pccdummy11(&self) -> Pccdummy11 { 
     unsafe {
        Pccdummy11(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY11 register."]
  #[inline] pub fn set_pccdummy11(&self, value: Pccdummy11) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY11 register."]
  #[inline] pub fn with_pccdummy11<F: FnOnce(Pccdummy11) -> Pccdummy11>(&self, f: F) -> &Self {
     let tmp = self.pccdummy11();
     self.set_pccdummy11(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY12 register."]
  #[inline] pub fn pccdummy12_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY12 register."]
  #[inline] pub fn pccdummy12_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
#[doc="Read the PCCDUMMY12 register."]
  #[inline] pub fn pccdummy12(&self) -> Pccdummy12 { 
     unsafe {
        Pccdummy12(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY12 register."]
  #[inline] pub fn set_pccdummy12(&self, value: Pccdummy12) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY12 register."]
  #[inline] pub fn with_pccdummy12<F: FnOnce(Pccdummy12) -> Pccdummy12>(&self, f: F) -> &Self {
     let tmp = self.pccdummy12();
     self.set_pccdummy12(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY13 register."]
  #[inline] pub fn pccdummy13_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY13 register."]
  #[inline] pub fn pccdummy13_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
#[doc="Read the PCCDUMMY13 register."]
  #[inline] pub fn pccdummy13(&self) -> Pccdummy13 { 
     unsafe {
        Pccdummy13(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY13 register."]
  #[inline] pub fn set_pccdummy13(&self, value: Pccdummy13) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY13 register."]
  #[inline] pub fn with_pccdummy13<F: FnOnce(Pccdummy13) -> Pccdummy13>(&self, f: F) -> &Self {
     let tmp = self.pccdummy13();
     self.set_pccdummy13(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY14 register."]
  #[inline] pub fn pccdummy14_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY14 register."]
  #[inline] pub fn pccdummy14_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
#[doc="Read the PCCDUMMY14 register."]
  #[inline] pub fn pccdummy14(&self) -> Pccdummy14 { 
     unsafe {
        Pccdummy14(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY14 register."]
  #[inline] pub fn set_pccdummy14(&self, value: Pccdummy14) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY14 register."]
  #[inline] pub fn with_pccdummy14<F: FnOnce(Pccdummy14) -> Pccdummy14>(&self, f: F) -> &Self {
     let tmp = self.pccdummy14();
     self.set_pccdummy14(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY15 register."]
  #[inline] pub fn pccdummy15_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY15 register."]
  #[inline] pub fn pccdummy15_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3c) as *mut u32
  }
#[doc="Read the PCCDUMMY15 register."]
  #[inline] pub fn pccdummy15(&self) -> Pccdummy15 { 
     unsafe {
        Pccdummy15(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY15 register."]
  #[inline] pub fn set_pccdummy15(&self, value: Pccdummy15) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY15 register."]
  #[inline] pub fn with_pccdummy15<F: FnOnce(Pccdummy15) -> Pccdummy15>(&self, f: F) -> &Self {
     let tmp = self.pccdummy15();
     self.set_pccdummy15(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY16 register."]
  #[inline] pub fn pccdummy16_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY16 register."]
  #[inline] pub fn pccdummy16_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
#[doc="Read the PCCDUMMY16 register."]
  #[inline] pub fn pccdummy16(&self) -> Pccdummy16 { 
     unsafe {
        Pccdummy16(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY16 register."]
  #[inline] pub fn set_pccdummy16(&self, value: Pccdummy16) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY16 register."]
  #[inline] pub fn with_pccdummy16<F: FnOnce(Pccdummy16) -> Pccdummy16>(&self, f: F) -> &Self {
     let tmp = self.pccdummy16();
     self.set_pccdummy16(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY17 register."]
  #[inline] pub fn pccdummy17_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY17 register."]
  #[inline] pub fn pccdummy17_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
#[doc="Read the PCCDUMMY17 register."]
  #[inline] pub fn pccdummy17(&self) -> Pccdummy17 { 
     unsafe {
        Pccdummy17(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY17 register."]
  #[inline] pub fn set_pccdummy17(&self, value: Pccdummy17) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY17 register."]
  #[inline] pub fn with_pccdummy17<F: FnOnce(Pccdummy17) -> Pccdummy17>(&self, f: F) -> &Self {
     let tmp = self.pccdummy17();
     self.set_pccdummy17(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY18 register."]
  #[inline] pub fn pccdummy18_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY18 register."]
  #[inline] pub fn pccdummy18_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
#[doc="Read the PCCDUMMY18 register."]
  #[inline] pub fn pccdummy18(&self) -> Pccdummy18 { 
     unsafe {
        Pccdummy18(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY18 register."]
  #[inline] pub fn set_pccdummy18(&self, value: Pccdummy18) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY18 register."]
  #[inline] pub fn with_pccdummy18<F: FnOnce(Pccdummy18) -> Pccdummy18>(&self, f: F) -> &Self {
     let tmp = self.pccdummy18();
     self.set_pccdummy18(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY19 register."]
  #[inline] pub fn pccdummy19_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY19 register."]
  #[inline] pub fn pccdummy19_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
#[doc="Read the PCCDUMMY19 register."]
  #[inline] pub fn pccdummy19(&self) -> Pccdummy19 { 
     unsafe {
        Pccdummy19(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY19 register."]
  #[inline] pub fn set_pccdummy19(&self, value: Pccdummy19) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY19 register."]
  #[inline] pub fn with_pccdummy19<F: FnOnce(Pccdummy19) -> Pccdummy19>(&self, f: F) -> &Self {
     let tmp = self.pccdummy19();
     self.set_pccdummy19(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY20 register."]
  #[inline] pub fn pccdummy20_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY20 register."]
  #[inline] pub fn pccdummy20_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
#[doc="Read the PCCDUMMY20 register."]
  #[inline] pub fn pccdummy20(&self) -> Pccdummy20 { 
     unsafe {
        Pccdummy20(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY20 register."]
  #[inline] pub fn set_pccdummy20(&self, value: Pccdummy20) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY20 register."]
  #[inline] pub fn with_pccdummy20<F: FnOnce(Pccdummy20) -> Pccdummy20>(&self, f: F) -> &Self {
     let tmp = self.pccdummy20();
     self.set_pccdummy20(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY21 register."]
  #[inline] pub fn pccdummy21_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x54) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY21 register."]
  #[inline] pub fn pccdummy21_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x54) as *mut u32
  }
#[doc="Read the PCCDUMMY21 register."]
  #[inline] pub fn pccdummy21(&self) -> Pccdummy21 { 
     unsafe {
        Pccdummy21(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY21 register."]
  #[inline] pub fn set_pccdummy21(&self, value: Pccdummy21) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY21 register."]
  #[inline] pub fn with_pccdummy21<F: FnOnce(Pccdummy21) -> Pccdummy21>(&self, f: F) -> &Self {
     let tmp = self.pccdummy21();
     self.set_pccdummy21(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY22 register."]
  #[inline] pub fn pccdummy22_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY22 register."]
  #[inline] pub fn pccdummy22_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
#[doc="Read the PCCDUMMY22 register."]
  #[inline] pub fn pccdummy22(&self) -> Pccdummy22 { 
     unsafe {
        Pccdummy22(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY22 register."]
  #[inline] pub fn set_pccdummy22(&self, value: Pccdummy22) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY22 register."]
  #[inline] pub fn with_pccdummy22<F: FnOnce(Pccdummy22) -> Pccdummy22>(&self, f: F) -> &Self {
     let tmp = self.pccdummy22();
     self.set_pccdummy22(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY23 register."]
  #[inline] pub fn pccdummy23_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x5c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY23 register."]
  #[inline] pub fn pccdummy23_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x5c) as *mut u32
  }
#[doc="Read the PCCDUMMY23 register."]
  #[inline] pub fn pccdummy23(&self) -> Pccdummy23 { 
     unsafe {
        Pccdummy23(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY23 register."]
  #[inline] pub fn set_pccdummy23(&self, value: Pccdummy23) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY23 register."]
  #[inline] pub fn with_pccdummy23<F: FnOnce(Pccdummy23) -> Pccdummy23>(&self, f: F) -> &Self {
     let tmp = self.pccdummy23();
     self.set_pccdummy23(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY24 register."]
  #[inline] pub fn pccdummy24_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x60) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY24 register."]
  #[inline] pub fn pccdummy24_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x60) as *mut u32
  }
#[doc="Read the PCCDUMMY24 register."]
  #[inline] pub fn pccdummy24(&self) -> Pccdummy24 { 
     unsafe {
        Pccdummy24(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY24 register."]
  #[inline] pub fn set_pccdummy24(&self, value: Pccdummy24) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY24 register."]
  #[inline] pub fn with_pccdummy24<F: FnOnce(Pccdummy24) -> Pccdummy24>(&self, f: F) -> &Self {
     let tmp = self.pccdummy24();
     self.set_pccdummy24(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY25 register."]
  #[inline] pub fn pccdummy25_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x64) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY25 register."]
  #[inline] pub fn pccdummy25_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x64) as *mut u32
  }
#[doc="Read the PCCDUMMY25 register."]
  #[inline] pub fn pccdummy25(&self) -> Pccdummy25 { 
     unsafe {
        Pccdummy25(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY25 register."]
  #[inline] pub fn set_pccdummy25(&self, value: Pccdummy25) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY25 register."]
  #[inline] pub fn with_pccdummy25<F: FnOnce(Pccdummy25) -> Pccdummy25>(&self, f: F) -> &Self {
     let tmp = self.pccdummy25();
     self.set_pccdummy25(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY26 register."]
  #[inline] pub fn pccdummy26_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x68) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY26 register."]
  #[inline] pub fn pccdummy26_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x68) as *mut u32
  }
#[doc="Read the PCCDUMMY26 register."]
  #[inline] pub fn pccdummy26(&self) -> Pccdummy26 { 
     unsafe {
        Pccdummy26(::core::ptr::read_volatile(((self.0 as usize) + 0x68) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY26 register."]
  #[inline] pub fn set_pccdummy26(&self, value: Pccdummy26) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x68) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY26 register."]
  #[inline] pub fn with_pccdummy26<F: FnOnce(Pccdummy26) -> Pccdummy26>(&self, f: F) -> &Self {
     let tmp = self.pccdummy26();
     self.set_pccdummy26(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY27 register."]
  #[inline] pub fn pccdummy27_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x6c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY27 register."]
  #[inline] pub fn pccdummy27_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x6c) as *mut u32
  }
#[doc="Read the PCCDUMMY27 register."]
  #[inline] pub fn pccdummy27(&self) -> Pccdummy27 { 
     unsafe {
        Pccdummy27(::core::ptr::read_volatile(((self.0 as usize) + 0x6c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY27 register."]
  #[inline] pub fn set_pccdummy27(&self, value: Pccdummy27) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY27 register."]
  #[inline] pub fn with_pccdummy27<F: FnOnce(Pccdummy27) -> Pccdummy27>(&self, f: F) -> &Self {
     let tmp = self.pccdummy27();
     self.set_pccdummy27(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY28 register."]
  #[inline] pub fn pccdummy28_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x70) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY28 register."]
  #[inline] pub fn pccdummy28_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x70) as *mut u32
  }
#[doc="Read the PCCDUMMY28 register."]
  #[inline] pub fn pccdummy28(&self) -> Pccdummy28 { 
     unsafe {
        Pccdummy28(::core::ptr::read_volatile(((self.0 as usize) + 0x70) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY28 register."]
  #[inline] pub fn set_pccdummy28(&self, value: Pccdummy28) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x70) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY28 register."]
  #[inline] pub fn with_pccdummy28<F: FnOnce(Pccdummy28) -> Pccdummy28>(&self, f: F) -> &Self {
     let tmp = self.pccdummy28();
     self.set_pccdummy28(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY29 register."]
  #[inline] pub fn pccdummy29_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x74) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY29 register."]
  #[inline] pub fn pccdummy29_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x74) as *mut u32
  }
#[doc="Read the PCCDUMMY29 register."]
  #[inline] pub fn pccdummy29(&self) -> Pccdummy29 { 
     unsafe {
        Pccdummy29(::core::ptr::read_volatile(((self.0 as usize) + 0x74) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY29 register."]
  #[inline] pub fn set_pccdummy29(&self, value: Pccdummy29) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x74) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY29 register."]
  #[inline] pub fn with_pccdummy29<F: FnOnce(Pccdummy29) -> Pccdummy29>(&self, f: F) -> &Self {
     let tmp = self.pccdummy29();
     self.set_pccdummy29(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY30 register."]
  #[inline] pub fn pccdummy30_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x78) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY30 register."]
  #[inline] pub fn pccdummy30_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x78) as *mut u32
  }
#[doc="Read the PCCDUMMY30 register."]
  #[inline] pub fn pccdummy30(&self) -> Pccdummy30 { 
     unsafe {
        Pccdummy30(::core::ptr::read_volatile(((self.0 as usize) + 0x78) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY30 register."]
  #[inline] pub fn set_pccdummy30(&self, value: Pccdummy30) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x78) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY30 register."]
  #[inline] pub fn with_pccdummy30<F: FnOnce(Pccdummy30) -> Pccdummy30>(&self, f: F) -> &Self {
     let tmp = self.pccdummy30();
     self.set_pccdummy30(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY31 register."]
  #[inline] pub fn pccdummy31_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x7c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY31 register."]
  #[inline] pub fn pccdummy31_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x7c) as *mut u32
  }
#[doc="Read the PCCDUMMY31 register."]
  #[inline] pub fn pccdummy31(&self) -> Pccdummy31 { 
     unsafe {
        Pccdummy31(::core::ptr::read_volatile(((self.0 as usize) + 0x7c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY31 register."]
  #[inline] pub fn set_pccdummy31(&self, value: Pccdummy31) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x7c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY31 register."]
  #[inline] pub fn with_pccdummy31<F: FnOnce(Pccdummy31) -> Pccdummy31>(&self, f: F) -> &Self {
     let tmp = self.pccdummy31();
     self.set_pccdummy31(f(tmp))
  }

#[doc="Get the *const pointer for the FTFC register."]
  #[inline] pub fn ftfc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x80) as *const u32
  }
#[doc="Get the *mut pointer for the FTFC register."]
  #[inline] pub fn ftfc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x80) as *mut u32
  }
#[doc="Read the FTFC register."]
  #[inline] pub fn ftfc(&self) -> Ftfc { 
     unsafe {
        Ftfc(::core::ptr::read_volatile(((self.0 as usize) + 0x80) as *const u32))
     }
  }
#[doc="Write the FTFC register."]
  #[inline] pub fn set_ftfc(&self, value: Ftfc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x80) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FTFC register."]
  #[inline] pub fn with_ftfc<F: FnOnce(Ftfc) -> Ftfc>(&self, f: F) -> &Self {
     let tmp = self.ftfc();
     self.set_ftfc(f(tmp))
  }

#[doc="Get the *const pointer for the DMAMUX register."]
  #[inline] pub fn dmamux_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x84) as *const u32
  }
#[doc="Get the *mut pointer for the DMAMUX register."]
  #[inline] pub fn dmamux_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x84) as *mut u32
  }
#[doc="Read the DMAMUX register."]
  #[inline] pub fn dmamux(&self) -> Dmamux { 
     unsafe {
        Dmamux(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
     }
  }
#[doc="Write the DMAMUX register."]
  #[inline] pub fn set_dmamux(&self, value: Dmamux) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DMAMUX register."]
  #[inline] pub fn with_dmamux<F: FnOnce(Dmamux) -> Dmamux>(&self, f: F) -> &Self {
     let tmp = self.dmamux();
     self.set_dmamux(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY34 register."]
  #[inline] pub fn pccdummy34_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x88) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY34 register."]
  #[inline] pub fn pccdummy34_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x88) as *mut u32
  }
#[doc="Read the PCCDUMMY34 register."]
  #[inline] pub fn pccdummy34(&self) -> Pccdummy34 { 
     unsafe {
        Pccdummy34(::core::ptr::read_volatile(((self.0 as usize) + 0x88) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY34 register."]
  #[inline] pub fn set_pccdummy34(&self, value: Pccdummy34) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x88) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY34 register."]
  #[inline] pub fn with_pccdummy34<F: FnOnce(Pccdummy34) -> Pccdummy34>(&self, f: F) -> &Self {
     let tmp = self.pccdummy34();
     self.set_pccdummy34(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY35 register."]
  #[inline] pub fn pccdummy35_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY35 register."]
  #[inline] pub fn pccdummy35_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8c) as *mut u32
  }
#[doc="Read the PCCDUMMY35 register."]
  #[inline] pub fn pccdummy35(&self) -> Pccdummy35 { 
     unsafe {
        Pccdummy35(::core::ptr::read_volatile(((self.0 as usize) + 0x8c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY35 register."]
  #[inline] pub fn set_pccdummy35(&self, value: Pccdummy35) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY35 register."]
  #[inline] pub fn with_pccdummy35<F: FnOnce(Pccdummy35) -> Pccdummy35>(&self, f: F) -> &Self {
     let tmp = self.pccdummy35();
     self.set_pccdummy35(f(tmp))
  }

#[doc="Get the *const pointer for the FLEXCAN0 register."]
  #[inline] pub fn flexcan0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x90) as *const u32
  }
#[doc="Get the *mut pointer for the FLEXCAN0 register."]
  #[inline] pub fn flexcan0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x90) as *mut u32
  }
#[doc="Read the FLEXCAN0 register."]
  #[inline] pub fn flexcan0(&self) -> Flexcan0 { 
     unsafe {
        Flexcan0(::core::ptr::read_volatile(((self.0 as usize) + 0x90) as *const u32))
     }
  }
#[doc="Write the FLEXCAN0 register."]
  #[inline] pub fn set_flexcan0(&self, value: Flexcan0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x90) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FLEXCAN0 register."]
  #[inline] pub fn with_flexcan0<F: FnOnce(Flexcan0) -> Flexcan0>(&self, f: F) -> &Self {
     let tmp = self.flexcan0();
     self.set_flexcan0(f(tmp))
  }

#[doc="Get the *const pointer for the FLEXCAN1 register."]
  #[inline] pub fn flexcan1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x94) as *const u32
  }
#[doc="Get the *mut pointer for the FLEXCAN1 register."]
  #[inline] pub fn flexcan1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x94) as *mut u32
  }
#[doc="Read the FLEXCAN1 register."]
  #[inline] pub fn flexcan1(&self) -> Flexcan1 { 
     unsafe {
        Flexcan1(::core::ptr::read_volatile(((self.0 as usize) + 0x94) as *const u32))
     }
  }
#[doc="Write the FLEXCAN1 register."]
  #[inline] pub fn set_flexcan1(&self, value: Flexcan1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x94) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FLEXCAN1 register."]
  #[inline] pub fn with_flexcan1<F: FnOnce(Flexcan1) -> Flexcan1>(&self, f: F) -> &Self {
     let tmp = self.flexcan1();
     self.set_flexcan1(f(tmp))
  }

#[doc="Get the *const pointer for the FTM3 register."]
  #[inline] pub fn ftm3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x98) as *const u32
  }
#[doc="Get the *mut pointer for the FTM3 register."]
  #[inline] pub fn ftm3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x98) as *mut u32
  }
#[doc="Read the FTM3 register."]
  #[inline] pub fn ftm3(&self) -> Ftm3 { 
     unsafe {
        Ftm3(::core::ptr::read_volatile(((self.0 as usize) + 0x98) as *const u32))
     }
  }
#[doc="Write the FTM3 register."]
  #[inline] pub fn set_ftm3(&self, value: Ftm3) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x98) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FTM3 register."]
  #[inline] pub fn with_ftm3<F: FnOnce(Ftm3) -> Ftm3>(&self, f: F) -> &Self {
     let tmp = self.ftm3();
     self.set_ftm3(f(tmp))
  }

#[doc="Get the *const pointer for the ADC1 register."]
  #[inline] pub fn adc1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x9c) as *const u32
  }
#[doc="Get the *mut pointer for the ADC1 register."]
  #[inline] pub fn adc1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x9c) as *mut u32
  }
#[doc="Read the ADC1 register."]
  #[inline] pub fn adc1(&self) -> Adc1 { 
     unsafe {
        Adc1(::core::ptr::read_volatile(((self.0 as usize) + 0x9c) as *const u32))
     }
  }
#[doc="Write the ADC1 register."]
  #[inline] pub fn set_adc1(&self, value: Adc1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x9c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ADC1 register."]
  #[inline] pub fn with_adc1<F: FnOnce(Adc1) -> Adc1>(&self, f: F) -> &Self {
     let tmp = self.adc1();
     self.set_adc1(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY40 register."]
  #[inline] pub fn pccdummy40_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa0) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY40 register."]
  #[inline] pub fn pccdummy40_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa0) as *mut u32
  }
#[doc="Read the PCCDUMMY40 register."]
  #[inline] pub fn pccdummy40(&self) -> Pccdummy40 { 
     unsafe {
        Pccdummy40(::core::ptr::read_volatile(((self.0 as usize) + 0xa0) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY40 register."]
  #[inline] pub fn set_pccdummy40(&self, value: Pccdummy40) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY40 register."]
  #[inline] pub fn with_pccdummy40<F: FnOnce(Pccdummy40) -> Pccdummy40>(&self, f: F) -> &Self {
     let tmp = self.pccdummy40();
     self.set_pccdummy40(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY41 register."]
  #[inline] pub fn pccdummy41_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa4) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY41 register."]
  #[inline] pub fn pccdummy41_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa4) as *mut u32
  }
#[doc="Read the PCCDUMMY41 register."]
  #[inline] pub fn pccdummy41(&self) -> Pccdummy41 { 
     unsafe {
        Pccdummy41(::core::ptr::read_volatile(((self.0 as usize) + 0xa4) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY41 register."]
  #[inline] pub fn set_pccdummy41(&self, value: Pccdummy41) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY41 register."]
  #[inline] pub fn with_pccdummy41<F: FnOnce(Pccdummy41) -> Pccdummy41>(&self, f: F) -> &Self {
     let tmp = self.pccdummy41();
     self.set_pccdummy41(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY42 register."]
  #[inline] pub fn pccdummy42_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xa8) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY42 register."]
  #[inline] pub fn pccdummy42_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xa8) as *mut u32
  }
#[doc="Read the PCCDUMMY42 register."]
  #[inline] pub fn pccdummy42(&self) -> Pccdummy42 { 
     unsafe {
        Pccdummy42(::core::ptr::read_volatile(((self.0 as usize) + 0xa8) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY42 register."]
  #[inline] pub fn set_pccdummy42(&self, value: Pccdummy42) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY42 register."]
  #[inline] pub fn with_pccdummy42<F: FnOnce(Pccdummy42) -> Pccdummy42>(&self, f: F) -> &Self {
     let tmp = self.pccdummy42();
     self.set_pccdummy42(f(tmp))
  }

#[doc="Get the *const pointer for the FLEXCAN2 register."]
  #[inline] pub fn flexcan2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xac) as *const u32
  }
#[doc="Get the *mut pointer for the FLEXCAN2 register."]
  #[inline] pub fn flexcan2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xac) as *mut u32
  }
#[doc="Read the FLEXCAN2 register."]
  #[inline] pub fn flexcan2(&self) -> Flexcan2 { 
     unsafe {
        Flexcan2(::core::ptr::read_volatile(((self.0 as usize) + 0xac) as *const u32))
     }
  }
#[doc="Write the FLEXCAN2 register."]
  #[inline] pub fn set_flexcan2(&self, value: Flexcan2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xac) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FLEXCAN2 register."]
  #[inline] pub fn with_flexcan2<F: FnOnce(Flexcan2) -> Flexcan2>(&self, f: F) -> &Self {
     let tmp = self.flexcan2();
     self.set_flexcan2(f(tmp))
  }

#[doc="Get the *const pointer for the LPSPI0 register."]
  #[inline] pub fn lpspi0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb0) as *const u32
  }
#[doc="Get the *mut pointer for the LPSPI0 register."]
  #[inline] pub fn lpspi0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb0) as *mut u32
  }
#[doc="Read the LPSPI0 register."]
  #[inline] pub fn lpspi0(&self) -> Lpspi0 { 
     unsafe {
        Lpspi0(::core::ptr::read_volatile(((self.0 as usize) + 0xb0) as *const u32))
     }
  }
#[doc="Write the LPSPI0 register."]
  #[inline] pub fn set_lpspi0(&self, value: Lpspi0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LPSPI0 register."]
  #[inline] pub fn with_lpspi0<F: FnOnce(Lpspi0) -> Lpspi0>(&self, f: F) -> &Self {
     let tmp = self.lpspi0();
     self.set_lpspi0(f(tmp))
  }

#[doc="Get the *const pointer for the LPSPI1 register."]
  #[inline] pub fn lpspi1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb4) as *const u32
  }
#[doc="Get the *mut pointer for the LPSPI1 register."]
  #[inline] pub fn lpspi1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb4) as *mut u32
  }
#[doc="Read the LPSPI1 register."]
  #[inline] pub fn lpspi1(&self) -> Lpspi1 { 
     unsafe {
        Lpspi1(::core::ptr::read_volatile(((self.0 as usize) + 0xb4) as *const u32))
     }
  }
#[doc="Write the LPSPI1 register."]
  #[inline] pub fn set_lpspi1(&self, value: Lpspi1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LPSPI1 register."]
  #[inline] pub fn with_lpspi1<F: FnOnce(Lpspi1) -> Lpspi1>(&self, f: F) -> &Self {
     let tmp = self.lpspi1();
     self.set_lpspi1(f(tmp))
  }

#[doc="Get the *const pointer for the LPSPI2 register."]
  #[inline] pub fn lpspi2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xb8) as *const u32
  }
#[doc="Get the *mut pointer for the LPSPI2 register."]
  #[inline] pub fn lpspi2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xb8) as *mut u32
  }
#[doc="Read the LPSPI2 register."]
  #[inline] pub fn lpspi2(&self) -> Lpspi2 { 
     unsafe {
        Lpspi2(::core::ptr::read_volatile(((self.0 as usize) + 0xb8) as *const u32))
     }
  }
#[doc="Write the LPSPI2 register."]
  #[inline] pub fn set_lpspi2(&self, value: Lpspi2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xb8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LPSPI2 register."]
  #[inline] pub fn with_lpspi2<F: FnOnce(Lpspi2) -> Lpspi2>(&self, f: F) -> &Self {
     let tmp = self.lpspi2();
     self.set_lpspi2(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY47 register."]
  #[inline] pub fn pccdummy47_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xbc) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY47 register."]
  #[inline] pub fn pccdummy47_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xbc) as *mut u32
  }
#[doc="Read the PCCDUMMY47 register."]
  #[inline] pub fn pccdummy47(&self) -> Pccdummy47 { 
     unsafe {
        Pccdummy47(::core::ptr::read_volatile(((self.0 as usize) + 0xbc) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY47 register."]
  #[inline] pub fn set_pccdummy47(&self, value: Pccdummy47) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xbc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY47 register."]
  #[inline] pub fn with_pccdummy47<F: FnOnce(Pccdummy47) -> Pccdummy47>(&self, f: F) -> &Self {
     let tmp = self.pccdummy47();
     self.set_pccdummy47(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY48 register."]
  #[inline] pub fn pccdummy48_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc0) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY48 register."]
  #[inline] pub fn pccdummy48_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc0) as *mut u32
  }
#[doc="Read the PCCDUMMY48 register."]
  #[inline] pub fn pccdummy48(&self) -> Pccdummy48 { 
     unsafe {
        Pccdummy48(::core::ptr::read_volatile(((self.0 as usize) + 0xc0) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY48 register."]
  #[inline] pub fn set_pccdummy48(&self, value: Pccdummy48) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY48 register."]
  #[inline] pub fn with_pccdummy48<F: FnOnce(Pccdummy48) -> Pccdummy48>(&self, f: F) -> &Self {
     let tmp = self.pccdummy48();
     self.set_pccdummy48(f(tmp))
  }

#[doc="Get the *const pointer for the PDB1 register."]
  #[inline] pub fn pdb1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc4) as *const u32
  }
#[doc="Get the *mut pointer for the PDB1 register."]
  #[inline] pub fn pdb1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc4) as *mut u32
  }
#[doc="Read the PDB1 register."]
  #[inline] pub fn pdb1(&self) -> Pdb1 { 
     unsafe {
        Pdb1(::core::ptr::read_volatile(((self.0 as usize) + 0xc4) as *const u32))
     }
  }
#[doc="Write the PDB1 register."]
  #[inline] pub fn set_pdb1(&self, value: Pdb1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PDB1 register."]
  #[inline] pub fn with_pdb1<F: FnOnce(Pdb1) -> Pdb1>(&self, f: F) -> &Self {
     let tmp = self.pdb1();
     self.set_pdb1(f(tmp))
  }

#[doc="Get the *const pointer for the CRC register."]
  #[inline] pub fn crc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc8) as *const u32
  }
#[doc="Get the *mut pointer for the CRC register."]
  #[inline] pub fn crc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc8) as *mut u32
  }
#[doc="Read the CRC register."]
  #[inline] pub fn crc(&self) -> Crc { 
     unsafe {
        Crc(::core::ptr::read_volatile(((self.0 as usize) + 0xc8) as *const u32))
     }
  }
#[doc="Write the CRC register."]
  #[inline] pub fn set_crc(&self, value: Crc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CRC register."]
  #[inline] pub fn with_crc<F: FnOnce(Crc) -> Crc>(&self, f: F) -> &Self {
     let tmp = self.crc();
     self.set_crc(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY51 register."]
  #[inline] pub fn pccdummy51_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xcc) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY51 register."]
  #[inline] pub fn pccdummy51_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xcc) as *mut u32
  }
#[doc="Read the PCCDUMMY51 register."]
  #[inline] pub fn pccdummy51(&self) -> Pccdummy51 { 
     unsafe {
        Pccdummy51(::core::ptr::read_volatile(((self.0 as usize) + 0xcc) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY51 register."]
  #[inline] pub fn set_pccdummy51(&self, value: Pccdummy51) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xcc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY51 register."]
  #[inline] pub fn with_pccdummy51<F: FnOnce(Pccdummy51) -> Pccdummy51>(&self, f: F) -> &Self {
     let tmp = self.pccdummy51();
     self.set_pccdummy51(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY52 register."]
  #[inline] pub fn pccdummy52_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd0) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY52 register."]
  #[inline] pub fn pccdummy52_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd0) as *mut u32
  }
#[doc="Read the PCCDUMMY52 register."]
  #[inline] pub fn pccdummy52(&self) -> Pccdummy52 { 
     unsafe {
        Pccdummy52(::core::ptr::read_volatile(((self.0 as usize) + 0xd0) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY52 register."]
  #[inline] pub fn set_pccdummy52(&self, value: Pccdummy52) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY52 register."]
  #[inline] pub fn with_pccdummy52<F: FnOnce(Pccdummy52) -> Pccdummy52>(&self, f: F) -> &Self {
     let tmp = self.pccdummy52();
     self.set_pccdummy52(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY53 register."]
  #[inline] pub fn pccdummy53_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd4) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY53 register."]
  #[inline] pub fn pccdummy53_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd4) as *mut u32
  }
#[doc="Read the PCCDUMMY53 register."]
  #[inline] pub fn pccdummy53(&self) -> Pccdummy53 { 
     unsafe {
        Pccdummy53(::core::ptr::read_volatile(((self.0 as usize) + 0xd4) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY53 register."]
  #[inline] pub fn set_pccdummy53(&self, value: Pccdummy53) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY53 register."]
  #[inline] pub fn with_pccdummy53<F: FnOnce(Pccdummy53) -> Pccdummy53>(&self, f: F) -> &Self {
     let tmp = self.pccdummy53();
     self.set_pccdummy53(f(tmp))
  }

#[doc="Get the *const pointer for the PDB0 register."]
  #[inline] pub fn pdb0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xd8) as *const u32
  }
#[doc="Get the *mut pointer for the PDB0 register."]
  #[inline] pub fn pdb0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xd8) as *mut u32
  }
#[doc="Read the PDB0 register."]
  #[inline] pub fn pdb0(&self) -> Pdb0 { 
     unsafe {
        Pdb0(::core::ptr::read_volatile(((self.0 as usize) + 0xd8) as *const u32))
     }
  }
#[doc="Write the PDB0 register."]
  #[inline] pub fn set_pdb0(&self, value: Pdb0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xd8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PDB0 register."]
  #[inline] pub fn with_pdb0<F: FnOnce(Pdb0) -> Pdb0>(&self, f: F) -> &Self {
     let tmp = self.pdb0();
     self.set_pdb0(f(tmp))
  }

#[doc="Get the *const pointer for the LPIT register."]
  #[inline] pub fn lpit_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xdc) as *const u32
  }
#[doc="Get the *mut pointer for the LPIT register."]
  #[inline] pub fn lpit_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xdc) as *mut u32
  }
#[doc="Read the LPIT register."]
  #[inline] pub fn lpit(&self) -> Lpit { 
     unsafe {
        Lpit(::core::ptr::read_volatile(((self.0 as usize) + 0xdc) as *const u32))
     }
  }
#[doc="Write the LPIT register."]
  #[inline] pub fn set_lpit(&self, value: Lpit) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xdc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LPIT register."]
  #[inline] pub fn with_lpit<F: FnOnce(Lpit) -> Lpit>(&self, f: F) -> &Self {
     let tmp = self.lpit();
     self.set_lpit(f(tmp))
  }

#[doc="Get the *const pointer for the FTM0 register."]
  #[inline] pub fn ftm0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe0) as *const u32
  }
#[doc="Get the *mut pointer for the FTM0 register."]
  #[inline] pub fn ftm0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe0) as *mut u32
  }
#[doc="Read the FTM0 register."]
  #[inline] pub fn ftm0(&self) -> Ftm0 { 
     unsafe {
        Ftm0(::core::ptr::read_volatile(((self.0 as usize) + 0xe0) as *const u32))
     }
  }
#[doc="Write the FTM0 register."]
  #[inline] pub fn set_ftm0(&self, value: Ftm0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FTM0 register."]
  #[inline] pub fn with_ftm0<F: FnOnce(Ftm0) -> Ftm0>(&self, f: F) -> &Self {
     let tmp = self.ftm0();
     self.set_ftm0(f(tmp))
  }

#[doc="Get the *const pointer for the FTM1 register."]
  #[inline] pub fn ftm1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe4) as *const u32
  }
#[doc="Get the *mut pointer for the FTM1 register."]
  #[inline] pub fn ftm1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe4) as *mut u32
  }
#[doc="Read the FTM1 register."]
  #[inline] pub fn ftm1(&self) -> Ftm1 { 
     unsafe {
        Ftm1(::core::ptr::read_volatile(((self.0 as usize) + 0xe4) as *const u32))
     }
  }
#[doc="Write the FTM1 register."]
  #[inline] pub fn set_ftm1(&self, value: Ftm1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FTM1 register."]
  #[inline] pub fn with_ftm1<F: FnOnce(Ftm1) -> Ftm1>(&self, f: F) -> &Self {
     let tmp = self.ftm1();
     self.set_ftm1(f(tmp))
  }

#[doc="Get the *const pointer for the FTM2 register."]
  #[inline] pub fn ftm2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xe8) as *const u32
  }
#[doc="Get the *mut pointer for the FTM2 register."]
  #[inline] pub fn ftm2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xe8) as *mut u32
  }
#[doc="Read the FTM2 register."]
  #[inline] pub fn ftm2(&self) -> Ftm2 { 
     unsafe {
        Ftm2(::core::ptr::read_volatile(((self.0 as usize) + 0xe8) as *const u32))
     }
  }
#[doc="Write the FTM2 register."]
  #[inline] pub fn set_ftm2(&self, value: Ftm2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FTM2 register."]
  #[inline] pub fn with_ftm2<F: FnOnce(Ftm2) -> Ftm2>(&self, f: F) -> &Self {
     let tmp = self.ftm2();
     self.set_ftm2(f(tmp))
  }

#[doc="Get the *const pointer for the ADC0 register."]
  #[inline] pub fn adc0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xec) as *const u32
  }
#[doc="Get the *mut pointer for the ADC0 register."]
  #[inline] pub fn adc0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xec) as *mut u32
  }
#[doc="Read the ADC0 register."]
  #[inline] pub fn adc0(&self) -> Adc0 { 
     unsafe {
        Adc0(::core::ptr::read_volatile(((self.0 as usize) + 0xec) as *const u32))
     }
  }
#[doc="Write the ADC0 register."]
  #[inline] pub fn set_adc0(&self, value: Adc0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xec) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ADC0 register."]
  #[inline] pub fn with_adc0<F: FnOnce(Adc0) -> Adc0>(&self, f: F) -> &Self {
     let tmp = self.adc0();
     self.set_adc0(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY60 register."]
  #[inline] pub fn pccdummy60_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xf0) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY60 register."]
  #[inline] pub fn pccdummy60_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xf0) as *mut u32
  }
#[doc="Read the PCCDUMMY60 register."]
  #[inline] pub fn pccdummy60(&self) -> Pccdummy60 { 
     unsafe {
        Pccdummy60(::core::ptr::read_volatile(((self.0 as usize) + 0xf0) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY60 register."]
  #[inline] pub fn set_pccdummy60(&self, value: Pccdummy60) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xf0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY60 register."]
  #[inline] pub fn with_pccdummy60<F: FnOnce(Pccdummy60) -> Pccdummy60>(&self, f: F) -> &Self {
     let tmp = self.pccdummy60();
     self.set_pccdummy60(f(tmp))
  }

#[doc="Get the *const pointer for the RTC register."]
  #[inline] pub fn rtc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xf4) as *const u32
  }
#[doc="Get the *mut pointer for the RTC register."]
  #[inline] pub fn rtc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xf4) as *mut u32
  }
#[doc="Read the RTC register."]
  #[inline] pub fn rtc(&self) -> Rtc { 
     unsafe {
        Rtc(::core::ptr::read_volatile(((self.0 as usize) + 0xf4) as *const u32))
     }
  }
#[doc="Write the RTC register."]
  #[inline] pub fn set_rtc(&self, value: Rtc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xf4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RTC register."]
  #[inline] pub fn with_rtc<F: FnOnce(Rtc) -> Rtc>(&self, f: F) -> &Self {
     let tmp = self.rtc();
     self.set_rtc(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY62 register."]
  #[inline] pub fn pccdummy62_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xf8) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY62 register."]
  #[inline] pub fn pccdummy62_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xf8) as *mut u32
  }
#[doc="Read the PCCDUMMY62 register."]
  #[inline] pub fn pccdummy62(&self) -> Pccdummy62 { 
     unsafe {
        Pccdummy62(::core::ptr::read_volatile(((self.0 as usize) + 0xf8) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY62 register."]
  #[inline] pub fn set_pccdummy62(&self, value: Pccdummy62) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xf8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY62 register."]
  #[inline] pub fn with_pccdummy62<F: FnOnce(Pccdummy62) -> Pccdummy62>(&self, f: F) -> &Self {
     let tmp = self.pccdummy62();
     self.set_pccdummy62(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY63 register."]
  #[inline] pub fn pccdummy63_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY63 register."]
  #[inline] pub fn pccdummy63_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc) as *mut u32
  }
#[doc="Read the PCCDUMMY63 register."]
  #[inline] pub fn pccdummy63(&self) -> Pccdummy63 { 
     unsafe {
        Pccdummy63(::core::ptr::read_volatile(((self.0 as usize) + 0xfc) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY63 register."]
  #[inline] pub fn set_pccdummy63(&self, value: Pccdummy63) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xfc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY63 register."]
  #[inline] pub fn with_pccdummy63<F: FnOnce(Pccdummy63) -> Pccdummy63>(&self, f: F) -> &Self {
     let tmp = self.pccdummy63();
     self.set_pccdummy63(f(tmp))
  }

#[doc="Get the *const pointer for the LPTMR0 register."]
  #[inline] pub fn lptmr0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x100) as *const u32
  }
#[doc="Get the *mut pointer for the LPTMR0 register."]
  #[inline] pub fn lptmr0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x100) as *mut u32
  }
#[doc="Read the LPTMR0 register."]
  #[inline] pub fn lptmr0(&self) -> Lptmr0 { 
     unsafe {
        Lptmr0(::core::ptr::read_volatile(((self.0 as usize) + 0x100) as *const u32))
     }
  }
#[doc="Write the LPTMR0 register."]
  #[inline] pub fn set_lptmr0(&self, value: Lptmr0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LPTMR0 register."]
  #[inline] pub fn with_lptmr0<F: FnOnce(Lptmr0) -> Lptmr0>(&self, f: F) -> &Self {
     let tmp = self.lptmr0();
     self.set_lptmr0(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY65 register."]
  #[inline] pub fn pccdummy65_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x104) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY65 register."]
  #[inline] pub fn pccdummy65_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x104) as *mut u32
  }
#[doc="Read the PCCDUMMY65 register."]
  #[inline] pub fn pccdummy65(&self) -> Pccdummy65 { 
     unsafe {
        Pccdummy65(::core::ptr::read_volatile(((self.0 as usize) + 0x104) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY65 register."]
  #[inline] pub fn set_pccdummy65(&self, value: Pccdummy65) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x104) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY65 register."]
  #[inline] pub fn with_pccdummy65<F: FnOnce(Pccdummy65) -> Pccdummy65>(&self, f: F) -> &Self {
     let tmp = self.pccdummy65();
     self.set_pccdummy65(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY66 register."]
  #[inline] pub fn pccdummy66_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x108) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY66 register."]
  #[inline] pub fn pccdummy66_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x108) as *mut u32
  }
#[doc="Read the PCCDUMMY66 register."]
  #[inline] pub fn pccdummy66(&self) -> Pccdummy66 { 
     unsafe {
        Pccdummy66(::core::ptr::read_volatile(((self.0 as usize) + 0x108) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY66 register."]
  #[inline] pub fn set_pccdummy66(&self, value: Pccdummy66) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x108) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY66 register."]
  #[inline] pub fn with_pccdummy66<F: FnOnce(Pccdummy66) -> Pccdummy66>(&self, f: F) -> &Self {
     let tmp = self.pccdummy66();
     self.set_pccdummy66(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY67 register."]
  #[inline] pub fn pccdummy67_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY67 register."]
  #[inline] pub fn pccdummy67_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10c) as *mut u32
  }
#[doc="Read the PCCDUMMY67 register."]
  #[inline] pub fn pccdummy67(&self) -> Pccdummy67 { 
     unsafe {
        Pccdummy67(::core::ptr::read_volatile(((self.0 as usize) + 0x10c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY67 register."]
  #[inline] pub fn set_pccdummy67(&self, value: Pccdummy67) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY67 register."]
  #[inline] pub fn with_pccdummy67<F: FnOnce(Pccdummy67) -> Pccdummy67>(&self, f: F) -> &Self {
     let tmp = self.pccdummy67();
     self.set_pccdummy67(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY68 register."]
  #[inline] pub fn pccdummy68_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x110) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY68 register."]
  #[inline] pub fn pccdummy68_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x110) as *mut u32
  }
#[doc="Read the PCCDUMMY68 register."]
  #[inline] pub fn pccdummy68(&self) -> Pccdummy68 { 
     unsafe {
        Pccdummy68(::core::ptr::read_volatile(((self.0 as usize) + 0x110) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY68 register."]
  #[inline] pub fn set_pccdummy68(&self, value: Pccdummy68) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x110) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY68 register."]
  #[inline] pub fn with_pccdummy68<F: FnOnce(Pccdummy68) -> Pccdummy68>(&self, f: F) -> &Self {
     let tmp = self.pccdummy68();
     self.set_pccdummy68(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY69 register."]
  #[inline] pub fn pccdummy69_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x114) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY69 register."]
  #[inline] pub fn pccdummy69_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x114) as *mut u32
  }
#[doc="Read the PCCDUMMY69 register."]
  #[inline] pub fn pccdummy69(&self) -> Pccdummy69 { 
     unsafe {
        Pccdummy69(::core::ptr::read_volatile(((self.0 as usize) + 0x114) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY69 register."]
  #[inline] pub fn set_pccdummy69(&self, value: Pccdummy69) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x114) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY69 register."]
  #[inline] pub fn with_pccdummy69<F: FnOnce(Pccdummy69) -> Pccdummy69>(&self, f: F) -> &Self {
     let tmp = self.pccdummy69();
     self.set_pccdummy69(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY70 register."]
  #[inline] pub fn pccdummy70_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x118) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY70 register."]
  #[inline] pub fn pccdummy70_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x118) as *mut u32
  }
#[doc="Read the PCCDUMMY70 register."]
  #[inline] pub fn pccdummy70(&self) -> Pccdummy70 { 
     unsafe {
        Pccdummy70(::core::ptr::read_volatile(((self.0 as usize) + 0x118) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY70 register."]
  #[inline] pub fn set_pccdummy70(&self, value: Pccdummy70) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x118) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY70 register."]
  #[inline] pub fn with_pccdummy70<F: FnOnce(Pccdummy70) -> Pccdummy70>(&self, f: F) -> &Self {
     let tmp = self.pccdummy70();
     self.set_pccdummy70(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY71 register."]
  #[inline] pub fn pccdummy71_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x11c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY71 register."]
  #[inline] pub fn pccdummy71_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x11c) as *mut u32
  }
#[doc="Read the PCCDUMMY71 register."]
  #[inline] pub fn pccdummy71(&self) -> Pccdummy71 { 
     unsafe {
        Pccdummy71(::core::ptr::read_volatile(((self.0 as usize) + 0x11c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY71 register."]
  #[inline] pub fn set_pccdummy71(&self, value: Pccdummy71) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x11c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY71 register."]
  #[inline] pub fn with_pccdummy71<F: FnOnce(Pccdummy71) -> Pccdummy71>(&self, f: F) -> &Self {
     let tmp = self.pccdummy71();
     self.set_pccdummy71(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY72 register."]
  #[inline] pub fn pccdummy72_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x120) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY72 register."]
  #[inline] pub fn pccdummy72_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x120) as *mut u32
  }
#[doc="Read the PCCDUMMY72 register."]
  #[inline] pub fn pccdummy72(&self) -> Pccdummy72 { 
     unsafe {
        Pccdummy72(::core::ptr::read_volatile(((self.0 as usize) + 0x120) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY72 register."]
  #[inline] pub fn set_pccdummy72(&self, value: Pccdummy72) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x120) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY72 register."]
  #[inline] pub fn with_pccdummy72<F: FnOnce(Pccdummy72) -> Pccdummy72>(&self, f: F) -> &Self {
     let tmp = self.pccdummy72();
     self.set_pccdummy72(f(tmp))
  }

#[doc="Get the *const pointer for the PORTA register."]
  #[inline] pub fn porta_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x124) as *const u32
  }
#[doc="Get the *mut pointer for the PORTA register."]
  #[inline] pub fn porta_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x124) as *mut u32
  }
#[doc="Read the PORTA register."]
  #[inline] pub fn porta(&self) -> Porta { 
     unsafe {
        Porta(::core::ptr::read_volatile(((self.0 as usize) + 0x124) as *const u32))
     }
  }
#[doc="Write the PORTA register."]
  #[inline] pub fn set_porta(&self, value: Porta) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x124) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PORTA register."]
  #[inline] pub fn with_porta<F: FnOnce(Porta) -> Porta>(&self, f: F) -> &Self {
     let tmp = self.porta();
     self.set_porta(f(tmp))
  }

#[doc="Get the *const pointer for the PORTB register."]
  #[inline] pub fn portb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x128) as *const u32
  }
#[doc="Get the *mut pointer for the PORTB register."]
  #[inline] pub fn portb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x128) as *mut u32
  }
#[doc="Read the PORTB register."]
  #[inline] pub fn portb(&self) -> Portb { 
     unsafe {
        Portb(::core::ptr::read_volatile(((self.0 as usize) + 0x128) as *const u32))
     }
  }
#[doc="Write the PORTB register."]
  #[inline] pub fn set_portb(&self, value: Portb) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x128) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PORTB register."]
  #[inline] pub fn with_portb<F: FnOnce(Portb) -> Portb>(&self, f: F) -> &Self {
     let tmp = self.portb();
     self.set_portb(f(tmp))
  }

#[doc="Get the *const pointer for the PORTC register."]
  #[inline] pub fn portc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x12c) as *const u32
  }
#[doc="Get the *mut pointer for the PORTC register."]
  #[inline] pub fn portc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x12c) as *mut u32
  }
#[doc="Read the PORTC register."]
  #[inline] pub fn portc(&self) -> Portc { 
     unsafe {
        Portc(::core::ptr::read_volatile(((self.0 as usize) + 0x12c) as *const u32))
     }
  }
#[doc="Write the PORTC register."]
  #[inline] pub fn set_portc(&self, value: Portc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x12c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PORTC register."]
  #[inline] pub fn with_portc<F: FnOnce(Portc) -> Portc>(&self, f: F) -> &Self {
     let tmp = self.portc();
     self.set_portc(f(tmp))
  }

#[doc="Get the *const pointer for the PORTD register."]
  #[inline] pub fn portd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x130) as *const u32
  }
#[doc="Get the *mut pointer for the PORTD register."]
  #[inline] pub fn portd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x130) as *mut u32
  }
#[doc="Read the PORTD register."]
  #[inline] pub fn portd(&self) -> Portd { 
     unsafe {
        Portd(::core::ptr::read_volatile(((self.0 as usize) + 0x130) as *const u32))
     }
  }
#[doc="Write the PORTD register."]
  #[inline] pub fn set_portd(&self, value: Portd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x130) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PORTD register."]
  #[inline] pub fn with_portd<F: FnOnce(Portd) -> Portd>(&self, f: F) -> &Self {
     let tmp = self.portd();
     self.set_portd(f(tmp))
  }

#[doc="Get the *const pointer for the PORTE register."]
  #[inline] pub fn porte_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x134) as *const u32
  }
#[doc="Get the *mut pointer for the PORTE register."]
  #[inline] pub fn porte_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x134) as *mut u32
  }
#[doc="Read the PORTE register."]
  #[inline] pub fn porte(&self) -> Porte { 
     unsafe {
        Porte(::core::ptr::read_volatile(((self.0 as usize) + 0x134) as *const u32))
     }
  }
#[doc="Write the PORTE register."]
  #[inline] pub fn set_porte(&self, value: Porte) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x134) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PORTE register."]
  #[inline] pub fn with_porte<F: FnOnce(Porte) -> Porte>(&self, f: F) -> &Self {
     let tmp = self.porte();
     self.set_porte(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY78 register."]
  #[inline] pub fn pccdummy78_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x138) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY78 register."]
  #[inline] pub fn pccdummy78_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x138) as *mut u32
  }
#[doc="Read the PCCDUMMY78 register."]
  #[inline] pub fn pccdummy78(&self) -> Pccdummy78 { 
     unsafe {
        Pccdummy78(::core::ptr::read_volatile(((self.0 as usize) + 0x138) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY78 register."]
  #[inline] pub fn set_pccdummy78(&self, value: Pccdummy78) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x138) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY78 register."]
  #[inline] pub fn with_pccdummy78<F: FnOnce(Pccdummy78) -> Pccdummy78>(&self, f: F) -> &Self {
     let tmp = self.pccdummy78();
     self.set_pccdummy78(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY79 register."]
  #[inline] pub fn pccdummy79_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x13c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY79 register."]
  #[inline] pub fn pccdummy79_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x13c) as *mut u32
  }
#[doc="Read the PCCDUMMY79 register."]
  #[inline] pub fn pccdummy79(&self) -> Pccdummy79 { 
     unsafe {
        Pccdummy79(::core::ptr::read_volatile(((self.0 as usize) + 0x13c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY79 register."]
  #[inline] pub fn set_pccdummy79(&self, value: Pccdummy79) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x13c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY79 register."]
  #[inline] pub fn with_pccdummy79<F: FnOnce(Pccdummy79) -> Pccdummy79>(&self, f: F) -> &Self {
     let tmp = self.pccdummy79();
     self.set_pccdummy79(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY80 register."]
  #[inline] pub fn pccdummy80_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x140) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY80 register."]
  #[inline] pub fn pccdummy80_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x140) as *mut u32
  }
#[doc="Read the PCCDUMMY80 register."]
  #[inline] pub fn pccdummy80(&self) -> Pccdummy80 { 
     unsafe {
        Pccdummy80(::core::ptr::read_volatile(((self.0 as usize) + 0x140) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY80 register."]
  #[inline] pub fn set_pccdummy80(&self, value: Pccdummy80) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x140) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY80 register."]
  #[inline] pub fn with_pccdummy80<F: FnOnce(Pccdummy80) -> Pccdummy80>(&self, f: F) -> &Self {
     let tmp = self.pccdummy80();
     self.set_pccdummy80(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY81 register."]
  #[inline] pub fn pccdummy81_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x144) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY81 register."]
  #[inline] pub fn pccdummy81_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x144) as *mut u32
  }
#[doc="Read the PCCDUMMY81 register."]
  #[inline] pub fn pccdummy81(&self) -> Pccdummy81 { 
     unsafe {
        Pccdummy81(::core::ptr::read_volatile(((self.0 as usize) + 0x144) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY81 register."]
  #[inline] pub fn set_pccdummy81(&self, value: Pccdummy81) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x144) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY81 register."]
  #[inline] pub fn with_pccdummy81<F: FnOnce(Pccdummy81) -> Pccdummy81>(&self, f: F) -> &Self {
     let tmp = self.pccdummy81();
     self.set_pccdummy81(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY82 register."]
  #[inline] pub fn pccdummy82_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x148) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY82 register."]
  #[inline] pub fn pccdummy82_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x148) as *mut u32
  }
#[doc="Read the PCCDUMMY82 register."]
  #[inline] pub fn pccdummy82(&self) -> Pccdummy82 { 
     unsafe {
        Pccdummy82(::core::ptr::read_volatile(((self.0 as usize) + 0x148) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY82 register."]
  #[inline] pub fn set_pccdummy82(&self, value: Pccdummy82) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x148) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY82 register."]
  #[inline] pub fn with_pccdummy82<F: FnOnce(Pccdummy82) -> Pccdummy82>(&self, f: F) -> &Self {
     let tmp = self.pccdummy82();
     self.set_pccdummy82(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY83 register."]
  #[inline] pub fn pccdummy83_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY83 register."]
  #[inline] pub fn pccdummy83_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14c) as *mut u32
  }
#[doc="Read the PCCDUMMY83 register."]
  #[inline] pub fn pccdummy83(&self) -> Pccdummy83 { 
     unsafe {
        Pccdummy83(::core::ptr::read_volatile(((self.0 as usize) + 0x14c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY83 register."]
  #[inline] pub fn set_pccdummy83(&self, value: Pccdummy83) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY83 register."]
  #[inline] pub fn with_pccdummy83<F: FnOnce(Pccdummy83) -> Pccdummy83>(&self, f: F) -> &Self {
     let tmp = self.pccdummy83();
     self.set_pccdummy83(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY84 register."]
  #[inline] pub fn pccdummy84_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x150) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY84 register."]
  #[inline] pub fn pccdummy84_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x150) as *mut u32
  }
#[doc="Read the PCCDUMMY84 register."]
  #[inline] pub fn pccdummy84(&self) -> Pccdummy84 { 
     unsafe {
        Pccdummy84(::core::ptr::read_volatile(((self.0 as usize) + 0x150) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY84 register."]
  #[inline] pub fn set_pccdummy84(&self, value: Pccdummy84) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x150) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY84 register."]
  #[inline] pub fn with_pccdummy84<F: FnOnce(Pccdummy84) -> Pccdummy84>(&self, f: F) -> &Self {
     let tmp = self.pccdummy84();
     self.set_pccdummy84(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY85 register."]
  #[inline] pub fn pccdummy85_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x154) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY85 register."]
  #[inline] pub fn pccdummy85_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x154) as *mut u32
  }
#[doc="Read the PCCDUMMY85 register."]
  #[inline] pub fn pccdummy85(&self) -> Pccdummy85 { 
     unsafe {
        Pccdummy85(::core::ptr::read_volatile(((self.0 as usize) + 0x154) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY85 register."]
  #[inline] pub fn set_pccdummy85(&self, value: Pccdummy85) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x154) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY85 register."]
  #[inline] pub fn with_pccdummy85<F: FnOnce(Pccdummy85) -> Pccdummy85>(&self, f: F) -> &Self {
     let tmp = self.pccdummy85();
     self.set_pccdummy85(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY86 register."]
  #[inline] pub fn pccdummy86_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x158) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY86 register."]
  #[inline] pub fn pccdummy86_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x158) as *mut u32
  }
#[doc="Read the PCCDUMMY86 register."]
  #[inline] pub fn pccdummy86(&self) -> Pccdummy86 { 
     unsafe {
        Pccdummy86(::core::ptr::read_volatile(((self.0 as usize) + 0x158) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY86 register."]
  #[inline] pub fn set_pccdummy86(&self, value: Pccdummy86) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x158) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY86 register."]
  #[inline] pub fn with_pccdummy86<F: FnOnce(Pccdummy86) -> Pccdummy86>(&self, f: F) -> &Self {
     let tmp = self.pccdummy86();
     self.set_pccdummy86(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY87 register."]
  #[inline] pub fn pccdummy87_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x15c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY87 register."]
  #[inline] pub fn pccdummy87_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x15c) as *mut u32
  }
#[doc="Read the PCCDUMMY87 register."]
  #[inline] pub fn pccdummy87(&self) -> Pccdummy87 { 
     unsafe {
        Pccdummy87(::core::ptr::read_volatile(((self.0 as usize) + 0x15c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY87 register."]
  #[inline] pub fn set_pccdummy87(&self, value: Pccdummy87) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x15c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY87 register."]
  #[inline] pub fn with_pccdummy87<F: FnOnce(Pccdummy87) -> Pccdummy87>(&self, f: F) -> &Self {
     let tmp = self.pccdummy87();
     self.set_pccdummy87(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY88 register."]
  #[inline] pub fn pccdummy88_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x160) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY88 register."]
  #[inline] pub fn pccdummy88_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x160) as *mut u32
  }
#[doc="Read the PCCDUMMY88 register."]
  #[inline] pub fn pccdummy88(&self) -> Pccdummy88 { 
     unsafe {
        Pccdummy88(::core::ptr::read_volatile(((self.0 as usize) + 0x160) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY88 register."]
  #[inline] pub fn set_pccdummy88(&self, value: Pccdummy88) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x160) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY88 register."]
  #[inline] pub fn with_pccdummy88<F: FnOnce(Pccdummy88) -> Pccdummy88>(&self, f: F) -> &Self {
     let tmp = self.pccdummy88();
     self.set_pccdummy88(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY89 register."]
  #[inline] pub fn pccdummy89_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x164) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY89 register."]
  #[inline] pub fn pccdummy89_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x164) as *mut u32
  }
#[doc="Read the PCCDUMMY89 register."]
  #[inline] pub fn pccdummy89(&self) -> Pccdummy89 { 
     unsafe {
        Pccdummy89(::core::ptr::read_volatile(((self.0 as usize) + 0x164) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY89 register."]
  #[inline] pub fn set_pccdummy89(&self, value: Pccdummy89) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x164) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY89 register."]
  #[inline] pub fn with_pccdummy89<F: FnOnce(Pccdummy89) -> Pccdummy89>(&self, f: F) -> &Self {
     let tmp = self.pccdummy89();
     self.set_pccdummy89(f(tmp))
  }

#[doc="Get the *const pointer for the FLEXIO register."]
  #[inline] pub fn flexio_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x168) as *const u32
  }
#[doc="Get the *mut pointer for the FLEXIO register."]
  #[inline] pub fn flexio_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x168) as *mut u32
  }
#[doc="Read the FLEXIO register."]
  #[inline] pub fn flexio(&self) -> Flexio { 
     unsafe {
        Flexio(::core::ptr::read_volatile(((self.0 as usize) + 0x168) as *const u32))
     }
  }
#[doc="Write the FLEXIO register."]
  #[inline] pub fn set_flexio(&self, value: Flexio) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x168) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FLEXIO register."]
  #[inline] pub fn with_flexio<F: FnOnce(Flexio) -> Flexio>(&self, f: F) -> &Self {
     let tmp = self.flexio();
     self.set_flexio(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY91 register."]
  #[inline] pub fn pccdummy91_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x16c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY91 register."]
  #[inline] pub fn pccdummy91_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x16c) as *mut u32
  }
#[doc="Read the PCCDUMMY91 register."]
  #[inline] pub fn pccdummy91(&self) -> Pccdummy91 { 
     unsafe {
        Pccdummy91(::core::ptr::read_volatile(((self.0 as usize) + 0x16c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY91 register."]
  #[inline] pub fn set_pccdummy91(&self, value: Pccdummy91) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x16c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY91 register."]
  #[inline] pub fn with_pccdummy91<F: FnOnce(Pccdummy91) -> Pccdummy91>(&self, f: F) -> &Self {
     let tmp = self.pccdummy91();
     self.set_pccdummy91(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY92 register."]
  #[inline] pub fn pccdummy92_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x170) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY92 register."]
  #[inline] pub fn pccdummy92_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x170) as *mut u32
  }
#[doc="Read the PCCDUMMY92 register."]
  #[inline] pub fn pccdummy92(&self) -> Pccdummy92 { 
     unsafe {
        Pccdummy92(::core::ptr::read_volatile(((self.0 as usize) + 0x170) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY92 register."]
  #[inline] pub fn set_pccdummy92(&self, value: Pccdummy92) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x170) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY92 register."]
  #[inline] pub fn with_pccdummy92<F: FnOnce(Pccdummy92) -> Pccdummy92>(&self, f: F) -> &Self {
     let tmp = self.pccdummy92();
     self.set_pccdummy92(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY93 register."]
  #[inline] pub fn pccdummy93_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x174) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY93 register."]
  #[inline] pub fn pccdummy93_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x174) as *mut u32
  }
#[doc="Read the PCCDUMMY93 register."]
  #[inline] pub fn pccdummy93(&self) -> Pccdummy93 { 
     unsafe {
        Pccdummy93(::core::ptr::read_volatile(((self.0 as usize) + 0x174) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY93 register."]
  #[inline] pub fn set_pccdummy93(&self, value: Pccdummy93) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x174) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY93 register."]
  #[inline] pub fn with_pccdummy93<F: FnOnce(Pccdummy93) -> Pccdummy93>(&self, f: F) -> &Self {
     let tmp = self.pccdummy93();
     self.set_pccdummy93(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY94 register."]
  #[inline] pub fn pccdummy94_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x178) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY94 register."]
  #[inline] pub fn pccdummy94_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x178) as *mut u32
  }
#[doc="Read the PCCDUMMY94 register."]
  #[inline] pub fn pccdummy94(&self) -> Pccdummy94 { 
     unsafe {
        Pccdummy94(::core::ptr::read_volatile(((self.0 as usize) + 0x178) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY94 register."]
  #[inline] pub fn set_pccdummy94(&self, value: Pccdummy94) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x178) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY94 register."]
  #[inline] pub fn with_pccdummy94<F: FnOnce(Pccdummy94) -> Pccdummy94>(&self, f: F) -> &Self {
     let tmp = self.pccdummy94();
     self.set_pccdummy94(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY95 register."]
  #[inline] pub fn pccdummy95_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x17c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY95 register."]
  #[inline] pub fn pccdummy95_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x17c) as *mut u32
  }
#[doc="Read the PCCDUMMY95 register."]
  #[inline] pub fn pccdummy95(&self) -> Pccdummy95 { 
     unsafe {
        Pccdummy95(::core::ptr::read_volatile(((self.0 as usize) + 0x17c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY95 register."]
  #[inline] pub fn set_pccdummy95(&self, value: Pccdummy95) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x17c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY95 register."]
  #[inline] pub fn with_pccdummy95<F: FnOnce(Pccdummy95) -> Pccdummy95>(&self, f: F) -> &Self {
     let tmp = self.pccdummy95();
     self.set_pccdummy95(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY96 register."]
  #[inline] pub fn pccdummy96_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x180) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY96 register."]
  #[inline] pub fn pccdummy96_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x180) as *mut u32
  }
#[doc="Read the PCCDUMMY96 register."]
  #[inline] pub fn pccdummy96(&self) -> Pccdummy96 { 
     unsafe {
        Pccdummy96(::core::ptr::read_volatile(((self.0 as usize) + 0x180) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY96 register."]
  #[inline] pub fn set_pccdummy96(&self, value: Pccdummy96) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x180) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY96 register."]
  #[inline] pub fn with_pccdummy96<F: FnOnce(Pccdummy96) -> Pccdummy96>(&self, f: F) -> &Self {
     let tmp = self.pccdummy96();
     self.set_pccdummy96(f(tmp))
  }

#[doc="Get the *const pointer for the EWM register."]
  #[inline] pub fn ewm_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x184) as *const u32
  }
#[doc="Get the *mut pointer for the EWM register."]
  #[inline] pub fn ewm_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x184) as *mut u32
  }
#[doc="Read the EWM register."]
  #[inline] pub fn ewm(&self) -> Ewm { 
     unsafe {
        Ewm(::core::ptr::read_volatile(((self.0 as usize) + 0x184) as *const u32))
     }
  }
#[doc="Write the EWM register."]
  #[inline] pub fn set_ewm(&self, value: Ewm) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x184) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the EWM register."]
  #[inline] pub fn with_ewm<F: FnOnce(Ewm) -> Ewm>(&self, f: F) -> &Self {
     let tmp = self.ewm();
     self.set_ewm(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY98 register."]
  #[inline] pub fn pccdummy98_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x188) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY98 register."]
  #[inline] pub fn pccdummy98_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x188) as *mut u32
  }
#[doc="Read the PCCDUMMY98 register."]
  #[inline] pub fn pccdummy98(&self) -> Pccdummy98 { 
     unsafe {
        Pccdummy98(::core::ptr::read_volatile(((self.0 as usize) + 0x188) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY98 register."]
  #[inline] pub fn set_pccdummy98(&self, value: Pccdummy98) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x188) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY98 register."]
  #[inline] pub fn with_pccdummy98<F: FnOnce(Pccdummy98) -> Pccdummy98>(&self, f: F) -> &Self {
     let tmp = self.pccdummy98();
     self.set_pccdummy98(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY99 register."]
  #[inline] pub fn pccdummy99_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY99 register."]
  #[inline] pub fn pccdummy99_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18c) as *mut u32
  }
#[doc="Read the PCCDUMMY99 register."]
  #[inline] pub fn pccdummy99(&self) -> Pccdummy99 { 
     unsafe {
        Pccdummy99(::core::ptr::read_volatile(((self.0 as usize) + 0x18c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY99 register."]
  #[inline] pub fn set_pccdummy99(&self, value: Pccdummy99) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY99 register."]
  #[inline] pub fn with_pccdummy99<F: FnOnce(Pccdummy99) -> Pccdummy99>(&self, f: F) -> &Self {
     let tmp = self.pccdummy99();
     self.set_pccdummy99(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY100 register."]
  #[inline] pub fn pccdummy100_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x190) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY100 register."]
  #[inline] pub fn pccdummy100_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x190) as *mut u32
  }
#[doc="Read the PCCDUMMY100 register."]
  #[inline] pub fn pccdummy100(&self) -> Pccdummy100 { 
     unsafe {
        Pccdummy100(::core::ptr::read_volatile(((self.0 as usize) + 0x190) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY100 register."]
  #[inline] pub fn set_pccdummy100(&self, value: Pccdummy100) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x190) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY100 register."]
  #[inline] pub fn with_pccdummy100<F: FnOnce(Pccdummy100) -> Pccdummy100>(&self, f: F) -> &Self {
     let tmp = self.pccdummy100();
     self.set_pccdummy100(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY101 register."]
  #[inline] pub fn pccdummy101_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x194) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY101 register."]
  #[inline] pub fn pccdummy101_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x194) as *mut u32
  }
#[doc="Read the PCCDUMMY101 register."]
  #[inline] pub fn pccdummy101(&self) -> Pccdummy101 { 
     unsafe {
        Pccdummy101(::core::ptr::read_volatile(((self.0 as usize) + 0x194) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY101 register."]
  #[inline] pub fn set_pccdummy101(&self, value: Pccdummy101) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x194) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY101 register."]
  #[inline] pub fn with_pccdummy101<F: FnOnce(Pccdummy101) -> Pccdummy101>(&self, f: F) -> &Self {
     let tmp = self.pccdummy101();
     self.set_pccdummy101(f(tmp))
  }

#[doc="Get the *const pointer for the LPI2C0 register."]
  #[inline] pub fn lpi2c0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x198) as *const u32
  }
#[doc="Get the *mut pointer for the LPI2C0 register."]
  #[inline] pub fn lpi2c0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x198) as *mut u32
  }
#[doc="Read the LPI2C0 register."]
  #[inline] pub fn lpi2c0(&self) -> Lpi2c0 { 
     unsafe {
        Lpi2c0(::core::ptr::read_volatile(((self.0 as usize) + 0x198) as *const u32))
     }
  }
#[doc="Write the LPI2C0 register."]
  #[inline] pub fn set_lpi2c0(&self, value: Lpi2c0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x198) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LPI2C0 register."]
  #[inline] pub fn with_lpi2c0<F: FnOnce(Lpi2c0) -> Lpi2c0>(&self, f: F) -> &Self {
     let tmp = self.lpi2c0();
     self.set_lpi2c0(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY103 register."]
  #[inline] pub fn pccdummy103_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x19c) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY103 register."]
  #[inline] pub fn pccdummy103_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x19c) as *mut u32
  }
#[doc="Read the PCCDUMMY103 register."]
  #[inline] pub fn pccdummy103(&self) -> Pccdummy103 { 
     unsafe {
        Pccdummy103(::core::ptr::read_volatile(((self.0 as usize) + 0x19c) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY103 register."]
  #[inline] pub fn set_pccdummy103(&self, value: Pccdummy103) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x19c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY103 register."]
  #[inline] pub fn with_pccdummy103<F: FnOnce(Pccdummy103) -> Pccdummy103>(&self, f: F) -> &Self {
     let tmp = self.pccdummy103();
     self.set_pccdummy103(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY104 register."]
  #[inline] pub fn pccdummy104_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1a0) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY104 register."]
  #[inline] pub fn pccdummy104_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1a0) as *mut u32
  }
#[doc="Read the PCCDUMMY104 register."]
  #[inline] pub fn pccdummy104(&self) -> Pccdummy104 { 
     unsafe {
        Pccdummy104(::core::ptr::read_volatile(((self.0 as usize) + 0x1a0) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY104 register."]
  #[inline] pub fn set_pccdummy104(&self, value: Pccdummy104) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY104 register."]
  #[inline] pub fn with_pccdummy104<F: FnOnce(Pccdummy104) -> Pccdummy104>(&self, f: F) -> &Self {
     let tmp = self.pccdummy104();
     self.set_pccdummy104(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY105 register."]
  #[inline] pub fn pccdummy105_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1a4) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY105 register."]
  #[inline] pub fn pccdummy105_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1a4) as *mut u32
  }
#[doc="Read the PCCDUMMY105 register."]
  #[inline] pub fn pccdummy105(&self) -> Pccdummy105 { 
     unsafe {
        Pccdummy105(::core::ptr::read_volatile(((self.0 as usize) + 0x1a4) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY105 register."]
  #[inline] pub fn set_pccdummy105(&self, value: Pccdummy105) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY105 register."]
  #[inline] pub fn with_pccdummy105<F: FnOnce(Pccdummy105) -> Pccdummy105>(&self, f: F) -> &Self {
     let tmp = self.pccdummy105();
     self.set_pccdummy105(f(tmp))
  }

#[doc="Get the *const pointer for the LPUART0 register."]
  #[inline] pub fn lpuart0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1a8) as *const u32
  }
#[doc="Get the *mut pointer for the LPUART0 register."]
  #[inline] pub fn lpuart0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1a8) as *mut u32
  }
#[doc="Read the LPUART0 register."]
  #[inline] pub fn lpuart0(&self) -> Lpuart0 { 
     unsafe {
        Lpuart0(::core::ptr::read_volatile(((self.0 as usize) + 0x1a8) as *const u32))
     }
  }
#[doc="Write the LPUART0 register."]
  #[inline] pub fn set_lpuart0(&self, value: Lpuart0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LPUART0 register."]
  #[inline] pub fn with_lpuart0<F: FnOnce(Lpuart0) -> Lpuart0>(&self, f: F) -> &Self {
     let tmp = self.lpuart0();
     self.set_lpuart0(f(tmp))
  }

#[doc="Get the *const pointer for the LPUART1 register."]
  #[inline] pub fn lpuart1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1ac) as *const u32
  }
#[doc="Get the *mut pointer for the LPUART1 register."]
  #[inline] pub fn lpuart1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1ac) as *mut u32
  }
#[doc="Read the LPUART1 register."]
  #[inline] pub fn lpuart1(&self) -> Lpuart1 { 
     unsafe {
        Lpuart1(::core::ptr::read_volatile(((self.0 as usize) + 0x1ac) as *const u32))
     }
  }
#[doc="Write the LPUART1 register."]
  #[inline] pub fn set_lpuart1(&self, value: Lpuart1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1ac) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LPUART1 register."]
  #[inline] pub fn with_lpuart1<F: FnOnce(Lpuart1) -> Lpuart1>(&self, f: F) -> &Self {
     let tmp = self.lpuart1();
     self.set_lpuart1(f(tmp))
  }

#[doc="Get the *const pointer for the LPUART2 register."]
  #[inline] pub fn lpuart2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1b0) as *const u32
  }
#[doc="Get the *mut pointer for the LPUART2 register."]
  #[inline] pub fn lpuart2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1b0) as *mut u32
  }
#[doc="Read the LPUART2 register."]
  #[inline] pub fn lpuart2(&self) -> Lpuart2 { 
     unsafe {
        Lpuart2(::core::ptr::read_volatile(((self.0 as usize) + 0x1b0) as *const u32))
     }
  }
#[doc="Write the LPUART2 register."]
  #[inline] pub fn set_lpuart2(&self, value: Lpuart2) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1b0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the LPUART2 register."]
  #[inline] pub fn with_lpuart2<F: FnOnce(Lpuart2) -> Lpuart2>(&self, f: F) -> &Self {
     let tmp = self.lpuart2();
     self.set_lpuart2(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY109 register."]
  #[inline] pub fn pccdummy109_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1b4) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY109 register."]
  #[inline] pub fn pccdummy109_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1b4) as *mut u32
  }
#[doc="Read the PCCDUMMY109 register."]
  #[inline] pub fn pccdummy109(&self) -> Pccdummy109 { 
     unsafe {
        Pccdummy109(::core::ptr::read_volatile(((self.0 as usize) + 0x1b4) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY109 register."]
  #[inline] pub fn set_pccdummy109(&self, value: Pccdummy109) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1b4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY109 register."]
  #[inline] pub fn with_pccdummy109<F: FnOnce(Pccdummy109) -> Pccdummy109>(&self, f: F) -> &Self {
     let tmp = self.pccdummy109();
     self.set_pccdummy109(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY110 register."]
  #[inline] pub fn pccdummy110_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1b8) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY110 register."]
  #[inline] pub fn pccdummy110_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1b8) as *mut u32
  }
#[doc="Read the PCCDUMMY110 register."]
  #[inline] pub fn pccdummy110(&self) -> Pccdummy110 { 
     unsafe {
        Pccdummy110(::core::ptr::read_volatile(((self.0 as usize) + 0x1b8) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY110 register."]
  #[inline] pub fn set_pccdummy110(&self, value: Pccdummy110) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1b8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY110 register."]
  #[inline] pub fn with_pccdummy110<F: FnOnce(Pccdummy110) -> Pccdummy110>(&self, f: F) -> &Self {
     let tmp = self.pccdummy110();
     self.set_pccdummy110(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY111 register."]
  #[inline] pub fn pccdummy111_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1bc) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY111 register."]
  #[inline] pub fn pccdummy111_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1bc) as *mut u32
  }
#[doc="Read the PCCDUMMY111 register."]
  #[inline] pub fn pccdummy111(&self) -> Pccdummy111 { 
     unsafe {
        Pccdummy111(::core::ptr::read_volatile(((self.0 as usize) + 0x1bc) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY111 register."]
  #[inline] pub fn set_pccdummy111(&self, value: Pccdummy111) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1bc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY111 register."]
  #[inline] pub fn with_pccdummy111<F: FnOnce(Pccdummy111) -> Pccdummy111>(&self, f: F) -> &Self {
     let tmp = self.pccdummy111();
     self.set_pccdummy111(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY112 register."]
  #[inline] pub fn pccdummy112_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c0) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY112 register."]
  #[inline] pub fn pccdummy112_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c0) as *mut u32
  }
#[doc="Read the PCCDUMMY112 register."]
  #[inline] pub fn pccdummy112(&self) -> Pccdummy112 { 
     unsafe {
        Pccdummy112(::core::ptr::read_volatile(((self.0 as usize) + 0x1c0) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY112 register."]
  #[inline] pub fn set_pccdummy112(&self, value: Pccdummy112) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY112 register."]
  #[inline] pub fn with_pccdummy112<F: FnOnce(Pccdummy112) -> Pccdummy112>(&self, f: F) -> &Self {
     let tmp = self.pccdummy112();
     self.set_pccdummy112(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY113 register."]
  #[inline] pub fn pccdummy113_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c4) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY113 register."]
  #[inline] pub fn pccdummy113_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c4) as *mut u32
  }
#[doc="Read the PCCDUMMY113 register."]
  #[inline] pub fn pccdummy113(&self) -> Pccdummy113 { 
     unsafe {
        Pccdummy113(::core::ptr::read_volatile(((self.0 as usize) + 0x1c4) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY113 register."]
  #[inline] pub fn set_pccdummy113(&self, value: Pccdummy113) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY113 register."]
  #[inline] pub fn with_pccdummy113<F: FnOnce(Pccdummy113) -> Pccdummy113>(&self, f: F) -> &Self {
     let tmp = self.pccdummy113();
     self.set_pccdummy113(f(tmp))
  }

#[doc="Get the *const pointer for the PCCDUMMY114 register."]
  #[inline] pub fn pccdummy114_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c8) as *const u32
  }
#[doc="Get the *mut pointer for the PCCDUMMY114 register."]
  #[inline] pub fn pccdummy114_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c8) as *mut u32
  }
#[doc="Read the PCCDUMMY114 register."]
  #[inline] pub fn pccdummy114(&self) -> Pccdummy114 { 
     unsafe {
        Pccdummy114(::core::ptr::read_volatile(((self.0 as usize) + 0x1c8) as *const u32))
     }
  }
#[doc="Write the PCCDUMMY114 register."]
  #[inline] pub fn set_pccdummy114(&self, value: Pccdummy114) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PCCDUMMY114 register."]
  #[inline] pub fn with_pccdummy114<F: FnOnce(Pccdummy114) -> Pccdummy114>(&self, f: F) -> &Self {
     let tmp = self.pccdummy114();
     self.set_pccdummy114(f(tmp))
  }

#[doc="Get the *const pointer for the CMP0 register."]
  #[inline] pub fn cmp0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1cc) as *const u32
  }
#[doc="Get the *mut pointer for the CMP0 register."]
  #[inline] pub fn cmp0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1cc) as *mut u32
  }
#[doc="Read the CMP0 register."]
  #[inline] pub fn cmp0(&self) -> Cmp0 { 
     unsafe {
        Cmp0(::core::ptr::read_volatile(((self.0 as usize) + 0x1cc) as *const u32))
     }
  }
#[doc="Write the CMP0 register."]
  #[inline] pub fn set_cmp0(&self, value: Cmp0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1cc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CMP0 register."]
  #[inline] pub fn with_cmp0<F: FnOnce(Cmp0) -> Cmp0>(&self, f: F) -> &Self {
     let tmp = self.cmp0();
     self.set_cmp0(f(tmp))
  }

}

#[doc="PCC Reserved Register 0"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy0(pub u32);
impl Pccdummy0 {
}
impl ::core::fmt::Display for Pccdummy0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 1"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy1(pub u32);
impl Pccdummy1 {
}
impl ::core::fmt::Display for Pccdummy1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 2"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy2(pub u32);
impl Pccdummy2 {
}
impl ::core::fmt::Display for Pccdummy2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 3"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy3(pub u32);
impl Pccdummy3 {
}
impl ::core::fmt::Display for Pccdummy3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 4"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy4(pub u32);
impl Pccdummy4 {
}
impl ::core::fmt::Display for Pccdummy4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 5"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy5(pub u32);
impl Pccdummy5 {
}
impl ::core::fmt::Display for Pccdummy5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 6"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy6(pub u32);
impl Pccdummy6 {
}
impl ::core::fmt::Display for Pccdummy6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 7"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy7(pub u32);
impl Pccdummy7 {
}
impl ::core::fmt::Display for Pccdummy7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 8"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy8(pub u32);
impl Pccdummy8 {
}
impl ::core::fmt::Display for Pccdummy8 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy8 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 9"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy9(pub u32);
impl Pccdummy9 {
}
impl ::core::fmt::Display for Pccdummy9 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy9 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 10"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy10(pub u32);
impl Pccdummy10 {
}
impl ::core::fmt::Display for Pccdummy10 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy10 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 11"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy11(pub u32);
impl Pccdummy11 {
}
impl ::core::fmt::Display for Pccdummy11 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy11 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 12"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy12(pub u32);
impl Pccdummy12 {
}
impl ::core::fmt::Display for Pccdummy12 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy12 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 13"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy13(pub u32);
impl Pccdummy13 {
}
impl ::core::fmt::Display for Pccdummy13 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy13 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 14"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy14(pub u32);
impl Pccdummy14 {
}
impl ::core::fmt::Display for Pccdummy14 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy14 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 15"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy15(pub u32);
impl Pccdummy15 {
}
impl ::core::fmt::Display for Pccdummy15 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy15 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 16"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy16(pub u32);
impl Pccdummy16 {
}
impl ::core::fmt::Display for Pccdummy16 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy16 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 17"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy17(pub u32);
impl Pccdummy17 {
}
impl ::core::fmt::Display for Pccdummy17 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy17 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 18"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy18(pub u32);
impl Pccdummy18 {
}
impl ::core::fmt::Display for Pccdummy18 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy18 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 19"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy19(pub u32);
impl Pccdummy19 {
}
impl ::core::fmt::Display for Pccdummy19 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy19 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 20"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy20(pub u32);
impl Pccdummy20 {
}
impl ::core::fmt::Display for Pccdummy20 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy20 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 21"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy21(pub u32);
impl Pccdummy21 {
}
impl ::core::fmt::Display for Pccdummy21 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy21 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 22"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy22(pub u32);
impl Pccdummy22 {
}
impl ::core::fmt::Display for Pccdummy22 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy22 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 23"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy23(pub u32);
impl Pccdummy23 {
}
impl ::core::fmt::Display for Pccdummy23 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy23 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 24"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy24(pub u32);
impl Pccdummy24 {
}
impl ::core::fmt::Display for Pccdummy24 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy24 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 25"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy25(pub u32);
impl Pccdummy25 {
}
impl ::core::fmt::Display for Pccdummy25 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy25 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 26"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy26(pub u32);
impl Pccdummy26 {
}
impl ::core::fmt::Display for Pccdummy26 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy26 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 27"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy27(pub u32);
impl Pccdummy27 {
}
impl ::core::fmt::Display for Pccdummy27 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy27 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 28"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy28(pub u32);
impl Pccdummy28 {
}
impl ::core::fmt::Display for Pccdummy28 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy28 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 29"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy29(pub u32);
impl Pccdummy29 {
}
impl ::core::fmt::Display for Pccdummy29 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy29 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 30"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy30(pub u32);
impl Pccdummy30 {
}
impl ::core::fmt::Display for Pccdummy30 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy30 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 31"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy31(pub u32);
impl Pccdummy31 {
}
impl ::core::fmt::Display for Pccdummy31 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy31 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC FTFC Register"]
#[derive(PartialEq, Eq)]
pub struct Ftfc(pub u32);
impl Ftfc {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Ftfc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ftfc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC DMAMUX Register"]
#[derive(PartialEq, Eq)]
pub struct Dmamux(pub u32);
impl Dmamux {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Dmamux {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmamux {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 34"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy34(pub u32);
impl Pccdummy34 {
}
impl ::core::fmt::Display for Pccdummy34 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy34 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 35"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy35(pub u32);
impl Pccdummy35 {
}
impl ::core::fmt::Display for Pccdummy35 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy35 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC FlexCAN0 Register"]
#[derive(PartialEq, Eq)]
pub struct Flexcan0(pub u32);
impl Flexcan0 {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Flexcan0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Flexcan0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC FlexCAN1 Register"]
#[derive(PartialEq, Eq)]
pub struct Flexcan1(pub u32);
impl Flexcan1 {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Flexcan1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Flexcan1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC FTM3 Register"]
#[derive(PartialEq, Eq)]
pub struct Ftm3(pub u32);
impl Ftm3 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Ftm3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ftm3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC ADC1 Register"]
#[derive(PartialEq, Eq)]
pub struct Adc1(pub u32);
impl Adc1 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Adc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Adc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 40"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy40(pub u32);
impl Pccdummy40 {
}
impl ::core::fmt::Display for Pccdummy40 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy40 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 41"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy41(pub u32);
impl Pccdummy41 {
}
impl ::core::fmt::Display for Pccdummy41 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy41 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 42"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy42(pub u32);
impl Pccdummy42 {
}
impl ::core::fmt::Display for Pccdummy42 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy42 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC FlexCAN2 Register"]
#[derive(PartialEq, Eq)]
pub struct Flexcan2(pub u32);
impl Flexcan2 {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Flexcan2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Flexcan2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC LPSPI0 Register"]
#[derive(PartialEq, Eq)]
pub struct Lpspi0(pub u32);
impl Lpspi0 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Lpspi0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lpspi0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC LPSPI1 Register"]
#[derive(PartialEq, Eq)]
pub struct Lpspi1(pub u32);
impl Lpspi1 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Lpspi1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lpspi1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC LPSPI2 Register"]
#[derive(PartialEq, Eq)]
pub struct Lpspi2(pub u32);
impl Lpspi2 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Lpspi2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lpspi2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 47"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy47(pub u32);
impl Pccdummy47 {
}
impl ::core::fmt::Display for Pccdummy47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy47 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 48"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy48(pub u32);
impl Pccdummy48 {
}
impl ::core::fmt::Display for Pccdummy48 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy48 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC PDB1 Register"]
#[derive(PartialEq, Eq)]
pub struct Pdb1(pub u32);
impl Pdb1 {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Pdb1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pdb1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC CRC Register"]
#[derive(PartialEq, Eq)]
pub struct Crc(pub u32);
impl Crc {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Crc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 51"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy51(pub u32);
impl Pccdummy51 {
}
impl ::core::fmt::Display for Pccdummy51 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy51 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 52"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy52(pub u32);
impl Pccdummy52 {
}
impl ::core::fmt::Display for Pccdummy52 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy52 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 53"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy53(pub u32);
impl Pccdummy53 {
}
impl ::core::fmt::Display for Pccdummy53 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy53 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC PDB0 Register"]
#[derive(PartialEq, Eq)]
pub struct Pdb0(pub u32);
impl Pdb0 {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Pdb0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pdb0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC LPIT Register"]
#[derive(PartialEq, Eq)]
pub struct Lpit(pub u32);
impl Lpit {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Lpit {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lpit {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC FTM0 Register"]
#[derive(PartialEq, Eq)]
pub struct Ftm0(pub u32);
impl Ftm0 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Ftm0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ftm0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC FTM1 Register"]
#[derive(PartialEq, Eq)]
pub struct Ftm1(pub u32);
impl Ftm1 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Ftm1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ftm1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC FTM2 Register"]
#[derive(PartialEq, Eq)]
pub struct Ftm2(pub u32);
impl Ftm2 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Ftm2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ftm2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC ADC0 Register"]
#[derive(PartialEq, Eq)]
pub struct Adc0(pub u32);
impl Adc0 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Adc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Adc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 60"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy60(pub u32);
impl Pccdummy60 {
}
impl ::core::fmt::Display for Pccdummy60 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy60 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC RTC Register"]
#[derive(PartialEq, Eq)]
pub struct Rtc(pub u32);
impl Rtc {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Rtc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rtc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 62"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy62(pub u32);
impl Pccdummy62 {
}
impl ::core::fmt::Display for Pccdummy62 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy62 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 63"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy63(pub u32);
impl Pccdummy63 {
}
impl ::core::fmt::Display for Pccdummy63 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy63 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC LPTMR0 Register"]
#[derive(PartialEq, Eq)]
pub struct Lptmr0(pub u32);
impl Lptmr0 {
#[doc="Peripheral Clock Divider Select"]
  #[inline] pub fn pcd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
#[doc="Peripheral Clock Divider Select"]
  #[inline] pub fn set_pcd(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Peripheral Clock Divider Fraction"]
  #[inline] pub fn frac(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Peripheral Clock Divider Fraction"]
  #[inline] pub fn set_frac(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Lptmr0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lptmr0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcd() != 0 { try!(write!(f, " pcd=0x{:x}", self.pcd()))}
      if self.frac() != 0 { try!(write!(f, " frac"))}
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 65"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy65(pub u32);
impl Pccdummy65 {
}
impl ::core::fmt::Display for Pccdummy65 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy65 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 66"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy66(pub u32);
impl Pccdummy66 {
}
impl ::core::fmt::Display for Pccdummy66 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy66 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 67"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy67(pub u32);
impl Pccdummy67 {
}
impl ::core::fmt::Display for Pccdummy67 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy67 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 68"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy68(pub u32);
impl Pccdummy68 {
}
impl ::core::fmt::Display for Pccdummy68 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy68 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 69"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy69(pub u32);
impl Pccdummy69 {
}
impl ::core::fmt::Display for Pccdummy69 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy69 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 70"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy70(pub u32);
impl Pccdummy70 {
}
impl ::core::fmt::Display for Pccdummy70 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy70 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 71"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy71(pub u32);
impl Pccdummy71 {
}
impl ::core::fmt::Display for Pccdummy71 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy71 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 72"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy72(pub u32);
impl Pccdummy72 {
}
impl ::core::fmt::Display for Pccdummy72 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy72 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC PORTA Register"]
#[derive(PartialEq, Eq)]
pub struct Porta(pub u32);
impl Porta {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Porta {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Porta {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC PORTB Register"]
#[derive(PartialEq, Eq)]
pub struct Portb(pub u32);
impl Portb {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Portb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Portb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC PORTC Register"]
#[derive(PartialEq, Eq)]
pub struct Portc(pub u32);
impl Portc {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Portc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Portc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC PORTD Register"]
#[derive(PartialEq, Eq)]
pub struct Portd(pub u32);
impl Portd {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Portd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Portd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC PORTE Register"]
#[derive(PartialEq, Eq)]
pub struct Porte(pub u32);
impl Porte {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Porte {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Porte {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 78"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy78(pub u32);
impl Pccdummy78 {
}
impl ::core::fmt::Display for Pccdummy78 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy78 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 79"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy79(pub u32);
impl Pccdummy79 {
}
impl ::core::fmt::Display for Pccdummy79 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy79 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 80"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy80(pub u32);
impl Pccdummy80 {
}
impl ::core::fmt::Display for Pccdummy80 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy80 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 81"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy81(pub u32);
impl Pccdummy81 {
}
impl ::core::fmt::Display for Pccdummy81 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy81 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 82"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy82(pub u32);
impl Pccdummy82 {
}
impl ::core::fmt::Display for Pccdummy82 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy82 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 83"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy83(pub u32);
impl Pccdummy83 {
}
impl ::core::fmt::Display for Pccdummy83 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy83 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 84"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy84(pub u32);
impl Pccdummy84 {
}
impl ::core::fmt::Display for Pccdummy84 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy84 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 85"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy85(pub u32);
impl Pccdummy85 {
}
impl ::core::fmt::Display for Pccdummy85 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy85 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 86"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy86(pub u32);
impl Pccdummy86 {
}
impl ::core::fmt::Display for Pccdummy86 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy86 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 87"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy87(pub u32);
impl Pccdummy87 {
}
impl ::core::fmt::Display for Pccdummy87 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy87 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 88"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy88(pub u32);
impl Pccdummy88 {
}
impl ::core::fmt::Display for Pccdummy88 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy88 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 89"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy89(pub u32);
impl Pccdummy89 {
}
impl ::core::fmt::Display for Pccdummy89 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy89 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC FlexIO Register"]
#[derive(PartialEq, Eq)]
pub struct Flexio(pub u32);
impl Flexio {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Flexio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Flexio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 91"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy91(pub u32);
impl Pccdummy91 {
}
impl ::core::fmt::Display for Pccdummy91 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy91 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 92"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy92(pub u32);
impl Pccdummy92 {
}
impl ::core::fmt::Display for Pccdummy92 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy92 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 93"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy93(pub u32);
impl Pccdummy93 {
}
impl ::core::fmt::Display for Pccdummy93 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy93 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 94"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy94(pub u32);
impl Pccdummy94 {
}
impl ::core::fmt::Display for Pccdummy94 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy94 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 95"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy95(pub u32);
impl Pccdummy95 {
}
impl ::core::fmt::Display for Pccdummy95 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy95 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 96"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy96(pub u32);
impl Pccdummy96 {
}
impl ::core::fmt::Display for Pccdummy96 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy96 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC EWM Register"]
#[derive(PartialEq, Eq)]
pub struct Ewm(pub u32);
impl Ewm {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Ewm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ewm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 98"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy98(pub u32);
impl Pccdummy98 {
}
impl ::core::fmt::Display for Pccdummy98 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy98 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 99"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy99(pub u32);
impl Pccdummy99 {
}
impl ::core::fmt::Display for Pccdummy99 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy99 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 100"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy100(pub u32);
impl Pccdummy100 {
}
impl ::core::fmt::Display for Pccdummy100 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy100 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 101"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy101(pub u32);
impl Pccdummy101 {
}
impl ::core::fmt::Display for Pccdummy101 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy101 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC LPI2C0 Register"]
#[derive(PartialEq, Eq)]
pub struct Lpi2c0(pub u32);
impl Lpi2c0 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Lpi2c0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lpi2c0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 103"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy103(pub u32);
impl Pccdummy103 {
}
impl ::core::fmt::Display for Pccdummy103 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy103 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 104"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy104(pub u32);
impl Pccdummy104 {
}
impl ::core::fmt::Display for Pccdummy104 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy104 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 105"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy105(pub u32);
impl Pccdummy105 {
}
impl ::core::fmt::Display for Pccdummy105 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy105 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC LPUART0 Register"]
#[derive(PartialEq, Eq)]
pub struct Lpuart0(pub u32);
impl Lpuart0 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Lpuart0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lpuart0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC LPUART1 Register"]
#[derive(PartialEq, Eq)]
pub struct Lpuart1(pub u32);
impl Lpuart1 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Lpuart1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lpuart1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC LPUART2 Register"]
#[derive(PartialEq, Eq)]
pub struct Lpuart2(pub u32);
impl Lpuart2 {
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
#[doc="Peripheral Clock Source Select"]
  #[inline] pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Lpuart2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lpuart2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 109"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy109(pub u32);
impl Pccdummy109 {
}
impl ::core::fmt::Display for Pccdummy109 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy109 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 110"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy110(pub u32);
impl Pccdummy110 {
}
impl ::core::fmt::Display for Pccdummy110 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy110 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 111"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy111(pub u32);
impl Pccdummy111 {
}
impl ::core::fmt::Display for Pccdummy111 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy111 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 112"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy112(pub u32);
impl Pccdummy112 {
}
impl ::core::fmt::Display for Pccdummy112 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy112 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 113"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy113(pub u32);
impl Pccdummy113 {
}
impl ::core::fmt::Display for Pccdummy113 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy113 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC Reserved Register 114"]
#[derive(PartialEq, Eq)]
pub struct Pccdummy114(pub u32);
impl Pccdummy114 {
}
impl ::core::fmt::Display for Pccdummy114 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccdummy114 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PCC CMP0 Register"]
#[derive(PartialEq, Eq)]
pub struct Cmp0(pub u32);
impl Cmp0 {
#[doc="Clock Gate Control"]
  #[inline] pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Clock Gate Control"]
  #[inline] pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Present"]
  #[inline] pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Present"]
  #[inline] pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Cmp0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cmp0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgc() != 0 { try!(write!(f, " cgc"))}
      if self.pr() != 0 { try!(write!(f, " pr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait Cgc {
   fn cgc(&self) -> u32;
   fn set_cgc(&self, value: u32);
}

impl Pcc {
   #[inline] pub fn cgc<P: Cgc>(&self, p: &P) -> u32 {
      p.cgc()
   }
   #[inline] pub fn set_cgc<P: Cgc>(&self, p: &P, value: u32) {
      p.set_cgc(value)
   }
}

pub trait Pr {
   fn pr(&self) -> u32;
   fn set_pr(&self, value: u32);
}

impl Pcc {
   #[inline] pub fn pr<P: Pr>(&self, p: &P) -> u32 {
      p.pr()
   }
   #[inline] pub fn set_pr<P: Pr>(&self, p: &P, value: u32) {
      p.set_pr(value)
   }
}

pub trait Pcs {
   fn pcs(&self) -> u32;
   fn set_pcs(&self, value: u32);
}

impl Pcc {
   #[inline] pub fn pcs<P: Pcs>(&self, p: &P) -> u32 {
      p.pcs()
   }
   #[inline] pub fn set_pcs<P: Pcs>(&self, p: &P, value: u32) {
      p.set_pcs(value)
   }
}

impl Cgc for super::flexcan::Can0 {
   #[inline] fn cgc(&self) -> u32 { PCC.flexcan0().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_flexcan0(|r| r.set_cgc(value)); }
}

impl Cgc for super::flexcan::Can1 {
   #[inline] fn cgc(&self) -> u32 { PCC.flexcan1().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_flexcan1(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm3 {
   #[inline] fn pcs(&self) -> u32 { PCC.ftm3().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm3(|r| r.set_pcs(value)); }
}

impl Cgc for super::flexcan::Can2 {
   #[inline] fn cgc(&self) -> u32 { PCC.flexcan2().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_flexcan2(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi0 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpspi0().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpspi0(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi0 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpspi0().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpspi0(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi1 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpspi1().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpspi1(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi1 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpspi1().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpspi1(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi2 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpspi2().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpspi2(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi2 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpspi2().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpspi2(|r| r.set_cgc(value)); }
}

impl Cgc for super::crc::Crc {
   #[inline] fn cgc(&self) -> u32 { PCC.crc().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_crc(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpit::Lpit0 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpit().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpit(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpit::Lpit0 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpit().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpit(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm0 {
   #[inline] fn pcs(&self) -> u32 { PCC.ftm0().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm0(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm0 {
   #[inline] fn cgc(&self) -> u32 { PCC.ftm0().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_ftm0(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm1 {
   #[inline] fn pcs(&self) -> u32 { PCC.ftm1().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm1(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm1 {
   #[inline] fn cgc(&self) -> u32 { PCC.ftm1().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_ftm1(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm2 {
   #[inline] fn pcs(&self) -> u32 { PCC.ftm2().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm2(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm2 {
   #[inline] fn cgc(&self) -> u32 { PCC.ftm2().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_ftm2(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Porta {
   #[inline] fn cgc(&self) -> u32 { PCC.porta().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_porta(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portb {
   #[inline] fn cgc(&self) -> u32 { PCC.portb().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_portb(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portc {
   #[inline] fn cgc(&self) -> u32 { PCC.portc().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_portc(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portd {
   #[inline] fn cgc(&self) -> u32 { PCC.portd().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_portd(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Porte {
   #[inline] fn cgc(&self) -> u32 { PCC.porte().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_porte(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart0 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpuart0().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpuart0(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart0 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpuart0().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpuart0(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart1 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpuart1().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpuart1(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart1 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpuart1().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpuart1(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart2 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpuart2().pcs() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpuart2(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart2 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpuart2().cgc() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpuart2(|r| r.set_cgc(value)); }
}


