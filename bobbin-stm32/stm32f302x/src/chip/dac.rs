#[allow(unused_imports)] use bobbin_common::*;

periph!( DAC, Dac, _DAC, DacPeriph, 0x40007400);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DAC Peripheral"]
pub struct DacPeriph(pub usize); 



impl DacPeriph {
#[doc="Get the *const pointer for the CR register."]
   #[inline] pub fn cr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x0) as *const u32
   }
#[doc="Get the *mut pointer for the CR register."]
   #[inline] pub fn cr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x0) as *mut u32
   }
#[doc="Read the CR register."]
   #[inline] pub fn cr(&self) -> Cr { 
      unsafe {
         Cr(read_volatile((self.0 + 0x0) as *const u32))
      }
   }
#[doc="Write the CR register."]
   #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let value = f(Cr(0));
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR register."]
   #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let tmp = self.cr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SWTRIGR register."]
   #[inline] pub fn swtrigr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x4) as *const u32
   }
#[doc="Get the *mut pointer for the SWTRIGR register."]
   #[inline] pub fn swtrigr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x4) as *mut u32
   }
#[doc="Write the SWTRIGR register."]
   #[inline] pub fn set_swtrigr<F: FnOnce(Swtrigr) -> Swtrigr>(&self, f: F) -> &Self {
      let value = f(Swtrigr(0));
      unsafe {
         write_volatile((self.0 + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DHR12R1 register."]
   #[inline] pub fn dhr12r1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x8) as *const u32
   }
#[doc="Get the *mut pointer for the DHR12R1 register."]
   #[inline] pub fn dhr12r1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x8) as *mut u32
   }
#[doc="Read the DHR12R1 register."]
   #[inline] pub fn dhr12r1(&self) -> Dhr12r1 { 
      unsafe {
         Dhr12r1(read_volatile((self.0 + 0x8) as *const u32))
      }
   }
#[doc="Write the DHR12R1 register."]
   #[inline] pub fn set_dhr12r1<F: FnOnce(Dhr12r1) -> Dhr12r1>(&self, f: F) -> &Self {
      let value = f(Dhr12r1(0));
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DHR12R1 register."]
   #[inline] pub fn with_dhr12r1<F: FnOnce(Dhr12r1) -> Dhr12r1>(&self, f: F) -> &Self {
      let tmp = self.dhr12r1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DHR12L1 register."]
   #[inline] pub fn dhr12l1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0xc) as *const u32
   }
#[doc="Get the *mut pointer for the DHR12L1 register."]
   #[inline] pub fn dhr12l1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0xc) as *mut u32
   }
#[doc="Read the DHR12L1 register."]
   #[inline] pub fn dhr12l1(&self) -> Dhr12l1 { 
      unsafe {
         Dhr12l1(read_volatile((self.0 + 0xc) as *const u32))
      }
   }
#[doc="Write the DHR12L1 register."]
   #[inline] pub fn set_dhr12l1<F: FnOnce(Dhr12l1) -> Dhr12l1>(&self, f: F) -> &Self {
      let value = f(Dhr12l1(0));
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DHR12L1 register."]
   #[inline] pub fn with_dhr12l1<F: FnOnce(Dhr12l1) -> Dhr12l1>(&self, f: F) -> &Self {
      let tmp = self.dhr12l1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DHR8R1 register."]
   #[inline] pub fn dhr8r1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x10) as *const u32
   }
#[doc="Get the *mut pointer for the DHR8R1 register."]
   #[inline] pub fn dhr8r1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x10) as *mut u32
   }
#[doc="Read the DHR8R1 register."]
   #[inline] pub fn dhr8r1(&self) -> Dhr8r1 { 
      unsafe {
         Dhr8r1(read_volatile((self.0 + 0x10) as *const u32))
      }
   }
#[doc="Write the DHR8R1 register."]
   #[inline] pub fn set_dhr8r1<F: FnOnce(Dhr8r1) -> Dhr8r1>(&self, f: F) -> &Self {
      let value = f(Dhr8r1(0));
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DHR8R1 register."]
   #[inline] pub fn with_dhr8r1<F: FnOnce(Dhr8r1) -> Dhr8r1>(&self, f: F) -> &Self {
      let tmp = self.dhr8r1();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DHR12R2 register."]
   #[inline] pub fn dhr12r2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x14) as *const u32
   }
#[doc="Get the *mut pointer for the DHR12R2 register."]
   #[inline] pub fn dhr12r2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x14) as *mut u32
   }
