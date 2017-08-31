//! PCC
#[allow(unused_imports)] use bobbin_common::*;

periph!(PCC, Pcc, 0x40065000);

#[doc="PCC"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcc(pub usize);
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
         Pccdummy0(read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY0 register."]
   #[inline] pub fn set_pccdummy0<F: FnOnce(Pccdummy0) -> Pccdummy0>(&self, f: F) -> &Self {
      let value = f(Pccdummy0(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY0 register."]
   #[inline] pub fn with_pccdummy0<F: FnOnce(Pccdummy0) -> Pccdummy0>(&self, f: F) -> &Self {
      let tmp = self.pccdummy0();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
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
         Pccdummy1(read_volatile((self.0 + 0x4) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY1 register."]
   #[inline] pub fn set_pccdummy1<F: FnOnce(Pccdummy1) -> Pccdummy1>(&self, f: F) -> &Self {
      let value = f(Pccdummy1(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY1 register."]
   #[inline] pub fn with_pccdummy1<F: FnOnce(Pccdummy1) -> Pccdummy1>(&self, f: F) -> &Self {
      let tmp = self.pccdummy1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
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
         Pccdummy2(read_volatile((self.0 + 0x8) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY2 register."]
   #[inline] pub fn set_pccdummy2<F: FnOnce(Pccdummy2) -> Pccdummy2>(&self, f: F) -> &Self {
      let value = f(Pccdummy2(0));
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY2 register."]
   #[inline] pub fn with_pccdummy2<F: FnOnce(Pccdummy2) -> Pccdummy2>(&self, f: F) -> &Self {
      let tmp = self.pccdummy2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
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
         Pccdummy3(read_volatile((self.0 + 0xc) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY3 register."]
   #[inline] pub fn set_pccdummy3<F: FnOnce(Pccdummy3) -> Pccdummy3>(&self, f: F) -> &Self {
      let value = f(Pccdummy3(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY3 register."]
   #[inline] pub fn with_pccdummy3<F: FnOnce(Pccdummy3) -> Pccdummy3>(&self, f: F) -> &Self {
      let tmp = self.pccdummy3();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
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
         Pccdummy4(read_volatile((self.0 + 0x10) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY4 register."]
   #[inline] pub fn set_pccdummy4<F: FnOnce(Pccdummy4) -> Pccdummy4>(&self, f: F) -> &Self {
      let value = f(Pccdummy4(0));
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY4 register."]
   #[inline] pub fn with_pccdummy4<F: FnOnce(Pccdummy4) -> Pccdummy4>(&self, f: F) -> &Self {
      let tmp = self.pccdummy4();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
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
         Pccdummy5(read_volatile((self.0 + 0x14) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY5 register."]
   #[inline] pub fn set_pccdummy5<F: FnOnce(Pccdummy5) -> Pccdummy5>(&self, f: F) -> &Self {
      let value = f(Pccdummy5(0));
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY5 register."]
   #[inline] pub fn with_pccdummy5<F: FnOnce(Pccdummy5) -> Pccdummy5>(&self, f: F) -> &Self {
      let tmp = self.pccdummy5();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
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
         Pccdummy6(read_volatile((self.0 + 0x18) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY6 register."]
   #[inline] pub fn set_pccdummy6<F: FnOnce(Pccdummy6) -> Pccdummy6>(&self, f: F) -> &Self {
      let value = f(Pccdummy6(0));
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY6 register."]
   #[inline] pub fn with_pccdummy6<F: FnOnce(Pccdummy6) -> Pccdummy6>(&self, f: F) -> &Self {
      let tmp = self.pccdummy6();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
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
         Pccdummy7(read_volatile((self.0 + 0x1c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY7 register."]
   #[inline] pub fn set_pccdummy7<F: FnOnce(Pccdummy7) -> Pccdummy7>(&self, f: F) -> &Self {
      let value = f(Pccdummy7(0));
      unsafe {
         write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY7 register."]
   #[inline] pub fn with_pccdummy7<F: FnOnce(Pccdummy7) -> Pccdummy7>(&self, f: F) -> &Self {
      let tmp = self.pccdummy7();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
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
         Pccdummy8(read_volatile((self.0 + 0x20) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY8 register."]
   #[inline] pub fn set_pccdummy8<F: FnOnce(Pccdummy8) -> Pccdummy8>(&self, f: F) -> &Self {
      let value = f(Pccdummy8(0));
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY8 register."]
   #[inline] pub fn with_pccdummy8<F: FnOnce(Pccdummy8) -> Pccdummy8>(&self, f: F) -> &Self {
      let tmp = self.pccdummy8();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
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
         Pccdummy9(read_volatile((self.0 + 0x24) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY9 register."]
   #[inline] pub fn set_pccdummy9<F: FnOnce(Pccdummy9) -> Pccdummy9>(&self, f: F) -> &Self {
      let value = f(Pccdummy9(0));
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY9 register."]
   #[inline] pub fn with_pccdummy9<F: FnOnce(Pccdummy9) -> Pccdummy9>(&self, f: F) -> &Self {
      let tmp = self.pccdummy9();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
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
         Pccdummy10(read_volatile((self.0 + 0x28) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY10 register."]
   #[inline] pub fn set_pccdummy10<F: FnOnce(Pccdummy10) -> Pccdummy10>(&self, f: F) -> &Self {
      let value = f(Pccdummy10(0));
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY10 register."]
   #[inline] pub fn with_pccdummy10<F: FnOnce(Pccdummy10) -> Pccdummy10>(&self, f: F) -> &Self {
      let tmp = self.pccdummy10();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
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
         Pccdummy11(read_volatile((self.0 + 0x2c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY11 register."]
   #[inline] pub fn set_pccdummy11<F: FnOnce(Pccdummy11) -> Pccdummy11>(&self, f: F) -> &Self {
      let value = f(Pccdummy11(0));
      unsafe {
         write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY11 register."]
   #[inline] pub fn with_pccdummy11<F: FnOnce(Pccdummy11) -> Pccdummy11>(&self, f: F) -> &Self {
      let tmp = self.pccdummy11();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
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
         Pccdummy12(read_volatile((self.0 + 0x30) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY12 register."]
   #[inline] pub fn set_pccdummy12<F: FnOnce(Pccdummy12) -> Pccdummy12>(&self, f: F) -> &Self {
      let value = f(Pccdummy12(0));
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY12 register."]
   #[inline] pub fn with_pccdummy12<F: FnOnce(Pccdummy12) -> Pccdummy12>(&self, f: F) -> &Self {
      let tmp = self.pccdummy12();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
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
         Pccdummy13(read_volatile((self.0 + 0x34) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY13 register."]
   #[inline] pub fn set_pccdummy13<F: FnOnce(Pccdummy13) -> Pccdummy13>(&self, f: F) -> &Self {
      let value = f(Pccdummy13(0));
      unsafe {
         write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY13 register."]
   #[inline] pub fn with_pccdummy13<F: FnOnce(Pccdummy13) -> Pccdummy13>(&self, f: F) -> &Self {
      let tmp = self.pccdummy13();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
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
         Pccdummy14(read_volatile((self.0 + 0x38) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY14 register."]
   #[inline] pub fn set_pccdummy14<F: FnOnce(Pccdummy14) -> Pccdummy14>(&self, f: F) -> &Self {
      let value = f(Pccdummy14(0));
      unsafe {
         write_volatile((self.0 + 0x38) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY14 register."]
   #[inline] pub fn with_pccdummy14<F: FnOnce(Pccdummy14) -> Pccdummy14>(&self, f: F) -> &Self {
      let tmp = self.pccdummy14();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x38) as *mut u32, value.0);
      }
      self
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
         Pccdummy15(read_volatile((self.0 + 0x3c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY15 register."]
   #[inline] pub fn set_pccdummy15<F: FnOnce(Pccdummy15) -> Pccdummy15>(&self, f: F) -> &Self {
      let value = f(Pccdummy15(0));
      unsafe {
         write_volatile((self.0 + 0x3c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY15 register."]
   #[inline] pub fn with_pccdummy15<F: FnOnce(Pccdummy15) -> Pccdummy15>(&self, f: F) -> &Self {
      let tmp = self.pccdummy15();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x3c) as *mut u32, value.0);
      }
      self
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
         Pccdummy16(read_volatile((self.0 + 0x40) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY16 register."]
   #[inline] pub fn set_pccdummy16<F: FnOnce(Pccdummy16) -> Pccdummy16>(&self, f: F) -> &Self {
      let value = f(Pccdummy16(0));
      unsafe {
         write_volatile((self.0 + 0x40) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY16 register."]
   #[inline] pub fn with_pccdummy16<F: FnOnce(Pccdummy16) -> Pccdummy16>(&self, f: F) -> &Self {
      let tmp = self.pccdummy16();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x40) as *mut u32, value.0);
      }
      self
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
         Pccdummy17(read_volatile((self.0 + 0x44) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY17 register."]
   #[inline] pub fn set_pccdummy17<F: FnOnce(Pccdummy17) -> Pccdummy17>(&self, f: F) -> &Self {
      let value = f(Pccdummy17(0));
      unsafe {
         write_volatile((self.0 + 0x44) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY17 register."]
   #[inline] pub fn with_pccdummy17<F: FnOnce(Pccdummy17) -> Pccdummy17>(&self, f: F) -> &Self {
      let tmp = self.pccdummy17();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x44) as *mut u32, value.0);
      }
      self
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
         Pccdummy18(read_volatile((self.0 + 0x48) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY18 register."]
   #[inline] pub fn set_pccdummy18<F: FnOnce(Pccdummy18) -> Pccdummy18>(&self, f: F) -> &Self {
      let value = f(Pccdummy18(0));
      unsafe {
         write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY18 register."]
   #[inline] pub fn with_pccdummy18<F: FnOnce(Pccdummy18) -> Pccdummy18>(&self, f: F) -> &Self {
      let tmp = self.pccdummy18();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
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
         Pccdummy19(read_volatile((self.0 + 0x4c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY19 register."]
   #[inline] pub fn set_pccdummy19<F: FnOnce(Pccdummy19) -> Pccdummy19>(&self, f: F) -> &Self {
      let value = f(Pccdummy19(0));
      unsafe {
         write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY19 register."]
   #[inline] pub fn with_pccdummy19<F: FnOnce(Pccdummy19) -> Pccdummy19>(&self, f: F) -> &Self {
      let tmp = self.pccdummy19();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
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
         Pccdummy20(read_volatile((self.0 + 0x50) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY20 register."]
   #[inline] pub fn set_pccdummy20<F: FnOnce(Pccdummy20) -> Pccdummy20>(&self, f: F) -> &Self {
      let value = f(Pccdummy20(0));
      unsafe {
         write_volatile((self.0 + 0x50) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY20 register."]
   #[inline] pub fn with_pccdummy20<F: FnOnce(Pccdummy20) -> Pccdummy20>(&self, f: F) -> &Self {
      let tmp = self.pccdummy20();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x50) as *mut u32, value.0);
      }
      self
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
         Pccdummy21(read_volatile((self.0 + 0x54) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY21 register."]
   #[inline] pub fn set_pccdummy21<F: FnOnce(Pccdummy21) -> Pccdummy21>(&self, f: F) -> &Self {
      let value = f(Pccdummy21(0));
      unsafe {
         write_volatile((self.0 + 0x54) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY21 register."]
   #[inline] pub fn with_pccdummy21<F: FnOnce(Pccdummy21) -> Pccdummy21>(&self, f: F) -> &Self {
      let tmp = self.pccdummy21();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x54) as *mut u32, value.0);
      }
      self
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
         Pccdummy22(read_volatile((self.0 + 0x58) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY22 register."]
   #[inline] pub fn set_pccdummy22<F: FnOnce(Pccdummy22) -> Pccdummy22>(&self, f: F) -> &Self {
      let value = f(Pccdummy22(0));
      unsafe {
         write_volatile((self.0 + 0x58) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY22 register."]
   #[inline] pub fn with_pccdummy22<F: FnOnce(Pccdummy22) -> Pccdummy22>(&self, f: F) -> &Self {
      let tmp = self.pccdummy22();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x58) as *mut u32, value.0);
      }
      self
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
         Pccdummy23(read_volatile((self.0 + 0x5c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY23 register."]
   #[inline] pub fn set_pccdummy23<F: FnOnce(Pccdummy23) -> Pccdummy23>(&self, f: F) -> &Self {
      let value = f(Pccdummy23(0));
      unsafe {
         write_volatile((self.0 + 0x5c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY23 register."]
   #[inline] pub fn with_pccdummy23<F: FnOnce(Pccdummy23) -> Pccdummy23>(&self, f: F) -> &Self {
      let tmp = self.pccdummy23();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x5c) as *mut u32, value.0);
      }
      self
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
         Pccdummy24(read_volatile((self.0 + 0x60) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY24 register."]
   #[inline] pub fn set_pccdummy24<F: FnOnce(Pccdummy24) -> Pccdummy24>(&self, f: F) -> &Self {
      let value = f(Pccdummy24(0));
      unsafe {
         write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY24 register."]
   #[inline] pub fn with_pccdummy24<F: FnOnce(Pccdummy24) -> Pccdummy24>(&self, f: F) -> &Self {
      let tmp = self.pccdummy24();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
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
         Pccdummy25(read_volatile((self.0 + 0x64) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY25 register."]
   #[inline] pub fn set_pccdummy25<F: FnOnce(Pccdummy25) -> Pccdummy25>(&self, f: F) -> &Self {
      let value = f(Pccdummy25(0));
      unsafe {
         write_volatile((self.0 + 0x64) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY25 register."]
   #[inline] pub fn with_pccdummy25<F: FnOnce(Pccdummy25) -> Pccdummy25>(&self, f: F) -> &Self {
      let tmp = self.pccdummy25();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x64) as *mut u32, value.0);
      }
      self
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
         Pccdummy26(read_volatile((self.0 + 0x68) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY26 register."]
   #[inline] pub fn set_pccdummy26<F: FnOnce(Pccdummy26) -> Pccdummy26>(&self, f: F) -> &Self {
      let value = f(Pccdummy26(0));
      unsafe {
         write_volatile((self.0 + 0x68) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY26 register."]
   #[inline] pub fn with_pccdummy26<F: FnOnce(Pccdummy26) -> Pccdummy26>(&self, f: F) -> &Self {
      let tmp = self.pccdummy26();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x68) as *mut u32, value.0);
      }
      self
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
         Pccdummy27(read_volatile((self.0 + 0x6c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY27 register."]
   #[inline] pub fn set_pccdummy27<F: FnOnce(Pccdummy27) -> Pccdummy27>(&self, f: F) -> &Self {
      let value = f(Pccdummy27(0));
      unsafe {
         write_volatile((self.0 + 0x6c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY27 register."]
   #[inline] pub fn with_pccdummy27<F: FnOnce(Pccdummy27) -> Pccdummy27>(&self, f: F) -> &Self {
      let tmp = self.pccdummy27();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x6c) as *mut u32, value.0);
      }
      self
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
         Pccdummy28(read_volatile((self.0 + 0x70) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY28 register."]
   #[inline] pub fn set_pccdummy28<F: FnOnce(Pccdummy28) -> Pccdummy28>(&self, f: F) -> &Self {
      let value = f(Pccdummy28(0));
      unsafe {
         write_volatile((self.0 + 0x70) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY28 register."]
   #[inline] pub fn with_pccdummy28<F: FnOnce(Pccdummy28) -> Pccdummy28>(&self, f: F) -> &Self {
      let tmp = self.pccdummy28();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x70) as *mut u32, value.0);
      }
      self
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
         Pccdummy29(read_volatile((self.0 + 0x74) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY29 register."]
   #[inline] pub fn set_pccdummy29<F: FnOnce(Pccdummy29) -> Pccdummy29>(&self, f: F) -> &Self {
      let value = f(Pccdummy29(0));
      unsafe {
         write_volatile((self.0 + 0x74) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY29 register."]
   #[inline] pub fn with_pccdummy29<F: FnOnce(Pccdummy29) -> Pccdummy29>(&self, f: F) -> &Self {
      let tmp = self.pccdummy29();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x74) as *mut u32, value.0);
      }
      self
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
         Pccdummy30(read_volatile((self.0 + 0x78) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY30 register."]
   #[inline] pub fn set_pccdummy30<F: FnOnce(Pccdummy30) -> Pccdummy30>(&self, f: F) -> &Self {
      let value = f(Pccdummy30(0));
      unsafe {
         write_volatile((self.0 + 0x78) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY30 register."]
   #[inline] pub fn with_pccdummy30<F: FnOnce(Pccdummy30) -> Pccdummy30>(&self, f: F) -> &Self {
      let tmp = self.pccdummy30();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x78) as *mut u32, value.0);
      }
      self
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
         Pccdummy31(read_volatile((self.0 + 0x7c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY31 register."]
   #[inline] pub fn set_pccdummy31<F: FnOnce(Pccdummy31) -> Pccdummy31>(&self, f: F) -> &Self {
      let value = f(Pccdummy31(0));
      unsafe {
         write_volatile((self.0 + 0x7c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY31 register."]
   #[inline] pub fn with_pccdummy31<F: FnOnce(Pccdummy31) -> Pccdummy31>(&self, f: F) -> &Self {
      let tmp = self.pccdummy31();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x7c) as *mut u32, value.0);
      }
      self
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
         Ftfc(read_volatile((self.0 + 0x80) as *const u32))
      }
   }
#[doc="Write the FTFC register."]
   #[inline] pub fn set_ftfc<F: FnOnce(Ftfc) -> Ftfc>(&self, f: F) -> &Self {
      let value = f(Ftfc(0));
      unsafe {
         write_volatile((self.0 + 0x80) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FTFC register."]
   #[inline] pub fn with_ftfc<F: FnOnce(Ftfc) -> Ftfc>(&self, f: F) -> &Self {
      let tmp = self.ftfc();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x80) as *mut u32, value.0);
      }
      self
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
         Dmamux(read_volatile((self.0 + 0x84) as *const u32))
      }
   }
#[doc="Write the DMAMUX register."]
   #[inline] pub fn set_dmamux<F: FnOnce(Dmamux) -> Dmamux>(&self, f: F) -> &Self {
      let value = f(Dmamux(0));
      unsafe {
         write_volatile((self.0 + 0x84) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DMAMUX register."]
   #[inline] pub fn with_dmamux<F: FnOnce(Dmamux) -> Dmamux>(&self, f: F) -> &Self {
      let tmp = self.dmamux();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x84) as *mut u32, value.0);
      }
      self
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
         Pccdummy34(read_volatile((self.0 + 0x88) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY34 register."]
   #[inline] pub fn set_pccdummy34<F: FnOnce(Pccdummy34) -> Pccdummy34>(&self, f: F) -> &Self {
      let value = f(Pccdummy34(0));
      unsafe {
         write_volatile((self.0 + 0x88) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY34 register."]
   #[inline] pub fn with_pccdummy34<F: FnOnce(Pccdummy34) -> Pccdummy34>(&self, f: F) -> &Self {
      let tmp = self.pccdummy34();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x88) as *mut u32, value.0);
      }
      self
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
         Pccdummy35(read_volatile((self.0 + 0x8c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY35 register."]
   #[inline] pub fn set_pccdummy35<F: FnOnce(Pccdummy35) -> Pccdummy35>(&self, f: F) -> &Self {
      let value = f(Pccdummy35(0));
      unsafe {
         write_volatile((self.0 + 0x8c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY35 register."]
   #[inline] pub fn with_pccdummy35<F: FnOnce(Pccdummy35) -> Pccdummy35>(&self, f: F) -> &Self {
      let tmp = self.pccdummy35();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8c) as *mut u32, value.0);
      }
      self
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
         Flexcan0(read_volatile((self.0 + 0x90) as *const u32))
      }
   }
#[doc="Write the FLEXCAN0 register."]
   #[inline] pub fn set_flexcan0<F: FnOnce(Flexcan0) -> Flexcan0>(&self, f: F) -> &Self {
      let value = f(Flexcan0(0));
      unsafe {
         write_volatile((self.0 + 0x90) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FLEXCAN0 register."]
   #[inline] pub fn with_flexcan0<F: FnOnce(Flexcan0) -> Flexcan0>(&self, f: F) -> &Self {
      let tmp = self.flexcan0();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x90) as *mut u32, value.0);
      }
      self
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
         Flexcan1(read_volatile((self.0 + 0x94) as *const u32))
      }
   }
#[doc="Write the FLEXCAN1 register."]
   #[inline] pub fn set_flexcan1<F: FnOnce(Flexcan1) -> Flexcan1>(&self, f: F) -> &Self {
      let value = f(Flexcan1(0));
      unsafe {
         write_volatile((self.0 + 0x94) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FLEXCAN1 register."]
   #[inline] pub fn with_flexcan1<F: FnOnce(Flexcan1) -> Flexcan1>(&self, f: F) -> &Self {
      let tmp = self.flexcan1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x94) as *mut u32, value.0);
      }
      self
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
         Ftm3(read_volatile((self.0 + 0x98) as *const u32))
      }
   }
#[doc="Write the FTM3 register."]
   #[inline] pub fn set_ftm3<F: FnOnce(Ftm3) -> Ftm3>(&self, f: F) -> &Self {
      let value = f(Ftm3(0));
      unsafe {
         write_volatile((self.0 + 0x98) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FTM3 register."]
   #[inline] pub fn with_ftm3<F: FnOnce(Ftm3) -> Ftm3>(&self, f: F) -> &Self {
      let tmp = self.ftm3();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x98) as *mut u32, value.0);
      }
      self
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
         Adc1(read_volatile((self.0 + 0x9c) as *const u32))
      }
   }
#[doc="Write the ADC1 register."]
   #[inline] pub fn set_adc1<F: FnOnce(Adc1) -> Adc1>(&self, f: F) -> &Self {
      let value = f(Adc1(0));
      unsafe {
         write_volatile((self.0 + 0x9c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ADC1 register."]
   #[inline] pub fn with_adc1<F: FnOnce(Adc1) -> Adc1>(&self, f: F) -> &Self {
      let tmp = self.adc1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x9c) as *mut u32, value.0);
      }
      self
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
         Pccdummy40(read_volatile((self.0 + 0xa0) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY40 register."]
   #[inline] pub fn set_pccdummy40<F: FnOnce(Pccdummy40) -> Pccdummy40>(&self, f: F) -> &Self {
      let value = f(Pccdummy40(0));
      unsafe {
         write_volatile((self.0 + 0xa0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY40 register."]
   #[inline] pub fn with_pccdummy40<F: FnOnce(Pccdummy40) -> Pccdummy40>(&self, f: F) -> &Self {
      let tmp = self.pccdummy40();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa0) as *mut u32, value.0);
      }
      self
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
         Pccdummy41(read_volatile((self.0 + 0xa4) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY41 register."]
   #[inline] pub fn set_pccdummy41<F: FnOnce(Pccdummy41) -> Pccdummy41>(&self, f: F) -> &Self {
      let value = f(Pccdummy41(0));
      unsafe {
         write_volatile((self.0 + 0xa4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY41 register."]
   #[inline] pub fn with_pccdummy41<F: FnOnce(Pccdummy41) -> Pccdummy41>(&self, f: F) -> &Self {
      let tmp = self.pccdummy41();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa4) as *mut u32, value.0);
      }
      self
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
         Pccdummy42(read_volatile((self.0 + 0xa8) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY42 register."]
   #[inline] pub fn set_pccdummy42<F: FnOnce(Pccdummy42) -> Pccdummy42>(&self, f: F) -> &Self {
      let value = f(Pccdummy42(0));
      unsafe {
         write_volatile((self.0 + 0xa8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY42 register."]
   #[inline] pub fn with_pccdummy42<F: FnOnce(Pccdummy42) -> Pccdummy42>(&self, f: F) -> &Self {
      let tmp = self.pccdummy42();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa8) as *mut u32, value.0);
      }
      self
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
         Flexcan2(read_volatile((self.0 + 0xac) as *const u32))
      }
   }
#[doc="Write the FLEXCAN2 register."]
   #[inline] pub fn set_flexcan2<F: FnOnce(Flexcan2) -> Flexcan2>(&self, f: F) -> &Self {
      let value = f(Flexcan2(0));
      unsafe {
         write_volatile((self.0 + 0xac) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FLEXCAN2 register."]
   #[inline] pub fn with_flexcan2<F: FnOnce(Flexcan2) -> Flexcan2>(&self, f: F) -> &Self {
      let tmp = self.flexcan2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xac) as *mut u32, value.0);
      }
      self
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
         Lpspi0(read_volatile((self.0 + 0xb0) as *const u32))
      }
   }
#[doc="Write the LPSPI0 register."]
   #[inline] pub fn set_lpspi0<F: FnOnce(Lpspi0) -> Lpspi0>(&self, f: F) -> &Self {
      let value = f(Lpspi0(0));
      unsafe {
         write_volatile((self.0 + 0xb0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LPSPI0 register."]
   #[inline] pub fn with_lpspi0<F: FnOnce(Lpspi0) -> Lpspi0>(&self, f: F) -> &Self {
      let tmp = self.lpspi0();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb0) as *mut u32, value.0);
      }
      self
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
         Lpspi1(read_volatile((self.0 + 0xb4) as *const u32))
      }
   }
#[doc="Write the LPSPI1 register."]
   #[inline] pub fn set_lpspi1<F: FnOnce(Lpspi1) -> Lpspi1>(&self, f: F) -> &Self {
      let value = f(Lpspi1(0));
      unsafe {
         write_volatile((self.0 + 0xb4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LPSPI1 register."]
   #[inline] pub fn with_lpspi1<F: FnOnce(Lpspi1) -> Lpspi1>(&self, f: F) -> &Self {
      let tmp = self.lpspi1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb4) as *mut u32, value.0);
      }
      self
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
         Lpspi2(read_volatile((self.0 + 0xb8) as *const u32))
      }
   }
#[doc="Write the LPSPI2 register."]
   #[inline] pub fn set_lpspi2<F: FnOnce(Lpspi2) -> Lpspi2>(&self, f: F) -> &Self {
      let value = f(Lpspi2(0));
      unsafe {
         write_volatile((self.0 + 0xb8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LPSPI2 register."]
   #[inline] pub fn with_lpspi2<F: FnOnce(Lpspi2) -> Lpspi2>(&self, f: F) -> &Self {
      let tmp = self.lpspi2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb8) as *mut u32, value.0);
      }
      self
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
         Pccdummy47(read_volatile((self.0 + 0xbc) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY47 register."]
   #[inline] pub fn set_pccdummy47<F: FnOnce(Pccdummy47) -> Pccdummy47>(&self, f: F) -> &Self {
      let value = f(Pccdummy47(0));
      unsafe {
         write_volatile((self.0 + 0xbc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY47 register."]
   #[inline] pub fn with_pccdummy47<F: FnOnce(Pccdummy47) -> Pccdummy47>(&self, f: F) -> &Self {
      let tmp = self.pccdummy47();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xbc) as *mut u32, value.0);
      }
      self
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
         Pccdummy48(read_volatile((self.0 + 0xc0) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY48 register."]
   #[inline] pub fn set_pccdummy48<F: FnOnce(Pccdummy48) -> Pccdummy48>(&self, f: F) -> &Self {
      let value = f(Pccdummy48(0));
      unsafe {
         write_volatile((self.0 + 0xc0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY48 register."]
   #[inline] pub fn with_pccdummy48<F: FnOnce(Pccdummy48) -> Pccdummy48>(&self, f: F) -> &Self {
      let tmp = self.pccdummy48();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc0) as *mut u32, value.0);
      }
      self
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
         Pdb1(read_volatile((self.0 + 0xc4) as *const u32))
      }
   }
#[doc="Write the PDB1 register."]
   #[inline] pub fn set_pdb1<F: FnOnce(Pdb1) -> Pdb1>(&self, f: F) -> &Self {
      let value = f(Pdb1(0));
      unsafe {
         write_volatile((self.0 + 0xc4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PDB1 register."]
   #[inline] pub fn with_pdb1<F: FnOnce(Pdb1) -> Pdb1>(&self, f: F) -> &Self {
      let tmp = self.pdb1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc4) as *mut u32, value.0);
      }
      self
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
         Crc(read_volatile((self.0 + 0xc8) as *const u32))
      }
   }
#[doc="Write the CRC register."]
   #[inline] pub fn set_crc<F: FnOnce(Crc) -> Crc>(&self, f: F) -> &Self {
      let value = f(Crc(0));
      unsafe {
         write_volatile((self.0 + 0xc8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CRC register."]
   #[inline] pub fn with_crc<F: FnOnce(Crc) -> Crc>(&self, f: F) -> &Self {
      let tmp = self.crc();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc8) as *mut u32, value.0);
      }
      self
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
         Pccdummy51(read_volatile((self.0 + 0xcc) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY51 register."]
   #[inline] pub fn set_pccdummy51<F: FnOnce(Pccdummy51) -> Pccdummy51>(&self, f: F) -> &Self {
      let value = f(Pccdummy51(0));
      unsafe {
         write_volatile((self.0 + 0xcc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY51 register."]
   #[inline] pub fn with_pccdummy51<F: FnOnce(Pccdummy51) -> Pccdummy51>(&self, f: F) -> &Self {
      let tmp = self.pccdummy51();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xcc) as *mut u32, value.0);
      }
      self
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
         Pccdummy52(read_volatile((self.0 + 0xd0) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY52 register."]
   #[inline] pub fn set_pccdummy52<F: FnOnce(Pccdummy52) -> Pccdummy52>(&self, f: F) -> &Self {
      let value = f(Pccdummy52(0));
      unsafe {
         write_volatile((self.0 + 0xd0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY52 register."]
   #[inline] pub fn with_pccdummy52<F: FnOnce(Pccdummy52) -> Pccdummy52>(&self, f: F) -> &Self {
      let tmp = self.pccdummy52();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xd0) as *mut u32, value.0);
      }
      self
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
         Pccdummy53(read_volatile((self.0 + 0xd4) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY53 register."]
   #[inline] pub fn set_pccdummy53<F: FnOnce(Pccdummy53) -> Pccdummy53>(&self, f: F) -> &Self {
      let value = f(Pccdummy53(0));
      unsafe {
         write_volatile((self.0 + 0xd4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY53 register."]
   #[inline] pub fn with_pccdummy53<F: FnOnce(Pccdummy53) -> Pccdummy53>(&self, f: F) -> &Self {
      let tmp = self.pccdummy53();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xd4) as *mut u32, value.0);
      }
      self
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
         Pdb0(read_volatile((self.0 + 0xd8) as *const u32))
      }
   }
#[doc="Write the PDB0 register."]
   #[inline] pub fn set_pdb0<F: FnOnce(Pdb0) -> Pdb0>(&self, f: F) -> &Self {
      let value = f(Pdb0(0));
      unsafe {
         write_volatile((self.0 + 0xd8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PDB0 register."]
   #[inline] pub fn with_pdb0<F: FnOnce(Pdb0) -> Pdb0>(&self, f: F) -> &Self {
      let tmp = self.pdb0();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xd8) as *mut u32, value.0);
      }
      self
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
         Lpit(read_volatile((self.0 + 0xdc) as *const u32))
      }
   }
#[doc="Write the LPIT register."]
   #[inline] pub fn set_lpit<F: FnOnce(Lpit) -> Lpit>(&self, f: F) -> &Self {
      let value = f(Lpit(0));
      unsafe {
         write_volatile((self.0 + 0xdc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LPIT register."]
   #[inline] pub fn with_lpit<F: FnOnce(Lpit) -> Lpit>(&self, f: F) -> &Self {
      let tmp = self.lpit();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xdc) as *mut u32, value.0);
      }
      self
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
         Ftm0(read_volatile((self.0 + 0xe0) as *const u32))
      }
   }
#[doc="Write the FTM0 register."]
   #[inline] pub fn set_ftm0<F: FnOnce(Ftm0) -> Ftm0>(&self, f: F) -> &Self {
      let value = f(Ftm0(0));
      unsafe {
         write_volatile((self.0 + 0xe0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FTM0 register."]
   #[inline] pub fn with_ftm0<F: FnOnce(Ftm0) -> Ftm0>(&self, f: F) -> &Self {
      let tmp = self.ftm0();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xe0) as *mut u32, value.0);
      }
      self
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
         Ftm1(read_volatile((self.0 + 0xe4) as *const u32))
      }
   }
#[doc="Write the FTM1 register."]
   #[inline] pub fn set_ftm1<F: FnOnce(Ftm1) -> Ftm1>(&self, f: F) -> &Self {
      let value = f(Ftm1(0));
      unsafe {
         write_volatile((self.0 + 0xe4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FTM1 register."]
   #[inline] pub fn with_ftm1<F: FnOnce(Ftm1) -> Ftm1>(&self, f: F) -> &Self {
      let tmp = self.ftm1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xe4) as *mut u32, value.0);
      }
      self
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
         Ftm2(read_volatile((self.0 + 0xe8) as *const u32))
      }
   }
#[doc="Write the FTM2 register."]
   #[inline] pub fn set_ftm2<F: FnOnce(Ftm2) -> Ftm2>(&self, f: F) -> &Self {
      let value = f(Ftm2(0));
      unsafe {
         write_volatile((self.0 + 0xe8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FTM2 register."]
   #[inline] pub fn with_ftm2<F: FnOnce(Ftm2) -> Ftm2>(&self, f: F) -> &Self {
      let tmp = self.ftm2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xe8) as *mut u32, value.0);
      }
      self
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
         Adc0(read_volatile((self.0 + 0xec) as *const u32))
      }
   }
#[doc="Write the ADC0 register."]
   #[inline] pub fn set_adc0<F: FnOnce(Adc0) -> Adc0>(&self, f: F) -> &Self {
      let value = f(Adc0(0));
      unsafe {
         write_volatile((self.0 + 0xec) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ADC0 register."]
   #[inline] pub fn with_adc0<F: FnOnce(Adc0) -> Adc0>(&self, f: F) -> &Self {
      let tmp = self.adc0();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xec) as *mut u32, value.0);
      }
      self
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
         Pccdummy60(read_volatile((self.0 + 0xf0) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY60 register."]
   #[inline] pub fn set_pccdummy60<F: FnOnce(Pccdummy60) -> Pccdummy60>(&self, f: F) -> &Self {
      let value = f(Pccdummy60(0));
      unsafe {
         write_volatile((self.0 + 0xf0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY60 register."]
   #[inline] pub fn with_pccdummy60<F: FnOnce(Pccdummy60) -> Pccdummy60>(&self, f: F) -> &Self {
      let tmp = self.pccdummy60();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xf0) as *mut u32, value.0);
      }
      self
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
         Rtc(read_volatile((self.0 + 0xf4) as *const u32))
      }
   }
#[doc="Write the RTC register."]
   #[inline] pub fn set_rtc<F: FnOnce(Rtc) -> Rtc>(&self, f: F) -> &Self {
      let value = f(Rtc(0));
      unsafe {
         write_volatile((self.0 + 0xf4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RTC register."]
   #[inline] pub fn with_rtc<F: FnOnce(Rtc) -> Rtc>(&self, f: F) -> &Self {
      let tmp = self.rtc();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xf4) as *mut u32, value.0);
      }
      self
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
         Pccdummy62(read_volatile((self.0 + 0xf8) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY62 register."]
   #[inline] pub fn set_pccdummy62<F: FnOnce(Pccdummy62) -> Pccdummy62>(&self, f: F) -> &Self {
      let value = f(Pccdummy62(0));
      unsafe {
         write_volatile((self.0 + 0xf8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY62 register."]
   #[inline] pub fn with_pccdummy62<F: FnOnce(Pccdummy62) -> Pccdummy62>(&self, f: F) -> &Self {
      let tmp = self.pccdummy62();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xf8) as *mut u32, value.0);
      }
      self
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
         Pccdummy63(read_volatile((self.0 + 0xfc) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY63 register."]
   #[inline] pub fn set_pccdummy63<F: FnOnce(Pccdummy63) -> Pccdummy63>(&self, f: F) -> &Self {
      let value = f(Pccdummy63(0));
      unsafe {
         write_volatile((self.0 + 0xfc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY63 register."]
   #[inline] pub fn with_pccdummy63<F: FnOnce(Pccdummy63) -> Pccdummy63>(&self, f: F) -> &Self {
      let tmp = self.pccdummy63();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xfc) as *mut u32, value.0);
      }
      self
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
         Lptmr0(read_volatile((self.0 + 0x100) as *const u32))
      }
   }
#[doc="Write the LPTMR0 register."]
   #[inline] pub fn set_lptmr0<F: FnOnce(Lptmr0) -> Lptmr0>(&self, f: F) -> &Self {
      let value = f(Lptmr0(0));
      unsafe {
         write_volatile((self.0 + 0x100) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LPTMR0 register."]
   #[inline] pub fn with_lptmr0<F: FnOnce(Lptmr0) -> Lptmr0>(&self, f: F) -> &Self {
      let tmp = self.lptmr0();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x100) as *mut u32, value.0);
      }
      self
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
         Pccdummy65(read_volatile((self.0 + 0x104) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY65 register."]
   #[inline] pub fn set_pccdummy65<F: FnOnce(Pccdummy65) -> Pccdummy65>(&self, f: F) -> &Self {
      let value = f(Pccdummy65(0));
      unsafe {
         write_volatile((self.0 + 0x104) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY65 register."]
   #[inline] pub fn with_pccdummy65<F: FnOnce(Pccdummy65) -> Pccdummy65>(&self, f: F) -> &Self {
      let tmp = self.pccdummy65();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x104) as *mut u32, value.0);
      }
      self
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
         Pccdummy66(read_volatile((self.0 + 0x108) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY66 register."]
   #[inline] pub fn set_pccdummy66<F: FnOnce(Pccdummy66) -> Pccdummy66>(&self, f: F) -> &Self {
      let value = f(Pccdummy66(0));
      unsafe {
         write_volatile((self.0 + 0x108) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY66 register."]
   #[inline] pub fn with_pccdummy66<F: FnOnce(Pccdummy66) -> Pccdummy66>(&self, f: F) -> &Self {
      let tmp = self.pccdummy66();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x108) as *mut u32, value.0);
      }
      self
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
         Pccdummy67(read_volatile((self.0 + 0x10c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY67 register."]
   #[inline] pub fn set_pccdummy67<F: FnOnce(Pccdummy67) -> Pccdummy67>(&self, f: F) -> &Self {
      let value = f(Pccdummy67(0));
      unsafe {
         write_volatile((self.0 + 0x10c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY67 register."]
   #[inline] pub fn with_pccdummy67<F: FnOnce(Pccdummy67) -> Pccdummy67>(&self, f: F) -> &Self {
      let tmp = self.pccdummy67();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x10c) as *mut u32, value.0);
      }
      self
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
         Pccdummy68(read_volatile((self.0 + 0x110) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY68 register."]
   #[inline] pub fn set_pccdummy68<F: FnOnce(Pccdummy68) -> Pccdummy68>(&self, f: F) -> &Self {
      let value = f(Pccdummy68(0));
      unsafe {
         write_volatile((self.0 + 0x110) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY68 register."]
   #[inline] pub fn with_pccdummy68<F: FnOnce(Pccdummy68) -> Pccdummy68>(&self, f: F) -> &Self {
      let tmp = self.pccdummy68();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x110) as *mut u32, value.0);
      }
      self
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
         Pccdummy69(read_volatile((self.0 + 0x114) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY69 register."]
   #[inline] pub fn set_pccdummy69<F: FnOnce(Pccdummy69) -> Pccdummy69>(&self, f: F) -> &Self {
      let value = f(Pccdummy69(0));
      unsafe {
         write_volatile((self.0 + 0x114) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY69 register."]
   #[inline] pub fn with_pccdummy69<F: FnOnce(Pccdummy69) -> Pccdummy69>(&self, f: F) -> &Self {
      let tmp = self.pccdummy69();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x114) as *mut u32, value.0);
      }
      self
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
         Pccdummy70(read_volatile((self.0 + 0x118) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY70 register."]
   #[inline] pub fn set_pccdummy70<F: FnOnce(Pccdummy70) -> Pccdummy70>(&self, f: F) -> &Self {
      let value = f(Pccdummy70(0));
      unsafe {
         write_volatile((self.0 + 0x118) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY70 register."]
   #[inline] pub fn with_pccdummy70<F: FnOnce(Pccdummy70) -> Pccdummy70>(&self, f: F) -> &Self {
      let tmp = self.pccdummy70();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x118) as *mut u32, value.0);
      }
      self
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
         Pccdummy71(read_volatile((self.0 + 0x11c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY71 register."]
   #[inline] pub fn set_pccdummy71<F: FnOnce(Pccdummy71) -> Pccdummy71>(&self, f: F) -> &Self {
      let value = f(Pccdummy71(0));
      unsafe {
         write_volatile((self.0 + 0x11c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY71 register."]
   #[inline] pub fn with_pccdummy71<F: FnOnce(Pccdummy71) -> Pccdummy71>(&self, f: F) -> &Self {
      let tmp = self.pccdummy71();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x11c) as *mut u32, value.0);
      }
      self
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
         Pccdummy72(read_volatile((self.0 + 0x120) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY72 register."]
   #[inline] pub fn set_pccdummy72<F: FnOnce(Pccdummy72) -> Pccdummy72>(&self, f: F) -> &Self {
      let value = f(Pccdummy72(0));
      unsafe {
         write_volatile((self.0 + 0x120) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY72 register."]
   #[inline] pub fn with_pccdummy72<F: FnOnce(Pccdummy72) -> Pccdummy72>(&self, f: F) -> &Self {
      let tmp = self.pccdummy72();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x120) as *mut u32, value.0);
      }
      self
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
         Porta(read_volatile((self.0 + 0x124) as *const u32))
      }
   }
#[doc="Write the PORTA register."]
   #[inline] pub fn set_porta<F: FnOnce(Porta) -> Porta>(&self, f: F) -> &Self {
      let value = f(Porta(0));
      unsafe {
         write_volatile((self.0 + 0x124) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PORTA register."]
   #[inline] pub fn with_porta<F: FnOnce(Porta) -> Porta>(&self, f: F) -> &Self {
      let tmp = self.porta();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x124) as *mut u32, value.0);
      }
      self
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
         Portb(read_volatile((self.0 + 0x128) as *const u32))
      }
   }
#[doc="Write the PORTB register."]
   #[inline] pub fn set_portb<F: FnOnce(Portb) -> Portb>(&self, f: F) -> &Self {
      let value = f(Portb(0));
      unsafe {
         write_volatile((self.0 + 0x128) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PORTB register."]
   #[inline] pub fn with_portb<F: FnOnce(Portb) -> Portb>(&self, f: F) -> &Self {
      let tmp = self.portb();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x128) as *mut u32, value.0);
      }
      self
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
         Portc(read_volatile((self.0 + 0x12c) as *const u32))
      }
   }
#[doc="Write the PORTC register."]
   #[inline] pub fn set_portc<F: FnOnce(Portc) -> Portc>(&self, f: F) -> &Self {
      let value = f(Portc(0));
      unsafe {
         write_volatile((self.0 + 0x12c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PORTC register."]
   #[inline] pub fn with_portc<F: FnOnce(Portc) -> Portc>(&self, f: F) -> &Self {
      let tmp = self.portc();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x12c) as *mut u32, value.0);
      }
      self
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
         Portd(read_volatile((self.0 + 0x130) as *const u32))
      }
   }
#[doc="Write the PORTD register."]
   #[inline] pub fn set_portd<F: FnOnce(Portd) -> Portd>(&self, f: F) -> &Self {
      let value = f(Portd(0));
      unsafe {
         write_volatile((self.0 + 0x130) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PORTD register."]
   #[inline] pub fn with_portd<F: FnOnce(Portd) -> Portd>(&self, f: F) -> &Self {
      let tmp = self.portd();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x130) as *mut u32, value.0);
      }
      self
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
         Porte(read_volatile((self.0 + 0x134) as *const u32))
      }
   }
#[doc="Write the PORTE register."]
   #[inline] pub fn set_porte<F: FnOnce(Porte) -> Porte>(&self, f: F) -> &Self {
      let value = f(Porte(0));
      unsafe {
         write_volatile((self.0 + 0x134) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PORTE register."]
   #[inline] pub fn with_porte<F: FnOnce(Porte) -> Porte>(&self, f: F) -> &Self {
      let tmp = self.porte();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x134) as *mut u32, value.0);
      }
      self
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
         Pccdummy78(read_volatile((self.0 + 0x138) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY78 register."]
   #[inline] pub fn set_pccdummy78<F: FnOnce(Pccdummy78) -> Pccdummy78>(&self, f: F) -> &Self {
      let value = f(Pccdummy78(0));
      unsafe {
         write_volatile((self.0 + 0x138) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY78 register."]
   #[inline] pub fn with_pccdummy78<F: FnOnce(Pccdummy78) -> Pccdummy78>(&self, f: F) -> &Self {
      let tmp = self.pccdummy78();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x138) as *mut u32, value.0);
      }
      self
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
         Pccdummy79(read_volatile((self.0 + 0x13c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY79 register."]
   #[inline] pub fn set_pccdummy79<F: FnOnce(Pccdummy79) -> Pccdummy79>(&self, f: F) -> &Self {
      let value = f(Pccdummy79(0));
      unsafe {
         write_volatile((self.0 + 0x13c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY79 register."]
   #[inline] pub fn with_pccdummy79<F: FnOnce(Pccdummy79) -> Pccdummy79>(&self, f: F) -> &Self {
      let tmp = self.pccdummy79();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x13c) as *mut u32, value.0);
      }
      self
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
         Pccdummy80(read_volatile((self.0 + 0x140) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY80 register."]
   #[inline] pub fn set_pccdummy80<F: FnOnce(Pccdummy80) -> Pccdummy80>(&self, f: F) -> &Self {
      let value = f(Pccdummy80(0));
      unsafe {
         write_volatile((self.0 + 0x140) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY80 register."]
   #[inline] pub fn with_pccdummy80<F: FnOnce(Pccdummy80) -> Pccdummy80>(&self, f: F) -> &Self {
      let tmp = self.pccdummy80();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x140) as *mut u32, value.0);
      }
      self
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
         Pccdummy81(read_volatile((self.0 + 0x144) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY81 register."]
   #[inline] pub fn set_pccdummy81<F: FnOnce(Pccdummy81) -> Pccdummy81>(&self, f: F) -> &Self {
      let value = f(Pccdummy81(0));
      unsafe {
         write_volatile((self.0 + 0x144) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY81 register."]
   #[inline] pub fn with_pccdummy81<F: FnOnce(Pccdummy81) -> Pccdummy81>(&self, f: F) -> &Self {
      let tmp = self.pccdummy81();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x144) as *mut u32, value.0);
      }
      self
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
         Pccdummy82(read_volatile((self.0 + 0x148) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY82 register."]
   #[inline] pub fn set_pccdummy82<F: FnOnce(Pccdummy82) -> Pccdummy82>(&self, f: F) -> &Self {
      let value = f(Pccdummy82(0));
      unsafe {
         write_volatile((self.0 + 0x148) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY82 register."]
   #[inline] pub fn with_pccdummy82<F: FnOnce(Pccdummy82) -> Pccdummy82>(&self, f: F) -> &Self {
      let tmp = self.pccdummy82();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x148) as *mut u32, value.0);
      }
      self
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
         Pccdummy83(read_volatile((self.0 + 0x14c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY83 register."]
   #[inline] pub fn set_pccdummy83<F: FnOnce(Pccdummy83) -> Pccdummy83>(&self, f: F) -> &Self {
      let value = f(Pccdummy83(0));
      unsafe {
         write_volatile((self.0 + 0x14c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY83 register."]
   #[inline] pub fn with_pccdummy83<F: FnOnce(Pccdummy83) -> Pccdummy83>(&self, f: F) -> &Self {
      let tmp = self.pccdummy83();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14c) as *mut u32, value.0);
      }
      self
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
         Pccdummy84(read_volatile((self.0 + 0x150) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY84 register."]
   #[inline] pub fn set_pccdummy84<F: FnOnce(Pccdummy84) -> Pccdummy84>(&self, f: F) -> &Self {
      let value = f(Pccdummy84(0));
      unsafe {
         write_volatile((self.0 + 0x150) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY84 register."]
   #[inline] pub fn with_pccdummy84<F: FnOnce(Pccdummy84) -> Pccdummy84>(&self, f: F) -> &Self {
      let tmp = self.pccdummy84();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x150) as *mut u32, value.0);
      }
      self
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
         Pccdummy85(read_volatile((self.0 + 0x154) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY85 register."]
   #[inline] pub fn set_pccdummy85<F: FnOnce(Pccdummy85) -> Pccdummy85>(&self, f: F) -> &Self {
      let value = f(Pccdummy85(0));
      unsafe {
         write_volatile((self.0 + 0x154) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY85 register."]
   #[inline] pub fn with_pccdummy85<F: FnOnce(Pccdummy85) -> Pccdummy85>(&self, f: F) -> &Self {
      let tmp = self.pccdummy85();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x154) as *mut u32, value.0);
      }
      self
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
         Pccdummy86(read_volatile((self.0 + 0x158) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY86 register."]
   #[inline] pub fn set_pccdummy86<F: FnOnce(Pccdummy86) -> Pccdummy86>(&self, f: F) -> &Self {
      let value = f(Pccdummy86(0));
      unsafe {
         write_volatile((self.0 + 0x158) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY86 register."]
   #[inline] pub fn with_pccdummy86<F: FnOnce(Pccdummy86) -> Pccdummy86>(&self, f: F) -> &Self {
      let tmp = self.pccdummy86();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x158) as *mut u32, value.0);
      }
      self
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
         Pccdummy87(read_volatile((self.0 + 0x15c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY87 register."]
   #[inline] pub fn set_pccdummy87<F: FnOnce(Pccdummy87) -> Pccdummy87>(&self, f: F) -> &Self {
      let value = f(Pccdummy87(0));
      unsafe {
         write_volatile((self.0 + 0x15c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY87 register."]
   #[inline] pub fn with_pccdummy87<F: FnOnce(Pccdummy87) -> Pccdummy87>(&self, f: F) -> &Self {
      let tmp = self.pccdummy87();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x15c) as *mut u32, value.0);
      }
      self
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
         Pccdummy88(read_volatile((self.0 + 0x160) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY88 register."]
   #[inline] pub fn set_pccdummy88<F: FnOnce(Pccdummy88) -> Pccdummy88>(&self, f: F) -> &Self {
      let value = f(Pccdummy88(0));
      unsafe {
         write_volatile((self.0 + 0x160) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY88 register."]
   #[inline] pub fn with_pccdummy88<F: FnOnce(Pccdummy88) -> Pccdummy88>(&self, f: F) -> &Self {
      let tmp = self.pccdummy88();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x160) as *mut u32, value.0);
      }
      self
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
         Pccdummy89(read_volatile((self.0 + 0x164) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY89 register."]
   #[inline] pub fn set_pccdummy89<F: FnOnce(Pccdummy89) -> Pccdummy89>(&self, f: F) -> &Self {
      let value = f(Pccdummy89(0));
      unsafe {
         write_volatile((self.0 + 0x164) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY89 register."]
   #[inline] pub fn with_pccdummy89<F: FnOnce(Pccdummy89) -> Pccdummy89>(&self, f: F) -> &Self {
      let tmp = self.pccdummy89();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x164) as *mut u32, value.0);
      }
      self
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
         Flexio(read_volatile((self.0 + 0x168) as *const u32))
      }
   }
#[doc="Write the FLEXIO register."]
   #[inline] pub fn set_flexio<F: FnOnce(Flexio) -> Flexio>(&self, f: F) -> &Self {
      let value = f(Flexio(0));
      unsafe {
         write_volatile((self.0 + 0x168) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FLEXIO register."]
   #[inline] pub fn with_flexio<F: FnOnce(Flexio) -> Flexio>(&self, f: F) -> &Self {
      let tmp = self.flexio();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x168) as *mut u32, value.0);
      }
      self
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
         Pccdummy91(read_volatile((self.0 + 0x16c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY91 register."]
   #[inline] pub fn set_pccdummy91<F: FnOnce(Pccdummy91) -> Pccdummy91>(&self, f: F) -> &Self {
      let value = f(Pccdummy91(0));
      unsafe {
         write_volatile((self.0 + 0x16c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY91 register."]
   #[inline] pub fn with_pccdummy91<F: FnOnce(Pccdummy91) -> Pccdummy91>(&self, f: F) -> &Self {
      let tmp = self.pccdummy91();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x16c) as *mut u32, value.0);
      }
      self
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
         Pccdummy92(read_volatile((self.0 + 0x170) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY92 register."]
   #[inline] pub fn set_pccdummy92<F: FnOnce(Pccdummy92) -> Pccdummy92>(&self, f: F) -> &Self {
      let value = f(Pccdummy92(0));
      unsafe {
         write_volatile((self.0 + 0x170) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY92 register."]
   #[inline] pub fn with_pccdummy92<F: FnOnce(Pccdummy92) -> Pccdummy92>(&self, f: F) -> &Self {
      let tmp = self.pccdummy92();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x170) as *mut u32, value.0);
      }
      self
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
         Pccdummy93(read_volatile((self.0 + 0x174) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY93 register."]
   #[inline] pub fn set_pccdummy93<F: FnOnce(Pccdummy93) -> Pccdummy93>(&self, f: F) -> &Self {
      let value = f(Pccdummy93(0));
      unsafe {
         write_volatile((self.0 + 0x174) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY93 register."]
   #[inline] pub fn with_pccdummy93<F: FnOnce(Pccdummy93) -> Pccdummy93>(&self, f: F) -> &Self {
      let tmp = self.pccdummy93();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x174) as *mut u32, value.0);
      }
      self
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
         Pccdummy94(read_volatile((self.0 + 0x178) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY94 register."]
   #[inline] pub fn set_pccdummy94<F: FnOnce(Pccdummy94) -> Pccdummy94>(&self, f: F) -> &Self {
      let value = f(Pccdummy94(0));
      unsafe {
         write_volatile((self.0 + 0x178) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY94 register."]
   #[inline] pub fn with_pccdummy94<F: FnOnce(Pccdummy94) -> Pccdummy94>(&self, f: F) -> &Self {
      let tmp = self.pccdummy94();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x178) as *mut u32, value.0);
      }
      self
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
         Pccdummy95(read_volatile((self.0 + 0x17c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY95 register."]
   #[inline] pub fn set_pccdummy95<F: FnOnce(Pccdummy95) -> Pccdummy95>(&self, f: F) -> &Self {
      let value = f(Pccdummy95(0));
      unsafe {
         write_volatile((self.0 + 0x17c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY95 register."]
   #[inline] pub fn with_pccdummy95<F: FnOnce(Pccdummy95) -> Pccdummy95>(&self, f: F) -> &Self {
      let tmp = self.pccdummy95();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x17c) as *mut u32, value.0);
      }
      self
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
         Pccdummy96(read_volatile((self.0 + 0x180) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY96 register."]
   #[inline] pub fn set_pccdummy96<F: FnOnce(Pccdummy96) -> Pccdummy96>(&self, f: F) -> &Self {
      let value = f(Pccdummy96(0));
      unsafe {
         write_volatile((self.0 + 0x180) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY96 register."]
   #[inline] pub fn with_pccdummy96<F: FnOnce(Pccdummy96) -> Pccdummy96>(&self, f: F) -> &Self {
      let tmp = self.pccdummy96();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x180) as *mut u32, value.0);
      }
      self
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
         Ewm(read_volatile((self.0 + 0x184) as *const u32))
      }
   }
#[doc="Write the EWM register."]
   #[inline] pub fn set_ewm<F: FnOnce(Ewm) -> Ewm>(&self, f: F) -> &Self {
      let value = f(Ewm(0));
      unsafe {
         write_volatile((self.0 + 0x184) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the EWM register."]
   #[inline] pub fn with_ewm<F: FnOnce(Ewm) -> Ewm>(&self, f: F) -> &Self {
      let tmp = self.ewm();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x184) as *mut u32, value.0);
      }
      self
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
         Pccdummy98(read_volatile((self.0 + 0x188) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY98 register."]
   #[inline] pub fn set_pccdummy98<F: FnOnce(Pccdummy98) -> Pccdummy98>(&self, f: F) -> &Self {
      let value = f(Pccdummy98(0));
      unsafe {
         write_volatile((self.0 + 0x188) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY98 register."]
   #[inline] pub fn with_pccdummy98<F: FnOnce(Pccdummy98) -> Pccdummy98>(&self, f: F) -> &Self {
      let tmp = self.pccdummy98();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x188) as *mut u32, value.0);
      }
      self
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
         Pccdummy99(read_volatile((self.0 + 0x18c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY99 register."]
   #[inline] pub fn set_pccdummy99<F: FnOnce(Pccdummy99) -> Pccdummy99>(&self, f: F) -> &Self {
      let value = f(Pccdummy99(0));
      unsafe {
         write_volatile((self.0 + 0x18c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY99 register."]
   #[inline] pub fn with_pccdummy99<F: FnOnce(Pccdummy99) -> Pccdummy99>(&self, f: F) -> &Self {
      let tmp = self.pccdummy99();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18c) as *mut u32, value.0);
      }
      self
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
         Pccdummy100(read_volatile((self.0 + 0x190) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY100 register."]
   #[inline] pub fn set_pccdummy100<F: FnOnce(Pccdummy100) -> Pccdummy100>(&self, f: F) -> &Self {
      let value = f(Pccdummy100(0));
      unsafe {
         write_volatile((self.0 + 0x190) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY100 register."]
   #[inline] pub fn with_pccdummy100<F: FnOnce(Pccdummy100) -> Pccdummy100>(&self, f: F) -> &Self {
      let tmp = self.pccdummy100();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x190) as *mut u32, value.0);
      }
      self
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
         Pccdummy101(read_volatile((self.0 + 0x194) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY101 register."]
   #[inline] pub fn set_pccdummy101<F: FnOnce(Pccdummy101) -> Pccdummy101>(&self, f: F) -> &Self {
      let value = f(Pccdummy101(0));
      unsafe {
         write_volatile((self.0 + 0x194) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY101 register."]
   #[inline] pub fn with_pccdummy101<F: FnOnce(Pccdummy101) -> Pccdummy101>(&self, f: F) -> &Self {
      let tmp = self.pccdummy101();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x194) as *mut u32, value.0);
      }
      self
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
         Lpi2c0(read_volatile((self.0 + 0x198) as *const u32))
      }
   }
#[doc="Write the LPI2C0 register."]
   #[inline] pub fn set_lpi2c0<F: FnOnce(Lpi2c0) -> Lpi2c0>(&self, f: F) -> &Self {
      let value = f(Lpi2c0(0));
      unsafe {
         write_volatile((self.0 + 0x198) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LPI2C0 register."]
   #[inline] pub fn with_lpi2c0<F: FnOnce(Lpi2c0) -> Lpi2c0>(&self, f: F) -> &Self {
      let tmp = self.lpi2c0();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x198) as *mut u32, value.0);
      }
      self
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
         Pccdummy103(read_volatile((self.0 + 0x19c) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY103 register."]
   #[inline] pub fn set_pccdummy103<F: FnOnce(Pccdummy103) -> Pccdummy103>(&self, f: F) -> &Self {
      let value = f(Pccdummy103(0));
      unsafe {
         write_volatile((self.0 + 0x19c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY103 register."]
   #[inline] pub fn with_pccdummy103<F: FnOnce(Pccdummy103) -> Pccdummy103>(&self, f: F) -> &Self {
      let tmp = self.pccdummy103();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x19c) as *mut u32, value.0);
      }
      self
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
         Pccdummy104(read_volatile((self.0 + 0x1a0) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY104 register."]
   #[inline] pub fn set_pccdummy104<F: FnOnce(Pccdummy104) -> Pccdummy104>(&self, f: F) -> &Self {
      let value = f(Pccdummy104(0));
      unsafe {
         write_volatile((self.0 + 0x1a0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY104 register."]
   #[inline] pub fn with_pccdummy104<F: FnOnce(Pccdummy104) -> Pccdummy104>(&self, f: F) -> &Self {
      let tmp = self.pccdummy104();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1a0) as *mut u32, value.0);
      }
      self
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
         Pccdummy105(read_volatile((self.0 + 0x1a4) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY105 register."]
   #[inline] pub fn set_pccdummy105<F: FnOnce(Pccdummy105) -> Pccdummy105>(&self, f: F) -> &Self {
      let value = f(Pccdummy105(0));
      unsafe {
         write_volatile((self.0 + 0x1a4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY105 register."]
   #[inline] pub fn with_pccdummy105<F: FnOnce(Pccdummy105) -> Pccdummy105>(&self, f: F) -> &Self {
      let tmp = self.pccdummy105();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1a4) as *mut u32, value.0);
      }
      self
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
         Lpuart0(read_volatile((self.0 + 0x1a8) as *const u32))
      }
   }
#[doc="Write the LPUART0 register."]
   #[inline] pub fn set_lpuart0<F: FnOnce(Lpuart0) -> Lpuart0>(&self, f: F) -> &Self {
      let value = f(Lpuart0(0));
      unsafe {
         write_volatile((self.0 + 0x1a8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LPUART0 register."]
   #[inline] pub fn with_lpuart0<F: FnOnce(Lpuart0) -> Lpuart0>(&self, f: F) -> &Self {
      let tmp = self.lpuart0();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1a8) as *mut u32, value.0);
      }
      self
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
         Lpuart1(read_volatile((self.0 + 0x1ac) as *const u32))
      }
   }
#[doc="Write the LPUART1 register."]
   #[inline] pub fn set_lpuart1<F: FnOnce(Lpuart1) -> Lpuart1>(&self, f: F) -> &Self {
      let value = f(Lpuart1(0));
      unsafe {
         write_volatile((self.0 + 0x1ac) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LPUART1 register."]
   #[inline] pub fn with_lpuart1<F: FnOnce(Lpuart1) -> Lpuart1>(&self, f: F) -> &Self {
      let tmp = self.lpuart1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1ac) as *mut u32, value.0);
      }
      self
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
         Lpuart2(read_volatile((self.0 + 0x1b0) as *const u32))
      }
   }
#[doc="Write the LPUART2 register."]
   #[inline] pub fn set_lpuart2<F: FnOnce(Lpuart2) -> Lpuart2>(&self, f: F) -> &Self {
      let value = f(Lpuart2(0));
      unsafe {
         write_volatile((self.0 + 0x1b0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LPUART2 register."]
   #[inline] pub fn with_lpuart2<F: FnOnce(Lpuart2) -> Lpuart2>(&self, f: F) -> &Self {
      let tmp = self.lpuart2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1b0) as *mut u32, value.0);
      }
      self
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
         Pccdummy109(read_volatile((self.0 + 0x1b4) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY109 register."]
   #[inline] pub fn set_pccdummy109<F: FnOnce(Pccdummy109) -> Pccdummy109>(&self, f: F) -> &Self {
      let value = f(Pccdummy109(0));
      unsafe {
         write_volatile((self.0 + 0x1b4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY109 register."]
   #[inline] pub fn with_pccdummy109<F: FnOnce(Pccdummy109) -> Pccdummy109>(&self, f: F) -> &Self {
      let tmp = self.pccdummy109();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1b4) as *mut u32, value.0);
      }
      self
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
         Pccdummy110(read_volatile((self.0 + 0x1b8) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY110 register."]
   #[inline] pub fn set_pccdummy110<F: FnOnce(Pccdummy110) -> Pccdummy110>(&self, f: F) -> &Self {
      let value = f(Pccdummy110(0));
      unsafe {
         write_volatile((self.0 + 0x1b8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY110 register."]
   #[inline] pub fn with_pccdummy110<F: FnOnce(Pccdummy110) -> Pccdummy110>(&self, f: F) -> &Self {
      let tmp = self.pccdummy110();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1b8) as *mut u32, value.0);
      }
      self
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
         Pccdummy111(read_volatile((self.0 + 0x1bc) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY111 register."]
   #[inline] pub fn set_pccdummy111<F: FnOnce(Pccdummy111) -> Pccdummy111>(&self, f: F) -> &Self {
      let value = f(Pccdummy111(0));
      unsafe {
         write_volatile((self.0 + 0x1bc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY111 register."]
   #[inline] pub fn with_pccdummy111<F: FnOnce(Pccdummy111) -> Pccdummy111>(&self, f: F) -> &Self {
      let tmp = self.pccdummy111();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1bc) as *mut u32, value.0);
      }
      self
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
         Pccdummy112(read_volatile((self.0 + 0x1c0) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY112 register."]
   #[inline] pub fn set_pccdummy112<F: FnOnce(Pccdummy112) -> Pccdummy112>(&self, f: F) -> &Self {
      let value = f(Pccdummy112(0));
      unsafe {
         write_volatile((self.0 + 0x1c0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY112 register."]
   #[inline] pub fn with_pccdummy112<F: FnOnce(Pccdummy112) -> Pccdummy112>(&self, f: F) -> &Self {
      let tmp = self.pccdummy112();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1c0) as *mut u32, value.0);
      }
      self
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
         Pccdummy113(read_volatile((self.0 + 0x1c4) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY113 register."]
   #[inline] pub fn set_pccdummy113<F: FnOnce(Pccdummy113) -> Pccdummy113>(&self, f: F) -> &Self {
      let value = f(Pccdummy113(0));
      unsafe {
         write_volatile((self.0 + 0x1c4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY113 register."]
   #[inline] pub fn with_pccdummy113<F: FnOnce(Pccdummy113) -> Pccdummy113>(&self, f: F) -> &Self {
      let tmp = self.pccdummy113();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1c4) as *mut u32, value.0);
      }
      self
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
         Pccdummy114(read_volatile((self.0 + 0x1c8) as *const u32))
      }
   }
#[doc="Write the PCCDUMMY114 register."]
   #[inline] pub fn set_pccdummy114<F: FnOnce(Pccdummy114) -> Pccdummy114>(&self, f: F) -> &Self {
      let value = f(Pccdummy114(0));
      unsafe {
         write_volatile((self.0 + 0x1c8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCDUMMY114 register."]
   #[inline] pub fn with_pccdummy114<F: FnOnce(Pccdummy114) -> Pccdummy114>(&self, f: F) -> &Self {
      let tmp = self.pccdummy114();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1c8) as *mut u32, value.0);
      }
      self
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
         Cmp0(read_volatile((self.0 + 0x1cc) as *const u32))
      }
   }
#[doc="Write the CMP0 register."]
   #[inline] pub fn set_cmp0<F: FnOnce(Cmp0) -> Cmp0>(&self, f: F) -> &Self {
      let value = f(Cmp0(0));
      unsafe {
         write_volatile((self.0 + 0x1cc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CMP0 register."]
   #[inline] pub fn with_cmp0<F: FnOnce(Cmp0) -> Cmp0>(&self, f: F) -> &Self {
      let tmp = self.cmp0();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1cc) as *mut u32, value.0);
      }
      self
   }

}

#[doc="PCC Reserved Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftfc(pub u32);
impl Ftfc {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmamux(pub u32);
impl Dmamux {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flexcan0(pub u32);
impl Flexcan0 {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flexcan1(pub u32);
impl Flexcan1 {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftm3(pub u32);
impl Ftm3 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adc1(pub u32);
impl Adc1 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flexcan2(pub u32);
impl Flexcan2 {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpspi0(pub u32);
impl Lpspi0 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpspi1(pub u32);
impl Lpspi1 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpspi2(pub u32);
impl Lpspi2 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdb1(pub u32);
impl Pdb1 {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crc(pub u32);
impl Crc {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdb0(pub u32);
impl Pdb0 {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpit(pub u32);
impl Lpit {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftm0(pub u32);
impl Ftm0 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftm1(pub u32);
impl Ftm1 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ftm2(pub u32);
impl Ftm2 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adc0(pub u32);
impl Adc0 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rtc(pub u32);
impl Rtc {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lptmr0(pub u32);
impl Lptmr0 {
#[doc="Peripheral Clock Divider Select"]
   #[inline] pub fn pcd(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Peripheral Clock Divider Select"]
   #[inline] pub fn set_pcd<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Peripheral Clock Divider Fraction"]
   #[inline] pub fn frac(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Peripheral Clock Divider Fraction"]
   #[inline] pub fn set_frac<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Porta(pub u32);
impl Porta {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Portb(pub u32);
impl Portb {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Portc(pub u32);
impl Portc {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Portd(pub u32);
impl Portd {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Porte(pub u32);
impl Porte {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Flexio(pub u32);
impl Flexio {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ewm(pub u32);
impl Ewm {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpi2c0(pub u32);
impl Lpi2c0 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpuart0(pub u32);
impl Lpuart0 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpuart1(pub u32);
impl Lpuart1 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lpuart2(pub u32);
impl Lpuart2 {
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn pcs(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
   }
#[doc="Peripheral Clock Source Select"]
   #[inline] pub fn set_pcs<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmp0(pub u32);
impl Cmp0 {
#[doc="Clock Gate Control"]
   #[inline] pub fn cgc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Clock Gate Control"]
   #[inline] pub fn set_cgc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Present"]
   #[inline] pub fn pr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Present"]
   #[inline] pub fn set_pr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
   #[inline] fn cgc(&self) -> u32 { PCC.flexcan0().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_flexcan0(|r| r.set_cgc(value)); }
}

impl Cgc for super::flexcan::Can1 {
   #[inline] fn cgc(&self) -> u32 { PCC.flexcan1().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_flexcan1(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm3 {
   #[inline] fn pcs(&self) -> u32 { PCC.ftm3().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm3(|r| r.set_pcs(value)); }
}

impl Pcs for super::adc::Adc1 {
   #[inline] fn pcs(&self) -> u32 { PCC.adc1().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_adc1(|r| r.set_pcs(value)); }
}

impl Cgc for super::adc::Adc1 {
   #[inline] fn cgc(&self) -> u32 { PCC.adc1().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_adc1(|r| r.set_cgc(value)); }
}

impl Cgc for super::flexcan::Can2 {
   #[inline] fn cgc(&self) -> u32 { PCC.flexcan2().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_flexcan2(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi0 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpspi0().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpspi0(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi0 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpspi0().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpspi0(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi1 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpspi1().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpspi1(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi1 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpspi1().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpspi1(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpspi::Lpspi2 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpspi2().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpspi2(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpspi::Lpspi2 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpspi2().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpspi2(|r| r.set_cgc(value)); }
}

impl Cgc for super::crc::Crc {
   #[inline] fn cgc(&self) -> u32 { PCC.crc().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_crc(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpit::Lpit0 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpit().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpit(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpit::Lpit0 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpit().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpit(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm0 {
   #[inline] fn pcs(&self) -> u32 { PCC.ftm0().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm0(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm0 {
   #[inline] fn cgc(&self) -> u32 { PCC.ftm0().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_ftm0(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm1 {
   #[inline] fn pcs(&self) -> u32 { PCC.ftm1().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm1(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm1 {
   #[inline] fn cgc(&self) -> u32 { PCC.ftm1().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_ftm1(|r| r.set_cgc(value)); }
}

impl Pcs for super::ftm::Ftm2 {
   #[inline] fn pcs(&self) -> u32 { PCC.ftm2().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_ftm2(|r| r.set_pcs(value)); }
}

impl Cgc for super::ftm::Ftm2 {
   #[inline] fn cgc(&self) -> u32 { PCC.ftm2().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_ftm2(|r| r.set_cgc(value)); }
}

impl Pcs for super::adc::Adc0 {
   #[inline] fn pcs(&self) -> u32 { PCC.adc0().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_adc0(|r| r.set_pcs(value)); }
}

impl Cgc for super::adc::Adc0 {
   #[inline] fn cgc(&self) -> u32 { PCC.adc0().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_adc0(|r| r.set_cgc(value)); }
}

impl Cgc for super::rtc::Rtc {
   #[inline] fn cgc(&self) -> u32 { PCC.rtc().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_rtc(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Porta {
   #[inline] fn cgc(&self) -> u32 { PCC.porta().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_porta(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portb {
   #[inline] fn cgc(&self) -> u32 { PCC.portb().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_portb(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portc {
   #[inline] fn cgc(&self) -> u32 { PCC.portc().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_portc(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Portd {
   #[inline] fn cgc(&self) -> u32 { PCC.portd().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_portd(|r| r.set_cgc(value)); }
}

impl Cgc for super::port::Porte {
   #[inline] fn cgc(&self) -> u32 { PCC.porte().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_porte(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart0 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpuart0().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpuart0(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart0 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpuart0().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpuart0(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart1 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpuart1().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpuart1(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart1 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpuart1().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpuart1(|r| r.set_cgc(value)); }
}

impl Pcs for super::lpuart::Lpuart2 {
   #[inline] fn pcs(&self) -> u32 { PCC.lpuart2().pcs().into() }
   #[inline] fn set_pcs(&self, value: u32) { PCC.with_lpuart2(|r| r.set_pcs(value)); }
}

impl Cgc for super::lpuart::Lpuart2 {
   #[inline] fn cgc(&self) -> u32 { PCC.lpuart2().cgc().into() }
   #[inline] fn set_cgc(&self, value: u32) { PCC.with_lpuart2(|r| r.set_cgc(value)); }
}


