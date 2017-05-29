pub const PCC: Pcc = Pcc(0x40065000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcc(pub u32);

impl Pcc {
  pub unsafe fn pccdummy0(&self) -> Pccdummy0 { 
     Pccdummy0(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_pccdummy0(&mut self, value: Pccdummy0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy0<F: FnOnce(Pccdummy0) -> Pccdummy0>(&mut self, f: F) {
     let tmp = self.pccdummy0();
     self.set_pccdummy0(f(tmp))
  }

  pub unsafe fn pccdummy1(&self) -> Pccdummy1 { 
     Pccdummy1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_pccdummy1(&mut self, value: Pccdummy1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy1<F: FnOnce(Pccdummy1) -> Pccdummy1>(&mut self, f: F) {
     let tmp = self.pccdummy1();
     self.set_pccdummy1(f(tmp))
  }

  pub unsafe fn pccdummy2(&self) -> Pccdummy2 { 
     Pccdummy2(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_pccdummy2(&mut self, value: Pccdummy2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy2<F: FnOnce(Pccdummy2) -> Pccdummy2>(&mut self, f: F) {
     let tmp = self.pccdummy2();
     self.set_pccdummy2(f(tmp))
  }

  pub unsafe fn pccdummy3(&self) -> Pccdummy3 { 
     Pccdummy3(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_pccdummy3(&mut self, value: Pccdummy3) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy3<F: FnOnce(Pccdummy3) -> Pccdummy3>(&mut self, f: F) {
     let tmp = self.pccdummy3();
     self.set_pccdummy3(f(tmp))
  }

  pub unsafe fn pccdummy4(&self) -> Pccdummy4 { 
     Pccdummy4(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_pccdummy4(&mut self, value: Pccdummy4) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy4<F: FnOnce(Pccdummy4) -> Pccdummy4>(&mut self, f: F) {
     let tmp = self.pccdummy4();
     self.set_pccdummy4(f(tmp))
  }

  pub unsafe fn pccdummy5(&self) -> Pccdummy5 { 
     Pccdummy5(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_pccdummy5(&mut self, value: Pccdummy5) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy5<F: FnOnce(Pccdummy5) -> Pccdummy5>(&mut self, f: F) {
     let tmp = self.pccdummy5();
     self.set_pccdummy5(f(tmp))
  }

  pub unsafe fn pccdummy6(&self) -> Pccdummy6 { 
     Pccdummy6(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_pccdummy6(&mut self, value: Pccdummy6) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy6<F: FnOnce(Pccdummy6) -> Pccdummy6>(&mut self, f: F) {
     let tmp = self.pccdummy6();
     self.set_pccdummy6(f(tmp))
  }

  pub unsafe fn pccdummy7(&self) -> Pccdummy7 { 
     Pccdummy7(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_pccdummy7(&mut self, value: Pccdummy7) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy7<F: FnOnce(Pccdummy7) -> Pccdummy7>(&mut self, f: F) {
     let tmp = self.pccdummy7();
     self.set_pccdummy7(f(tmp))
  }

  pub unsafe fn pccdummy8(&self) -> Pccdummy8 { 
     Pccdummy8(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
  }
  pub unsafe fn set_pccdummy8(&mut self, value: Pccdummy8) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy8<F: FnOnce(Pccdummy8) -> Pccdummy8>(&mut self, f: F) {
     let tmp = self.pccdummy8();
     self.set_pccdummy8(f(tmp))
  }

  pub unsafe fn pccdummy9(&self) -> Pccdummy9 { 
     Pccdummy9(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
  }
  pub unsafe fn set_pccdummy9(&mut self, value: Pccdummy9) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy9<F: FnOnce(Pccdummy9) -> Pccdummy9>(&mut self, f: F) {
     let tmp = self.pccdummy9();
     self.set_pccdummy9(f(tmp))
  }

  pub unsafe fn pccdummy10(&self) -> Pccdummy10 { 
     Pccdummy10(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
  }
  pub unsafe fn set_pccdummy10(&mut self, value: Pccdummy10) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy10<F: FnOnce(Pccdummy10) -> Pccdummy10>(&mut self, f: F) {
     let tmp = self.pccdummy10();
     self.set_pccdummy10(f(tmp))
  }

  pub unsafe fn pccdummy11(&self) -> Pccdummy11 { 
     Pccdummy11(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
  }
  pub unsafe fn set_pccdummy11(&mut self, value: Pccdummy11) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy11<F: FnOnce(Pccdummy11) -> Pccdummy11>(&mut self, f: F) {
     let tmp = self.pccdummy11();
     self.set_pccdummy11(f(tmp))
  }

  pub unsafe fn pccdummy12(&self) -> Pccdummy12 { 
     Pccdummy12(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
  }
  pub unsafe fn set_pccdummy12(&mut self, value: Pccdummy12) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy12<F: FnOnce(Pccdummy12) -> Pccdummy12>(&mut self, f: F) {
     let tmp = self.pccdummy12();
     self.set_pccdummy12(f(tmp))
  }

  pub unsafe fn pccdummy13(&self) -> Pccdummy13 { 
     Pccdummy13(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
  }
  pub unsafe fn set_pccdummy13(&mut self, value: Pccdummy13) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy13<F: FnOnce(Pccdummy13) -> Pccdummy13>(&mut self, f: F) {
     let tmp = self.pccdummy13();
     self.set_pccdummy13(f(tmp))
  }

  pub unsafe fn pccdummy14(&self) -> Pccdummy14 { 
     Pccdummy14(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
  }
  pub unsafe fn set_pccdummy14(&mut self, value: Pccdummy14) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy14<F: FnOnce(Pccdummy14) -> Pccdummy14>(&mut self, f: F) {
     let tmp = self.pccdummy14();
     self.set_pccdummy14(f(tmp))
  }

  pub unsafe fn pccdummy15(&self) -> Pccdummy15 { 
     Pccdummy15(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
  }
  pub unsafe fn set_pccdummy15(&mut self, value: Pccdummy15) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy15<F: FnOnce(Pccdummy15) -> Pccdummy15>(&mut self, f: F) {
     let tmp = self.pccdummy15();
     self.set_pccdummy15(f(tmp))
  }

  pub unsafe fn pccdummy16(&self) -> Pccdummy16 { 
     Pccdummy16(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
  }
  pub unsafe fn set_pccdummy16(&mut self, value: Pccdummy16) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy16<F: FnOnce(Pccdummy16) -> Pccdummy16>(&mut self, f: F) {
     let tmp = self.pccdummy16();
     self.set_pccdummy16(f(tmp))
  }

  pub unsafe fn pccdummy17(&self) -> Pccdummy17 { 
     Pccdummy17(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
  }
  pub unsafe fn set_pccdummy17(&mut self, value: Pccdummy17) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy17<F: FnOnce(Pccdummy17) -> Pccdummy17>(&mut self, f: F) {
     let tmp = self.pccdummy17();
     self.set_pccdummy17(f(tmp))
  }

  pub unsafe fn pccdummy18(&self) -> Pccdummy18 { 
     Pccdummy18(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
  }
  pub unsafe fn set_pccdummy18(&mut self, value: Pccdummy18) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy18<F: FnOnce(Pccdummy18) -> Pccdummy18>(&mut self, f: F) {
     let tmp = self.pccdummy18();
     self.set_pccdummy18(f(tmp))
  }

  pub unsafe fn pccdummy19(&self) -> Pccdummy19 { 
     Pccdummy19(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
  }
  pub unsafe fn set_pccdummy19(&mut self, value: Pccdummy19) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy19<F: FnOnce(Pccdummy19) -> Pccdummy19>(&mut self, f: F) {
     let tmp = self.pccdummy19();
     self.set_pccdummy19(f(tmp))
  }

  pub unsafe fn pccdummy20(&self) -> Pccdummy20 { 
     Pccdummy20(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
  }
  pub unsafe fn set_pccdummy20(&mut self, value: Pccdummy20) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy20<F: FnOnce(Pccdummy20) -> Pccdummy20>(&mut self, f: F) {
     let tmp = self.pccdummy20();
     self.set_pccdummy20(f(tmp))
  }

  pub unsafe fn pccdummy21(&self) -> Pccdummy21 { 
     Pccdummy21(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
  }
  pub unsafe fn set_pccdummy21(&mut self, value: Pccdummy21) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy21<F: FnOnce(Pccdummy21) -> Pccdummy21>(&mut self, f: F) {
     let tmp = self.pccdummy21();
     self.set_pccdummy21(f(tmp))
  }

  pub unsafe fn pccdummy22(&self) -> Pccdummy22 { 
     Pccdummy22(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
  }
  pub unsafe fn set_pccdummy22(&mut self, value: Pccdummy22) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy22<F: FnOnce(Pccdummy22) -> Pccdummy22>(&mut self, f: F) {
     let tmp = self.pccdummy22();
     self.set_pccdummy22(f(tmp))
  }

  pub unsafe fn pccdummy23(&self) -> Pccdummy23 { 
     Pccdummy23(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
  }
  pub unsafe fn set_pccdummy23(&mut self, value: Pccdummy23) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy23<F: FnOnce(Pccdummy23) -> Pccdummy23>(&mut self, f: F) {
     let tmp = self.pccdummy23();
     self.set_pccdummy23(f(tmp))
  }

  pub unsafe fn pccdummy24(&self) -> Pccdummy24 { 
     Pccdummy24(::core::ptr::read_volatile(((self.0 as usize) + 0x60) as *const u32))
  }
  pub unsafe fn set_pccdummy24(&mut self, value: Pccdummy24) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x60) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy24<F: FnOnce(Pccdummy24) -> Pccdummy24>(&mut self, f: F) {
     let tmp = self.pccdummy24();
     self.set_pccdummy24(f(tmp))
  }

  pub unsafe fn pccdummy25(&self) -> Pccdummy25 { 
     Pccdummy25(::core::ptr::read_volatile(((self.0 as usize) + 0x64) as *const u32))
  }
  pub unsafe fn set_pccdummy25(&mut self, value: Pccdummy25) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x64) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy25<F: FnOnce(Pccdummy25) -> Pccdummy25>(&mut self, f: F) {
     let tmp = self.pccdummy25();
     self.set_pccdummy25(f(tmp))
  }

  pub unsafe fn pccdummy26(&self) -> Pccdummy26 { 
     Pccdummy26(::core::ptr::read_volatile(((self.0 as usize) + 0x68) as *const u32))
  }
  pub unsafe fn set_pccdummy26(&mut self, value: Pccdummy26) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x68) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy26<F: FnOnce(Pccdummy26) -> Pccdummy26>(&mut self, f: F) {
     let tmp = self.pccdummy26();
     self.set_pccdummy26(f(tmp))
  }

  pub unsafe fn pccdummy27(&self) -> Pccdummy27 { 
     Pccdummy27(::core::ptr::read_volatile(((self.0 as usize) + 0x6c) as *const u32))
  }
  pub unsafe fn set_pccdummy27(&mut self, value: Pccdummy27) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x6c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy27<F: FnOnce(Pccdummy27) -> Pccdummy27>(&mut self, f: F) {
     let tmp = self.pccdummy27();
     self.set_pccdummy27(f(tmp))
  }

  pub unsafe fn pccdummy28(&self) -> Pccdummy28 { 
     Pccdummy28(::core::ptr::read_volatile(((self.0 as usize) + 0x70) as *const u32))
  }
  pub unsafe fn set_pccdummy28(&mut self, value: Pccdummy28) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x70) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy28<F: FnOnce(Pccdummy28) -> Pccdummy28>(&mut self, f: F) {
     let tmp = self.pccdummy28();
     self.set_pccdummy28(f(tmp))
  }

  pub unsafe fn pccdummy29(&self) -> Pccdummy29 { 
     Pccdummy29(::core::ptr::read_volatile(((self.0 as usize) + 0x74) as *const u32))
  }
  pub unsafe fn set_pccdummy29(&mut self, value: Pccdummy29) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x74) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy29<F: FnOnce(Pccdummy29) -> Pccdummy29>(&mut self, f: F) {
     let tmp = self.pccdummy29();
     self.set_pccdummy29(f(tmp))
  }

  pub unsafe fn pccdummy30(&self) -> Pccdummy30 { 
     Pccdummy30(::core::ptr::read_volatile(((self.0 as usize) + 0x78) as *const u32))
  }
  pub unsafe fn set_pccdummy30(&mut self, value: Pccdummy30) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x78) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy30<F: FnOnce(Pccdummy30) -> Pccdummy30>(&mut self, f: F) {
     let tmp = self.pccdummy30();
     self.set_pccdummy30(f(tmp))
  }

  pub unsafe fn pccdummy31(&self) -> Pccdummy31 { 
     Pccdummy31(::core::ptr::read_volatile(((self.0 as usize) + 0x7c) as *const u32))
  }
  pub unsafe fn set_pccdummy31(&mut self, value: Pccdummy31) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x7c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy31<F: FnOnce(Pccdummy31) -> Pccdummy31>(&mut self, f: F) {
     let tmp = self.pccdummy31();
     self.set_pccdummy31(f(tmp))
  }

  pub unsafe fn ftfc(&self) -> Ftfc { 
     Ftfc(::core::ptr::read_volatile(((self.0 as usize) + 0x80) as *const u32))
  }
  pub unsafe fn set_ftfc(&mut self, value: Ftfc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x80) as *mut u32, value.0);
  }
  pub unsafe fn with_ftfc<F: FnOnce(Ftfc) -> Ftfc>(&mut self, f: F) {
     let tmp = self.ftfc();
     self.set_ftfc(f(tmp))
  }

  pub unsafe fn dmamux(&self) -> Dmamux { 
     Dmamux(::core::ptr::read_volatile(((self.0 as usize) + 0x84) as *const u32))
  }
  pub unsafe fn set_dmamux(&mut self, value: Dmamux) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x84) as *mut u32, value.0);
  }
  pub unsafe fn with_dmamux<F: FnOnce(Dmamux) -> Dmamux>(&mut self, f: F) {
     let tmp = self.dmamux();
     self.set_dmamux(f(tmp))
  }

  pub unsafe fn pccdummy34(&self) -> Pccdummy34 { 
     Pccdummy34(::core::ptr::read_volatile(((self.0 as usize) + 0x88) as *const u32))
  }
  pub unsafe fn set_pccdummy34(&mut self, value: Pccdummy34) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x88) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy34<F: FnOnce(Pccdummy34) -> Pccdummy34>(&mut self, f: F) {
     let tmp = self.pccdummy34();
     self.set_pccdummy34(f(tmp))
  }

  pub unsafe fn pccdummy35(&self) -> Pccdummy35 { 
     Pccdummy35(::core::ptr::read_volatile(((self.0 as usize) + 0x8c) as *const u32))
  }
  pub unsafe fn set_pccdummy35(&mut self, value: Pccdummy35) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy35<F: FnOnce(Pccdummy35) -> Pccdummy35>(&mut self, f: F) {
     let tmp = self.pccdummy35();
     self.set_pccdummy35(f(tmp))
  }

  pub unsafe fn flexcan0(&self) -> Flexcan0 { 
     Flexcan0(::core::ptr::read_volatile(((self.0 as usize) + 0x90) as *const u32))
  }
  pub unsafe fn set_flexcan0(&mut self, value: Flexcan0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x90) as *mut u32, value.0);
  }
  pub unsafe fn with_flexcan0<F: FnOnce(Flexcan0) -> Flexcan0>(&mut self, f: F) {
     let tmp = self.flexcan0();
     self.set_flexcan0(f(tmp))
  }

  pub unsafe fn flexcan1(&self) -> Flexcan1 { 
     Flexcan1(::core::ptr::read_volatile(((self.0 as usize) + 0x94) as *const u32))
  }
  pub unsafe fn set_flexcan1(&mut self, value: Flexcan1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x94) as *mut u32, value.0);
  }
  pub unsafe fn with_flexcan1<F: FnOnce(Flexcan1) -> Flexcan1>(&mut self, f: F) {
     let tmp = self.flexcan1();
     self.set_flexcan1(f(tmp))
  }

  pub unsafe fn ftm3(&self) -> Ftm3 { 
     Ftm3(::core::ptr::read_volatile(((self.0 as usize) + 0x98) as *const u32))
  }
  pub unsafe fn set_ftm3(&mut self, value: Ftm3) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x98) as *mut u32, value.0);
  }
  pub unsafe fn with_ftm3<F: FnOnce(Ftm3) -> Ftm3>(&mut self, f: F) {
     let tmp = self.ftm3();
     self.set_ftm3(f(tmp))
  }

  pub unsafe fn adc1(&self) -> Adc1 { 
     Adc1(::core::ptr::read_volatile(((self.0 as usize) + 0x9c) as *const u32))
  }
  pub unsafe fn set_adc1(&mut self, value: Adc1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x9c) as *mut u32, value.0);
  }
  pub unsafe fn with_adc1<F: FnOnce(Adc1) -> Adc1>(&mut self, f: F) {
     let tmp = self.adc1();
     self.set_adc1(f(tmp))
  }

  pub unsafe fn pccdummy40(&self) -> Pccdummy40 { 
     Pccdummy40(::core::ptr::read_volatile(((self.0 as usize) + 0xa0) as *const u32))
  }
  pub unsafe fn set_pccdummy40(&mut self, value: Pccdummy40) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa0) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy40<F: FnOnce(Pccdummy40) -> Pccdummy40>(&mut self, f: F) {
     let tmp = self.pccdummy40();
     self.set_pccdummy40(f(tmp))
  }

  pub unsafe fn pccdummy41(&self) -> Pccdummy41 { 
     Pccdummy41(::core::ptr::read_volatile(((self.0 as usize) + 0xa4) as *const u32))
  }
  pub unsafe fn set_pccdummy41(&mut self, value: Pccdummy41) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa4) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy41<F: FnOnce(Pccdummy41) -> Pccdummy41>(&mut self, f: F) {
     let tmp = self.pccdummy41();
     self.set_pccdummy41(f(tmp))
  }

  pub unsafe fn pccdummy42(&self) -> Pccdummy42 { 
     Pccdummy42(::core::ptr::read_volatile(((self.0 as usize) + 0xa8) as *const u32))
  }
  pub unsafe fn set_pccdummy42(&mut self, value: Pccdummy42) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xa8) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy42<F: FnOnce(Pccdummy42) -> Pccdummy42>(&mut self, f: F) {
     let tmp = self.pccdummy42();
     self.set_pccdummy42(f(tmp))
  }

  pub unsafe fn flexcan2(&self) -> Flexcan2 { 
     Flexcan2(::core::ptr::read_volatile(((self.0 as usize) + 0xac) as *const u32))
  }
  pub unsafe fn set_flexcan2(&mut self, value: Flexcan2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xac) as *mut u32, value.0);
  }
  pub unsafe fn with_flexcan2<F: FnOnce(Flexcan2) -> Flexcan2>(&mut self, f: F) {
     let tmp = self.flexcan2();
     self.set_flexcan2(f(tmp))
  }

  pub unsafe fn lpspi0(&self) -> Lpspi0 { 
     Lpspi0(::core::ptr::read_volatile(((self.0 as usize) + 0xb0) as *const u32))
  }
  pub unsafe fn set_lpspi0(&mut self, value: Lpspi0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb0) as *mut u32, value.0);
  }
  pub unsafe fn with_lpspi0<F: FnOnce(Lpspi0) -> Lpspi0>(&mut self, f: F) {
     let tmp = self.lpspi0();
     self.set_lpspi0(f(tmp))
  }

  pub unsafe fn lpspi1(&self) -> Lpspi1 { 
     Lpspi1(::core::ptr::read_volatile(((self.0 as usize) + 0xb4) as *const u32))
  }
  pub unsafe fn set_lpspi1(&mut self, value: Lpspi1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb4) as *mut u32, value.0);
  }
  pub unsafe fn with_lpspi1<F: FnOnce(Lpspi1) -> Lpspi1>(&mut self, f: F) {
     let tmp = self.lpspi1();
     self.set_lpspi1(f(tmp))
  }

  pub unsafe fn lpspi2(&self) -> Lpspi2 { 
     Lpspi2(::core::ptr::read_volatile(((self.0 as usize) + 0xb8) as *const u32))
  }
  pub unsafe fn set_lpspi2(&mut self, value: Lpspi2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xb8) as *mut u32, value.0);
  }
  pub unsafe fn with_lpspi2<F: FnOnce(Lpspi2) -> Lpspi2>(&mut self, f: F) {
     let tmp = self.lpspi2();
     self.set_lpspi2(f(tmp))
  }

  pub unsafe fn pccdummy47(&self) -> Pccdummy47 { 
     Pccdummy47(::core::ptr::read_volatile(((self.0 as usize) + 0xbc) as *const u32))
  }
  pub unsafe fn set_pccdummy47(&mut self, value: Pccdummy47) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xbc) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy47<F: FnOnce(Pccdummy47) -> Pccdummy47>(&mut self, f: F) {
     let tmp = self.pccdummy47();
     self.set_pccdummy47(f(tmp))
  }

  pub unsafe fn pccdummy48(&self) -> Pccdummy48 { 
     Pccdummy48(::core::ptr::read_volatile(((self.0 as usize) + 0xc0) as *const u32))
  }
  pub unsafe fn set_pccdummy48(&mut self, value: Pccdummy48) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc0) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy48<F: FnOnce(Pccdummy48) -> Pccdummy48>(&mut self, f: F) {
     let tmp = self.pccdummy48();
     self.set_pccdummy48(f(tmp))
  }

  pub unsafe fn pdb1(&self) -> Pdb1 { 
     Pdb1(::core::ptr::read_volatile(((self.0 as usize) + 0xc4) as *const u32))
  }
  pub unsafe fn set_pdb1(&mut self, value: Pdb1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc4) as *mut u32, value.0);
  }
  pub unsafe fn with_pdb1<F: FnOnce(Pdb1) -> Pdb1>(&mut self, f: F) {
     let tmp = self.pdb1();
     self.set_pdb1(f(tmp))
  }

  pub unsafe fn crc(&self) -> Crc { 
     Crc(::core::ptr::read_volatile(((self.0 as usize) + 0xc8) as *const u32))
  }
  pub unsafe fn set_crc(&mut self, value: Crc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc8) as *mut u32, value.0);
  }
  pub unsafe fn with_crc<F: FnOnce(Crc) -> Crc>(&mut self, f: F) {
     let tmp = self.crc();
     self.set_crc(f(tmp))
  }

  pub unsafe fn pccdummy51(&self) -> Pccdummy51 { 
     Pccdummy51(::core::ptr::read_volatile(((self.0 as usize) + 0xcc) as *const u32))
  }
  pub unsafe fn set_pccdummy51(&mut self, value: Pccdummy51) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xcc) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy51<F: FnOnce(Pccdummy51) -> Pccdummy51>(&mut self, f: F) {
     let tmp = self.pccdummy51();
     self.set_pccdummy51(f(tmp))
  }

  pub unsafe fn pccdummy52(&self) -> Pccdummy52 { 
     Pccdummy52(::core::ptr::read_volatile(((self.0 as usize) + 0xd0) as *const u32))
  }
  pub unsafe fn set_pccdummy52(&mut self, value: Pccdummy52) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd0) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy52<F: FnOnce(Pccdummy52) -> Pccdummy52>(&mut self, f: F) {
     let tmp = self.pccdummy52();
     self.set_pccdummy52(f(tmp))
  }

  pub unsafe fn pccdummy53(&self) -> Pccdummy53 { 
     Pccdummy53(::core::ptr::read_volatile(((self.0 as usize) + 0xd4) as *const u32))
  }
  pub unsafe fn set_pccdummy53(&mut self, value: Pccdummy53) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd4) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy53<F: FnOnce(Pccdummy53) -> Pccdummy53>(&mut self, f: F) {
     let tmp = self.pccdummy53();
     self.set_pccdummy53(f(tmp))
  }

  pub unsafe fn pdb0(&self) -> Pdb0 { 
     Pdb0(::core::ptr::read_volatile(((self.0 as usize) + 0xd8) as *const u32))
  }
  pub unsafe fn set_pdb0(&mut self, value: Pdb0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xd8) as *mut u32, value.0);
  }
  pub unsafe fn with_pdb0<F: FnOnce(Pdb0) -> Pdb0>(&mut self, f: F) {
     let tmp = self.pdb0();
     self.set_pdb0(f(tmp))
  }

  pub unsafe fn lpit(&self) -> Lpit { 
     Lpit(::core::ptr::read_volatile(((self.0 as usize) + 0xdc) as *const u32))
  }
  pub unsafe fn set_lpit(&mut self, value: Lpit) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xdc) as *mut u32, value.0);
  }
  pub unsafe fn with_lpit<F: FnOnce(Lpit) -> Lpit>(&mut self, f: F) {
     let tmp = self.lpit();
     self.set_lpit(f(tmp))
  }

  pub unsafe fn ftm0(&self) -> Ftm0 { 
     Ftm0(::core::ptr::read_volatile(((self.0 as usize) + 0xe0) as *const u32))
  }
  pub unsafe fn set_ftm0(&mut self, value: Ftm0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xe0) as *mut u32, value.0);
  }
  pub unsafe fn with_ftm0<F: FnOnce(Ftm0) -> Ftm0>(&mut self, f: F) {
     let tmp = self.ftm0();
     self.set_ftm0(f(tmp))
  }

  pub unsafe fn ftm1(&self) -> Ftm1 { 
     Ftm1(::core::ptr::read_volatile(((self.0 as usize) + 0xe4) as *const u32))
  }
  pub unsafe fn set_ftm1(&mut self, value: Ftm1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xe4) as *mut u32, value.0);
  }
  pub unsafe fn with_ftm1<F: FnOnce(Ftm1) -> Ftm1>(&mut self, f: F) {
     let tmp = self.ftm1();
     self.set_ftm1(f(tmp))
  }

  pub unsafe fn ftm2(&self) -> Ftm2 { 
     Ftm2(::core::ptr::read_volatile(((self.0 as usize) + 0xe8) as *const u32))
  }
  pub unsafe fn set_ftm2(&mut self, value: Ftm2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xe8) as *mut u32, value.0);
  }
  pub unsafe fn with_ftm2<F: FnOnce(Ftm2) -> Ftm2>(&mut self, f: F) {
     let tmp = self.ftm2();
     self.set_ftm2(f(tmp))
  }

  pub unsafe fn adc0(&self) -> Adc0 { 
     Adc0(::core::ptr::read_volatile(((self.0 as usize) + 0xec) as *const u32))
  }
  pub unsafe fn set_adc0(&mut self, value: Adc0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xec) as *mut u32, value.0);
  }
  pub unsafe fn with_adc0<F: FnOnce(Adc0) -> Adc0>(&mut self, f: F) {
     let tmp = self.adc0();
     self.set_adc0(f(tmp))
  }

  pub unsafe fn pccdummy60(&self) -> Pccdummy60 { 
     Pccdummy60(::core::ptr::read_volatile(((self.0 as usize) + 0xf0) as *const u32))
  }
  pub unsafe fn set_pccdummy60(&mut self, value: Pccdummy60) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xf0) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy60<F: FnOnce(Pccdummy60) -> Pccdummy60>(&mut self, f: F) {
     let tmp = self.pccdummy60();
     self.set_pccdummy60(f(tmp))
  }

  pub unsafe fn rtc(&self) -> Rtc { 
     Rtc(::core::ptr::read_volatile(((self.0 as usize) + 0xf4) as *const u32))
  }
  pub unsafe fn set_rtc(&mut self, value: Rtc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xf4) as *mut u32, value.0);
  }
  pub unsafe fn with_rtc<F: FnOnce(Rtc) -> Rtc>(&mut self, f: F) {
     let tmp = self.rtc();
     self.set_rtc(f(tmp))
  }

  pub unsafe fn pccdummy62(&self) -> Pccdummy62 { 
     Pccdummy62(::core::ptr::read_volatile(((self.0 as usize) + 0xf8) as *const u32))
  }
  pub unsafe fn set_pccdummy62(&mut self, value: Pccdummy62) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xf8) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy62<F: FnOnce(Pccdummy62) -> Pccdummy62>(&mut self, f: F) {
     let tmp = self.pccdummy62();
     self.set_pccdummy62(f(tmp))
  }

  pub unsafe fn pccdummy63(&self) -> Pccdummy63 { 
     Pccdummy63(::core::ptr::read_volatile(((self.0 as usize) + 0xfc) as *const u32))
  }
  pub unsafe fn set_pccdummy63(&mut self, value: Pccdummy63) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xfc) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy63<F: FnOnce(Pccdummy63) -> Pccdummy63>(&mut self, f: F) {
     let tmp = self.pccdummy63();
     self.set_pccdummy63(f(tmp))
  }

  pub unsafe fn lptmr0(&self) -> Lptmr0 { 
     Lptmr0(::core::ptr::read_volatile(((self.0 as usize) + 0x100) as *const u32))
  }
  pub unsafe fn set_lptmr0(&mut self, value: Lptmr0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x100) as *mut u32, value.0);
  }
  pub unsafe fn with_lptmr0<F: FnOnce(Lptmr0) -> Lptmr0>(&mut self, f: F) {
     let tmp = self.lptmr0();
     self.set_lptmr0(f(tmp))
  }

  pub unsafe fn pccdummy65(&self) -> Pccdummy65 { 
     Pccdummy65(::core::ptr::read_volatile(((self.0 as usize) + 0x104) as *const u32))
  }
  pub unsafe fn set_pccdummy65(&mut self, value: Pccdummy65) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x104) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy65<F: FnOnce(Pccdummy65) -> Pccdummy65>(&mut self, f: F) {
     let tmp = self.pccdummy65();
     self.set_pccdummy65(f(tmp))
  }

  pub unsafe fn pccdummy66(&self) -> Pccdummy66 { 
     Pccdummy66(::core::ptr::read_volatile(((self.0 as usize) + 0x108) as *const u32))
  }
  pub unsafe fn set_pccdummy66(&mut self, value: Pccdummy66) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x108) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy66<F: FnOnce(Pccdummy66) -> Pccdummy66>(&mut self, f: F) {
     let tmp = self.pccdummy66();
     self.set_pccdummy66(f(tmp))
  }

  pub unsafe fn pccdummy67(&self) -> Pccdummy67 { 
     Pccdummy67(::core::ptr::read_volatile(((self.0 as usize) + 0x10c) as *const u32))
  }
  pub unsafe fn set_pccdummy67(&mut self, value: Pccdummy67) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy67<F: FnOnce(Pccdummy67) -> Pccdummy67>(&mut self, f: F) {
     let tmp = self.pccdummy67();
     self.set_pccdummy67(f(tmp))
  }

  pub unsafe fn pccdummy68(&self) -> Pccdummy68 { 
     Pccdummy68(::core::ptr::read_volatile(((self.0 as usize) + 0x110) as *const u32))
  }
  pub unsafe fn set_pccdummy68(&mut self, value: Pccdummy68) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x110) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy68<F: FnOnce(Pccdummy68) -> Pccdummy68>(&mut self, f: F) {
     let tmp = self.pccdummy68();
     self.set_pccdummy68(f(tmp))
  }

  pub unsafe fn pccdummy69(&self) -> Pccdummy69 { 
     Pccdummy69(::core::ptr::read_volatile(((self.0 as usize) + 0x114) as *const u32))
  }
  pub unsafe fn set_pccdummy69(&mut self, value: Pccdummy69) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x114) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy69<F: FnOnce(Pccdummy69) -> Pccdummy69>(&mut self, f: F) {
     let tmp = self.pccdummy69();
     self.set_pccdummy69(f(tmp))
  }

  pub unsafe fn pccdummy70(&self) -> Pccdummy70 { 
     Pccdummy70(::core::ptr::read_volatile(((self.0 as usize) + 0x118) as *const u32))
  }
  pub unsafe fn set_pccdummy70(&mut self, value: Pccdummy70) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x118) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy70<F: FnOnce(Pccdummy70) -> Pccdummy70>(&mut self, f: F) {
     let tmp = self.pccdummy70();
     self.set_pccdummy70(f(tmp))
  }

  pub unsafe fn pccdummy71(&self) -> Pccdummy71 { 
     Pccdummy71(::core::ptr::read_volatile(((self.0 as usize) + 0x11c) as *const u32))
  }
  pub unsafe fn set_pccdummy71(&mut self, value: Pccdummy71) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x11c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy71<F: FnOnce(Pccdummy71) -> Pccdummy71>(&mut self, f: F) {
     let tmp = self.pccdummy71();
     self.set_pccdummy71(f(tmp))
  }

  pub unsafe fn pccdummy72(&self) -> Pccdummy72 { 
     Pccdummy72(::core::ptr::read_volatile(((self.0 as usize) + 0x120) as *const u32))
  }
  pub unsafe fn set_pccdummy72(&mut self, value: Pccdummy72) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x120) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy72<F: FnOnce(Pccdummy72) -> Pccdummy72>(&mut self, f: F) {
     let tmp = self.pccdummy72();
     self.set_pccdummy72(f(tmp))
  }

  pub unsafe fn porta(&self) -> Porta { 
     Porta(::core::ptr::read_volatile(((self.0 as usize) + 0x124) as *const u32))
  }
  pub unsafe fn set_porta(&mut self, value: Porta) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x124) as *mut u32, value.0);
  }
  pub unsafe fn with_porta<F: FnOnce(Porta) -> Porta>(&mut self, f: F) {
     let tmp = self.porta();
     self.set_porta(f(tmp))
  }

  pub unsafe fn portb(&self) -> Portb { 
     Portb(::core::ptr::read_volatile(((self.0 as usize) + 0x128) as *const u32))
  }
  pub unsafe fn set_portb(&mut self, value: Portb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x128) as *mut u32, value.0);
  }
  pub unsafe fn with_portb<F: FnOnce(Portb) -> Portb>(&mut self, f: F) {
     let tmp = self.portb();
     self.set_portb(f(tmp))
  }

  pub unsafe fn portc(&self) -> Portc { 
     Portc(::core::ptr::read_volatile(((self.0 as usize) + 0x12c) as *const u32))
  }
  pub unsafe fn set_portc(&mut self, value: Portc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x12c) as *mut u32, value.0);
  }
  pub unsafe fn with_portc<F: FnOnce(Portc) -> Portc>(&mut self, f: F) {
     let tmp = self.portc();
     self.set_portc(f(tmp))
  }

  pub unsafe fn portd(&self) -> Portd { 
     Portd(::core::ptr::read_volatile(((self.0 as usize) + 0x130) as *const u32))
  }
  pub unsafe fn set_portd(&mut self, value: Portd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x130) as *mut u32, value.0);
  }
  pub unsafe fn with_portd<F: FnOnce(Portd) -> Portd>(&mut self, f: F) {
     let tmp = self.portd();
     self.set_portd(f(tmp))
  }

  pub unsafe fn porte(&self) -> Porte { 
     Porte(::core::ptr::read_volatile(((self.0 as usize) + 0x134) as *const u32))
  }
  pub unsafe fn set_porte(&mut self, value: Porte) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x134) as *mut u32, value.0);
  }
  pub unsafe fn with_porte<F: FnOnce(Porte) -> Porte>(&mut self, f: F) {
     let tmp = self.porte();
     self.set_porte(f(tmp))
  }

  pub unsafe fn pccdummy78(&self) -> Pccdummy78 { 
     Pccdummy78(::core::ptr::read_volatile(((self.0 as usize) + 0x138) as *const u32))
  }
  pub unsafe fn set_pccdummy78(&mut self, value: Pccdummy78) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x138) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy78<F: FnOnce(Pccdummy78) -> Pccdummy78>(&mut self, f: F) {
     let tmp = self.pccdummy78();
     self.set_pccdummy78(f(tmp))
  }

  pub unsafe fn pccdummy79(&self) -> Pccdummy79 { 
     Pccdummy79(::core::ptr::read_volatile(((self.0 as usize) + 0x13c) as *const u32))
  }
  pub unsafe fn set_pccdummy79(&mut self, value: Pccdummy79) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x13c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy79<F: FnOnce(Pccdummy79) -> Pccdummy79>(&mut self, f: F) {
     let tmp = self.pccdummy79();
     self.set_pccdummy79(f(tmp))
  }

  pub unsafe fn pccdummy80(&self) -> Pccdummy80 { 
     Pccdummy80(::core::ptr::read_volatile(((self.0 as usize) + 0x140) as *const u32))
  }
  pub unsafe fn set_pccdummy80(&mut self, value: Pccdummy80) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x140) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy80<F: FnOnce(Pccdummy80) -> Pccdummy80>(&mut self, f: F) {
     let tmp = self.pccdummy80();
     self.set_pccdummy80(f(tmp))
  }

  pub unsafe fn pccdummy81(&self) -> Pccdummy81 { 
     Pccdummy81(::core::ptr::read_volatile(((self.0 as usize) + 0x144) as *const u32))
  }
  pub unsafe fn set_pccdummy81(&mut self, value: Pccdummy81) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x144) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy81<F: FnOnce(Pccdummy81) -> Pccdummy81>(&mut self, f: F) {
     let tmp = self.pccdummy81();
     self.set_pccdummy81(f(tmp))
  }

  pub unsafe fn pccdummy82(&self) -> Pccdummy82 { 
     Pccdummy82(::core::ptr::read_volatile(((self.0 as usize) + 0x148) as *const u32))
  }
  pub unsafe fn set_pccdummy82(&mut self, value: Pccdummy82) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x148) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy82<F: FnOnce(Pccdummy82) -> Pccdummy82>(&mut self, f: F) {
     let tmp = self.pccdummy82();
     self.set_pccdummy82(f(tmp))
  }

  pub unsafe fn pccdummy83(&self) -> Pccdummy83 { 
     Pccdummy83(::core::ptr::read_volatile(((self.0 as usize) + 0x14c) as *const u32))
  }
  pub unsafe fn set_pccdummy83(&mut self, value: Pccdummy83) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy83<F: FnOnce(Pccdummy83) -> Pccdummy83>(&mut self, f: F) {
     let tmp = self.pccdummy83();
     self.set_pccdummy83(f(tmp))
  }

  pub unsafe fn pccdummy84(&self) -> Pccdummy84 { 
     Pccdummy84(::core::ptr::read_volatile(((self.0 as usize) + 0x150) as *const u32))
  }
  pub unsafe fn set_pccdummy84(&mut self, value: Pccdummy84) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x150) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy84<F: FnOnce(Pccdummy84) -> Pccdummy84>(&mut self, f: F) {
     let tmp = self.pccdummy84();
     self.set_pccdummy84(f(tmp))
  }

  pub unsafe fn pccdummy85(&self) -> Pccdummy85 { 
     Pccdummy85(::core::ptr::read_volatile(((self.0 as usize) + 0x154) as *const u32))
  }
  pub unsafe fn set_pccdummy85(&mut self, value: Pccdummy85) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x154) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy85<F: FnOnce(Pccdummy85) -> Pccdummy85>(&mut self, f: F) {
     let tmp = self.pccdummy85();
     self.set_pccdummy85(f(tmp))
  }

  pub unsafe fn pccdummy86(&self) -> Pccdummy86 { 
     Pccdummy86(::core::ptr::read_volatile(((self.0 as usize) + 0x158) as *const u32))
  }
  pub unsafe fn set_pccdummy86(&mut self, value: Pccdummy86) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x158) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy86<F: FnOnce(Pccdummy86) -> Pccdummy86>(&mut self, f: F) {
     let tmp = self.pccdummy86();
     self.set_pccdummy86(f(tmp))
  }

  pub unsafe fn pccdummy87(&self) -> Pccdummy87 { 
     Pccdummy87(::core::ptr::read_volatile(((self.0 as usize) + 0x15c) as *const u32))
  }
  pub unsafe fn set_pccdummy87(&mut self, value: Pccdummy87) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x15c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy87<F: FnOnce(Pccdummy87) -> Pccdummy87>(&mut self, f: F) {
     let tmp = self.pccdummy87();
     self.set_pccdummy87(f(tmp))
  }

  pub unsafe fn pccdummy88(&self) -> Pccdummy88 { 
     Pccdummy88(::core::ptr::read_volatile(((self.0 as usize) + 0x160) as *const u32))
  }
  pub unsafe fn set_pccdummy88(&mut self, value: Pccdummy88) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x160) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy88<F: FnOnce(Pccdummy88) -> Pccdummy88>(&mut self, f: F) {
     let tmp = self.pccdummy88();
     self.set_pccdummy88(f(tmp))
  }

  pub unsafe fn pccdummy89(&self) -> Pccdummy89 { 
     Pccdummy89(::core::ptr::read_volatile(((self.0 as usize) + 0x164) as *const u32))
  }
  pub unsafe fn set_pccdummy89(&mut self, value: Pccdummy89) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x164) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy89<F: FnOnce(Pccdummy89) -> Pccdummy89>(&mut self, f: F) {
     let tmp = self.pccdummy89();
     self.set_pccdummy89(f(tmp))
  }

  pub unsafe fn flexio(&self) -> Flexio { 
     Flexio(::core::ptr::read_volatile(((self.0 as usize) + 0x168) as *const u32))
  }
  pub unsafe fn set_flexio(&mut self, value: Flexio) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x168) as *mut u32, value.0);
  }
  pub unsafe fn with_flexio<F: FnOnce(Flexio) -> Flexio>(&mut self, f: F) {
     let tmp = self.flexio();
     self.set_flexio(f(tmp))
  }

  pub unsafe fn pccdummy91(&self) -> Pccdummy91 { 
     Pccdummy91(::core::ptr::read_volatile(((self.0 as usize) + 0x16c) as *const u32))
  }
  pub unsafe fn set_pccdummy91(&mut self, value: Pccdummy91) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x16c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy91<F: FnOnce(Pccdummy91) -> Pccdummy91>(&mut self, f: F) {
     let tmp = self.pccdummy91();
     self.set_pccdummy91(f(tmp))
  }

  pub unsafe fn pccdummy92(&self) -> Pccdummy92 { 
     Pccdummy92(::core::ptr::read_volatile(((self.0 as usize) + 0x170) as *const u32))
  }
  pub unsafe fn set_pccdummy92(&mut self, value: Pccdummy92) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x170) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy92<F: FnOnce(Pccdummy92) -> Pccdummy92>(&mut self, f: F) {
     let tmp = self.pccdummy92();
     self.set_pccdummy92(f(tmp))
  }

  pub unsafe fn pccdummy93(&self) -> Pccdummy93 { 
     Pccdummy93(::core::ptr::read_volatile(((self.0 as usize) + 0x174) as *const u32))
  }
  pub unsafe fn set_pccdummy93(&mut self, value: Pccdummy93) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x174) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy93<F: FnOnce(Pccdummy93) -> Pccdummy93>(&mut self, f: F) {
     let tmp = self.pccdummy93();
     self.set_pccdummy93(f(tmp))
  }

  pub unsafe fn pccdummy94(&self) -> Pccdummy94 { 
     Pccdummy94(::core::ptr::read_volatile(((self.0 as usize) + 0x178) as *const u32))
  }
  pub unsafe fn set_pccdummy94(&mut self, value: Pccdummy94) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x178) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy94<F: FnOnce(Pccdummy94) -> Pccdummy94>(&mut self, f: F) {
     let tmp = self.pccdummy94();
     self.set_pccdummy94(f(tmp))
  }

  pub unsafe fn pccdummy95(&self) -> Pccdummy95 { 
     Pccdummy95(::core::ptr::read_volatile(((self.0 as usize) + 0x17c) as *const u32))
  }
  pub unsafe fn set_pccdummy95(&mut self, value: Pccdummy95) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x17c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy95<F: FnOnce(Pccdummy95) -> Pccdummy95>(&mut self, f: F) {
     let tmp = self.pccdummy95();
     self.set_pccdummy95(f(tmp))
  }

  pub unsafe fn pccdummy96(&self) -> Pccdummy96 { 
     Pccdummy96(::core::ptr::read_volatile(((self.0 as usize) + 0x180) as *const u32))
  }
  pub unsafe fn set_pccdummy96(&mut self, value: Pccdummy96) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x180) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy96<F: FnOnce(Pccdummy96) -> Pccdummy96>(&mut self, f: F) {
     let tmp = self.pccdummy96();
     self.set_pccdummy96(f(tmp))
  }

  pub unsafe fn ewm(&self) -> Ewm { 
     Ewm(::core::ptr::read_volatile(((self.0 as usize) + 0x184) as *const u32))
  }
  pub unsafe fn set_ewm(&mut self, value: Ewm) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x184) as *mut u32, value.0);
  }
  pub unsafe fn with_ewm<F: FnOnce(Ewm) -> Ewm>(&mut self, f: F) {
     let tmp = self.ewm();
     self.set_ewm(f(tmp))
  }

  pub unsafe fn pccdummy98(&self) -> Pccdummy98 { 
     Pccdummy98(::core::ptr::read_volatile(((self.0 as usize) + 0x188) as *const u32))
  }
  pub unsafe fn set_pccdummy98(&mut self, value: Pccdummy98) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x188) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy98<F: FnOnce(Pccdummy98) -> Pccdummy98>(&mut self, f: F) {
     let tmp = self.pccdummy98();
     self.set_pccdummy98(f(tmp))
  }

  pub unsafe fn pccdummy99(&self) -> Pccdummy99 { 
     Pccdummy99(::core::ptr::read_volatile(((self.0 as usize) + 0x18c) as *const u32))
  }
  pub unsafe fn set_pccdummy99(&mut self, value: Pccdummy99) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy99<F: FnOnce(Pccdummy99) -> Pccdummy99>(&mut self, f: F) {
     let tmp = self.pccdummy99();
     self.set_pccdummy99(f(tmp))
  }

  pub unsafe fn pccdummy100(&self) -> Pccdummy100 { 
     Pccdummy100(::core::ptr::read_volatile(((self.0 as usize) + 0x190) as *const u32))
  }
  pub unsafe fn set_pccdummy100(&mut self, value: Pccdummy100) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x190) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy100<F: FnOnce(Pccdummy100) -> Pccdummy100>(&mut self, f: F) {
     let tmp = self.pccdummy100();
     self.set_pccdummy100(f(tmp))
  }

  pub unsafe fn pccdummy101(&self) -> Pccdummy101 { 
     Pccdummy101(::core::ptr::read_volatile(((self.0 as usize) + 0x194) as *const u32))
  }
  pub unsafe fn set_pccdummy101(&mut self, value: Pccdummy101) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x194) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy101<F: FnOnce(Pccdummy101) -> Pccdummy101>(&mut self, f: F) {
     let tmp = self.pccdummy101();
     self.set_pccdummy101(f(tmp))
  }

  pub unsafe fn lpi2c0(&self) -> Lpi2c0 { 
     Lpi2c0(::core::ptr::read_volatile(((self.0 as usize) + 0x198) as *const u32))
  }
  pub unsafe fn set_lpi2c0(&mut self, value: Lpi2c0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x198) as *mut u32, value.0);
  }
  pub unsafe fn with_lpi2c0<F: FnOnce(Lpi2c0) -> Lpi2c0>(&mut self, f: F) {
     let tmp = self.lpi2c0();
     self.set_lpi2c0(f(tmp))
  }

  pub unsafe fn pccdummy103(&self) -> Pccdummy103 { 
     Pccdummy103(::core::ptr::read_volatile(((self.0 as usize) + 0x19c) as *const u32))
  }
  pub unsafe fn set_pccdummy103(&mut self, value: Pccdummy103) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x19c) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy103<F: FnOnce(Pccdummy103) -> Pccdummy103>(&mut self, f: F) {
     let tmp = self.pccdummy103();
     self.set_pccdummy103(f(tmp))
  }

  pub unsafe fn pccdummy104(&self) -> Pccdummy104 { 
     Pccdummy104(::core::ptr::read_volatile(((self.0 as usize) + 0x1a0) as *const u32))
  }
  pub unsafe fn set_pccdummy104(&mut self, value: Pccdummy104) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1a0) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy104<F: FnOnce(Pccdummy104) -> Pccdummy104>(&mut self, f: F) {
     let tmp = self.pccdummy104();
     self.set_pccdummy104(f(tmp))
  }

  pub unsafe fn pccdummy105(&self) -> Pccdummy105 { 
     Pccdummy105(::core::ptr::read_volatile(((self.0 as usize) + 0x1a4) as *const u32))
  }
  pub unsafe fn set_pccdummy105(&mut self, value: Pccdummy105) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1a4) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy105<F: FnOnce(Pccdummy105) -> Pccdummy105>(&mut self, f: F) {
     let tmp = self.pccdummy105();
     self.set_pccdummy105(f(tmp))
  }

  pub unsafe fn lpuart0(&self) -> Lpuart0 { 
     Lpuart0(::core::ptr::read_volatile(((self.0 as usize) + 0x1a8) as *const u32))
  }
  pub unsafe fn set_lpuart0(&mut self, value: Lpuart0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1a8) as *mut u32, value.0);
  }
  pub unsafe fn with_lpuart0<F: FnOnce(Lpuart0) -> Lpuart0>(&mut self, f: F) {
     let tmp = self.lpuart0();
     self.set_lpuart0(f(tmp))
  }

  pub unsafe fn lpuart1(&self) -> Lpuart1 { 
     Lpuart1(::core::ptr::read_volatile(((self.0 as usize) + 0x1ac) as *const u32))
  }
  pub unsafe fn set_lpuart1(&mut self, value: Lpuart1) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1ac) as *mut u32, value.0);
  }
  pub unsafe fn with_lpuart1<F: FnOnce(Lpuart1) -> Lpuart1>(&mut self, f: F) {
     let tmp = self.lpuart1();
     self.set_lpuart1(f(tmp))
  }

  pub unsafe fn lpuart2(&self) -> Lpuart2 { 
     Lpuart2(::core::ptr::read_volatile(((self.0 as usize) + 0x1b0) as *const u32))
  }
  pub unsafe fn set_lpuart2(&mut self, value: Lpuart2) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1b0) as *mut u32, value.0);
  }
  pub unsafe fn with_lpuart2<F: FnOnce(Lpuart2) -> Lpuart2>(&mut self, f: F) {
     let tmp = self.lpuart2();
     self.set_lpuart2(f(tmp))
  }

  pub unsafe fn pccdummy109(&self) -> Pccdummy109 { 
     Pccdummy109(::core::ptr::read_volatile(((self.0 as usize) + 0x1b4) as *const u32))
  }
  pub unsafe fn set_pccdummy109(&mut self, value: Pccdummy109) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1b4) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy109<F: FnOnce(Pccdummy109) -> Pccdummy109>(&mut self, f: F) {
     let tmp = self.pccdummy109();
     self.set_pccdummy109(f(tmp))
  }

  pub unsafe fn pccdummy110(&self) -> Pccdummy110 { 
     Pccdummy110(::core::ptr::read_volatile(((self.0 as usize) + 0x1b8) as *const u32))
  }
  pub unsafe fn set_pccdummy110(&mut self, value: Pccdummy110) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1b8) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy110<F: FnOnce(Pccdummy110) -> Pccdummy110>(&mut self, f: F) {
     let tmp = self.pccdummy110();
     self.set_pccdummy110(f(tmp))
  }

  pub unsafe fn pccdummy111(&self) -> Pccdummy111 { 
     Pccdummy111(::core::ptr::read_volatile(((self.0 as usize) + 0x1bc) as *const u32))
  }
  pub unsafe fn set_pccdummy111(&mut self, value: Pccdummy111) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1bc) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy111<F: FnOnce(Pccdummy111) -> Pccdummy111>(&mut self, f: F) {
     let tmp = self.pccdummy111();
     self.set_pccdummy111(f(tmp))
  }

  pub unsafe fn pccdummy112(&self) -> Pccdummy112 { 
     Pccdummy112(::core::ptr::read_volatile(((self.0 as usize) + 0x1c0) as *const u32))
  }
  pub unsafe fn set_pccdummy112(&mut self, value: Pccdummy112) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c0) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy112<F: FnOnce(Pccdummy112) -> Pccdummy112>(&mut self, f: F) {
     let tmp = self.pccdummy112();
     self.set_pccdummy112(f(tmp))
  }

  pub unsafe fn pccdummy113(&self) -> Pccdummy113 { 
     Pccdummy113(::core::ptr::read_volatile(((self.0 as usize) + 0x1c4) as *const u32))
  }
  pub unsafe fn set_pccdummy113(&mut self, value: Pccdummy113) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c4) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy113<F: FnOnce(Pccdummy113) -> Pccdummy113>(&mut self, f: F) {
     let tmp = self.pccdummy113();
     self.set_pccdummy113(f(tmp))
  }

  pub unsafe fn pccdummy114(&self) -> Pccdummy114 { 
     Pccdummy114(::core::ptr::read_volatile(((self.0 as usize) + 0x1c8) as *const u32))
  }
  pub unsafe fn set_pccdummy114(&mut self, value: Pccdummy114) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c8) as *mut u32, value.0);
  }
  pub unsafe fn with_pccdummy114<F: FnOnce(Pccdummy114) -> Pccdummy114>(&mut self, f: F) {
     let tmp = self.pccdummy114();
     self.set_pccdummy114(f(tmp))
  }

  pub unsafe fn cmp0(&self) -> Cmp0 { 
     Cmp0(::core::ptr::read_volatile(((self.0 as usize) + 0x1cc) as *const u32))
  }
  pub unsafe fn set_cmp0(&mut self, value: Cmp0) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1cc) as *mut u32, value.0);
  }
  pub unsafe fn with_cmp0<F: FnOnce(Cmp0) -> Cmp0>(&mut self, f: F) {
     let tmp = self.cmp0();
     self.set_cmp0(f(tmp))
  }

}

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

