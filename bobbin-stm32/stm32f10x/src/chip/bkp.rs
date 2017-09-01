//! Backup registers
#[allow(unused_imports)] use bobbin_common::*;

periph!(BKP, Bkp, 0x40006c04);

#[doc="Backup registers"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bkp(pub usize);
impl Bkp {
   #[doc="Get the *const pointer for the DR1 register."]
   #[inline] pub fn dr1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }

   #[doc="Get the *mut pointer for the DR1 register."]
   #[inline] pub fn dr1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }

   #[doc="Read the DR1 register."]
   #[inline] pub fn dr1(&self) -> Dr1 { 
      unsafe {
         Dr1(read_volatile((self.0 + 0x0) as *const u32))
      }
   }

   #[doc="Write the DR1 register."]
   #[inline] pub fn set_dr1<F: FnOnce(Dr1) -> Dr1>(&self, f: F) -> &Self {
      let value = f(Dr1(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR1 register."]
   #[inline] pub fn with_dr1<F: FnOnce(Dr1) -> Dr1>(&self, f: F) -> &Self {
      let tmp = self.dr1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR2 register."]
   #[inline] pub fn dr2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }

   #[doc="Get the *mut pointer for the DR2 register."]
   #[inline] pub fn dr2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }

   #[doc="Read the DR2 register."]
   #[inline] pub fn dr2(&self) -> Dr2 { 
      unsafe {
         Dr2(read_volatile((self.0 + 0x4) as *const u32))
      }
   }

   #[doc="Write the DR2 register."]
   #[inline] pub fn set_dr2<F: FnOnce(Dr2) -> Dr2>(&self, f: F) -> &Self {
      let value = f(Dr2(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR2 register."]
   #[inline] pub fn with_dr2<F: FnOnce(Dr2) -> Dr2>(&self, f: F) -> &Self {
      let tmp = self.dr2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR3 register."]
   #[inline] pub fn dr3_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }

   #[doc="Get the *mut pointer for the DR3 register."]
   #[inline] pub fn dr3_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }

   #[doc="Read the DR3 register."]
   #[inline] pub fn dr3(&self) -> Dr3 { 
      unsafe {
         Dr3(read_volatile((self.0 + 0x8) as *const u32))
      }
   }

   #[doc="Write the DR3 register."]
   #[inline] pub fn set_dr3<F: FnOnce(Dr3) -> Dr3>(&self, f: F) -> &Self {
      let value = f(Dr3(0));
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR3 register."]
   #[inline] pub fn with_dr3<F: FnOnce(Dr3) -> Dr3>(&self, f: F) -> &Self {
      let tmp = self.dr3();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR4 register."]
   #[inline] pub fn dr4_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc) as *const u32
   }

   #[doc="Get the *mut pointer for the DR4 register."]
   #[inline] pub fn dr4_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc) as *mut u32
   }

   #[doc="Read the DR4 register."]
   #[inline] pub fn dr4(&self) -> Dr4 { 
      unsafe {
         Dr4(read_volatile((self.0 + 0xc) as *const u32))
      }
   }

   #[doc="Write the DR4 register."]
   #[inline] pub fn set_dr4<F: FnOnce(Dr4) -> Dr4>(&self, f: F) -> &Self {
      let value = f(Dr4(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR4 register."]
   #[inline] pub fn with_dr4<F: FnOnce(Dr4) -> Dr4>(&self, f: F) -> &Self {
      let tmp = self.dr4();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR5 register."]
   #[inline] pub fn dr5_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x10) as *const u32
   }

   #[doc="Get the *mut pointer for the DR5 register."]
   #[inline] pub fn dr5_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x10) as *mut u32
   }

   #[doc="Read the DR5 register."]
   #[inline] pub fn dr5(&self) -> Dr5 { 
      unsafe {
         Dr5(read_volatile((self.0 + 0x10) as *const u32))
      }
   }

   #[doc="Write the DR5 register."]
   #[inline] pub fn set_dr5<F: FnOnce(Dr5) -> Dr5>(&self, f: F) -> &Self {
      let value = f(Dr5(0));
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR5 register."]
   #[inline] pub fn with_dr5<F: FnOnce(Dr5) -> Dr5>(&self, f: F) -> &Self {
      let tmp = self.dr5();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR6 register."]
   #[inline] pub fn dr6_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x14) as *const u32
   }

   #[doc="Get the *mut pointer for the DR6 register."]
   #[inline] pub fn dr6_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x14) as *mut u32
   }

   #[doc="Read the DR6 register."]
   #[inline] pub fn dr6(&self) -> Dr6 { 
      unsafe {
         Dr6(read_volatile((self.0 + 0x14) as *const u32))
      }
   }

   #[doc="Write the DR6 register."]
   #[inline] pub fn set_dr6<F: FnOnce(Dr6) -> Dr6>(&self, f: F) -> &Self {
      let value = f(Dr6(0));
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR6 register."]
   #[inline] pub fn with_dr6<F: FnOnce(Dr6) -> Dr6>(&self, f: F) -> &Self {
      let tmp = self.dr6();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR7 register."]
   #[inline] pub fn dr7_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x18) as *const u32
   }

   #[doc="Get the *mut pointer for the DR7 register."]
   #[inline] pub fn dr7_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x18) as *mut u32
   }

   #[doc="Read the DR7 register."]
   #[inline] pub fn dr7(&self) -> Dr7 { 
      unsafe {
         Dr7(read_volatile((self.0 + 0x18) as *const u32))
      }
   }

   #[doc="Write the DR7 register."]
   #[inline] pub fn set_dr7<F: FnOnce(Dr7) -> Dr7>(&self, f: F) -> &Self {
      let value = f(Dr7(0));
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR7 register."]
   #[inline] pub fn with_dr7<F: FnOnce(Dr7) -> Dr7>(&self, f: F) -> &Self {
      let tmp = self.dr7();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR8 register."]
   #[inline] pub fn dr8_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x1c) as *const u32
   }

   #[doc="Get the *mut pointer for the DR8 register."]
   #[inline] pub fn dr8_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x1c) as *mut u32
   }

   #[doc="Read the DR8 register."]
   #[inline] pub fn dr8(&self) -> Dr8 { 
      unsafe {
         Dr8(read_volatile((self.0 + 0x1c) as *const u32))
      }
   }

   #[doc="Write the DR8 register."]
   #[inline] pub fn set_dr8<F: FnOnce(Dr8) -> Dr8>(&self, f: F) -> &Self {
      let value = f(Dr8(0));
      unsafe {
         write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR8 register."]
   #[inline] pub fn with_dr8<F: FnOnce(Dr8) -> Dr8>(&self, f: F) -> &Self {
      let tmp = self.dr8();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR9 register."]
   #[inline] pub fn dr9_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x20) as *const u32
   }

   #[doc="Get the *mut pointer for the DR9 register."]
   #[inline] pub fn dr9_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x20) as *mut u32
   }

   #[doc="Read the DR9 register."]
   #[inline] pub fn dr9(&self) -> Dr9 { 
      unsafe {
         Dr9(read_volatile((self.0 + 0x20) as *const u32))
      }
   }

   #[doc="Write the DR9 register."]
   #[inline] pub fn set_dr9<F: FnOnce(Dr9) -> Dr9>(&self, f: F) -> &Self {
      let value = f(Dr9(0));
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR9 register."]
   #[inline] pub fn with_dr9<F: FnOnce(Dr9) -> Dr9>(&self, f: F) -> &Self {
      let tmp = self.dr9();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR10 register."]
   #[inline] pub fn dr10_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x24) as *const u32
   }

   #[doc="Get the *mut pointer for the DR10 register."]
   #[inline] pub fn dr10_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x24) as *mut u32
   }

   #[doc="Read the DR10 register."]
   #[inline] pub fn dr10(&self) -> Dr10 { 
      unsafe {
         Dr10(read_volatile((self.0 + 0x24) as *const u32))
      }
   }

   #[doc="Write the DR10 register."]
   #[inline] pub fn set_dr10<F: FnOnce(Dr10) -> Dr10>(&self, f: F) -> &Self {
      let value = f(Dr10(0));
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR10 register."]
   #[inline] pub fn with_dr10<F: FnOnce(Dr10) -> Dr10>(&self, f: F) -> &Self {
      let tmp = self.dr10();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR11 register."]
   #[inline] pub fn dr11_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x3c) as *const u32
   }

   #[doc="Get the *mut pointer for the DR11 register."]
   #[inline] pub fn dr11_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x3c) as *mut u32
   }

   #[doc="Read the DR11 register."]
   #[inline] pub fn dr11(&self) -> Dr11 { 
      unsafe {
         Dr11(read_volatile((self.0 + 0x3c) as *const u32))
      }
   }

   #[doc="Write the DR11 register."]
   #[inline] pub fn set_dr11<F: FnOnce(Dr11) -> Dr11>(&self, f: F) -> &Self {
      let value = f(Dr11(0));
      unsafe {
         write_volatile((self.0 + 0x3c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR11 register."]
   #[inline] pub fn with_dr11<F: FnOnce(Dr11) -> Dr11>(&self, f: F) -> &Self {
      let tmp = self.dr11();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x3c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR12 register."]
   #[inline] pub fn dr12_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x40) as *const u32
   }

   #[doc="Get the *mut pointer for the DR12 register."]
   #[inline] pub fn dr12_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x40) as *mut u32
   }

   #[doc="Read the DR12 register."]
   #[inline] pub fn dr12(&self) -> Dr12 { 
      unsafe {
         Dr12(read_volatile((self.0 + 0x40) as *const u32))
      }
   }

   #[doc="Write the DR12 register."]
   #[inline] pub fn set_dr12<F: FnOnce(Dr12) -> Dr12>(&self, f: F) -> &Self {
      let value = f(Dr12(0));
      unsafe {
         write_volatile((self.0 + 0x40) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR12 register."]
   #[inline] pub fn with_dr12<F: FnOnce(Dr12) -> Dr12>(&self, f: F) -> &Self {
      let tmp = self.dr12();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x40) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR13 register."]
   #[inline] pub fn dr13_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x44) as *const u32
   }

   #[doc="Get the *mut pointer for the DR13 register."]
   #[inline] pub fn dr13_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x44) as *mut u32
   }

   #[doc="Read the DR13 register."]
   #[inline] pub fn dr13(&self) -> Dr13 { 
      unsafe {
         Dr13(read_volatile((self.0 + 0x44) as *const u32))
      }
   }

   #[doc="Write the DR13 register."]
   #[inline] pub fn set_dr13<F: FnOnce(Dr13) -> Dr13>(&self, f: F) -> &Self {
      let value = f(Dr13(0));
      unsafe {
         write_volatile((self.0 + 0x44) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR13 register."]
   #[inline] pub fn with_dr13<F: FnOnce(Dr13) -> Dr13>(&self, f: F) -> &Self {
      let tmp = self.dr13();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x44) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR14 register."]
   #[inline] pub fn dr14_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x48) as *const u32
   }

   #[doc="Get the *mut pointer for the DR14 register."]
   #[inline] pub fn dr14_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x48) as *mut u32
   }

   #[doc="Read the DR14 register."]
   #[inline] pub fn dr14(&self) -> Dr14 { 
      unsafe {
         Dr14(read_volatile((self.0 + 0x48) as *const u32))
      }
   }

   #[doc="Write the DR14 register."]
   #[inline] pub fn set_dr14<F: FnOnce(Dr14) -> Dr14>(&self, f: F) -> &Self {
      let value = f(Dr14(0));
      unsafe {
         write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR14 register."]
   #[inline] pub fn with_dr14<F: FnOnce(Dr14) -> Dr14>(&self, f: F) -> &Self {
      let tmp = self.dr14();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x48) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR15 register."]
   #[inline] pub fn dr15_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4c) as *const u32
   }

   #[doc="Get the *mut pointer for the DR15 register."]
   #[inline] pub fn dr15_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4c) as *mut u32
   }

   #[doc="Read the DR15 register."]
   #[inline] pub fn dr15(&self) -> Dr15 { 
      unsafe {
         Dr15(read_volatile((self.0 + 0x4c) as *const u32))
      }
   }

   #[doc="Write the DR15 register."]
   #[inline] pub fn set_dr15<F: FnOnce(Dr15) -> Dr15>(&self, f: F) -> &Self {
      let value = f(Dr15(0));
      unsafe {
         write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR15 register."]
   #[inline] pub fn with_dr15<F: FnOnce(Dr15) -> Dr15>(&self, f: F) -> &Self {
      let tmp = self.dr15();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x4c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR16 register."]
   #[inline] pub fn dr16_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x50) as *const u32
   }

   #[doc="Get the *mut pointer for the DR16 register."]
   #[inline] pub fn dr16_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x50) as *mut u32
   }

   #[doc="Read the DR16 register."]
   #[inline] pub fn dr16(&self) -> Dr16 { 
      unsafe {
         Dr16(read_volatile((self.0 + 0x50) as *const u32))
      }
   }

   #[doc="Write the DR16 register."]
   #[inline] pub fn set_dr16<F: FnOnce(Dr16) -> Dr16>(&self, f: F) -> &Self {
      let value = f(Dr16(0));
      unsafe {
         write_volatile((self.0 + 0x50) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR16 register."]
   #[inline] pub fn with_dr16<F: FnOnce(Dr16) -> Dr16>(&self, f: F) -> &Self {
      let tmp = self.dr16();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x50) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR17 register."]
   #[inline] pub fn dr17_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x54) as *const u32
   }

   #[doc="Get the *mut pointer for the DR17 register."]
   #[inline] pub fn dr17_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x54) as *mut u32
   }

   #[doc="Read the DR17 register."]
   #[inline] pub fn dr17(&self) -> Dr17 { 
      unsafe {
         Dr17(read_volatile((self.0 + 0x54) as *const u32))
      }
   }

   #[doc="Write the DR17 register."]
   #[inline] pub fn set_dr17<F: FnOnce(Dr17) -> Dr17>(&self, f: F) -> &Self {
      let value = f(Dr17(0));
      unsafe {
         write_volatile((self.0 + 0x54) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR17 register."]
   #[inline] pub fn with_dr17<F: FnOnce(Dr17) -> Dr17>(&self, f: F) -> &Self {
      let tmp = self.dr17();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x54) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR18 register."]
   #[inline] pub fn dr18_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x58) as *const u32
   }

   #[doc="Get the *mut pointer for the DR18 register."]
   #[inline] pub fn dr18_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x58) as *mut u32
   }

   #[doc="Read the DR18 register."]
   #[inline] pub fn dr18(&self) -> Dr18 { 
      unsafe {
         Dr18(read_volatile((self.0 + 0x58) as *const u32))
      }
   }

   #[doc="Write the DR18 register."]
   #[inline] pub fn set_dr18<F: FnOnce(Dr18) -> Dr18>(&self, f: F) -> &Self {
      let value = f(Dr18(0));
      unsafe {
         write_volatile((self.0 + 0x58) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR18 register."]
   #[inline] pub fn with_dr18<F: FnOnce(Dr18) -> Dr18>(&self, f: F) -> &Self {
      let tmp = self.dr18();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x58) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR19 register."]
   #[inline] pub fn dr19_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x5c) as *const u32
   }

   #[doc="Get the *mut pointer for the DR19 register."]
   #[inline] pub fn dr19_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x5c) as *mut u32
   }

   #[doc="Read the DR19 register."]
   #[inline] pub fn dr19(&self) -> Dr19 { 
      unsafe {
         Dr19(read_volatile((self.0 + 0x5c) as *const u32))
      }
   }

   #[doc="Write the DR19 register."]
   #[inline] pub fn set_dr19<F: FnOnce(Dr19) -> Dr19>(&self, f: F) -> &Self {
      let value = f(Dr19(0));
      unsafe {
         write_volatile((self.0 + 0x5c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR19 register."]
   #[inline] pub fn with_dr19<F: FnOnce(Dr19) -> Dr19>(&self, f: F) -> &Self {
      let tmp = self.dr19();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x5c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR20 register."]
   #[inline] pub fn dr20_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x60) as *const u32
   }

   #[doc="Get the *mut pointer for the DR20 register."]
   #[inline] pub fn dr20_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x60) as *mut u32
   }

   #[doc="Read the DR20 register."]
   #[inline] pub fn dr20(&self) -> Dr20 { 
      unsafe {
         Dr20(read_volatile((self.0 + 0x60) as *const u32))
      }
   }

   #[doc="Write the DR20 register."]
   #[inline] pub fn set_dr20<F: FnOnce(Dr20) -> Dr20>(&self, f: F) -> &Self {
      let value = f(Dr20(0));
      unsafe {
         write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR20 register."]
   #[inline] pub fn with_dr20<F: FnOnce(Dr20) -> Dr20>(&self, f: F) -> &Self {
      let tmp = self.dr20();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x60) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR21 register."]
   #[inline] pub fn dr21_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x64) as *const u32
   }

   #[doc="Get the *mut pointer for the DR21 register."]
   #[inline] pub fn dr21_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x64) as *mut u32
   }

   #[doc="Read the DR21 register."]
   #[inline] pub fn dr21(&self) -> Dr21 { 
      unsafe {
         Dr21(read_volatile((self.0 + 0x64) as *const u32))
      }
   }

   #[doc="Write the DR21 register."]
   #[inline] pub fn set_dr21<F: FnOnce(Dr21) -> Dr21>(&self, f: F) -> &Self {
      let value = f(Dr21(0));
      unsafe {
         write_volatile((self.0 + 0x64) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR21 register."]
   #[inline] pub fn with_dr21<F: FnOnce(Dr21) -> Dr21>(&self, f: F) -> &Self {
      let tmp = self.dr21();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x64) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR22 register."]
   #[inline] pub fn dr22_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x68) as *const u32
   }

   #[doc="Get the *mut pointer for the DR22 register."]
   #[inline] pub fn dr22_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x68) as *mut u32
   }

   #[doc="Read the DR22 register."]
   #[inline] pub fn dr22(&self) -> Dr22 { 
      unsafe {
         Dr22(read_volatile((self.0 + 0x68) as *const u32))
      }
   }

   #[doc="Write the DR22 register."]
   #[inline] pub fn set_dr22<F: FnOnce(Dr22) -> Dr22>(&self, f: F) -> &Self {
      let value = f(Dr22(0));
      unsafe {
         write_volatile((self.0 + 0x68) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR22 register."]
   #[inline] pub fn with_dr22<F: FnOnce(Dr22) -> Dr22>(&self, f: F) -> &Self {
      let tmp = self.dr22();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x68) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR23 register."]
   #[inline] pub fn dr23_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x6c) as *const u32
   }

   #[doc="Get the *mut pointer for the DR23 register."]
   #[inline] pub fn dr23_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x6c) as *mut u32
   }

   #[doc="Read the DR23 register."]
   #[inline] pub fn dr23(&self) -> Dr23 { 
      unsafe {
         Dr23(read_volatile((self.0 + 0x6c) as *const u32))
      }
   }

   #[doc="Write the DR23 register."]
   #[inline] pub fn set_dr23<F: FnOnce(Dr23) -> Dr23>(&self, f: F) -> &Self {
      let value = f(Dr23(0));
      unsafe {
         write_volatile((self.0 + 0x6c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR23 register."]
   #[inline] pub fn with_dr23<F: FnOnce(Dr23) -> Dr23>(&self, f: F) -> &Self {
      let tmp = self.dr23();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x6c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR24 register."]
   #[inline] pub fn dr24_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x70) as *const u32
   }

   #[doc="Get the *mut pointer for the DR24 register."]
   #[inline] pub fn dr24_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x70) as *mut u32
   }

   #[doc="Read the DR24 register."]
   #[inline] pub fn dr24(&self) -> Dr24 { 
      unsafe {
         Dr24(read_volatile((self.0 + 0x70) as *const u32))
      }
   }

   #[doc="Write the DR24 register."]
   #[inline] pub fn set_dr24<F: FnOnce(Dr24) -> Dr24>(&self, f: F) -> &Self {
      let value = f(Dr24(0));
      unsafe {
         write_volatile((self.0 + 0x70) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR24 register."]
   #[inline] pub fn with_dr24<F: FnOnce(Dr24) -> Dr24>(&self, f: F) -> &Self {
      let tmp = self.dr24();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x70) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR25 register."]
   #[inline] pub fn dr25_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x74) as *const u32
   }

   #[doc="Get the *mut pointer for the DR25 register."]
   #[inline] pub fn dr25_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x74) as *mut u32
   }

   #[doc="Read the DR25 register."]
   #[inline] pub fn dr25(&self) -> Dr25 { 
      unsafe {
         Dr25(read_volatile((self.0 + 0x74) as *const u32))
      }
   }

   #[doc="Write the DR25 register."]
   #[inline] pub fn set_dr25<F: FnOnce(Dr25) -> Dr25>(&self, f: F) -> &Self {
      let value = f(Dr25(0));
      unsafe {
         write_volatile((self.0 + 0x74) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR25 register."]
   #[inline] pub fn with_dr25<F: FnOnce(Dr25) -> Dr25>(&self, f: F) -> &Self {
      let tmp = self.dr25();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x74) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR26 register."]
   #[inline] pub fn dr26_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x78) as *const u32
   }

   #[doc="Get the *mut pointer for the DR26 register."]
   #[inline] pub fn dr26_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x78) as *mut u32
   }

   #[doc="Read the DR26 register."]
   #[inline] pub fn dr26(&self) -> Dr26 { 
      unsafe {
         Dr26(read_volatile((self.0 + 0x78) as *const u32))
      }
   }

   #[doc="Write the DR26 register."]
   #[inline] pub fn set_dr26<F: FnOnce(Dr26) -> Dr26>(&self, f: F) -> &Self {
      let value = f(Dr26(0));
      unsafe {
         write_volatile((self.0 + 0x78) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR26 register."]
   #[inline] pub fn with_dr26<F: FnOnce(Dr26) -> Dr26>(&self, f: F) -> &Self {
      let tmp = self.dr26();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x78) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR27 register."]
   #[inline] pub fn dr27_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x7c) as *const u32
   }

   #[doc="Get the *mut pointer for the DR27 register."]
   #[inline] pub fn dr27_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x7c) as *mut u32
   }

   #[doc="Read the DR27 register."]
   #[inline] pub fn dr27(&self) -> Dr27 { 
      unsafe {
         Dr27(read_volatile((self.0 + 0x7c) as *const u32))
      }
   }

   #[doc="Write the DR27 register."]
   #[inline] pub fn set_dr27<F: FnOnce(Dr27) -> Dr27>(&self, f: F) -> &Self {
      let value = f(Dr27(0));
      unsafe {
         write_volatile((self.0 + 0x7c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR27 register."]
   #[inline] pub fn with_dr27<F: FnOnce(Dr27) -> Dr27>(&self, f: F) -> &Self {
      let tmp = self.dr27();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x7c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR28 register."]
   #[inline] pub fn dr28_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x80) as *const u32
   }

   #[doc="Get the *mut pointer for the DR28 register."]
   #[inline] pub fn dr28_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x80) as *mut u32
   }

   #[doc="Read the DR28 register."]
   #[inline] pub fn dr28(&self) -> Dr28 { 
      unsafe {
         Dr28(read_volatile((self.0 + 0x80) as *const u32))
      }
   }

   #[doc="Write the DR28 register."]
   #[inline] pub fn set_dr28<F: FnOnce(Dr28) -> Dr28>(&self, f: F) -> &Self {
      let value = f(Dr28(0));
      unsafe {
         write_volatile((self.0 + 0x80) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR28 register."]
   #[inline] pub fn with_dr28<F: FnOnce(Dr28) -> Dr28>(&self, f: F) -> &Self {
      let tmp = self.dr28();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x80) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR29 register."]
   #[inline] pub fn dr29_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x84) as *const u32
   }

   #[doc="Get the *mut pointer for the DR29 register."]
   #[inline] pub fn dr29_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x84) as *mut u32
   }

   #[doc="Read the DR29 register."]
   #[inline] pub fn dr29(&self) -> Dr29 { 
      unsafe {
         Dr29(read_volatile((self.0 + 0x84) as *const u32))
      }
   }

   #[doc="Write the DR29 register."]
   #[inline] pub fn set_dr29<F: FnOnce(Dr29) -> Dr29>(&self, f: F) -> &Self {
      let value = f(Dr29(0));
      unsafe {
         write_volatile((self.0 + 0x84) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR29 register."]
   #[inline] pub fn with_dr29<F: FnOnce(Dr29) -> Dr29>(&self, f: F) -> &Self {
      let tmp = self.dr29();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x84) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR30 register."]
   #[inline] pub fn dr30_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x88) as *const u32
   }

   #[doc="Get the *mut pointer for the DR30 register."]
   #[inline] pub fn dr30_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x88) as *mut u32
   }

   #[doc="Read the DR30 register."]
   #[inline] pub fn dr30(&self) -> Dr30 { 
      unsafe {
         Dr30(read_volatile((self.0 + 0x88) as *const u32))
      }
   }

   #[doc="Write the DR30 register."]
   #[inline] pub fn set_dr30<F: FnOnce(Dr30) -> Dr30>(&self, f: F) -> &Self {
      let value = f(Dr30(0));
      unsafe {
         write_volatile((self.0 + 0x88) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR30 register."]
   #[inline] pub fn with_dr30<F: FnOnce(Dr30) -> Dr30>(&self, f: F) -> &Self {
      let tmp = self.dr30();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x88) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR31 register."]
   #[inline] pub fn dr31_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8c) as *const u32
   }

   #[doc="Get the *mut pointer for the DR31 register."]
   #[inline] pub fn dr31_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8c) as *mut u32
   }

   #[doc="Read the DR31 register."]
   #[inline] pub fn dr31(&self) -> Dr31 { 
      unsafe {
         Dr31(read_volatile((self.0 + 0x8c) as *const u32))
      }
   }

   #[doc="Write the DR31 register."]
   #[inline] pub fn set_dr31<F: FnOnce(Dr31) -> Dr31>(&self, f: F) -> &Self {
      let value = f(Dr31(0));
      unsafe {
         write_volatile((self.0 + 0x8c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR31 register."]
   #[inline] pub fn with_dr31<F: FnOnce(Dr31) -> Dr31>(&self, f: F) -> &Self {
      let tmp = self.dr31();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR32 register."]
   #[inline] pub fn dr32_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x90) as *const u32
   }

   #[doc="Get the *mut pointer for the DR32 register."]
   #[inline] pub fn dr32_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x90) as *mut u32
   }

   #[doc="Read the DR32 register."]
   #[inline] pub fn dr32(&self) -> Dr32 { 
      unsafe {
         Dr32(read_volatile((self.0 + 0x90) as *const u32))
      }
   }

   #[doc="Write the DR32 register."]
   #[inline] pub fn set_dr32<F: FnOnce(Dr32) -> Dr32>(&self, f: F) -> &Self {
      let value = f(Dr32(0));
      unsafe {
         write_volatile((self.0 + 0x90) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR32 register."]
   #[inline] pub fn with_dr32<F: FnOnce(Dr32) -> Dr32>(&self, f: F) -> &Self {
      let tmp = self.dr32();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x90) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR33 register."]
   #[inline] pub fn dr33_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x94) as *const u32
   }

   #[doc="Get the *mut pointer for the DR33 register."]
   #[inline] pub fn dr33_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x94) as *mut u32
   }

   #[doc="Read the DR33 register."]
   #[inline] pub fn dr33(&self) -> Dr33 { 
      unsafe {
         Dr33(read_volatile((self.0 + 0x94) as *const u32))
      }
   }

   #[doc="Write the DR33 register."]
   #[inline] pub fn set_dr33<F: FnOnce(Dr33) -> Dr33>(&self, f: F) -> &Self {
      let value = f(Dr33(0));
      unsafe {
         write_volatile((self.0 + 0x94) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR33 register."]
   #[inline] pub fn with_dr33<F: FnOnce(Dr33) -> Dr33>(&self, f: F) -> &Self {
      let tmp = self.dr33();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x94) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR34 register."]
   #[inline] pub fn dr34_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x98) as *const u32
   }

   #[doc="Get the *mut pointer for the DR34 register."]
   #[inline] pub fn dr34_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x98) as *mut u32
   }

   #[doc="Read the DR34 register."]
   #[inline] pub fn dr34(&self) -> Dr34 { 
      unsafe {
         Dr34(read_volatile((self.0 + 0x98) as *const u32))
      }
   }

   #[doc="Write the DR34 register."]
   #[inline] pub fn set_dr34<F: FnOnce(Dr34) -> Dr34>(&self, f: F) -> &Self {
      let value = f(Dr34(0));
      unsafe {
         write_volatile((self.0 + 0x98) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR34 register."]
   #[inline] pub fn with_dr34<F: FnOnce(Dr34) -> Dr34>(&self, f: F) -> &Self {
      let tmp = self.dr34();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x98) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR35 register."]
   #[inline] pub fn dr35_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x9c) as *const u32
   }

   #[doc="Get the *mut pointer for the DR35 register."]
   #[inline] pub fn dr35_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x9c) as *mut u32
   }

   #[doc="Read the DR35 register."]
   #[inline] pub fn dr35(&self) -> Dr35 { 
      unsafe {
         Dr35(read_volatile((self.0 + 0x9c) as *const u32))
      }
   }

   #[doc="Write the DR35 register."]
   #[inline] pub fn set_dr35<F: FnOnce(Dr35) -> Dr35>(&self, f: F) -> &Self {
      let value = f(Dr35(0));
      unsafe {
         write_volatile((self.0 + 0x9c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR35 register."]
   #[inline] pub fn with_dr35<F: FnOnce(Dr35) -> Dr35>(&self, f: F) -> &Self {
      let tmp = self.dr35();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x9c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR36 register."]
   #[inline] pub fn dr36_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa0) as *const u32
   }

   #[doc="Get the *mut pointer for the DR36 register."]
   #[inline] pub fn dr36_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa0) as *mut u32
   }

   #[doc="Read the DR36 register."]
   #[inline] pub fn dr36(&self) -> Dr36 { 
      unsafe {
         Dr36(read_volatile((self.0 + 0xa0) as *const u32))
      }
   }

   #[doc="Write the DR36 register."]
   #[inline] pub fn set_dr36<F: FnOnce(Dr36) -> Dr36>(&self, f: F) -> &Self {
      let value = f(Dr36(0));
      unsafe {
         write_volatile((self.0 + 0xa0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR36 register."]
   #[inline] pub fn with_dr36<F: FnOnce(Dr36) -> Dr36>(&self, f: F) -> &Self {
      let tmp = self.dr36();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR37 register."]
   #[inline] pub fn dr37_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa4) as *const u32
   }

   #[doc="Get the *mut pointer for the DR37 register."]
   #[inline] pub fn dr37_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa4) as *mut u32
   }

   #[doc="Read the DR37 register."]
   #[inline] pub fn dr37(&self) -> Dr37 { 
      unsafe {
         Dr37(read_volatile((self.0 + 0xa4) as *const u32))
      }
   }

   #[doc="Write the DR37 register."]
   #[inline] pub fn set_dr37<F: FnOnce(Dr37) -> Dr37>(&self, f: F) -> &Self {
      let value = f(Dr37(0));
      unsafe {
         write_volatile((self.0 + 0xa4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR37 register."]
   #[inline] pub fn with_dr37<F: FnOnce(Dr37) -> Dr37>(&self, f: F) -> &Self {
      let tmp = self.dr37();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR38 register."]
   #[inline] pub fn dr38_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xa8) as *const u32
   }

   #[doc="Get the *mut pointer for the DR38 register."]
   #[inline] pub fn dr38_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xa8) as *mut u32
   }

   #[doc="Read the DR38 register."]
   #[inline] pub fn dr38(&self) -> Dr38 { 
      unsafe {
         Dr38(read_volatile((self.0 + 0xa8) as *const u32))
      }
   }

   #[doc="Write the DR38 register."]
   #[inline] pub fn set_dr38<F: FnOnce(Dr38) -> Dr38>(&self, f: F) -> &Self {
      let value = f(Dr38(0));
      unsafe {
         write_volatile((self.0 + 0xa8) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR38 register."]
   #[inline] pub fn with_dr38<F: FnOnce(Dr38) -> Dr38>(&self, f: F) -> &Self {
      let tmp = self.dr38();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xa8) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR39 register."]
   #[inline] pub fn dr39_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xac) as *const u32
   }

   #[doc="Get the *mut pointer for the DR39 register."]
   #[inline] pub fn dr39_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xac) as *mut u32
   }

   #[doc="Read the DR39 register."]
   #[inline] pub fn dr39(&self) -> Dr39 { 
      unsafe {
         Dr39(read_volatile((self.0 + 0xac) as *const u32))
      }
   }

   #[doc="Write the DR39 register."]
   #[inline] pub fn set_dr39<F: FnOnce(Dr39) -> Dr39>(&self, f: F) -> &Self {
      let value = f(Dr39(0));
      unsafe {
         write_volatile((self.0 + 0xac) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR39 register."]
   #[inline] pub fn with_dr39<F: FnOnce(Dr39) -> Dr39>(&self, f: F) -> &Self {
      let tmp = self.dr39();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xac) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR40 register."]
   #[inline] pub fn dr40_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb0) as *const u32
   }

   #[doc="Get the *mut pointer for the DR40 register."]
   #[inline] pub fn dr40_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb0) as *mut u32
   }

   #[doc="Read the DR40 register."]
   #[inline] pub fn dr40(&self) -> Dr40 { 
      unsafe {
         Dr40(read_volatile((self.0 + 0xb0) as *const u32))
      }
   }

   #[doc="Write the DR40 register."]
   #[inline] pub fn set_dr40<F: FnOnce(Dr40) -> Dr40>(&self, f: F) -> &Self {
      let value = f(Dr40(0));
      unsafe {
         write_volatile((self.0 + 0xb0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR40 register."]
   #[inline] pub fn with_dr40<F: FnOnce(Dr40) -> Dr40>(&self, f: F) -> &Self {
      let tmp = self.dr40();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb0) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR41 register."]
   #[inline] pub fn dr41_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb4) as *const u32
   }

   #[doc="Get the *mut pointer for the DR41 register."]
   #[inline] pub fn dr41_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb4) as *mut u32
   }

   #[doc="Read the DR41 register."]
   #[inline] pub fn dr41(&self) -> Dr41 { 
      unsafe {
         Dr41(read_volatile((self.0 + 0xb4) as *const u32))
      }
   }

   #[doc="Write the DR41 register."]
   #[inline] pub fn set_dr41<F: FnOnce(Dr41) -> Dr41>(&self, f: F) -> &Self {
      let value = f(Dr41(0));
      unsafe {
         write_volatile((self.0 + 0xb4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR41 register."]
   #[inline] pub fn with_dr41<F: FnOnce(Dr41) -> Dr41>(&self, f: F) -> &Self {
      let tmp = self.dr41();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb4) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the DR42 register."]
   #[inline] pub fn dr42_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xb8) as *const u32
   }

   #[doc="Get the *mut pointer for the DR42 register."]
   #[inline] pub fn dr42_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xb8) as *mut u32
   }

   #[doc="Read the DR42 register."]
   #[inline] pub fn dr42(&self) -> Dr42 { 
      unsafe {
         Dr42(read_volatile((self.0 + 0xb8) as *const u32))
      }
   }

   #[doc="Write the DR42 register."]
   #[inline] pub fn set_dr42<F: FnOnce(Dr42) -> Dr42>(&self, f: F) -> &Self {
      let value = f(Dr42(0));
      unsafe {
         write_volatile((self.0 + 0xb8) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the DR42 register."]
   #[inline] pub fn with_dr42<F: FnOnce(Dr42) -> Dr42>(&self, f: F) -> &Self {
      let tmp = self.dr42();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xb8) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the RTCCR register."]
   #[inline] pub fn rtccr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x28) as *const u32
   }

   #[doc="Get the *mut pointer for the RTCCR register."]
   #[inline] pub fn rtccr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x28) as *mut u32
   }

   #[doc="Read the RTCCR register."]
   #[inline] pub fn rtccr(&self) -> Rtccr { 
      unsafe {
         Rtccr(read_volatile((self.0 + 0x28) as *const u32))
      }
   }

   #[doc="Write the RTCCR register."]
   #[inline] pub fn set_rtccr<F: FnOnce(Rtccr) -> Rtccr>(&self, f: F) -> &Self {
      let value = f(Rtccr(0));
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the RTCCR register."]
   #[inline] pub fn with_rtccr<F: FnOnce(Rtccr) -> Rtccr>(&self, f: F) -> &Self {
      let tmp = self.rtccr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the CR register."]
   #[inline] pub fn cr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x2c) as *const u32
   }

   #[doc="Get the *mut pointer for the CR register."]
   #[inline] pub fn cr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x2c) as *mut u32
   }

   #[doc="Read the CR register."]
   #[inline] pub fn cr(&self) -> Cr { 
      unsafe {
         Cr(read_volatile((self.0 + 0x2c) as *const u32))
      }
   }

   #[doc="Write the CR register."]
   #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let value = f(Cr(0));
      unsafe {
         write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CR register."]
   #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let tmp = self.cr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x2c) as *mut u32, value.0);
      }
      self
   }

   #[doc="Get the *const pointer for the CSR register."]
   #[inline] pub fn csr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x30) as *const u32
   }

   #[doc="Get the *mut pointer for the CSR register."]
   #[inline] pub fn csr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x30) as *mut u32
   }

   #[doc="Read the CSR register."]
   #[inline] pub fn csr(&self) -> Csr { 
      unsafe {
         Csr(read_volatile((self.0 + 0x30) as *const u32))
      }
   }

   #[doc="Write the CSR register."]
   #[inline] pub fn set_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
      let value = f(Csr(0));
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }

   #[doc="Modify the CSR register."]
   #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
      let tmp = self.csr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x30) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr1(pub u32);
