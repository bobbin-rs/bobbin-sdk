pub const SPI0: Spi = Spi(0x4002c000);
pub const SPI1: Spi = Spi(0x4002d000);
pub const SPI2: Spi = Spi(0x400ac000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Spi(pub u32);

impl Spi {
  pub unsafe fn mcr(&self) -> Mcr { 
     Mcr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_mcr(&mut self, value: Mcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&mut self, f: F) {
     let tmp = self.mcr();
     self.set_mcr(f(tmp))
  }

  pub unsafe fn tcr(&self) -> Tcr { 
     Tcr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_tcr(&mut self, value: Tcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_tcr<F: FnOnce(Tcr) -> Tcr>(&mut self, f: F) {
     let tmp = self.tcr();
     self.set_tcr(f(tmp))
  }

  pub unsafe fn ctar(&self, index: usize) -> Ctar { 
     assert!(index < 2);
     Ctar(::core::ptr::read_volatile(((self.0 as usize) + 0xc + (index << 2)) as *const u32))
  }
  pub unsafe fn set_ctar(&mut self, index: usize, value: Ctar) {
     assert!(index < 2);
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc + (index << 2)) as *mut u32, value.0);
  }
  pub unsafe fn with_ctar<F: FnOnce(Ctar) -> Ctar>(&mut self, index: usize, f: F) {
     let tmp = self.ctar(index);
     self.set_ctar(index, f(tmp))
  }

  pub unsafe fn ctar_slave(&self) -> CtarSlave { 
     CtarSlave(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_ctar_slave(&mut self, value: CtarSlave) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_ctar_slave<F: FnOnce(CtarSlave) -> CtarSlave>(&mut self, f: F) {
     let tmp = self.ctar_slave();
     self.set_ctar_slave(f(tmp))
  }

  pub unsafe fn sr(&self) -> Sr { 
     Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
  }
  pub unsafe fn set_sr(&mut self, value: Sr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
  }
  pub unsafe fn with_sr<F: FnOnce(Sr) -> Sr>(&mut self, f: F) {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

  pub unsafe fn rser(&self) -> Rser { 
     Rser(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
  }
  pub unsafe fn set_rser(&mut self, value: Rser) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
  }
  pub unsafe fn with_rser<F: FnOnce(Rser) -> Rser>(&mut self, f: F) {
     let tmp = self.rser();
     self.set_rser(f(tmp))
  }

  pub unsafe fn pushr(&self) -> Pushr { 
     Pushr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
  }
  pub unsafe fn set_pushr(&mut self, value: Pushr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
  }
  pub unsafe fn with_pushr<F: FnOnce(Pushr) -> Pushr>(&mut self, f: F) {
     let tmp = self.pushr();
     self.set_pushr(f(tmp))
  }

  pub unsafe fn pushr_slave(&self) -> PushrSlave { 
     PushrSlave(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
  }
  pub unsafe fn set_pushr_slave(&mut self, value: PushrSlave) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
  }
  pub unsafe fn with_pushr_slave<F: FnOnce(PushrSlave) -> PushrSlave>(&mut self, f: F) {
     let tmp = self.pushr_slave();
     self.set_pushr_slave(f(tmp))
  }

  pub unsafe fn popr(&self) -> Popr { 
     Popr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
  }

  pub unsafe fn txfr(&self, index: usize) -> Txfr { 
     assert!(index < 4);
     Txfr(::core::ptr::read_volatile(((self.0 as usize) + 0x3c + (index << 2)) as *const u32))
  }

  pub unsafe fn rxfr(&self, index: usize) -> Rxfr { 
     assert!(index < 4);
     Rxfr(::core::ptr::read_volatile(((self.0 as usize) + 0x7c + (index << 2)) as *const u32))
  }

}

#[derive(PartialEq, Eq)]
pub struct Mcr(pub u32);

impl Mcr {
  pub fn halt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_halt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn smpl_pt(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_smpl_pt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn clr_rxf(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_clr_rxf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn clr_txf(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_clr_txf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn dis_rxf(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_dis_rxf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn dis_txf(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_dis_txf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn mdis(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_mdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn doze(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_doze(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn pcsis(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3f // [21:16]
  }
  pub fn set_pcsis(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rooe(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_rooe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn pcsse(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_pcsse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn mtfe(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_mtfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn frz(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_frz(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn dconf(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
  pub fn set_dconf(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn cont_scke(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_cont_scke(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn mstr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_mstr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.halt() != 0 { try!(write!(f, " halt"))}
      if self.smpl_pt() != 0 { try!(write!(f, " smpl_pt=0x{:x}", self.smpl_pt()))}
      if self.clr_rxf() != 0 { try!(write!(f, " clr_rxf"))}
      if self.clr_txf() != 0 { try!(write!(f, " clr_txf"))}
      if self.dis_rxf() != 0 { try!(write!(f, " dis_rxf"))}
      if self.dis_txf() != 0 { try!(write!(f, " dis_txf"))}
      if self.mdis() != 0 { try!(write!(f, " mdis"))}
      if self.doze() != 0 { try!(write!(f, " doze"))}
      if self.pcsis() != 0 { try!(write!(f, " pcsis=0x{:x}", self.pcsis()))}
      if self.rooe() != 0 { try!(write!(f, " rooe"))}
      if self.pcsse() != 0 { try!(write!(f, " pcsse"))}
      if self.mtfe() != 0 { try!(write!(f, " mtfe"))}
      if self.frz() != 0 { try!(write!(f, " frz"))}
      if self.dconf() != 0 { try!(write!(f, " dconf=0x{:x}", self.dconf()))}
      if self.cont_scke() != 0 { try!(write!(f, " cont_scke"))}
      if self.mstr() != 0 { try!(write!(f, " mstr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Tcr(pub u32);

impl Tcr {
  pub fn spi_tcnt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  pub fn set_spi_tcnt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Tcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Tcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.spi_tcnt() != 0 { try!(write!(f, " spi_tcnt=0x{:x}", self.spi_tcnt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ctar(pub u32);

impl Ctar {
  pub fn br(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_br(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  pub fn asc(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_asc(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn cssck(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  pub fn set_cssck(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  pub fn pbr(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_pbr(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn pdt(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x3 // [19:18]
  }
  pub fn set_pdt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn pasc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
  pub fn set_pasc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn pcssck(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x3 // [23:22]
  }
  pub fn set_pcssck(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn lsbfe(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_lsbfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn cpha(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_cpha(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn cpol(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_cpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn fmsz(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0xf // [30:27]
  }
  pub fn set_fmsz(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 27);
     self.0 |= value << 27;
     self
  }

  pub fn dbr(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_dbr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Ctar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ctar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.br() != 0 { try!(write!(f, " br=0x{:x}", self.br()))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.asc() != 0 { try!(write!(f, " asc=0x{:x}", self.asc()))}
      if self.cssck() != 0 { try!(write!(f, " cssck=0x{:x}", self.cssck()))}
      if self.pbr() != 0 { try!(write!(f, " pbr=0x{:x}", self.pbr()))}
      if self.pdt() != 0 { try!(write!(f, " pdt=0x{:x}", self.pdt()))}
      if self.pasc() != 0 { try!(write!(f, " pasc=0x{:x}", self.pasc()))}
      if self.pcssck() != 0 { try!(write!(f, " pcssck=0x{:x}", self.pcssck()))}
      if self.lsbfe() != 0 { try!(write!(f, " lsbfe"))}
      if self.cpha() != 0 { try!(write!(f, " cpha"))}
      if self.cpol() != 0 { try!(write!(f, " cpol"))}
      if self.fmsz() != 0 { try!(write!(f, " fmsz=0x{:x}", self.fmsz()))}
      if self.dbr() != 0 { try!(write!(f, " dbr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct CtarSlave(pub u32);

impl CtarSlave {
  pub fn cpha(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_cpha(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn cpol(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_cpol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn fmsz(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1f // [31:27]
  }
  pub fn set_fmsz(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 27);
     self.0 |= value << 27;
     self
  }

}

impl ::core::fmt::Display for CtarSlave {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for CtarSlave {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cpha() != 0 { try!(write!(f, " cpha"))}
      if self.cpol() != 0 { try!(write!(f, " cpol"))}
      if self.fmsz() != 0 { try!(write!(f, " fmsz=0x{:x}", self.fmsz()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);

impl Sr {
  pub fn popnxtptr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_popnxtptr(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rxctr(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
  pub fn set_rxctr(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

  pub fn txnxtptr(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_txnxtptr(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn txctr(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0xf // [15:12]
  }
  pub fn set_txctr(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 12);
     self.0 |= value << 12;
     self
  }

  pub fn rfdf(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_rfdf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn rfof(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_rfof(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn tfff(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_tfff(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn tfuf(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_tfuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn eoqf(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_eoqf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn txrxs(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_txrxs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn tcf(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_tcf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
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
      if self.popnxtptr() != 0 { try!(write!(f, " popnxtptr=0x{:x}", self.popnxtptr()))}
      if self.rxctr() != 0 { try!(write!(f, " rxctr=0x{:x}", self.rxctr()))}
      if self.txnxtptr() != 0 { try!(write!(f, " txnxtptr=0x{:x}", self.txnxtptr()))}
      if self.txctr() != 0 { try!(write!(f, " txctr=0x{:x}", self.txctr()))}
      if self.rfdf() != 0 { try!(write!(f, " rfdf"))}
      if self.rfof() != 0 { try!(write!(f, " rfof"))}
      if self.tfff() != 0 { try!(write!(f, " tfff"))}
      if self.tfuf() != 0 { try!(write!(f, " tfuf"))}
      if self.eoqf() != 0 { try!(write!(f, " eoqf"))}
      if self.txrxs() != 0 { try!(write!(f, " txrxs"))}
      if self.tcf() != 0 { try!(write!(f, " tcf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rser(pub u32);

impl Rser {
  pub fn rfdf_dirs(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_rfdf_dirs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rfdf_re(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_rfdf_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn rfof_re(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_rfof_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn tfff_dirs(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_tfff_dirs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn tfff_re(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_tfff_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn tfuf_re(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_tfuf_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn eoqf_re(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_eoqf_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn tcf_re(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_tcf_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Rser {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rser {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rfdf_dirs() != 0 { try!(write!(f, " rfdf_dirs"))}
      if self.rfdf_re() != 0 { try!(write!(f, " rfdf_re"))}
      if self.rfof_re() != 0 { try!(write!(f, " rfof_re"))}
      if self.tfff_dirs() != 0 { try!(write!(f, " tfff_dirs"))}
      if self.tfff_re() != 0 { try!(write!(f, " tfff_re"))}
      if self.tfuf_re() != 0 { try!(write!(f, " tfuf_re"))}
      if self.eoqf_re() != 0 { try!(write!(f, " eoqf_re"))}
      if self.tcf_re() != 0 { try!(write!(f, " tcf_re"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pushr(pub u32);

impl Pushr {
  pub fn txdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txdata(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn pcs(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3f // [21:16]
  }
  pub fn set_pcs(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ctcnt(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_ctcnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn eoq(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_eoq(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn ctas(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x7 // [30:28]
  }
  pub fn set_ctas(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn cont(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_cont(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Pushr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pushr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txdata() != 0 { try!(write!(f, " txdata=0x{:x}", self.txdata()))}
      if self.pcs() != 0 { try!(write!(f, " pcs=0x{:x}", self.pcs()))}
      if self.ctcnt() != 0 { try!(write!(f, " ctcnt"))}
      if self.eoq() != 0 { try!(write!(f, " eoq"))}
      if self.ctas() != 0 { try!(write!(f, " ctas=0x{:x}", self.ctas()))}
      if self.cont() != 0 { try!(write!(f, " cont"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct PushrSlave(pub u32);

impl PushrSlave {
  pub fn txdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_txdata(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for PushrSlave {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for PushrSlave {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Popr(pub u32);

impl Popr {
  pub fn rxdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_rxdata(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Popr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Popr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Txfr(pub u32);

impl Txfr {
  pub fn txdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_txdata(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn txcmd_txdata(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  pub fn set_txcmd_txdata(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Txfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Txfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txdata() != 0 { try!(write!(f, " txdata=0x{:x}", self.txdata()))}
      if self.txcmd_txdata() != 0 { try!(write!(f, " txcmd_txdata=0x{:x}", self.txcmd_txdata()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxfr(pub u32);

impl Rxfr {
  pub fn rxdata(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_rxdata(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rxfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