#[derive(PartialEq, Eq)]
pub struct Ftfc(pub u32);

impl Ftfc {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Dmamux(pub u32);

impl Dmamux {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Flexcan0(pub u32);

impl Flexcan0 {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Flexcan1(pub u32);

impl Flexcan1 {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Ftm3(pub u32);

impl Ftm3 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Adc1(pub u32);

impl Adc1 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Flexcan2(pub u32);

impl Flexcan2 {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Lpspi0(pub u32);

impl Lpspi0 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Lpspi1(pub u32);

impl Lpspi1 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Lpspi2(pub u32);

impl Lpspi2 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Pdb1(pub u32);

impl Pdb1 {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Crc(pub u32);

impl Crc {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Pdb0(pub u32);

impl Pdb0 {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Lpit(pub u32);

impl Lpit {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Ftm0(pub u32);

impl Ftm0 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Ftm1(pub u32);

impl Ftm1 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Ftm2(pub u32);

impl Ftm2 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Adc0(pub u32);

impl Adc0 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Rtc(pub u32);

impl Rtc {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Lptmr0(pub u32);

impl Lptmr0 {
  pub fn pcd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_pcd(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn frac(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_frac(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Porta(pub u32);

impl Porta {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Portb(pub u32);

impl Portb {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Portc(pub u32);

impl Portc {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Portd(pub u32);

impl Portd {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Porte(pub u32);

impl Porte {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Flexio(pub u32);

impl Flexio {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Ewm(pub u32);

impl Ewm {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Lpi2c0(pub u32);

impl Lpi2c0 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Lpuart0(pub u32);

impl Lpuart0 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Lpuart1(pub u32);

impl Lpuart1 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Lpuart2(pub u32);

impl Lpuart2 {
  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

#[derive(PartialEq, Eq)]
pub struct Cmp0(pub u32);

impl Cmp0 {
  pub fn cgc(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cgc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
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

