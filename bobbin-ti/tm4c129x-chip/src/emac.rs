pub const EMAC0: Emac = Emac(0x400ec000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Emac(pub u32);

impl Emac {
  pub unsafe fn cfg(&self) -> Cfg { 
     Cfg(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }
  pub unsafe fn set_cfg(&mut self, value: Cfg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
  }
  pub unsafe fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&mut self, f: F) {
     let tmp = self.cfg();
     self.set_cfg(f(tmp))
  }

  pub unsafe fn framefltr(&self) -> Framefltr { 
     Framefltr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
  }
  pub unsafe fn set_framefltr(&mut self, value: Framefltr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }
  pub unsafe fn with_framefltr<F: FnOnce(Framefltr) -> Framefltr>(&mut self, f: F) {
     let tmp = self.framefltr();
     self.set_framefltr(f(tmp))
  }

  pub unsafe fn hashtblh(&self) -> Hashtblh { 
     Hashtblh(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
  }
  pub unsafe fn set_hashtblh(&mut self, value: Hashtblh) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
  }
  pub unsafe fn with_hashtblh<F: FnOnce(Hashtblh) -> Hashtblh>(&mut self, f: F) {
     let tmp = self.hashtblh();
     self.set_hashtblh(f(tmp))
  }

  pub unsafe fn hashtbll(&self) -> Hashtbll { 
     Hashtbll(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
  }
  pub unsafe fn set_hashtbll(&mut self, value: Hashtbll) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
  }
  pub unsafe fn with_hashtbll<F: FnOnce(Hashtbll) -> Hashtbll>(&mut self, f: F) {
     let tmp = self.hashtbll();
     self.set_hashtbll(f(tmp))
  }

  pub unsafe fn miiaddr(&self) -> Miiaddr { 
     Miiaddr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
  }
  pub unsafe fn set_miiaddr(&mut self, value: Miiaddr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
  }
  pub unsafe fn with_miiaddr<F: FnOnce(Miiaddr) -> Miiaddr>(&mut self, f: F) {
     let tmp = self.miiaddr();
     self.set_miiaddr(f(tmp))
  }

  pub unsafe fn miidata(&self) -> Miidata { 
     Miidata(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
  }
  pub unsafe fn set_miidata(&mut self, value: Miidata) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
  }
  pub unsafe fn with_miidata<F: FnOnce(Miidata) -> Miidata>(&mut self, f: F) {
     let tmp = self.miidata();
     self.set_miidata(f(tmp))
  }

  pub unsafe fn flowctl(&self) -> Flowctl { 
     Flowctl(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
  }
  pub unsafe fn set_flowctl(&mut self, value: Flowctl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
  }
  pub unsafe fn with_flowctl<F: FnOnce(Flowctl) -> Flowctl>(&mut self, f: F) {
     let tmp = self.flowctl();
     self.set_flowctl(f(tmp))
  }

  pub unsafe fn vlantg(&self) -> Vlantg { 
     Vlantg(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
  }
  pub unsafe fn set_vlantg(&mut self, value: Vlantg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
  }
  pub unsafe fn with_vlantg<F: FnOnce(Vlantg) -> Vlantg>(&mut self, f: F) {
     let tmp = self.vlantg();
     self.set_vlantg(f(tmp))
  }

  pub unsafe fn status(&self) -> Status { 
     Status(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
  }
  pub unsafe fn set_status(&mut self, value: Status) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
  }
  pub unsafe fn with_status<F: FnOnce(Status) -> Status>(&mut self, f: F) {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

  pub unsafe fn rwuff(&self) -> Rwuff { 
     Rwuff(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
  }
  pub unsafe fn set_rwuff(&mut self, value: Rwuff) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
  }
  pub unsafe fn with_rwuff<F: FnOnce(Rwuff) -> Rwuff>(&mut self, f: F) {
     let tmp = self.rwuff();
     self.set_rwuff(f(tmp))
  }

  pub unsafe fn pmtctlstat(&self) -> Pmtctlstat { 
     Pmtctlstat(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
  }
  pub unsafe fn set_pmtctlstat(&mut self, value: Pmtctlstat) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
  }
  pub unsafe fn with_pmtctlstat<F: FnOnce(Pmtctlstat) -> Pmtctlstat>(&mut self, f: F) {
     let tmp = self.pmtctlstat();
     self.set_pmtctlstat(f(tmp))
  }

  pub unsafe fn ris(&self) -> Ris { 
     Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
  }
  pub unsafe fn set_ris(&mut self, value: Ris) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
  }
  pub unsafe fn with_ris<F: FnOnce(Ris) -> Ris>(&mut self, f: F) {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

  pub unsafe fn im(&self) -> Im { 
     Im(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
  }
  pub unsafe fn set_im(&mut self, value: Im) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
  }
  pub unsafe fn with_im<F: FnOnce(Im) -> Im>(&mut self, f: F) {
     let tmp = self.im();
     self.set_im(f(tmp))
  }

  pub unsafe fn addr0h(&self) -> Addr0h { 
     Addr0h(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
  }
  pub unsafe fn set_addr0h(&mut self, value: Addr0h) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
  }
  pub unsafe fn with_addr0h<F: FnOnce(Addr0h) -> Addr0h>(&mut self, f: F) {
     let tmp = self.addr0h();
     self.set_addr0h(f(tmp))
  }

  pub unsafe fn addr0l(&self) -> Addr0l { 
     Addr0l(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
  }
  pub unsafe fn set_addr0l(&mut self, value: Addr0l) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
  }
  pub unsafe fn with_addr0l<F: FnOnce(Addr0l) -> Addr0l>(&mut self, f: F) {
     let tmp = self.addr0l();
     self.set_addr0l(f(tmp))
  }

  pub unsafe fn addr1h(&self) -> Addr1h { 
     Addr1h(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
  }
  pub unsafe fn set_addr1h(&mut self, value: Addr1h) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
  }
  pub unsafe fn with_addr1h<F: FnOnce(Addr1h) -> Addr1h>(&mut self, f: F) {
     let tmp = self.addr1h();
     self.set_addr1h(f(tmp))
  }

  pub unsafe fn addr1l(&self) -> Addr1l { 
     Addr1l(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
  }
  pub unsafe fn set_addr1l(&mut self, value: Addr1l) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
  }
  pub unsafe fn with_addr1l<F: FnOnce(Addr1l) -> Addr1l>(&mut self, f: F) {
     let tmp = self.addr1l();
     self.set_addr1l(f(tmp))
  }

  pub unsafe fn addr2h(&self) -> Addr2h { 
     Addr2h(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
  }
  pub unsafe fn set_addr2h(&mut self, value: Addr2h) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
  }
  pub unsafe fn with_addr2h<F: FnOnce(Addr2h) -> Addr2h>(&mut self, f: F) {
     let tmp = self.addr2h();
     self.set_addr2h(f(tmp))
  }

  pub unsafe fn addr2l(&self) -> Addr2l { 
     Addr2l(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
  }
  pub unsafe fn set_addr2l(&mut self, value: Addr2l) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
  }
  pub unsafe fn with_addr2l<F: FnOnce(Addr2l) -> Addr2l>(&mut self, f: F) {
     let tmp = self.addr2l();
     self.set_addr2l(f(tmp))
  }

  pub unsafe fn addr3h(&self) -> Addr3h { 
     Addr3h(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
  }
  pub unsafe fn set_addr3h(&mut self, value: Addr3h) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
  }
  pub unsafe fn with_addr3h<F: FnOnce(Addr3h) -> Addr3h>(&mut self, f: F) {
     let tmp = self.addr3h();
     self.set_addr3h(f(tmp))
  }

  pub unsafe fn addr3l(&self) -> Addr3l { 
     Addr3l(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
  }
  pub unsafe fn set_addr3l(&mut self, value: Addr3l) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
  }
  pub unsafe fn with_addr3l<F: FnOnce(Addr3l) -> Addr3l>(&mut self, f: F) {
     let tmp = self.addr3l();
     self.set_addr3l(f(tmp))
  }

  pub unsafe fn wdogto(&self) -> Wdogto { 
     Wdogto(::core::ptr::read_volatile(((self.0 as usize) + 0xdc) as *const u32))
  }
  pub unsafe fn set_wdogto(&mut self, value: Wdogto) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xdc) as *mut u32, value.0);
  }
  pub unsafe fn with_wdogto<F: FnOnce(Wdogto) -> Wdogto>(&mut self, f: F) {
     let tmp = self.wdogto();
     self.set_wdogto(f(tmp))
  }

  pub unsafe fn mmcctrl(&self) -> Mmcctrl { 
     Mmcctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x100) as *const u32))
  }
  pub unsafe fn set_mmcctrl(&mut self, value: Mmcctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x100) as *mut u32, value.0);
  }
  pub unsafe fn with_mmcctrl<F: FnOnce(Mmcctrl) -> Mmcctrl>(&mut self, f: F) {
     let tmp = self.mmcctrl();
     self.set_mmcctrl(f(tmp))
  }

  pub unsafe fn mmcrxris(&self) -> Mmcrxris { 
     Mmcrxris(::core::ptr::read_volatile(((self.0 as usize) + 0x104) as *const u32))
  }
  pub unsafe fn set_mmcrxris(&mut self, value: Mmcrxris) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x104) as *mut u32, value.0);
  }
  pub unsafe fn with_mmcrxris<F: FnOnce(Mmcrxris) -> Mmcrxris>(&mut self, f: F) {
     let tmp = self.mmcrxris();
     self.set_mmcrxris(f(tmp))
  }

  pub unsafe fn mmctxris(&self) -> Mmctxris { 
     Mmctxris(::core::ptr::read_volatile(((self.0 as usize) + 0x108) as *const u32))
  }
  pub unsafe fn set_mmctxris(&mut self, value: Mmctxris) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x108) as *mut u32, value.0);
  }
  pub unsafe fn with_mmctxris<F: FnOnce(Mmctxris) -> Mmctxris>(&mut self, f: F) {
     let tmp = self.mmctxris();
     self.set_mmctxris(f(tmp))
  }

  pub unsafe fn mmcrxim(&self) -> Mmcrxim { 
     Mmcrxim(::core::ptr::read_volatile(((self.0 as usize) + 0x10c) as *const u32))
  }
  pub unsafe fn set_mmcrxim(&mut self, value: Mmcrxim) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10c) as *mut u32, value.0);
  }
  pub unsafe fn with_mmcrxim<F: FnOnce(Mmcrxim) -> Mmcrxim>(&mut self, f: F) {
     let tmp = self.mmcrxim();
     self.set_mmcrxim(f(tmp))
  }

  pub unsafe fn mmctxim(&self) -> Mmctxim { 
     Mmctxim(::core::ptr::read_volatile(((self.0 as usize) + 0x110) as *const u32))
  }
  pub unsafe fn set_mmctxim(&mut self, value: Mmctxim) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x110) as *mut u32, value.0);
  }
  pub unsafe fn with_mmctxim<F: FnOnce(Mmctxim) -> Mmctxim>(&mut self, f: F) {
     let tmp = self.mmctxim();
     self.set_mmctxim(f(tmp))
  }

  pub unsafe fn txcntgb(&self) -> Txcntgb { 
     Txcntgb(::core::ptr::read_volatile(((self.0 as usize) + 0x118) as *const u32))
  }
  pub unsafe fn set_txcntgb(&mut self, value: Txcntgb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x118) as *mut u32, value.0);
  }
  pub unsafe fn with_txcntgb<F: FnOnce(Txcntgb) -> Txcntgb>(&mut self, f: F) {
     let tmp = self.txcntgb();
     self.set_txcntgb(f(tmp))
  }

  pub unsafe fn txcntscol(&self) -> Txcntscol { 
     Txcntscol(::core::ptr::read_volatile(((self.0 as usize) + 0x14c) as *const u32))
  }
  pub unsafe fn set_txcntscol(&mut self, value: Txcntscol) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14c) as *mut u32, value.0);
  }
  pub unsafe fn with_txcntscol<F: FnOnce(Txcntscol) -> Txcntscol>(&mut self, f: F) {
     let tmp = self.txcntscol();
     self.set_txcntscol(f(tmp))
  }

  pub unsafe fn txcntmcol(&self) -> Txcntmcol { 
     Txcntmcol(::core::ptr::read_volatile(((self.0 as usize) + 0x150) as *const u32))
  }
  pub unsafe fn set_txcntmcol(&mut self, value: Txcntmcol) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x150) as *mut u32, value.0);
  }
  pub unsafe fn with_txcntmcol<F: FnOnce(Txcntmcol) -> Txcntmcol>(&mut self, f: F) {
     let tmp = self.txcntmcol();
     self.set_txcntmcol(f(tmp))
  }

  pub unsafe fn txoctcntg(&self) -> Txoctcntg { 
     Txoctcntg(::core::ptr::read_volatile(((self.0 as usize) + 0x164) as *const u32))
  }
  pub unsafe fn set_txoctcntg(&mut self, value: Txoctcntg) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x164) as *mut u32, value.0);
  }
  pub unsafe fn with_txoctcntg<F: FnOnce(Txoctcntg) -> Txoctcntg>(&mut self, f: F) {
     let tmp = self.txoctcntg();
     self.set_txoctcntg(f(tmp))
  }

  pub unsafe fn rxcntgb(&self) -> Rxcntgb { 
     Rxcntgb(::core::ptr::read_volatile(((self.0 as usize) + 0x180) as *const u32))
  }
  pub unsafe fn set_rxcntgb(&mut self, value: Rxcntgb) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x180) as *mut u32, value.0);
  }
  pub unsafe fn with_rxcntgb<F: FnOnce(Rxcntgb) -> Rxcntgb>(&mut self, f: F) {
     let tmp = self.rxcntgb();
     self.set_rxcntgb(f(tmp))
  }

  pub unsafe fn rxcntcrcerr(&self) -> Rxcntcrcerr { 
     Rxcntcrcerr(::core::ptr::read_volatile(((self.0 as usize) + 0x194) as *const u32))
  }
  pub unsafe fn set_rxcntcrcerr(&mut self, value: Rxcntcrcerr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x194) as *mut u32, value.0);
  }
  pub unsafe fn with_rxcntcrcerr<F: FnOnce(Rxcntcrcerr) -> Rxcntcrcerr>(&mut self, f: F) {
     let tmp = self.rxcntcrcerr();
     self.set_rxcntcrcerr(f(tmp))
  }

  pub unsafe fn rxcntalgnerr(&self) -> Rxcntalgnerr { 
     Rxcntalgnerr(::core::ptr::read_volatile(((self.0 as usize) + 0x198) as *const u32))
  }
  pub unsafe fn set_rxcntalgnerr(&mut self, value: Rxcntalgnerr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x198) as *mut u32, value.0);
  }
  pub unsafe fn with_rxcntalgnerr<F: FnOnce(Rxcntalgnerr) -> Rxcntalgnerr>(&mut self, f: F) {
     let tmp = self.rxcntalgnerr();
     self.set_rxcntalgnerr(f(tmp))
  }

  pub unsafe fn rxcntguni(&self) -> Rxcntguni { 
     Rxcntguni(::core::ptr::read_volatile(((self.0 as usize) + 0x1c4) as *const u32))
  }
  pub unsafe fn set_rxcntguni(&mut self, value: Rxcntguni) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x1c4) as *mut u32, value.0);
  }
  pub unsafe fn with_rxcntguni<F: FnOnce(Rxcntguni) -> Rxcntguni>(&mut self, f: F) {
     let tmp = self.rxcntguni();
     self.set_rxcntguni(f(tmp))
  }

  pub unsafe fn vlnincrep(&self) -> Vlnincrep { 
     Vlnincrep(::core::ptr::read_volatile(((self.0 as usize) + 0x584) as *const u32))
  }
  pub unsafe fn set_vlnincrep(&mut self, value: Vlnincrep) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x584) as *mut u32, value.0);
  }
  pub unsafe fn with_vlnincrep<F: FnOnce(Vlnincrep) -> Vlnincrep>(&mut self, f: F) {
     let tmp = self.vlnincrep();
     self.set_vlnincrep(f(tmp))
  }

  pub unsafe fn vlanhash(&self) -> Vlanhash { 
     Vlanhash(::core::ptr::read_volatile(((self.0 as usize) + 0x588) as *const u32))
  }
  pub unsafe fn set_vlanhash(&mut self, value: Vlanhash) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x588) as *mut u32, value.0);
  }
  pub unsafe fn with_vlanhash<F: FnOnce(Vlanhash) -> Vlanhash>(&mut self, f: F) {
     let tmp = self.vlanhash();
     self.set_vlanhash(f(tmp))
  }

  pub unsafe fn timstctrl(&self) -> Timstctrl { 
     Timstctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x700) as *const u32))
  }
  pub unsafe fn set_timstctrl(&mut self, value: Timstctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x700) as *mut u32, value.0);
  }
  pub unsafe fn with_timstctrl<F: FnOnce(Timstctrl) -> Timstctrl>(&mut self, f: F) {
     let tmp = self.timstctrl();
     self.set_timstctrl(f(tmp))
  }

  pub unsafe fn subsecinc(&self) -> Subsecinc { 
     Subsecinc(::core::ptr::read_volatile(((self.0 as usize) + 0x704) as *const u32))
  }
  pub unsafe fn set_subsecinc(&mut self, value: Subsecinc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x704) as *mut u32, value.0);
  }
  pub unsafe fn with_subsecinc<F: FnOnce(Subsecinc) -> Subsecinc>(&mut self, f: F) {
     let tmp = self.subsecinc();
     self.set_subsecinc(f(tmp))
  }

  pub unsafe fn timsec(&self) -> Timsec { 
     Timsec(::core::ptr::read_volatile(((self.0 as usize) + 0x708) as *const u32))
  }
  pub unsafe fn set_timsec(&mut self, value: Timsec) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x708) as *mut u32, value.0);
  }
  pub unsafe fn with_timsec<F: FnOnce(Timsec) -> Timsec>(&mut self, f: F) {
     let tmp = self.timsec();
     self.set_timsec(f(tmp))
  }

  pub unsafe fn timnano(&self) -> Timnano { 
     Timnano(::core::ptr::read_volatile(((self.0 as usize) + 0x70c) as *const u32))
  }
  pub unsafe fn set_timnano(&mut self, value: Timnano) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x70c) as *mut u32, value.0);
  }
  pub unsafe fn with_timnano<F: FnOnce(Timnano) -> Timnano>(&mut self, f: F) {
     let tmp = self.timnano();
     self.set_timnano(f(tmp))
  }

  pub unsafe fn timsecu(&self) -> Timsecu { 
     Timsecu(::core::ptr::read_volatile(((self.0 as usize) + 0x710) as *const u32))
  }
  pub unsafe fn set_timsecu(&mut self, value: Timsecu) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x710) as *mut u32, value.0);
  }
  pub unsafe fn with_timsecu<F: FnOnce(Timsecu) -> Timsecu>(&mut self, f: F) {
     let tmp = self.timsecu();
     self.set_timsecu(f(tmp))
  }

  pub unsafe fn timnanou(&self) -> Timnanou { 
     Timnanou(::core::ptr::read_volatile(((self.0 as usize) + 0x714) as *const u32))
  }
  pub unsafe fn set_timnanou(&mut self, value: Timnanou) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x714) as *mut u32, value.0);
  }
  pub unsafe fn with_timnanou<F: FnOnce(Timnanou) -> Timnanou>(&mut self, f: F) {
     let tmp = self.timnanou();
     self.set_timnanou(f(tmp))
  }

  pub unsafe fn timadd(&self) -> Timadd { 
     Timadd(::core::ptr::read_volatile(((self.0 as usize) + 0x718) as *const u32))
  }
  pub unsafe fn set_timadd(&mut self, value: Timadd) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x718) as *mut u32, value.0);
  }
  pub unsafe fn with_timadd<F: FnOnce(Timadd) -> Timadd>(&mut self, f: F) {
     let tmp = self.timadd();
     self.set_timadd(f(tmp))
  }

  pub unsafe fn targsec(&self) -> Targsec { 
     Targsec(::core::ptr::read_volatile(((self.0 as usize) + 0x71c) as *const u32))
  }
  pub unsafe fn set_targsec(&mut self, value: Targsec) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x71c) as *mut u32, value.0);
  }
  pub unsafe fn with_targsec<F: FnOnce(Targsec) -> Targsec>(&mut self, f: F) {
     let tmp = self.targsec();
     self.set_targsec(f(tmp))
  }

  pub unsafe fn targnano(&self) -> Targnano { 
     Targnano(::core::ptr::read_volatile(((self.0 as usize) + 0x720) as *const u32))
  }
  pub unsafe fn set_targnano(&mut self, value: Targnano) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x720) as *mut u32, value.0);
  }
  pub unsafe fn with_targnano<F: FnOnce(Targnano) -> Targnano>(&mut self, f: F) {
     let tmp = self.targnano();
     self.set_targnano(f(tmp))
  }

  pub unsafe fn hwordsec(&self) -> Hwordsec { 
     Hwordsec(::core::ptr::read_volatile(((self.0 as usize) + 0x724) as *const u32))
  }
  pub unsafe fn set_hwordsec(&mut self, value: Hwordsec) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x724) as *mut u32, value.0);
  }
  pub unsafe fn with_hwordsec<F: FnOnce(Hwordsec) -> Hwordsec>(&mut self, f: F) {
     let tmp = self.hwordsec();
     self.set_hwordsec(f(tmp))
  }

  pub unsafe fn timstat(&self) -> Timstat { 
     Timstat(::core::ptr::read_volatile(((self.0 as usize) + 0x728) as *const u32))
  }
  pub unsafe fn set_timstat(&mut self, value: Timstat) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x728) as *mut u32, value.0);
  }
  pub unsafe fn with_timstat<F: FnOnce(Timstat) -> Timstat>(&mut self, f: F) {
     let tmp = self.timstat();
     self.set_timstat(f(tmp))
  }

  pub unsafe fn ppsctrl(&self) -> Ppsctrl { 
     Ppsctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x72c) as *const u32))
  }
  pub unsafe fn set_ppsctrl(&mut self, value: Ppsctrl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x72c) as *mut u32, value.0);
  }
  pub unsafe fn with_ppsctrl<F: FnOnce(Ppsctrl) -> Ppsctrl>(&mut self, f: F) {
     let tmp = self.ppsctrl();
     self.set_ppsctrl(f(tmp))
  }

  pub unsafe fn pps0intvl(&self) -> Pps0intvl { 
     Pps0intvl(::core::ptr::read_volatile(((self.0 as usize) + 0x760) as *const u32))
  }
  pub unsafe fn set_pps0intvl(&mut self, value: Pps0intvl) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x760) as *mut u32, value.0);
  }
  pub unsafe fn with_pps0intvl<F: FnOnce(Pps0intvl) -> Pps0intvl>(&mut self, f: F) {
     let tmp = self.pps0intvl();
     self.set_pps0intvl(f(tmp))
  }

  pub unsafe fn pps0width(&self) -> Pps0width { 
     Pps0width(::core::ptr::read_volatile(((self.0 as usize) + 0x764) as *const u32))
  }
  pub unsafe fn set_pps0width(&mut self, value: Pps0width) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x764) as *mut u32, value.0);
  }
  pub unsafe fn with_pps0width<F: FnOnce(Pps0width) -> Pps0width>(&mut self, f: F) {
     let tmp = self.pps0width();
     self.set_pps0width(f(tmp))
  }

  pub unsafe fn dmabusmod(&self) -> Dmabusmod { 
     Dmabusmod(::core::ptr::read_volatile(((self.0 as usize) + 0xc00) as *const u32))
  }
  pub unsafe fn set_dmabusmod(&mut self, value: Dmabusmod) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc00) as *mut u32, value.0);
  }
  pub unsafe fn with_dmabusmod<F: FnOnce(Dmabusmod) -> Dmabusmod>(&mut self, f: F) {
     let tmp = self.dmabusmod();
     self.set_dmabusmod(f(tmp))
  }

  pub unsafe fn set_txpolld(&mut self, value: Txpolld) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc04) as *mut u32, value.0);
  }

  pub unsafe fn set_rxpolld(&mut self, value: Rxpolld) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc08) as *mut u32, value.0);
  }

  pub unsafe fn rxdladdr(&self) -> Rxdladdr { 
     Rxdladdr(::core::ptr::read_volatile(((self.0 as usize) + 0xc0c) as *const u32))
  }
  pub unsafe fn set_rxdladdr(&mut self, value: Rxdladdr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc0c) as *mut u32, value.0);
  }
  pub unsafe fn with_rxdladdr<F: FnOnce(Rxdladdr) -> Rxdladdr>(&mut self, f: F) {
     let tmp = self.rxdladdr();
     self.set_rxdladdr(f(tmp))
  }

  pub unsafe fn txdladdr(&self) -> Txdladdr { 
     Txdladdr(::core::ptr::read_volatile(((self.0 as usize) + 0xc10) as *const u32))
  }
  pub unsafe fn set_txdladdr(&mut self, value: Txdladdr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc10) as *mut u32, value.0);
  }
  pub unsafe fn with_txdladdr<F: FnOnce(Txdladdr) -> Txdladdr>(&mut self, f: F) {
     let tmp = self.txdladdr();
     self.set_txdladdr(f(tmp))
  }

  pub unsafe fn dmaris(&self) -> Dmaris { 
     Dmaris(::core::ptr::read_volatile(((self.0 as usize) + 0xc14) as *const u32))
  }
  pub unsafe fn set_dmaris(&mut self, value: Dmaris) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc14) as *mut u32, value.0);
  }
  pub unsafe fn with_dmaris<F: FnOnce(Dmaris) -> Dmaris>(&mut self, f: F) {
     let tmp = self.dmaris();
     self.set_dmaris(f(tmp))
  }

  pub unsafe fn dmaopmode(&self) -> Dmaopmode { 
     Dmaopmode(::core::ptr::read_volatile(((self.0 as usize) + 0xc18) as *const u32))
  }
  pub unsafe fn set_dmaopmode(&mut self, value: Dmaopmode) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc18) as *mut u32, value.0);
  }
  pub unsafe fn with_dmaopmode<F: FnOnce(Dmaopmode) -> Dmaopmode>(&mut self, f: F) {
     let tmp = self.dmaopmode();
     self.set_dmaopmode(f(tmp))
  }

  pub unsafe fn dmaim(&self) -> Dmaim { 
     Dmaim(::core::ptr::read_volatile(((self.0 as usize) + 0xc1c) as *const u32))
  }
  pub unsafe fn set_dmaim(&mut self, value: Dmaim) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc1c) as *mut u32, value.0);
  }
  pub unsafe fn with_dmaim<F: FnOnce(Dmaim) -> Dmaim>(&mut self, f: F) {
     let tmp = self.dmaim();
     self.set_dmaim(f(tmp))
  }

  pub unsafe fn mfboc(&self) -> Mfboc { 
     Mfboc(::core::ptr::read_volatile(((self.0 as usize) + 0xc20) as *const u32))
  }
  pub unsafe fn set_mfboc(&mut self, value: Mfboc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc20) as *mut u32, value.0);
  }
  pub unsafe fn with_mfboc<F: FnOnce(Mfboc) -> Mfboc>(&mut self, f: F) {
     let tmp = self.mfboc();
     self.set_mfboc(f(tmp))
  }

  pub unsafe fn rxintwdt(&self) -> Rxintwdt { 
     Rxintwdt(::core::ptr::read_volatile(((self.0 as usize) + 0xc24) as *const u32))
  }
  pub unsafe fn set_rxintwdt(&mut self, value: Rxintwdt) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc24) as *mut u32, value.0);
  }
  pub unsafe fn with_rxintwdt<F: FnOnce(Rxintwdt) -> Rxintwdt>(&mut self, f: F) {
     let tmp = self.rxintwdt();
     self.set_rxintwdt(f(tmp))
  }

  pub unsafe fn hostxdesc(&self) -> Hostxdesc { 
     Hostxdesc(::core::ptr::read_volatile(((self.0 as usize) + 0xc48) as *const u32))
  }
  pub unsafe fn set_hostxdesc(&mut self, value: Hostxdesc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc48) as *mut u32, value.0);
  }
  pub unsafe fn with_hostxdesc<F: FnOnce(Hostxdesc) -> Hostxdesc>(&mut self, f: F) {
     let tmp = self.hostxdesc();
     self.set_hostxdesc(f(tmp))
  }

  pub unsafe fn hosrxdesc(&self) -> Hosrxdesc { 
     Hosrxdesc(::core::ptr::read_volatile(((self.0 as usize) + 0xc4c) as *const u32))
  }
  pub unsafe fn set_hosrxdesc(&mut self, value: Hosrxdesc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc4c) as *mut u32, value.0);
  }
  pub unsafe fn with_hosrxdesc<F: FnOnce(Hosrxdesc) -> Hosrxdesc>(&mut self, f: F) {
     let tmp = self.hosrxdesc();
     self.set_hosrxdesc(f(tmp))
  }

  pub unsafe fn hostxba(&self) -> Hostxba { 
     Hostxba(::core::ptr::read_volatile(((self.0 as usize) + 0xc50) as *const u32))
  }
  pub unsafe fn set_hostxba(&mut self, value: Hostxba) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc50) as *mut u32, value.0);
  }
  pub unsafe fn with_hostxba<F: FnOnce(Hostxba) -> Hostxba>(&mut self, f: F) {
     let tmp = self.hostxba();
     self.set_hostxba(f(tmp))
  }

  pub unsafe fn hosrxba(&self) -> Hosrxba { 
     Hosrxba(::core::ptr::read_volatile(((self.0 as usize) + 0xc54) as *const u32))
  }
  pub unsafe fn set_hosrxba(&mut self, value: Hosrxba) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc54) as *mut u32, value.0);
  }
  pub unsafe fn with_hosrxba<F: FnOnce(Hosrxba) -> Hosrxba>(&mut self, f: F) {
     let tmp = self.hosrxba();
     self.set_hosrxba(f(tmp))
  }

  pub unsafe fn pp(&self) -> Pp { 
     Pp(::core::ptr::read_volatile(((self.0 as usize) + 0xfc0) as *const u32))
  }
  pub unsafe fn set_pp(&mut self, value: Pp) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xfc0) as *mut u32, value.0);
  }
  pub unsafe fn with_pp<F: FnOnce(Pp) -> Pp>(&mut self, f: F) {
     let tmp = self.pp();
     self.set_pp(f(tmp))
  }

  pub unsafe fn pc(&self) -> Pc { 
     Pc(::core::ptr::read_volatile(((self.0 as usize) + 0xfc4) as *const u32))
  }
  pub unsafe fn set_pc(&mut self, value: Pc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xfc4) as *mut u32, value.0);
  }
  pub unsafe fn with_pc<F: FnOnce(Pc) -> Pc>(&mut self, f: F) {
     let tmp = self.pc();
     self.set_pc(f(tmp))
  }

  pub unsafe fn cc(&self) -> Cc { 
     Cc(::core::ptr::read_volatile(((self.0 as usize) + 0xfc8) as *const u32))
  }
  pub unsafe fn set_cc(&mut self, value: Cc) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0xfc8) as *mut u32, value.0);
  }
  pub unsafe fn with_cc<F: FnOnce(Cc) -> Cc>(&mut self, f: F) {
     let tmp = self.cc();
     self.set_cc(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cfg(pub u32);

impl Cfg {
  pub fn prelen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  pub fn set_prelen(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn re(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn te(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_te(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_dc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn bl(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  pub fn set_bl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn acs(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_acs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn dr(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_dr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn ipc(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_ipc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn dupm(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_dupm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn loopbm(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_loopbm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn dro(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_dro(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn fes(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_fes(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn ps(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_ps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn discrs(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_discrs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ifg(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7 // [19:17]
  }
  pub fn set_ifg(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn jfen(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_jfen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn jd(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_jd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn wddis(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_wddis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn cst(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_cst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn twokpen(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_twokpen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

}

impl ::core::fmt::Display for Cfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prelen() != 0 { try!(write!(f, " prelen=0x{:x}", self.prelen()))}
      if self.re() != 0 { try!(write!(f, " re"))}
      if self.te() != 0 { try!(write!(f, " te"))}
      if self.dc() != 0 { try!(write!(f, " dc"))}
      if self.bl() != 0 { try!(write!(f, " bl=0x{:x}", self.bl()))}
      if self.acs() != 0 { try!(write!(f, " acs"))}
      if self.dr() != 0 { try!(write!(f, " dr"))}
      if self.ipc() != 0 { try!(write!(f, " ipc"))}
      if self.dupm() != 0 { try!(write!(f, " dupm"))}
      if self.loopbm() != 0 { try!(write!(f, " loopbm"))}
      if self.dro() != 0 { try!(write!(f, " dro"))}
      if self.fes() != 0 { try!(write!(f, " fes"))}
      if self.ps() != 0 { try!(write!(f, " ps"))}
      if self.discrs() != 0 { try!(write!(f, " discrs"))}
      if self.ifg() != 0 { try!(write!(f, " ifg=0x{:x}", self.ifg()))}
      if self.jfen() != 0 { try!(write!(f, " jfen"))}
      if self.jd() != 0 { try!(write!(f, " jd"))}
      if self.wddis() != 0 { try!(write!(f, " wddis"))}
      if self.cst() != 0 { try!(write!(f, " cst"))}
      if self.twokpen() != 0 { try!(write!(f, " twokpen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Framefltr(pub u32);

impl Framefltr {
  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn huc(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_huc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn hmc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_hmc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn daif(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_daif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn dbf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_dbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pcf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  pub fn set_pcf(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn saif(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_saif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn saf(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_saf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn hpf(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_hpf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn vtfe(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_vtfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ra(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_ra(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Framefltr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Framefltr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pr() != 0 { try!(write!(f, " pr"))}
      if self.huc() != 0 { try!(write!(f, " huc"))}
      if self.hmc() != 0 { try!(write!(f, " hmc"))}
      if self.daif() != 0 { try!(write!(f, " daif"))}
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.dbf() != 0 { try!(write!(f, " dbf"))}
      if self.pcf() != 0 { try!(write!(f, " pcf=0x{:x}", self.pcf()))}
      if self.saif() != 0 { try!(write!(f, " saif"))}
      if self.saf() != 0 { try!(write!(f, " saf"))}
      if self.hpf() != 0 { try!(write!(f, " hpf"))}
      if self.vtfe() != 0 { try!(write!(f, " vtfe"))}
      if self.ra() != 0 { try!(write!(f, " ra"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hashtblh(pub u32);

impl Hashtblh {
  pub fn hth(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_hth(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Hashtblh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hashtblh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hashtbll(pub u32);

impl Hashtbll {
  pub fn htl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_htl(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Hashtbll {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hashtbll {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Miiaddr(pub u32);

impl Miiaddr {
  pub fn miib(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_miib(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn miiw(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_miiw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn cr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0xf // [5:2]
  }
  pub fn set_cr(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 2);
     self.0 |= value << 2;
     self
  }

  pub fn mii(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1f // [10:6]
  }
  pub fn set_mii(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 6);
     self.0 |= value << 6;
     self
  }

  pub fn pla(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1f // [15:11]
  }
  pub fn set_pla(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 11);
     self.0 |= value << 11;
     self
  }

}

impl ::core::fmt::Display for Miiaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Miiaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.miib() != 0 { try!(write!(f, " miib"))}
      if self.miiw() != 0 { try!(write!(f, " miiw"))}
      if self.cr() != 0 { try!(write!(f, " cr=0x{:x}", self.cr()))}
      if self.mii() != 0 { try!(write!(f, " mii=0x{:x}", self.mii()))}
      if self.pla() != 0 { try!(write!(f, " pla=0x{:x}", self.pla()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Miidata(pub u32);

impl Miidata {
  pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_data(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Miidata {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Miidata {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Flowctl(pub u32);

impl Flowctl {
  pub fn fcbbpa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_fcbbpa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tfe(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn rfe(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_rfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn up(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_up(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn plt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_plt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn dzqp(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_dzqp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn pt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  pub fn set_pt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Flowctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Flowctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fcbbpa() != 0 { try!(write!(f, " fcbbpa"))}
      if self.tfe() != 0 { try!(write!(f, " tfe"))}
      if self.rfe() != 0 { try!(write!(f, " rfe"))}
      if self.up() != 0 { try!(write!(f, " up"))}
      if self.plt() != 0 { try!(write!(f, " plt=0x{:x}", self.plt()))}
      if self.dzqp() != 0 { try!(write!(f, " dzqp"))}
      if self.pt() != 0 { try!(write!(f, " pt=0x{:x}", self.pt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Vlantg(pub u32);

impl Vlantg {
  pub fn vl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_vl(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn etv(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_etv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn vtim(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_vtim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn esvl(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_esvl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn vthm(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_vthm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

}

impl ::core::fmt::Display for Vlantg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Vlantg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vl() != 0 { try!(write!(f, " vl=0x{:x}", self.vl()))}
      if self.etv() != 0 { try!(write!(f, " etv"))}
      if self.vtim() != 0 { try!(write!(f, " vtim"))}
      if self.esvl() != 0 { try!(write!(f, " esvl"))}
      if self.vthm() != 0 { try!(write!(f, " vthm"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Status(pub u32);

impl Status {
  pub fn rpe(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_rpe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn rfcfc(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3 // [2:1]
  }
  pub fn set_rfcfc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn rwc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_rwc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn rrc(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  pub fn set_rrc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn rxf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_rxf(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn tpe(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_tpe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn tfc(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x3 // [18:17]
  }
  pub fn set_tfc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn txpaused(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_txpaused(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  pub fn trc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
  pub fn set_trc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn twc(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_twc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn txfe(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_txfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn txff(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_txff(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

}

impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rpe() != 0 { try!(write!(f, " rpe"))}
      if self.rfcfc() != 0 { try!(write!(f, " rfcfc=0x{:x}", self.rfcfc()))}
      if self.rwc() != 0 { try!(write!(f, " rwc"))}
      if self.rrc() != 0 { try!(write!(f, " rrc=0x{:x}", self.rrc()))}
      if self.rxf() != 0 { try!(write!(f, " rxf=0x{:x}", self.rxf()))}
      if self.tpe() != 0 { try!(write!(f, " tpe"))}
      if self.tfc() != 0 { try!(write!(f, " tfc=0x{:x}", self.tfc()))}
      if self.txpaused() != 0 { try!(write!(f, " txpaused"))}
      if self.trc() != 0 { try!(write!(f, " trc=0x{:x}", self.trc()))}
      if self.twc() != 0 { try!(write!(f, " twc"))}
      if self.txfe() != 0 { try!(write!(f, " txfe"))}
      if self.txff() != 0 { try!(write!(f, " txff"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rwuff(pub u32);

impl Rwuff {
  pub fn wakeupfil(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_wakeupfil(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rwuff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rwuff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pmtctlstat(pub u32);

impl Pmtctlstat {
  pub fn pwrdwn(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_pwrdwn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn mgkpkten(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_mgkpkten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn wupfren(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_wupfren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn mgkprx(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_mgkprx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn wuprx(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_wuprx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn glblucast(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_glblucast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn rwkptr(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  pub fn set_rwkptr(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn wupfrrst(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_wupfrrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Pmtctlstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pmtctlstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwrdwn() != 0 { try!(write!(f, " pwrdwn"))}
      if self.mgkpkten() != 0 { try!(write!(f, " mgkpkten"))}
      if self.wupfren() != 0 { try!(write!(f, " wupfren"))}
      if self.mgkprx() != 0 { try!(write!(f, " mgkprx"))}
      if self.wuprx() != 0 { try!(write!(f, " wuprx"))}
      if self.glblucast() != 0 { try!(write!(f, " glblucast"))}
      if self.rwkptr() != 0 { try!(write!(f, " rwkptr=0x{:x}", self.rwkptr()))}
      if self.wupfrrst() != 0 { try!(write!(f, " wupfrrst"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ris(pub u32);

impl Ris {
  pub fn pmt(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_pmt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn mmc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_mmc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn mmcrx(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_mmcrx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn mmctx(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_mmctx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ts(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_ts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
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
      if self.pmt() != 0 { try!(write!(f, " pmt"))}
      if self.mmc() != 0 { try!(write!(f, " mmc"))}
      if self.mmcrx() != 0 { try!(write!(f, " mmcrx"))}
      if self.mmctx() != 0 { try!(write!(f, " mmctx"))}
      if self.ts() != 0 { try!(write!(f, " ts"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Im(pub u32);

impl Im {
  pub fn pmt(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_pmt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn tsi(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_tsi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
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
      if self.pmt() != 0 { try!(write!(f, " pmt"))}
      if self.tsi() != 0 { try!(write!(f, " tsi"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Addr0h(pub u32);

impl Addr0h {
  pub fn addrhi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_addrhi(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_ae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Addr0h {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Addr0h {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addrhi() != 0 { try!(write!(f, " addrhi=0x{:x}", self.addrhi()))}
      if self.ae() != 0 { try!(write!(f, " ae"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Addr0l(pub u32);

impl Addr0l {
  pub fn addrlo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_addrlo(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Addr0l {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Addr0l {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Addr1h(pub u32);

impl Addr1h {
  pub fn addrhi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_addrhi(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn mbc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3f // [29:24]
  }
  pub fn set_mbc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 24);
     self.0 |= value << 24;
     self
  }

  pub fn sa(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_sa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_ae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Addr1h {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Addr1h {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addrhi() != 0 { try!(write!(f, " addrhi=0x{:x}", self.addrhi()))}
      if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
      if self.sa() != 0 { try!(write!(f, " sa"))}
      if self.ae() != 0 { try!(write!(f, " ae"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Addr1l(pub u32);

impl Addr1l {
  pub fn addrlo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_addrlo(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Addr1l {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Addr1l {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Addr2h(pub u32);

impl Addr2h {
  pub fn addrhi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_addrhi(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn mbc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3f // [29:24]
  }
  pub fn set_mbc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 24);
     self.0 |= value << 24;
     self
  }

  pub fn sa(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_sa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_ae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Addr2h {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Addr2h {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addrhi() != 0 { try!(write!(f, " addrhi=0x{:x}", self.addrhi()))}
      if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
      if self.sa() != 0 { try!(write!(f, " sa"))}
      if self.ae() != 0 { try!(write!(f, " ae"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Addr2l(pub u32);

impl Addr2l {
  pub fn addrlo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_addrlo(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Addr2l {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Addr2l {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Addr3h(pub u32);

impl Addr3h {
  pub fn addrhi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_addrhi(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn mbc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3f // [29:24]
  }
  pub fn set_mbc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 24);
     self.0 |= value << 24;
     self
  }

  pub fn sa(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  pub fn set_sa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_ae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Addr3h {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Addr3h {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addrhi() != 0 { try!(write!(f, " addrhi=0x{:x}", self.addrhi()))}
      if self.mbc() != 0 { try!(write!(f, " mbc=0x{:x}", self.mbc()))}
      if self.sa() != 0 { try!(write!(f, " sa"))}
      if self.ae() != 0 { try!(write!(f, " ae"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Addr3l(pub u32);

impl Addr3l {
  pub fn addrlo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_addrlo(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Addr3l {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Addr3l {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Wdogto(pub u32);

impl Wdogto {
  pub fn wto(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3fff // [13:0]
  }
  pub fn set_wto(mut self, value: u32) -> Self {
     assert!((value & !0x3fff) == 0);
     self.0 &= !(0x3fff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn pwe(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_pwe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Wdogto {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Wdogto {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wto() != 0 { try!(write!(f, " wto=0x{:x}", self.wto()))}
      if self.pwe() != 0 { try!(write!(f, " pwe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mmcctrl(pub u32);

impl Mmcctrl {
  pub fn cntrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_cntrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn cntstpro(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_cntstpro(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn rstonrd(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_rstonrd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn cntfreez(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_cntfreez(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn cntprst(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_cntprst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cntprstlvl(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_cntprstlvl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn ucdbc(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_ucdbc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Mmcctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mmcctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cntrst() != 0 { try!(write!(f, " cntrst"))}
      if self.cntstpro() != 0 { try!(write!(f, " cntstpro"))}
      if self.rstonrd() != 0 { try!(write!(f, " rstonrd"))}
      if self.cntfreez() != 0 { try!(write!(f, " cntfreez"))}
      if self.cntprst() != 0 { try!(write!(f, " cntprst"))}
      if self.cntprstlvl() != 0 { try!(write!(f, " cntprstlvl"))}
      if self.ucdbc() != 0 { try!(write!(f, " ucdbc"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mmcrxris(pub u32);

impl Mmcrxris {
  pub fn gbf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_gbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn crcerr(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_crcerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn algnerr(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_algnerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ucgf(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_ucgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

}

impl ::core::fmt::Display for Mmcrxris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mmcrxris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gbf() != 0 { try!(write!(f, " gbf"))}
      if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
      if self.algnerr() != 0 { try!(write!(f, " algnerr"))}
      if self.ucgf() != 0 { try!(write!(f, " ucgf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mmctxris(pub u32);

impl Mmctxris {
  pub fn gbf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_gbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn scollgf(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_scollgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn mcollgf(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_mcollgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn octcnt(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_octcnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

}

impl ::core::fmt::Display for Mmctxris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mmctxris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gbf() != 0 { try!(write!(f, " gbf"))}
      if self.scollgf() != 0 { try!(write!(f, " scollgf"))}
      if self.mcollgf() != 0 { try!(write!(f, " mcollgf"))}
      if self.octcnt() != 0 { try!(write!(f, " octcnt"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mmcrxim(pub u32);

impl Mmcrxim {
  pub fn gbf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_gbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn crcerr(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_crcerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn algnerr(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_algnerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ucgf(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_ucgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

}

impl ::core::fmt::Display for Mmcrxim {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mmcrxim {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gbf() != 0 { try!(write!(f, " gbf"))}
      if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
      if self.algnerr() != 0 { try!(write!(f, " algnerr"))}
      if self.ucgf() != 0 { try!(write!(f, " ucgf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mmctxim(pub u32);

impl Mmctxim {
  pub fn gbf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_gbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn scollgf(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_scollgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn mcollgf(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_mcollgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn octcnt(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_octcnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

}

impl ::core::fmt::Display for Mmctxim {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mmctxim {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gbf() != 0 { try!(write!(f, " gbf"))}
      if self.scollgf() != 0 { try!(write!(f, " scollgf"))}
      if self.mcollgf() != 0 { try!(write!(f, " mcollgf"))}
      if self.octcnt() != 0 { try!(write!(f, " octcnt"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Txcntgb(pub u32);

impl Txcntgb {
  pub fn txfrmgb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_txfrmgb(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Txcntgb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Txcntgb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Txcntscol(pub u32);

impl Txcntscol {
  pub fn txsnglcolg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_txsnglcolg(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Txcntscol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Txcntscol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Txcntmcol(pub u32);

impl Txcntmcol {
  pub fn txmultcolg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_txmultcolg(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Txcntmcol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Txcntmcol {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Txoctcntg(pub u32);

impl Txoctcntg {
  pub fn txoctg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_txoctg(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Txoctcntg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Txoctcntg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxcntgb(pub u32);

impl Rxcntgb {
  pub fn rxfrmgb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_rxfrmgb(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rxcntgb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxcntgb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxcntcrcerr(pub u32);

impl Rxcntcrcerr {
  pub fn rxcrcerr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_rxcrcerr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rxcntcrcerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxcntcrcerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxcntalgnerr(pub u32);

impl Rxcntalgnerr {
  pub fn rxalgnerr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_rxalgnerr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rxcntalgnerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxcntalgnerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxcntguni(pub u32);

impl Rxcntguni {
  pub fn rxucastg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_rxucastg(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rxcntguni {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxcntguni {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Vlnincrep(pub u32);

impl Vlnincrep {
  pub fn vlt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_vlt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn vlc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_vlc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn vlp(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_vlp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  pub fn csvl(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  pub fn set_csvl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

}

impl ::core::fmt::Display for Vlnincrep {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Vlnincrep {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vlt() != 0 { try!(write!(f, " vlt=0x{:x}", self.vlt()))}
      if self.vlc() != 0 { try!(write!(f, " vlc=0x{:x}", self.vlc()))}
      if self.vlp() != 0 { try!(write!(f, " vlp"))}
      if self.csvl() != 0 { try!(write!(f, " csvl"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Vlanhash(pub u32);

impl Vlanhash {
  pub fn vlht(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_vlht(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Vlanhash {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Vlanhash {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vlht() != 0 { try!(write!(f, " vlht=0x{:x}", self.vlht()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Timstctrl(pub u32);

impl Timstctrl {
  pub fn tsen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tsen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tsfcupdt(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tsfcupdt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn tsinit(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tsinit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tsupdt(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tsupdt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn inttrig(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_inttrig(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn addregup(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_addregup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn allf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_allf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dgtlbin(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_dgtlbin(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn ptpver2(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_ptpver2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn ptpeth(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_ptpeth(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn ptpipv6(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_ptpipv6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn ptpipv4(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_ptpipv4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn tsevnt(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_tsevnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn tsmast(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_tsmast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn selptp(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  pub fn set_selptp(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ptpfltr(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_ptpfltr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}

impl ::core::fmt::Display for Timstctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Timstctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tsen() != 0 { try!(write!(f, " tsen"))}
      if self.tsfcupdt() != 0 { try!(write!(f, " tsfcupdt"))}
      if self.tsinit() != 0 { try!(write!(f, " tsinit"))}
      if self.tsupdt() != 0 { try!(write!(f, " tsupdt"))}
      if self.inttrig() != 0 { try!(write!(f, " inttrig"))}
      if self.addregup() != 0 { try!(write!(f, " addregup"))}
      if self.allf() != 0 { try!(write!(f, " allf"))}
      if self.dgtlbin() != 0 { try!(write!(f, " dgtlbin"))}
      if self.ptpver2() != 0 { try!(write!(f, " ptpver2"))}
      if self.ptpeth() != 0 { try!(write!(f, " ptpeth"))}
      if self.ptpipv6() != 0 { try!(write!(f, " ptpipv6"))}
      if self.ptpipv4() != 0 { try!(write!(f, " ptpipv4"))}
      if self.tsevnt() != 0 { try!(write!(f, " tsevnt"))}
      if self.tsmast() != 0 { try!(write!(f, " tsmast"))}
      if self.selptp() != 0 { try!(write!(f, " selptp=0x{:x}", self.selptp()))}
      if self.ptpfltr() != 0 { try!(write!(f, " ptpfltr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Subsecinc(pub u32);

impl Subsecinc {
  pub fn ssinc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_ssinc(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Subsecinc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Subsecinc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ssinc() != 0 { try!(write!(f, " ssinc=0x{:x}", self.ssinc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Timsec(pub u32);

impl Timsec {
  pub fn tss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_tss(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Timsec {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Timsec {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Timnano(pub u32);

impl Timnano {
  pub fn tsss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
  pub fn set_tsss(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Timnano {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Timnano {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tsss() != 0 { try!(write!(f, " tsss=0x{:x}", self.tsss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Timsecu(pub u32);

impl Timsecu {
  pub fn tss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_tss(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Timsecu {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Timsecu {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Timnanou(pub u32);

impl Timnanou {
  pub fn tsss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
  pub fn set_tsss(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn addsub(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_addsub(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Timnanou {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Timnanou {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tsss() != 0 { try!(write!(f, " tsss=0x{:x}", self.tsss()))}
      if self.addsub() != 0 { try!(write!(f, " addsub"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Timadd(pub u32);

impl Timadd {
  pub fn tsar(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_tsar(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Timadd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Timadd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Targsec(pub u32);

impl Targsec {
  pub fn tstr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_tstr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Targsec {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Targsec {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Targnano(pub u32);

impl Targnano {
  pub fn ttslo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
  pub fn set_ttslo(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn trgtbusy(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_trgtbusy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Targnano {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Targnano {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ttslo() != 0 { try!(write!(f, " ttslo=0x{:x}", self.ttslo()))}
      if self.trgtbusy() != 0 { try!(write!(f, " trgtbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hwordsec(pub u32);

impl Hwordsec {
  pub fn tshwr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_tshwr(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Hwordsec {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hwordsec {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tshwr() != 0 { try!(write!(f, " tshwr=0x{:x}", self.tshwr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Timstat(pub u32);

impl Timstat {
  pub fn tssovf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tssovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tstargt(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tstargt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}

impl ::core::fmt::Display for Timstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Timstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tssovf() != 0 { try!(write!(f, " tssovf"))}
      if self.tstargt() != 0 { try!(write!(f, " tstargt"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ppsctrl(pub u32);

impl Ppsctrl {
  pub fn ppsctrl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  pub fn set_ppsctrl(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  pub fn ppsen0(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_ppsen0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn trgmods0(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  pub fn set_trgmods0(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

}

impl ::core::fmt::Display for Ppsctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ppsctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ppsctrl() != 0 { try!(write!(f, " ppsctrl=0x{:x}", self.ppsctrl()))}
      if self.ppsen0() != 0 { try!(write!(f, " ppsen0"))}
      if self.trgmods0() != 0 { try!(write!(f, " trgmods0=0x{:x}", self.trgmods0()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pps0intvl(pub u32);

impl Pps0intvl {
  pub fn pps0int(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_pps0int(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Pps0intvl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pps0intvl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pps0width(pub u32);

impl Pps0width {
  pub fn emac_pps0width(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_emac_pps0width(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Pps0width {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pps0width {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dmabusmod(pub u32);

impl Dmabusmod {
  pub fn swr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_swr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn da(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_da(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn dsl(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1f // [6:2]
  }
  pub fn set_dsl(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 2);
     self.0 |= value << 2;
     self
  }

  pub fn atds(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_atds(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn pbl(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3f // [13:8]
  }
  pub fn set_pbl(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 8);
     self.0 |= value << 8;
     self
  }

  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
  pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn fb(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_fb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rpbl(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x3f // [22:17]
  }
  pub fn set_rpbl(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 17);
     self.0 |= value << 17;
     self
  }

  pub fn usp(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_usp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn _8xpbl(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_8xpbl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn aal(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_aal(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn mb(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_mb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  pub fn txpr(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_txpr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn rib(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_rib(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Dmabusmod {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dmabusmod {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swr() != 0 { try!(write!(f, " swr"))}
      if self.da() != 0 { try!(write!(f, " da"))}
      if self.dsl() != 0 { try!(write!(f, " dsl=0x{:x}", self.dsl()))}
      if self.atds() != 0 { try!(write!(f, " atds"))}
      if self.pbl() != 0 { try!(write!(f, " pbl=0x{:x}", self.pbl()))}
      if self.pr() != 0 { try!(write!(f, " pr=0x{:x}", self.pr()))}
      if self.fb() != 0 { try!(write!(f, " fb"))}
      if self.rpbl() != 0 { try!(write!(f, " rpbl=0x{:x}", self.rpbl()))}
      if self.usp() != 0 { try!(write!(f, " usp"))}
      if self._8xpbl() != 0 { try!(write!(f, " _8xpbl"))}
      if self.aal() != 0 { try!(write!(f, " aal"))}
      if self.mb() != 0 { try!(write!(f, " mb"))}
      if self.txpr() != 0 { try!(write!(f, " txpr"))}
      if self.rib() != 0 { try!(write!(f, " rib"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Txpolld(pub u32);

impl Txpolld {
  pub fn tpd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_tpd(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Txpolld {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Txpolld {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxpolld(pub u32);

impl Rxpolld {
  pub fn rpd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_rpd(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rxpolld {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxpolld {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxdladdr(pub u32);

impl Rxdladdr {
  pub fn strxlist(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3fffffff // [31:2]
  }
  pub fn set_strxlist(mut self, value: u32) -> Self {
     assert!((value & !0x3fffffff) == 0);
     self.0 &= !(0x3fffffff << 2);
     self.0 |= value << 2;
     self
  }

}

impl ::core::fmt::Display for Rxdladdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxdladdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.strxlist() != 0 { try!(write!(f, " strxlist=0x{:x}", self.strxlist()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Txdladdr(pub u32);

impl Txdladdr {
  pub fn txdladdr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3fffffff // [31:2]
  }
  pub fn set_txdladdr(mut self, value: u32) -> Self {
     assert!((value & !0x3fffffff) == 0);
     self.0 &= !(0x3fffffff << 2);
     self.0 |= value << 2;
     self
  }

}

impl ::core::fmt::Display for Txdladdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Txdladdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.txdladdr() != 0 { try!(write!(f, " txdladdr=0x{:x}", self.txdladdr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dmaris(pub u32);

impl Dmaris {
  pub fn ti(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_ti(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tps(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn tu(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tjt(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tjt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn ovf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_ovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn unf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_unf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn ri(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_ri(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ru(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_ru(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn rps(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_rps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn rwt(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_rwt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn eti(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_eti(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn fbi(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_fbi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn eri(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_eri(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn ais(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_ais(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn nis(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_nis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn rs(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7 // [19:17]
  }
  pub fn set_rs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn ts(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x7 // [22:20]
  }
  pub fn set_ts(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x7 // [25:23]
  }
  pub fn set_ae(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn mmc(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  pub fn set_mmc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  pub fn pmt(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_pmt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn tt(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  pub fn set_tt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 29);
     self.0 |= value << 29;
     self
  }

}

impl ::core::fmt::Display for Dmaris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dmaris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ti() != 0 { try!(write!(f, " ti"))}
      if self.tps() != 0 { try!(write!(f, " tps"))}
      if self.tu() != 0 { try!(write!(f, " tu"))}
      if self.tjt() != 0 { try!(write!(f, " tjt"))}
      if self.ovf() != 0 { try!(write!(f, " ovf"))}
      if self.unf() != 0 { try!(write!(f, " unf"))}
      if self.ri() != 0 { try!(write!(f, " ri"))}
      if self.ru() != 0 { try!(write!(f, " ru"))}
      if self.rps() != 0 { try!(write!(f, " rps"))}
      if self.rwt() != 0 { try!(write!(f, " rwt"))}
      if self.eti() != 0 { try!(write!(f, " eti"))}
      if self.fbi() != 0 { try!(write!(f, " fbi"))}
      if self.eri() != 0 { try!(write!(f, " eri"))}
      if self.ais() != 0 { try!(write!(f, " ais"))}
      if self.nis() != 0 { try!(write!(f, " nis"))}
      if self.rs() != 0 { try!(write!(f, " rs=0x{:x}", self.rs()))}
      if self.ts() != 0 { try!(write!(f, " ts=0x{:x}", self.ts()))}
      if self.ae() != 0 { try!(write!(f, " ae=0x{:x}", self.ae()))}
      if self.mmc() != 0 { try!(write!(f, " mmc"))}
      if self.pmt() != 0 { try!(write!(f, " pmt"))}
      if self.tt() != 0 { try!(write!(f, " tt"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dmaopmode(pub u32);

impl Dmaopmode {
  pub fn sr(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_sr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn osf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_osf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn rtc(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
  pub fn set_rtc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dgf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_dgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn fuf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_fuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn fef(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_fef(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn ttc(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x7 // [16:14]
  }
  pub fn set_ttc(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn ftf(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_ftf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn tsf(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_tsf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn dff(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_dff(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn rsf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_rsf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

}

impl ::core::fmt::Display for Dmaopmode {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dmaopmode {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sr() != 0 { try!(write!(f, " sr"))}
      if self.osf() != 0 { try!(write!(f, " osf"))}
      if self.rtc() != 0 { try!(write!(f, " rtc=0x{:x}", self.rtc()))}
      if self.dgf() != 0 { try!(write!(f, " dgf"))}
      if self.fuf() != 0 { try!(write!(f, " fuf"))}
      if self.fef() != 0 { try!(write!(f, " fef"))}
      if self.st() != 0 { try!(write!(f, " st"))}
      if self.ttc() != 0 { try!(write!(f, " ttc=0x{:x}", self.ttc()))}
      if self.ftf() != 0 { try!(write!(f, " ftf"))}
      if self.tsf() != 0 { try!(write!(f, " tsf"))}
      if self.dff() != 0 { try!(write!(f, " dff"))}
      if self.rsf() != 0 { try!(write!(f, " rsf"))}
      if self.dt() != 0 { try!(write!(f, " dt"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Dmaim(pub u32);

impl Dmaim {
  pub fn tie(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_tie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tse(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn tue(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_tue(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn tje(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_tje(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn ove(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_ove(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn une(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_une(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn rie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_rie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn rue(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_rue(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn rse(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_rse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn rwe(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_rwe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn ete(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_ete(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn fbe(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_fbe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn ere(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_ere(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn aie(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_aie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn nie(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_nie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}

impl ::core::fmt::Display for Dmaim {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Dmaim {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.tse() != 0 { try!(write!(f, " tse"))}
      if self.tue() != 0 { try!(write!(f, " tue"))}
      if self.tje() != 0 { try!(write!(f, " tje"))}
      if self.ove() != 0 { try!(write!(f, " ove"))}
      if self.une() != 0 { try!(write!(f, " une"))}
      if self.rie() != 0 { try!(write!(f, " rie"))}
      if self.rue() != 0 { try!(write!(f, " rue"))}
      if self.rse() != 0 { try!(write!(f, " rse"))}
      if self.rwe() != 0 { try!(write!(f, " rwe"))}
      if self.ete() != 0 { try!(write!(f, " ete"))}
      if self.fbe() != 0 { try!(write!(f, " fbe"))}
      if self.ere() != 0 { try!(write!(f, " ere"))}
      if self.aie() != 0 { try!(write!(f, " aie"))}
      if self.nie() != 0 { try!(write!(f, " nie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Mfboc(pub u32);

impl Mfboc {
  pub fn misfrmcnt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_misfrmcnt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn miscntovf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_miscntovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  pub fn ovffrmcnt(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7ff // [27:17]
  }
  pub fn set_ovffrmcnt(mut self, value: u32) -> Self {
     assert!((value & !0x7ff) == 0);
     self.0 &= !(0x7ff << 17);
     self.0 |= value << 17;
     self
  }

  pub fn ovfcntovf(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  pub fn set_ovfcntovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

}

impl ::core::fmt::Display for Mfboc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Mfboc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.misfrmcnt() != 0 { try!(write!(f, " misfrmcnt=0x{:x}", self.misfrmcnt()))}
      if self.miscntovf() != 0 { try!(write!(f, " miscntovf"))}
      if self.ovffrmcnt() != 0 { try!(write!(f, " ovffrmcnt=0x{:x}", self.ovffrmcnt()))}
      if self.ovfcntovf() != 0 { try!(write!(f, " ovfcntovf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Rxintwdt(pub u32);

impl Rxintwdt {
  pub fn riwt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_riwt(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Rxintwdt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Rxintwdt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.riwt() != 0 { try!(write!(f, " riwt=0x{:x}", self.riwt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hostxdesc(pub u32);

impl Hostxdesc {
  pub fn curtxdesc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_curtxdesc(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Hostxdesc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hostxdesc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hosrxdesc(pub u32);

impl Hosrxdesc {
  pub fn currxdesc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_currxdesc(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Hosrxdesc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hosrxdesc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hostxba(pub u32);

impl Hostxba {
  pub fn curtxbufa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_curtxbufa(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Hostxba {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hostxba {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Hosrxba(pub u32);

impl Hosrxba {
  pub fn currxbufa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_currxbufa(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Hosrxba {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Hosrxba {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pp(pub u32);

impl Pp {
  pub fn phytype(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_phytype(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn mactype(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  pub fn set_mactype(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

}

impl ::core::fmt::Display for Pp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.phytype() != 0 { try!(write!(f, " phytype=0x{:x}", self.phytype()))}
      if self.mactype() != 0 { try!(write!(f, " mactype=0x{:x}", self.mactype()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Pc(pub u32);

impl Pc {
  pub fn phyhold(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_phyhold(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn anmode(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3 // [2:1]
  }
  pub fn set_anmode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn anen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_anen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn fastansel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  pub fn set_fastansel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn fastanen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_fastanen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn extfd(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_extfd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn fastlupd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_fastlupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn fastrxdv(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_fastrxdv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn mdixen(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_mdixen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn fastmdix(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_fastmdix(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn rbstmdix(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_rbstmdix(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mdiswap(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_mdiswap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn polswap(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_polswap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn fastldmode(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1f // [19:15]
  }
  pub fn set_fastldmode(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 15);
     self.0 |= value << 15;
     self
  }

  pub fn tdrrun(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  pub fn set_tdrrun(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  pub fn lrr(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  pub fn set_lrr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  pub fn isomiill(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  pub fn set_isomiill(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  pub fn rxeridle(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  pub fn set_rxeridle(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  pub fn nibdetdis(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  pub fn set_nibdetdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  pub fn digrestart(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  pub fn set_digrestart(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  pub fn pintfs(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x7 // [30:28]
  }
  pub fn set_pintfs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 28);
     self.0 |= value << 28;
     self
  }

  pub fn phyext(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_phyext(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}

impl ::core::fmt::Display for Pc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Pc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.phyhold() != 0 { try!(write!(f, " phyhold"))}
      if self.anmode() != 0 { try!(write!(f, " anmode=0x{:x}", self.anmode()))}
      if self.anen() != 0 { try!(write!(f, " anen"))}
      if self.fastansel() != 0 { try!(write!(f, " fastansel=0x{:x}", self.fastansel()))}
      if self.fastanen() != 0 { try!(write!(f, " fastanen"))}
      if self.extfd() != 0 { try!(write!(f, " extfd"))}
      if self.fastlupd() != 0 { try!(write!(f, " fastlupd"))}
      if self.fastrxdv() != 0 { try!(write!(f, " fastrxdv"))}
      if self.mdixen() != 0 { try!(write!(f, " mdixen"))}
      if self.fastmdix() != 0 { try!(write!(f, " fastmdix"))}
      if self.rbstmdix() != 0 { try!(write!(f, " rbstmdix"))}
      if self.mdiswap() != 0 { try!(write!(f, " mdiswap"))}
      if self.polswap() != 0 { try!(write!(f, " polswap"))}
      if self.fastldmode() != 0 { try!(write!(f, " fastldmode=0x{:x}", self.fastldmode()))}
      if self.tdrrun() != 0 { try!(write!(f, " tdrrun"))}
      if self.lrr() != 0 { try!(write!(f, " lrr"))}
      if self.isomiill() != 0 { try!(write!(f, " isomiill"))}
      if self.rxeridle() != 0 { try!(write!(f, " rxeridle"))}
      if self.nibdetdis() != 0 { try!(write!(f, " nibdetdis"))}
      if self.digrestart() != 0 { try!(write!(f, " digrestart"))}
      if self.pintfs() != 0 { try!(write!(f, " pintfs=0x{:x}", self.pintfs()))}
      if self.phyext() != 0 { try!(write!(f, " phyext"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cc(pub u32);

impl Cc {
  pub fn pol(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  pub fn set_pol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  pub fn ptpcen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  pub fn set_ptpcen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}

impl ::core::fmt::Display for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pol() != 0 { try!(write!(f, " pol"))}
      if self.ptpcen() != 0 { try!(write!(f, " ptpcen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

