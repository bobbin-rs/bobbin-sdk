pub const DMA1: Dma = Dma(0x40026000);
pub const DMA2: Dma = Dma(0x40026400);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dma(pub u32);

impl Dma {
  pub unsafe fn lisr(&self) -> Lisr { 
     Lisr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }

  pub unsafe fn hisr(&self) -> Hisr { 
     Hisr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }

  pub unsafe fn lifcr(&self) -> Lifcr { 
     Lifcr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_lifcr(&mut self, value: Lifcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_lifcr<F: FnOnce(Lifcr) -> Lifcr>(&mut self, f: F) {
     let tmp = self.lifcr();
     self.set_lifcr(f(tmp))
  }

  pub unsafe fn hifcr(&self) -> Hifcr { 
     Hifcr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_hifcr(&mut self, value: Hifcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_hifcr<F: FnOnce(Hifcr) -> Hifcr>(&mut self, f: F) {
     let tmp = self.hifcr();
     self.set_hifcr(f(tmp))
  }

  pub unsafe fn scr(&self, index: usize) -> Scr { 
     assert!(index < 8);
     Scr(::core::ptr::read_volatile(((self.0 as usize) + 0x10 + (index * 24)) as *const u32))
  }
  pub unsafe fn set_scr(&mut self, index: usize, value: Scr) {
     assert!(index < 8);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10 + (index * 24)) as *mut u32, value.0);
  }
  pub unsafe fn with_scr<F: FnOnce(Scr) -> Scr>(&mut self, index: usize, f: F) {
     let tmp = self.scr(index);
     self.set_scr(index, f(tmp))
  }

  pub unsafe fn sndtr(&self, index: usize) -> Sndtr { 
     assert!(index < 8);
     Sndtr(::core::ptr::read_volatile(((self.0 as usize) + 0x14 + (index * 24)) as *const u32))
  }
  pub unsafe fn set_sndtr(&mut self, index: usize, value: Sndtr) {
     assert!(index < 8);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14 + (index * 24)) as *mut u32, value.0);
  }
  pub unsafe fn with_sndtr<F: FnOnce(Sndtr) -> Sndtr>(&mut self, index: usize, f: F) {
     let tmp = self.sndtr(index);
     self.set_sndtr(index, f(tmp))
  }