#[doc="Read the DHR12R2 register."]
   #[inline] pub fn dhr12r2(&self) -> Dhr12r2 { 
      unsafe {
         Dhr12r2(read_volatile((self.0 + 0x14) as *const u32))
      }
   }
#[doc="Write the DHR12R2 register."]
   #[inline] pub fn set_dhr12r2<F: FnOnce(Dhr12r2) -> Dhr12r2>(&self, f: F) -> &Self {
      let value = f(Dhr12r2(0));
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DHR12R2 register."]
   #[inline] pub fn with_dhr12r2<F: FnOnce(Dhr12r2) -> Dhr12r2>(&self, f: F) -> &Self {
      let tmp = self.dhr12r2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DHR12L2 register."]
   #[inline] pub fn dhr12l2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x18) as *const u32
   }
#[doc="Get the *mut pointer for the DHR12L2 register."]
   #[inline] pub fn dhr12l2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x18) as *mut u32
   }
#[doc="Read the DHR12L2 register."]
   #[inline] pub fn dhr12l2(&self) -> Dhr12l2 { 
      unsafe {
         Dhr12l2(read_volatile((self.0 + 0x18) as *const u32))
      }
   }
#[doc="Write the DHR12L2 register."]
   #[inline] pub fn set_dhr12l2<F: FnOnce(Dhr12l2) -> Dhr12l2>(&self, f: F) -> &Self {
      let value = f(Dhr12l2(0));
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DHR12L2 register."]
   #[inline] pub fn with_dhr12l2<F: FnOnce(Dhr12l2) -> Dhr12l2>(&self, f: F) -> &Self {
      let tmp = self.dhr12l2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x18) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DHR8R2 register."]
   #[inline] pub fn dhr8r2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x1c) as *const u32
   }
#[doc="Get the *mut pointer for the DHR8R2 register."]
   #[inline] pub fn dhr8r2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x1c) as *mut u32
   }
#[doc="Read the DHR8R2 register."]
   #[inline] pub fn dhr8r2(&self) -> Dhr8r2 { 
      unsafe {
         Dhr8r2(read_volatile((self.0 + 0x1c) as *const u32))
      }
   }
#[doc="Write the DHR8R2 register."]
   #[inline] pub fn set_dhr8r2<F: FnOnce(Dhr8r2) -> Dhr8r2>(&self, f: F) -> &Self {
      let value = f(Dhr8r2(0));
      unsafe {
         write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DHR8R2 register."]
   #[inline] pub fn with_dhr8r2<F: FnOnce(Dhr8r2) -> Dhr8r2>(&self, f: F) -> &Self {
      let tmp = self.dhr8r2();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x1c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DHR12RD register."]
   #[inline] pub fn dhr12rd_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x20) as *const u32
   }
#[doc="Get the *mut pointer for the DHR12RD register."]
   #[inline] pub fn dhr12rd_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x20) as *mut u32
   }
#[doc="Read the DHR12RD register."]
   #[inline] pub fn dhr12rd(&self) -> Dhr12rd { 
      unsafe {
         Dhr12rd(read_volatile((self.0 + 0x20) as *const u32))
      }
   }
#[doc="Write the DHR12RD register."]
   #[inline] pub fn set_dhr12rd<F: FnOnce(Dhr12rd) -> Dhr12rd>(&self, f: F) -> &Self {
      let value = f(Dhr12rd(0));
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DHR12RD register."]
   #[inline] pub fn with_dhr12rd<F: FnOnce(Dhr12rd) -> Dhr12rd>(&self, f: F) -> &Self {
      let tmp = self.dhr12rd();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DHR12LD register."]
   #[inline] pub fn dhr12ld_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x24) as *const u32
   }
#[doc="Get the *mut pointer for the DHR12LD register."]
   #[inline] pub fn dhr12ld_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x24) as *mut u32
   }
#[doc="Read the DHR12LD register."]
   #[inline] pub fn dhr12ld(&self) -> Dhr12ld { 
      unsafe {
         Dhr12ld(read_volatile((self.0 + 0x24) as *const u32))
      }
   }
#[doc="Write the DHR12LD register."]
   #[inline] pub fn set_dhr12ld<F: FnOnce(Dhr12ld) -> Dhr12ld>(&self, f: F) -> &Self {
      let value = f(Dhr12ld(0));
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DHR12LD register."]
   #[inline] pub fn with_dhr12ld<F: FnOnce(Dhr12ld) -> Dhr12ld>(&self, f: F) -> &Self {
      let tmp = self.dhr12ld();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x24) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DHR8RD register."]
   #[inline] pub fn dhr8rd_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x28) as *const u32
   }
