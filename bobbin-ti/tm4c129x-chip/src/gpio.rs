pub const GPIOA: Gpio = Gpio(0x40004000);
pub const GPIOB: Gpio = Gpio(0x40005000);
pub const GPIOC: Gpio = Gpio(0x40006000);
pub const GPIOD: Gpio = Gpio(0x40007000);
pub const GPIOE: Gpio = Gpio(0x40024000);
pub const GPIOF: Gpio = Gpio(0x40025000);
pub const GPIOG: Gpio = Gpio(0x40026000);
pub const GPIOH: Gpio = Gpio(0x40027000);
pub const GPIOJ: Gpio = Gpio(0x4003d000);
pub const GPIOA_AHB: Gpio = Gpio(0x40058000);
pub const GPIOB_AHB: Gpio = Gpio(0x40059000);
pub const GPIOC_AHB: Gpio = Gpio(0x4005a000);
pub const GPIOD_AHB: Gpio = Gpio(0x4005b000);
pub const GPIOE_AHB: Gpio = Gpio(0x4005c000);
pub const GPIOF_AHB: Gpio = Gpio(0x4005d000);
pub const GPIOG_AHB: Gpio = Gpio(0x4005e000);
pub const GPIOH_AHB: Gpio = Gpio(0x4005f000);
pub const GPIOJ_AHB: Gpio = Gpio(0x40060000);
pub const GPIOK: Gpio = Gpio(0x40061000);
pub const GPIOL: Gpio = Gpio(0x40062000);
pub const GPIOM: Gpio = Gpio(0x40063000);
pub const GPION: Gpio = Gpio(0x40064000);
pub const GPIOP: Gpio = Gpio(0x40065000);
pub const GPIOQ: Gpio = Gpio(0x40066000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gpio(pub u32);

impl Gpio {
  pub unsafe fn data(&self) -> Data { 
     Data(::core::ptr::read_volatile(((self.0 as usize) + 0x3fc) as *const u32))
  }
  pub unsafe fn set_data(&mut self, value: Data) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x3fc) as *mut u32, value.0);
  }
  pub unsafe fn with_data<F: FnOnce(Data) -> Data>(&mut self, f: F) {
     let tmp = self.data();
     self.set_data(f(tmp))
  }

  pub unsafe fn dir(&self) -> Dir { 
     Dir(::core::ptr::read_volatile(((self.0 as usize) + 0x400) as *const u32))
  }
  pub unsafe fn set_dir(&mut self, value: Dir) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x400) as *mut u32, value.0);
  }
  pub unsafe fn with_dir<F: FnOnce(Dir) -> Dir>(&mut self, f: F) {
     let tmp = self.dir();
     self.set_dir(f(tmp))
  }

  pub unsafe fn is(&self) -> Is { 
     Is(::core::ptr::read_volatile(((self.0 as usize) + 0x404) as *const u32))
  }
  pub unsafe fn set_is(&mut self, value: Is) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x404) as *mut u32, value.0);
  }
  pub unsafe fn with_is<F: FnOnce(Is) -> Is>(&mut self, f: F) {
     let tmp = self.is();
     self.set_is(f(tmp))
  }

  pub unsafe fn ibe(&self) -> Ibe { 
     Ibe(::core::ptr::read_volatile(((self.0 as usize) + 0x408) as *const u32))
  }
  pub unsafe fn set_ibe(&mut self, value: Ibe) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x408) as *mut u32, value.0);
  }
  pub unsafe fn with_ibe<F: FnOnce(Ibe) -> Ibe>(&mut self, f: F) {
     let tmp = self.ibe();
     self.set_ibe(f(tmp))
  }

  pub unsafe fn iev(&self) -> Iev { 
     Iev(::core::ptr::read_volatile(((self.0 as usize) + 0x40c) as *const u32))
  }
  pub unsafe fn set_iev(&mut self, value: Iev) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x40c) as *mut u32, value.0);
  }
  pub unsafe fn with_iev<F: FnOnce(Iev) -> Iev>(&mut self, f: F) {
     let tmp = self.iev();
     self.set_iev(f(tmp))
  }

  pub unsafe fn im(&self) -> Im { 
     Im(::core::ptr::read_volatile(((self.0 as usize) + 0x410) as *const u32))
  }
  pub unsafe fn set_im(&mut self, value: Im) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x410) as *mut u32, value.0);
  }
  pub unsafe fn with_im<F: FnOnce(Im) -> Im>(&mut self, f: F) {
     let tmp = self.im();
     self.set_im(f(tmp))
  }

  pub unsafe fn ris(&self) -> Ris { 
     Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x414) as *const u32))
  }
  pub unsafe fn set_ris(&mut self, value: Ris) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x414) as *mut u32, value.0);
  }
  pub unsafe fn with_ris<F: FnOnce(Ris) -> Ris>(&mut self, f: F) {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

  pub unsafe fn mis(&self) -> Mis { 
     Mis(::core::ptr::read_volatile(((self.0 as usize) + 0x418) as *const u32))
  }
  pub unsafe fn set_mis(&mut self, value: Mis) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x418) as *mut u32, value.0);
  }
  pub unsafe fn with_mis<F: FnOnce(Mis) -> Mis>(&mut self, f: F) {
     let tmp = self.mis();
     self.set_mis(f(tmp))
  }

  pub unsafe fn set_icr(&mut self, value: Icr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x41c) as *mut u32, value.0);
  }

  pub unsafe fn afsel(&self) -> Afsel { 
     Afsel(::core::ptr::read_volatile(((self.0 as usize) + 0x420) as *const u32))
  }
  pub unsafe fn set_afsel(&mut self, value: Afsel) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x420) as *mut u32, value.0);
  }
  pub unsafe fn with_afsel<F: FnOnce(Afsel) -> Afsel>(&mut self, f: F) {
     let tmp = self.afsel();
     self.set_afsel(f(tmp))
  }

  pub unsafe fn dr2r(&self) -> Dr2r { 
     Dr2r(::core::ptr::read_volatile(((self.0 as usize) + 0x500) as *const u32))
  }
  pub unsafe fn set_dr2r(&mut self, value: Dr2r) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x500) as *mut u32, value.0);
  }
  pub unsafe fn with_dr2r<F: FnOnce(Dr2r) -> Dr2r>(&mut self, f: F) {
     let tmp = self.dr2r();
     self.set_dr2r(f(tmp))
  }

  pub unsafe fn dr4r(&self) -> Dr4r { 
     Dr4r(::core::ptr::read_volatile(((self.0 as usize) + 0x504) as *const u32))
  }
  pub unsafe fn set_dr4r(&mut self, value: Dr4r) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x504) as *mut u32, value.0);
  }
  pub unsafe fn with_dr4r<F: FnOnce(Dr4r) -> Dr4r>(&mut self, f: F) {
     let tmp = self.dr4r();
     self.set_dr4r(f(tmp))
  }

  pub unsafe fn dr8r(&self) -> Dr8r { 
     Dr8r(::core::ptr::read_volatile(((self.0 as usize) + 0x508) as *const u32))
  }
  pub unsafe fn set_dr8r(&mut self, value: Dr8r) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x508) as *mut u32, value.0);
  }
  pub unsafe fn with_dr8r<F: FnOnce(Dr8r) -> Dr8r>(&mut self, f: F) {
     let tmp = self.dr8r();
     self.set_dr8r(f(tmp))
  }

  pub unsafe fn odr(&self) -> Odr { 
     Odr(::core::ptr::read_volatile(((self.0 as usize) + 0x50c) as *const u32))
  }
  pub unsafe fn set_odr(&mut self, value: Odr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x50c) as *mut u32, value.0);
  }
  pub unsafe fn with_odr<F: FnOnce(Odr) -> Odr>(&mut self, f: F) {
     let tmp = self.odr();
     self.set_odr(f(tmp))
  }

  pub unsafe fn pur(&self) -> Pur { 
     Pur(::core::ptr::read_volatile(((self.0 as usize) + 0x510) as *const u32))
  }
  pub unsafe fn set_pur(&mut self, value: Pur) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x510) as *mut u32, value.0);
  }
  pub unsafe fn with_pur<F: FnOnce(Pur) -> Pur>(&mut self, f: F) {
     let tmp = self.pur();
     self.set_pur(f(tmp))
  }

  pub unsafe fn pdr(&self) -> Pdr { 
     Pdr(::core::ptr::read_volatile(((self.0 as usize) + 0x514) as *const u32))
  }
  pub unsafe fn set_pdr(&mut self, value: Pdr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x514) as *mut u32, value.0);
  }
  pub unsafe fn with_pdr<F: FnOnce(Pdr) -> Pdr>(&mut self, f: F) {
     let tmp = self.pdr();
     self.set_pdr(f(tmp))
  }

  pub unsafe fn slr(&self) -> Slr { 
     Slr(::core::ptr::read_volatile(((self.0 as usize) + 0x518) as *const u32))
  }
  pub unsafe fn set_slr(&mut self, value: Slr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x518) as *mut u32, value.0);
  }
  pub unsafe fn with_slr<F: FnOnce(Slr) -> Slr>(&mut self, f: F) {
     let tmp = self.slr();
     self.set_slr(f(tmp))
  }

  pub unsafe fn den(&self) -> Den { 
     Den(::core::ptr::read_volatile(((self.0 as usize) + 0x51c) as *const u32))
  }
  pub unsafe fn set_den(&mut self, value: Den) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x51c) as *mut u32, value.0);
  }
  pub unsafe fn with_den<F: FnOnce(Den) -> Den>(&mut self, f: F) {
     let tmp = self.den();
     self.set_den(f(tmp))
  }

  pub unsafe fn lock(&self) -> Lock { 
     Lock(::core::ptr::read_volatile(((self.0 as usize) + 0x520) as *const u32))
  }
  pub unsafe fn set_lock(&mut self, value: Lock) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x520) as *mut u32, value.0);
  }
  pub unsafe fn with_lock<F: FnOnce(Lock) -> Lock>(&mut self, f: F) {
     let tmp = self.lock();
     self.set_lock(f(tmp))
  }

  pub unsafe fn cr(&self) -> Cr { 
     Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x524) as *const u32))
  }

  pub unsafe fn amsel(&self) -> Amsel { 
     Amsel(::core::ptr::read_volatile(((self.0 as usize) + 0x528) as *const u32))
  }
  pub unsafe fn set_amsel(&mut self, value: Amsel) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x528) as *mut u32, value.0);
  }
  pub unsafe fn with_amsel<F: FnOnce(Amsel) -> Amsel>(&mut self, f: F) {
     let tmp = self.amsel();
     self.set_amsel(f(tmp))
  }

  pub unsafe fn pctl(&self) -> Pctl { 
     Pctl(::core::ptr::read_volatile(((self.0 as usize) + 0x52c) as *const u32))
  }
  pub unsafe fn set_pctl(&mut self, value: Pctl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x52c) as *mut u32, value.0);
  }
  pub unsafe fn with_pctl<F: FnOnce(Pctl) -> Pctl>(&mut self, f: F) {
     let tmp = self.pctl();
     self.set_pctl(f(tmp))
  }

  pub unsafe fn adcctl(&self) -> Adcctl { 
     Adcctl(::core::ptr::read_volatile(((self.0 as usize) + 0x530) as *const u32))
  }
  pub unsafe fn set_adcctl(&mut self, value: Adcctl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x530) as *mut u32, value.0);
  }
  pub unsafe fn with_adcctl<F: FnOnce(Adcctl) -> Adcctl>(&mut self, f: F) {
     let tmp = self.adcctl();
     self.set_adcctl(f(tmp))
  }

  pub unsafe fn dmactl(&self) -> Dmactl { 
     Dmactl(::core::ptr::read_volatile(((self.0 as usize) + 0x534) as *const u32))
  }
  pub unsafe fn set_dmactl(&mut self, value: Dmactl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x534) as *mut u32, value.0);
  }
  pub unsafe fn with_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&mut self, f: F) {
     let tmp = self.dmactl();
     self.set_dmactl(f(tmp))
  }

  pub unsafe fn gpiosi(&self) -> Gpiosi { 
     Gpiosi(::core::ptr::read_volatile(((self.0 as usize) + 0x539) as *const u32))
  }
  pub unsafe fn set_gpiosi(&mut self, value: Gpiosi) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x539) as *mut u32, value.0);
  }
  pub unsafe fn with_gpiosi<F: FnOnce(Gpiosi) -> Gpiosi>(&mut self, f: F) {
     let tmp = self.gpiosi();
     self.set_gpiosi(f(tmp))
  }

  pub unsafe fn gpiodr12r(&self) -> Gpiodr12r { 
     Gpiodr12r(::core::ptr::read_volatile(((self.0 as usize) + 0x53c) as *const u32))
  }
  pub unsafe fn set_gpiodr12r(&mut self, value: Gpiodr12r) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x53c) as *mut u32, value.0);
  }
  pub unsafe fn with_gpiodr12r<F: FnOnce(Gpiodr12r) -> Gpiodr12r>(&mut self, f: F) {
     let tmp = self.gpiodr12r();
     self.set_gpiodr12r(f(tmp))
  }

  pub unsafe fn gpiowakepen(&self) -> Gpiowakepen { 
     Gpiowakepen(::core::ptr::read_volatile(((self.0 as usize) + 0x540) as *const u32))
  }
  pub unsafe fn set_gpiowakepen(&mut self, value: Gpiowakepen) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x540) as *mut u32, value.0);
  }
  pub unsafe fn with_gpiowakepen<F: FnOnce(Gpiowakepen) -> Gpiowakepen>(&mut self, f: F) {
     let tmp = self.gpiowakepen();
     self.set_gpiowakepen(f(tmp))
  }

  pub unsafe fn gpiowakelvl(&self) -> Gpiowakelvl { 
     Gpiowakelvl(::core::ptr::read_volatile(((self.0 as usize) + 0x544) as *const u32))
  }
  pub unsafe fn set_gpiowakelvl(&mut self, value: Gpiowakelvl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x544) as *mut u32, value.0);
  }
  pub unsafe fn with_gpiowakelvl<F: FnOnce(Gpiowakelvl) -> Gpiowakelvl>(&mut self, f: F) {
     let tmp = self.gpiowakelvl();
     self.set_gpiowakelvl(f(tmp))
  }

  pub unsafe fn gpiowakestat(&self) -> Gpiowakestat { 
     Gpiowakestat(::core::ptr::read_volatile(((self.0 as usize) + 0x548) as *const u32))
  }
  pub unsafe fn set_gpiowakestat(&mut self, value: Gpiowakestat) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x548) as *mut u32, value.0);
  }
  pub unsafe fn with_gpiowakestat<F: FnOnce(Gpiowakestat) -> Gpiowakestat>(&mut self, f: F) {
     let tmp = self.gpiowakestat();
     self.set_gpiowakestat(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Data(pub u32);

impl Data {
  pub fn data(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_data(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Data {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data(0) != 0 { try!(write!(f, " data[0]"))}
      if self.data(1) != 0 { try!(write!(f, " data[1]"))}
      if self.data(2) != 0 { try!(write!(f, " data[2]"))}
      if self.data(3) != 0 { try!(write!(f, " data[3]"))}
      if self.data(4) != 0 { try!(write!(f, " data[4]"))}
      if self.data(5) != 0 { try!(write!(f, " data[5]"))}
      if self.data(6) != 0 { try!(write!(f, " data[6]"))}
      if self.data(7) != 0 { try!(write!(f, " data[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dir(pub u32);

impl Dir {
  pub fn dir(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_dir(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Dir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dir {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dir(0) != 0 { try!(write!(f, " dir[0]"))}
      if self.dir(1) != 0 { try!(write!(f, " dir[1]"))}
      if self.dir(2) != 0 { try!(write!(f, " dir[2]"))}
      if self.dir(3) != 0 { try!(write!(f, " dir[3]"))}
      if self.dir(4) != 0 { try!(write!(f, " dir[4]"))}
      if self.dir(5) != 0 { try!(write!(f, " dir[5]"))}
      if self.dir(6) != 0 { try!(write!(f, " dir[6]"))}
      if self.dir(7) != 0 { try!(write!(f, " dir[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Is(pub u32);

impl Is {
  pub fn is(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_is(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Is {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Is {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.is(0) != 0 { try!(write!(f, " is[0]"))}
      if self.is(1) != 0 { try!(write!(f, " is[1]"))}
      if self.is(2) != 0 { try!(write!(f, " is[2]"))}
      if self.is(3) != 0 { try!(write!(f, " is[3]"))}
      if self.is(4) != 0 { try!(write!(f, " is[4]"))}
      if self.is(5) != 0 { try!(write!(f, " is[5]"))}
      if self.is(6) != 0 { try!(write!(f, " is[6]"))}
      if self.is(7) != 0 { try!(write!(f, " is[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ibe(pub u32);

impl Ibe {
  pub fn ibe(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_ibe(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Ibe {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ibe {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ibe(0) != 0 { try!(write!(f, " ibe[0]"))}
      if self.ibe(1) != 0 { try!(write!(f, " ibe[1]"))}
      if self.ibe(2) != 0 { try!(write!(f, " ibe[2]"))}
      if self.ibe(3) != 0 { try!(write!(f, " ibe[3]"))}
      if self.ibe(4) != 0 { try!(write!(f, " ibe[4]"))}
      if self.ibe(5) != 0 { try!(write!(f, " ibe[5]"))}
      if self.ibe(6) != 0 { try!(write!(f, " ibe[6]"))}
      if self.ibe(7) != 0 { try!(write!(f, " ibe[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Iev(pub u32);

impl Iev {
  pub fn iev(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_iev(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Iev {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Iev {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.iev(0) != 0 { try!(write!(f, " iev[0]"))}
      if self.iev(1) != 0 { try!(write!(f, " iev[1]"))}
      if self.iev(2) != 0 { try!(write!(f, " iev[2]"))}
      if self.iev(3) != 0 { try!(write!(f, " iev[3]"))}
      if self.iev(4) != 0 { try!(write!(f, " iev[4]"))}
      if self.iev(5) != 0 { try!(write!(f, " iev[5]"))}
      if self.iev(6) != 0 { try!(write!(f, " iev[6]"))}
      if self.iev(7) != 0 { try!(write!(f, " iev[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Im(pub u32);

impl Im {
  pub fn im(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_im(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn dmaime(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dmaime(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Im {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Im {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.im(0) != 0 { try!(write!(f, " im[0]"))}
      if self.im(1) != 0 { try!(write!(f, " im[1]"))}
      if self.im(2) != 0 { try!(write!(f, " im[2]"))}
      if self.im(3) != 0 { try!(write!(f, " im[3]"))}
      if self.im(4) != 0 { try!(write!(f, " im[4]"))}
      if self.im(5) != 0 { try!(write!(f, " im[5]"))}
      if self.im(6) != 0 { try!(write!(f, " im[6]"))}
      if self.im(7) != 0 { try!(write!(f, " im[7]"))}
      if self.dmaime() != 0 { try!(write!(f, " dmaime"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ris(pub u32);

impl Ris {
  pub fn ris(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_ris(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn dmaris(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dmaris(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ris(0) != 0 { try!(write!(f, " ris[0]"))}
      if self.ris(1) != 0 { try!(write!(f, " ris[1]"))}
      if self.ris(2) != 0 { try!(write!(f, " ris[2]"))}
      if self.ris(3) != 0 { try!(write!(f, " ris[3]"))}
      if self.ris(4) != 0 { try!(write!(f, " ris[4]"))}
      if self.ris(5) != 0 { try!(write!(f, " ris[5]"))}
      if self.ris(6) != 0 { try!(write!(f, " ris[6]"))}
      if self.ris(7) != 0 { try!(write!(f, " ris[7]"))}
      if self.dmaris() != 0 { try!(write!(f, " dmaris"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mis(pub u32);

impl Mis {
  pub fn mis(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_mis(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn dmamis(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dmamis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Mis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mis(0) != 0 { try!(write!(f, " mis[0]"))}
      if self.mis(1) != 0 { try!(write!(f, " mis[1]"))}
      if self.mis(2) != 0 { try!(write!(f, " mis[2]"))}
      if self.mis(3) != 0 { try!(write!(f, " mis[3]"))}
      if self.mis(4) != 0 { try!(write!(f, " mis[4]"))}
      if self.mis(5) != 0 { try!(write!(f, " mis[5]"))}
      if self.mis(6) != 0 { try!(write!(f, " mis[6]"))}
      if self.mis(7) != 0 { try!(write!(f, " mis[7]"))}
      if self.dmamis() != 0 { try!(write!(f, " dmamis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Icr(pub u32);

impl Icr {
  pub fn icr(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_icr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn dmamic(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dmamic(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.icr(0) != 0 { try!(write!(f, " icr[0]"))}
      if self.icr(1) != 0 { try!(write!(f, " icr[1]"))}
      if self.icr(2) != 0 { try!(write!(f, " icr[2]"))}
      if self.icr(3) != 0 { try!(write!(f, " icr[3]"))}
      if self.icr(4) != 0 { try!(write!(f, " icr[4]"))}
      if self.icr(5) != 0 { try!(write!(f, " icr[5]"))}
      if self.icr(6) != 0 { try!(write!(f, " icr[6]"))}
      if self.icr(7) != 0 { try!(write!(f, " icr[7]"))}
      if self.dmamic() != 0 { try!(write!(f, " dmamic"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Afsel(pub u32);

impl Afsel {
  pub fn afsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_afsel(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Afsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Afsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.afsel(0) != 0 { try!(write!(f, " afsel[0]"))}
      if self.afsel(1) != 0 { try!(write!(f, " afsel[1]"))}
      if self.afsel(2) != 0 { try!(write!(f, " afsel[2]"))}
      if self.afsel(3) != 0 { try!(write!(f, " afsel[3]"))}
      if self.afsel(4) != 0 { try!(write!(f, " afsel[4]"))}
      if self.afsel(5) != 0 { try!(write!(f, " afsel[5]"))}
      if self.afsel(6) != 0 { try!(write!(f, " afsel[6]"))}
      if self.afsel(7) != 0 { try!(write!(f, " afsel[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dr2r(pub u32);

impl Dr2r {
  pub fn drv2(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_drv2(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Dr2r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr2r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv2(0) != 0 { try!(write!(f, " drv2[0]"))}
      if self.drv2(1) != 0 { try!(write!(f, " drv2[1]"))}
      if self.drv2(2) != 0 { try!(write!(f, " drv2[2]"))}
      if self.drv2(3) != 0 { try!(write!(f, " drv2[3]"))}
      if self.drv2(4) != 0 { try!(write!(f, " drv2[4]"))}
      if self.drv2(5) != 0 { try!(write!(f, " drv2[5]"))}
      if self.drv2(6) != 0 { try!(write!(f, " drv2[6]"))}
      if self.drv2(7) != 0 { try!(write!(f, " drv2[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dr4r(pub u32);

impl Dr4r {
  pub fn drv4(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_drv4(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Dr4r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr4r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv4(0) != 0 { try!(write!(f, " drv4[0]"))}
      if self.drv4(1) != 0 { try!(write!(f, " drv4[1]"))}
      if self.drv4(2) != 0 { try!(write!(f, " drv4[2]"))}
      if self.drv4(3) != 0 { try!(write!(f, " drv4[3]"))}
      if self.drv4(4) != 0 { try!(write!(f, " drv4[4]"))}
      if self.drv4(5) != 0 { try!(write!(f, " drv4[5]"))}
      if self.drv4(6) != 0 { try!(write!(f, " drv4[6]"))}
      if self.drv4(7) != 0 { try!(write!(f, " drv4[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dr8r(pub u32);

impl Dr8r {
  pub fn drv8(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_drv8(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Dr8r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dr8r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv8(0) != 0 { try!(write!(f, " drv8[0]"))}
      if self.drv8(1) != 0 { try!(write!(f, " drv8[1]"))}
      if self.drv8(2) != 0 { try!(write!(f, " drv8[2]"))}
      if self.drv8(3) != 0 { try!(write!(f, " drv8[3]"))}
      if self.drv8(4) != 0 { try!(write!(f, " drv8[4]"))}
      if self.drv8(5) != 0 { try!(write!(f, " drv8[5]"))}
      if self.drv8(6) != 0 { try!(write!(f, " drv8[6]"))}
      if self.drv8(7) != 0 { try!(write!(f, " drv8[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Odr(pub u32);

impl Odr {
  pub fn ode(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_ode(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Odr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Odr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ode(0) != 0 { try!(write!(f, " ode[0]"))}
      if self.ode(1) != 0 { try!(write!(f, " ode[1]"))}
      if self.ode(2) != 0 { try!(write!(f, " ode[2]"))}
      if self.ode(3) != 0 { try!(write!(f, " ode[3]"))}
      if self.ode(4) != 0 { try!(write!(f, " ode[4]"))}
      if self.ode(5) != 0 { try!(write!(f, " ode[5]"))}
      if self.ode(6) != 0 { try!(write!(f, " ode[6]"))}
      if self.ode(7) != 0 { try!(write!(f, " ode[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pur(pub u32);

impl Pur {
  pub fn pue(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_pue(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Pur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pur {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pue(0) != 0 { try!(write!(f, " pue[0]"))}
      if self.pue(1) != 0 { try!(write!(f, " pue[1]"))}
      if self.pue(2) != 0 { try!(write!(f, " pue[2]"))}
      if self.pue(3) != 0 { try!(write!(f, " pue[3]"))}
      if self.pue(4) != 0 { try!(write!(f, " pue[4]"))}
      if self.pue(5) != 0 { try!(write!(f, " pue[5]"))}
      if self.pue(6) != 0 { try!(write!(f, " pue[6]"))}
      if self.pue(7) != 0 { try!(write!(f, " pue[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pdr(pub u32);

impl Pdr {
  pub fn pde(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_pde(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Pdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pde(0) != 0 { try!(write!(f, " pde[0]"))}
      if self.pde(1) != 0 { try!(write!(f, " pde[1]"))}
      if self.pde(2) != 0 { try!(write!(f, " pde[2]"))}
      if self.pde(3) != 0 { try!(write!(f, " pde[3]"))}
      if self.pde(4) != 0 { try!(write!(f, " pde[4]"))}
      if self.pde(5) != 0 { try!(write!(f, " pde[5]"))}
      if self.pde(6) != 0 { try!(write!(f, " pde[6]"))}
      if self.pde(7) != 0 { try!(write!(f, " pde[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Slr(pub u32);

impl Slr {
  pub fn slr(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_slr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Slr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Slr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.slr(0) != 0 { try!(write!(f, " slr[0]"))}
      if self.slr(1) != 0 { try!(write!(f, " slr[1]"))}
      if self.slr(2) != 0 { try!(write!(f, " slr[2]"))}
      if self.slr(3) != 0 { try!(write!(f, " slr[3]"))}
      if self.slr(4) != 0 { try!(write!(f, " slr[4]"))}
      if self.slr(5) != 0 { try!(write!(f, " slr[5]"))}
      if self.slr(6) != 0 { try!(write!(f, " slr[6]"))}
      if self.slr(7) != 0 { try!(write!(f, " slr[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Den(pub u32);

impl Den {
  pub fn den(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_den(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Den {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Den {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.den(0) != 0 { try!(write!(f, " den[0]"))}
      if self.den(1) != 0 { try!(write!(f, " den[1]"))}
      if self.den(2) != 0 { try!(write!(f, " den[2]"))}
      if self.den(3) != 0 { try!(write!(f, " den[3]"))}
      if self.den(4) != 0 { try!(write!(f, " den[4]"))}
      if self.den(5) != 0 { try!(write!(f, " den[5]"))}
      if self.den(6) != 0 { try!(write!(f, " den[6]"))}
      if self.den(7) != 0 { try!(write!(f, " den[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Lock(pub u32);

impl Lock {
  pub fn gpio_lock(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_gpio_lock(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Lock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Lock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);

impl Cr {
  pub fn cr(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_cr(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.cr(0) != 0 { try!(write!(f, " cr[0]"))}
      if self.cr(1) != 0 { try!(write!(f, " cr[1]"))}
      if self.cr(2) != 0 { try!(write!(f, " cr[2]"))}
      if self.cr(3) != 0 { try!(write!(f, " cr[3]"))}
      if self.cr(4) != 0 { try!(write!(f, " cr[4]"))}
      if self.cr(5) != 0 { try!(write!(f, " cr[5]"))}
      if self.cr(6) != 0 { try!(write!(f, " cr[6]"))}
      if self.cr(7) != 0 { try!(write!(f, " cr[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Amsel(pub u32);

impl Amsel {
  pub fn gpioamsel(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_gpioamsel(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Amsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Amsel {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gpioamsel(0) != 0 { try!(write!(f, " gpioamsel[0]"))}
      if self.gpioamsel(1) != 0 { try!(write!(f, " gpioamsel[1]"))}
      if self.gpioamsel(2) != 0 { try!(write!(f, " gpioamsel[2]"))}
      if self.gpioamsel(3) != 0 { try!(write!(f, " gpioamsel[3]"))}
      if self.gpioamsel(4) != 0 { try!(write!(f, " gpioamsel[4]"))}
      if self.gpioamsel(5) != 0 { try!(write!(f, " gpioamsel[5]"))}
      if self.gpioamsel(6) != 0 { try!(write!(f, " gpioamsel[6]"))}
      if self.gpioamsel(7) != 0 { try!(write!(f, " gpioamsel[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pctl(pub u32);

impl Pctl {
  pub fn pmc(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0xf // [3:0]
  }
  pub fn set_pmc(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0xf) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Pctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pmc(0) != 0 { try!(write!(f, " pmc[0]=0x{:x}", self.pmc(0)))}
      if self.pmc(1) != 0 { try!(write!(f, " pmc[1]=0x{:x}", self.pmc(1)))}
      if self.pmc(2) != 0 { try!(write!(f, " pmc[2]=0x{:x}", self.pmc(2)))}
      if self.pmc(3) != 0 { try!(write!(f, " pmc[3]=0x{:x}", self.pmc(3)))}
      if self.pmc(4) != 0 { try!(write!(f, " pmc[4]=0x{:x}", self.pmc(4)))}
      if self.pmc(5) != 0 { try!(write!(f, " pmc[5]=0x{:x}", self.pmc(5)))}
      if self.pmc(6) != 0 { try!(write!(f, " pmc[6]=0x{:x}", self.pmc(6)))}
      if self.pmc(7) != 0 { try!(write!(f, " pmc[7]=0x{:x}", self.pmc(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Adcctl(pub u32);

impl Adcctl {
  pub fn adcen(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_adcen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Adcctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Adcctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.adcen(0) != 0 { try!(write!(f, " adcen[0]"))}
      if self.adcen(1) != 0 { try!(write!(f, " adcen[1]"))}
      if self.adcen(2) != 0 { try!(write!(f, " adcen[2]"))}
      if self.adcen(3) != 0 { try!(write!(f, " adcen[3]"))}
      if self.adcen(4) != 0 { try!(write!(f, " adcen[4]"))}
      if self.adcen(5) != 0 { try!(write!(f, " adcen[5]"))}
      if self.adcen(6) != 0 { try!(write!(f, " adcen[6]"))}
      if self.adcen(7) != 0 { try!(write!(f, " adcen[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dmactl(pub u32);

impl Dmactl {
  pub fn dmaen(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_dmaen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Dmactl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dmactl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dmaen(0) != 0 { try!(write!(f, " dmaen[0]"))}
      if self.dmaen(1) != 0 { try!(write!(f, " dmaen[1]"))}
      if self.dmaen(2) != 0 { try!(write!(f, " dmaen[2]"))}
      if self.dmaen(3) != 0 { try!(write!(f, " dmaen[3]"))}
      if self.dmaen(4) != 0 { try!(write!(f, " dmaen[4]"))}
      if self.dmaen(5) != 0 { try!(write!(f, " dmaen[5]"))}
      if self.dmaen(6) != 0 { try!(write!(f, " dmaen[6]"))}
      if self.dmaen(7) != 0 { try!(write!(f, " dmaen[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Gpiosi(pub u32);

impl Gpiosi {
  pub fn sum(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_sum(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Gpiosi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Gpiosi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sum(0) != 0 { try!(write!(f, " sum[0]"))}
      if self.sum(1) != 0 { try!(write!(f, " sum[1]"))}
      if self.sum(2) != 0 { try!(write!(f, " sum[2]"))}
      if self.sum(3) != 0 { try!(write!(f, " sum[3]"))}
      if self.sum(4) != 0 { try!(write!(f, " sum[4]"))}
      if self.sum(5) != 0 { try!(write!(f, " sum[5]"))}
      if self.sum(6) != 0 { try!(write!(f, " sum[6]"))}
      if self.sum(7) != 0 { try!(write!(f, " sum[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Gpiodr12r(pub u32);

impl Gpiodr12r {
  pub fn drv12(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_drv12(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Gpiodr12r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Gpiodr12r {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.drv12(0) != 0 { try!(write!(f, " drv12[0]"))}
      if self.drv12(1) != 0 { try!(write!(f, " drv12[1]"))}
      if self.drv12(2) != 0 { try!(write!(f, " drv12[2]"))}
      if self.drv12(3) != 0 { try!(write!(f, " drv12[3]"))}
      if self.drv12(4) != 0 { try!(write!(f, " drv12[4]"))}
      if self.drv12(5) != 0 { try!(write!(f, " drv12[5]"))}
      if self.drv12(6) != 0 { try!(write!(f, " drv12[6]"))}
      if self.drv12(7) != 0 { try!(write!(f, " drv12[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Gpiowakepen(pub u32);

impl Gpiowakepen {
  pub fn wakep7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_wakep7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn wakep6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_wakep6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn wakep5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_wakep5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn wakep4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_wakep4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Gpiowakepen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Gpiowakepen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakep7() != 0 { try!(write!(f, " wakep7"))}
      if self.wakep6() != 0 { try!(write!(f, " wakep6"))}
      if self.wakep5() != 0 { try!(write!(f, " wakep5"))}
      if self.wakep4() != 0 { try!(write!(f, " wakep4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Gpiowakelvl(pub u32);

impl Gpiowakelvl {
  pub fn wakelvl7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_wakelvl7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn wakelvl6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_wakelvl6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn wakelvl5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_wakelvl5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn wakelvl4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_wakelvl4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Gpiowakelvl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Gpiowakelvl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakelvl7() != 0 { try!(write!(f, " wakelvl7"))}
      if self.wakelvl6() != 0 { try!(write!(f, " wakelvl6"))}
      if self.wakelvl5() != 0 { try!(write!(f, " wakelvl5"))}
      if self.wakelvl4() != 0 { try!(write!(f, " wakelvl4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Gpiowakestat(pub u32);

impl Gpiowakestat {
  pub fn wakestat7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_wakestat7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn wakestat6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_wakestat6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn wakestat5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_wakestat5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn wakestat4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_wakestat4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}

impl ::core::fmt::Display for Gpiowakestat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Gpiowakestat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wakestat7() != 0 { try!(write!(f, " wakestat7"))}
      if self.wakestat6() != 0 { try!(write!(f, " wakestat6"))}
      if self.wakestat5() != 0 { try!(write!(f, " wakestat5"))}
      if self.wakestat4() != 0 { try!(write!(f, " wakestat4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

pub trait Pin {
   fn port(&self) -> Gpio;
   fn index(&self) -> usize;
}

pub trait AfU0rx {
   fn af_u0rx(&self) -> usize;
}

pub trait AfI2c9scl {
   fn af_i2c9scl(&self) -> usize;
}

pub trait AfT0ccp0 {
   fn af_t0ccp0(&self) -> usize;
}

pub trait AfCan0rx {
   fn af_can0rx(&self) -> usize;
}

pub trait AfU0tx {
   fn af_u0tx(&self) -> usize;
}

pub trait AfI2c9sda {
   fn af_i2c9sda(&self) -> usize;
}

pub trait AfT0ccp1 {
   fn af_t0ccp1(&self) -> usize;
}

pub trait AfCan0tx {
   fn af_can0tx(&self) -> usize;
}

pub trait AfU4rx {
   fn af_u4rx(&self) -> usize;
}

pub trait AfI2c8scl {
   fn af_i2c8scl(&self) -> usize;
}

pub trait AfT1ccp0 {
   fn af_t1ccp0(&self) -> usize;
}

pub trait AfSsi0clk {
   fn af_ssi0clk(&self) -> usize;
}

pub trait AfU4tx {
   fn af_u4tx(&self) -> usize;
}

pub trait AfI2c8sda {
   fn af_i2c8sda(&self) -> usize;
}

pub trait AfT1ccp1 {
   fn af_t1ccp1(&self) -> usize;
}

pub trait AfSsi0fss {
   fn af_ssi0fss(&self) -> usize;
}

pub trait AfU3rx {
   fn af_u3rx(&self) -> usize;
}

pub trait AfI2c7scl {
   fn af_i2c7scl(&self) -> usize;
}

pub trait AfT2ccp0 {
   fn af_t2ccp0(&self) -> usize;
}

pub trait AfSsi0xdat0 {
   fn af_ssi0xdat0(&self) -> usize;
}

pub trait AfU3tx {
   fn af_u3tx(&self) -> usize;
}

pub trait AfI2c7sda {
   fn af_i2c7sda(&self) -> usize;
}

pub trait AfT2ccp1 {
   fn af_t2ccp1(&self) -> usize;
}

pub trait AfSsi0xdat1 {
   fn af_ssi0xdat1(&self) -> usize;
}

pub trait AfU2rx {
   fn af_u2rx(&self) -> usize;
}

pub trait AfI2c6scl {
   fn af_i2c6scl(&self) -> usize;
}

pub trait AfT3ccp0 {
   fn af_t3ccp0(&self) -> usize;
}

pub trait AfUsb0epen {
   fn af_usb0epen(&self) -> usize;
}

pub trait AfSsi0xdat2 {
   fn af_ssi0xdat2(&self) -> usize;
}

pub trait AfEpi0s8 {
   fn af_epi0s8(&self) -> usize;
}

pub trait AfU2tx {
   fn af_u2tx(&self) -> usize;
}

pub trait AfI2c6sda {
   fn af_i2c6sda(&self) -> usize;
}

pub trait AfT3ccp1 {
   fn af_t3ccp1(&self) -> usize;
}

pub trait AfUsb0pflt {
   fn af_usb0pflt(&self) -> usize;
}

pub trait AfSsi0xdat3 {
   fn af_ssi0xdat3(&self) -> usize;
}

pub trait AfEpi0s9 {
   fn af_epi0s9(&self) -> usize;
}

pub trait AfUsb0id {
   fn af_usb0id(&self) -> usize;
}

pub trait AfU1rx {
   fn af_u1rx(&self) -> usize;
}

pub trait AfI2c5scl {
   fn af_i2c5scl(&self) -> usize;
}

pub trait AfT4ccp0 {
   fn af_t4ccp0(&self) -> usize;
}

pub trait AfCan1rx {
   fn af_can1rx(&self) -> usize;
}

pub trait AfUsb0vbus {
   fn af_usb0vbus(&self) -> usize;
}

pub trait AfU1tx {
   fn af_u1tx(&self) -> usize;
}

pub trait AfI2c5sda {
   fn af_i2c5sda(&self) -> usize;
}

pub trait AfT4ccp1 {
   fn af_t4ccp1(&self) -> usize;
}

pub trait AfCan1tx {
   fn af_can1tx(&self) -> usize;
}

pub trait AfI2c0scl {
   fn af_i2c0scl(&self) -> usize;
}

pub trait AfT5ccp0 {
   fn af_t5ccp0(&self) -> usize;
}

pub trait AfUsb0stp {
   fn af_usb0stp(&self) -> usize;
}

pub trait AfEpi0s27 {
   fn af_epi0s27(&self) -> usize;
}

pub trait AfI2c0sda {
   fn af_i2c0sda(&self) -> usize;
}

pub trait AfT5ccp1 {
   fn af_t5ccp1(&self) -> usize;
}

pub trait AfUsb0clk {
   fn af_usb0clk(&self) -> usize;
}

pub trait AfEpi0s28 {
   fn af_epi0s28(&self) -> usize;
}

pub trait AfAin10 {
   fn af_ain10(&self) -> usize;
}

pub trait AfU0cts {
   fn af_u0cts(&self) -> usize;
}

pub trait AfSsi1fss {
   fn af_ssi1fss(&self) -> usize;
}

pub trait AfAin11 {
   fn af_ain11(&self) -> usize;
}

pub trait AfU0rts {
   fn af_u0rts(&self) -> usize;
}

pub trait AfSsi1clk {
   fn af_ssi1clk(&self) -> usize;
}

pub trait AfTck {
   fn af_tck(&self) -> usize;
}

pub trait AfSwclk {
   fn af_swclk(&self) -> usize;
}

pub trait AfTms {
   fn af_tms(&self) -> usize;
}

pub trait AfSwdio {
   fn af_swdio(&self) -> usize;
}

pub trait AfTdi {
   fn af_tdi(&self) -> usize;
}

pub trait AfTdo {
   fn af_tdo(&self) -> usize;
}

pub trait AfSwo {
   fn af_swo(&self) -> usize;
}

pub trait AfC1Neg {
   fn af_c1_neg(&self) -> usize;
}

pub trait AfU7rx {
   fn af_u7rx(&self) -> usize;
}

pub trait AfEpi0s7 {
   fn af_epi0s7(&self) -> usize;
}

pub trait AfC1Pos {
   fn af_c1_pos(&self) -> usize;
}

pub trait AfU7tx {
   fn af_u7tx(&self) -> usize;
}

pub trait AfRtcclk {
   fn af_rtcclk(&self) -> usize;
}

pub trait AfEpi0s6 {
   fn af_epi0s6(&self) -> usize;
}

pub trait AfC0Neg {
   fn af_c0_neg(&self) -> usize;
}

pub trait AfU5rx {
   fn af_u5rx(&self) -> usize;
}

pub trait AfEpi0s5 {
   fn af_epi0s5(&self) -> usize;
}

pub trait AfC0Pos {
   fn af_c0_pos(&self) -> usize;
}

pub trait AfU5tx {
   fn af_u5tx(&self) -> usize;
}

pub trait AfEpi0s4 {
   fn af_epi0s4(&self) -> usize;
}

pub trait AfAin15 {
   fn af_ain15(&self) -> usize;
}

pub trait AfC0o {
   fn af_c0o(&self) -> usize;
}

pub trait AfSsi2xdat1 {
   fn af_ssi2xdat1(&self) -> usize;
}

pub trait AfAin14 {
   fn af_ain14(&self) -> usize;
}

pub trait AfC1o {
   fn af_c1o(&self) -> usize;
}

pub trait AfSsi2xdat0 {
   fn af_ssi2xdat0(&self) -> usize;
}

pub trait AfAin13 {
   fn af_ain13(&self) -> usize;
}

pub trait AfC2o {
   fn af_c2o(&self) -> usize;
}

pub trait AfSsi2fss {
   fn af_ssi2fss(&self) -> usize;
}

pub trait AfAin12 {
   fn af_ain12(&self) -> usize;
}

pub trait AfSsi2clk {
   fn af_ssi2clk(&self) -> usize;
}

pub trait AfAin7 {
   fn af_ain7(&self) -> usize;
}

pub trait AfSsi1xdat2 {
   fn af_ssi1xdat2(&self) -> usize;
}

pub trait AfAin6 {
   fn af_ain6(&self) -> usize;
}

pub trait AfSsi1xdat3 {
   fn af_ssi1xdat3(&self) -> usize;
}

pub trait AfAin5 {
   fn af_ain5(&self) -> usize;
}

pub trait AfU2rts {
   fn af_u2rts(&self) -> usize;
}

pub trait AfSsi2xdat3 {
   fn af_ssi2xdat3(&self) -> usize;
}

pub trait AfAin4 {
   fn af_ain4(&self) -> usize;
}

pub trait AfU2cts {
   fn af_u2cts(&self) -> usize;
}

pub trait AfNmi {
   fn af_nmi(&self) -> usize;
}

pub trait AfSsi2xdat2 {
   fn af_ssi2xdat2(&self) -> usize;
}

pub trait AfAin3 {
   fn af_ain3(&self) -> usize;
}

pub trait AfU1rts {
   fn af_u1rts(&self) -> usize;
}

pub trait AfAin2 {
   fn af_ain2(&self) -> usize;
}

pub trait AfU1dsr {
   fn af_u1dsr(&self) -> usize;
}

pub trait AfAin1 {
   fn af_ain1(&self) -> usize;
}

pub trait AfU1dcd {
   fn af_u1dcd(&self) -> usize;
}

pub trait AfAin0 {
   fn af_ain0(&self) -> usize;
}

pub trait AfU1dtr {
   fn af_u1dtr(&self) -> usize;
}

pub trait AfAin9 {
   fn af_ain9(&self) -> usize;
}

pub trait AfU1ri {
   fn af_u1ri(&self) -> usize;
}

pub trait AfSsi1xdat0 {
   fn af_ssi1xdat0(&self) -> usize;
}

pub trait AfAin8 {
   fn af_ain8(&self) -> usize;
}

pub trait AfSsi1xdat1 {
   fn af_ssi1xdat1(&self) -> usize;
}

pub trait AfEn0led0 {
   fn af_en0led0(&self) -> usize;
}

pub trait AfM0pwm0 {
   fn af_m0pwm0(&self) -> usize;
}

pub trait AfSsi3xdat1 {
   fn af_ssi3xdat1(&self) -> usize;
}

pub trait AfTrd2 {
   fn af_trd2(&self) -> usize;
}

pub trait AfEn0led2 {
   fn af_en0led2(&self) -> usize;
}

pub trait AfM0pwm1 {
   fn af_m0pwm1(&self) -> usize;
}

pub trait AfSsi3xdat0 {
   fn af_ssi3xdat0(&self) -> usize;
}

pub trait AfTrd1 {
   fn af_trd1(&self) -> usize;
}

pub trait AfM0pwm2 {
   fn af_m0pwm2(&self) -> usize;
}

pub trait AfSsi3fss {
   fn af_ssi3fss(&self) -> usize;
}

pub trait AfTrd0 {
   fn af_trd0(&self) -> usize;
}

pub trait AfM0pwm3 {
   fn af_m0pwm3(&self) -> usize;
}

pub trait AfSsi3clk {
   fn af_ssi3clk(&self) -> usize;
}

pub trait AfTrclk {
   fn af_trclk(&self) -> usize;
}

pub trait AfEn0led1 {
   fn af_en0led1(&self) -> usize;
}

pub trait AfM0fault0 {
   fn af_m0fault0(&self) -> usize;
}

pub trait AfSsi3xdat2 {
   fn af_ssi3xdat2(&self) -> usize;
}

pub trait AfTrd3 {
   fn af_trd3(&self) -> usize;
}

pub trait AfI2c1scl {
   fn af_i2c1scl(&self) -> usize;
}

pub trait AfEn0pps {
   fn af_en0pps(&self) -> usize;
}

pub trait AfM0pwm4 {
   fn af_m0pwm4(&self) -> usize;
}

pub trait AfEpi0s11 {
   fn af_epi0s11(&self) -> usize;
}

pub trait AfI2c1sda {
   fn af_i2c1sda(&self) -> usize;
}

pub trait AfM0pwm5 {
   fn af_m0pwm5(&self) -> usize;
}

pub trait AfEpi0s10 {
   fn af_epi0s10(&self) -> usize;
}

pub trait AfEpi0s0 {
   fn af_epi0s0(&self) -> usize;
}

pub trait AfEpi0s1 {
   fn af_epi0s1(&self) -> usize;
}

pub trait AfU0dcd {
   fn af_u0dcd(&self) -> usize;
}

pub trait AfEpi0s2 {
   fn af_epi0s2(&self) -> usize;
}

pub trait AfU0dsr {
   fn af_u0dsr(&self) -> usize;
}

pub trait AfEpi0s3 {
   fn af_epi0s3(&self) -> usize;
}

pub trait AfAin16 {
   fn af_ain16(&self) -> usize;
}

pub trait AfAin17 {
   fn af_ain17(&self) -> usize;
}

pub trait AfAin18 {
   fn af_ain18(&self) -> usize;
}

pub trait AfU4rts {
   fn af_u4rts(&self) -> usize;
}

pub trait AfAin19 {
   fn af_ain19(&self) -> usize;
}

pub trait AfU4cts {
   fn af_u4cts(&self) -> usize;
}

pub trait AfI2c3scl {
   fn af_i2c3scl(&self) -> usize;
}

pub trait AfM0pwm6 {
   fn af_m0pwm6(&self) -> usize;
}

pub trait AfEpi0s32 {
   fn af_epi0s32(&self) -> usize;
}

pub trait AfI2c3sda {
   fn af_i2c3sda(&self) -> usize;
}

pub trait AfM0pwm7 {
   fn af_m0pwm7(&self) -> usize;
}

pub trait AfEpi0s31 {
   fn af_epi0s31(&self) -> usize;
}

pub trait AfI2c4scl {
   fn af_i2c4scl(&self) -> usize;
}

pub trait AfM0fault1 {
   fn af_m0fault1(&self) -> usize;
}

pub trait AfEpi0s25 {
   fn af_epi0s25(&self) -> usize;
}

pub trait AfU0ri {
   fn af_u0ri(&self) -> usize;
}

pub trait AfI2c4sda {
   fn af_i2c4sda(&self) -> usize;
}

pub trait AfM0fault2 {
   fn af_m0fault2(&self) -> usize;
}

pub trait AfEpi0s24 {
   fn af_epi0s24(&self) -> usize;
}

pub trait AfI2c2sda {
   fn af_i2c2sda(&self) -> usize;
}

pub trait AfM0fault3 {
   fn af_m0fault3(&self) -> usize;
}

pub trait AfUsb0d0 {
   fn af_usb0d0(&self) -> usize;
}

pub trait AfEpi0s16 {
   fn af_epi0s16(&self) -> usize;
}

pub trait AfI2c2scl {
   fn af_i2c2scl(&self) -> usize;
}

pub trait AfPha0 {
   fn af_pha0(&self) -> usize;
}

pub trait AfUsb0d1 {
   fn af_usb0d1(&self) -> usize;
}

pub trait AfEpi0s17 {
   fn af_epi0s17(&self) -> usize;
}

pub trait AfPhb0 {
   fn af_phb0(&self) -> usize;
}

pub trait AfUsb0d2 {
   fn af_usb0d2(&self) -> usize;
}

pub trait AfEpi0s18 {
   fn af_epi0s18(&self) -> usize;
}

pub trait AfIdx0 {
   fn af_idx0(&self) -> usize;
}

pub trait AfUsb0d3 {
   fn af_usb0d3(&self) -> usize;
}

pub trait AfEpi0s19 {
   fn af_epi0s19(&self) -> usize;
}

pub trait AfUsb0d4 {
   fn af_usb0d4(&self) -> usize;
}

pub trait AfEpi0s26 {
   fn af_epi0s26(&self) -> usize;
}

pub trait AfUsb0d5 {
   fn af_usb0d5(&self) -> usize;
}

pub trait AfEpi0s33 {
   fn af_epi0s33(&self) -> usize;
}

pub trait AfUsb0dp {
   fn af_usb0dp(&self) -> usize;
}

pub trait AfUsb0dm {
   fn af_usb0dm(&self) -> usize;
}

pub trait AfEpi0s15 {
   fn af_epi0s15(&self) -> usize;
}

pub trait AfEpi0s14 {
   fn af_epi0s14(&self) -> usize;
}

pub trait AfEpi0s13 {
   fn af_epi0s13(&self) -> usize;
}

pub trait AfEpi0s12 {
   fn af_epi0s12(&self) -> usize;
}

pub trait AfTmpr3 {
   fn af_tmpr3(&self) -> usize;
}

pub trait AfTmpr2 {
   fn af_tmpr2(&self) -> usize;
}

pub trait AfTmpr1 {
   fn af_tmpr1(&self) -> usize;
}

pub trait AfTmpr0 {
   fn af_tmpr0(&self) -> usize;
}

pub trait AfU1cts {
   fn af_u1cts(&self) -> usize;
}

pub trait AfEpi0s29 {
   fn af_epi0s29(&self) -> usize;
}

pub trait AfEpi0s30 {
   fn af_epi0s30(&self) -> usize;
}

pub trait AfU3rts {
   fn af_u3rts(&self) -> usize;
}

pub trait AfEpi0s34 {
   fn af_epi0s34(&self) -> usize;
}

pub trait AfU3cts {
   fn af_u3cts(&self) -> usize;
}

pub trait AfEpi0s35 {
   fn af_epi0s35(&self) -> usize;
}

pub trait AfC2Pos {
   fn af_c2_pos(&self) -> usize;
}

pub trait AfU6rx {
   fn af_u6rx(&self) -> usize;
}

pub trait AfSsi3xdat {
   fn af_ssi3xdat(&self) -> usize;
}

pub trait AfC2Neg {
   fn af_c2_neg(&self) -> usize;
}

pub trait AfU6tx {
   fn af_u6tx(&self) -> usize;
}

pub trait AfU0dtr {
   fn af_u0dtr(&self) -> usize;
}

pub trait AfUsb0nxt {
   fn af_usb0nxt(&self) -> usize;
}

pub trait AfUsb0dir {
   fn af_usb0dir(&self) -> usize;
}

pub trait AfUsb0d7 {
   fn af_usb0d7(&self) -> usize;
}

pub trait AfUsb0d6 {
   fn af_usb0d6(&self) -> usize;
}

pub trait AfEpi0s20 {
   fn af_epi0s20(&self) -> usize;
}

pub trait AfEpi0s21 {
   fn af_epi0s21(&self) -> usize;
}

pub trait AfEpi0s22 {
   fn af_epi0s22(&self) -> usize;
}

pub trait AfEpi0s23 {
   fn af_epi0s23(&self) -> usize;
}

pub trait AfDivsclk {
   fn af_divsclk(&self) -> usize;
}

pub const PA0: Pa0 = Pa0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa0 {}

impl Pin for Pa0 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 0 }
}

impl AfU0rx for Pa0 {
   fn af_u0rx(&self) -> usize { 1 }
}

impl AfI2c9scl for Pa0 {
   fn af_i2c9scl(&self) -> usize { 2 }
}

impl AfT0ccp0 for Pa0 {
   fn af_t0ccp0(&self) -> usize { 3 }
}

impl AfCan0rx for Pa0 {
   fn af_can0rx(&self) -> usize { 7 }
}

pub const PA1: Pa1 = Pa1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa1 {}

impl Pin for Pa1 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 1 }
}

impl AfU0tx for Pa1 {
   fn af_u0tx(&self) -> usize { 1 }
}

impl AfI2c9sda for Pa1 {
   fn af_i2c9sda(&self) -> usize { 2 }
}

impl AfT0ccp1 for Pa1 {
   fn af_t0ccp1(&self) -> usize { 3 }
}

impl AfCan0tx for Pa1 {
   fn af_can0tx(&self) -> usize { 7 }
}

pub const PA2: Pa2 = Pa2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa2 {}

impl Pin for Pa2 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 2 }
}

impl AfU4rx for Pa2 {
   fn af_u4rx(&self) -> usize { 1 }
}

impl AfI2c8scl for Pa2 {
   fn af_i2c8scl(&self) -> usize { 2 }
}

impl AfT1ccp0 for Pa2 {
   fn af_t1ccp0(&self) -> usize { 3 }
}

impl AfSsi0clk for Pa2 {
   fn af_ssi0clk(&self) -> usize { 15 }
}

pub const PA3: Pa3 = Pa3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa3 {}

impl Pin for Pa3 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 3 }
}

impl AfU4tx for Pa3 {
   fn af_u4tx(&self) -> usize { 1 }
}

impl AfI2c8sda for Pa3 {
   fn af_i2c8sda(&self) -> usize { 2 }
}

impl AfT1ccp1 for Pa3 {
   fn af_t1ccp1(&self) -> usize { 3 }
}

impl AfSsi0fss for Pa3 {
   fn af_ssi0fss(&self) -> usize { 15 }
}

pub const PA4: Pa4 = Pa4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa4 {}

impl Pin for Pa4 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 4 }
}

impl AfU3rx for Pa4 {
   fn af_u3rx(&self) -> usize { 1 }
}

impl AfI2c7scl for Pa4 {
   fn af_i2c7scl(&self) -> usize { 2 }
}

impl AfT2ccp0 for Pa4 {
   fn af_t2ccp0(&self) -> usize { 3 }
}

impl AfSsi0xdat0 for Pa4 {
   fn af_ssi0xdat0(&self) -> usize { 15 }
}

pub const PA5: Pa5 = Pa5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa5 {}

impl Pin for Pa5 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 5 }
}

impl AfU3tx for Pa5 {
   fn af_u3tx(&self) -> usize { 1 }
}

impl AfI2c7sda for Pa5 {
   fn af_i2c7sda(&self) -> usize { 2 }
}

impl AfT2ccp1 for Pa5 {
   fn af_t2ccp1(&self) -> usize { 3 }
}

impl AfSsi0xdat1 for Pa5 {
   fn af_ssi0xdat1(&self) -> usize { 15 }
}

pub const PA6: Pa6 = Pa6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa6 {}

impl Pin for Pa6 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 6 }
}

impl AfU2rx for Pa6 {
   fn af_u2rx(&self) -> usize { 1 }
}

impl AfI2c6scl for Pa6 {
   fn af_i2c6scl(&self) -> usize { 2 }
}

impl AfT3ccp0 for Pa6 {
   fn af_t3ccp0(&self) -> usize { 3 }
}

impl AfUsb0epen for Pa6 {
   fn af_usb0epen(&self) -> usize { 5 }
}

impl AfSsi0xdat2 for Pa6 {
   fn af_ssi0xdat2(&self) -> usize { 13 }
}

impl AfEpi0s8 for Pa6 {
   fn af_epi0s8(&self) -> usize { 15 }
}

pub const PA7: Pa7 = Pa7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pa7 {}

impl Pin for Pa7 {
   fn port(&self) -> Gpio { GPIOA }
   fn index(&self) -> usize { 7 }
}

impl AfU2tx for Pa7 {
   fn af_u2tx(&self) -> usize { 1 }
}

impl AfI2c6sda for Pa7 {
   fn af_i2c6sda(&self) -> usize { 2 }
}

impl AfT3ccp1 for Pa7 {
   fn af_t3ccp1(&self) -> usize { 3 }
}

impl AfUsb0pflt for Pa7 {
   fn af_usb0pflt(&self) -> usize { 5 }
}

impl AfUsb0epen for Pa7 {
   fn af_usb0epen(&self) -> usize { 11 }
}

impl AfSsi0xdat3 for Pa7 {
   fn af_ssi0xdat3(&self) -> usize { 13 }
}

impl AfEpi0s9 for Pa7 {
   fn af_epi0s9(&self) -> usize { 15 }
}

pub const PB0: Pb0 = Pb0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb0 {}

impl Pin for Pb0 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 0 }
}

impl AfUsb0id for Pb0 {
   fn af_usb0id(&self) -> usize { 0 }
}

impl AfU1rx for Pb0 {
   fn af_u1rx(&self) -> usize { 1 }
}

impl AfI2c5scl for Pb0 {
   fn af_i2c5scl(&self) -> usize { 2 }
}

impl AfT4ccp0 for Pb0 {
   fn af_t4ccp0(&self) -> usize { 3 }
}

impl AfCan1rx for Pb0 {
   fn af_can1rx(&self) -> usize { 7 }
}

pub const PB1: Pb1 = Pb1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb1 {}

impl Pin for Pb1 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 1 }
}

impl AfUsb0vbus for Pb1 {
   fn af_usb0vbus(&self) -> usize { 0 }
}

impl AfU1tx for Pb1 {
   fn af_u1tx(&self) -> usize { 1 }
}

impl AfI2c5sda for Pb1 {
   fn af_i2c5sda(&self) -> usize { 2 }
}

impl AfT4ccp1 for Pb1 {
   fn af_t4ccp1(&self) -> usize { 3 }
}

impl AfCan1tx for Pb1 {
   fn af_can1tx(&self) -> usize { 7 }
}

pub const PB2: Pb2 = Pb2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb2 {}

impl Pin for Pb2 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 2 }
}

impl AfI2c0scl for Pb2 {
   fn af_i2c0scl(&self) -> usize { 2 }
}

impl AfT5ccp0 for Pb2 {
   fn af_t5ccp0(&self) -> usize { 3 }
}

impl AfUsb0stp for Pb2 {
   fn af_usb0stp(&self) -> usize { 14 }
}

impl AfEpi0s27 for Pb2 {
   fn af_epi0s27(&self) -> usize { 15 }
}

pub const PB3: Pb3 = Pb3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb3 {}

impl Pin for Pb3 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 3 }
}

impl AfI2c0sda for Pb3 {
   fn af_i2c0sda(&self) -> usize { 2 }
}

impl AfT5ccp1 for Pb3 {
   fn af_t5ccp1(&self) -> usize { 3 }
}

impl AfUsb0clk for Pb3 {
   fn af_usb0clk(&self) -> usize { 14 }
}

impl AfEpi0s28 for Pb3 {
   fn af_epi0s28(&self) -> usize { 15 }
}

pub const PB4: Pb4 = Pb4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb4 {}

impl Pin for Pb4 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 4 }
}

impl AfAin10 for Pb4 {
   fn af_ain10(&self) -> usize { 0 }
}

impl AfU0cts for Pb4 {
   fn af_u0cts(&self) -> usize { 1 }
}

impl AfI2c5scl for Pb4 {
   fn af_i2c5scl(&self) -> usize { 2 }
}

impl AfSsi1fss for Pb4 {
   fn af_ssi1fss(&self) -> usize { 15 }
}

pub const PB5: Pb5 = Pb5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pb5 {}

impl Pin for Pb5 {
   fn port(&self) -> Gpio { GPIOB }
   fn index(&self) -> usize { 5 }
}

impl AfAin11 for Pb5 {
   fn af_ain11(&self) -> usize { 0 }
}

impl AfU0rts for Pb5 {
   fn af_u0rts(&self) -> usize { 1 }
}

impl AfI2c5sda for Pb5 {
   fn af_i2c5sda(&self) -> usize { 2 }
}

impl AfSsi1clk for Pb5 {
   fn af_ssi1clk(&self) -> usize { 15 }
}

pub const PC0: Pc0 = Pc0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc0 {}

impl Pin for Pc0 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 0 }
}

impl AfTck for Pc0 {
   fn af_tck(&self) -> usize { 1 }
}

impl AfSwclk for Pc0 {
   fn af_swclk(&self) -> usize { 1 }
}

pub const PC1: Pc1 = Pc1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc1 {}

impl Pin for Pc1 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 1 }
}

impl AfTms for Pc1 {
   fn af_tms(&self) -> usize { 1 }
}

impl AfSwdio for Pc1 {
   fn af_swdio(&self) -> usize { 1 }
}

pub const PC2: Pc2 = Pc2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc2 {}

impl Pin for Pc2 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 2 }
}

impl AfTdi for Pc2 {
   fn af_tdi(&self) -> usize { 1 }
}

pub const PC3: Pc3 = Pc3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc3 {}

impl Pin for Pc3 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 3 }
}

impl AfTdo for Pc3 {
   fn af_tdo(&self) -> usize { 1 }
}

impl AfSwo for Pc3 {
   fn af_swo(&self) -> usize { 1 }
}

pub const PC4: Pc4 = Pc4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc4 {}

impl Pin for Pc4 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 4 }
}

impl AfC1Neg for Pc4 {
   fn af_c1_neg(&self) -> usize { 0 }
}

impl AfU7rx for Pc4 {
   fn af_u7rx(&self) -> usize { 1 }
}

impl AfEpi0s7 for Pc4 {
   fn af_epi0s7(&self) -> usize { 15 }
}

pub const PC5: Pc5 = Pc5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc5 {}

impl Pin for Pc5 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 5 }
}

impl AfC1Pos for Pc5 {
   fn af_c1_pos(&self) -> usize { 0 }
}

impl AfU7tx for Pc5 {
   fn af_u7tx(&self) -> usize { 1 }
}

impl AfRtcclk for Pc5 {
   fn af_rtcclk(&self) -> usize { 7 }
}

impl AfEpi0s6 for Pc5 {
   fn af_epi0s6(&self) -> usize { 15 }
}

pub const PC6: Pc6 = Pc6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc6 {}

impl Pin for Pc6 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 6 }
}

impl AfC0Neg for Pc6 {
   fn af_c0_neg(&self) -> usize { 0 }
}

impl AfU5rx for Pc6 {
   fn af_u5rx(&self) -> usize { 1 }
}

impl AfEpi0s5 for Pc6 {
   fn af_epi0s5(&self) -> usize { 15 }
}

pub const PC7: Pc7 = Pc7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pc7 {}

impl Pin for Pc7 {
   fn port(&self) -> Gpio { GPIOC }
   fn index(&self) -> usize { 7 }
}

impl AfC0Pos for Pc7 {
   fn af_c0_pos(&self) -> usize { 0 }
}

impl AfU5tx for Pc7 {
   fn af_u5tx(&self) -> usize { 1 }
}

impl AfEpi0s4 for Pc7 {
   fn af_epi0s4(&self) -> usize { 15 }
}

pub const PD0: Pd0 = Pd0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd0 {}

impl Pin for Pd0 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 0 }
}

impl AfAin15 for Pd0 {
   fn af_ain15(&self) -> usize { 0 }
}

impl AfI2c7scl for Pd0 {
   fn af_i2c7scl(&self) -> usize { 2 }
}

impl AfT0ccp0 for Pd0 {
   fn af_t0ccp0(&self) -> usize { 3 }
}

impl AfC0o for Pd0 {
   fn af_c0o(&self) -> usize { 5 }
}

impl AfSsi2xdat1 for Pd0 {
   fn af_ssi2xdat1(&self) -> usize { 15 }
}

pub const PD1: Pd1 = Pd1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd1 {}

impl Pin for Pd1 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 1 }
}

impl AfAin14 for Pd1 {
   fn af_ain14(&self) -> usize { 0 }
}

impl AfI2c7sda for Pd1 {
   fn af_i2c7sda(&self) -> usize { 2 }
}

impl AfT0ccp1 for Pd1 {
   fn af_t0ccp1(&self) -> usize { 3 }
}

impl AfC1o for Pd1 {
   fn af_c1o(&self) -> usize { 5 }
}

impl AfSsi2xdat0 for Pd1 {
   fn af_ssi2xdat0(&self) -> usize { 15 }
}

pub const PD2: Pd2 = Pd2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd2 {}

impl Pin for Pd2 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 2 }
}

impl AfAin13 for Pd2 {
   fn af_ain13(&self) -> usize { 0 }
}

impl AfI2c8scl for Pd2 {
   fn af_i2c8scl(&self) -> usize { 2 }
}

impl AfT1ccp0 for Pd2 {
   fn af_t1ccp0(&self) -> usize { 3 }
}

impl AfC2o for Pd2 {
   fn af_c2o(&self) -> usize { 5 }
}

impl AfSsi2fss for Pd2 {
   fn af_ssi2fss(&self) -> usize { 15 }
}

pub const PD3: Pd3 = Pd3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd3 {}

impl Pin for Pd3 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 3 }
}

impl AfAin12 for Pd3 {
   fn af_ain12(&self) -> usize { 0 }
}

impl AfI2c8sda for Pd3 {
   fn af_i2c8sda(&self) -> usize { 2 }
}

impl AfT1ccp1 for Pd3 {
   fn af_t1ccp1(&self) -> usize { 3 }
}

impl AfSsi2clk for Pd3 {
   fn af_ssi2clk(&self) -> usize { 15 }
}

pub const PD4: Pd4 = Pd4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd4 {}

impl Pin for Pd4 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 4 }
}

impl AfAin7 for Pd4 {
   fn af_ain7(&self) -> usize { 0 }
}

impl AfU2rx for Pd4 {
   fn af_u2rx(&self) -> usize { 1 }
}

impl AfT3ccp0 for Pd4 {
   fn af_t3ccp0(&self) -> usize { 3 }
}

impl AfSsi1xdat2 for Pd4 {
   fn af_ssi1xdat2(&self) -> usize { 15 }
}

pub const PD5: Pd5 = Pd5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd5 {}

impl Pin for Pd5 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 5 }
}

impl AfAin6 for Pd5 {
   fn af_ain6(&self) -> usize { 0 }
}

impl AfU2tx for Pd5 {
   fn af_u2tx(&self) -> usize { 1 }
}

impl AfT3ccp1 for Pd5 {
   fn af_t3ccp1(&self) -> usize { 3 }
}

impl AfSsi1xdat3 for Pd5 {
   fn af_ssi1xdat3(&self) -> usize { 15 }
}

pub const PD6: Pd6 = Pd6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd6 {}

impl Pin for Pd6 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 6 }
}

impl AfAin5 for Pd6 {
   fn af_ain5(&self) -> usize { 0 }
}

impl AfU2rts for Pd6 {
   fn af_u2rts(&self) -> usize { 1 }
}

impl AfT4ccp0 for Pd6 {
   fn af_t4ccp0(&self) -> usize { 3 }
}

impl AfUsb0epen for Pd6 {
   fn af_usb0epen(&self) -> usize { 5 }
}

impl AfSsi2xdat3 for Pd6 {
   fn af_ssi2xdat3(&self) -> usize { 15 }
}

pub const PD7: Pd7 = Pd7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pd7 {}

impl Pin for Pd7 {
   fn port(&self) -> Gpio { GPIOD }
   fn index(&self) -> usize { 7 }
}

impl AfAin4 for Pd7 {
   fn af_ain4(&self) -> usize { 0 }
}

impl AfU2cts for Pd7 {
   fn af_u2cts(&self) -> usize { 1 }
}

impl AfT4ccp1 for Pd7 {
   fn af_t4ccp1(&self) -> usize { 3 }
}

impl AfUsb0pflt for Pd7 {
   fn af_usb0pflt(&self) -> usize { 5 }
}

impl AfNmi for Pd7 {
   fn af_nmi(&self) -> usize { 8 }
}

impl AfSsi2xdat2 for Pd7 {
   fn af_ssi2xdat2(&self) -> usize { 15 }
}

pub const PE0: Pe0 = Pe0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe0 {}

impl Pin for Pe0 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 0 }
}

impl AfAin3 for Pe0 {
   fn af_ain3(&self) -> usize { 0 }
}

impl AfU1rts for Pe0 {
   fn af_u1rts(&self) -> usize { 1 }
}

pub const PE1: Pe1 = Pe1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe1 {}

impl Pin for Pe1 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 1 }
}

impl AfAin2 for Pe1 {
   fn af_ain2(&self) -> usize { 0 }
}

impl AfU1dsr for Pe1 {
   fn af_u1dsr(&self) -> usize { 1 }
}

pub const PE2: Pe2 = Pe2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe2 {}

impl Pin for Pe2 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 2 }
}

impl AfAin1 for Pe2 {
   fn af_ain1(&self) -> usize { 0 }
}

impl AfU1dcd for Pe2 {
   fn af_u1dcd(&self) -> usize { 1 }
}

pub const PE3: Pe3 = Pe3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe3 {}

impl Pin for Pe3 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 3 }
}

impl AfAin0 for Pe3 {
   fn af_ain0(&self) -> usize { 0 }
}

impl AfU1dtr for Pe3 {
   fn af_u1dtr(&self) -> usize { 1 }
}

pub const PE4: Pe4 = Pe4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe4 {}

impl Pin for Pe4 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 4 }
}

impl AfAin9 for Pe4 {
   fn af_ain9(&self) -> usize { 0 }
}

impl AfU1ri for Pe4 {
   fn af_u1ri(&self) -> usize { 1 }
}

impl AfSsi1xdat0 for Pe4 {
   fn af_ssi1xdat0(&self) -> usize { 15 }
}

pub const PE5: Pe5 = Pe5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pe5 {}

impl Pin for Pe5 {
   fn port(&self) -> Gpio { GPIOE }
   fn index(&self) -> usize { 5 }
}

impl AfAin8 for Pe5 {
   fn af_ain8(&self) -> usize { 0 }
}

impl AfSsi1xdat1 for Pe5 {
   fn af_ssi1xdat1(&self) -> usize { 15 }
}

pub const PF0: Pf0 = Pf0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf0 {}

impl Pin for Pf0 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 0 }
}

impl AfEn0led0 for Pf0 {
   fn af_en0led0(&self) -> usize { 5 }
}

impl AfM0pwm0 for Pf0 {
   fn af_m0pwm0(&self) -> usize { 6 }
}

impl AfSsi3xdat1 for Pf0 {
   fn af_ssi3xdat1(&self) -> usize { 14 }
}

impl AfTrd2 for Pf0 {
   fn af_trd2(&self) -> usize { 15 }
}

pub const PF1: Pf1 = Pf1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf1 {}

impl Pin for Pf1 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 1 }
}

impl AfEn0led2 for Pf1 {
   fn af_en0led2(&self) -> usize { 5 }
}

impl AfM0pwm1 for Pf1 {
   fn af_m0pwm1(&self) -> usize { 6 }
}

impl AfSsi3xdat0 for Pf1 {
   fn af_ssi3xdat0(&self) -> usize { 14 }
}

impl AfTrd1 for Pf1 {
   fn af_trd1(&self) -> usize { 15 }
}

pub const PF2: Pf2 = Pf2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf2 {}

impl Pin for Pf2 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 2 }
}

impl AfM0pwm2 for Pf2 {
   fn af_m0pwm2(&self) -> usize { 6 }
}

impl AfSsi3fss for Pf2 {
   fn af_ssi3fss(&self) -> usize { 14 }
}

impl AfTrd0 for Pf2 {
   fn af_trd0(&self) -> usize { 15 }
}

pub const PF3: Pf3 = Pf3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf3 {}

impl Pin for Pf3 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 3 }
}

impl AfM0pwm3 for Pf3 {
   fn af_m0pwm3(&self) -> usize { 6 }
}

impl AfSsi3clk for Pf3 {
   fn af_ssi3clk(&self) -> usize { 14 }
}

impl AfTrclk for Pf3 {
   fn af_trclk(&self) -> usize { 15 }
}

pub const PF4: Pf4 = Pf4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pf4 {}

impl Pin for Pf4 {
   fn port(&self) -> Gpio { GPIOF }
   fn index(&self) -> usize { 4 }
}

impl AfEn0led1 for Pf4 {
   fn af_en0led1(&self) -> usize { 5 }
}

impl AfM0fault0 for Pf4 {
   fn af_m0fault0(&self) -> usize { 6 }
}

impl AfSsi3xdat2 for Pf4 {
   fn af_ssi3xdat2(&self) -> usize { 14 }
}

impl AfTrd3 for Pf4 {
   fn af_trd3(&self) -> usize { 15 }
}

pub const PG0: Pg0 = Pg0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg0 {}

impl Pin for Pg0 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 0 }
}

impl AfI2c1scl for Pg0 {
   fn af_i2c1scl(&self) -> usize { 2 }
}

impl AfEn0pps for Pg0 {
   fn af_en0pps(&self) -> usize { 5 }
}

impl AfM0pwm4 for Pg0 {
   fn af_m0pwm4(&self) -> usize { 6 }
}

impl AfEpi0s11 for Pg0 {
   fn af_epi0s11(&self) -> usize { 15 }
}

pub const PG1: Pg1 = Pg1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pg1 {}

impl Pin for Pg1 {
   fn port(&self) -> Gpio { GPIOG }
   fn index(&self) -> usize { 1 }
}

impl AfI2c1sda for Pg1 {
   fn af_i2c1sda(&self) -> usize { 2 }
}

impl AfM0pwm5 for Pg1 {
   fn af_m0pwm5(&self) -> usize { 6 }
}

impl AfEpi0s10 for Pg1 {
   fn af_epi0s10(&self) -> usize { 15 }
}

pub const PH0: Ph0 = Ph0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph0 {}

impl Pin for Ph0 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 0 }
}

impl AfU0rts for Ph0 {
   fn af_u0rts(&self) -> usize { 1 }
}

impl AfEpi0s0 for Ph0 {
   fn af_epi0s0(&self) -> usize { 15 }
}

pub const PH1: Ph1 = Ph1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph1 {}

impl Pin for Ph1 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 1 }
}

impl AfU0cts for Ph1 {
   fn af_u0cts(&self) -> usize { 1 }
}

impl AfEpi0s1 for Ph1 {
   fn af_epi0s1(&self) -> usize { 15 }
}

pub const PH2: Ph2 = Ph2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph2 {}

impl Pin for Ph2 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 2 }
}

impl AfU0dcd for Ph2 {
   fn af_u0dcd(&self) -> usize { 1 }
}

impl AfEpi0s2 for Ph2 {
   fn af_epi0s2(&self) -> usize { 15 }
}

pub const PH3: Ph3 = Ph3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Ph3 {}

impl Pin for Ph3 {
   fn port(&self) -> Gpio { GPIOH }
   fn index(&self) -> usize { 3 }
}

impl AfU0dsr for Ph3 {
   fn af_u0dsr(&self) -> usize { 1 }
}

impl AfEpi0s3 for Ph3 {
   fn af_epi0s3(&self) -> usize { 15 }
}

pub const PJ0: Pj0 = Pj0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj0 {}

impl Pin for Pj0 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 0 }
}

impl AfU3rx for Pj0 {
   fn af_u3rx(&self) -> usize { 1 }
}

impl AfEn0pps for Pj0 {
   fn af_en0pps(&self) -> usize { 5 }
}

pub const PJ1: Pj1 = Pj1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pj1 {}

impl Pin for Pj1 {
   fn port(&self) -> Gpio { GPIOJ }
   fn index(&self) -> usize { 1 }
}

impl AfU3tx for Pj1 {
   fn af_u3tx(&self) -> usize { 1 }
}

pub const PK0: Pk0 = Pk0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk0 {}

impl Pin for Pk0 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 0 }
}

impl AfAin16 for Pk0 {
   fn af_ain16(&self) -> usize { 0 }
}

impl AfU4rx for Pk0 {
   fn af_u4rx(&self) -> usize { 1 }
}

impl AfEpi0s0 for Pk0 {
   fn af_epi0s0(&self) -> usize { 15 }
}

pub const PK1: Pk1 = Pk1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk1 {}

impl Pin for Pk1 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 1 }
}

impl AfAin17 for Pk1 {
   fn af_ain17(&self) -> usize { 0 }
}

impl AfU4tx for Pk1 {
   fn af_u4tx(&self) -> usize { 1 }
}

impl AfEpi0s1 for Pk1 {
   fn af_epi0s1(&self) -> usize { 15 }
}

pub const PK2: Pk2 = Pk2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk2 {}

impl Pin for Pk2 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 2 }
}

impl AfAin18 for Pk2 {
   fn af_ain18(&self) -> usize { 0 }
}

impl AfU4rts for Pk2 {
   fn af_u4rts(&self) -> usize { 1 }
}

impl AfEpi0s2 for Pk2 {
   fn af_epi0s2(&self) -> usize { 15 }
}

pub const PK3: Pk3 = Pk3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk3 {}

impl Pin for Pk3 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 3 }
}

impl AfAin19 for Pk3 {
   fn af_ain19(&self) -> usize { 0 }
}

impl AfU4cts for Pk3 {
   fn af_u4cts(&self) -> usize { 1 }
}

impl AfEpi0s3 for Pk3 {
   fn af_epi0s3(&self) -> usize { 15 }
}

pub const PK4: Pk4 = Pk4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk4 {}

impl Pin for Pk4 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 4 }
}

impl AfI2c3scl for Pk4 {
   fn af_i2c3scl(&self) -> usize { 2 }
}

impl AfEn0led0 for Pk4 {
   fn af_en0led0(&self) -> usize { 5 }
}

impl AfM0pwm6 for Pk4 {
   fn af_m0pwm6(&self) -> usize { 6 }
}

impl AfEpi0s32 for Pk4 {
   fn af_epi0s32(&self) -> usize { 15 }
}

pub const PK5: Pk5 = Pk5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk5 {}

impl Pin for Pk5 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 5 }
}

impl AfI2c3sda for Pk5 {
   fn af_i2c3sda(&self) -> usize { 2 }
}

impl AfEn0led2 for Pk5 {
   fn af_en0led2(&self) -> usize { 5 }
}

impl AfM0pwm7 for Pk5 {
   fn af_m0pwm7(&self) -> usize { 6 }
}

impl AfEpi0s31 for Pk5 {
   fn af_epi0s31(&self) -> usize { 15 }
}

pub const PK6: Pk6 = Pk6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk6 {}

impl Pin for Pk6 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 6 }
}

impl AfI2c4scl for Pk6 {
   fn af_i2c4scl(&self) -> usize { 2 }
}

impl AfEn0led1 for Pk6 {
   fn af_en0led1(&self) -> usize { 5 }
}

impl AfM0fault1 for Pk6 {
   fn af_m0fault1(&self) -> usize { 6 }
}

impl AfEpi0s25 for Pk6 {
   fn af_epi0s25(&self) -> usize { 15 }
}

pub const PK7: Pk7 = Pk7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pk7 {}

impl Pin for Pk7 {
   fn port(&self) -> Gpio { GPIOK }
   fn index(&self) -> usize { 7 }
}

impl AfU0ri for Pk7 {
   fn af_u0ri(&self) -> usize { 1 }
}

impl AfI2c4sda for Pk7 {
   fn af_i2c4sda(&self) -> usize { 2 }
}

impl AfRtcclk for Pk7 {
   fn af_rtcclk(&self) -> usize { 5 }
}

impl AfM0fault2 for Pk7 {
   fn af_m0fault2(&self) -> usize { 6 }
}

impl AfEpi0s24 for Pk7 {
   fn af_epi0s24(&self) -> usize { 15 }
}

pub const PL0: Pl0 = Pl0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pl0 {}

impl Pin for Pl0 {
   fn port(&self) -> Gpio { GPIOL }
   fn index(&self) -> usize { 0 }
}

impl AfI2c2sda for Pl0 {
   fn af_i2c2sda(&self) -> usize { 2 }
}

impl AfM0fault3 for Pl0 {
   fn af_m0fault3(&self) -> usize { 6 }
}

impl AfUsb0d0 for Pl0 {
   fn af_usb0d0(&self) -> usize { 14 }
}

impl AfEpi0s16 for Pl0 {
   fn af_epi0s16(&self) -> usize { 15 }
}

pub const PL1: Pl1 = Pl1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pl1 {}

impl Pin for Pl1 {
   fn port(&self) -> Gpio { GPIOL }
   fn index(&self) -> usize { 1 }
}

impl AfI2c2scl for Pl1 {
   fn af_i2c2scl(&self) -> usize { 2 }
}

impl AfPha0 for Pl1 {
   fn af_pha0(&self) -> usize { 6 }
}

impl AfUsb0d1 for Pl1 {
   fn af_usb0d1(&self) -> usize { 14 }
}

impl AfEpi0s17 for Pl1 {
   fn af_epi0s17(&self) -> usize { 15 }
}

pub const PL2: Pl2 = Pl2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pl2 {}

impl Pin for Pl2 {
   fn port(&self) -> Gpio { GPIOL }
   fn index(&self) -> usize { 2 }
}

impl AfC0o for Pl2 {
   fn af_c0o(&self) -> usize { 5 }
}

impl AfPhb0 for Pl2 {
   fn af_phb0(&self) -> usize { 6 }
}

impl AfUsb0d2 for Pl2 {
   fn af_usb0d2(&self) -> usize { 14 }
}

impl AfEpi0s18 for Pl2 {
   fn af_epi0s18(&self) -> usize { 15 }
}

pub const PL3: Pl3 = Pl3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pl3 {}

impl Pin for Pl3 {
   fn port(&self) -> Gpio { GPIOL }
   fn index(&self) -> usize { 3 }
}

impl AfC1o for Pl3 {
   fn af_c1o(&self) -> usize { 5 }
}

impl AfIdx0 for Pl3 {
   fn af_idx0(&self) -> usize { 6 }
}

impl AfUsb0d3 for Pl3 {
   fn af_usb0d3(&self) -> usize { 14 }
}

impl AfEpi0s19 for Pl3 {
   fn af_epi0s19(&self) -> usize { 15 }
}

pub const PL4: Pl4 = Pl4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pl4 {}

impl Pin for Pl4 {
   fn port(&self) -> Gpio { GPIOL }
   fn index(&self) -> usize { 4 }
}

impl AfT0ccp0 for Pl4 {
   fn af_t0ccp0(&self) -> usize { 3 }
}

impl AfUsb0d4 for Pl4 {
   fn af_usb0d4(&self) -> usize { 14 }
}

impl AfEpi0s26 for Pl4 {
   fn af_epi0s26(&self) -> usize { 15 }
}

pub const PL5: Pl5 = Pl5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pl5 {}

impl Pin for Pl5 {
   fn port(&self) -> Gpio { GPIOL }
   fn index(&self) -> usize { 5 }
}

impl AfT0ccp1 for Pl5 {
   fn af_t0ccp1(&self) -> usize { 3 }
}

impl AfUsb0d5 for Pl5 {
   fn af_usb0d5(&self) -> usize { 14 }
}

impl AfEpi0s33 for Pl5 {
   fn af_epi0s33(&self) -> usize { 15 }
}

pub const PL6: Pl6 = Pl6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pl6 {}

impl Pin for Pl6 {
   fn port(&self) -> Gpio { GPIOL }
   fn index(&self) -> usize { 6 }
}

impl AfUsb0dp for Pl6 {
   fn af_usb0dp(&self) -> usize { 0 }
}

impl AfT1ccp0 for Pl6 {
   fn af_t1ccp0(&self) -> usize { 3 }
}

pub const PL7: Pl7 = Pl7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pl7 {}

impl Pin for Pl7 {
   fn port(&self) -> Gpio { GPIOL }
   fn index(&self) -> usize { 7 }
}

impl AfUsb0dm for Pl7 {
   fn af_usb0dm(&self) -> usize { 0 }
}

impl AfT1ccp1 for Pl7 {
   fn af_t1ccp1(&self) -> usize { 3 }
}

pub const PM0: Pm0 = Pm0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pm0 {}

impl Pin for Pm0 {
   fn port(&self) -> Gpio { GPIOM }
   fn index(&self) -> usize { 0 }
}

impl AfT2ccp0 for Pm0 {
   fn af_t2ccp0(&self) -> usize { 3 }
}

impl AfEpi0s15 for Pm0 {
   fn af_epi0s15(&self) -> usize { 15 }
}

pub const PM1: Pm1 = Pm1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pm1 {}

impl Pin for Pm1 {
   fn port(&self) -> Gpio { GPIOM }
   fn index(&self) -> usize { 1 }
}

impl AfT2ccp1 for Pm1 {
   fn af_t2ccp1(&self) -> usize { 3 }
}

impl AfEpi0s14 for Pm1 {
   fn af_epi0s14(&self) -> usize { 15 }
}

pub const PM2: Pm2 = Pm2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pm2 {}

impl Pin for Pm2 {
   fn port(&self) -> Gpio { GPIOM }
   fn index(&self) -> usize { 2 }
}

impl AfT3ccp0 for Pm2 {
   fn af_t3ccp0(&self) -> usize { 3 }
}

impl AfEpi0s13 for Pm2 {
   fn af_epi0s13(&self) -> usize { 15 }
}

pub const PM3: Pm3 = Pm3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pm3 {}

impl Pin for Pm3 {
   fn port(&self) -> Gpio { GPIOM }
   fn index(&self) -> usize { 3 }
}

impl AfT3ccp1 for Pm3 {
   fn af_t3ccp1(&self) -> usize { 3 }
}

impl AfEpi0s12 for Pm3 {
   fn af_epi0s12(&self) -> usize { 15 }
}

pub const PM4: Pm4 = Pm4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pm4 {}

impl Pin for Pm4 {
   fn port(&self) -> Gpio { GPIOM }
   fn index(&self) -> usize { 4 }
}

impl AfTmpr3 for Pm4 {
   fn af_tmpr3(&self) -> usize { 0 }
}

impl AfU0cts for Pm4 {
   fn af_u0cts(&self) -> usize { 1 }
}

impl AfT4ccp0 for Pm4 {
   fn af_t4ccp0(&self) -> usize { 3 }
}

pub const PM5: Pm5 = Pm5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pm5 {}

impl Pin for Pm5 {
   fn port(&self) -> Gpio { GPIOM }
   fn index(&self) -> usize { 5 }
}

impl AfTmpr2 for Pm5 {
   fn af_tmpr2(&self) -> usize { 0 }
}

impl AfU0dcd for Pm5 {
   fn af_u0dcd(&self) -> usize { 1 }
}

impl AfT4ccp1 for Pm5 {
   fn af_t4ccp1(&self) -> usize { 3 }
}

pub const PM6: Pm6 = Pm6 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pm6 {}

impl Pin for Pm6 {
   fn port(&self) -> Gpio { GPIOM }
   fn index(&self) -> usize { 6 }
}

impl AfTmpr1 for Pm6 {
   fn af_tmpr1(&self) -> usize { 0 }
}

impl AfU0dsr for Pm6 {
   fn af_u0dsr(&self) -> usize { 1 }
}

impl AfT5ccp0 for Pm6 {
   fn af_t5ccp0(&self) -> usize { 3 }
}

pub const PM7: Pm7 = Pm7 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pm7 {}

impl Pin for Pm7 {
   fn port(&self) -> Gpio { GPIOM }
   fn index(&self) -> usize { 7 }
}

impl AfTmpr0 for Pm7 {
   fn af_tmpr0(&self) -> usize { 0 }
}

impl AfU0ri for Pm7 {
   fn af_u0ri(&self) -> usize { 1 }
}

impl AfT5ccp1 for Pm7 {
   fn af_t5ccp1(&self) -> usize { 3 }
}

pub const PN0: Pn0 = Pn0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pn0 {}

impl Pin for Pn0 {
   fn port(&self) -> Gpio { GPION }
   fn index(&self) -> usize { 0 }
}

impl AfU1rts for Pn0 {
   fn af_u1rts(&self) -> usize { 1 }
}

pub const PN1: Pn1 = Pn1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pn1 {}

impl Pin for Pn1 {
   fn port(&self) -> Gpio { GPION }
   fn index(&self) -> usize { 1 }
}

impl AfU1cts for Pn1 {
   fn af_u1cts(&self) -> usize { 1 }
}

pub const PN2: Pn2 = Pn2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pn2 {}

impl Pin for Pn2 {
   fn port(&self) -> Gpio { GPION }
   fn index(&self) -> usize { 2 }
}

impl AfU1dcd for Pn2 {
   fn af_u1dcd(&self) -> usize { 1 }
}

impl AfU2rts for Pn2 {
   fn af_u2rts(&self) -> usize { 2 }
}

impl AfEpi0s29 for Pn2 {
   fn af_epi0s29(&self) -> usize { 15 }
}

pub const PN3: Pn3 = Pn3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pn3 {}

impl Pin for Pn3 {
   fn port(&self) -> Gpio { GPION }
   fn index(&self) -> usize { 3 }
}

impl AfU1dsr for Pn3 {
   fn af_u1dsr(&self) -> usize { 1 }
}

impl AfU2cts for Pn3 {
   fn af_u2cts(&self) -> usize { 2 }
}

impl AfEpi0s30 for Pn3 {
   fn af_epi0s30(&self) -> usize { 15 }
}

pub const PN4: Pn4 = Pn4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pn4 {}

impl Pin for Pn4 {
   fn port(&self) -> Gpio { GPION }
   fn index(&self) -> usize { 4 }
}

impl AfU1dtr for Pn4 {
   fn af_u1dtr(&self) -> usize { 1 }
}

impl AfU3rts for Pn4 {
   fn af_u3rts(&self) -> usize { 2 }
}

impl AfI2c2sda for Pn4 {
   fn af_i2c2sda(&self) -> usize { 3 }
}

impl AfEpi0s34 for Pn4 {
   fn af_epi0s34(&self) -> usize { 15 }
}

pub const PN5: Pn5 = Pn5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pn5 {}

impl Pin for Pn5 {
   fn port(&self) -> Gpio { GPION }
   fn index(&self) -> usize { 5 }
}

impl AfU1ri for Pn5 {
   fn af_u1ri(&self) -> usize { 1 }
}

impl AfU3cts for Pn5 {
   fn af_u3cts(&self) -> usize { 2 }
}

impl AfI2c2scl for Pn5 {
   fn af_i2c2scl(&self) -> usize { 3 }
}

impl AfEpi0s35 for Pn5 {
   fn af_epi0s35(&self) -> usize { 15 }
}

pub const PP0: Pp0 = Pp0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pp0 {}

impl Pin for Pp0 {
   fn port(&self) -> Gpio { GPIOP }
   fn index(&self) -> usize { 0 }
}

impl AfC2Pos for Pp0 {
   fn af_c2_pos(&self) -> usize { 0 }
}

impl AfU6rx for Pp0 {
   fn af_u6rx(&self) -> usize { 1 }
}

impl AfSsi3xdat for Pp0 {
   fn af_ssi3xdat(&self) -> usize { 15 }
}

pub const PP1: Pp1 = Pp1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pp1 {}

impl Pin for Pp1 {
   fn port(&self) -> Gpio { GPIOP }
   fn index(&self) -> usize { 1 }
}

impl AfC2Neg for Pp1 {
   fn af_c2_neg(&self) -> usize { 0 }
}

impl AfU6tx for Pp1 {
   fn af_u6tx(&self) -> usize { 1 }
}

impl AfSsi3xdat for Pp1 {
   fn af_ssi3xdat(&self) -> usize { 15 }
}

pub const PP2: Pp2 = Pp2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pp2 {}

impl Pin for Pp2 {
   fn port(&self) -> Gpio { GPIOP }
   fn index(&self) -> usize { 2 }
}

impl AfU0dtr for Pp2 {
   fn af_u0dtr(&self) -> usize { 1 }
}

impl AfUsb0nxt for Pp2 {
   fn af_usb0nxt(&self) -> usize { 14 }
}

impl AfEpi0s29 for Pp2 {
   fn af_epi0s29(&self) -> usize { 15 }
}

pub const PP3: Pp3 = Pp3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pp3 {}

impl Pin for Pp3 {
   fn port(&self) -> Gpio { GPIOP }
   fn index(&self) -> usize { 3 }
}

impl AfU1cts for Pp3 {
   fn af_u1cts(&self) -> usize { 1 }
}

impl AfU0dcd for Pp3 {
   fn af_u0dcd(&self) -> usize { 2 }
}

impl AfRtcclk for Pp3 {
   fn af_rtcclk(&self) -> usize { 7 }
}

impl AfUsb0dir for Pp3 {
   fn af_usb0dir(&self) -> usize { 14 }
}

impl AfEpi0s30 for Pp3 {
   fn af_epi0s30(&self) -> usize { 15 }
}

pub const PP4: Pp4 = Pp4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pp4 {}

impl Pin for Pp4 {
   fn port(&self) -> Gpio { GPIOP }
   fn index(&self) -> usize { 4 }
}

impl AfU3rts for Pp4 {
   fn af_u3rts(&self) -> usize { 1 }
}

impl AfU0dsr for Pp4 {
   fn af_u0dsr(&self) -> usize { 2 }
}

impl AfUsb0d7 for Pp4 {
   fn af_usb0d7(&self) -> usize { 14 }
}

pub const PP5: Pp5 = Pp5 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pp5 {}

impl Pin for Pp5 {
   fn port(&self) -> Gpio { GPIOP }
   fn index(&self) -> usize { 5 }
}

impl AfU3cts for Pp5 {
   fn af_u3cts(&self) -> usize { 1 }
}

impl AfI2c2scl for Pp5 {
   fn af_i2c2scl(&self) -> usize { 2 }
}

impl AfUsb0d6 for Pp5 {
   fn af_usb0d6(&self) -> usize { 14 }
}

pub const PQ0: Pq0 = Pq0 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pq0 {}

impl Pin for Pq0 {
   fn port(&self) -> Gpio { GPIOQ }
   fn index(&self) -> usize { 0 }
}

impl AfSsi3clk for Pq0 {
   fn af_ssi3clk(&self) -> usize { 14 }
}

impl AfEpi0s20 for Pq0 {
   fn af_epi0s20(&self) -> usize { 15 }
}

pub const PQ1: Pq1 = Pq1 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pq1 {}

impl Pin for Pq1 {
   fn port(&self) -> Gpio { GPIOQ }
   fn index(&self) -> usize { 1 }
}

impl AfSsi3fss for Pq1 {
   fn af_ssi3fss(&self) -> usize { 14 }
}

impl AfEpi0s21 for Pq1 {
   fn af_epi0s21(&self) -> usize { 15 }
}

pub const PQ2: Pq2 = Pq2 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pq2 {}

impl Pin for Pq2 {
   fn port(&self) -> Gpio { GPIOQ }
   fn index(&self) -> usize { 2 }
}

impl AfSsi3xdat0 for Pq2 {
   fn af_ssi3xdat0(&self) -> usize { 14 }
}

impl AfEpi0s22 for Pq2 {
   fn af_epi0s22(&self) -> usize { 15 }
}

pub const PQ3: Pq3 = Pq3 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pq3 {}

impl Pin for Pq3 {
   fn port(&self) -> Gpio { GPIOQ }
   fn index(&self) -> usize { 3 }
}

impl AfSsi3xdat1 for Pq3 {
   fn af_ssi3xdat1(&self) -> usize { 14 }
}

impl AfEpi0s23 for Pq3 {
   fn af_epi0s23(&self) -> usize { 15 }
}

pub const PQ4: Pq4 = Pq4 {}; 

#[derive(Clone, Copy, PartialEq)]
pub struct Pq4 {}

impl Pin for Pq4 {
   fn port(&self) -> Gpio { GPIOQ }
   fn index(&self) -> usize { 4 }
}

impl AfU1rx for Pq4 {
   fn af_u1rx(&self) -> usize { 1 }
}

impl AfDivsclk for Pq4 {
   fn af_divsclk(&self) -> usize { 7 }
}