impl Dr1 {
   #[doc="Backup data"]
   #[inline] pub fn d1(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d1(&self) -> bool {
      self.d1 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d1<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d1() != 0 { try!(write!(f, " d1=0x{:x}", self.d1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr2(pub u32);
impl Dr2 {
   #[doc="Backup data"]
   #[inline] pub fn d2(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d2(&self) -> bool {
      self.d2 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d2<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d2() != 0 { try!(write!(f, " d2=0x{:x}", self.d2()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr3(pub u32);
impl Dr3 {
   #[doc="Backup data"]
   #[inline] pub fn d3(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d3(&self) -> bool {
      self.d3 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d3<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d3() != 0 { try!(write!(f, " d3=0x{:x}", self.d3()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr4(pub u32);
impl Dr4 {
   #[doc="Backup data"]
   #[inline] pub fn d4(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d4(&self) -> bool {
      self.d4 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d4<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr4 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d4() != 0 { try!(write!(f, " d4=0x{:x}", self.d4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr5(pub u32);
impl Dr5 {
   #[doc="Backup data"]
   #[inline] pub fn d5(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d5(&self) -> bool {
      self.d5 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d5<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr5 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d5() != 0 { try!(write!(f, " d5=0x{:x}", self.d5()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr6(pub u32);
impl Dr6 {
   #[doc="Backup data"]
   #[inline] pub fn d6(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d6(&self) -> bool {
      self.d6 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d6<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr6 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d6() != 0 { try!(write!(f, " d6=0x{:x}", self.d6()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr7(pub u32);
impl Dr7 {
   #[doc="Backup data"]
   #[inline] pub fn d7(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d7(&self) -> bool {
      self.d7 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d7<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr7 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d7() != 0 { try!(write!(f, " d7=0x{:x}", self.d7()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr8(pub u32);
impl Dr8 {
   #[doc="Backup data"]
   #[inline] pub fn d8(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d8(&self) -> bool {
      self.d8 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d8<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr8 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr8 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d8() != 0 { try!(write!(f, " d8=0x{:x}", self.d8()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr9(pub u32);
impl Dr9 {
   #[doc="Backup data"]
   #[inline] pub fn d9(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d9(&self) -> bool {
      self.d9 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d9<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr9 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr9 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d9() != 0 { try!(write!(f, " d9=0x{:x}", self.d9()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr10(pub u32);
impl Dr10 {
   #[doc="Backup data"]
   #[inline] pub fn d10(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d10(&self) -> bool {
      self.d10 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d10<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr10 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr10 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d10() != 0 { try!(write!(f, " d10=0x{:x}", self.d10()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr11(pub u32);
impl Dr11 {
   #[doc="Backup data"]
   #[inline] pub fn dr11(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_dr11(&self) -> bool {
      self.dr11 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_dr11<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr11 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr11 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dr11() != 0 { try!(write!(f, " dr11=0x{:x}", self.dr11()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr12(pub u32);
impl Dr12 {
   #[doc="Backup data"]
   #[inline] pub fn dr12(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_dr12(&self) -> bool {
      self.dr12 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_dr12<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr12 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr12 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dr12() != 0 { try!(write!(f, " dr12=0x{:x}", self.dr12()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr13(pub u32);
impl Dr13 {
   #[doc="Backup data"]
   #[inline] pub fn dr13(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_dr13(&self) -> bool {
      self.dr13 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_dr13<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr13 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr13 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dr13() != 0 { try!(write!(f, " dr13=0x{:x}", self.dr13()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr14(pub u32);
impl Dr14 {
   #[doc="Backup data"]
   #[inline] pub fn d14(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d14(&self) -> bool {
      self.d14 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d14<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr14 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr14 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d14() != 0 { try!(write!(f, " d14=0x{:x}", self.d14()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr15(pub u32);
impl Dr15 {
   #[doc="Backup data"]
   #[inline] pub fn d15(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d15(&self) -> bool {
      self.d15 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d15<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr15 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr15 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d15() != 0 { try!(write!(f, " d15=0x{:x}", self.d15()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr16(pub u32);
impl Dr16 {
   #[doc="Backup data"]
   #[inline] pub fn d16(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d16(&self) -> bool {
      self.d16 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d16<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr16 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr16 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d16() != 0 { try!(write!(f, " d16=0x{:x}", self.d16()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr17(pub u32);
impl Dr17 {
   #[doc="Backup data"]
   #[inline] pub fn d17(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d17(&self) -> bool {
      self.d17 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d17<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr17 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr17 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d17() != 0 { try!(write!(f, " d17=0x{:x}", self.d17()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr18(pub u32);
impl Dr18 {
   #[doc="Backup data"]
   #[inline] pub fn d18(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d18(&self) -> bool {
      self.d18 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d18<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr18 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr18 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d18() != 0 { try!(write!(f, " d18=0x{:x}", self.d18()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr19(pub u32);
impl Dr19 {
   #[doc="Backup data"]
   #[inline] pub fn d19(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d19(&self) -> bool {
      self.d19 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d19<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr19 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr19 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d19() != 0 { try!(write!(f, " d19=0x{:x}", self.d19()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr20(pub u32);
impl Dr20 {
   #[doc="Backup data"]
   #[inline] pub fn d20(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d20(&self) -> bool {
      self.d20 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d20<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr20 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr20 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d20() != 0 { try!(write!(f, " d20=0x{:x}", self.d20()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr21(pub u32);
impl Dr21 {
   #[doc="Backup data"]
   #[inline] pub fn d21(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d21(&self) -> bool {
      self.d21 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d21<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr21 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr21 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d21() != 0 { try!(write!(f, " d21=0x{:x}", self.d21()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr22(pub u32);
impl Dr22 {
   #[doc="Backup data"]
   #[inline] pub fn d22(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d22(&self) -> bool {
      self.d22 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d22<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr22 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr22 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d22() != 0 { try!(write!(f, " d22=0x{:x}", self.d22()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr23(pub u32);
impl Dr23 {
   #[doc="Backup data"]
   #[inline] pub fn d23(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d23(&self) -> bool {
      self.d23 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d23<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr23 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr23 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d23() != 0 { try!(write!(f, " d23=0x{:x}", self.d23()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr24(pub u32);
impl Dr24 {
   #[doc="Backup data"]
   #[inline] pub fn d24(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d24(&self) -> bool {
      self.d24 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d24<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr24 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr24 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d24() != 0 { try!(write!(f, " d24=0x{:x}", self.d24()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr25(pub u32);
impl Dr25 {
   #[doc="Backup data"]
   #[inline] pub fn d25(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d25(&self) -> bool {
      self.d25 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d25<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr25 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr25 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d25() != 0 { try!(write!(f, " d25=0x{:x}", self.d25()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr26(pub u32);
impl Dr26 {
   #[doc="Backup data"]
   #[inline] pub fn d26(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d26(&self) -> bool {
      self.d26 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d26<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr26 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr26 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d26() != 0 { try!(write!(f, " d26=0x{:x}", self.d26()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr27(pub u32);
impl Dr27 {
   #[doc="Backup data"]
   #[inline] pub fn d27(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d27(&self) -> bool {
      self.d27 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d27<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr27 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr27 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d27() != 0 { try!(write!(f, " d27=0x{:x}", self.d27()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr28(pub u32);
impl Dr28 {
   #[doc="Backup data"]
   #[inline] pub fn d28(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d28(&self) -> bool {
      self.d28 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d28<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr28 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr28 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d28() != 0 { try!(write!(f, " d28=0x{:x}", self.d28()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr29(pub u32);
impl Dr29 {
   #[doc="Backup data"]
   #[inline] pub fn d29(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d29(&self) -> bool {
      self.d29 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d29<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr29 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr29 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d29() != 0 { try!(write!(f, " d29=0x{:x}", self.d29()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr30(pub u32);
impl Dr30 {
   #[doc="Backup data"]
   #[inline] pub fn d30(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d30(&self) -> bool {
      self.d30 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d30<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr30 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr30 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d30() != 0 { try!(write!(f, " d30=0x{:x}", self.d30()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr31(pub u32);
impl Dr31 {
   #[doc="Backup data"]
   #[inline] pub fn d31(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d31(&self) -> bool {
      self.d31 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d31<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr31 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr31 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d31() != 0 { try!(write!(f, " d31=0x{:x}", self.d31()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr32(pub u32);
impl Dr32 {
   #[doc="Backup data"]
   #[inline] pub fn d32(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d32(&self) -> bool {
      self.d32 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d32<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr32 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr32 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d32() != 0 { try!(write!(f, " d32=0x{:x}", self.d32()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr33(pub u32);
impl Dr33 {
   #[doc="Backup data"]
   #[inline] pub fn d33(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d33(&self) -> bool {
      self.d33 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d33<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr33 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr33 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d33() != 0 { try!(write!(f, " d33=0x{:x}", self.d33()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr34(pub u32);
impl Dr34 {
   #[doc="Backup data"]
   #[inline] pub fn d34(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d34(&self) -> bool {
      self.d34 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d34<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr34 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr34 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d34() != 0 { try!(write!(f, " d34=0x{:x}", self.d34()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr35(pub u32);
impl Dr35 {
   #[doc="Backup data"]
   #[inline] pub fn d35(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d35(&self) -> bool {
      self.d35 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d35<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr35 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr35 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d35() != 0 { try!(write!(f, " d35=0x{:x}", self.d35()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr36(pub u32);
impl Dr36 {
   #[doc="Backup data"]
   #[inline] pub fn d36(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d36(&self) -> bool {
      self.d36 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d36<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr36 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr36 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d36() != 0 { try!(write!(f, " d36=0x{:x}", self.d36()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr37(pub u32);
impl Dr37 {
   #[doc="Backup data"]
   #[inline] pub fn d37(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d37(&self) -> bool {
      self.d37 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d37<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr37 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr37 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d37() != 0 { try!(write!(f, " d37=0x{:x}", self.d37()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr38(pub u32);
impl Dr38 {
   #[doc="Backup data"]
   #[inline] pub fn d38(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d38(&self) -> bool {
      self.d38 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d38<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr38 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr38 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d38() != 0 { try!(write!(f, " d38=0x{:x}", self.d38()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr39(pub u32);
impl Dr39 {
   #[doc="Backup data"]
   #[inline] pub fn d39(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d39(&self) -> bool {
      self.d39 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d39<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr39 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr39 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d39() != 0 { try!(write!(f, " d39=0x{:x}", self.d39()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr40(pub u32);
impl Dr40 {
   #[doc="Backup data"]
   #[inline] pub fn d40(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d40(&self) -> bool {
      self.d40 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d40<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr40 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr40 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d40() != 0 { try!(write!(f, " d40=0x{:x}", self.d40()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr41(pub u32);
impl Dr41 {
   #[doc="Backup data"]
   #[inline] pub fn d41(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d41(&self) -> bool {
      self.d41 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d41<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr41 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr41 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d41() != 0 { try!(write!(f, " d41=0x{:x}", self.d41()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup data register (BKP_DR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr42(pub u32);
impl Dr42 {
   #[doc="Backup data"]
   #[inline] pub fn d42(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }

   #[doc="Backup data"]
   #[inline] pub fn test_d42(&self) -> bool {
      self.d42 != 0
   }

   #[doc="Backup data"]
   #[inline] pub fn set_d42<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}

impl ::core::fmt::Display for Dr42 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr42 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d42() != 0 { try!(write!(f, " d42=0x{:x}", self.d42()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="RTC clock calibration register (BKP_RTCCR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rtccr(pub u32);
impl Rtccr {
   #[doc="Calibration value"]
   #[inline] pub fn cal(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }

   #[doc="Calibration value"]
   #[inline] pub fn test_cal(&self) -> bool {
      self.cal != 0
   }

   #[doc="Calibration value"]
   #[inline] pub fn set_cal<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Calibration Clock Output"]
   #[inline] pub fn cco(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }

   #[doc="Calibration Clock Output"]
   #[inline] pub fn test_cco(&self) -> bool {
      self.cco != 0
   }

   #[doc="Calibration Clock Output"]
   #[inline] pub fn set_cco<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

   #[doc="Alarm or second output enable"]
   #[inline] pub fn asoe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }

   #[doc="Alarm or second output enable"]
   #[inline] pub fn test_asoe(&self) -> bool {
      self.asoe != 0
   }

   #[doc="Alarm or second output enable"]
   #[inline] pub fn set_asoe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="Alarm or second output selection"]
   #[inline] pub fn asos(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }

   #[doc="Alarm or second output selection"]
   #[inline] pub fn test_asos(&self) -> bool {
      self.asos != 0
   }

   #[doc="Alarm or second output selection"]
   #[inline] pub fn set_asos<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}

impl ::core::fmt::Display for Rtccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rtccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cal() != 0 { try!(write!(f, " cal=0x{:x}", self.cal()))}
      if self.cco() != 0 { try!(write!(f, " cco"))}
      if self.asoe() != 0 { try!(write!(f, " asoe"))}
      if self.asos() != 0 { try!(write!(f, " asos"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="Backup control register (BKP_CR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
   #[doc="Tamper pin enable"]
   #[inline] pub fn tpe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Tamper pin enable"]
   #[inline] pub fn test_tpe(&self) -> bool {
      self.tpe != 0
   }

   #[doc="Tamper pin enable"]
   #[inline] pub fn set_tpe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Tamper pin active level"]
   #[inline] pub fn tpal(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Tamper pin active level"]
   #[inline] pub fn test_tpal(&self) -> bool {
      self.tpal != 0
   }

   #[doc="Tamper pin active level"]
   #[inline] pub fn set_tpal<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}

impl ::core::fmt::Display for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tpe() != 0 { try!(write!(f, " tpe"))}
      if self.tpal() != 0 { try!(write!(f, " tpal"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="BKP_CSR control/status register (BKP_CSR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
   #[doc="Clear Tamper event"]
   #[inline] pub fn cte(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }

   #[doc="Clear Tamper event"]
   #[inline] pub fn test_cte(&self) -> bool {
      self.cte != 0
   }

   #[doc="Clear Tamper event"]
   #[inline] pub fn set_cte<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

   #[doc="Clear Tamper Interrupt"]
   #[inline] pub fn cti(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }

   #[doc="Clear Tamper Interrupt"]
   #[inline] pub fn test_cti(&self) -> bool {
      self.cti != 0
   }

   #[doc="Clear Tamper Interrupt"]
   #[inline] pub fn set_cti<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

   #[doc="Tamper Pin interrupt enable"]
   #[inline] pub fn tpie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }

   #[doc="Tamper Pin interrupt enable"]
   #[inline] pub fn test_tpie(&self) -> bool {
      self.tpie != 0
   }

   #[doc="Tamper Pin interrupt enable"]
   #[inline] pub fn set_tpie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

   #[doc="Tamper Event Flag"]
   #[inline] pub fn tef(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }

   #[doc="Tamper Event Flag"]
   #[inline] pub fn test_tef(&self) -> bool {
      self.tef != 0
   }

   #[doc="Tamper Event Flag"]
   #[inline] pub fn set_tef<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

   #[doc="Tamper Interrupt Flag"]
   #[inline] pub fn tif(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }

   #[doc="Tamper Interrupt Flag"]
   #[inline] pub fn test_tif(&self) -> bool {
      self.tif != 0
   }

   #[doc="Tamper Interrupt Flag"]
   #[inline] pub fn set_tif<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}

impl ::core::fmt::Display for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Csr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cte() != 0 { try!(write!(f, " cte"))}
      if self.cti() != 0 { try!(write!(f, " cti"))}
      if self.tpie() != 0 { try!(write!(f, " tpie"))}
      if self.tef() != 0 { try!(write!(f, " tef"))}
      if self.tif() != 0 { try!(write!(f, " tif"))}
      try!(write!(f, "]"));
      Ok(())
   }
}