#[doc="Get the *mut pointer for the DHR8RD register."]
   #[inline] pub fn dhr8rd_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x28) as *mut u32
   }
#[doc="Read the DHR8RD register."]
   #[inline] pub fn dhr8rd(&self) -> Dhr8rd { 
      unsafe {
         Dhr8rd(read_volatile((self.0 + 0x28) as *const u32))
      }
   }
#[doc="Write the DHR8RD register."]
   #[inline] pub fn set_dhr8rd<F: FnOnce(Dhr8rd) -> Dhr8rd>(&self, f: F) -> &Self {
      let value = f(Dhr8rd(0));
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DHR8RD register."]
   #[inline] pub fn with_dhr8rd<F: FnOnce(Dhr8rd) -> Dhr8rd>(&self, f: F) -> &Self {
      let tmp = self.dhr8rd();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x28) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DOR1 register."]
   #[inline] pub fn dor1_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x2c) as *const u32
   }
#[doc="Get the *mut pointer for the DOR1 register."]
   #[inline] pub fn dor1_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x2c) as *mut u32
   }
#[doc="Read the DOR1 register."]
   #[inline] pub fn dor1(&self) -> Dor1 { 
      unsafe {
         Dor1(read_volatile((self.0 + 0x2c) as *const u32))
      }
   }

#[doc="Get the *const pointer for the DOR2 register."]
   #[inline] pub fn dor2_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x30) as *const u32
   }
#[doc="Get the *mut pointer for the DOR2 register."]
   #[inline] pub fn dor2_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x30) as *mut u32
   }
#[doc="Read the DOR2 register."]
   #[inline] pub fn dor2(&self) -> Dor2 { 
      unsafe {
         Dor2(read_volatile((self.0 + 0x30) as *const u32))
      }
   }

#[doc="Get the *const pointer for the SR register."]
   #[inline] pub fn sr_ptr(&self) -> *const u32 { 
      ((self.0 as usize) + 0x34) as *const u32
   }
#[doc="Get the *mut pointer for the SR register."]
   #[inline] pub fn sr_mut(&self) -> *mut u32 { 
      ((self.0 as usize) + 0x34) as *mut u32
   }
#[doc="Read the SR register."]
   #[inline] pub fn sr(&self) -> Sr { 
      unsafe {
         Sr(read_volatile((self.0 + 0x34) as *const u32))
      }
   }
#[doc="Write the SR register."]
   #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let value = f(Sr(0));
      unsafe {
         write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SR register."]
   #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let tmp = self.sr();
      let value = f(tmp);
      unsafe {
         write_volatile((self.0 + 0x34) as *mut u32, value.0);
      }
      self
   }

}

#[doc="control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="DAC channel2 DMA underrun interrupt enable"]
   #[inline] pub fn dmaudrie2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="DAC channel2 DMA underrun interrupt enable"]
   #[inline] pub fn set_dmaudrie2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="DAC channel2 DMA enable"]
   #[inline] pub fn dmaen2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="DAC channel2 DMA enable"]
   #[inline] pub fn set_dmaen2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="DAC channel2 mask/amplitude selector"]
   #[inline] pub fn mamp2(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
   }
#[doc="DAC channel2 mask/amplitude selector"]
   #[inline] pub fn set_mamp2<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 24);
      self.0 |= value << 24;
      self
   }

#[doc="DAC channel2 noise/triangle wave generation enable"]
   #[inline] pub fn wave2(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
   }
#[doc="DAC channel2 noise/triangle wave generation enable"]
   #[inline] pub fn set_wave2<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="DAC channel2 trigger selection"]
   #[inline] pub fn tsel2(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
   }
#[doc="DAC channel2 trigger selection"]
   #[inline] pub fn set_tsel2<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 19);
      self.0 |= value << 19;
      self
   }