  pub unsafe fn spar(&self, index: usize) -> Spar { 
     assert!(index < 8);
     Spar(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index * 24)) as *const u32))
  }
  pub unsafe fn set_spar(&mut self, index: usize, value: Spar) {
     assert!(index < 8);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index * 24)) as *mut u32, value.0);
  }
  pub unsafe fn with_spar<F: FnOnce(Spar) -> Spar>(&mut self, index: usize, f: F) {
     let tmp = self.spar(index);
     self.set_spar(index, f(tmp))
  }

  pub unsafe fn sm0ar(&self, index: usize) -> Sm0ar { 
     assert!(index < 8);
     Sm0ar(::core::ptr::read_volatile(((self.0 as usize) + 0x1c + (index * 24)) as *const u32))
  }
  pub unsafe fn set_sm0ar(&mut self, index: usize, value: Sm0ar) {
     assert!(index < 8);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c + (index * 24)) as *mut u32, value.0);
  }
  pub unsafe fn with_sm0ar<F: FnOnce(Sm0ar) -> Sm0ar>(&mut self, index: usize, f: F) {
     let tmp = self.sm0ar(index);
     self.set_sm0ar(index, f(tmp))
  }

  pub unsafe fn sm1ar(&self, index: usize) -> Sm1ar { 
     assert!(index < 8);
     Sm1ar(::core::ptr::read_volatile(((self.0 as usize) + 0x20 + (index * 24)) as *const u32))
  }
  pub unsafe fn set_sm1ar(&mut self, index: usize, value: Sm1ar) {
     assert!(index < 8);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20 + (index * 24)) as *mut u32, value.0);
  }
  pub unsafe fn with_sm1ar<F: FnOnce(Sm1ar) -> Sm1ar>(&mut self, index: usize, f: F) {
     let tmp = self.sm1ar(index);
     self.set_sm1ar(index, f(tmp))
  }

  pub unsafe fn sfcr(&self, index: usize) -> Sfcr { 
     assert!(index < 8);
     Sfcr(::core::ptr::read_volatile(((self.0 as usize) + 0x24 + (index * 24)) as *const u32))
  }
  pub unsafe fn set_sfcr(&mut self, index: usize, value: Sfcr) {
     assert!(index < 8);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24 + (index * 24)) as *mut u32, value.0);
  }
  pub unsafe fn with_sfcr<F: FnOnce(Sfcr) -> Sfcr>(&mut self, index: usize, f: F) {
     let tmp = self.sfcr(index);
     self.set_sfcr(index, f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Lisr(pub u32);

impl Lisr {
  pub fn tcif3(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_tcif3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn htif3(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_htif3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn teif3(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_teif3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn dmeif3(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_dmeif3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn feif3(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_feif3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn tcif2(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_tcif2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn htif2(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_htif2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn teif2(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_teif2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn dmeif2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_dmeif2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn feif2(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_feif2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn tcif1(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_tcif1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn htif1(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_htif1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn teif1(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_teif1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn dmeif1(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dmeif1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn feif1(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_feif1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn tcif0(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_tcif0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn htif0(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_htif0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn teif0(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_teif0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dmeif0(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_dmeif0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn feif0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_feif0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Lisr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Lisr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tcif3() != 0 { try!(write!(f, " tcif3"))}
      if self.htif3() != 0 { try!(write!(f, " htif3"))}
      if self.teif3() != 0 { try!(write!(f, " teif3"))}
      if self.dmeif3() != 0 { try!(write!(f, " dmeif3"))}
      if self.feif3() != 0 { try!(write!(f, " feif3"))}
      if self.tcif2() != 0 { try!(write!(f, " tcif2"))}
      if self.htif2() != 0 { try!(write!(f, " htif2"))}
      if self.teif2() != 0 { try!(write!(f, " teif2"))}
      if self.dmeif2() != 0 { try!(write!(f, " dmeif2"))}
      if self.feif2() != 0 { try!(write!(f, " feif2"))}
      if self.tcif1() != 0 { try!(write!(f, " tcif1"))}
      if self.htif1() != 0 { try!(write!(f, " htif1"))}
      if self.teif1() != 0 { try!(write!(f, " teif1"))}
      if self.dmeif1() != 0 { try!(write!(f, " dmeif1"))}
      if self.feif1() != 0 { try!(write!(f, " feif1"))}
      if self.tcif0() != 0 { try!(write!(f, " tcif0"))}
      if self.htif0() != 0 { try!(write!(f, " htif0"))}
      if self.teif0() != 0 { try!(write!(f, " teif0"))}
      if self.dmeif0() != 0 { try!(write!(f, " dmeif0"))}
      if self.feif0() != 0 { try!(write!(f, " feif0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hisr(pub u32);

impl Hisr {
  pub fn tcif7(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_tcif7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn htif7(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_htif7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn teif7(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_teif7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn dmeif7(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_dmeif7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn feif7(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_feif7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn tcif6(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_tcif6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn htif6(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_htif6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn teif6(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_teif6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn dmeif6(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_dmeif6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn feif6(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_feif6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn tcif5(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_tcif5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn htif5(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_htif5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn teif5(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_teif5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn dmeif5(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_dmeif5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn feif5(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_feif5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn tcif4(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_tcif4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn htif4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_htif4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn teif4(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_teif4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dmeif4(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_dmeif4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn feif4(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_feif4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Hisr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hisr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tcif7() != 0 { try!(write!(f, " tcif7"))}
      if self.htif7() != 0 { try!(write!(f, " htif7"))}
      if self.teif7() != 0 { try!(write!(f, " teif7"))}
      if self.dmeif7() != 0 { try!(write!(f, " dmeif7"))}
      if self.feif7() != 0 { try!(write!(f, " feif7"))}
      if self.tcif6() != 0 { try!(write!(f, " tcif6"))}
      if self.htif6() != 0 { try!(write!(f, " htif6"))}
      if self.teif6() != 0 { try!(write!(f, " teif6"))}
      if self.dmeif6() != 0 { try!(write!(f, " dmeif6"))}
      if self.feif6() != 0 { try!(write!(f, " feif6"))}
      if self.tcif5() != 0 { try!(write!(f, " tcif5"))}
      if self.htif5() != 0 { try!(write!(f, " htif5"))}
      if self.teif5() != 0 { try!(write!(f, " teif5"))}
      if self.dmeif5() != 0 { try!(write!(f, " dmeif5"))}
      if self.feif5() != 0 { try!(write!(f, " feif5"))}
      if self.tcif4() != 0 { try!(write!(f, " tcif4"))}
      if self.htif4() != 0 { try!(write!(f, " htif4"))}
      if self.teif4() != 0 { try!(write!(f, " teif4"))}
      if self.dmeif4() != 0 { try!(write!(f, " dmeif4"))}
      if self.feif4() != 0 { try!(write!(f, " feif4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Lifcr(pub u32);

impl Lifcr {
  pub fn ctcif3(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_ctcif3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn chtif3(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_chtif3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn cteif3(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_cteif3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn cdmeif3(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_cdmeif3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cfeif3(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_cfeif3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn ctcif2(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_ctcif2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn chtif2(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_chtif2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn cteif2(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_cteif2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn cdmeif2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_cdmeif2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn cfeif2(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_cfeif2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ctcif1(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_ctcif1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn chtif1(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_chtif1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn cteif1(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_cteif1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn cdmeif1(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_cdmeif1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn cfeif1(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_cfeif1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ctcif0(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_ctcif0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn chtif0(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_chtif0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cteif0(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_cteif0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn cdmeif0(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_cdmeif0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cfeif0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_cfeif0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Lifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Lifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ctcif3() != 0 { try!(write!(f, " ctcif3"))}
      if self.chtif3() != 0 { try!(write!(f, " chtif3"))}
      if self.cteif3() != 0 { try!(write!(f, " cteif3"))}
      if self.cdmeif3() != 0 { try!(write!(f, " cdmeif3"))}
      if self.cfeif3() != 0 { try!(write!(f, " cfeif3"))}
      if self.ctcif2() != 0 { try!(write!(f, " ctcif2"))}
      if self.chtif2() != 0 { try!(write!(f, " chtif2"))}
      if self.cteif2() != 0 { try!(write!(f, " cteif2"))}
      if self.cdmeif2() != 0 { try!(write!(f, " cdmeif2"))}
      if self.cfeif2() != 0 { try!(write!(f, " cfeif2"))}
      if self.ctcif1() != 0 { try!(write!(f, " ctcif1"))}
      if self.chtif1() != 0 { try!(write!(f, " chtif1"))}
      if self.cteif1() != 0 { try!(write!(f, " cteif1"))}
      if self.cdmeif1() != 0 { try!(write!(f, " cdmeif1"))}
      if self.cfeif1() != 0 { try!(write!(f, " cfeif1"))}
      if self.ctcif0() != 0 { try!(write!(f, " ctcif0"))}
      if self.chtif0() != 0 { try!(write!(f, " chtif0"))}
      if self.cteif0() != 0 { try!(write!(f, " cteif0"))}
      if self.cdmeif0() != 0 { try!(write!(f, " cdmeif0"))}
      if self.cfeif0() != 0 { try!(write!(f, " cfeif0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hifcr(pub u32);

impl Hifcr {
  pub fn ctcif7(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_ctcif7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn chtif7(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_chtif7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn cteif7(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_cteif7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn cdmeif7(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_cdmeif7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cfeif7(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_cfeif7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn ctcif6(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_ctcif6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn chtif6(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_chtif6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn cteif6(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_cteif6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn cdmeif6(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_cdmeif6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn cfeif6(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_cfeif6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ctcif5(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_ctcif5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn chtif5(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_chtif5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn cteif5(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_cteif5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn cdmeif5(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_cdmeif5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn cfeif5(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_cfeif5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ctcif4(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_ctcif4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn chtif4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_chtif4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cteif4(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_cteif4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn cdmeif4(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_cdmeif4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cfeif4(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_cfeif4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Hifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ctcif7() != 0 { try!(write!(f, " ctcif7"))}
      if self.chtif7() != 0 { try!(write!(f, " chtif7"))}
      if self.cteif7() != 0 { try!(write!(f, " cteif7"))}
      if self.cdmeif7() != 0 { try!(write!(f, " cdmeif7"))}
      if self.cfeif7() != 0 { try!(write!(f, " cfeif7"))}
      if self.ctcif6() != 0 { try!(write!(f, " ctcif6"))}
      if self.chtif6() != 0 { try!(write!(f, " chtif6"))}
      if self.cteif6() != 0 { try!(write!(f, " cteif6"))}
      if self.cdmeif6() != 0 { try!(write!(f, " cdmeif6"))}
      if self.cfeif6() != 0 { try!(write!(f, " cfeif6"))}
      if self.ctcif5() != 0 { try!(write!(f, " ctcif5"))}
      if self.chtif5() != 0 { try!(write!(f, " chtif5"))}
      if self.cteif5() != 0 { try!(write!(f, " cteif5"))}
      if self.cdmeif5() != 0 { try!(write!(f, " cdmeif5"))}
      if self.cfeif5() != 0 { try!(write!(f, " cfeif5"))}
      if self.ctcif4() != 0 { try!(write!(f, " ctcif4"))}
      if self.chtif4() != 0 { try!(write!(f, " chtif4"))}
      if self.cteif4() != 0 { try!(write!(f, " cteif4"))}
      if self.cdmeif4() != 0 { try!(write!(f, " cdmeif4"))}
      if self.cfeif4() != 0 { try!(write!(f, " cfeif4"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Scr(pub u32);

impl Scr {
  pub fn chsel(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x7 // [27:25]
  }
  pub fn set_chsel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn mburst(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x3 // [24:23]
  }
  pub fn set_mburst(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn pburst(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x3 // [22:21]
  }
  pub fn set_pburst(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn ct(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_ct(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn dbm(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_dbm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn pl(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_pl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn pincos(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_pincos(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn msize(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x3 // [14:13]
  }
  pub fn set_msize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn psize(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x3 // [12:11]
  }
  pub fn set_psize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn minc(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_minc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn pinc(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_pinc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn circ(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_circ(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dir(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  pub fn set_dir(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn pfctrl(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_pfctrl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn tcie(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_tcie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn htie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_htie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn teie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_teie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn dmeie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_dmeie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
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
      if self.chsel() != 0 { try!(write!(f, " chsel=0x{:x}", self.chsel()))}
      if self.mburst() != 0 { try!(write!(f, " mburst=0x{:x}", self.mburst()))}
      if self.pburst() != 0 { try!(write!(f, " pburst=0x{:x}", self.pburst()))}
      if self.ct() != 0 { try!(write!(f, " ct"))}
      if self.dbm() != 0 { try!(write!(f, " dbm"))}
      if self.pl() != 0 { try!(write!(f, " pl=0x{:x}", self.pl()))}
      if self.pincos() != 0 { try!(write!(f, " pincos"))}
      if self.msize() != 0 { try!(write!(f, " msize=0x{:x}", self.msize()))}
      if self.psize() != 0 { try!(write!(f, " psize=0x{:x}", self.psize()))}
      if self.minc() != 0 { try!(write!(f, " minc"))}
      if self.pinc() != 0 { try!(write!(f, " pinc"))}
      if self.circ() != 0 { try!(write!(f, " circ"))}
      if self.dir() != 0 { try!(write!(f, " dir=0x{:x}", self.dir()))}
      if self.pfctrl() != 0 { try!(write!(f, " pfctrl"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.htie() != 0 { try!(write!(f, " htie"))}
      if self.teie() != 0 { try!(write!(f, " teie"))}
      if self.dmeie() != 0 { try!(write!(f, " dmeie"))}
      if self.en() != 0 { try!(write!(f, " en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sndtr(pub u32);

impl Sndtr {
  pub fn ndt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ndt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sndtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sndtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ndt() != 0 { try!(write!(f, " ndt=0x{:x}", self.ndt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Spar(pub u32);

impl Spar {
  pub fn pa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_pa(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Spar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Spar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sm0ar(pub u32);

impl Sm0ar {
  pub fn m0a(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_m0a(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sm0ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sm0ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sm1ar(pub u32);

impl Sm1ar {
  pub fn m1a(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_m1a(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sm1ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sm1ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sfcr(pub u32);

impl Sfcr {
  pub fn feie(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_feie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn fs(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x7 // [5:3]
  }
  pub fn set_fs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dmdis(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_dmdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn fth(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_fth(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Sfcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Sfcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.feie() != 0 { try!(write!(f, " feie"))}
      if self.fs() != 0 { try!(write!(f, " fs=0x{:x}", self.fs()))}
      if self.dmdis() != 0 { try!(write!(f, " dmdis"))}
      if self.fth() != 0 { try!(write!(f, " fth=0x{:x}", self.fth()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

