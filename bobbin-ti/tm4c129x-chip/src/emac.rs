pub const EMAC0: Emac = Emac(0x400ec000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Emac(pub u32);
impl Emac {
  #[inline]
  pub fn cfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline]
  pub fn cfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline]
  pub fn cfg(&self) -> Cfg { 
     unsafe {
       Cfg(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline]
  pub fn set_cfg(&self, value: Cfg) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Emac {
     let tmp = self.cfg();
     self.set_cfg(f(tmp))
  }

  #[inline]
  pub fn framefltr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline]
  pub fn framefltr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline]
  pub fn framefltr(&self) -> Framefltr { 
     unsafe {
       Framefltr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  #[inline]
  pub fn set_framefltr(&self, value: Framefltr) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_framefltr<F: FnOnce(Framefltr) -> Framefltr>(&self, f: F) -> &Emac {
     let tmp = self.framefltr();
     self.set_framefltr(f(tmp))
  }

  #[inline]
  pub fn hashtblh_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
  #[inline]
  pub fn hashtblh_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
  #[inline]
  pub fn hashtblh(&self) -> Hashtblh { 
     unsafe {
       Hashtblh(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  #[inline]
  pub fn set_hashtblh(&self, value: Hashtblh) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_hashtblh<F: FnOnce(Hashtblh) -> Hashtblh>(&self, f: F) -> &Emac {
     let tmp = self.hashtblh();
     self.set_hashtblh(f(tmp))
  }

  #[inline]
  pub fn hashtbll_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline]
  pub fn hashtbll_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline]
  pub fn hashtbll(&self) -> Hashtbll { 
     unsafe {
       Hashtbll(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline]
  pub fn set_hashtbll(&self, value: Hashtbll) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_hashtbll<F: FnOnce(Hashtbll) -> Hashtbll>(&self, f: F) -> &Emac {
     let tmp = self.hashtbll();
     self.set_hashtbll(f(tmp))
  }

  #[inline]
  pub fn miiaddr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
  #[inline]
  pub fn miiaddr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
  #[inline]
  pub fn miiaddr(&self) -> Miiaddr { 
     unsafe {
       Miiaddr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  #[inline]
  pub fn set_miiaddr(&self, value: Miiaddr) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_miiaddr<F: FnOnce(Miiaddr) -> Miiaddr>(&self, f: F) -> &Emac {
     let tmp = self.miiaddr();
     self.set_miiaddr(f(tmp))
  }

  #[inline]
  pub fn miidata_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline]
  pub fn miidata_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline]
  pub fn miidata(&self) -> Miidata { 
     unsafe {
       Miidata(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline]
  pub fn set_miidata(&self, value: Miidata) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_miidata<F: FnOnce(Miidata) -> Miidata>(&self, f: F) -> &Emac {
     let tmp = self.miidata();
     self.set_miidata(f(tmp))
  }

  #[inline]
  pub fn flowctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
  #[inline]
  pub fn flowctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
  #[inline]
  pub fn flowctl(&self) -> Flowctl { 
     unsafe {
       Flowctl(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
  #[inline]
  pub fn set_flowctl(&self, value: Flowctl) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_flowctl<F: FnOnce(Flowctl) -> Flowctl>(&self, f: F) -> &Emac {
     let tmp = self.flowctl();
     self.set_flowctl(f(tmp))
  }

  #[inline]
  pub fn vlantg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
  #[inline]
  pub fn vlantg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
  #[inline]
  pub fn vlantg(&self) -> Vlantg { 
     unsafe {
       Vlantg(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
  #[inline]
  pub fn set_vlantg(&self, value: Vlantg) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_vlantg<F: FnOnce(Vlantg) -> Vlantg>(&self, f: F) -> &Emac {
     let tmp = self.vlantg();
     self.set_vlantg(f(tmp))
  }

  #[inline]
  pub fn status_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline]
  pub fn status_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline]
  pub fn status(&self) -> Status { 
     unsafe {
       Status(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
  #[inline]
  pub fn set_status(&self, value: Status) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Emac {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

  #[inline]
  pub fn rwuff_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
  #[inline]
  pub fn rwuff_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
  #[inline]
  pub fn rwuff(&self) -> Rwuff { 
     unsafe {
       Rwuff(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
  #[inline]
  pub fn set_rwuff(&self, value: Rwuff) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_rwuff<F: FnOnce(Rwuff) -> Rwuff>(&self, f: F) -> &Emac {
     let tmp = self.rwuff();
     self.set_rwuff(f(tmp))
  }

  #[inline]
  pub fn pmtctlstat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
  #[inline]
  pub fn pmtctlstat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
  #[inline]
  pub fn pmtctlstat(&self) -> Pmtctlstat { 
     unsafe {
       Pmtctlstat(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
  #[inline]
  pub fn set_pmtctlstat(&self, value: Pmtctlstat) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pmtctlstat<F: FnOnce(Pmtctlstat) -> Pmtctlstat>(&self, f: F) -> &Emac {
     let tmp = self.pmtctlstat();
     self.set_pmtctlstat(f(tmp))
  }

  #[inline]
  pub fn ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
  #[inline]
  pub fn ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
  #[inline]
  pub fn ris(&self) -> Ris { 
     unsafe {
       Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
  #[inline]
  pub fn set_ris(&self, value: Ris) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Emac {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

  #[inline]
  pub fn im_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3c) as *const u32
  }
  #[inline]
  pub fn im_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3c) as *mut u32
  }
  #[inline]
  pub fn im(&self) -> Im { 
     unsafe {
       Im(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
     }
  }
  #[inline]
  pub fn set_im(&self, value: Im) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Emac {
     let tmp = self.im();
     self.set_im(f(tmp))
  }

  #[inline]
  pub fn addr0h_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
  #[inline]
  pub fn addr0h_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
  #[inline]
  pub fn addr0h(&self) -> Addr0h { 
     unsafe {
       Addr0h(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
  #[inline]
  pub fn set_addr0h(&self, value: Addr0h) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_addr0h<F: FnOnce(Addr0h) -> Addr0h>(&self, f: F) -> &Emac {
     let tmp = self.addr0h();
     self.set_addr0h(f(tmp))
  }

  #[inline]
  pub fn addr0l_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
  #[inline]
  pub fn addr0l_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
  #[inline]
  pub fn addr0l(&self) -> Addr0l { 
     unsafe {
       Addr0l(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
  #[inline]
  pub fn set_addr0l(&self, value: Addr0l) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_addr0l<F: FnOnce(Addr0l) -> Addr0l>(&self, f: F) -> &Emac {
     let tmp = self.addr0l();
     self.set_addr0l(f(tmp))
  }

  #[inline]
  pub fn addr1h_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
  #[inline]
  pub fn addr1h_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
  #[inline]
  pub fn addr1h(&self) -> Addr1h { 
     unsafe {
       Addr1h(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
  #[inline]
  pub fn set_addr1h(&self, value: Addr1h) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_addr1h<F: FnOnce(Addr1h) -> Addr1h>(&self, f: F) -> &Emac {
     let tmp = self.addr1h();
     self.set_addr1h(f(tmp))
  }

  #[inline]
  pub fn addr1l_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
  #[inline]
  pub fn addr1l_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
  #[inline]
  pub fn addr1l(&self) -> Addr1l { 
     unsafe {
       Addr1l(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
  #[inline]
  pub fn set_addr1l(&self, value: Addr1l) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_addr1l<F: FnOnce(Addr1l) -> Addr1l>(&self, f: F) -> &Emac {
     let tmp = self.addr1l();
     self.set_addr1l(f(tmp))
  }

  #[inline]
  pub fn addr2h_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x50) as *const u32
  }
  #[inline]
  pub fn addr2h_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x50) as *mut u32
  }
  #[inline]
  pub fn addr2h(&self) -> Addr2h { 
     unsafe {
       Addr2h(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
  #[inline]
  pub fn set_addr2h(&self, value: Addr2h) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_addr2h<F: FnOnce(Addr2h) -> Addr2h>(&self, f: F) -> &Emac {
     let tmp = self.addr2h();
     self.set_addr2h(f(tmp))
  }

  #[inline]
  pub fn addr2l_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x54) as *const u32
  }
  #[inline]
  pub fn addr2l_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x54) as *mut u32
  }
  #[inline]
  pub fn addr2l(&self) -> Addr2l { 
     unsafe {
       Addr2l(::core::ptr::read_volatile(((self.0 as usize) + 0x54) as *const u32))
     }
  }
  #[inline]
  pub fn set_addr2l(&self, value: Addr2l) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x54) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_addr2l<F: FnOnce(Addr2l) -> Addr2l>(&self, f: F) -> &Emac {
     let tmp = self.addr2l();
     self.set_addr2l(f(tmp))
  }

  #[inline]
  pub fn addr3h_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x58) as *const u32
  }
  #[inline]
  pub fn addr3h_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x58) as *mut u32
  }
  #[inline]
  pub fn addr3h(&self) -> Addr3h { 
     unsafe {
       Addr3h(::core::ptr::read_volatile(((self.0 as usize) + 0x58) as *const u32))
     }
  }
  #[inline]
  pub fn set_addr3h(&self, value: Addr3h) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x58) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_addr3h<F: FnOnce(Addr3h) -> Addr3h>(&self, f: F) -> &Emac {
     let tmp = self.addr3h();
     self.set_addr3h(f(tmp))
  }

  #[inline]
  pub fn addr3l_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x5c) as *const u32
  }
  #[inline]
  pub fn addr3l_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x5c) as *mut u32
  }
  #[inline]
  pub fn addr3l(&self) -> Addr3l { 
     unsafe {
       Addr3l(::core::ptr::read_volatile(((self.0 as usize) + 0x5c) as *const u32))
     }
  }
  #[inline]
  pub fn set_addr3l(&self, value: Addr3l) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x5c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_addr3l<F: FnOnce(Addr3l) -> Addr3l>(&self, f: F) -> &Emac {
     let tmp = self.addr3l();
     self.set_addr3l(f(tmp))
  }

  #[inline]
  pub fn wdogto_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xdc) as *const u32
  }
  #[inline]
  pub fn wdogto_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xdc) as *mut u32
  }
  #[inline]
  pub fn wdogto(&self) -> Wdogto { 
     unsafe {
       Wdogto(::core::ptr::read_volatile(((self.0 as usize) + 0xdc) as *const u32))
     }
  }
  #[inline]
  pub fn set_wdogto(&self, value: Wdogto) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xdc) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_wdogto<F: FnOnce(Wdogto) -> Wdogto>(&self, f: F) -> &Emac {
     let tmp = self.wdogto();
     self.set_wdogto(f(tmp))
  }

  #[inline]
  pub fn mmcctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x100) as *const u32
  }
  #[inline]
  pub fn mmcctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x100) as *mut u32
  }
  #[inline]
  pub fn mmcctrl(&self) -> Mmcctrl { 
     unsafe {
       Mmcctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x100) as *const u32))
     }
  }
  #[inline]
  pub fn set_mmcctrl(&self, value: Mmcctrl) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x100) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_mmcctrl<F: FnOnce(Mmcctrl) -> Mmcctrl>(&self, f: F) -> &Emac {
     let tmp = self.mmcctrl();
     self.set_mmcctrl(f(tmp))
  }

  #[inline]
  pub fn mmcrxris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x104) as *const u32
  }
  #[inline]
  pub fn mmcrxris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x104) as *mut u32
  }
  #[inline]
  pub fn mmcrxris(&self) -> Mmcrxris { 
     unsafe {
       Mmcrxris(::core::ptr::read_volatile(((self.0 as usize) + 0x104) as *const u32))
     }
  }
  #[inline]
  pub fn set_mmcrxris(&self, value: Mmcrxris) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x104) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_mmcrxris<F: FnOnce(Mmcrxris) -> Mmcrxris>(&self, f: F) -> &Emac {
     let tmp = self.mmcrxris();
     self.set_mmcrxris(f(tmp))
  }

  #[inline]
  pub fn mmctxris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x108) as *const u32
  }
  #[inline]
  pub fn mmctxris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x108) as *mut u32
  }
  #[inline]
  pub fn mmctxris(&self) -> Mmctxris { 
     unsafe {
       Mmctxris(::core::ptr::read_volatile(((self.0 as usize) + 0x108) as *const u32))
     }
  }
  #[inline]
  pub fn set_mmctxris(&self, value: Mmctxris) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x108) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_mmctxris<F: FnOnce(Mmctxris) -> Mmctxris>(&self, f: F) -> &Emac {
     let tmp = self.mmctxris();
     self.set_mmctxris(f(tmp))
  }

  #[inline]
  pub fn mmcrxim_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10c) as *const u32
  }
  #[inline]
  pub fn mmcrxim_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10c) as *mut u32
  }
  #[inline]
  pub fn mmcrxim(&self) -> Mmcrxim { 
     unsafe {
       Mmcrxim(::core::ptr::read_volatile(((self.0 as usize) + 0x10c) as *const u32))
     }
  }
  #[inline]
  pub fn set_mmcrxim(&self, value: Mmcrxim) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_mmcrxim<F: FnOnce(Mmcrxim) -> Mmcrxim>(&self, f: F) -> &Emac {
     let tmp = self.mmcrxim();
     self.set_mmcrxim(f(tmp))
  }

  #[inline]
  pub fn mmctxim_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x110) as *const u32
  }
  #[inline]
  pub fn mmctxim_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x110) as *mut u32
  }
  #[inline]
  pub fn mmctxim(&self) -> Mmctxim { 
     unsafe {
       Mmctxim(::core::ptr::read_volatile(((self.0 as usize) + 0x110) as *const u32))
     }
  }
  #[inline]
  pub fn set_mmctxim(&self, value: Mmctxim) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x110) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_mmctxim<F: FnOnce(Mmctxim) -> Mmctxim>(&self, f: F) -> &Emac {
     let tmp = self.mmctxim();
     self.set_mmctxim(f(tmp))
  }

  #[inline]
  pub fn txcntgb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x118) as *const u32
  }
  #[inline]
  pub fn txcntgb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x118) as *mut u32
  }
  #[inline]
  pub fn txcntgb(&self) -> Txcntgb { 
     unsafe {
       Txcntgb(::core::ptr::read_volatile(((self.0 as usize) + 0x118) as *const u32))
     }
  }
  #[inline]
  pub fn set_txcntgb(&self, value: Txcntgb) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x118) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_txcntgb<F: FnOnce(Txcntgb) -> Txcntgb>(&self, f: F) -> &Emac {
     let tmp = self.txcntgb();
     self.set_txcntgb(f(tmp))
  }

  #[inline]
  pub fn txcntscol_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14c) as *const u32
  }
  #[inline]
  pub fn txcntscol_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14c) as *mut u32
  }
  #[inline]
  pub fn txcntscol(&self) -> Txcntscol { 
     unsafe {
       Txcntscol(::core::ptr::read_volatile(((self.0 as usize) + 0x14c) as *const u32))
     }
  }
  #[inline]
  pub fn set_txcntscol(&self, value: Txcntscol) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_txcntscol<F: FnOnce(Txcntscol) -> Txcntscol>(&self, f: F) -> &Emac {
     let tmp = self.txcntscol();
     self.set_txcntscol(f(tmp))
  }

  #[inline]
  pub fn txcntmcol_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x150) as *const u32
  }
  #[inline]
  pub fn txcntmcol_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x150) as *mut u32
  }
  #[inline]
  pub fn txcntmcol(&self) -> Txcntmcol { 
     unsafe {
       Txcntmcol(::core::ptr::read_volatile(((self.0 as usize) + 0x150) as *const u32))
     }
  }
  #[inline]
  pub fn set_txcntmcol(&self, value: Txcntmcol) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x150) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_txcntmcol<F: FnOnce(Txcntmcol) -> Txcntmcol>(&self, f: F) -> &Emac {
     let tmp = self.txcntmcol();
     self.set_txcntmcol(f(tmp))
  }

  #[inline]
  pub fn txoctcntg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x164) as *const u32
  }
  #[inline]
  pub fn txoctcntg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x164) as *mut u32
  }
  #[inline]
  pub fn txoctcntg(&self) -> Txoctcntg { 
     unsafe {
       Txoctcntg(::core::ptr::read_volatile(((self.0 as usize) + 0x164) as *const u32))
     }
  }
  #[inline]
  pub fn set_txoctcntg(&self, value: Txoctcntg) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x164) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_txoctcntg<F: FnOnce(Txoctcntg) -> Txoctcntg>(&self, f: F) -> &Emac {
     let tmp = self.txoctcntg();
     self.set_txoctcntg(f(tmp))
  }

  #[inline]
  pub fn rxcntgb_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x180) as *const u32
  }
  #[inline]
  pub fn rxcntgb_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x180) as *mut u32
  }
  #[inline]
  pub fn rxcntgb(&self) -> Rxcntgb { 
     unsafe {
       Rxcntgb(::core::ptr::read_volatile(((self.0 as usize) + 0x180) as *const u32))
     }
  }
  #[inline]
  pub fn set_rxcntgb(&self, value: Rxcntgb) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x180) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_rxcntgb<F: FnOnce(Rxcntgb) -> Rxcntgb>(&self, f: F) -> &Emac {
     let tmp = self.rxcntgb();
     self.set_rxcntgb(f(tmp))
  }

  #[inline]
  pub fn rxcntcrcerr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x194) as *const u32
  }
  #[inline]
  pub fn rxcntcrcerr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x194) as *mut u32
  }
  #[inline]
  pub fn rxcntcrcerr(&self) -> Rxcntcrcerr { 
     unsafe {
       Rxcntcrcerr(::core::ptr::read_volatile(((self.0 as usize) + 0x194) as *const u32))
     }
  }
  #[inline]
  pub fn set_rxcntcrcerr(&self, value: Rxcntcrcerr) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x194) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_rxcntcrcerr<F: FnOnce(Rxcntcrcerr) -> Rxcntcrcerr>(&self, f: F) -> &Emac {
     let tmp = self.rxcntcrcerr();
     self.set_rxcntcrcerr(f(tmp))
  }

  #[inline]
  pub fn rxcntalgnerr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x198) as *const u32
  }
  #[inline]
  pub fn rxcntalgnerr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x198) as *mut u32
  }
  #[inline]
  pub fn rxcntalgnerr(&self) -> Rxcntalgnerr { 
     unsafe {
       Rxcntalgnerr(::core::ptr::read_volatile(((self.0 as usize) + 0x198) as *const u32))
     }
  }
  #[inline]
  pub fn set_rxcntalgnerr(&self, value: Rxcntalgnerr) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x198) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_rxcntalgnerr<F: FnOnce(Rxcntalgnerr) -> Rxcntalgnerr>(&self, f: F) -> &Emac {
     let tmp = self.rxcntalgnerr();
     self.set_rxcntalgnerr(f(tmp))
  }

  #[inline]
  pub fn rxcntguni_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c4) as *const u32
  }
  #[inline]
  pub fn rxcntguni_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c4) as *mut u32
  }
  #[inline]
  pub fn rxcntguni(&self) -> Rxcntguni { 
     unsafe {
       Rxcntguni(::core::ptr::read_volatile(((self.0 as usize) + 0x1c4) as *const u32))
     }
  }
  #[inline]
  pub fn set_rxcntguni(&self, value: Rxcntguni) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x1c4) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_rxcntguni<F: FnOnce(Rxcntguni) -> Rxcntguni>(&self, f: F) -> &Emac {
     let tmp = self.rxcntguni();
     self.set_rxcntguni(f(tmp))
  }

  #[inline]
  pub fn vlnincrep_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x584) as *const u32
  }
  #[inline]
  pub fn vlnincrep_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x584) as *mut u32
  }
  #[inline]
  pub fn vlnincrep(&self) -> Vlnincrep { 
     unsafe {
       Vlnincrep(::core::ptr::read_volatile(((self.0 as usize) + 0x584) as *const u32))
     }
  }
  #[inline]
  pub fn set_vlnincrep(&self, value: Vlnincrep) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x584) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_vlnincrep<F: FnOnce(Vlnincrep) -> Vlnincrep>(&self, f: F) -> &Emac {
     let tmp = self.vlnincrep();
     self.set_vlnincrep(f(tmp))
  }

  #[inline]
  pub fn vlanhash_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x588) as *const u32
  }
  #[inline]
  pub fn vlanhash_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x588) as *mut u32
  }
  #[inline]
  pub fn vlanhash(&self) -> Vlanhash { 
     unsafe {
       Vlanhash(::core::ptr::read_volatile(((self.0 as usize) + 0x588) as *const u32))
     }
  }
  #[inline]
  pub fn set_vlanhash(&self, value: Vlanhash) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x588) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_vlanhash<F: FnOnce(Vlanhash) -> Vlanhash>(&self, f: F) -> &Emac {
     let tmp = self.vlanhash();
     self.set_vlanhash(f(tmp))
  }

  #[inline]
  pub fn timstctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x700) as *const u32
  }
  #[inline]
  pub fn timstctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x700) as *mut u32
  }
  #[inline]
  pub fn timstctrl(&self) -> Timstctrl { 
     unsafe {
       Timstctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x700) as *const u32))
     }
  }
  #[inline]
  pub fn set_timstctrl(&self, value: Timstctrl) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x700) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_timstctrl<F: FnOnce(Timstctrl) -> Timstctrl>(&self, f: F) -> &Emac {
     let tmp = self.timstctrl();
     self.set_timstctrl(f(tmp))
  }

  #[inline]
  pub fn subsecinc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x704) as *const u32
  }
  #[inline]
  pub fn subsecinc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x704) as *mut u32
  }
  #[inline]
  pub fn subsecinc(&self) -> Subsecinc { 
     unsafe {
       Subsecinc(::core::ptr::read_volatile(((self.0 as usize) + 0x704) as *const u32))
     }
  }
  #[inline]
  pub fn set_subsecinc(&self, value: Subsecinc) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x704) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_subsecinc<F: FnOnce(Subsecinc) -> Subsecinc>(&self, f: F) -> &Emac {
     let tmp = self.subsecinc();
     self.set_subsecinc(f(tmp))
  }

  #[inline]
  pub fn timsec_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x708) as *const u32
  }
  #[inline]
  pub fn timsec_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x708) as *mut u32
  }
  #[inline]
  pub fn timsec(&self) -> Timsec { 
     unsafe {
       Timsec(::core::ptr::read_volatile(((self.0 as usize) + 0x708) as *const u32))
     }
  }
  #[inline]
  pub fn set_timsec(&self, value: Timsec) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x708) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_timsec<F: FnOnce(Timsec) -> Timsec>(&self, f: F) -> &Emac {
     let tmp = self.timsec();
     self.set_timsec(f(tmp))
  }

  #[inline]
  pub fn timnano_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x70c) as *const u32
  }
  #[inline]
  pub fn timnano_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x70c) as *mut u32
  }
  #[inline]
  pub fn timnano(&self) -> Timnano { 
     unsafe {
       Timnano(::core::ptr::read_volatile(((self.0 as usize) + 0x70c) as *const u32))
     }
  }
  #[inline]
  pub fn set_timnano(&self, value: Timnano) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x70c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_timnano<F: FnOnce(Timnano) -> Timnano>(&self, f: F) -> &Emac {
     let tmp = self.timnano();
     self.set_timnano(f(tmp))
  }

  #[inline]
  pub fn timsecu_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x710) as *const u32
  }
  #[inline]
  pub fn timsecu_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x710) as *mut u32
  }
  #[inline]
  pub fn timsecu(&self) -> Timsecu { 
     unsafe {
       Timsecu(::core::ptr::read_volatile(((self.0 as usize) + 0x710) as *const u32))
     }
  }
  #[inline]
  pub fn set_timsecu(&self, value: Timsecu) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x710) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_timsecu<F: FnOnce(Timsecu) -> Timsecu>(&self, f: F) -> &Emac {
     let tmp = self.timsecu();
     self.set_timsecu(f(tmp))
  }

  #[inline]
  pub fn timnanou_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x714) as *const u32
  }
  #[inline]
  pub fn timnanou_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x714) as *mut u32
  }
  #[inline]
  pub fn timnanou(&self) -> Timnanou { 
     unsafe {
       Timnanou(::core::ptr::read_volatile(((self.0 as usize) + 0x714) as *const u32))
     }
  }
  #[inline]
  pub fn set_timnanou(&self, value: Timnanou) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x714) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_timnanou<F: FnOnce(Timnanou) -> Timnanou>(&self, f: F) -> &Emac {
     let tmp = self.timnanou();
     self.set_timnanou(f(tmp))
  }

  #[inline]
  pub fn timadd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x718) as *const u32
  }
  #[inline]
  pub fn timadd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x718) as *mut u32
  }
  #[inline]
  pub fn timadd(&self) -> Timadd { 
     unsafe {
       Timadd(::core::ptr::read_volatile(((self.0 as usize) + 0x718) as *const u32))
     }
  }
  #[inline]
  pub fn set_timadd(&self, value: Timadd) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x718) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_timadd<F: FnOnce(Timadd) -> Timadd>(&self, f: F) -> &Emac {
     let tmp = self.timadd();
     self.set_timadd(f(tmp))
  }

  #[inline]
  pub fn targsec_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x71c) as *const u32
  }
  #[inline]
  pub fn targsec_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x71c) as *mut u32
  }
  #[inline]
  pub fn targsec(&self) -> Targsec { 
     unsafe {
       Targsec(::core::ptr::read_volatile(((self.0 as usize) + 0x71c) as *const u32))
     }
  }
  #[inline]
  pub fn set_targsec(&self, value: Targsec) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x71c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_targsec<F: FnOnce(Targsec) -> Targsec>(&self, f: F) -> &Emac {
     let tmp = self.targsec();
     self.set_targsec(f(tmp))
  }

  #[inline]
  pub fn targnano_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x720) as *const u32
  }
  #[inline]
  pub fn targnano_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x720) as *mut u32
  }
  #[inline]
  pub fn targnano(&self) -> Targnano { 
     unsafe {
       Targnano(::core::ptr::read_volatile(((self.0 as usize) + 0x720) as *const u32))
     }
  }
  #[inline]
  pub fn set_targnano(&self, value: Targnano) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x720) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_targnano<F: FnOnce(Targnano) -> Targnano>(&self, f: F) -> &Emac {
     let tmp = self.targnano();
     self.set_targnano(f(tmp))
  }

  #[inline]
  pub fn hwordsec_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x724) as *const u32
  }
  #[inline]
  pub fn hwordsec_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x724) as *mut u32
  }
  #[inline]
  pub fn hwordsec(&self) -> Hwordsec { 
     unsafe {
       Hwordsec(::core::ptr::read_volatile(((self.0 as usize) + 0x724) as *const u32))
     }
  }
  #[inline]
  pub fn set_hwordsec(&self, value: Hwordsec) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x724) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_hwordsec<F: FnOnce(Hwordsec) -> Hwordsec>(&self, f: F) -> &Emac {
     let tmp = self.hwordsec();
     self.set_hwordsec(f(tmp))
  }

  #[inline]
  pub fn timstat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x728) as *const u32
  }
  #[inline]
  pub fn timstat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x728) as *mut u32
  }
  #[inline]
  pub fn timstat(&self) -> Timstat { 
     unsafe {
       Timstat(::core::ptr::read_volatile(((self.0 as usize) + 0x728) as *const u32))
     }
  }
  #[inline]
  pub fn set_timstat(&self, value: Timstat) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x728) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_timstat<F: FnOnce(Timstat) -> Timstat>(&self, f: F) -> &Emac {
     let tmp = self.timstat();
     self.set_timstat(f(tmp))
  }

  #[inline]
  pub fn ppsctrl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x72c) as *const u32
  }
  #[inline]
  pub fn ppsctrl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x72c) as *mut u32
  }
  #[inline]
  pub fn ppsctrl(&self) -> Ppsctrl { 
     unsafe {
       Ppsctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x72c) as *const u32))
     }
  }
  #[inline]
  pub fn set_ppsctrl(&self, value: Ppsctrl) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x72c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_ppsctrl<F: FnOnce(Ppsctrl) -> Ppsctrl>(&self, f: F) -> &Emac {
     let tmp = self.ppsctrl();
     self.set_ppsctrl(f(tmp))
  }

  #[inline]
  pub fn pps0intvl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x760) as *const u32
  }
  #[inline]
  pub fn pps0intvl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x760) as *mut u32
  }
  #[inline]
  pub fn pps0intvl(&self) -> Pps0intvl { 
     unsafe {
       Pps0intvl(::core::ptr::read_volatile(((self.0 as usize) + 0x760) as *const u32))
     }
  }
  #[inline]
  pub fn set_pps0intvl(&self, value: Pps0intvl) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x760) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pps0intvl<F: FnOnce(Pps0intvl) -> Pps0intvl>(&self, f: F) -> &Emac {
     let tmp = self.pps0intvl();
     self.set_pps0intvl(f(tmp))
  }

  #[inline]
  pub fn pps0width_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x764) as *const u32
  }
  #[inline]
  pub fn pps0width_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x764) as *mut u32
  }
  #[inline]
  pub fn pps0width(&self) -> Pps0width { 
     unsafe {
       Pps0width(::core::ptr::read_volatile(((self.0 as usize) + 0x764) as *const u32))
     }
  }
  #[inline]
  pub fn set_pps0width(&self, value: Pps0width) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x764) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pps0width<F: FnOnce(Pps0width) -> Pps0width>(&self, f: F) -> &Emac {
     let tmp = self.pps0width();
     self.set_pps0width(f(tmp))
  }

  #[inline]
  pub fn dmabusmod_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc00) as *const u32
  }
  #[inline]
  pub fn dmabusmod_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc00) as *mut u32
  }
  #[inline]
  pub fn dmabusmod(&self) -> Dmabusmod { 
     unsafe {
       Dmabusmod(::core::ptr::read_volatile(((self.0 as usize) + 0xc00) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmabusmod(&self, value: Dmabusmod) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc00) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmabusmod<F: FnOnce(Dmabusmod) -> Dmabusmod>(&self, f: F) -> &Emac {
     let tmp = self.dmabusmod();
     self.set_dmabusmod(f(tmp))
  }

  #[inline]
  pub fn txpolld_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc04) as *const u32
  }
  #[inline]
  pub fn txpolld_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc04) as *mut u32
  }
  #[inline]
  pub fn set_txpolld(&self, value: Txpolld) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc04) as *mut u32, value.0);
     }
     self
  }

  #[inline]
  pub fn rxpolld_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc08) as *const u32
  }
  #[inline]
  pub fn rxpolld_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc08) as *mut u32
  }
  #[inline]
  pub fn set_rxpolld(&self, value: Rxpolld) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc08) as *mut u32, value.0);
     }
     self
  }

  #[inline]
  pub fn rxdladdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc0c) as *const u32
  }
  #[inline]
  pub fn rxdladdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc0c) as *mut u32
  }
  #[inline]
  pub fn rxdladdr(&self) -> Rxdladdr { 
     unsafe {
       Rxdladdr(::core::ptr::read_volatile(((self.0 as usize) + 0xc0c) as *const u32))
     }
  }
  #[inline]
  pub fn set_rxdladdr(&self, value: Rxdladdr) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc0c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_rxdladdr<F: FnOnce(Rxdladdr) -> Rxdladdr>(&self, f: F) -> &Emac {
     let tmp = self.rxdladdr();
     self.set_rxdladdr(f(tmp))
  }

  #[inline]
  pub fn txdladdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc10) as *const u32
  }
  #[inline]
  pub fn txdladdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc10) as *mut u32
  }
  #[inline]
  pub fn txdladdr(&self) -> Txdladdr { 
     unsafe {
       Txdladdr(::core::ptr::read_volatile(((self.0 as usize) + 0xc10) as *const u32))
     }
  }
  #[inline]
  pub fn set_txdladdr(&self, value: Txdladdr) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc10) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_txdladdr<F: FnOnce(Txdladdr) -> Txdladdr>(&self, f: F) -> &Emac {
     let tmp = self.txdladdr();
     self.set_txdladdr(f(tmp))
  }

  #[inline]
  pub fn dmaris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc14) as *const u32
  }
  #[inline]
  pub fn dmaris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc14) as *mut u32
  }
  #[inline]
  pub fn dmaris(&self) -> Dmaris { 
     unsafe {
       Dmaris(::core::ptr::read_volatile(((self.0 as usize) + 0xc14) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmaris(&self, value: Dmaris) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc14) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmaris<F: FnOnce(Dmaris) -> Dmaris>(&self, f: F) -> &Emac {
     let tmp = self.dmaris();
     self.set_dmaris(f(tmp))
  }

  #[inline]
  pub fn dmaopmode_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc18) as *const u32
  }
  #[inline]
  pub fn dmaopmode_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc18) as *mut u32
  }
  #[inline]
  pub fn dmaopmode(&self) -> Dmaopmode { 
     unsafe {
       Dmaopmode(::core::ptr::read_volatile(((self.0 as usize) + 0xc18) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmaopmode(&self, value: Dmaopmode) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc18) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmaopmode<F: FnOnce(Dmaopmode) -> Dmaopmode>(&self, f: F) -> &Emac {
     let tmp = self.dmaopmode();
     self.set_dmaopmode(f(tmp))
  }

  #[inline]
  pub fn dmaim_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc1c) as *const u32
  }
  #[inline]
  pub fn dmaim_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc1c) as *mut u32
  }
  #[inline]
  pub fn dmaim(&self) -> Dmaim { 
     unsafe {
       Dmaim(::core::ptr::read_volatile(((self.0 as usize) + 0xc1c) as *const u32))
     }
  }
  #[inline]
  pub fn set_dmaim(&self, value: Dmaim) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc1c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_dmaim<F: FnOnce(Dmaim) -> Dmaim>(&self, f: F) -> &Emac {
     let tmp = self.dmaim();
     self.set_dmaim(f(tmp))
  }

  #[inline]
  pub fn mfboc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc20) as *const u32
  }
  #[inline]
  pub fn mfboc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc20) as *mut u32
  }
  #[inline]
  pub fn mfboc(&self) -> Mfboc { 
     unsafe {
       Mfboc(::core::ptr::read_volatile(((self.0 as usize) + 0xc20) as *const u32))
     }
  }
  #[inline]
  pub fn set_mfboc(&self, value: Mfboc) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc20) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_mfboc<F: FnOnce(Mfboc) -> Mfboc>(&self, f: F) -> &Emac {
     let tmp = self.mfboc();
     self.set_mfboc(f(tmp))
  }

  #[inline]
  pub fn rxintwdt_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc24) as *const u32
  }
  #[inline]
  pub fn rxintwdt_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc24) as *mut u32
  }
  #[inline]
  pub fn rxintwdt(&self) -> Rxintwdt { 
     unsafe {
       Rxintwdt(::core::ptr::read_volatile(((self.0 as usize) + 0xc24) as *const u32))
     }
  }
  #[inline]
  pub fn set_rxintwdt(&self, value: Rxintwdt) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc24) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_rxintwdt<F: FnOnce(Rxintwdt) -> Rxintwdt>(&self, f: F) -> &Emac {
     let tmp = self.rxintwdt();
     self.set_rxintwdt(f(tmp))
  }

  #[inline]
  pub fn hostxdesc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc48) as *const u32
  }
  #[inline]
  pub fn hostxdesc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc48) as *mut u32
  }
  #[inline]
  pub fn hostxdesc(&self) -> Hostxdesc { 
     unsafe {
       Hostxdesc(::core::ptr::read_volatile(((self.0 as usize) + 0xc48) as *const u32))
     }
  }
  #[inline]
  pub fn set_hostxdesc(&self, value: Hostxdesc) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc48) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_hostxdesc<F: FnOnce(Hostxdesc) -> Hostxdesc>(&self, f: F) -> &Emac {
     let tmp = self.hostxdesc();
     self.set_hostxdesc(f(tmp))
  }

  #[inline]
  pub fn hosrxdesc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc4c) as *const u32
  }
  #[inline]
  pub fn hosrxdesc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc4c) as *mut u32
  }
  #[inline]
  pub fn hosrxdesc(&self) -> Hosrxdesc { 
     unsafe {
       Hosrxdesc(::core::ptr::read_volatile(((self.0 as usize) + 0xc4c) as *const u32))
     }
  }
  #[inline]
  pub fn set_hosrxdesc(&self, value: Hosrxdesc) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc4c) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_hosrxdesc<F: FnOnce(Hosrxdesc) -> Hosrxdesc>(&self, f: F) -> &Emac {
     let tmp = self.hosrxdesc();
     self.set_hosrxdesc(f(tmp))
  }

  #[inline]
  pub fn hostxba_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc50) as *const u32
  }
  #[inline]
  pub fn hostxba_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc50) as *mut u32
  }
  #[inline]
  pub fn hostxba(&self) -> Hostxba { 
     unsafe {
       Hostxba(::core::ptr::read_volatile(((self.0 as usize) + 0xc50) as *const u32))
     }
  }
  #[inline]
  pub fn set_hostxba(&self, value: Hostxba) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc50) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_hostxba<F: FnOnce(Hostxba) -> Hostxba>(&self, f: F) -> &Emac {
     let tmp = self.hostxba();
     self.set_hostxba(f(tmp))
  }

  #[inline]
  pub fn hosrxba_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc54) as *const u32
  }
  #[inline]
  pub fn hosrxba_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc54) as *mut u32
  }
  #[inline]
  pub fn hosrxba(&self) -> Hosrxba { 
     unsafe {
       Hosrxba(::core::ptr::read_volatile(((self.0 as usize) + 0xc54) as *const u32))
     }
  }
  #[inline]
  pub fn set_hosrxba(&self, value: Hosrxba) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc54) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_hosrxba<F: FnOnce(Hosrxba) -> Hosrxba>(&self, f: F) -> &Emac {
     let tmp = self.hosrxba();
     self.set_hosrxba(f(tmp))
  }

  #[inline]
  pub fn pp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc0) as *const u32
  }
  #[inline]
  pub fn pp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc0) as *mut u32
  }
  #[inline]
  pub fn pp(&self) -> Pp { 
     unsafe {
       Pp(::core::ptr::read_volatile(((self.0 as usize) + 0xfc0) as *const u32))
     }
  }
  #[inline]
  pub fn set_pp(&self, value: Pp) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xfc0) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Emac {
     let tmp = self.pp();
     self.set_pp(f(tmp))
  }

  #[inline]
  pub fn pc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc4) as *const u32
  }
  #[inline]
  pub fn pc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc4) as *mut u32
  }
  #[inline]
  pub fn pc(&self) -> Pc { 
     unsafe {
       Pc(::core::ptr::read_volatile(((self.0 as usize) + 0xfc4) as *const u32))
     }
  }
  #[inline]
  pub fn set_pc(&self, value: Pc) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xfc4) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_pc<F: FnOnce(Pc) -> Pc>(&self, f: F) -> &Emac {
     let tmp = self.pc();
     self.set_pc(f(tmp))
  }

  #[inline]
  pub fn cc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc8) as *const u32
  }
  #[inline]
  pub fn cc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc8) as *mut u32
  }
  #[inline]
  pub fn cc(&self) -> Cc { 
     unsafe {
       Cc(::core::ptr::read_volatile(((self.0 as usize) + 0xfc8) as *const u32))
     }
  }
  #[inline]
  pub fn set_cc(&self, value: Cc) -> &Emac {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xfc8) as *mut u32, value.0);
     }
     self
  }
  #[inline]
  pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Emac {
     let tmp = self.cc();
     self.set_cc(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
  #[inline]
  pub fn prelen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
  #[inline]
  pub fn set_prelen(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn re(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_re(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn te(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_te(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn dc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_dc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn bl(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  #[inline]
  pub fn set_bl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn acs(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_acs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn dr(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_dr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn ipc(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_ipc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn dupm(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_dupm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn loopbm(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_loopbm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn dro(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_dro(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn fes(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_fes(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn ps(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_ps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn discrs(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_discrs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn ifg(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7 // [19:17]
  }
  #[inline]
  pub fn set_ifg(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn jfen(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline]
  pub fn set_jfen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn jd(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  #[inline]
  pub fn set_jd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  #[inline]
  pub fn wddis(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline]
  pub fn set_wddis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  #[inline]
  pub fn cst(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_cst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn twokpen(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline]
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
  #[inline]
  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn huc(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_huc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn hmc(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_hmc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn daif(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_daif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn dbf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_dbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn pcf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
  #[inline]
  pub fn set_pcf(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn saif(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_saif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn saf(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_saf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn hpf(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_hpf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn vtfe(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_vtfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn ra(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
  #[inline]
  pub fn hth(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn htl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn miib(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_miib(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn miiw(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_miiw(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn cr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0xf // [5:2]
  }
  #[inline]
  pub fn set_cr(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn mii(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1f // [10:6]
  }
  #[inline]
  pub fn set_mii(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn pla(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1f // [15:11]
  }
  #[inline]
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
  #[inline]
  pub fn data(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
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
  #[inline]
  pub fn fcbbpa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_fcbbpa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tfe(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_tfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn rfe(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_rfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn up(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_up(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn plt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline]
  pub fn set_plt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn dzqp(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_dzqp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn pt(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  #[inline]
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
  #[inline]
  pub fn vl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_vl(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn etv(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_etv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn vtim(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_vtim(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn esvl(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_esvl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn vthm(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
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
  #[inline]
  pub fn rpe(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_rpe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn rfcfc(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3 // [2:1]
  }
  #[inline]
  pub fn set_rfcfc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn rwc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_rwc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn rrc(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  #[inline]
  pub fn set_rrc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn rxf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  #[inline]
  pub fn set_rxf(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn tpe(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_tpe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn tfc(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x3 // [18:17]
  }
  #[inline]
  pub fn set_tfc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn txpaused(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
  pub fn set_txpaused(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

  #[inline]
  pub fn trc(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
  #[inline]
  pub fn set_trc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn twc(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  #[inline]
  pub fn set_twc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  #[inline]
  pub fn txfe(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_txfe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn txff(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
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
  #[inline]
  pub fn wakeupfil(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn pwrdwn(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_pwrdwn(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn mgkpkten(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_mgkpkten(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn wupfren(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_wupfren(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn mgkprx(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mgkprx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn wuprx(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_wuprx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn glblucast(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_glblucast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn rwkptr(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x7 // [26:24]
  }
  #[inline]
  pub fn set_rwkptr(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn wupfrrst(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
  #[inline]
  pub fn pmt(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_pmt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn mmc(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_mmc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn mmcrx(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_mmcrx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn mmctx(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_mmctx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn ts(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
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
  #[inline]
  pub fn pmt(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_pmt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn tsi(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
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
  #[inline]
  pub fn addrhi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_addrhi(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
  #[inline]
  pub fn addrlo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn addrhi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_addrhi(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn mbc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3f // [29:24]
  }
  #[inline]
  pub fn set_mbc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn sa(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline]
  pub fn set_sa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline]
  pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
  #[inline]
  pub fn addrlo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn addrhi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_addrhi(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn mbc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3f // [29:24]
  }
  #[inline]
  pub fn set_mbc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn sa(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline]
  pub fn set_sa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline]
  pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
  #[inline]
  pub fn addrlo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn addrhi(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_addrhi(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn mbc(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x3f // [29:24]
  }
  #[inline]
  pub fn set_mbc(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn sa(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline]
  pub fn set_sa(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline]
  pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
  #[inline]
  pub fn addrlo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn wto(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3fff // [13:0]
  }
  #[inline]
  pub fn set_wto(mut self, value: u32) -> Self {
     assert!((value & !0x3fff) == 0);
     self.0 &= !(0x3fff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn pwe(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
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
  #[inline]
  pub fn cntrst(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_cntrst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn cntstpro(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_cntstpro(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn rstonrd(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_rstonrd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn cntfreez(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_cntfreez(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn cntprst(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_cntprst(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn cntprstlvl(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_cntprstlvl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn ucdbc(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
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
  #[inline]
  pub fn gbf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_gbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn crcerr(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_crcerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn algnerr(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_algnerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn ucgf(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
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
  #[inline]
  pub fn gbf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_gbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn scollgf(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_scollgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn mcollgf(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_mcollgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn octcnt(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline]
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
  #[inline]
  pub fn gbf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_gbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn crcerr(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_crcerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn algnerr(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_algnerr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn ucgf(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
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
  #[inline]
  pub fn gbf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_gbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn scollgf(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_scollgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn mcollgf(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_mcollgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn octcnt(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline]
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
  #[inline]
  pub fn txfrmgb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn txsnglcolg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn txmultcolg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn txoctg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn rxfrmgb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn rxcrcerr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn rxalgnerr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn rxucastg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn vlt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_vlt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn vlc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  #[inline]
  pub fn set_vlc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn vlp(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
  pub fn set_vlp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

  #[inline]
  pub fn csvl(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
  #[inline]
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
  #[inline]
  pub fn vlht(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
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
  #[inline]
  pub fn tsen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_tsen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tsfcupdt(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_tsfcupdt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn tsinit(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_tsinit(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn tsupdt(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_tsupdt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn inttrig(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_inttrig(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn addregup(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_addregup(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn allf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_allf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn dgtlbin(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_dgtlbin(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn ptpver2(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_ptpver2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn ptpeth(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_ptpeth(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn ptpipv6(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_ptpipv6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn ptpipv4(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_ptpipv4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn tsevnt(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_tsevnt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn tsmast(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_tsmast(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn selptp(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x3 // [17:16]
  }
  #[inline]
  pub fn set_selptp(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn ptpfltr(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
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
  #[inline]
  pub fn ssinc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline]
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
  #[inline]
  pub fn tss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn tsss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
  #[inline]
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
  #[inline]
  pub fn tss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn tsss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
  #[inline]
  pub fn set_tsss(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn addsub(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
  #[inline]
  pub fn tsar(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn tstr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn ttslo(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fffffff // [30:0]
  }
  #[inline]
  pub fn set_ttslo(mut self, value: u32) -> Self {
     assert!((value & !0x7fffffff) == 0);
     self.0 &= !(0x7fffffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn trgtbusy(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
  #[inline]
  pub fn tshwr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
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
  #[inline]
  pub fn tssovf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_tssovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tstargt(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
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
  #[inline]
  pub fn ppsctrl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
  #[inline]
  pub fn set_ppsctrl(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn ppsen0(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_ppsen0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn trgmods0(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  #[inline]
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
  #[inline]
  pub fn pps0int(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn emac_pps0width(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn swr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_swr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn da(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_da(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn dsl(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1f // [6:2]
  }
  #[inline]
  pub fn set_dsl(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn atds(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_atds(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn pbl(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3f // [13:8]
  }
  #[inline]
  pub fn set_pbl(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn pr(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
  #[inline]
  pub fn set_pr(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn fb(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_fb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn rpbl(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x3f // [22:17]
  }
  #[inline]
  pub fn set_rpbl(mut self, value: u32) -> Self {
     assert!((value & !0x3f) == 0);
     self.0 &= !(0x3f << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn usp(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline]
  pub fn set_usp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  #[inline]
  pub fn _8xpbl(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_8xpbl(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn aal(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_aal(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn mb(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
  pub fn set_mb(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 26);
     self.0 |= value << 26;
     self
  }

  #[inline]
  pub fn txpr(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline]
  pub fn set_txpr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  #[inline]
  pub fn rib(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
  #[inline]
  pub fn tpd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn rpd(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn strxlist(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3fffffff // [31:2]
  }
  #[inline]
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
  #[inline]
  pub fn txdladdr(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3fffffff // [31:2]
  }
  #[inline]
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
  #[inline]
  pub fn ti(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_ti(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tps(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_tps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn tu(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_tu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn tjt(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_tjt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn ovf(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_ovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn unf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_unf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn ri(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_ri(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn ru(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_ru(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn rps(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_rps(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn rwt(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_rwt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn eti(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_eti(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn fbi(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_fbi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn eri(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_eri(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn ais(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_ais(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn nis(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_nis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn rs(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7 // [19:17]
  }
  #[inline]
  pub fn set_rs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn ts(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x7 // [22:20]
  }
  #[inline]
  pub fn set_ts(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn ae(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x7 // [25:23]
  }
  #[inline]
  pub fn set_ae(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 23);
     self.0 |= value << 23;
     self
  }

  #[inline]
  pub fn mmc(&self) -> u32 {
     ((self.0 as u32) >> 27) & 0x1 // [27]
  }
  #[inline]
  pub fn set_mmc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 27);
     self.0 |= value << 27;
     self
  }

  #[inline]
  pub fn pmt(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  #[inline]
  pub fn set_pmt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 28);
     self.0 |= value << 28;
     self
  }

  #[inline]
  pub fn tt(&self) -> u32 {
     ((self.0 as u32) >> 29) & 0x1 // [29]
  }
  #[inline]
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
  #[inline]
  pub fn sr(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_sr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn osf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_osf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn rtc(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x3 // [4:3]
  }
  #[inline]
  pub fn set_rtc(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn dgf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_dgf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn fuf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_fuf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn fef(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_fef(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn ttc(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x7 // [16:14]
  }
  #[inline]
  pub fn set_ttc(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn ftf(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline]
  pub fn set_ftf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn tsf(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  #[inline]
  pub fn set_tsf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  #[inline]
  pub fn dff(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_dff(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn rsf(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_rsf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 26) & 0x1 // [26]
  }
  #[inline]
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
  #[inline]
  pub fn tie(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_tie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn tse(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline]
  pub fn set_tse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn tue(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline]
  pub fn set_tue(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline]
  pub fn tje(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_tje(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn ove(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline]
  pub fn set_ove(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn une(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline]
  pub fn set_une(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline]
  pub fn rie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_rie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn rue(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_rue(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn rse(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_rse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn rwe(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_rwe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn ete(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_ete(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn fbe(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_fbe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn ere(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_ere(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn aie(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline]
  pub fn set_aie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn nie(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
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
  #[inline]
  pub fn misfrmcnt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  #[inline]
  pub fn set_misfrmcnt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn miscntovf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline]
  pub fn set_miscntovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline]
  pub fn ovffrmcnt(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x7ff // [27:17]
  }
  #[inline]
  pub fn set_ovffrmcnt(mut self, value: u32) -> Self {
     assert!((value & !0x7ff) == 0);
     self.0 &= !(0x7ff << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn ovfcntovf(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x1 // [28]
  }
  #[inline]
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
  #[inline]
  pub fn riwt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  #[inline]
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
  #[inline]
  pub fn curtxdesc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn currxdesc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn curtxbufa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn currxbufa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline]
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
  #[inline]
  pub fn phytype(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  #[inline]
  pub fn set_phytype(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn mactype(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
  #[inline]
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
  #[inline]
  pub fn phyhold(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline]
  pub fn set_phyhold(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline]
  pub fn anmode(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x3 // [2:1]
  }
  #[inline]
  pub fn set_anmode(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline]
  pub fn anen(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline]
  pub fn set_anen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline]
  pub fn fastansel(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
  #[inline]
  pub fn set_fastansel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline]
  pub fn fastanen(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline]
  pub fn set_fastanen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline]
  pub fn extfd(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline]
  pub fn set_extfd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline]
  pub fn fastlupd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline]
  pub fn set_fastlupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline]
  pub fn fastrxdv(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline]
  pub fn set_fastrxdv(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline]
  pub fn mdixen(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline]
  pub fn set_mdixen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline]
  pub fn fastmdix(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline]
  pub fn set_fastmdix(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline]
  pub fn rbstmdix(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline]
  pub fn set_rbstmdix(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline]
  pub fn mdiswap(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline]
  pub fn set_mdiswap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline]
  pub fn polswap(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline]
  pub fn set_polswap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline]
  pub fn fastldmode(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1f // [19:15]
  }
  #[inline]
  pub fn set_fastldmode(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 15);
     self.0 |= value << 15;
     self
  }

  #[inline]
  pub fn tdrrun(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
  #[inline]
  pub fn set_tdrrun(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

  #[inline]
  pub fn lrr(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
  #[inline]
  pub fn set_lrr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

  #[inline]
  pub fn isomiill(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
  #[inline]
  pub fn set_isomiill(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

  #[inline]
  pub fn rxeridle(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
  #[inline]
  pub fn set_rxeridle(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

  #[inline]
  pub fn nibdetdis(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0x1 // [24]
  }
  #[inline]
  pub fn set_nibdetdis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 24);
     self.0 |= value << 24;
     self
  }

  #[inline]
  pub fn digrestart(&self) -> u32 {
     ((self.0 as u32) >> 25) & 0x1 // [25]
  }
  #[inline]
  pub fn set_digrestart(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 25);
     self.0 |= value << 25;
     self
  }

  #[inline]
  pub fn pintfs(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x7 // [30:28]
  }
  #[inline]
  pub fn set_pintfs(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 28);
     self.0 |= value << 28;
     self
  }

  #[inline]
  pub fn phyext(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline]
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
  #[inline]
  pub fn pol(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline]
  pub fn set_pol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

  #[inline]
  pub fn ptpcen(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
  #[inline]
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