#[doc="DAC channel2 trigger enable"]
   #[inline] pub fn ten2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="DAC channel2 trigger enable"]
   #[inline] pub fn set_ten2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="DAC channel2 output buffer disable"]
   #[inline] pub fn boff2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="DAC channel2 output buffer disable"]
   #[inline] pub fn set_boff2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="DAC channel2 enable"]
   #[inline] pub fn en2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="DAC channel2 enable"]
   #[inline] pub fn set_en2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="DAC channel1 DMA Underrun Interrupt enable"]
   #[inline] pub fn dmaudrie1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="DAC channel1 DMA Underrun Interrupt enable"]
   #[inline] pub fn set_dmaudrie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="DAC channel1 DMA enable"]
   #[inline] pub fn dmaen1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="DAC channel1 DMA enable"]
   #[inline] pub fn set_dmaen1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="DAC channel1 mask/amplitude selector"]
   #[inline] pub fn mamp1(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
   }
#[doc="DAC channel1 mask/amplitude selector"]
   #[inline] pub fn set_mamp1<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 8);
      self.0 |= value << 8;
      self
   }

#[doc="DAC channel1 noise/triangle wave generation enable"]
   #[inline] pub fn wave1(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="DAC channel1 noise/triangle wave generation enable"]
   #[inline] pub fn set_wave1<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="DAC channel1 trigger selection"]
   #[inline] pub fn tsel1(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
   }
#[doc="DAC channel1 trigger selection"]
   #[inline] pub fn set_tsel1<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="DAC channel1 trigger enable"]
   #[inline] pub fn ten1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="DAC channel1 trigger enable"]
   #[inline] pub fn set_ten1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="DAC channel1 output buffer disable"]
   #[inline] pub fn boff1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="DAC channel1 output buffer disable"]
   #[inline] pub fn set_boff1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="DAC channel1 enable"]
   #[inline] pub fn en1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="DAC channel1 enable"]
   #[inline] pub fn set_en1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
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
      if self.dmaudrie2() != 0 { try!(write!(f, " dmaudrie2"))}
      if self.dmaen2() != 0 { try!(write!(f, " dmaen2"))}
      if self.mamp2() != 0 { try!(write!(f, " mamp2=0x{:x}", self.mamp2()))}
      if self.wave2() != 0 { try!(write!(f, " wave2=0x{:x}", self.wave2()))}
      if self.tsel2() != 0 { try!(write!(f, " tsel2=0x{:x}", self.tsel2()))}
      if self.ten2() != 0 { try!(write!(f, " ten2"))}
      if self.boff2() != 0 { try!(write!(f, " boff2"))}
      if self.en2() != 0 { try!(write!(f, " en2"))}
      if self.dmaudrie1() != 0 { try!(write!(f, " dmaudrie1"))}
      if self.dmaen1() != 0 { try!(write!(f, " dmaen1"))}
      if self.mamp1() != 0 { try!(write!(f, " mamp1=0x{:x}", self.mamp1()))}
      if self.wave1() != 0 { try!(write!(f, " wave1=0x{:x}", self.wave1()))}
      if self.tsel1() != 0 { try!(write!(f, " tsel1=0x{:x}", self.tsel1()))}
      if self.ten1() != 0 { try!(write!(f, " ten1"))}
      if self.boff1() != 0 { try!(write!(f, " boff1"))}
      if self.en1() != 0 { try!(write!(f, " en1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="software trigger register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Swtrigr(pub u32);
impl Swtrigr {
#[doc="DAC channel2 software trigger"]
   #[inline] pub fn swtrig2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="DAC channel2 software trigger"]
   #[inline] pub fn set_swtrig2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="DAC channel1 software trigger"]
   #[inline] pub fn swtrig1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="DAC channel1 software trigger"]
   #[inline] pub fn set_swtrig1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Swtrigr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Swtrigr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swtrig2() != 0 { try!(write!(f, " swtrig2"))}
      if self.swtrig1() != 0 { try!(write!(f, " swtrig1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="channel1 12-bit right-aligned data holding register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dhr12r1(pub u32);
impl Dhr12r1 {
#[doc="DAC channel1 12-bit right-aligned data"]
   #[inline] pub fn dacc1dhr(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="DAC channel1 12-bit right-aligned data"]
   #[inline] pub fn set_dacc1dhr<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dhr12r1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhr12r1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="channel1 12-bit left aligned data holding register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dhr12l1(pub u32);
impl Dhr12l1 {
#[doc="DAC channel1 12-bit left-aligned data"]
   #[inline] pub fn dacc1dhr(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
   }
#[doc="DAC channel1 12-bit left-aligned data"]
   #[inline] pub fn set_dacc1dhr<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Dhr12l1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhr12l1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="channel1 8-bit right aligned data holding register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dhr8r1(pub u32);
impl Dhr8r1 {
#[doc="DAC channel1 8-bit right-aligned data"]
   #[inline] pub fn dacc1dhr(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="DAC channel1 8-bit right-aligned data"]
   #[inline] pub fn set_dacc1dhr<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dhr8r1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhr8r1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="channel2 12-bit right aligned data holding register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dhr12r2(pub u32);
impl Dhr12r2 {
#[doc="DAC channel2 12-bit right-aligned data"]
   #[inline] pub fn dacc2dhr(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="DAC channel2 12-bit right-aligned data"]
   #[inline] pub fn set_dacc2dhr<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dhr12r2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhr12r2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="channel2 12-bit left aligned data holding register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dhr12l2(pub u32);
impl Dhr12l2 {
#[doc="DAC channel2 12-bit left-aligned data"]
   #[inline] pub fn dacc2dhr(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
   }
#[doc="DAC channel2 12-bit left-aligned data"]
   #[inline] pub fn set_dacc2dhr<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Dhr12l2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhr12l2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="channel2 8-bit right-aligned data holding register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dhr8r2(pub u32);
impl Dhr8r2 {
#[doc="DAC channel2 8-bit right-aligned data"]
   #[inline] pub fn dacc2dhr(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="DAC channel2 8-bit right-aligned data"]
   #[inline] pub fn set_dacc2dhr<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dhr8r2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhr8r2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Dual DAC 12-bit right-aligned data holding register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dhr12rd(pub u32);
impl Dhr12rd {
#[doc="DAC channel2 12-bit right-aligned data"]
   #[inline] pub fn dacc2dhr(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
   }
#[doc="DAC channel2 12-bit right-aligned data"]
   #[inline] pub fn set_dacc2dhr<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="DAC channel1 12-bit right-aligned data"]
   #[inline] pub fn dacc1dhr(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="DAC channel1 12-bit right-aligned data"]
   #[inline] pub fn set_dacc1dhr<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dhr12rd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhr12rd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DUAL DAC 12-bit left aligned data holding register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dhr12ld(pub u32);
impl Dhr12ld {
#[doc="DAC channel2 12-bit left-aligned data"]
   #[inline] pub fn dacc2dhr(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xfff) as u16) } // [31:20]
   }
#[doc="DAC channel2 12-bit left-aligned data"]
   #[inline] pub fn set_dacc2dhr<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 20);
      self.0 |= value << 20;
      self
   }

#[doc="DAC channel1 12-bit left-aligned data"]
   #[inline] pub fn dacc1dhr(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
   }
#[doc="DAC channel1 12-bit left-aligned data"]
   #[inline] pub fn set_dacc1dhr<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Dhr12ld {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhr12ld {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DUAL DAC 8-bit right aligned data holding register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dhr8rd(pub u32);
impl Dhr8rd {
#[doc="DAC channel2 8-bit right-aligned data"]
   #[inline] pub fn dacc2dhr(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="DAC channel2 8-bit right-aligned data"]
   #[inline] pub fn set_dacc2dhr<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="DAC channel1 8-bit right-aligned data"]
   #[inline] pub fn dacc1dhr(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="DAC channel1 8-bit right-aligned data"]
   #[inline] pub fn set_dacc1dhr<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dhr8rd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dhr8rd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
      if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="channel1 data output register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dor1(pub u32);
impl Dor1 {
#[doc="DAC channel1 data output"]
   #[inline] pub fn dacc1dor(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="DAC channel1 data output"]
   #[inline] pub fn set_dacc1dor<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dor1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dor1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc1dor() != 0 { try!(write!(f, " dacc1dor=0x{:x}", self.dacc1dor()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="channel2 data output register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dor2(pub u32);
impl Dor2 {
#[doc="DAC channel2 data output"]
   #[inline] pub fn dacc2dor(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="DAC channel2 data output"]
   #[inline] pub fn set_dacc2dor<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dor2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dor2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dacc2dor() != 0 { try!(write!(f, " dacc2dor=0x{:x}", self.dacc2dor()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="DAC channel2 DMA underrun flag"]
   #[inline] pub fn dmaudr2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="DAC channel2 DMA underrun flag"]
   #[inline] pub fn set_dmaudr2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="DAC channel1 DMA underrun flag"]
   #[inline] pub fn dmaudr1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="DAC channel1 DMA underrun flag"]
   #[inline] pub fn set_dmaudr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

}
impl ::core::fmt::Display for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dmaudr2() != 0 { try!(write!(f, " dmaudr2"))}
      if self.dmaudr1() != 0 { try!(write!(f, " dmaudr1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

