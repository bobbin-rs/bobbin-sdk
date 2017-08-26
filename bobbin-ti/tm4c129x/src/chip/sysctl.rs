//! Register map for SYSCTL peripheral
#[allow(unused_imports)] use bobbin_common::*;

periph!(SysctlPeriph, SYSCTL, Sysctl, 0x400fe000);

#[doc="Register map for SYSCTL peripheral"]
pub trait SysctlPeriph : Base {
#[doc="Get the *const pointer for the DID0 register."]
   #[inline] fn did0_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the DID0 register."]
   #[inline] fn did0_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the DID0 register."]
   #[inline] fn did0(&self) -> Did0 { 
      unsafe {
         Did0(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }
#[doc="Write the DID0 register."]
   #[inline] fn set_did0<F: FnOnce(Did0) -> Did0>(&self, f: F) -> &Self {
      let value = f(Did0(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DID0 register."]
   #[inline] fn with_did0<F: FnOnce(Did0) -> Did0>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Did0(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DID1 register."]
   #[inline] fn did1_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the DID1 register."]
   #[inline] fn did1_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the DID1 register."]
   #[inline] fn did1(&self) -> Did1 { 
      unsafe {
         Did1(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      }
   }
#[doc="Write the DID1 register."]
   #[inline] fn set_did1<F: FnOnce(Did1) -> Did1>(&self, f: F) -> &Self {
      let value = f(Did1(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DID1 register."]
   #[inline] fn with_did1<F: FnOnce(Did1) -> Did1>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Did1(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PTBOCTL register."]
   #[inline] fn ptboctl_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x38)
   }
#[doc="Get the *mut pointer for the PTBOCTL register."]
   #[inline] fn ptboctl_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x38)
   }
#[doc="Read the PTBOCTL register."]
   #[inline] fn ptboctl(&self) -> Ptboctl { 
      unsafe {
         Ptboctl(::core::ptr::read_volatile((self.base() + 0x38) as *const u32))
      }
   }
#[doc="Write the PTBOCTL register."]
   #[inline] fn set_ptboctl<F: FnOnce(Ptboctl) -> Ptboctl>(&self, f: F) -> &Self {
      let value = f(Ptboctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x38) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PTBOCTL register."]
   #[inline] fn with_ptboctl<F: FnOnce(Ptboctl) -> Ptboctl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ptboctl(::core::ptr::read_volatile((self.base() + 0x38) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x38) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RIS register."]
   #[inline] fn ris_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x50)
   }
#[doc="Get the *mut pointer for the RIS register."]
   #[inline] fn ris_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x50)
   }
#[doc="Read the RIS register."]
   #[inline] fn ris(&self) -> Ris { 
      unsafe {
         Ris(::core::ptr::read_volatile((self.base() + 0x50) as *const u32))
      }
   }
#[doc="Write the RIS register."]
   #[inline] fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
      let value = f(Ris(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x50) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RIS register."]
   #[inline] fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ris(::core::ptr::read_volatile((self.base() + 0x50) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x50) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IMC register."]
   #[inline] fn imc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x54)
   }
#[doc="Get the *mut pointer for the IMC register."]
   #[inline] fn imc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x54)
   }
#[doc="Read the IMC register."]
   #[inline] fn imc(&self) -> Imc { 
      unsafe {
         Imc(::core::ptr::read_volatile((self.base() + 0x54) as *const u32))
      }
   }
#[doc="Write the IMC register."]
   #[inline] fn set_imc<F: FnOnce(Imc) -> Imc>(&self, f: F) -> &Self {
      let value = f(Imc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x54) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the IMC register."]
   #[inline] fn with_imc<F: FnOnce(Imc) -> Imc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Imc(::core::ptr::read_volatile((self.base() + 0x54) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x54) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MISC register."]
   #[inline] fn misc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x58)
   }
#[doc="Get the *mut pointer for the MISC register."]
   #[inline] fn misc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x58)
   }
#[doc="Read the MISC register."]
   #[inline] fn misc(&self) -> Misc { 
      unsafe {
         Misc(::core::ptr::read_volatile((self.base() + 0x58) as *const u32))
      }
   }
#[doc="Write the MISC register."]
   #[inline] fn set_misc<F: FnOnce(Misc) -> Misc>(&self, f: F) -> &Self {
      let value = f(Misc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x58) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MISC register."]
   #[inline] fn with_misc<F: FnOnce(Misc) -> Misc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Misc(::core::ptr::read_volatile((self.base() + 0x58) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x58) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RESC register."]
   #[inline] fn resc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x5c)
   }
#[doc="Get the *mut pointer for the RESC register."]
   #[inline] fn resc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x5c)
   }
#[doc="Read the RESC register."]
   #[inline] fn resc(&self) -> Resc { 
      unsafe {
         Resc(::core::ptr::read_volatile((self.base() + 0x5c) as *const u32))
      }
   }
#[doc="Write the RESC register."]
   #[inline] fn set_resc<F: FnOnce(Resc) -> Resc>(&self, f: F) -> &Self {
      let value = f(Resc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RESC register."]
   #[inline] fn with_resc<F: FnOnce(Resc) -> Resc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Resc(::core::ptr::read_volatile((self.base() + 0x5c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PWRTC register."]
   #[inline] fn pwrtc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x60)
   }
#[doc="Get the *mut pointer for the PWRTC register."]
   #[inline] fn pwrtc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x60)
   }
#[doc="Read the PWRTC register."]
   #[inline] fn pwrtc(&self) -> Pwrtc { 
      unsafe {
         Pwrtc(::core::ptr::read_volatile((self.base() + 0x60) as *const u32))
      }
   }
#[doc="Write the PWRTC register."]
   #[inline] fn set_pwrtc<F: FnOnce(Pwrtc) -> Pwrtc>(&self, f: F) -> &Self {
      let value = f(Pwrtc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x60) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PWRTC register."]
   #[inline] fn with_pwrtc<F: FnOnce(Pwrtc) -> Pwrtc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pwrtc(::core::ptr::read_volatile((self.base() + 0x60) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x60) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the NMIC register."]
   #[inline] fn nmic_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x64)
   }
#[doc="Get the *mut pointer for the NMIC register."]
   #[inline] fn nmic_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x64)
   }
#[doc="Read the NMIC register."]
   #[inline] fn nmic(&self) -> Nmic { 
      unsafe {
         Nmic(::core::ptr::read_volatile((self.base() + 0x64) as *const u32))
      }
   }
#[doc="Write the NMIC register."]
   #[inline] fn set_nmic<F: FnOnce(Nmic) -> Nmic>(&self, f: F) -> &Self {
      let value = f(Nmic(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x64) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the NMIC register."]
   #[inline] fn with_nmic<F: FnOnce(Nmic) -> Nmic>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Nmic(::core::ptr::read_volatile((self.base() + 0x64) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x64) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MOSCCTL register."]
   #[inline] fn moscctl_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x7c)
   }
#[doc="Get the *mut pointer for the MOSCCTL register."]
   #[inline] fn moscctl_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x7c)
   }
#[doc="Read the MOSCCTL register."]
   #[inline] fn moscctl(&self) -> Moscctl { 
      unsafe {
         Moscctl(::core::ptr::read_volatile((self.base() + 0x7c) as *const u32))
      }
   }
#[doc="Write the MOSCCTL register."]
   #[inline] fn set_moscctl<F: FnOnce(Moscctl) -> Moscctl>(&self, f: F) -> &Self {
      let value = f(Moscctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x7c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MOSCCTL register."]
   #[inline] fn with_moscctl<F: FnOnce(Moscctl) -> Moscctl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Moscctl(::core::ptr::read_volatile((self.base() + 0x7c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x7c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RSCLKCFG register."]
   #[inline] fn rsclkcfg_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xb0)
   }
#[doc="Get the *mut pointer for the RSCLKCFG register."]
   #[inline] fn rsclkcfg_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xb0)
   }
#[doc="Read the RSCLKCFG register."]
   #[inline] fn rsclkcfg(&self) -> Rsclkcfg { 
      unsafe {
         Rsclkcfg(::core::ptr::read_volatile((self.base() + 0xb0) as *const u32))
      }
   }
#[doc="Write the RSCLKCFG register."]
   #[inline] fn set_rsclkcfg<F: FnOnce(Rsclkcfg) -> Rsclkcfg>(&self, f: F) -> &Self {
      let value = f(Rsclkcfg(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xb0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RSCLKCFG register."]
   #[inline] fn with_rsclkcfg<F: FnOnce(Rsclkcfg) -> Rsclkcfg>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rsclkcfg(::core::ptr::read_volatile((self.base() + 0xb0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xb0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MEMTIM0 register."]
   #[inline] fn memtim0_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc0)
   }
#[doc="Get the *mut pointer for the MEMTIM0 register."]
   #[inline] fn memtim0_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc0)
   }
#[doc="Read the MEMTIM0 register."]
   #[inline] fn memtim0(&self) -> Memtim0 { 
      unsafe {
         Memtim0(::core::ptr::read_volatile((self.base() + 0xc0) as *const u32))
      }
   }
#[doc="Write the MEMTIM0 register."]
   #[inline] fn set_memtim0<F: FnOnce(Memtim0) -> Memtim0>(&self, f: F) -> &Self {
      let value = f(Memtim0(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MEMTIM0 register."]
   #[inline] fn with_memtim0<F: FnOnce(Memtim0) -> Memtim0>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Memtim0(::core::ptr::read_volatile((self.base() + 0xc0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ALTCLKCFG register."]
   #[inline] fn altclkcfg_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x138)
   }
#[doc="Get the *mut pointer for the ALTCLKCFG register."]
   #[inline] fn altclkcfg_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x138)
   }
#[doc="Read the ALTCLKCFG register."]
   #[inline] fn altclkcfg(&self) -> Altclkcfg { 
      unsafe {
         Altclkcfg(::core::ptr::read_volatile((self.base() + 0x138) as *const u32))
      }
   }
#[doc="Write the ALTCLKCFG register."]
   #[inline] fn set_altclkcfg<F: FnOnce(Altclkcfg) -> Altclkcfg>(&self, f: F) -> &Self {
      let value = f(Altclkcfg(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x138) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ALTCLKCFG register."]
   #[inline] fn with_altclkcfg<F: FnOnce(Altclkcfg) -> Altclkcfg>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Altclkcfg(::core::ptr::read_volatile((self.base() + 0x138) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x138) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DSCLKCFG register."]
   #[inline] fn dsclkcfg_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x144)
   }
#[doc="Get the *mut pointer for the DSCLKCFG register."]
   #[inline] fn dsclkcfg_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x144)
   }
#[doc="Read the DSCLKCFG register."]
   #[inline] fn dsclkcfg(&self) -> Dsclkcfg { 
      unsafe {
         Dsclkcfg(::core::ptr::read_volatile((self.base() + 0x144) as *const u32))
      }
   }
#[doc="Write the DSCLKCFG register."]
   #[inline] fn set_dsclkcfg<F: FnOnce(Dsclkcfg) -> Dsclkcfg>(&self, f: F) -> &Self {
      let value = f(Dsclkcfg(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x144) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DSCLKCFG register."]
   #[inline] fn with_dsclkcfg<F: FnOnce(Dsclkcfg) -> Dsclkcfg>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dsclkcfg(::core::ptr::read_volatile((self.base() + 0x144) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x144) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DIVSCLK register."]
   #[inline] fn divsclk_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x148)
   }
#[doc="Get the *mut pointer for the DIVSCLK register."]
   #[inline] fn divsclk_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x148)
   }
#[doc="Read the DIVSCLK register."]
   #[inline] fn divsclk(&self) -> Divsclk { 
      unsafe {
         Divsclk(::core::ptr::read_volatile((self.base() + 0x148) as *const u32))
      }
   }
#[doc="Write the DIVSCLK register."]
   #[inline] fn set_divsclk<F: FnOnce(Divsclk) -> Divsclk>(&self, f: F) -> &Self {
      let value = f(Divsclk(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x148) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DIVSCLK register."]
   #[inline] fn with_divsclk<F: FnOnce(Divsclk) -> Divsclk>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Divsclk(::core::ptr::read_volatile((self.base() + 0x148) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x148) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SYSPROP register."]
   #[inline] fn sysprop_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x14c)
   }
#[doc="Get the *mut pointer for the SYSPROP register."]
   #[inline] fn sysprop_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x14c)
   }
#[doc="Read the SYSPROP register."]
   #[inline] fn sysprop(&self) -> Sysprop { 
      unsafe {
         Sysprop(::core::ptr::read_volatile((self.base() + 0x14c) as *const u32))
      }
   }
#[doc="Write the SYSPROP register."]
   #[inline] fn set_sysprop<F: FnOnce(Sysprop) -> Sysprop>(&self, f: F) -> &Self {
      let value = f(Sysprop(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SYSPROP register."]
   #[inline] fn with_sysprop<F: FnOnce(Sysprop) -> Sysprop>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Sysprop(::core::ptr::read_volatile((self.base() + 0x14c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PIOSCCAL register."]
   #[inline] fn piosccal_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x150)
   }
#[doc="Get the *mut pointer for the PIOSCCAL register."]
   #[inline] fn piosccal_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x150)
   }
#[doc="Read the PIOSCCAL register."]
   #[inline] fn piosccal(&self) -> Piosccal { 
      unsafe {
         Piosccal(::core::ptr::read_volatile((self.base() + 0x150) as *const u32))
      }
   }
#[doc="Write the PIOSCCAL register."]
   #[inline] fn set_piosccal<F: FnOnce(Piosccal) -> Piosccal>(&self, f: F) -> &Self {
      let value = f(Piosccal(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x150) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PIOSCCAL register."]
   #[inline] fn with_piosccal<F: FnOnce(Piosccal) -> Piosccal>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Piosccal(::core::ptr::read_volatile((self.base() + 0x150) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x150) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PIOSCSTAT register."]
   #[inline] fn pioscstat_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x154)
   }
#[doc="Get the *mut pointer for the PIOSCSTAT register."]
   #[inline] fn pioscstat_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x154)
   }
#[doc="Read the PIOSCSTAT register."]
   #[inline] fn pioscstat(&self) -> Pioscstat { 
      unsafe {
         Pioscstat(::core::ptr::read_volatile((self.base() + 0x154) as *const u32))
      }
   }
#[doc="Write the PIOSCSTAT register."]
   #[inline] fn set_pioscstat<F: FnOnce(Pioscstat) -> Pioscstat>(&self, f: F) -> &Self {
      let value = f(Pioscstat(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x154) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PIOSCSTAT register."]
   #[inline] fn with_pioscstat<F: FnOnce(Pioscstat) -> Pioscstat>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pioscstat(::core::ptr::read_volatile((self.base() + 0x154) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x154) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PLLFREQ0 register."]
   #[inline] fn pllfreq0_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x160)
   }
#[doc="Get the *mut pointer for the PLLFREQ0 register."]
   #[inline] fn pllfreq0_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x160)
   }
#[doc="Read the PLLFREQ0 register."]
   #[inline] fn pllfreq0(&self) -> Pllfreq0 { 
      unsafe {
         Pllfreq0(::core::ptr::read_volatile((self.base() + 0x160) as *const u32))
      }
   }
#[doc="Write the PLLFREQ0 register."]
   #[inline] fn set_pllfreq0<F: FnOnce(Pllfreq0) -> Pllfreq0>(&self, f: F) -> &Self {
      let value = f(Pllfreq0(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x160) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PLLFREQ0 register."]
   #[inline] fn with_pllfreq0<F: FnOnce(Pllfreq0) -> Pllfreq0>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pllfreq0(::core::ptr::read_volatile((self.base() + 0x160) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x160) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PLLFREQ1 register."]
   #[inline] fn pllfreq1_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x164)
   }
#[doc="Get the *mut pointer for the PLLFREQ1 register."]
   #[inline] fn pllfreq1_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x164)
   }
#[doc="Read the PLLFREQ1 register."]
   #[inline] fn pllfreq1(&self) -> Pllfreq1 { 
      unsafe {
         Pllfreq1(::core::ptr::read_volatile((self.base() + 0x164) as *const u32))
      }
   }
#[doc="Write the PLLFREQ1 register."]
   #[inline] fn set_pllfreq1<F: FnOnce(Pllfreq1) -> Pllfreq1>(&self, f: F) -> &Self {
      let value = f(Pllfreq1(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x164) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PLLFREQ1 register."]
   #[inline] fn with_pllfreq1<F: FnOnce(Pllfreq1) -> Pllfreq1>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pllfreq1(::core::ptr::read_volatile((self.base() + 0x164) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x164) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PLLSTAT register."]
   #[inline] fn pllstat_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x168)
   }
#[doc="Get the *mut pointer for the PLLSTAT register."]
   #[inline] fn pllstat_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x168)
   }
#[doc="Read the PLLSTAT register."]
   #[inline] fn pllstat(&self) -> Pllstat { 
      unsafe {
         Pllstat(::core::ptr::read_volatile((self.base() + 0x168) as *const u32))
      }
   }
#[doc="Write the PLLSTAT register."]
   #[inline] fn set_pllstat<F: FnOnce(Pllstat) -> Pllstat>(&self, f: F) -> &Self {
      let value = f(Pllstat(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x168) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PLLSTAT register."]
   #[inline] fn with_pllstat<F: FnOnce(Pllstat) -> Pllstat>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pllstat(::core::ptr::read_volatile((self.base() + 0x168) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x168) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SLPPWRCFG register."]
   #[inline] fn slppwrcfg_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x188)
   }
#[doc="Get the *mut pointer for the SLPPWRCFG register."]
   #[inline] fn slppwrcfg_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x188)
   }
#[doc="Read the SLPPWRCFG register."]
   #[inline] fn slppwrcfg(&self) -> Slppwrcfg { 
      unsafe {
         Slppwrcfg(::core::ptr::read_volatile((self.base() + 0x188) as *const u32))
      }
   }
#[doc="Write the SLPPWRCFG register."]
   #[inline] fn set_slppwrcfg<F: FnOnce(Slppwrcfg) -> Slppwrcfg>(&self, f: F) -> &Self {
      let value = f(Slppwrcfg(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x188) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SLPPWRCFG register."]
   #[inline] fn with_slppwrcfg<F: FnOnce(Slppwrcfg) -> Slppwrcfg>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Slppwrcfg(::core::ptr::read_volatile((self.base() + 0x188) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x188) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DSLPPWRCFG register."]
   #[inline] fn dslppwrcfg_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x18c)
   }
#[doc="Get the *mut pointer for the DSLPPWRCFG register."]
   #[inline] fn dslppwrcfg_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x18c)
   }
#[doc="Read the DSLPPWRCFG register."]
   #[inline] fn dslppwrcfg(&self) -> Dslppwrcfg { 
      unsafe {
         Dslppwrcfg(::core::ptr::read_volatile((self.base() + 0x18c) as *const u32))
      }
   }
#[doc="Write the DSLPPWRCFG register."]
   #[inline] fn set_dslppwrcfg<F: FnOnce(Dslppwrcfg) -> Dslppwrcfg>(&self, f: F) -> &Self {
      let value = f(Dslppwrcfg(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DSLPPWRCFG register."]
   #[inline] fn with_dslppwrcfg<F: FnOnce(Dslppwrcfg) -> Dslppwrcfg>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dslppwrcfg(::core::ptr::read_volatile((self.base() + 0x18c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the NVMSTAT register."]
   #[inline] fn nvmstat_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x1a0)
   }
#[doc="Get the *mut pointer for the NVMSTAT register."]
   #[inline] fn nvmstat_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x1a0)
   }
#[doc="Read the NVMSTAT register."]
   #[inline] fn nvmstat(&self) -> Nvmstat { 
      unsafe {
         Nvmstat(::core::ptr::read_volatile((self.base() + 0x1a0) as *const u32))
      }
   }
#[doc="Write the NVMSTAT register."]
   #[inline] fn set_nvmstat<F: FnOnce(Nvmstat) -> Nvmstat>(&self, f: F) -> &Self {
      let value = f(Nvmstat(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1a0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the NVMSTAT register."]
   #[inline] fn with_nvmstat<F: FnOnce(Nvmstat) -> Nvmstat>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Nvmstat(::core::ptr::read_volatile((self.base() + 0x1a0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1a0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the LDOSPCTL register."]
   #[inline] fn ldospctl_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x1b4)
   }
#[doc="Get the *mut pointer for the LDOSPCTL register."]
   #[inline] fn ldospctl_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x1b4)
   }
#[doc="Read the LDOSPCTL register."]
   #[inline] fn ldospctl(&self) -> Ldospctl { 
      unsafe {
         Ldospctl(::core::ptr::read_volatile((self.base() + 0x1b4) as *const u32))
      }
   }
#[doc="Write the LDOSPCTL register."]
   #[inline] fn set_ldospctl<F: FnOnce(Ldospctl) -> Ldospctl>(&self, f: F) -> &Self {
      let value = f(Ldospctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1b4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LDOSPCTL register."]
   #[inline] fn with_ldospctl<F: FnOnce(Ldospctl) -> Ldospctl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ldospctl(::core::ptr::read_volatile((self.base() + 0x1b4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1b4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the LDODPCTL register."]
   #[inline] fn ldodpctl_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x1bc)
   }
#[doc="Get the *mut pointer for the LDODPCTL register."]
   #[inline] fn ldodpctl_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x1bc)
   }
#[doc="Read the LDODPCTL register."]
   #[inline] fn ldodpctl(&self) -> Ldodpctl { 
      unsafe {
         Ldodpctl(::core::ptr::read_volatile((self.base() + 0x1bc) as *const u32))
      }
   }
#[doc="Write the LDODPCTL register."]
   #[inline] fn set_ldodpctl<F: FnOnce(Ldodpctl) -> Ldodpctl>(&self, f: F) -> &Self {
      let value = f(Ldodpctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1bc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LDODPCTL register."]
   #[inline] fn with_ldodpctl<F: FnOnce(Ldodpctl) -> Ldodpctl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ldodpctl(::core::ptr::read_volatile((self.base() + 0x1bc) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1bc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RESBEHAVCTL register."]
   #[inline] fn resbehavctl_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x1d8)
   }
#[doc="Get the *mut pointer for the RESBEHAVCTL register."]
   #[inline] fn resbehavctl_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x1d8)
   }
#[doc="Read the RESBEHAVCTL register."]
   #[inline] fn resbehavctl(&self) -> Resbehavctl { 
      unsafe {
         Resbehavctl(::core::ptr::read_volatile((self.base() + 0x1d8) as *const u32))
      }
   }
#[doc="Write the RESBEHAVCTL register."]
   #[inline] fn set_resbehavctl<F: FnOnce(Resbehavctl) -> Resbehavctl>(&self, f: F) -> &Self {
      let value = f(Resbehavctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1d8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RESBEHAVCTL register."]
   #[inline] fn with_resbehavctl<F: FnOnce(Resbehavctl) -> Resbehavctl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Resbehavctl(::core::ptr::read_volatile((self.base() + 0x1d8) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1d8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the HSSR register."]
   #[inline] fn hssr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x1f4)
   }
#[doc="Get the *mut pointer for the HSSR register."]
   #[inline] fn hssr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x1f4)
   }
#[doc="Read the HSSR register."]
   #[inline] fn hssr(&self) -> Hssr { 
      unsafe {
         Hssr(::core::ptr::read_volatile((self.base() + 0x1f4) as *const u32))
      }
   }
#[doc="Write the HSSR register."]
   #[inline] fn set_hssr<F: FnOnce(Hssr) -> Hssr>(&self, f: F) -> &Self {
      let value = f(Hssr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1f4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the HSSR register."]
   #[inline] fn with_hssr<F: FnOnce(Hssr) -> Hssr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Hssr(::core::ptr::read_volatile((self.base() + 0x1f4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1f4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the USBPDS register."]
   #[inline] fn usbpds_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x280)
   }
#[doc="Get the *mut pointer for the USBPDS register."]
   #[inline] fn usbpds_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x280)
   }
#[doc="Read the USBPDS register."]
   #[inline] fn usbpds(&self) -> Usbpds { 
      unsafe {
         Usbpds(::core::ptr::read_volatile((self.base() + 0x280) as *const u32))
      }
   }
#[doc="Write the USBPDS register."]
   #[inline] fn set_usbpds<F: FnOnce(Usbpds) -> Usbpds>(&self, f: F) -> &Self {
      let value = f(Usbpds(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x280) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the USBPDS register."]
   #[inline] fn with_usbpds<F: FnOnce(Usbpds) -> Usbpds>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Usbpds(::core::ptr::read_volatile((self.base() + 0x280) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x280) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the USBMPC register."]
   #[inline] fn usbmpc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x284)
   }
#[doc="Get the *mut pointer for the USBMPC register."]
   #[inline] fn usbmpc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x284)
   }
#[doc="Read the USBMPC register."]
   #[inline] fn usbmpc(&self) -> Usbmpc { 
      unsafe {
         Usbmpc(::core::ptr::read_volatile((self.base() + 0x284) as *const u32))
      }
   }
#[doc="Write the USBMPC register."]
   #[inline] fn set_usbmpc<F: FnOnce(Usbmpc) -> Usbmpc>(&self, f: F) -> &Self {
      let value = f(Usbmpc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x284) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the USBMPC register."]
   #[inline] fn with_usbmpc<F: FnOnce(Usbmpc) -> Usbmpc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Usbmpc(::core::ptr::read_volatile((self.base() + 0x284) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x284) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EMACPDS register."]
   #[inline] fn emacpds_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x288)
   }
#[doc="Get the *mut pointer for the EMACPDS register."]
   #[inline] fn emacpds_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x288)
   }
#[doc="Read the EMACPDS register."]
   #[inline] fn emacpds(&self) -> Emacpds { 
      unsafe {
         Emacpds(::core::ptr::read_volatile((self.base() + 0x288) as *const u32))
      }
   }
#[doc="Write the EMACPDS register."]
   #[inline] fn set_emacpds<F: FnOnce(Emacpds) -> Emacpds>(&self, f: F) -> &Self {
      let value = f(Emacpds(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x288) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the EMACPDS register."]
   #[inline] fn with_emacpds<F: FnOnce(Emacpds) -> Emacpds>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Emacpds(::core::ptr::read_volatile((self.base() + 0x288) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x288) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the EMACMPC register."]
   #[inline] fn emacmpc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x28c)
   }
#[doc="Get the *mut pointer for the EMACMPC register."]
   #[inline] fn emacmpc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x28c)
   }
#[doc="Read the EMACMPC register."]
   #[inline] fn emacmpc(&self) -> Emacmpc { 
      unsafe {
         Emacmpc(::core::ptr::read_volatile((self.base() + 0x28c) as *const u32))
      }
   }
#[doc="Write the EMACMPC register."]
   #[inline] fn set_emacmpc<F: FnOnce(Emacmpc) -> Emacmpc>(&self, f: F) -> &Self {
      let value = f(Emacmpc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x28c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the EMACMPC register."]
   #[inline] fn with_emacmpc<F: FnOnce(Emacmpc) -> Emacmpc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Emacmpc(::core::ptr::read_volatile((self.base() + 0x28c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x28c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPWD register."]
   #[inline] fn ppwd_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x300)
   }
#[doc="Get the *mut pointer for the PPWD register."]
   #[inline] fn ppwd_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x300)
   }
#[doc="Read the PPWD register."]
   #[inline] fn ppwd(&self) -> Ppwd { 
      unsafe {
         Ppwd(::core::ptr::read_volatile((self.base() + 0x300) as *const u32))
      }
   }
#[doc="Write the PPWD register."]
   #[inline] fn set_ppwd<F: FnOnce(Ppwd) -> Ppwd>(&self, f: F) -> &Self {
      let value = f(Ppwd(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x300) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPWD register."]
   #[inline] fn with_ppwd<F: FnOnce(Ppwd) -> Ppwd>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppwd(::core::ptr::read_volatile((self.base() + 0x300) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x300) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPTIMER register."]
   #[inline] fn pptimer_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x304)
   }
#[doc="Get the *mut pointer for the PPTIMER register."]
   #[inline] fn pptimer_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x304)
   }
#[doc="Read the PPTIMER register."]
   #[inline] fn pptimer(&self) -> Pptimer { 
      unsafe {
         Pptimer(::core::ptr::read_volatile((self.base() + 0x304) as *const u32))
      }
   }
#[doc="Write the PPTIMER register."]
   #[inline] fn set_pptimer<F: FnOnce(Pptimer) -> Pptimer>(&self, f: F) -> &Self {
      let value = f(Pptimer(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x304) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPTIMER register."]
   #[inline] fn with_pptimer<F: FnOnce(Pptimer) -> Pptimer>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pptimer(::core::ptr::read_volatile((self.base() + 0x304) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x304) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPGPIO register."]
   #[inline] fn ppgpio_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x308)
   }
#[doc="Get the *mut pointer for the PPGPIO register."]
   #[inline] fn ppgpio_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x308)
   }
#[doc="Read the PPGPIO register."]
   #[inline] fn ppgpio(&self) -> Ppgpio { 
      unsafe {
         Ppgpio(::core::ptr::read_volatile((self.base() + 0x308) as *const u32))
      }
   }
#[doc="Write the PPGPIO register."]
   #[inline] fn set_ppgpio<F: FnOnce(Ppgpio) -> Ppgpio>(&self, f: F) -> &Self {
      let value = f(Ppgpio(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x308) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPGPIO register."]
   #[inline] fn with_ppgpio<F: FnOnce(Ppgpio) -> Ppgpio>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppgpio(::core::ptr::read_volatile((self.base() + 0x308) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x308) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPDMA register."]
   #[inline] fn ppdma_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x30c)
   }
#[doc="Get the *mut pointer for the PPDMA register."]
   #[inline] fn ppdma_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x30c)
   }
#[doc="Read the PPDMA register."]
   #[inline] fn ppdma(&self) -> Ppdma { 
      unsafe {
         Ppdma(::core::ptr::read_volatile((self.base() + 0x30c) as *const u32))
      }
   }
#[doc="Write the PPDMA register."]
   #[inline] fn set_ppdma<F: FnOnce(Ppdma) -> Ppdma>(&self, f: F) -> &Self {
      let value = f(Ppdma(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x30c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPDMA register."]
   #[inline] fn with_ppdma<F: FnOnce(Ppdma) -> Ppdma>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppdma(::core::ptr::read_volatile((self.base() + 0x30c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x30c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPEPI register."]
   #[inline] fn ppepi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x310)
   }
#[doc="Get the *mut pointer for the PPEPI register."]
   #[inline] fn ppepi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x310)
   }
#[doc="Read the PPEPI register."]
   #[inline] fn ppepi(&self) -> Ppepi { 
      unsafe {
         Ppepi(::core::ptr::read_volatile((self.base() + 0x310) as *const u32))
      }
   }
#[doc="Write the PPEPI register."]
   #[inline] fn set_ppepi<F: FnOnce(Ppepi) -> Ppepi>(&self, f: F) -> &Self {
      let value = f(Ppepi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x310) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPEPI register."]
   #[inline] fn with_ppepi<F: FnOnce(Ppepi) -> Ppepi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppepi(::core::ptr::read_volatile((self.base() + 0x310) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x310) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPHIB register."]
   #[inline] fn pphib_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x314)
   }
#[doc="Get the *mut pointer for the PPHIB register."]
   #[inline] fn pphib_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x314)
   }
#[doc="Read the PPHIB register."]
   #[inline] fn pphib(&self) -> Pphib { 
      unsafe {
         Pphib(::core::ptr::read_volatile((self.base() + 0x314) as *const u32))
      }
   }
#[doc="Write the PPHIB register."]
   #[inline] fn set_pphib<F: FnOnce(Pphib) -> Pphib>(&self, f: F) -> &Self {
      let value = f(Pphib(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x314) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPHIB register."]
   #[inline] fn with_pphib<F: FnOnce(Pphib) -> Pphib>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pphib(::core::ptr::read_volatile((self.base() + 0x314) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x314) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPUART register."]
   #[inline] fn ppuart_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x318)
   }
#[doc="Get the *mut pointer for the PPUART register."]
   #[inline] fn ppuart_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x318)
   }
#[doc="Read the PPUART register."]
   #[inline] fn ppuart(&self) -> Ppuart { 
      unsafe {
         Ppuart(::core::ptr::read_volatile((self.base() + 0x318) as *const u32))
      }
   }
#[doc="Write the PPUART register."]
   #[inline] fn set_ppuart<F: FnOnce(Ppuart) -> Ppuart>(&self, f: F) -> &Self {
      let value = f(Ppuart(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x318) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPUART register."]
   #[inline] fn with_ppuart<F: FnOnce(Ppuart) -> Ppuart>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppuart(::core::ptr::read_volatile((self.base() + 0x318) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x318) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPSSI register."]
   #[inline] fn ppssi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x31c)
   }
#[doc="Get the *mut pointer for the PPSSI register."]
   #[inline] fn ppssi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x31c)
   }
#[doc="Read the PPSSI register."]
   #[inline] fn ppssi(&self) -> Ppssi { 
      unsafe {
         Ppssi(::core::ptr::read_volatile((self.base() + 0x31c) as *const u32))
      }
   }
#[doc="Write the PPSSI register."]
   #[inline] fn set_ppssi<F: FnOnce(Ppssi) -> Ppssi>(&self, f: F) -> &Self {
      let value = f(Ppssi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x31c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPSSI register."]
   #[inline] fn with_ppssi<F: FnOnce(Ppssi) -> Ppssi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppssi(::core::ptr::read_volatile((self.base() + 0x31c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x31c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPI2C register."]
   #[inline] fn ppi2c_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x320)
   }
#[doc="Get the *mut pointer for the PPI2C register."]
   #[inline] fn ppi2c_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x320)
   }
#[doc="Read the PPI2C register."]
   #[inline] fn ppi2c(&self) -> Ppi2c { 
      unsafe {
         Ppi2c(::core::ptr::read_volatile((self.base() + 0x320) as *const u32))
      }
   }
#[doc="Write the PPI2C register."]
   #[inline] fn set_ppi2c<F: FnOnce(Ppi2c) -> Ppi2c>(&self, f: F) -> &Self {
      let value = f(Ppi2c(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x320) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPI2C register."]
   #[inline] fn with_ppi2c<F: FnOnce(Ppi2c) -> Ppi2c>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppi2c(::core::ptr::read_volatile((self.base() + 0x320) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x320) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPUSB register."]
   #[inline] fn ppusb_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x328)
   }
#[doc="Get the *mut pointer for the PPUSB register."]
   #[inline] fn ppusb_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x328)
   }
#[doc="Read the PPUSB register."]
   #[inline] fn ppusb(&self) -> Ppusb { 
      unsafe {
         Ppusb(::core::ptr::read_volatile((self.base() + 0x328) as *const u32))
      }
   }
#[doc="Write the PPUSB register."]
   #[inline] fn set_ppusb<F: FnOnce(Ppusb) -> Ppusb>(&self, f: F) -> &Self {
      let value = f(Ppusb(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x328) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPUSB register."]
   #[inline] fn with_ppusb<F: FnOnce(Ppusb) -> Ppusb>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppusb(::core::ptr::read_volatile((self.base() + 0x328) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x328) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPEPHY register."]
   #[inline] fn ppephy_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x330)
   }
#[doc="Get the *mut pointer for the PPEPHY register."]
   #[inline] fn ppephy_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x330)
   }
#[doc="Read the PPEPHY register."]
   #[inline] fn ppephy(&self) -> Ppephy { 
      unsafe {
         Ppephy(::core::ptr::read_volatile((self.base() + 0x330) as *const u32))
      }
   }
#[doc="Write the PPEPHY register."]
   #[inline] fn set_ppephy<F: FnOnce(Ppephy) -> Ppephy>(&self, f: F) -> &Self {
      let value = f(Ppephy(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x330) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPEPHY register."]
   #[inline] fn with_ppephy<F: FnOnce(Ppephy) -> Ppephy>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppephy(::core::ptr::read_volatile((self.base() + 0x330) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x330) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPCAN register."]
   #[inline] fn ppcan_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x334)
   }
#[doc="Get the *mut pointer for the PPCAN register."]
   #[inline] fn ppcan_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x334)
   }
#[doc="Read the PPCAN register."]
   #[inline] fn ppcan(&self) -> Ppcan { 
      unsafe {
         Ppcan(::core::ptr::read_volatile((self.base() + 0x334) as *const u32))
      }
   }
#[doc="Write the PPCAN register."]
   #[inline] fn set_ppcan<F: FnOnce(Ppcan) -> Ppcan>(&self, f: F) -> &Self {
      let value = f(Ppcan(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x334) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPCAN register."]
   #[inline] fn with_ppcan<F: FnOnce(Ppcan) -> Ppcan>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppcan(::core::ptr::read_volatile((self.base() + 0x334) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x334) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPADC register."]
   #[inline] fn ppadc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x338)
   }
#[doc="Get the *mut pointer for the PPADC register."]
   #[inline] fn ppadc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x338)
   }
#[doc="Read the PPADC register."]
   #[inline] fn ppadc(&self) -> Ppadc { 
      unsafe {
         Ppadc(::core::ptr::read_volatile((self.base() + 0x338) as *const u32))
      }
   }
#[doc="Write the PPADC register."]
   #[inline] fn set_ppadc<F: FnOnce(Ppadc) -> Ppadc>(&self, f: F) -> &Self {
      let value = f(Ppadc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x338) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPADC register."]
   #[inline] fn with_ppadc<F: FnOnce(Ppadc) -> Ppadc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppadc(::core::ptr::read_volatile((self.base() + 0x338) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x338) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPACMP register."]
   #[inline] fn ppacmp_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x33c)
   }
#[doc="Get the *mut pointer for the PPACMP register."]
   #[inline] fn ppacmp_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x33c)
   }
#[doc="Read the PPACMP register."]
   #[inline] fn ppacmp(&self) -> Ppacmp { 
      unsafe {
         Ppacmp(::core::ptr::read_volatile((self.base() + 0x33c) as *const u32))
      }
   }
#[doc="Write the PPACMP register."]
   #[inline] fn set_ppacmp<F: FnOnce(Ppacmp) -> Ppacmp>(&self, f: F) -> &Self {
      let value = f(Ppacmp(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x33c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPACMP register."]
   #[inline] fn with_ppacmp<F: FnOnce(Ppacmp) -> Ppacmp>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppacmp(::core::ptr::read_volatile((self.base() + 0x33c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x33c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPPWM register."]
   #[inline] fn pppwm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x340)
   }
#[doc="Get the *mut pointer for the PPPWM register."]
   #[inline] fn pppwm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x340)
   }
#[doc="Read the PPPWM register."]
   #[inline] fn pppwm(&self) -> Pppwm { 
      unsafe {
         Pppwm(::core::ptr::read_volatile((self.base() + 0x340) as *const u32))
      }
   }
#[doc="Write the PPPWM register."]
   #[inline] fn set_pppwm<F: FnOnce(Pppwm) -> Pppwm>(&self, f: F) -> &Self {
      let value = f(Pppwm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x340) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPPWM register."]
   #[inline] fn with_pppwm<F: FnOnce(Pppwm) -> Pppwm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pppwm(::core::ptr::read_volatile((self.base() + 0x340) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x340) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPQEI register."]
   #[inline] fn ppqei_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x344)
   }
#[doc="Get the *mut pointer for the PPQEI register."]
   #[inline] fn ppqei_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x344)
   }
#[doc="Read the PPQEI register."]
   #[inline] fn ppqei(&self) -> Ppqei { 
      unsafe {
         Ppqei(::core::ptr::read_volatile((self.base() + 0x344) as *const u32))
      }
   }
#[doc="Write the PPQEI register."]
   #[inline] fn set_ppqei<F: FnOnce(Ppqei) -> Ppqei>(&self, f: F) -> &Self {
      let value = f(Ppqei(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x344) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPQEI register."]
   #[inline] fn with_ppqei<F: FnOnce(Ppqei) -> Ppqei>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppqei(::core::ptr::read_volatile((self.base() + 0x344) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x344) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPLPC register."]
   #[inline] fn pplpc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x348)
   }
#[doc="Get the *mut pointer for the PPLPC register."]
   #[inline] fn pplpc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x348)
   }
#[doc="Read the PPLPC register."]
   #[inline] fn pplpc(&self) -> Pplpc { 
      unsafe {
         Pplpc(::core::ptr::read_volatile((self.base() + 0x348) as *const u32))
      }
   }
#[doc="Write the PPLPC register."]
   #[inline] fn set_pplpc<F: FnOnce(Pplpc) -> Pplpc>(&self, f: F) -> &Self {
      let value = f(Pplpc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x348) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPLPC register."]
   #[inline] fn with_pplpc<F: FnOnce(Pplpc) -> Pplpc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pplpc(::core::ptr::read_volatile((self.base() + 0x348) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x348) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPPECI register."]
   #[inline] fn pppeci_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x350)
   }
#[doc="Get the *mut pointer for the PPPECI register."]
   #[inline] fn pppeci_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x350)
   }
#[doc="Read the PPPECI register."]
   #[inline] fn pppeci(&self) -> Pppeci { 
      unsafe {
         Pppeci(::core::ptr::read_volatile((self.base() + 0x350) as *const u32))
      }
   }
#[doc="Write the PPPECI register."]
   #[inline] fn set_pppeci<F: FnOnce(Pppeci) -> Pppeci>(&self, f: F) -> &Self {
      let value = f(Pppeci(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x350) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPPECI register."]
   #[inline] fn with_pppeci<F: FnOnce(Pppeci) -> Pppeci>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pppeci(::core::ptr::read_volatile((self.base() + 0x350) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x350) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPFAN register."]
   #[inline] fn ppfan_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x354)
   }
#[doc="Get the *mut pointer for the PPFAN register."]
   #[inline] fn ppfan_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x354)
   }
#[doc="Read the PPFAN register."]
   #[inline] fn ppfan(&self) -> Ppfan { 
      unsafe {
         Ppfan(::core::ptr::read_volatile((self.base() + 0x354) as *const u32))
      }
   }
#[doc="Write the PPFAN register."]
   #[inline] fn set_ppfan<F: FnOnce(Ppfan) -> Ppfan>(&self, f: F) -> &Self {
      let value = f(Ppfan(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x354) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPFAN register."]
   #[inline] fn with_ppfan<F: FnOnce(Ppfan) -> Ppfan>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppfan(::core::ptr::read_volatile((self.base() + 0x354) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x354) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPEEPROM register."]
   #[inline] fn ppeeprom_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x358)
   }
#[doc="Get the *mut pointer for the PPEEPROM register."]
   #[inline] fn ppeeprom_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x358)
   }
#[doc="Read the PPEEPROM register."]
   #[inline] fn ppeeprom(&self) -> Ppeeprom { 
      unsafe {
         Ppeeprom(::core::ptr::read_volatile((self.base() + 0x358) as *const u32))
      }
   }
#[doc="Write the PPEEPROM register."]
   #[inline] fn set_ppeeprom<F: FnOnce(Ppeeprom) -> Ppeeprom>(&self, f: F) -> &Self {
      let value = f(Ppeeprom(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x358) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPEEPROM register."]
   #[inline] fn with_ppeeprom<F: FnOnce(Ppeeprom) -> Ppeeprom>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppeeprom(::core::ptr::read_volatile((self.base() + 0x358) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x358) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPWTIMER register."]
   #[inline] fn ppwtimer_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x35c)
   }
#[doc="Get the *mut pointer for the PPWTIMER register."]
   #[inline] fn ppwtimer_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x35c)
   }
#[doc="Read the PPWTIMER register."]
   #[inline] fn ppwtimer(&self) -> Ppwtimer { 
      unsafe {
         Ppwtimer(::core::ptr::read_volatile((self.base() + 0x35c) as *const u32))
      }
   }
#[doc="Write the PPWTIMER register."]
   #[inline] fn set_ppwtimer<F: FnOnce(Ppwtimer) -> Ppwtimer>(&self, f: F) -> &Self {
      let value = f(Ppwtimer(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x35c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPWTIMER register."]
   #[inline] fn with_ppwtimer<F: FnOnce(Ppwtimer) -> Ppwtimer>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppwtimer(::core::ptr::read_volatile((self.base() + 0x35c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x35c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPRTS register."]
   #[inline] fn pprts_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x370)
   }
#[doc="Get the *mut pointer for the PPRTS register."]
   #[inline] fn pprts_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x370)
   }
#[doc="Read the PPRTS register."]
   #[inline] fn pprts(&self) -> Pprts { 
      unsafe {
         Pprts(::core::ptr::read_volatile((self.base() + 0x370) as *const u32))
      }
   }
#[doc="Write the PPRTS register."]
   #[inline] fn set_pprts<F: FnOnce(Pprts) -> Pprts>(&self, f: F) -> &Self {
      let value = f(Pprts(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x370) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPRTS register."]
   #[inline] fn with_pprts<F: FnOnce(Pprts) -> Pprts>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pprts(::core::ptr::read_volatile((self.base() + 0x370) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x370) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPCCM register."]
   #[inline] fn ppccm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x374)
   }
#[doc="Get the *mut pointer for the PPCCM register."]
   #[inline] fn ppccm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x374)
   }
#[doc="Read the PPCCM register."]
   #[inline] fn ppccm(&self) -> Ppccm { 
      unsafe {
         Ppccm(::core::ptr::read_volatile((self.base() + 0x374) as *const u32))
      }
   }
#[doc="Write the PPCCM register."]
   #[inline] fn set_ppccm<F: FnOnce(Ppccm) -> Ppccm>(&self, f: F) -> &Self {
      let value = f(Ppccm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x374) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPCCM register."]
   #[inline] fn with_ppccm<F: FnOnce(Ppccm) -> Ppccm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppccm(::core::ptr::read_volatile((self.base() + 0x374) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x374) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPLCD register."]
   #[inline] fn pplcd_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x390)
   }
#[doc="Get the *mut pointer for the PPLCD register."]
   #[inline] fn pplcd_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x390)
   }
#[doc="Read the PPLCD register."]
   #[inline] fn pplcd(&self) -> Pplcd { 
      unsafe {
         Pplcd(::core::ptr::read_volatile((self.base() + 0x390) as *const u32))
      }
   }
#[doc="Write the PPLCD register."]
   #[inline] fn set_pplcd<F: FnOnce(Pplcd) -> Pplcd>(&self, f: F) -> &Self {
      let value = f(Pplcd(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x390) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPLCD register."]
   #[inline] fn with_pplcd<F: FnOnce(Pplcd) -> Pplcd>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pplcd(::core::ptr::read_volatile((self.base() + 0x390) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x390) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPOWIRE register."]
   #[inline] fn ppowire_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x398)
   }
#[doc="Get the *mut pointer for the PPOWIRE register."]
   #[inline] fn ppowire_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x398)
   }
#[doc="Read the PPOWIRE register."]
   #[inline] fn ppowire(&self) -> Ppowire { 
      unsafe {
         Ppowire(::core::ptr::read_volatile((self.base() + 0x398) as *const u32))
      }
   }
#[doc="Write the PPOWIRE register."]
   #[inline] fn set_ppowire<F: FnOnce(Ppowire) -> Ppowire>(&self, f: F) -> &Self {
      let value = f(Ppowire(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x398) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPOWIRE register."]
   #[inline] fn with_ppowire<F: FnOnce(Ppowire) -> Ppowire>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppowire(::core::ptr::read_volatile((self.base() + 0x398) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x398) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPEMAC register."]
   #[inline] fn ppemac_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x39c)
   }
#[doc="Get the *mut pointer for the PPEMAC register."]
   #[inline] fn ppemac_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x39c)
   }
#[doc="Read the PPEMAC register."]
   #[inline] fn ppemac(&self) -> Ppemac { 
      unsafe {
         Ppemac(::core::ptr::read_volatile((self.base() + 0x39c) as *const u32))
      }
   }
#[doc="Write the PPEMAC register."]
   #[inline] fn set_ppemac<F: FnOnce(Ppemac) -> Ppemac>(&self, f: F) -> &Self {
      let value = f(Ppemac(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x39c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPEMAC register."]
   #[inline] fn with_ppemac<F: FnOnce(Ppemac) -> Ppemac>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ppemac(::core::ptr::read_volatile((self.base() + 0x39c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x39c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PPHIM register."]
   #[inline] fn pphim_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x3a4)
   }
#[doc="Get the *mut pointer for the PPHIM register."]
   #[inline] fn pphim_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x3a4)
   }
#[doc="Read the PPHIM register."]
   #[inline] fn pphim(&self) -> Pphim { 
      unsafe {
         Pphim(::core::ptr::read_volatile((self.base() + 0x3a4) as *const u32))
      }
   }
#[doc="Write the PPHIM register."]
   #[inline] fn set_pphim<F: FnOnce(Pphim) -> Pphim>(&self, f: F) -> &Self {
      let value = f(Pphim(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x3a4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PPHIM register."]
   #[inline] fn with_pphim<F: FnOnce(Pphim) -> Pphim>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pphim(::core::ptr::read_volatile((self.base() + 0x3a4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x3a4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRWD register."]
   #[inline] fn srwd_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x500)
   }
#[doc="Get the *mut pointer for the SRWD register."]
   #[inline] fn srwd_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x500)
   }
#[doc="Read the SRWD register."]
   #[inline] fn srwd(&self) -> Srwd { 
      unsafe {
         Srwd(::core::ptr::read_volatile((self.base() + 0x500) as *const u32))
      }
   }
#[doc="Write the SRWD register."]
   #[inline] fn set_srwd<F: FnOnce(Srwd) -> Srwd>(&self, f: F) -> &Self {
      let value = f(Srwd(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x500) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRWD register."]
   #[inline] fn with_srwd<F: FnOnce(Srwd) -> Srwd>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srwd(::core::ptr::read_volatile((self.base() + 0x500) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x500) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRTIMER register."]
   #[inline] fn srtimer_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x504)
   }
#[doc="Get the *mut pointer for the SRTIMER register."]
   #[inline] fn srtimer_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x504)
   }
#[doc="Read the SRTIMER register."]
   #[inline] fn srtimer(&self) -> Srtimer { 
      unsafe {
         Srtimer(::core::ptr::read_volatile((self.base() + 0x504) as *const u32))
      }
   }
#[doc="Write the SRTIMER register."]
   #[inline] fn set_srtimer<F: FnOnce(Srtimer) -> Srtimer>(&self, f: F) -> &Self {
      let value = f(Srtimer(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x504) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRTIMER register."]
   #[inline] fn with_srtimer<F: FnOnce(Srtimer) -> Srtimer>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srtimer(::core::ptr::read_volatile((self.base() + 0x504) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x504) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRGPIO register."]
   #[inline] fn srgpio_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x508)
   }
#[doc="Get the *mut pointer for the SRGPIO register."]
   #[inline] fn srgpio_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x508)
   }
#[doc="Read the SRGPIO register."]
   #[inline] fn srgpio(&self) -> Srgpio { 
      unsafe {
         Srgpio(::core::ptr::read_volatile((self.base() + 0x508) as *const u32))
      }
   }
#[doc="Write the SRGPIO register."]
   #[inline] fn set_srgpio<F: FnOnce(Srgpio) -> Srgpio>(&self, f: F) -> &Self {
      let value = f(Srgpio(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x508) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRGPIO register."]
   #[inline] fn with_srgpio<F: FnOnce(Srgpio) -> Srgpio>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srgpio(::core::ptr::read_volatile((self.base() + 0x508) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x508) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRDMA register."]
   #[inline] fn srdma_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x50c)
   }
#[doc="Get the *mut pointer for the SRDMA register."]
   #[inline] fn srdma_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x50c)
   }
#[doc="Read the SRDMA register."]
   #[inline] fn srdma(&self) -> Srdma { 
      unsafe {
         Srdma(::core::ptr::read_volatile((self.base() + 0x50c) as *const u32))
      }
   }
#[doc="Write the SRDMA register."]
   #[inline] fn set_srdma<F: FnOnce(Srdma) -> Srdma>(&self, f: F) -> &Self {
      let value = f(Srdma(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x50c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRDMA register."]
   #[inline] fn with_srdma<F: FnOnce(Srdma) -> Srdma>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srdma(::core::ptr::read_volatile((self.base() + 0x50c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x50c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SREPI register."]
   #[inline] fn srepi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x510)
   }
#[doc="Get the *mut pointer for the SREPI register."]
   #[inline] fn srepi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x510)
   }
#[doc="Read the SREPI register."]
   #[inline] fn srepi(&self) -> Srepi { 
      unsafe {
         Srepi(::core::ptr::read_volatile((self.base() + 0x510) as *const u32))
      }
   }
#[doc="Write the SREPI register."]
   #[inline] fn set_srepi<F: FnOnce(Srepi) -> Srepi>(&self, f: F) -> &Self {
      let value = f(Srepi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x510) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SREPI register."]
   #[inline] fn with_srepi<F: FnOnce(Srepi) -> Srepi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srepi(::core::ptr::read_volatile((self.base() + 0x510) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x510) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRHIB register."]
   #[inline] fn srhib_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x514)
   }
#[doc="Get the *mut pointer for the SRHIB register."]
   #[inline] fn srhib_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x514)
   }
#[doc="Read the SRHIB register."]
   #[inline] fn srhib(&self) -> Srhib { 
      unsafe {
         Srhib(::core::ptr::read_volatile((self.base() + 0x514) as *const u32))
      }
   }
#[doc="Write the SRHIB register."]
   #[inline] fn set_srhib<F: FnOnce(Srhib) -> Srhib>(&self, f: F) -> &Self {
      let value = f(Srhib(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x514) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRHIB register."]
   #[inline] fn with_srhib<F: FnOnce(Srhib) -> Srhib>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srhib(::core::ptr::read_volatile((self.base() + 0x514) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x514) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRUART register."]
   #[inline] fn sruart_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x518)
   }
#[doc="Get the *mut pointer for the SRUART register."]
   #[inline] fn sruart_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x518)
   }
#[doc="Read the SRUART register."]
   #[inline] fn sruart(&self) -> Sruart { 
      unsafe {
         Sruart(::core::ptr::read_volatile((self.base() + 0x518) as *const u32))
      }
   }
#[doc="Write the SRUART register."]
   #[inline] fn set_sruart<F: FnOnce(Sruart) -> Sruart>(&self, f: F) -> &Self {
      let value = f(Sruart(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x518) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRUART register."]
   #[inline] fn with_sruart<F: FnOnce(Sruart) -> Sruart>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Sruart(::core::ptr::read_volatile((self.base() + 0x518) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x518) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRSSI register."]
   #[inline] fn srssi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x51c)
   }
#[doc="Get the *mut pointer for the SRSSI register."]
   #[inline] fn srssi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x51c)
   }
#[doc="Read the SRSSI register."]
   #[inline] fn srssi(&self) -> Srssi { 
      unsafe {
         Srssi(::core::ptr::read_volatile((self.base() + 0x51c) as *const u32))
      }
   }
#[doc="Write the SRSSI register."]
   #[inline] fn set_srssi<F: FnOnce(Srssi) -> Srssi>(&self, f: F) -> &Self {
      let value = f(Srssi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x51c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRSSI register."]
   #[inline] fn with_srssi<F: FnOnce(Srssi) -> Srssi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srssi(::core::ptr::read_volatile((self.base() + 0x51c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x51c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRI2C register."]
   #[inline] fn sri2c_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x520)
   }
#[doc="Get the *mut pointer for the SRI2C register."]
   #[inline] fn sri2c_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x520)
   }
#[doc="Read the SRI2C register."]
   #[inline] fn sri2c(&self) -> Sri2c { 
      unsafe {
         Sri2c(::core::ptr::read_volatile((self.base() + 0x520) as *const u32))
      }
   }
#[doc="Write the SRI2C register."]
   #[inline] fn set_sri2c<F: FnOnce(Sri2c) -> Sri2c>(&self, f: F) -> &Self {
      let value = f(Sri2c(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x520) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRI2C register."]
   #[inline] fn with_sri2c<F: FnOnce(Sri2c) -> Sri2c>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Sri2c(::core::ptr::read_volatile((self.base() + 0x520) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x520) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRUSB register."]
   #[inline] fn srusb_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x528)
   }
#[doc="Get the *mut pointer for the SRUSB register."]
   #[inline] fn srusb_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x528)
   }
#[doc="Read the SRUSB register."]
   #[inline] fn srusb(&self) -> Srusb { 
      unsafe {
         Srusb(::core::ptr::read_volatile((self.base() + 0x528) as *const u32))
      }
   }
#[doc="Write the SRUSB register."]
   #[inline] fn set_srusb<F: FnOnce(Srusb) -> Srusb>(&self, f: F) -> &Self {
      let value = f(Srusb(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x528) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRUSB register."]
   #[inline] fn with_srusb<F: FnOnce(Srusb) -> Srusb>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srusb(::core::ptr::read_volatile((self.base() + 0x528) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x528) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SREPHY register."]
   #[inline] fn srephy_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x530)
   }
#[doc="Get the *mut pointer for the SREPHY register."]
   #[inline] fn srephy_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x530)
   }
#[doc="Read the SREPHY register."]
   #[inline] fn srephy(&self) -> Srephy { 
      unsafe {
         Srephy(::core::ptr::read_volatile((self.base() + 0x530) as *const u32))
      }
   }
#[doc="Write the SREPHY register."]
   #[inline] fn set_srephy<F: FnOnce(Srephy) -> Srephy>(&self, f: F) -> &Self {
      let value = f(Srephy(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x530) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SREPHY register."]
   #[inline] fn with_srephy<F: FnOnce(Srephy) -> Srephy>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srephy(::core::ptr::read_volatile((self.base() + 0x530) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x530) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRCAN register."]
   #[inline] fn srcan_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x534)
   }
#[doc="Get the *mut pointer for the SRCAN register."]
   #[inline] fn srcan_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x534)
   }
#[doc="Read the SRCAN register."]
   #[inline] fn srcan(&self) -> Srcan { 
      unsafe {
         Srcan(::core::ptr::read_volatile((self.base() + 0x534) as *const u32))
      }
   }
#[doc="Write the SRCAN register."]
   #[inline] fn set_srcan<F: FnOnce(Srcan) -> Srcan>(&self, f: F) -> &Self {
      let value = f(Srcan(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x534) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRCAN register."]
   #[inline] fn with_srcan<F: FnOnce(Srcan) -> Srcan>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srcan(::core::ptr::read_volatile((self.base() + 0x534) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x534) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRADC register."]
   #[inline] fn sradc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x538)
   }
#[doc="Get the *mut pointer for the SRADC register."]
   #[inline] fn sradc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x538)
   }
#[doc="Read the SRADC register."]
   #[inline] fn sradc(&self) -> Sradc { 
      unsafe {
         Sradc(::core::ptr::read_volatile((self.base() + 0x538) as *const u32))
      }
   }
#[doc="Write the SRADC register."]
   #[inline] fn set_sradc<F: FnOnce(Sradc) -> Sradc>(&self, f: F) -> &Self {
      let value = f(Sradc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x538) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRADC register."]
   #[inline] fn with_sradc<F: FnOnce(Sradc) -> Sradc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Sradc(::core::ptr::read_volatile((self.base() + 0x538) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x538) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRACMP register."]
   #[inline] fn sracmp_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x53c)
   }
#[doc="Get the *mut pointer for the SRACMP register."]
   #[inline] fn sracmp_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x53c)
   }
#[doc="Read the SRACMP register."]
   #[inline] fn sracmp(&self) -> Sracmp { 
      unsafe {
         Sracmp(::core::ptr::read_volatile((self.base() + 0x53c) as *const u32))
      }
   }
#[doc="Write the SRACMP register."]
   #[inline] fn set_sracmp<F: FnOnce(Sracmp) -> Sracmp>(&self, f: F) -> &Self {
      let value = f(Sracmp(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x53c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRACMP register."]
   #[inline] fn with_sracmp<F: FnOnce(Sracmp) -> Sracmp>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Sracmp(::core::ptr::read_volatile((self.base() + 0x53c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x53c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRPWM register."]
   #[inline] fn srpwm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x540)
   }
#[doc="Get the *mut pointer for the SRPWM register."]
   #[inline] fn srpwm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x540)
   }
#[doc="Read the SRPWM register."]
   #[inline] fn srpwm(&self) -> Srpwm { 
      unsafe {
         Srpwm(::core::ptr::read_volatile((self.base() + 0x540) as *const u32))
      }
   }
#[doc="Write the SRPWM register."]
   #[inline] fn set_srpwm<F: FnOnce(Srpwm) -> Srpwm>(&self, f: F) -> &Self {
      let value = f(Srpwm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x540) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRPWM register."]
   #[inline] fn with_srpwm<F: FnOnce(Srpwm) -> Srpwm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srpwm(::core::ptr::read_volatile((self.base() + 0x540) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x540) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRQEI register."]
   #[inline] fn srqei_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x544)
   }
#[doc="Get the *mut pointer for the SRQEI register."]
   #[inline] fn srqei_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x544)
   }
#[doc="Read the SRQEI register."]
   #[inline] fn srqei(&self) -> Srqei { 
      unsafe {
         Srqei(::core::ptr::read_volatile((self.base() + 0x544) as *const u32))
      }
   }
#[doc="Write the SRQEI register."]
   #[inline] fn set_srqei<F: FnOnce(Srqei) -> Srqei>(&self, f: F) -> &Self {
      let value = f(Srqei(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x544) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRQEI register."]
   #[inline] fn with_srqei<F: FnOnce(Srqei) -> Srqei>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srqei(::core::ptr::read_volatile((self.base() + 0x544) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x544) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SREEPROM register."]
   #[inline] fn sreeprom_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x558)
   }
#[doc="Get the *mut pointer for the SREEPROM register."]
   #[inline] fn sreeprom_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x558)
   }
#[doc="Read the SREEPROM register."]
   #[inline] fn sreeprom(&self) -> Sreeprom { 
      unsafe {
         Sreeprom(::core::ptr::read_volatile((self.base() + 0x558) as *const u32))
      }
   }
#[doc="Write the SREEPROM register."]
   #[inline] fn set_sreeprom<F: FnOnce(Sreeprom) -> Sreeprom>(&self, f: F) -> &Self {
      let value = f(Sreeprom(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x558) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SREEPROM register."]
   #[inline] fn with_sreeprom<F: FnOnce(Sreeprom) -> Sreeprom>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Sreeprom(::core::ptr::read_volatile((self.base() + 0x558) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x558) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SRCCM register."]
   #[inline] fn srccm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x574)
   }
#[doc="Get the *mut pointer for the SRCCM register."]
   #[inline] fn srccm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x574)
   }
#[doc="Read the SRCCM register."]
   #[inline] fn srccm(&self) -> Srccm { 
      unsafe {
         Srccm(::core::ptr::read_volatile((self.base() + 0x574) as *const u32))
      }
   }
#[doc="Write the SRCCM register."]
   #[inline] fn set_srccm<F: FnOnce(Srccm) -> Srccm>(&self, f: F) -> &Self {
      let value = f(Srccm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x574) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SRCCM register."]
   #[inline] fn with_srccm<F: FnOnce(Srccm) -> Srccm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Srccm(::core::ptr::read_volatile((self.base() + 0x574) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x574) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SREMAC register."]
   #[inline] fn sremac_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x59c)
   }
#[doc="Get the *mut pointer for the SREMAC register."]
   #[inline] fn sremac_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x59c)
   }
#[doc="Read the SREMAC register."]
   #[inline] fn sremac(&self) -> Sremac { 
      unsafe {
         Sremac(::core::ptr::read_volatile((self.base() + 0x59c) as *const u32))
      }
   }
#[doc="Write the SREMAC register."]
   #[inline] fn set_sremac<F: FnOnce(Sremac) -> Sremac>(&self, f: F) -> &Self {
      let value = f(Sremac(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x59c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SREMAC register."]
   #[inline] fn with_sremac<F: FnOnce(Sremac) -> Sremac>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Sremac(::core::ptr::read_volatile((self.base() + 0x59c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x59c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCWD register."]
   #[inline] fn rcgcwd_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x600)
   }
#[doc="Get the *mut pointer for the RCGCWD register."]
   #[inline] fn rcgcwd_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x600)
   }
#[doc="Read the RCGCWD register."]
   #[inline] fn rcgcwd(&self) -> Rcgcwd { 
      unsafe {
         Rcgcwd(::core::ptr::read_volatile((self.base() + 0x600) as *const u32))
      }
   }
#[doc="Write the RCGCWD register."]
   #[inline] fn set_rcgcwd<F: FnOnce(Rcgcwd) -> Rcgcwd>(&self, f: F) -> &Self {
      let value = f(Rcgcwd(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x600) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCWD register."]
   #[inline] fn with_rcgcwd<F: FnOnce(Rcgcwd) -> Rcgcwd>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcwd(::core::ptr::read_volatile((self.base() + 0x600) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x600) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCTIMER register."]
   #[inline] fn rcgctimer_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x604)
   }
#[doc="Get the *mut pointer for the RCGCTIMER register."]
   #[inline] fn rcgctimer_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x604)
   }
#[doc="Read the RCGCTIMER register."]
   #[inline] fn rcgctimer(&self) -> Rcgctimer { 
      unsafe {
         Rcgctimer(::core::ptr::read_volatile((self.base() + 0x604) as *const u32))
      }
   }
#[doc="Write the RCGCTIMER register."]
   #[inline] fn set_rcgctimer<F: FnOnce(Rcgctimer) -> Rcgctimer>(&self, f: F) -> &Self {
      let value = f(Rcgctimer(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x604) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCTIMER register."]
   #[inline] fn with_rcgctimer<F: FnOnce(Rcgctimer) -> Rcgctimer>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgctimer(::core::ptr::read_volatile((self.base() + 0x604) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x604) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCGPIO register."]
   #[inline] fn rcgcgpio_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x608)
   }
#[doc="Get the *mut pointer for the RCGCGPIO register."]
   #[inline] fn rcgcgpio_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x608)
   }
#[doc="Read the RCGCGPIO register."]
   #[inline] fn rcgcgpio(&self) -> Rcgcgpio { 
      unsafe {
         Rcgcgpio(::core::ptr::read_volatile((self.base() + 0x608) as *const u32))
      }
   }
#[doc="Write the RCGCGPIO register."]
   #[inline] fn set_rcgcgpio<F: FnOnce(Rcgcgpio) -> Rcgcgpio>(&self, f: F) -> &Self {
      let value = f(Rcgcgpio(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x608) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCGPIO register."]
   #[inline] fn with_rcgcgpio<F: FnOnce(Rcgcgpio) -> Rcgcgpio>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcgpio(::core::ptr::read_volatile((self.base() + 0x608) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x608) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCDMA register."]
   #[inline] fn rcgcdma_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x60c)
   }
#[doc="Get the *mut pointer for the RCGCDMA register."]
   #[inline] fn rcgcdma_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x60c)
   }
#[doc="Read the RCGCDMA register."]
   #[inline] fn rcgcdma(&self) -> Rcgcdma { 
      unsafe {
         Rcgcdma(::core::ptr::read_volatile((self.base() + 0x60c) as *const u32))
      }
   }
#[doc="Write the RCGCDMA register."]
   #[inline] fn set_rcgcdma<F: FnOnce(Rcgcdma) -> Rcgcdma>(&self, f: F) -> &Self {
      let value = f(Rcgcdma(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x60c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCDMA register."]
   #[inline] fn with_rcgcdma<F: FnOnce(Rcgcdma) -> Rcgcdma>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcdma(::core::ptr::read_volatile((self.base() + 0x60c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x60c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCEPI register."]
   #[inline] fn rcgcepi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x610)
   }
#[doc="Get the *mut pointer for the RCGCEPI register."]
   #[inline] fn rcgcepi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x610)
   }
#[doc="Read the RCGCEPI register."]
   #[inline] fn rcgcepi(&self) -> Rcgcepi { 
      unsafe {
         Rcgcepi(::core::ptr::read_volatile((self.base() + 0x610) as *const u32))
      }
   }
#[doc="Write the RCGCEPI register."]
   #[inline] fn set_rcgcepi<F: FnOnce(Rcgcepi) -> Rcgcepi>(&self, f: F) -> &Self {
      let value = f(Rcgcepi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x610) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCEPI register."]
   #[inline] fn with_rcgcepi<F: FnOnce(Rcgcepi) -> Rcgcepi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcepi(::core::ptr::read_volatile((self.base() + 0x610) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x610) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCHIB register."]
   #[inline] fn rcgchib_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x614)
   }
#[doc="Get the *mut pointer for the RCGCHIB register."]
   #[inline] fn rcgchib_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x614)
   }
#[doc="Read the RCGCHIB register."]
   #[inline] fn rcgchib(&self) -> Rcgchib { 
      unsafe {
         Rcgchib(::core::ptr::read_volatile((self.base() + 0x614) as *const u32))
      }
   }
#[doc="Write the RCGCHIB register."]
   #[inline] fn set_rcgchib<F: FnOnce(Rcgchib) -> Rcgchib>(&self, f: F) -> &Self {
      let value = f(Rcgchib(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x614) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCHIB register."]
   #[inline] fn with_rcgchib<F: FnOnce(Rcgchib) -> Rcgchib>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgchib(::core::ptr::read_volatile((self.base() + 0x614) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x614) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCUART register."]
   #[inline] fn rcgcuart_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x618)
   }
#[doc="Get the *mut pointer for the RCGCUART register."]
   #[inline] fn rcgcuart_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x618)
   }
#[doc="Read the RCGCUART register."]
   #[inline] fn rcgcuart(&self) -> Rcgcuart { 
      unsafe {
         Rcgcuart(::core::ptr::read_volatile((self.base() + 0x618) as *const u32))
      }
   }
#[doc="Write the RCGCUART register."]
   #[inline] fn set_rcgcuart<F: FnOnce(Rcgcuart) -> Rcgcuart>(&self, f: F) -> &Self {
      let value = f(Rcgcuart(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x618) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCUART register."]
   #[inline] fn with_rcgcuart<F: FnOnce(Rcgcuart) -> Rcgcuart>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcuart(::core::ptr::read_volatile((self.base() + 0x618) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x618) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCSSI register."]
   #[inline] fn rcgcssi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x61c)
   }
#[doc="Get the *mut pointer for the RCGCSSI register."]
   #[inline] fn rcgcssi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x61c)
   }
#[doc="Read the RCGCSSI register."]
   #[inline] fn rcgcssi(&self) -> Rcgcssi { 
      unsafe {
         Rcgcssi(::core::ptr::read_volatile((self.base() + 0x61c) as *const u32))
      }
   }
#[doc="Write the RCGCSSI register."]
   #[inline] fn set_rcgcssi<F: FnOnce(Rcgcssi) -> Rcgcssi>(&self, f: F) -> &Self {
      let value = f(Rcgcssi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x61c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCSSI register."]
   #[inline] fn with_rcgcssi<F: FnOnce(Rcgcssi) -> Rcgcssi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcssi(::core::ptr::read_volatile((self.base() + 0x61c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x61c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCI2C register."]
   #[inline] fn rcgci2c_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x620)
   }
#[doc="Get the *mut pointer for the RCGCI2C register."]
   #[inline] fn rcgci2c_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x620)
   }
#[doc="Read the RCGCI2C register."]
   #[inline] fn rcgci2c(&self) -> Rcgci2c { 
      unsafe {
         Rcgci2c(::core::ptr::read_volatile((self.base() + 0x620) as *const u32))
      }
   }
#[doc="Write the RCGCI2C register."]
   #[inline] fn set_rcgci2c<F: FnOnce(Rcgci2c) -> Rcgci2c>(&self, f: F) -> &Self {
      let value = f(Rcgci2c(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x620) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCI2C register."]
   #[inline] fn with_rcgci2c<F: FnOnce(Rcgci2c) -> Rcgci2c>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgci2c(::core::ptr::read_volatile((self.base() + 0x620) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x620) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCUSB register."]
   #[inline] fn rcgcusb_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x628)
   }
#[doc="Get the *mut pointer for the RCGCUSB register."]
   #[inline] fn rcgcusb_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x628)
   }
#[doc="Read the RCGCUSB register."]
   #[inline] fn rcgcusb(&self) -> Rcgcusb { 
      unsafe {
         Rcgcusb(::core::ptr::read_volatile((self.base() + 0x628) as *const u32))
      }
   }
#[doc="Write the RCGCUSB register."]
   #[inline] fn set_rcgcusb<F: FnOnce(Rcgcusb) -> Rcgcusb>(&self, f: F) -> &Self {
      let value = f(Rcgcusb(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x628) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCUSB register."]
   #[inline] fn with_rcgcusb<F: FnOnce(Rcgcusb) -> Rcgcusb>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcusb(::core::ptr::read_volatile((self.base() + 0x628) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x628) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCEPHY register."]
   #[inline] fn rcgcephy_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x630)
   }
#[doc="Get the *mut pointer for the RCGCEPHY register."]
   #[inline] fn rcgcephy_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x630)
   }
#[doc="Read the RCGCEPHY register."]
   #[inline] fn rcgcephy(&self) -> Rcgcephy { 
      unsafe {
         Rcgcephy(::core::ptr::read_volatile((self.base() + 0x630) as *const u32))
      }
   }
#[doc="Write the RCGCEPHY register."]
   #[inline] fn set_rcgcephy<F: FnOnce(Rcgcephy) -> Rcgcephy>(&self, f: F) -> &Self {
      let value = f(Rcgcephy(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x630) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCEPHY register."]
   #[inline] fn with_rcgcephy<F: FnOnce(Rcgcephy) -> Rcgcephy>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcephy(::core::ptr::read_volatile((self.base() + 0x630) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x630) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCCAN register."]
   #[inline] fn rcgccan_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x634)
   }
#[doc="Get the *mut pointer for the RCGCCAN register."]
   #[inline] fn rcgccan_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x634)
   }
#[doc="Read the RCGCCAN register."]
   #[inline] fn rcgccan(&self) -> Rcgccan { 
      unsafe {
         Rcgccan(::core::ptr::read_volatile((self.base() + 0x634) as *const u32))
      }
   }
#[doc="Write the RCGCCAN register."]
   #[inline] fn set_rcgccan<F: FnOnce(Rcgccan) -> Rcgccan>(&self, f: F) -> &Self {
      let value = f(Rcgccan(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x634) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCCAN register."]
   #[inline] fn with_rcgccan<F: FnOnce(Rcgccan) -> Rcgccan>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgccan(::core::ptr::read_volatile((self.base() + 0x634) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x634) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCADC register."]
   #[inline] fn rcgcadc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x638)
   }
#[doc="Get the *mut pointer for the RCGCADC register."]
   #[inline] fn rcgcadc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x638)
   }
#[doc="Read the RCGCADC register."]
   #[inline] fn rcgcadc(&self) -> Rcgcadc { 
      unsafe {
         Rcgcadc(::core::ptr::read_volatile((self.base() + 0x638) as *const u32))
      }
   }
#[doc="Write the RCGCADC register."]
   #[inline] fn set_rcgcadc<F: FnOnce(Rcgcadc) -> Rcgcadc>(&self, f: F) -> &Self {
      let value = f(Rcgcadc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x638) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCADC register."]
   #[inline] fn with_rcgcadc<F: FnOnce(Rcgcadc) -> Rcgcadc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcadc(::core::ptr::read_volatile((self.base() + 0x638) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x638) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCACMP register."]
   #[inline] fn rcgcacmp_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x63c)
   }
#[doc="Get the *mut pointer for the RCGCACMP register."]
   #[inline] fn rcgcacmp_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x63c)
   }
#[doc="Read the RCGCACMP register."]
   #[inline] fn rcgcacmp(&self) -> Rcgcacmp { 
      unsafe {
         Rcgcacmp(::core::ptr::read_volatile((self.base() + 0x63c) as *const u32))
      }
   }
#[doc="Write the RCGCACMP register."]
   #[inline] fn set_rcgcacmp<F: FnOnce(Rcgcacmp) -> Rcgcacmp>(&self, f: F) -> &Self {
      let value = f(Rcgcacmp(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x63c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCACMP register."]
   #[inline] fn with_rcgcacmp<F: FnOnce(Rcgcacmp) -> Rcgcacmp>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcacmp(::core::ptr::read_volatile((self.base() + 0x63c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x63c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCPWM register."]
   #[inline] fn rcgcpwm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x640)
   }
#[doc="Get the *mut pointer for the RCGCPWM register."]
   #[inline] fn rcgcpwm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x640)
   }
#[doc="Read the RCGCPWM register."]
   #[inline] fn rcgcpwm(&self) -> Rcgcpwm { 
      unsafe {
         Rcgcpwm(::core::ptr::read_volatile((self.base() + 0x640) as *const u32))
      }
   }
#[doc="Write the RCGCPWM register."]
   #[inline] fn set_rcgcpwm<F: FnOnce(Rcgcpwm) -> Rcgcpwm>(&self, f: F) -> &Self {
      let value = f(Rcgcpwm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x640) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCPWM register."]
   #[inline] fn with_rcgcpwm<F: FnOnce(Rcgcpwm) -> Rcgcpwm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcpwm(::core::ptr::read_volatile((self.base() + 0x640) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x640) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCQEI register."]
   #[inline] fn rcgcqei_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x644)
   }
#[doc="Get the *mut pointer for the RCGCQEI register."]
   #[inline] fn rcgcqei_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x644)
   }
#[doc="Read the RCGCQEI register."]
   #[inline] fn rcgcqei(&self) -> Rcgcqei { 
      unsafe {
         Rcgcqei(::core::ptr::read_volatile((self.base() + 0x644) as *const u32))
      }
   }
#[doc="Write the RCGCQEI register."]
   #[inline] fn set_rcgcqei<F: FnOnce(Rcgcqei) -> Rcgcqei>(&self, f: F) -> &Self {
      let value = f(Rcgcqei(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x644) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCQEI register."]
   #[inline] fn with_rcgcqei<F: FnOnce(Rcgcqei) -> Rcgcqei>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcqei(::core::ptr::read_volatile((self.base() + 0x644) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x644) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCEEPROM register."]
   #[inline] fn rcgceeprom_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x658)
   }
#[doc="Get the *mut pointer for the RCGCEEPROM register."]
   #[inline] fn rcgceeprom_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x658)
   }
#[doc="Read the RCGCEEPROM register."]
   #[inline] fn rcgceeprom(&self) -> Rcgceeprom { 
      unsafe {
         Rcgceeprom(::core::ptr::read_volatile((self.base() + 0x658) as *const u32))
      }
   }
#[doc="Write the RCGCEEPROM register."]
   #[inline] fn set_rcgceeprom<F: FnOnce(Rcgceeprom) -> Rcgceeprom>(&self, f: F) -> &Self {
      let value = f(Rcgceeprom(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x658) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCEEPROM register."]
   #[inline] fn with_rcgceeprom<F: FnOnce(Rcgceeprom) -> Rcgceeprom>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgceeprom(::core::ptr::read_volatile((self.base() + 0x658) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x658) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCCCM register."]
   #[inline] fn rcgcccm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x674)
   }
#[doc="Get the *mut pointer for the RCGCCCM register."]
   #[inline] fn rcgcccm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x674)
   }
#[doc="Read the RCGCCCM register."]
   #[inline] fn rcgcccm(&self) -> Rcgcccm { 
      unsafe {
         Rcgcccm(::core::ptr::read_volatile((self.base() + 0x674) as *const u32))
      }
   }
#[doc="Write the RCGCCCM register."]
   #[inline] fn set_rcgcccm<F: FnOnce(Rcgcccm) -> Rcgcccm>(&self, f: F) -> &Self {
      let value = f(Rcgcccm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x674) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCCCM register."]
   #[inline] fn with_rcgcccm<F: FnOnce(Rcgcccm) -> Rcgcccm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcccm(::core::ptr::read_volatile((self.base() + 0x674) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x674) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RCGCEMAC register."]
   #[inline] fn rcgcemac_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x69c)
   }
#[doc="Get the *mut pointer for the RCGCEMAC register."]
   #[inline] fn rcgcemac_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x69c)
   }
#[doc="Read the RCGCEMAC register."]
   #[inline] fn rcgcemac(&self) -> Rcgcemac { 
      unsafe {
         Rcgcemac(::core::ptr::read_volatile((self.base() + 0x69c) as *const u32))
      }
   }
#[doc="Write the RCGCEMAC register."]
   #[inline] fn set_rcgcemac<F: FnOnce(Rcgcemac) -> Rcgcemac>(&self, f: F) -> &Self {
      let value = f(Rcgcemac(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x69c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RCGCEMAC register."]
   #[inline] fn with_rcgcemac<F: FnOnce(Rcgcemac) -> Rcgcemac>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Rcgcemac(::core::ptr::read_volatile((self.base() + 0x69c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x69c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCWD register."]
   #[inline] fn scgcwd_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x700)
   }
#[doc="Get the *mut pointer for the SCGCWD register."]
   #[inline] fn scgcwd_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x700)
   }
#[doc="Read the SCGCWD register."]
   #[inline] fn scgcwd(&self) -> Scgcwd { 
      unsafe {
         Scgcwd(::core::ptr::read_volatile((self.base() + 0x700) as *const u32))
      }
   }
#[doc="Write the SCGCWD register."]
   #[inline] fn set_scgcwd<F: FnOnce(Scgcwd) -> Scgcwd>(&self, f: F) -> &Self {
      let value = f(Scgcwd(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x700) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCWD register."]
   #[inline] fn with_scgcwd<F: FnOnce(Scgcwd) -> Scgcwd>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcwd(::core::ptr::read_volatile((self.base() + 0x700) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x700) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCTIMER register."]
   #[inline] fn scgctimer_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x704)
   }
#[doc="Get the *mut pointer for the SCGCTIMER register."]
   #[inline] fn scgctimer_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x704)
   }
#[doc="Read the SCGCTIMER register."]
   #[inline] fn scgctimer(&self) -> Scgctimer { 
      unsafe {
         Scgctimer(::core::ptr::read_volatile((self.base() + 0x704) as *const u32))
      }
   }
#[doc="Write the SCGCTIMER register."]
   #[inline] fn set_scgctimer<F: FnOnce(Scgctimer) -> Scgctimer>(&self, f: F) -> &Self {
      let value = f(Scgctimer(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x704) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCTIMER register."]
   #[inline] fn with_scgctimer<F: FnOnce(Scgctimer) -> Scgctimer>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgctimer(::core::ptr::read_volatile((self.base() + 0x704) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x704) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCGPIO register."]
   #[inline] fn scgcgpio_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x708)
   }
#[doc="Get the *mut pointer for the SCGCGPIO register."]
   #[inline] fn scgcgpio_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x708)
   }
#[doc="Read the SCGCGPIO register."]
   #[inline] fn scgcgpio(&self) -> Scgcgpio { 
      unsafe {
         Scgcgpio(::core::ptr::read_volatile((self.base() + 0x708) as *const u32))
      }
   }
#[doc="Write the SCGCGPIO register."]
   #[inline] fn set_scgcgpio<F: FnOnce(Scgcgpio) -> Scgcgpio>(&self, f: F) -> &Self {
      let value = f(Scgcgpio(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x708) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCGPIO register."]
   #[inline] fn with_scgcgpio<F: FnOnce(Scgcgpio) -> Scgcgpio>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcgpio(::core::ptr::read_volatile((self.base() + 0x708) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x708) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCDMA register."]
   #[inline] fn scgcdma_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x70c)
   }
#[doc="Get the *mut pointer for the SCGCDMA register."]
   #[inline] fn scgcdma_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x70c)
   }
#[doc="Read the SCGCDMA register."]
   #[inline] fn scgcdma(&self) -> Scgcdma { 
      unsafe {
         Scgcdma(::core::ptr::read_volatile((self.base() + 0x70c) as *const u32))
      }
   }
#[doc="Write the SCGCDMA register."]
   #[inline] fn set_scgcdma<F: FnOnce(Scgcdma) -> Scgcdma>(&self, f: F) -> &Self {
      let value = f(Scgcdma(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x70c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCDMA register."]
   #[inline] fn with_scgcdma<F: FnOnce(Scgcdma) -> Scgcdma>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcdma(::core::ptr::read_volatile((self.base() + 0x70c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x70c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCEPI register."]
   #[inline] fn scgcepi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x710)
   }
#[doc="Get the *mut pointer for the SCGCEPI register."]
   #[inline] fn scgcepi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x710)
   }
#[doc="Read the SCGCEPI register."]
   #[inline] fn scgcepi(&self) -> Scgcepi { 
      unsafe {
         Scgcepi(::core::ptr::read_volatile((self.base() + 0x710) as *const u32))
      }
   }
#[doc="Write the SCGCEPI register."]
   #[inline] fn set_scgcepi<F: FnOnce(Scgcepi) -> Scgcepi>(&self, f: F) -> &Self {
      let value = f(Scgcepi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x710) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCEPI register."]
   #[inline] fn with_scgcepi<F: FnOnce(Scgcepi) -> Scgcepi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcepi(::core::ptr::read_volatile((self.base() + 0x710) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x710) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCHIB register."]
   #[inline] fn scgchib_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x714)
   }
#[doc="Get the *mut pointer for the SCGCHIB register."]
   #[inline] fn scgchib_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x714)
   }
#[doc="Read the SCGCHIB register."]
   #[inline] fn scgchib(&self) -> Scgchib { 
      unsafe {
         Scgchib(::core::ptr::read_volatile((self.base() + 0x714) as *const u32))
      }
   }
#[doc="Write the SCGCHIB register."]
   #[inline] fn set_scgchib<F: FnOnce(Scgchib) -> Scgchib>(&self, f: F) -> &Self {
      let value = f(Scgchib(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x714) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCHIB register."]
   #[inline] fn with_scgchib<F: FnOnce(Scgchib) -> Scgchib>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgchib(::core::ptr::read_volatile((self.base() + 0x714) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x714) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCUART register."]
   #[inline] fn scgcuart_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x718)
   }
#[doc="Get the *mut pointer for the SCGCUART register."]
   #[inline] fn scgcuart_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x718)
   }
#[doc="Read the SCGCUART register."]
   #[inline] fn scgcuart(&self) -> Scgcuart { 
      unsafe {
         Scgcuart(::core::ptr::read_volatile((self.base() + 0x718) as *const u32))
      }
   }
#[doc="Write the SCGCUART register."]
   #[inline] fn set_scgcuart<F: FnOnce(Scgcuart) -> Scgcuart>(&self, f: F) -> &Self {
      let value = f(Scgcuart(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x718) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCUART register."]
   #[inline] fn with_scgcuart<F: FnOnce(Scgcuart) -> Scgcuart>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcuart(::core::ptr::read_volatile((self.base() + 0x718) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x718) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCSSI register."]
   #[inline] fn scgcssi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x71c)
   }
#[doc="Get the *mut pointer for the SCGCSSI register."]
   #[inline] fn scgcssi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x71c)
   }
#[doc="Read the SCGCSSI register."]
   #[inline] fn scgcssi(&self) -> Scgcssi { 
      unsafe {
         Scgcssi(::core::ptr::read_volatile((self.base() + 0x71c) as *const u32))
      }
   }
#[doc="Write the SCGCSSI register."]
   #[inline] fn set_scgcssi<F: FnOnce(Scgcssi) -> Scgcssi>(&self, f: F) -> &Self {
      let value = f(Scgcssi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x71c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCSSI register."]
   #[inline] fn with_scgcssi<F: FnOnce(Scgcssi) -> Scgcssi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcssi(::core::ptr::read_volatile((self.base() + 0x71c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x71c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCI2C register."]
   #[inline] fn scgci2c_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x720)
   }
#[doc="Get the *mut pointer for the SCGCI2C register."]
   #[inline] fn scgci2c_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x720)
   }
#[doc="Read the SCGCI2C register."]
   #[inline] fn scgci2c(&self) -> Scgci2c { 
      unsafe {
         Scgci2c(::core::ptr::read_volatile((self.base() + 0x720) as *const u32))
      }
   }
#[doc="Write the SCGCI2C register."]
   #[inline] fn set_scgci2c<F: FnOnce(Scgci2c) -> Scgci2c>(&self, f: F) -> &Self {
      let value = f(Scgci2c(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x720) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCI2C register."]
   #[inline] fn with_scgci2c<F: FnOnce(Scgci2c) -> Scgci2c>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgci2c(::core::ptr::read_volatile((self.base() + 0x720) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x720) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCUSB register."]
   #[inline] fn scgcusb_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x728)
   }
#[doc="Get the *mut pointer for the SCGCUSB register."]
   #[inline] fn scgcusb_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x728)
   }
#[doc="Read the SCGCUSB register."]
   #[inline] fn scgcusb(&self) -> Scgcusb { 
      unsafe {
         Scgcusb(::core::ptr::read_volatile((self.base() + 0x728) as *const u32))
      }
   }
#[doc="Write the SCGCUSB register."]
   #[inline] fn set_scgcusb<F: FnOnce(Scgcusb) -> Scgcusb>(&self, f: F) -> &Self {
      let value = f(Scgcusb(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x728) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCUSB register."]
   #[inline] fn with_scgcusb<F: FnOnce(Scgcusb) -> Scgcusb>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcusb(::core::ptr::read_volatile((self.base() + 0x728) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x728) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCEPHY register."]
   #[inline] fn scgcephy_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x730)
   }
#[doc="Get the *mut pointer for the SCGCEPHY register."]
   #[inline] fn scgcephy_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x730)
   }
#[doc="Read the SCGCEPHY register."]
   #[inline] fn scgcephy(&self) -> Scgcephy { 
      unsafe {
         Scgcephy(::core::ptr::read_volatile((self.base() + 0x730) as *const u32))
      }
   }
#[doc="Write the SCGCEPHY register."]
   #[inline] fn set_scgcephy<F: FnOnce(Scgcephy) -> Scgcephy>(&self, f: F) -> &Self {
      let value = f(Scgcephy(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x730) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCEPHY register."]
   #[inline] fn with_scgcephy<F: FnOnce(Scgcephy) -> Scgcephy>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcephy(::core::ptr::read_volatile((self.base() + 0x730) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x730) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCCAN register."]
   #[inline] fn scgccan_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x734)
   }
#[doc="Get the *mut pointer for the SCGCCAN register."]
   #[inline] fn scgccan_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x734)
   }
#[doc="Read the SCGCCAN register."]
   #[inline] fn scgccan(&self) -> Scgccan { 
      unsafe {
         Scgccan(::core::ptr::read_volatile((self.base() + 0x734) as *const u32))
      }
   }
#[doc="Write the SCGCCAN register."]
   #[inline] fn set_scgccan<F: FnOnce(Scgccan) -> Scgccan>(&self, f: F) -> &Self {
      let value = f(Scgccan(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x734) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCCAN register."]
   #[inline] fn with_scgccan<F: FnOnce(Scgccan) -> Scgccan>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgccan(::core::ptr::read_volatile((self.base() + 0x734) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x734) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCADC register."]
   #[inline] fn scgcadc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x738)
   }
#[doc="Get the *mut pointer for the SCGCADC register."]
   #[inline] fn scgcadc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x738)
   }
#[doc="Read the SCGCADC register."]
   #[inline] fn scgcadc(&self) -> Scgcadc { 
      unsafe {
         Scgcadc(::core::ptr::read_volatile((self.base() + 0x738) as *const u32))
      }
   }
#[doc="Write the SCGCADC register."]
   #[inline] fn set_scgcadc<F: FnOnce(Scgcadc) -> Scgcadc>(&self, f: F) -> &Self {
      let value = f(Scgcadc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x738) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCADC register."]
   #[inline] fn with_scgcadc<F: FnOnce(Scgcadc) -> Scgcadc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcadc(::core::ptr::read_volatile((self.base() + 0x738) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x738) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCACMP register."]
   #[inline] fn scgcacmp_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x73c)
   }
#[doc="Get the *mut pointer for the SCGCACMP register."]
   #[inline] fn scgcacmp_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x73c)
   }
#[doc="Read the SCGCACMP register."]
   #[inline] fn scgcacmp(&self) -> Scgcacmp { 
      unsafe {
         Scgcacmp(::core::ptr::read_volatile((self.base() + 0x73c) as *const u32))
      }
   }
#[doc="Write the SCGCACMP register."]
   #[inline] fn set_scgcacmp<F: FnOnce(Scgcacmp) -> Scgcacmp>(&self, f: F) -> &Self {
      let value = f(Scgcacmp(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x73c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCACMP register."]
   #[inline] fn with_scgcacmp<F: FnOnce(Scgcacmp) -> Scgcacmp>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcacmp(::core::ptr::read_volatile((self.base() + 0x73c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x73c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCPWM register."]
   #[inline] fn scgcpwm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x740)
   }
#[doc="Get the *mut pointer for the SCGCPWM register."]
   #[inline] fn scgcpwm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x740)
   }
#[doc="Read the SCGCPWM register."]
   #[inline] fn scgcpwm(&self) -> Scgcpwm { 
      unsafe {
         Scgcpwm(::core::ptr::read_volatile((self.base() + 0x740) as *const u32))
      }
   }
#[doc="Write the SCGCPWM register."]
   #[inline] fn set_scgcpwm<F: FnOnce(Scgcpwm) -> Scgcpwm>(&self, f: F) -> &Self {
      let value = f(Scgcpwm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x740) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCPWM register."]
   #[inline] fn with_scgcpwm<F: FnOnce(Scgcpwm) -> Scgcpwm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcpwm(::core::ptr::read_volatile((self.base() + 0x740) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x740) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCQEI register."]
   #[inline] fn scgcqei_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x744)
   }
#[doc="Get the *mut pointer for the SCGCQEI register."]
   #[inline] fn scgcqei_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x744)
   }
#[doc="Read the SCGCQEI register."]
   #[inline] fn scgcqei(&self) -> Scgcqei { 
      unsafe {
         Scgcqei(::core::ptr::read_volatile((self.base() + 0x744) as *const u32))
      }
   }
#[doc="Write the SCGCQEI register."]
   #[inline] fn set_scgcqei<F: FnOnce(Scgcqei) -> Scgcqei>(&self, f: F) -> &Self {
      let value = f(Scgcqei(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x744) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCQEI register."]
   #[inline] fn with_scgcqei<F: FnOnce(Scgcqei) -> Scgcqei>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcqei(::core::ptr::read_volatile((self.base() + 0x744) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x744) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCEEPROM register."]
   #[inline] fn scgceeprom_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x758)
   }
#[doc="Get the *mut pointer for the SCGCEEPROM register."]
   #[inline] fn scgceeprom_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x758)
   }
#[doc="Read the SCGCEEPROM register."]
   #[inline] fn scgceeprom(&self) -> Scgceeprom { 
      unsafe {
         Scgceeprom(::core::ptr::read_volatile((self.base() + 0x758) as *const u32))
      }
   }
#[doc="Write the SCGCEEPROM register."]
   #[inline] fn set_scgceeprom<F: FnOnce(Scgceeprom) -> Scgceeprom>(&self, f: F) -> &Self {
      let value = f(Scgceeprom(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x758) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCEEPROM register."]
   #[inline] fn with_scgceeprom<F: FnOnce(Scgceeprom) -> Scgceeprom>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgceeprom(::core::ptr::read_volatile((self.base() + 0x758) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x758) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCCCM register."]
   #[inline] fn scgcccm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x774)
   }
#[doc="Get the *mut pointer for the SCGCCCM register."]
   #[inline] fn scgcccm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x774)
   }
#[doc="Read the SCGCCCM register."]
   #[inline] fn scgcccm(&self) -> Scgcccm { 
      unsafe {
         Scgcccm(::core::ptr::read_volatile((self.base() + 0x774) as *const u32))
      }
   }
#[doc="Write the SCGCCCM register."]
   #[inline] fn set_scgcccm<F: FnOnce(Scgcccm) -> Scgcccm>(&self, f: F) -> &Self {
      let value = f(Scgcccm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x774) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCCCM register."]
   #[inline] fn with_scgcccm<F: FnOnce(Scgcccm) -> Scgcccm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcccm(::core::ptr::read_volatile((self.base() + 0x774) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x774) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCGCEMAC register."]
   #[inline] fn scgcemac_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x79c)
   }
#[doc="Get the *mut pointer for the SCGCEMAC register."]
   #[inline] fn scgcemac_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x79c)
   }
#[doc="Read the SCGCEMAC register."]
   #[inline] fn scgcemac(&self) -> Scgcemac { 
      unsafe {
         Scgcemac(::core::ptr::read_volatile((self.base() + 0x79c) as *const u32))
      }
   }
#[doc="Write the SCGCEMAC register."]
   #[inline] fn set_scgcemac<F: FnOnce(Scgcemac) -> Scgcemac>(&self, f: F) -> &Self {
      let value = f(Scgcemac(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x79c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCGCEMAC register."]
   #[inline] fn with_scgcemac<F: FnOnce(Scgcemac) -> Scgcemac>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scgcemac(::core::ptr::read_volatile((self.base() + 0x79c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x79c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCWD register."]
   #[inline] fn dcgcwd_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x800)
   }
#[doc="Get the *mut pointer for the DCGCWD register."]
   #[inline] fn dcgcwd_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x800)
   }
#[doc="Read the DCGCWD register."]
   #[inline] fn dcgcwd(&self) -> Dcgcwd { 
      unsafe {
         Dcgcwd(::core::ptr::read_volatile((self.base() + 0x800) as *const u32))
      }
   }
#[doc="Write the DCGCWD register."]
   #[inline] fn set_dcgcwd<F: FnOnce(Dcgcwd) -> Dcgcwd>(&self, f: F) -> &Self {
      let value = f(Dcgcwd(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x800) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCWD register."]
   #[inline] fn with_dcgcwd<F: FnOnce(Dcgcwd) -> Dcgcwd>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcwd(::core::ptr::read_volatile((self.base() + 0x800) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x800) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCTIMER register."]
   #[inline] fn dcgctimer_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x804)
   }
#[doc="Get the *mut pointer for the DCGCTIMER register."]
   #[inline] fn dcgctimer_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x804)
   }
#[doc="Read the DCGCTIMER register."]
   #[inline] fn dcgctimer(&self) -> Dcgctimer { 
      unsafe {
         Dcgctimer(::core::ptr::read_volatile((self.base() + 0x804) as *const u32))
      }
   }
#[doc="Write the DCGCTIMER register."]
   #[inline] fn set_dcgctimer<F: FnOnce(Dcgctimer) -> Dcgctimer>(&self, f: F) -> &Self {
      let value = f(Dcgctimer(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x804) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCTIMER register."]
   #[inline] fn with_dcgctimer<F: FnOnce(Dcgctimer) -> Dcgctimer>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgctimer(::core::ptr::read_volatile((self.base() + 0x804) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x804) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCGPIO register."]
   #[inline] fn dcgcgpio_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x808)
   }
#[doc="Get the *mut pointer for the DCGCGPIO register."]
   #[inline] fn dcgcgpio_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x808)
   }
#[doc="Read the DCGCGPIO register."]
   #[inline] fn dcgcgpio(&self) -> Dcgcgpio { 
      unsafe {
         Dcgcgpio(::core::ptr::read_volatile((self.base() + 0x808) as *const u32))
      }
   }
#[doc="Write the DCGCGPIO register."]
   #[inline] fn set_dcgcgpio<F: FnOnce(Dcgcgpio) -> Dcgcgpio>(&self, f: F) -> &Self {
      let value = f(Dcgcgpio(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x808) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCGPIO register."]
   #[inline] fn with_dcgcgpio<F: FnOnce(Dcgcgpio) -> Dcgcgpio>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcgpio(::core::ptr::read_volatile((self.base() + 0x808) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x808) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCDMA register."]
   #[inline] fn dcgcdma_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x80c)
   }
#[doc="Get the *mut pointer for the DCGCDMA register."]
   #[inline] fn dcgcdma_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x80c)
   }
#[doc="Read the DCGCDMA register."]
   #[inline] fn dcgcdma(&self) -> Dcgcdma { 
      unsafe {
         Dcgcdma(::core::ptr::read_volatile((self.base() + 0x80c) as *const u32))
      }
   }
#[doc="Write the DCGCDMA register."]
   #[inline] fn set_dcgcdma<F: FnOnce(Dcgcdma) -> Dcgcdma>(&self, f: F) -> &Self {
      let value = f(Dcgcdma(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x80c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCDMA register."]
   #[inline] fn with_dcgcdma<F: FnOnce(Dcgcdma) -> Dcgcdma>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcdma(::core::ptr::read_volatile((self.base() + 0x80c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x80c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCEPI register."]
   #[inline] fn dcgcepi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x810)
   }
#[doc="Get the *mut pointer for the DCGCEPI register."]
   #[inline] fn dcgcepi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x810)
   }
#[doc="Read the DCGCEPI register."]
   #[inline] fn dcgcepi(&self) -> Dcgcepi { 
      unsafe {
         Dcgcepi(::core::ptr::read_volatile((self.base() + 0x810) as *const u32))
      }
   }
#[doc="Write the DCGCEPI register."]
   #[inline] fn set_dcgcepi<F: FnOnce(Dcgcepi) -> Dcgcepi>(&self, f: F) -> &Self {
      let value = f(Dcgcepi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x810) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCEPI register."]
   #[inline] fn with_dcgcepi<F: FnOnce(Dcgcepi) -> Dcgcepi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcepi(::core::ptr::read_volatile((self.base() + 0x810) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x810) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCHIB register."]
   #[inline] fn dcgchib_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x814)
   }
#[doc="Get the *mut pointer for the DCGCHIB register."]
   #[inline] fn dcgchib_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x814)
   }
#[doc="Read the DCGCHIB register."]
   #[inline] fn dcgchib(&self) -> Dcgchib { 
      unsafe {
         Dcgchib(::core::ptr::read_volatile((self.base() + 0x814) as *const u32))
      }
   }
#[doc="Write the DCGCHIB register."]
   #[inline] fn set_dcgchib<F: FnOnce(Dcgchib) -> Dcgchib>(&self, f: F) -> &Self {
      let value = f(Dcgchib(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x814) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCHIB register."]
   #[inline] fn with_dcgchib<F: FnOnce(Dcgchib) -> Dcgchib>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgchib(::core::ptr::read_volatile((self.base() + 0x814) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x814) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCUART register."]
   #[inline] fn dcgcuart_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x818)
   }
#[doc="Get the *mut pointer for the DCGCUART register."]
   #[inline] fn dcgcuart_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x818)
   }
#[doc="Read the DCGCUART register."]
   #[inline] fn dcgcuart(&self) -> Dcgcuart { 
      unsafe {
         Dcgcuart(::core::ptr::read_volatile((self.base() + 0x818) as *const u32))
      }
   }
#[doc="Write the DCGCUART register."]
   #[inline] fn set_dcgcuart<F: FnOnce(Dcgcuart) -> Dcgcuart>(&self, f: F) -> &Self {
      let value = f(Dcgcuart(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x818) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCUART register."]
   #[inline] fn with_dcgcuart<F: FnOnce(Dcgcuart) -> Dcgcuart>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcuart(::core::ptr::read_volatile((self.base() + 0x818) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x818) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCSSI register."]
   #[inline] fn dcgcssi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x81c)
   }
#[doc="Get the *mut pointer for the DCGCSSI register."]
   #[inline] fn dcgcssi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x81c)
   }
#[doc="Read the DCGCSSI register."]
   #[inline] fn dcgcssi(&self) -> Dcgcssi { 
      unsafe {
         Dcgcssi(::core::ptr::read_volatile((self.base() + 0x81c) as *const u32))
      }
   }
#[doc="Write the DCGCSSI register."]
   #[inline] fn set_dcgcssi<F: FnOnce(Dcgcssi) -> Dcgcssi>(&self, f: F) -> &Self {
      let value = f(Dcgcssi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x81c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCSSI register."]
   #[inline] fn with_dcgcssi<F: FnOnce(Dcgcssi) -> Dcgcssi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcssi(::core::ptr::read_volatile((self.base() + 0x81c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x81c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCI2C register."]
   #[inline] fn dcgci2c_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x820)
   }
#[doc="Get the *mut pointer for the DCGCI2C register."]
   #[inline] fn dcgci2c_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x820)
   }
#[doc="Read the DCGCI2C register."]
   #[inline] fn dcgci2c(&self) -> Dcgci2c { 
      unsafe {
         Dcgci2c(::core::ptr::read_volatile((self.base() + 0x820) as *const u32))
      }
   }
#[doc="Write the DCGCI2C register."]
   #[inline] fn set_dcgci2c<F: FnOnce(Dcgci2c) -> Dcgci2c>(&self, f: F) -> &Self {
      let value = f(Dcgci2c(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x820) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCI2C register."]
   #[inline] fn with_dcgci2c<F: FnOnce(Dcgci2c) -> Dcgci2c>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgci2c(::core::ptr::read_volatile((self.base() + 0x820) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x820) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCUSB register."]
   #[inline] fn dcgcusb_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x828)
   }
#[doc="Get the *mut pointer for the DCGCUSB register."]
   #[inline] fn dcgcusb_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x828)
   }
#[doc="Read the DCGCUSB register."]
   #[inline] fn dcgcusb(&self) -> Dcgcusb { 
      unsafe {
         Dcgcusb(::core::ptr::read_volatile((self.base() + 0x828) as *const u32))
      }
   }
#[doc="Write the DCGCUSB register."]
   #[inline] fn set_dcgcusb<F: FnOnce(Dcgcusb) -> Dcgcusb>(&self, f: F) -> &Self {
      let value = f(Dcgcusb(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x828) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCUSB register."]
   #[inline] fn with_dcgcusb<F: FnOnce(Dcgcusb) -> Dcgcusb>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcusb(::core::ptr::read_volatile((self.base() + 0x828) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x828) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCEPHY register."]
   #[inline] fn dcgcephy_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x830)
   }
#[doc="Get the *mut pointer for the DCGCEPHY register."]
   #[inline] fn dcgcephy_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x830)
   }
#[doc="Read the DCGCEPHY register."]
   #[inline] fn dcgcephy(&self) -> Dcgcephy { 
      unsafe {
         Dcgcephy(::core::ptr::read_volatile((self.base() + 0x830) as *const u32))
      }
   }
#[doc="Write the DCGCEPHY register."]
   #[inline] fn set_dcgcephy<F: FnOnce(Dcgcephy) -> Dcgcephy>(&self, f: F) -> &Self {
      let value = f(Dcgcephy(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x830) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCEPHY register."]
   #[inline] fn with_dcgcephy<F: FnOnce(Dcgcephy) -> Dcgcephy>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcephy(::core::ptr::read_volatile((self.base() + 0x830) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x830) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCCAN register."]
   #[inline] fn dcgccan_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x834)
   }
#[doc="Get the *mut pointer for the DCGCCAN register."]
   #[inline] fn dcgccan_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x834)
   }
#[doc="Read the DCGCCAN register."]
   #[inline] fn dcgccan(&self) -> Dcgccan { 
      unsafe {
         Dcgccan(::core::ptr::read_volatile((self.base() + 0x834) as *const u32))
      }
   }
#[doc="Write the DCGCCAN register."]
   #[inline] fn set_dcgccan<F: FnOnce(Dcgccan) -> Dcgccan>(&self, f: F) -> &Self {
      let value = f(Dcgccan(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x834) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCCAN register."]
   #[inline] fn with_dcgccan<F: FnOnce(Dcgccan) -> Dcgccan>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgccan(::core::ptr::read_volatile((self.base() + 0x834) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x834) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCADC register."]
   #[inline] fn dcgcadc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x838)
   }
#[doc="Get the *mut pointer for the DCGCADC register."]
   #[inline] fn dcgcadc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x838)
   }
#[doc="Read the DCGCADC register."]
   #[inline] fn dcgcadc(&self) -> Dcgcadc { 
      unsafe {
         Dcgcadc(::core::ptr::read_volatile((self.base() + 0x838) as *const u32))
      }
   }
#[doc="Write the DCGCADC register."]
   #[inline] fn set_dcgcadc<F: FnOnce(Dcgcadc) -> Dcgcadc>(&self, f: F) -> &Self {
      let value = f(Dcgcadc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x838) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCADC register."]
   #[inline] fn with_dcgcadc<F: FnOnce(Dcgcadc) -> Dcgcadc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcadc(::core::ptr::read_volatile((self.base() + 0x838) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x838) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCACMP register."]
   #[inline] fn dcgcacmp_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x83c)
   }
#[doc="Get the *mut pointer for the DCGCACMP register."]
   #[inline] fn dcgcacmp_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x83c)
   }
#[doc="Read the DCGCACMP register."]
   #[inline] fn dcgcacmp(&self) -> Dcgcacmp { 
      unsafe {
         Dcgcacmp(::core::ptr::read_volatile((self.base() + 0x83c) as *const u32))
      }
   }
#[doc="Write the DCGCACMP register."]
   #[inline] fn set_dcgcacmp<F: FnOnce(Dcgcacmp) -> Dcgcacmp>(&self, f: F) -> &Self {
      let value = f(Dcgcacmp(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x83c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCACMP register."]
   #[inline] fn with_dcgcacmp<F: FnOnce(Dcgcacmp) -> Dcgcacmp>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcacmp(::core::ptr::read_volatile((self.base() + 0x83c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x83c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCPWM register."]
   #[inline] fn dcgcpwm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x840)
   }
#[doc="Get the *mut pointer for the DCGCPWM register."]
   #[inline] fn dcgcpwm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x840)
   }
#[doc="Read the DCGCPWM register."]
   #[inline] fn dcgcpwm(&self) -> Dcgcpwm { 
      unsafe {
         Dcgcpwm(::core::ptr::read_volatile((self.base() + 0x840) as *const u32))
      }
   }
#[doc="Write the DCGCPWM register."]
   #[inline] fn set_dcgcpwm<F: FnOnce(Dcgcpwm) -> Dcgcpwm>(&self, f: F) -> &Self {
      let value = f(Dcgcpwm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x840) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCPWM register."]
   #[inline] fn with_dcgcpwm<F: FnOnce(Dcgcpwm) -> Dcgcpwm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcpwm(::core::ptr::read_volatile((self.base() + 0x840) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x840) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCQEI register."]
   #[inline] fn dcgcqei_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x844)
   }
#[doc="Get the *mut pointer for the DCGCQEI register."]
   #[inline] fn dcgcqei_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x844)
   }
#[doc="Read the DCGCQEI register."]
   #[inline] fn dcgcqei(&self) -> Dcgcqei { 
      unsafe {
         Dcgcqei(::core::ptr::read_volatile((self.base() + 0x844) as *const u32))
      }
   }
#[doc="Write the DCGCQEI register."]
   #[inline] fn set_dcgcqei<F: FnOnce(Dcgcqei) -> Dcgcqei>(&self, f: F) -> &Self {
      let value = f(Dcgcqei(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x844) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCQEI register."]
   #[inline] fn with_dcgcqei<F: FnOnce(Dcgcqei) -> Dcgcqei>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcqei(::core::ptr::read_volatile((self.base() + 0x844) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x844) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCEEPROM register."]
   #[inline] fn dcgceeprom_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x858)
   }
#[doc="Get the *mut pointer for the DCGCEEPROM register."]
   #[inline] fn dcgceeprom_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x858)
   }
#[doc="Read the DCGCEEPROM register."]
   #[inline] fn dcgceeprom(&self) -> Dcgceeprom { 
      unsafe {
         Dcgceeprom(::core::ptr::read_volatile((self.base() + 0x858) as *const u32))
      }
   }
#[doc="Write the DCGCEEPROM register."]
   #[inline] fn set_dcgceeprom<F: FnOnce(Dcgceeprom) -> Dcgceeprom>(&self, f: F) -> &Self {
      let value = f(Dcgceeprom(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x858) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCEEPROM register."]
   #[inline] fn with_dcgceeprom<F: FnOnce(Dcgceeprom) -> Dcgceeprom>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgceeprom(::core::ptr::read_volatile((self.base() + 0x858) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x858) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCCCM register."]
   #[inline] fn dcgcccm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x874)
   }
#[doc="Get the *mut pointer for the DCGCCCM register."]
   #[inline] fn dcgcccm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x874)
   }
#[doc="Read the DCGCCCM register."]
   #[inline] fn dcgcccm(&self) -> Dcgcccm { 
      unsafe {
         Dcgcccm(::core::ptr::read_volatile((self.base() + 0x874) as *const u32))
      }
   }
#[doc="Write the DCGCCCM register."]
   #[inline] fn set_dcgcccm<F: FnOnce(Dcgcccm) -> Dcgcccm>(&self, f: F) -> &Self {
      let value = f(Dcgcccm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x874) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCCCM register."]
   #[inline] fn with_dcgcccm<F: FnOnce(Dcgcccm) -> Dcgcccm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcccm(::core::ptr::read_volatile((self.base() + 0x874) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x874) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the DCGCEMAC register."]
   #[inline] fn dcgcemac_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x89c)
   }
#[doc="Get the *mut pointer for the DCGCEMAC register."]
   #[inline] fn dcgcemac_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x89c)
   }
#[doc="Read the DCGCEMAC register."]
   #[inline] fn dcgcemac(&self) -> Dcgcemac { 
      unsafe {
         Dcgcemac(::core::ptr::read_volatile((self.base() + 0x89c) as *const u32))
      }
   }
#[doc="Write the DCGCEMAC register."]
   #[inline] fn set_dcgcemac<F: FnOnce(Dcgcemac) -> Dcgcemac>(&self, f: F) -> &Self {
      let value = f(Dcgcemac(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x89c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the DCGCEMAC register."]
   #[inline] fn with_dcgcemac<F: FnOnce(Dcgcemac) -> Dcgcemac>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Dcgcemac(::core::ptr::read_volatile((self.base() + 0x89c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x89c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCWD register."]
   #[inline] fn pcwd_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x900)
   }
#[doc="Get the *mut pointer for the PCWD register."]
   #[inline] fn pcwd_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x900)
   }
#[doc="Read the PCWD register."]
   #[inline] fn pcwd(&self) -> Pcwd { 
      unsafe {
         Pcwd(::core::ptr::read_volatile((self.base() + 0x900) as *const u32))
      }
   }
#[doc="Write the PCWD register."]
   #[inline] fn set_pcwd<F: FnOnce(Pcwd) -> Pcwd>(&self, f: F) -> &Self {
      let value = f(Pcwd(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x900) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCWD register."]
   #[inline] fn with_pcwd<F: FnOnce(Pcwd) -> Pcwd>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcwd(::core::ptr::read_volatile((self.base() + 0x900) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x900) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCTIMER register."]
   #[inline] fn pctimer_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x904)
   }
#[doc="Get the *mut pointer for the PCTIMER register."]
   #[inline] fn pctimer_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x904)
   }
#[doc="Read the PCTIMER register."]
   #[inline] fn pctimer(&self) -> Pctimer { 
      unsafe {
         Pctimer(::core::ptr::read_volatile((self.base() + 0x904) as *const u32))
      }
   }
#[doc="Write the PCTIMER register."]
   #[inline] fn set_pctimer<F: FnOnce(Pctimer) -> Pctimer>(&self, f: F) -> &Self {
      let value = f(Pctimer(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x904) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCTIMER register."]
   #[inline] fn with_pctimer<F: FnOnce(Pctimer) -> Pctimer>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pctimer(::core::ptr::read_volatile((self.base() + 0x904) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x904) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCGPIO register."]
   #[inline] fn pcgpio_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x908)
   }
#[doc="Get the *mut pointer for the PCGPIO register."]
   #[inline] fn pcgpio_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x908)
   }
#[doc="Read the PCGPIO register."]
   #[inline] fn pcgpio(&self) -> Pcgpio { 
      unsafe {
         Pcgpio(::core::ptr::read_volatile((self.base() + 0x908) as *const u32))
      }
   }
#[doc="Write the PCGPIO register."]
   #[inline] fn set_pcgpio<F: FnOnce(Pcgpio) -> Pcgpio>(&self, f: F) -> &Self {
      let value = f(Pcgpio(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x908) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCGPIO register."]
   #[inline] fn with_pcgpio<F: FnOnce(Pcgpio) -> Pcgpio>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcgpio(::core::ptr::read_volatile((self.base() + 0x908) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x908) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCDMA register."]
   #[inline] fn pcdma_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x90c)
   }
#[doc="Get the *mut pointer for the PCDMA register."]
   #[inline] fn pcdma_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x90c)
   }
#[doc="Read the PCDMA register."]
   #[inline] fn pcdma(&self) -> Pcdma { 
      unsafe {
         Pcdma(::core::ptr::read_volatile((self.base() + 0x90c) as *const u32))
      }
   }
#[doc="Write the PCDMA register."]
   #[inline] fn set_pcdma<F: FnOnce(Pcdma) -> Pcdma>(&self, f: F) -> &Self {
      let value = f(Pcdma(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x90c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCDMA register."]
   #[inline] fn with_pcdma<F: FnOnce(Pcdma) -> Pcdma>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcdma(::core::ptr::read_volatile((self.base() + 0x90c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x90c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCEPI register."]
   #[inline] fn pcepi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x910)
   }
#[doc="Get the *mut pointer for the PCEPI register."]
   #[inline] fn pcepi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x910)
   }
#[doc="Read the PCEPI register."]
   #[inline] fn pcepi(&self) -> Pcepi { 
      unsafe {
         Pcepi(::core::ptr::read_volatile((self.base() + 0x910) as *const u32))
      }
   }
#[doc="Write the PCEPI register."]
   #[inline] fn set_pcepi<F: FnOnce(Pcepi) -> Pcepi>(&self, f: F) -> &Self {
      let value = f(Pcepi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x910) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCEPI register."]
   #[inline] fn with_pcepi<F: FnOnce(Pcepi) -> Pcepi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcepi(::core::ptr::read_volatile((self.base() + 0x910) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x910) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCHIB register."]
   #[inline] fn pchib_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x914)
   }
#[doc="Get the *mut pointer for the PCHIB register."]
   #[inline] fn pchib_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x914)
   }
#[doc="Read the PCHIB register."]
   #[inline] fn pchib(&self) -> Pchib { 
      unsafe {
         Pchib(::core::ptr::read_volatile((self.base() + 0x914) as *const u32))
      }
   }
#[doc="Write the PCHIB register."]
   #[inline] fn set_pchib<F: FnOnce(Pchib) -> Pchib>(&self, f: F) -> &Self {
      let value = f(Pchib(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x914) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCHIB register."]
   #[inline] fn with_pchib<F: FnOnce(Pchib) -> Pchib>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pchib(::core::ptr::read_volatile((self.base() + 0x914) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x914) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCUART register."]
   #[inline] fn pcuart_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x918)
   }
#[doc="Get the *mut pointer for the PCUART register."]
   #[inline] fn pcuart_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x918)
   }
#[doc="Read the PCUART register."]
   #[inline] fn pcuart(&self) -> Pcuart { 
      unsafe {
         Pcuart(::core::ptr::read_volatile((self.base() + 0x918) as *const u32))
      }
   }
#[doc="Write the PCUART register."]
   #[inline] fn set_pcuart<F: FnOnce(Pcuart) -> Pcuart>(&self, f: F) -> &Self {
      let value = f(Pcuart(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x918) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCUART register."]
   #[inline] fn with_pcuart<F: FnOnce(Pcuart) -> Pcuart>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcuart(::core::ptr::read_volatile((self.base() + 0x918) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x918) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCSSI register."]
   #[inline] fn pcssi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x91c)
   }
#[doc="Get the *mut pointer for the PCSSI register."]
   #[inline] fn pcssi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x91c)
   }
#[doc="Read the PCSSI register."]
   #[inline] fn pcssi(&self) -> Pcssi { 
      unsafe {
         Pcssi(::core::ptr::read_volatile((self.base() + 0x91c) as *const u32))
      }
   }
#[doc="Write the PCSSI register."]
   #[inline] fn set_pcssi<F: FnOnce(Pcssi) -> Pcssi>(&self, f: F) -> &Self {
      let value = f(Pcssi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x91c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCSSI register."]
   #[inline] fn with_pcssi<F: FnOnce(Pcssi) -> Pcssi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcssi(::core::ptr::read_volatile((self.base() + 0x91c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x91c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCI2C register."]
   #[inline] fn pci2c_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x920)
   }
#[doc="Get the *mut pointer for the PCI2C register."]
   #[inline] fn pci2c_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x920)
   }
#[doc="Read the PCI2C register."]
   #[inline] fn pci2c(&self) -> Pci2c { 
      unsafe {
         Pci2c(::core::ptr::read_volatile((self.base() + 0x920) as *const u32))
      }
   }
#[doc="Write the PCI2C register."]
   #[inline] fn set_pci2c<F: FnOnce(Pci2c) -> Pci2c>(&self, f: F) -> &Self {
      let value = f(Pci2c(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x920) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCI2C register."]
   #[inline] fn with_pci2c<F: FnOnce(Pci2c) -> Pci2c>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pci2c(::core::ptr::read_volatile((self.base() + 0x920) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x920) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCUSB register."]
   #[inline] fn pcusb_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x928)
   }
#[doc="Get the *mut pointer for the PCUSB register."]
   #[inline] fn pcusb_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x928)
   }
#[doc="Read the PCUSB register."]
   #[inline] fn pcusb(&self) -> Pcusb { 
      unsafe {
         Pcusb(::core::ptr::read_volatile((self.base() + 0x928) as *const u32))
      }
   }
#[doc="Write the PCUSB register."]
   #[inline] fn set_pcusb<F: FnOnce(Pcusb) -> Pcusb>(&self, f: F) -> &Self {
      let value = f(Pcusb(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x928) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCUSB register."]
   #[inline] fn with_pcusb<F: FnOnce(Pcusb) -> Pcusb>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcusb(::core::ptr::read_volatile((self.base() + 0x928) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x928) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCEPHY register."]
   #[inline] fn pcephy_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x930)
   }
#[doc="Get the *mut pointer for the PCEPHY register."]
   #[inline] fn pcephy_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x930)
   }
#[doc="Read the PCEPHY register."]
   #[inline] fn pcephy(&self) -> Pcephy { 
      unsafe {
         Pcephy(::core::ptr::read_volatile((self.base() + 0x930) as *const u32))
      }
   }
#[doc="Write the PCEPHY register."]
   #[inline] fn set_pcephy<F: FnOnce(Pcephy) -> Pcephy>(&self, f: F) -> &Self {
      let value = f(Pcephy(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x930) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCEPHY register."]
   #[inline] fn with_pcephy<F: FnOnce(Pcephy) -> Pcephy>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcephy(::core::ptr::read_volatile((self.base() + 0x930) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x930) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCCAN register."]
   #[inline] fn pccan_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x934)
   }
#[doc="Get the *mut pointer for the PCCAN register."]
   #[inline] fn pccan_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x934)
   }
#[doc="Read the PCCAN register."]
   #[inline] fn pccan(&self) -> Pccan { 
      unsafe {
         Pccan(::core::ptr::read_volatile((self.base() + 0x934) as *const u32))
      }
   }
#[doc="Write the PCCAN register."]
   #[inline] fn set_pccan<F: FnOnce(Pccan) -> Pccan>(&self, f: F) -> &Self {
      let value = f(Pccan(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x934) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCAN register."]
   #[inline] fn with_pccan<F: FnOnce(Pccan) -> Pccan>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pccan(::core::ptr::read_volatile((self.base() + 0x934) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x934) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCADC register."]
   #[inline] fn pcadc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x938)
   }
#[doc="Get the *mut pointer for the PCADC register."]
   #[inline] fn pcadc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x938)
   }
#[doc="Read the PCADC register."]
   #[inline] fn pcadc(&self) -> Pcadc { 
      unsafe {
         Pcadc(::core::ptr::read_volatile((self.base() + 0x938) as *const u32))
      }
   }
#[doc="Write the PCADC register."]
   #[inline] fn set_pcadc<F: FnOnce(Pcadc) -> Pcadc>(&self, f: F) -> &Self {
      let value = f(Pcadc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x938) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCADC register."]
   #[inline] fn with_pcadc<F: FnOnce(Pcadc) -> Pcadc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcadc(::core::ptr::read_volatile((self.base() + 0x938) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x938) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCACMP register."]
   #[inline] fn pcacmp_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x93c)
   }
#[doc="Get the *mut pointer for the PCACMP register."]
   #[inline] fn pcacmp_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x93c)
   }
#[doc="Read the PCACMP register."]
   #[inline] fn pcacmp(&self) -> Pcacmp { 
      unsafe {
         Pcacmp(::core::ptr::read_volatile((self.base() + 0x93c) as *const u32))
      }
   }
#[doc="Write the PCACMP register."]
   #[inline] fn set_pcacmp<F: FnOnce(Pcacmp) -> Pcacmp>(&self, f: F) -> &Self {
      let value = f(Pcacmp(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x93c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCACMP register."]
   #[inline] fn with_pcacmp<F: FnOnce(Pcacmp) -> Pcacmp>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcacmp(::core::ptr::read_volatile((self.base() + 0x93c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x93c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCPWM register."]
   #[inline] fn pcpwm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x940)
   }
#[doc="Get the *mut pointer for the PCPWM register."]
   #[inline] fn pcpwm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x940)
   }
#[doc="Read the PCPWM register."]
   #[inline] fn pcpwm(&self) -> Pcpwm { 
      unsafe {
         Pcpwm(::core::ptr::read_volatile((self.base() + 0x940) as *const u32))
      }
   }
#[doc="Write the PCPWM register."]
   #[inline] fn set_pcpwm<F: FnOnce(Pcpwm) -> Pcpwm>(&self, f: F) -> &Self {
      let value = f(Pcpwm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x940) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCPWM register."]
   #[inline] fn with_pcpwm<F: FnOnce(Pcpwm) -> Pcpwm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcpwm(::core::ptr::read_volatile((self.base() + 0x940) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x940) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCQEI register."]
   #[inline] fn pcqei_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x944)
   }
#[doc="Get the *mut pointer for the PCQEI register."]
   #[inline] fn pcqei_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x944)
   }
#[doc="Read the PCQEI register."]
   #[inline] fn pcqei(&self) -> Pcqei { 
      unsafe {
         Pcqei(::core::ptr::read_volatile((self.base() + 0x944) as *const u32))
      }
   }
#[doc="Write the PCQEI register."]
   #[inline] fn set_pcqei<F: FnOnce(Pcqei) -> Pcqei>(&self, f: F) -> &Self {
      let value = f(Pcqei(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x944) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCQEI register."]
   #[inline] fn with_pcqei<F: FnOnce(Pcqei) -> Pcqei>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcqei(::core::ptr::read_volatile((self.base() + 0x944) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x944) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCEEPROM register."]
   #[inline] fn pceeprom_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x958)
   }
#[doc="Get the *mut pointer for the PCEEPROM register."]
   #[inline] fn pceeprom_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x958)
   }
#[doc="Read the PCEEPROM register."]
   #[inline] fn pceeprom(&self) -> Pceeprom { 
      unsafe {
         Pceeprom(::core::ptr::read_volatile((self.base() + 0x958) as *const u32))
      }
   }
#[doc="Write the PCEEPROM register."]
   #[inline] fn set_pceeprom<F: FnOnce(Pceeprom) -> Pceeprom>(&self, f: F) -> &Self {
      let value = f(Pceeprom(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x958) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCEEPROM register."]
   #[inline] fn with_pceeprom<F: FnOnce(Pceeprom) -> Pceeprom>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pceeprom(::core::ptr::read_volatile((self.base() + 0x958) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x958) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCCCM register."]
   #[inline] fn pcccm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x974)
   }
#[doc="Get the *mut pointer for the PCCCM register."]
   #[inline] fn pcccm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x974)
   }
#[doc="Read the PCCCM register."]
   #[inline] fn pcccm(&self) -> Pcccm { 
      unsafe {
         Pcccm(::core::ptr::read_volatile((self.base() + 0x974) as *const u32))
      }
   }
#[doc="Write the PCCCM register."]
   #[inline] fn set_pcccm<F: FnOnce(Pcccm) -> Pcccm>(&self, f: F) -> &Self {
      let value = f(Pcccm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x974) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCCCM register."]
   #[inline] fn with_pcccm<F: FnOnce(Pcccm) -> Pcccm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcccm(::core::ptr::read_volatile((self.base() + 0x974) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x974) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PCEMAC register."]
   #[inline] fn pcemac_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x99c)
   }
#[doc="Get the *mut pointer for the PCEMAC register."]
   #[inline] fn pcemac_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x99c)
   }
#[doc="Read the PCEMAC register."]
   #[inline] fn pcemac(&self) -> Pcemac { 
      unsafe {
         Pcemac(::core::ptr::read_volatile((self.base() + 0x99c) as *const u32))
      }
   }
#[doc="Write the PCEMAC register."]
   #[inline] fn set_pcemac<F: FnOnce(Pcemac) -> Pcemac>(&self, f: F) -> &Self {
      let value = f(Pcemac(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x99c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PCEMAC register."]
   #[inline] fn with_pcemac<F: FnOnce(Pcemac) -> Pcemac>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pcemac(::core::ptr::read_volatile((self.base() + 0x99c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x99c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRWD register."]
   #[inline] fn prwd_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa00)
   }
#[doc="Get the *mut pointer for the PRWD register."]
   #[inline] fn prwd_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa00)
   }
#[doc="Read the PRWD register."]
   #[inline] fn prwd(&self) -> Prwd { 
      unsafe {
         Prwd(::core::ptr::read_volatile((self.base() + 0xa00) as *const u32))
      }
   }
#[doc="Write the PRWD register."]
   #[inline] fn set_prwd<F: FnOnce(Prwd) -> Prwd>(&self, f: F) -> &Self {
      let value = f(Prwd(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa00) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRWD register."]
   #[inline] fn with_prwd<F: FnOnce(Prwd) -> Prwd>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prwd(::core::ptr::read_volatile((self.base() + 0xa00) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa00) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRTIMER register."]
   #[inline] fn prtimer_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa04)
   }
#[doc="Get the *mut pointer for the PRTIMER register."]
   #[inline] fn prtimer_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa04)
   }
#[doc="Read the PRTIMER register."]
   #[inline] fn prtimer(&self) -> Prtimer { 
      unsafe {
         Prtimer(::core::ptr::read_volatile((self.base() + 0xa04) as *const u32))
      }
   }
#[doc="Write the PRTIMER register."]
   #[inline] fn set_prtimer<F: FnOnce(Prtimer) -> Prtimer>(&self, f: F) -> &Self {
      let value = f(Prtimer(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa04) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRTIMER register."]
   #[inline] fn with_prtimer<F: FnOnce(Prtimer) -> Prtimer>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prtimer(::core::ptr::read_volatile((self.base() + 0xa04) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa04) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRGPIO register."]
   #[inline] fn prgpio_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa08)
   }
#[doc="Get the *mut pointer for the PRGPIO register."]
   #[inline] fn prgpio_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa08)
   }
#[doc="Read the PRGPIO register."]
   #[inline] fn prgpio(&self) -> Prgpio { 
      unsafe {
         Prgpio(::core::ptr::read_volatile((self.base() + 0xa08) as *const u32))
      }
   }
#[doc="Write the PRGPIO register."]
   #[inline] fn set_prgpio<F: FnOnce(Prgpio) -> Prgpio>(&self, f: F) -> &Self {
      let value = f(Prgpio(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa08) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRGPIO register."]
   #[inline] fn with_prgpio<F: FnOnce(Prgpio) -> Prgpio>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prgpio(::core::ptr::read_volatile((self.base() + 0xa08) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa08) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRDMA register."]
   #[inline] fn prdma_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa0c)
   }
#[doc="Get the *mut pointer for the PRDMA register."]
   #[inline] fn prdma_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa0c)
   }
#[doc="Read the PRDMA register."]
   #[inline] fn prdma(&self) -> Prdma { 
      unsafe {
         Prdma(::core::ptr::read_volatile((self.base() + 0xa0c) as *const u32))
      }
   }
#[doc="Write the PRDMA register."]
   #[inline] fn set_prdma<F: FnOnce(Prdma) -> Prdma>(&self, f: F) -> &Self {
      let value = f(Prdma(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa0c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRDMA register."]
   #[inline] fn with_prdma<F: FnOnce(Prdma) -> Prdma>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prdma(::core::ptr::read_volatile((self.base() + 0xa0c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa0c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PREPI register."]
   #[inline] fn prepi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa10)
   }
#[doc="Get the *mut pointer for the PREPI register."]
   #[inline] fn prepi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa10)
   }
#[doc="Read the PREPI register."]
   #[inline] fn prepi(&self) -> Prepi { 
      unsafe {
         Prepi(::core::ptr::read_volatile((self.base() + 0xa10) as *const u32))
      }
   }
#[doc="Write the PREPI register."]
   #[inline] fn set_prepi<F: FnOnce(Prepi) -> Prepi>(&self, f: F) -> &Self {
      let value = f(Prepi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PREPI register."]
   #[inline] fn with_prepi<F: FnOnce(Prepi) -> Prepi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prepi(::core::ptr::read_volatile((self.base() + 0xa10) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRHIB register."]
   #[inline] fn prhib_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa14)
   }
#[doc="Get the *mut pointer for the PRHIB register."]
   #[inline] fn prhib_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa14)
   }
#[doc="Read the PRHIB register."]
   #[inline] fn prhib(&self) -> Prhib { 
      unsafe {
         Prhib(::core::ptr::read_volatile((self.base() + 0xa14) as *const u32))
      }
   }
#[doc="Write the PRHIB register."]
   #[inline] fn set_prhib<F: FnOnce(Prhib) -> Prhib>(&self, f: F) -> &Self {
      let value = f(Prhib(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRHIB register."]
   #[inline] fn with_prhib<F: FnOnce(Prhib) -> Prhib>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prhib(::core::ptr::read_volatile((self.base() + 0xa14) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRUART register."]
   #[inline] fn pruart_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa18)
   }
#[doc="Get the *mut pointer for the PRUART register."]
   #[inline] fn pruart_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa18)
   }
#[doc="Read the PRUART register."]
   #[inline] fn pruart(&self) -> Pruart { 
      unsafe {
         Pruart(::core::ptr::read_volatile((self.base() + 0xa18) as *const u32))
      }
   }
#[doc="Write the PRUART register."]
   #[inline] fn set_pruart<F: FnOnce(Pruart) -> Pruart>(&self, f: F) -> &Self {
      let value = f(Pruart(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRUART register."]
   #[inline] fn with_pruart<F: FnOnce(Pruart) -> Pruart>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pruart(::core::ptr::read_volatile((self.base() + 0xa18) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa18) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRSSI register."]
   #[inline] fn prssi_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa1c)
   }
#[doc="Get the *mut pointer for the PRSSI register."]
   #[inline] fn prssi_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa1c)
   }
#[doc="Read the PRSSI register."]
   #[inline] fn prssi(&self) -> Prssi { 
      unsafe {
         Prssi(::core::ptr::read_volatile((self.base() + 0xa1c) as *const u32))
      }
   }
#[doc="Write the PRSSI register."]
   #[inline] fn set_prssi<F: FnOnce(Prssi) -> Prssi>(&self, f: F) -> &Self {
      let value = f(Prssi(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa1c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRSSI register."]
   #[inline] fn with_prssi<F: FnOnce(Prssi) -> Prssi>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prssi(::core::ptr::read_volatile((self.base() + 0xa1c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa1c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRI2C register."]
   #[inline] fn pri2c_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa20)
   }
#[doc="Get the *mut pointer for the PRI2C register."]
   #[inline] fn pri2c_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa20)
   }
#[doc="Read the PRI2C register."]
   #[inline] fn pri2c(&self) -> Pri2c { 
      unsafe {
         Pri2c(::core::ptr::read_volatile((self.base() + 0xa20) as *const u32))
      }
   }
#[doc="Write the PRI2C register."]
   #[inline] fn set_pri2c<F: FnOnce(Pri2c) -> Pri2c>(&self, f: F) -> &Self {
      let value = f(Pri2c(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRI2C register."]
   #[inline] fn with_pri2c<F: FnOnce(Pri2c) -> Pri2c>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pri2c(::core::ptr::read_volatile((self.base() + 0xa20) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRUSB register."]
   #[inline] fn prusb_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa28)
   }
#[doc="Get the *mut pointer for the PRUSB register."]
   #[inline] fn prusb_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa28)
   }
#[doc="Read the PRUSB register."]
   #[inline] fn prusb(&self) -> Prusb { 
      unsafe {
         Prusb(::core::ptr::read_volatile((self.base() + 0xa28) as *const u32))
      }
   }
#[doc="Write the PRUSB register."]
   #[inline] fn set_prusb<F: FnOnce(Prusb) -> Prusb>(&self, f: F) -> &Self {
      let value = f(Prusb(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRUSB register."]
   #[inline] fn with_prusb<F: FnOnce(Prusb) -> Prusb>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prusb(::core::ptr::read_volatile((self.base() + 0xa28) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa28) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PREPHY register."]
   #[inline] fn prephy_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa30)
   }
#[doc="Get the *mut pointer for the PREPHY register."]
   #[inline] fn prephy_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa30)
   }
#[doc="Read the PREPHY register."]
   #[inline] fn prephy(&self) -> Prephy { 
      unsafe {
         Prephy(::core::ptr::read_volatile((self.base() + 0xa30) as *const u32))
      }
   }
#[doc="Write the PREPHY register."]
   #[inline] fn set_prephy<F: FnOnce(Prephy) -> Prephy>(&self, f: F) -> &Self {
      let value = f(Prephy(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa30) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PREPHY register."]
   #[inline] fn with_prephy<F: FnOnce(Prephy) -> Prephy>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prephy(::core::ptr::read_volatile((self.base() + 0xa30) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa30) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRCAN register."]
   #[inline] fn prcan_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa34)
   }
#[doc="Get the *mut pointer for the PRCAN register."]
   #[inline] fn prcan_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa34)
   }
#[doc="Read the PRCAN register."]
   #[inline] fn prcan(&self) -> Prcan { 
      unsafe {
         Prcan(::core::ptr::read_volatile((self.base() + 0xa34) as *const u32))
      }
   }
#[doc="Write the PRCAN register."]
   #[inline] fn set_prcan<F: FnOnce(Prcan) -> Prcan>(&self, f: F) -> &Self {
      let value = f(Prcan(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa34) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRCAN register."]
   #[inline] fn with_prcan<F: FnOnce(Prcan) -> Prcan>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prcan(::core::ptr::read_volatile((self.base() + 0xa34) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa34) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRADC register."]
   #[inline] fn pradc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa38)
   }
#[doc="Get the *mut pointer for the PRADC register."]
   #[inline] fn pradc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa38)
   }
#[doc="Read the PRADC register."]
   #[inline] fn pradc(&self) -> Pradc { 
      unsafe {
         Pradc(::core::ptr::read_volatile((self.base() + 0xa38) as *const u32))
      }
   }
#[doc="Write the PRADC register."]
   #[inline] fn set_pradc<F: FnOnce(Pradc) -> Pradc>(&self, f: F) -> &Self {
      let value = f(Pradc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa38) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRADC register."]
   #[inline] fn with_pradc<F: FnOnce(Pradc) -> Pradc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pradc(::core::ptr::read_volatile((self.base() + 0xa38) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa38) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRACMP register."]
   #[inline] fn pracmp_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa3c)
   }
#[doc="Get the *mut pointer for the PRACMP register."]
   #[inline] fn pracmp_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa3c)
   }
#[doc="Read the PRACMP register."]
   #[inline] fn pracmp(&self) -> Pracmp { 
      unsafe {
         Pracmp(::core::ptr::read_volatile((self.base() + 0xa3c) as *const u32))
      }
   }
#[doc="Write the PRACMP register."]
   #[inline] fn set_pracmp<F: FnOnce(Pracmp) -> Pracmp>(&self, f: F) -> &Self {
      let value = f(Pracmp(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa3c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRACMP register."]
   #[inline] fn with_pracmp<F: FnOnce(Pracmp) -> Pracmp>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pracmp(::core::ptr::read_volatile((self.base() + 0xa3c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa3c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRPWM register."]
   #[inline] fn prpwm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa40)
   }
#[doc="Get the *mut pointer for the PRPWM register."]
   #[inline] fn prpwm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa40)
   }
#[doc="Read the PRPWM register."]
   #[inline] fn prpwm(&self) -> Prpwm { 
      unsafe {
         Prpwm(::core::ptr::read_volatile((self.base() + 0xa40) as *const u32))
      }
   }
#[doc="Write the PRPWM register."]
   #[inline] fn set_prpwm<F: FnOnce(Prpwm) -> Prpwm>(&self, f: F) -> &Self {
      let value = f(Prpwm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa40) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRPWM register."]
   #[inline] fn with_prpwm<F: FnOnce(Prpwm) -> Prpwm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prpwm(::core::ptr::read_volatile((self.base() + 0xa40) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa40) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRQEI register."]
   #[inline] fn prqei_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa44)
   }
#[doc="Get the *mut pointer for the PRQEI register."]
   #[inline] fn prqei_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa44)
   }
#[doc="Read the PRQEI register."]
   #[inline] fn prqei(&self) -> Prqei { 
      unsafe {
         Prqei(::core::ptr::read_volatile((self.base() + 0xa44) as *const u32))
      }
   }
#[doc="Write the PRQEI register."]
   #[inline] fn set_prqei<F: FnOnce(Prqei) -> Prqei>(&self, f: F) -> &Self {
      let value = f(Prqei(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa44) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRQEI register."]
   #[inline] fn with_prqei<F: FnOnce(Prqei) -> Prqei>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prqei(::core::ptr::read_volatile((self.base() + 0xa44) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa44) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PREEPROM register."]
   #[inline] fn preeprom_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa58)
   }
#[doc="Get the *mut pointer for the PREEPROM register."]
   #[inline] fn preeprom_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa58)
   }
#[doc="Read the PREEPROM register."]
   #[inline] fn preeprom(&self) -> Preeprom { 
      unsafe {
         Preeprom(::core::ptr::read_volatile((self.base() + 0xa58) as *const u32))
      }
   }
#[doc="Write the PREEPROM register."]
   #[inline] fn set_preeprom<F: FnOnce(Preeprom) -> Preeprom>(&self, f: F) -> &Self {
      let value = f(Preeprom(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa58) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PREEPROM register."]
   #[inline] fn with_preeprom<F: FnOnce(Preeprom) -> Preeprom>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Preeprom(::core::ptr::read_volatile((self.base() + 0xa58) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa58) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PRCCM register."]
   #[inline] fn prccm_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa74)
   }
#[doc="Get the *mut pointer for the PRCCM register."]
   #[inline] fn prccm_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa74)
   }
#[doc="Read the PRCCM register."]
   #[inline] fn prccm(&self) -> Prccm { 
      unsafe {
         Prccm(::core::ptr::read_volatile((self.base() + 0xa74) as *const u32))
      }
   }
#[doc="Write the PRCCM register."]
   #[inline] fn set_prccm<F: FnOnce(Prccm) -> Prccm>(&self, f: F) -> &Self {
      let value = f(Prccm(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa74) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PRCCM register."]
   #[inline] fn with_prccm<F: FnOnce(Prccm) -> Prccm>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Prccm(::core::ptr::read_volatile((self.base() + 0xa74) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa74) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the PREMAC register."]
   #[inline] fn premac_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xa9c)
   }
#[doc="Get the *mut pointer for the PREMAC register."]
   #[inline] fn premac_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xa9c)
   }
#[doc="Read the PREMAC register."]
   #[inline] fn premac(&self) -> Premac { 
      unsafe {
         Premac(::core::ptr::read_volatile((self.base() + 0xa9c) as *const u32))
      }
   }
#[doc="Write the PREMAC register."]
   #[inline] fn set_premac<F: FnOnce(Premac) -> Premac>(&self, f: F) -> &Self {
      let value = f(Premac(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa9c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PREMAC register."]
   #[inline] fn with_premac<F: FnOnce(Premac) -> Premac>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Premac(::core::ptr::read_volatile((self.base() + 0xa9c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xa9c) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Device Identification 0"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Did0(pub u32);
impl Did0 {
#[doc="Minor Revision"]
   #[inline] pub fn min(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Minor Revision"]
   #[inline] pub fn set_min<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Major Revision"]
   #[inline] pub fn maj(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Major Revision"]
   #[inline] pub fn set_maj<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Device Class"]
   #[inline] pub fn class(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Device Class"]
   #[inline] pub fn set_class<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="DID0 Version"]
   #[inline] pub fn ver(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x7) as u8) } // [30:28]
   }
#[doc="DID0 Version"]
   #[inline] pub fn set_ver<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 28);
      self.0 |= value << 28;
      self
   }

}
impl ::core::fmt::Display for Did0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Did0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.min() != 0 { try!(write!(f, " min=0x{:x}", self.min()))}
      if self.maj() != 0 { try!(write!(f, " maj=0x{:x}", self.maj()))}
      if self.class() != 0 { try!(write!(f, " class=0x{:x}", self.class()))}
      if self.ver() != 0 { try!(write!(f, " ver=0x{:x}", self.ver()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Device Identification 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Did1(pub u32);
impl Did1 {
#[doc="Qualification Status"]
   #[inline] pub fn qual(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Qualification Status"]
   #[inline] pub fn set_qual<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="RoHS-Compliance"]
   #[inline] pub fn rohs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="RoHS-Compliance"]
   #[inline] pub fn set_rohs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Package Type"]
   #[inline] pub fn pkg(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
   }
#[doc="Package Type"]
   #[inline] pub fn set_pkg<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Temperature Range"]
   #[inline] pub fn temp(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
   }
#[doc="Temperature Range"]
   #[inline] pub fn set_temp<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Package Pin Count"]
   #[inline] pub fn pincnt(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
   }
#[doc="Package Pin Count"]
   #[inline] pub fn set_pincnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="Part Number"]
   #[inline] pub fn prtno(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Part Number"]
   #[inline] pub fn set_prtno<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Family"]
   #[inline] pub fn fam(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
   }
#[doc="Family"]
   #[inline] pub fn set_fam<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 24);
      self.0 |= value << 24;
      self
   }

#[doc="DID1 Version"]
   #[inline] pub fn ver(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
   }
#[doc="DID1 Version"]
   #[inline] pub fn set_ver<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 28);
      self.0 |= value << 28;
      self
   }

}
impl ::core::fmt::Display for Did1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Did1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.qual() != 0 { try!(write!(f, " qual=0x{:x}", self.qual()))}
      if self.rohs() != 0 { try!(write!(f, " rohs"))}
      if self.pkg() != 0 { try!(write!(f, " pkg=0x{:x}", self.pkg()))}
      if self.temp() != 0 { try!(write!(f, " temp=0x{:x}", self.temp()))}
      if self.pincnt() != 0 { try!(write!(f, " pincnt=0x{:x}", self.pincnt()))}
      if self.prtno() != 0 { try!(write!(f, " prtno=0x{:x}", self.prtno()))}
      if self.fam() != 0 { try!(write!(f, " fam=0x{:x}", self.fam()))}
      if self.ver() != 0 { try!(write!(f, " ver=0x{:x}", self.ver()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Power-Temp Brown Out Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ptboctl(pub u32);
impl Ptboctl {
#[doc="VDD (VDDS) under BOR Event Action"]
   #[inline] pub fn vdd_ubor(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="VDD (VDDS) under BOR Event Action"]
   #[inline] pub fn set_vdd_ubor<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="VDDA under BOR Event Action"]
   #[inline] pub fn vdda_ubor(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
   }
#[doc="VDDA under BOR Event Action"]
   #[inline] pub fn set_vdda_ubor<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Ptboctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ptboctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vdd_ubor() != 0 { try!(write!(f, " vdd_ubor=0x{:x}", self.vdd_ubor()))}
      if self.vdda_ubor() != 0 { try!(write!(f, " vdda_ubor=0x{:x}", self.vdda_ubor()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Raw Interrupt Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
#[doc="Brown-Out Reset Raw Interrupt Status"]
   #[inline] pub fn borris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Brown-Out Reset Raw Interrupt Status"]
   #[inline] pub fn set_borris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Main Oscillator Failure Raw Interrupt Status"]
   #[inline] pub fn mofris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Main Oscillator Failure Raw Interrupt Status"]
   #[inline] pub fn set_mofris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="PLL Lock Raw Interrupt Status"]
   #[inline] pub fn plllris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="PLL Lock Raw Interrupt Status"]
   #[inline] pub fn set_plllris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="MOSC Power Up Raw Interrupt Status"]
   #[inline] pub fn moscpupris(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="MOSC Power Up Raw Interrupt Status"]
   #[inline] pub fn set_moscpupris<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
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
      if self.borris() != 0 { try!(write!(f, " borris"))}
      if self.mofris() != 0 { try!(write!(f, " mofris"))}
      if self.plllris() != 0 { try!(write!(f, " plllris"))}
      if self.moscpupris() != 0 { try!(write!(f, " moscpupris"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Mask Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Imc(pub u32);
impl Imc {
#[doc="Brown-Out Reset Interrupt Mask"]
   #[inline] pub fn borim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Brown-Out Reset Interrupt Mask"]
   #[inline] pub fn set_borim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Main Oscillator Failure Interrupt Mask"]
   #[inline] pub fn mofim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Main Oscillator Failure Interrupt Mask"]
   #[inline] pub fn set_mofim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="PLL Lock Interrupt Mask"]
   #[inline] pub fn plllim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="PLL Lock Interrupt Mask"]
   #[inline] pub fn set_plllim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="MOSC Power Up Interrupt Mask"]
   #[inline] pub fn moscpupim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="MOSC Power Up Interrupt Mask"]
   #[inline] pub fn set_moscpupim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Imc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Imc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.borim() != 0 { try!(write!(f, " borim"))}
      if self.mofim() != 0 { try!(write!(f, " mofim"))}
      if self.plllim() != 0 { try!(write!(f, " plllim"))}
      if self.moscpupim() != 0 { try!(write!(f, " moscpupim"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Masked Interrupt Status and Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Misc(pub u32);
impl Misc {
#[doc="BOR Masked Interrupt Status"]
   #[inline] pub fn bormis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="BOR Masked Interrupt Status"]
   #[inline] pub fn set_bormis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Main Oscillator Failure Masked Interrupt Status"]
   #[inline] pub fn mofmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Main Oscillator Failure Masked Interrupt Status"]
   #[inline] pub fn set_mofmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="PLL Lock Masked Interrupt Status"]
   #[inline] pub fn plllmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="PLL Lock Masked Interrupt Status"]
   #[inline] pub fn set_plllmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="MOSC Power Up Masked Interrupt Status"]
   #[inline] pub fn moscpupmis(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="MOSC Power Up Masked Interrupt Status"]
   #[inline] pub fn set_moscpupmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Misc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Misc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bormis() != 0 { try!(write!(f, " bormis"))}
      if self.mofmis() != 0 { try!(write!(f, " mofmis"))}
      if self.plllmis() != 0 { try!(write!(f, " plllmis"))}
      if self.moscpupmis() != 0 { try!(write!(f, " moscpupmis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Reset Cause"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Resc(pub u32);
impl Resc {
#[doc="External Reset"]
   #[inline] pub fn ext(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="External Reset"]
   #[inline] pub fn set_ext<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Power-On Reset"]
   #[inline] pub fn por(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Power-On Reset"]
   #[inline] pub fn set_por<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Brown-Out Reset"]
   #[inline] pub fn bor(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Brown-Out Reset"]
   #[inline] pub fn set_bor<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Watchdog Timer 0 Reset"]
   #[inline] pub fn wdt0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Watchdog Timer 0 Reset"]
   #[inline] pub fn set_wdt0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Software Reset"]
   #[inline] pub fn sw(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Software Reset"]
   #[inline] pub fn set_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Watchdog Timer 1 Reset"]
   #[inline] pub fn wdt1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Watchdog Timer 1 Reset"]
   #[inline] pub fn set_wdt1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="HIB Reset"]
   #[inline] pub fn hib(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="HIB Reset"]
   #[inline] pub fn set_hib<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="HSSR Reset"]
   #[inline] pub fn hssr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="HSSR Reset"]
   #[inline] pub fn set_hssr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="MOSC Failure Reset"]
   #[inline] pub fn moscfail(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="MOSC Failure Reset"]
   #[inline] pub fn set_moscfail<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Resc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Resc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ext() != 0 { try!(write!(f, " ext"))}
      if self.por() != 0 { try!(write!(f, " por"))}
      if self.bor() != 0 { try!(write!(f, " bor"))}
      if self.wdt0() != 0 { try!(write!(f, " wdt0"))}
      if self.sw() != 0 { try!(write!(f, " sw"))}
      if self.wdt1() != 0 { try!(write!(f, " wdt1"))}
      if self.hib() != 0 { try!(write!(f, " hib"))}
      if self.hssr() != 0 { try!(write!(f, " hssr"))}
      if self.moscfail() != 0 { try!(write!(f, " moscfail"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Power-Temperature Cause"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pwrtc(pub u32);
impl Pwrtc {
#[doc="VDD Under BOR Status"]
   #[inline] pub fn vdd_ubor(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="VDD Under BOR Status"]
   #[inline] pub fn set_vdd_ubor<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="VDDA Under BOR Status"]
   #[inline] pub fn vdda_ubor(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="VDDA Under BOR Status"]
   #[inline] pub fn set_vdda_ubor<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Pwrtc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pwrtc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vdd_ubor() != 0 { try!(write!(f, " vdd_ubor"))}
      if self.vdda_ubor() != 0 { try!(write!(f, " vdda_ubor"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="NMI Cause Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Nmic(pub u32);
impl Nmic {
#[doc="External Pin NMI"]
   #[inline] pub fn external(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="External Pin NMI"]
   #[inline] pub fn set_external<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Power/Brown Out Event NMI"]
   #[inline] pub fn power(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Power/Brown Out Event NMI"]
   #[inline] pub fn set_power<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Watch Dog Timer (WDT) 0 NMI"]
   #[inline] pub fn wdt0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Watch Dog Timer (WDT) 0 NMI"]
   #[inline] pub fn set_wdt0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Watch Dog Timer (WDT) 1 NMI"]
   #[inline] pub fn wdt1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Watch Dog Timer (WDT) 1 NMI"]
   #[inline] pub fn set_wdt1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Tamper Event NMI"]
   #[inline] pub fn tamper(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Tamper Event NMI"]
   #[inline] pub fn set_tamper<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="MOSC Failure NMI"]
   #[inline] pub fn moscfail(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="MOSC Failure NMI"]
   #[inline] pub fn set_moscfail<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Nmic {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Nmic {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.external() != 0 { try!(write!(f, " external"))}
      if self.power() != 0 { try!(write!(f, " power"))}
      if self.wdt0() != 0 { try!(write!(f, " wdt0"))}
      if self.wdt1() != 0 { try!(write!(f, " wdt1"))}
      if self.tamper() != 0 { try!(write!(f, " tamper"))}
      if self.moscfail() != 0 { try!(write!(f, " moscfail"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Main Oscillator Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Moscctl(pub u32);
impl Moscctl {
#[doc="Clock Validation for MOSC"]
   #[inline] pub fn cval(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Clock Validation for MOSC"]
   #[inline] pub fn set_cval<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="MOSC Failure Action"]
   #[inline] pub fn moscim(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="MOSC Failure Action"]
   #[inline] pub fn set_moscim<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="No Crystal Connected"]
   #[inline] pub fn noxtal(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="No Crystal Connected"]
   #[inline] pub fn set_noxtal<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Power Down"]
   #[inline] pub fn pwrdn(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Power Down"]
   #[inline] pub fn set_pwrdn<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Oscillator Range"]
   #[inline] pub fn oscrng(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Oscillator Range"]
   #[inline] pub fn set_oscrng<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Moscctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Moscctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cval() != 0 { try!(write!(f, " cval"))}
      if self.moscim() != 0 { try!(write!(f, " moscim"))}
      if self.noxtal() != 0 { try!(write!(f, " noxtal"))}
      if self.pwrdn() != 0 { try!(write!(f, " pwrdn"))}
      if self.oscrng() != 0 { try!(write!(f, " oscrng"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Run and Sleep Mode Configuration Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rsclkcfg(pub u32);
impl Rsclkcfg {
#[doc="PLL System Clock Divisor"]
   #[inline] pub fn psysdiv(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="PLL System Clock Divisor"]
   #[inline] pub fn set_psysdiv<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Oscillator System Clock Divisor"]
   #[inline] pub fn osysdiv(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3ff) as u16) } // [19:10]
   }
#[doc="Oscillator System Clock Divisor"]
   #[inline] pub fn set_osysdiv<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Oscillator Source"]
   #[inline] pub fn oscsrc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
   }
#[doc="Oscillator Source"]
   #[inline] pub fn set_oscsrc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 20);
      self.0 |= value << 20;
      self
   }

#[doc="PLL Source"]
   #[inline] pub fn pllsrc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
   }
#[doc="PLL Source"]
   #[inline] pub fn set_pllsrc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Use PLL"]
   #[inline] pub fn usepll(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="Use PLL"]
   #[inline] pub fn set_usepll<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="Auto Clock Gating"]
   #[inline] pub fn acg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
   }
#[doc="Auto Clock Gating"]
   #[inline] pub fn set_acg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 29);
      self.0 |= value << 29;
      self
   }

#[doc="New PLLFREQ Accept"]
   #[inline] pub fn newfreq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="New PLLFREQ Accept"]
   #[inline] pub fn set_newfreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Memory Timing Register Update"]
   #[inline] pub fn memtimu(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Memory Timing Register Update"]
   #[inline] pub fn set_memtimu<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Rsclkcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rsclkcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.psysdiv() != 0 { try!(write!(f, " psysdiv=0x{:x}", self.psysdiv()))}
      if self.osysdiv() != 0 { try!(write!(f, " osysdiv=0x{:x}", self.osysdiv()))}
      if self.oscsrc() != 0 { try!(write!(f, " oscsrc=0x{:x}", self.oscsrc()))}
      if self.pllsrc() != 0 { try!(write!(f, " pllsrc=0x{:x}", self.pllsrc()))}
      if self.usepll() != 0 { try!(write!(f, " usepll"))}
      if self.acg() != 0 { try!(write!(f, " acg"))}
      if self.newfreq() != 0 { try!(write!(f, " newfreq"))}
      if self.memtimu() != 0 { try!(write!(f, " memtimu"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Memory Timing Parameter Register 0 for Main Flash and EEPROM"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Memtim0(pub u32);
impl Memtim0 {
#[doc="Flash Wait State"]
   #[inline] pub fn fws(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Flash Wait State"]
   #[inline] pub fn set_fws<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Flash Bank Clock Edge"]
   #[inline] pub fn fbce(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Flash Bank Clock Edge"]
   #[inline] pub fn set_fbce<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Flash Bank Clock High Time"]
   #[inline] pub fn fbcht(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0xf) as u8) } // [9:6]
   }
#[doc="Flash Bank Clock High Time"]
   #[inline] pub fn set_fbcht<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 6);
      self.0 |= value << 6;
      self
   }

#[doc="EEPROM Wait States"]
   #[inline] pub fn ews(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
   }
#[doc="EEPROM Wait States"]
   #[inline] pub fn set_ews<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 16);
      self.0 |= value << 16;
      self
   }

#[doc="EEPROM Bank Clock Edge"]
   #[inline] pub fn ebce(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
   }
#[doc="EEPROM Bank Clock Edge"]
   #[inline] pub fn set_ebce<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 21);
      self.0 |= value << 21;
      self
   }

#[doc="EEPROM Clock High Time"]
   #[inline] pub fn ebcht(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0xf) as u8) } // [25:22]
   }
#[doc="EEPROM Clock High Time"]
   #[inline] pub fn set_ebcht<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 22);
      self.0 |= value << 22;
      self
   }

}
impl ::core::fmt::Display for Memtim0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Memtim0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fws() != 0 { try!(write!(f, " fws=0x{:x}", self.fws()))}
      if self.fbce() != 0 { try!(write!(f, " fbce"))}
      if self.fbcht() != 0 { try!(write!(f, " fbcht=0x{:x}", self.fbcht()))}
      if self.ews() != 0 { try!(write!(f, " ews=0x{:x}", self.ews()))}
      if self.ebce() != 0 { try!(write!(f, " ebce"))}
      if self.ebcht() != 0 { try!(write!(f, " ebcht=0x{:x}", self.ebcht()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Alternate Clock Configuration"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Altclkcfg(pub u32);
impl Altclkcfg {
#[doc="Alternate Clock Source"]
   #[inline] pub fn altclk(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Alternate Clock Source"]
   #[inline] pub fn set_altclk<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Altclkcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Altclkcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.altclk() != 0 { try!(write!(f, " altclk=0x{:x}", self.altclk()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Deep Sleep Clock Configuration Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dsclkcfg(pub u32);
impl Dsclkcfg {
#[doc="Deep Sleep Clock Divisor"]
   #[inline] pub fn dssysdiv(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="Deep Sleep Clock Divisor"]
   #[inline] pub fn set_dssysdiv<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Deep Sleep Oscillator Source"]
   #[inline] pub fn dsoscsrc(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
   }
#[doc="Deep Sleep Oscillator Source"]
   #[inline] pub fn set_dsoscsrc<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 20);
      self.0 |= value << 20;
      self
   }

#[doc="MOSC Disable Power Down"]
   #[inline] pub fn moscdpd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="MOSC Disable Power Down"]
   #[inline] pub fn set_moscdpd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="PIOSC Power Down"]
   #[inline] pub fn pioscpd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="PIOSC Power Down"]
   #[inline] pub fn set_pioscpd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Dsclkcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dsclkcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dssysdiv() != 0 { try!(write!(f, " dssysdiv=0x{:x}", self.dssysdiv()))}
      if self.dsoscsrc() != 0 { try!(write!(f, " dsoscsrc=0x{:x}", self.dsoscsrc()))}
      if self.moscdpd() != 0 { try!(write!(f, " moscdpd"))}
      if self.pioscpd() != 0 { try!(write!(f, " pioscpd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Divisor and Source Clock Configuration"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Divsclk(pub u32);
impl Divsclk {
#[doc="Divisor Value"]
   #[inline] pub fn div(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Divisor Value"]
   #[inline] pub fn set_div<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Clock Source"]
   #[inline] pub fn src(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
   }
#[doc="Clock Source"]
   #[inline] pub fn set_src<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="DIVSCLK Enable"]
   #[inline] pub fn en(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="DIVSCLK Enable"]
   #[inline] pub fn set_en<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Divsclk {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Divsclk {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
      if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
      if self.en() != 0 { try!(write!(f, " en"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Properties"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sysprop(pub u32);
impl Sysprop {
#[doc="FPU Present"]
   #[inline] pub fn fpu(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="FPU Present"]
   #[inline] pub fn set_fpu<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Sysprop {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sysprop {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fpu() != 0 { try!(write!(f, " fpu"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Precision Internal Oscillator Calibration"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piosccal(pub u32);
impl Piosccal {
#[doc="User Trim Value"]
   #[inline] pub fn ut(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="User Trim Value"]
   #[inline] pub fn set_ut<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Update Trim"]
   #[inline] pub fn update(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Update Trim"]
   #[inline] pub fn set_update<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Start Calibration"]
   #[inline] pub fn cal(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Start Calibration"]
   #[inline] pub fn set_cal<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Use User Trim Value"]
   #[inline] pub fn uten(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Use User Trim Value"]
   #[inline] pub fn set_uten<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Piosccal {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Piosccal {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ut() != 0 { try!(write!(f, " ut=0x{:x}", self.ut()))}
      if self.update() != 0 { try!(write!(f, " update"))}
      if self.cal() != 0 { try!(write!(f, " cal"))}
      if self.uten() != 0 { try!(write!(f, " uten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Precision Internal Oscillator Statistics"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pioscstat(pub u32);
impl Pioscstat {
#[doc="Calibration Trim Value"]
   #[inline] pub fn ct(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
   }
#[doc="Calibration Trim Value"]
   #[inline] pub fn set_ct<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Calibration Result"]
   #[inline] pub fn cr(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
   }
#[doc="Calibration Result"]
   #[inline] pub fn set_cr<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Default Trim Value"]
   #[inline] pub fn dt(&self) -> bits::U7 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
   }
#[doc="Default Trim Value"]
   #[inline] pub fn set_dt<V: Into<bits::U7>>(mut self, value: V) -> Self {
      let value: bits::U7 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7f << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Pioscstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pioscstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ct() != 0 { try!(write!(f, " ct=0x{:x}", self.ct()))}
      if self.cr() != 0 { try!(write!(f, " cr=0x{:x}", self.cr()))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PLL Frequency 0"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pllfreq0(pub u32);
impl Pllfreq0 {
#[doc="PLL M Integer Value"]
   #[inline] pub fn mint(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
   }
#[doc="PLL M Integer Value"]
   #[inline] pub fn set_mint<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="PLL M Fractional Value"]
   #[inline] pub fn mfrac(&self) -> bits::U10 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3ff) as u16) } // [19:10]
   }
#[doc="PLL M Fractional Value"]
   #[inline] pub fn set_mfrac<V: Into<bits::U10>>(mut self, value: V) -> Self {
      let value: bits::U10 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3ff << 10);
      self.0 |= value << 10;
      self
   }

#[doc="PLL Power"]
   #[inline] pub fn pllpwr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
   }
#[doc="PLL Power"]
   #[inline] pub fn set_pllpwr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 23);
      self.0 |= value << 23;
      self
   }

}
impl ::core::fmt::Display for Pllfreq0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pllfreq0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mint() != 0 { try!(write!(f, " mint=0x{:x}", self.mint()))}
      if self.mfrac() != 0 { try!(write!(f, " mfrac=0x{:x}", self.mfrac()))}
      if self.pllpwr() != 0 { try!(write!(f, " pllpwr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PLL Frequency 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pllfreq1(pub u32);
impl Pllfreq1 {
#[doc="PLL N Value"]
   #[inline] pub fn n(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
   }
#[doc="PLL N Value"]
   #[inline] pub fn set_n<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 0);
      self.0 |= value << 0;
      self
   }

#[doc="PLL Q Value"]
   #[inline] pub fn q(&self) -> bits::U5 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
   }
#[doc="PLL Q Value"]
   #[inline] pub fn set_q<V: Into<bits::U5>>(mut self, value: V) -> Self {
      let value: bits::U5 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1f << 8);
      self.0 |= value << 8;
      self
   }

}
impl ::core::fmt::Display for Pllfreq1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pllfreq1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.n() != 0 { try!(write!(f, " n=0x{:x}", self.n()))}
      if self.q() != 0 { try!(write!(f, " q=0x{:x}", self.q()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PLL Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pllstat(pub u32);
impl Pllstat {
#[doc="PLL Lock"]
   #[inline] pub fn lock(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PLL Lock"]
   #[inline] pub fn set_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pllstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pllstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lock() != 0 { try!(write!(f, " lock"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Sleep Power Configuration"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Slppwrcfg(pub u32);
impl Slppwrcfg {
#[doc="SRAM Power Modes"]
   #[inline] pub fn srampm(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="SRAM Power Modes"]
   #[inline] pub fn set_srampm<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Flash Power Modes"]
   #[inline] pub fn flashpm(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }
#[doc="Flash Power Modes"]
   #[inline] pub fn set_flashpm<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

}
impl ::core::fmt::Display for Slppwrcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Slppwrcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.srampm() != 0 { try!(write!(f, " srampm=0x{:x}", self.srampm()))}
      if self.flashpm() != 0 { try!(write!(f, " flashpm=0x{:x}", self.flashpm()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Deep-Sleep Power Configuration"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dslppwrcfg(pub u32);
impl Dslppwrcfg {
#[doc="SRAM Power Modes"]
   #[inline] pub fn srampm(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="SRAM Power Modes"]
   #[inline] pub fn set_srampm<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Flash Power Modes"]
   #[inline] pub fn flashpm(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }
#[doc="Flash Power Modes"]
   #[inline] pub fn set_flashpm<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Temperature Sense Power Down"]
   #[inline] pub fn tspd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Temperature Sense Power Down"]
   #[inline] pub fn set_tspd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="LDO Sleep Mode"]
   #[inline] pub fn ldosm(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="LDO Sleep Mode"]
   #[inline] pub fn set_ldosm<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}
impl ::core::fmt::Display for Dslppwrcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dslppwrcfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.srampm() != 0 { try!(write!(f, " srampm=0x{:x}", self.srampm()))}
      if self.flashpm() != 0 { try!(write!(f, " flashpm=0x{:x}", self.flashpm()))}
      if self.tspd() != 0 { try!(write!(f, " tspd"))}
      if self.ldosm() != 0 { try!(write!(f, " ldosm"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Non-Volatile Memory Information"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Nvmstat(pub u32);
impl Nvmstat {
#[doc="32 Word Flash Write Buffer Available"]
   #[inline] pub fn fwb(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="32 Word Flash Write Buffer Available"]
   #[inline] pub fn set_fwb<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Nvmstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Nvmstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fwb() != 0 { try!(write!(f, " fwb"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LDO Sleep Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ldospctl(pub u32);
impl Ldospctl {
#[doc="LDO Output Voltage"]
   #[inline] pub fn vldo(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="LDO Output Voltage"]
   #[inline] pub fn set_vldo<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Voltage Adjust Enable"]
   #[inline] pub fn vadjen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Voltage Adjust Enable"]
   #[inline] pub fn set_vadjen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Ldospctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ldospctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vldo() != 0 { try!(write!(f, " vldo=0x{:x}", self.vldo()))}
      if self.vadjen() != 0 { try!(write!(f, " vadjen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LDO Deep-Sleep Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ldodpctl(pub u32);
impl Ldodpctl {
#[doc="LDO Output Voltage"]
   #[inline] pub fn vldo(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="LDO Output Voltage"]
   #[inline] pub fn set_vldo<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Voltage Adjust Enable"]
   #[inline] pub fn vadjen(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Voltage Adjust Enable"]
   #[inline] pub fn set_vadjen<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

}
impl ::core::fmt::Display for Ldodpctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ldodpctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vldo() != 0 { try!(write!(f, " vldo=0x{:x}", self.vldo()))}
      if self.vadjen() != 0 { try!(write!(f, " vadjen"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Reset Behavior Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Resbehavctl(pub u32);
impl Resbehavctl {
#[doc="External RST Pin Operation"]
   #[inline] pub fn extres(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="External RST Pin Operation"]
   #[inline] pub fn set_extres<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="BOR Reset operation"]
   #[inline] pub fn bor(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="BOR Reset operation"]
   #[inline] pub fn set_bor<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Watchdog 0 Reset Operation"]
   #[inline] pub fn wdog0(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }
#[doc="Watchdog 0 Reset Operation"]
   #[inline] pub fn set_wdog0<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Watchdog 1 Reset Operation"]
   #[inline] pub fn wdog1(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Watchdog 1 Reset Operation"]
   #[inline] pub fn set_wdog1<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

}
impl ::core::fmt::Display for Resbehavctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Resbehavctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.extres() != 0 { try!(write!(f, " extres=0x{:x}", self.extres()))}
      if self.bor() != 0 { try!(write!(f, " bor=0x{:x}", self.bor()))}
      if self.wdog0() != 0 { try!(write!(f, " wdog0=0x{:x}", self.wdog0()))}
      if self.wdog1() != 0 { try!(write!(f, " wdog1=0x{:x}", self.wdog1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Hardware System Service Request"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Hssr(pub u32);
impl Hssr {
#[doc="Command Descriptor Pointer"]
   #[inline] pub fn cdoff(&self) -> bits::U24 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
   }
#[doc="Command Descriptor Pointer"]
   #[inline] pub fn set_cdoff<V: Into<bits::U24>>(mut self, value: V) -> Self {
      let value: bits::U24 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffff << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Write Key"]
   #[inline] pub fn key(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Write Key"]
   #[inline] pub fn set_key<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Hssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Hssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cdoff() != 0 { try!(write!(f, " cdoff=0x{:x}", self.cdoff()))}
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USB Power Domain Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Usbpds(pub u32);
impl Usbpds {
#[doc="Power Domain Status"]
   #[inline] pub fn pwrstat(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Power Domain Status"]
   #[inline] pub fn set_pwrstat<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Memory Array Power Status"]
   #[inline] pub fn memstat(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="Memory Array Power Status"]
   #[inline] pub fn set_memstat<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

}
impl ::core::fmt::Display for Usbpds {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Usbpds {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwrstat() != 0 { try!(write!(f, " pwrstat=0x{:x}", self.pwrstat()))}
      if self.memstat() != 0 { try!(write!(f, " memstat=0x{:x}", self.memstat()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="USB Memory Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Usbmpc(pub u32);
impl Usbmpc {
#[doc="Memory Array Power Control"]
   #[inline] pub fn pwrctl(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Memory Array Power Control"]
   #[inline] pub fn set_pwrctl<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Usbmpc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Usbmpc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwrctl() != 0 { try!(write!(f, " pwrctl=0x{:x}", self.pwrctl()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC Power Domain Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Emacpds(pub u32);
impl Emacpds {
#[doc="Power Domain Status"]
   #[inline] pub fn pwrstat(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Power Domain Status"]
   #[inline] pub fn set_pwrstat<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Memory Array Power Status"]
   #[inline] pub fn memstat(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="Memory Array Power Status"]
   #[inline] pub fn set_memstat<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

}
impl ::core::fmt::Display for Emacpds {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Emacpds {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwrstat() != 0 { try!(write!(f, " pwrstat=0x{:x}", self.pwrstat()))}
      if self.memstat() != 0 { try!(write!(f, " memstat=0x{:x}", self.memstat()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC Memory Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Emacmpc(pub u32);
impl Emacmpc {
#[doc="Memory Array Power Control"]
   #[inline] pub fn pwrctl(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Memory Array Power Control"]
   #[inline] pub fn set_pwrctl<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Emacmpc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Emacmpc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwrctl() != 0 { try!(write!(f, " pwrctl=0x{:x}", self.pwrctl()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppwd(pub u32);
impl Ppwd {
#[doc="Watchdog Timer 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Watchdog Timer 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Watchdog Timer 1 Present"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Watchdog Timer 1 Present"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Ppwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="16/32-Bit General-Purpose Timer Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pptimer(pub u32);
impl Pptimer {
#[doc="16/32-Bit General-Purpose Timer 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="16/32-Bit General-Purpose Timer 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 1 Present"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="16/32-Bit General-Purpose Timer 1 Present"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 2 Present"]
   #[inline] pub fn p2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="16/32-Bit General-Purpose Timer 2 Present"]
   #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 3 Present"]
   #[inline] pub fn p3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="16/32-Bit General-Purpose Timer 3 Present"]
   #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 4 Present"]
   #[inline] pub fn p4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="16/32-Bit General-Purpose Timer 4 Present"]
   #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 5 Present"]
   #[inline] pub fn p5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="16/32-Bit General-Purpose Timer 5 Present"]
   #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 6 Present"]
   #[inline] pub fn p6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="16/32-Bit General-Purpose Timer 6 Present"]
   #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 7 Present"]
   #[inline] pub fn p7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="16/32-Bit General-Purpose Timer 7 Present"]
   #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Pptimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pptimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      if self.p2() != 0 { try!(write!(f, " p2"))}
      if self.p3() != 0 { try!(write!(f, " p3"))}
      if self.p4() != 0 { try!(write!(f, " p4"))}
      if self.p5() != 0 { try!(write!(f, " p5"))}
      if self.p6() != 0 { try!(write!(f, " p6"))}
      if self.p7() != 0 { try!(write!(f, " p7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="General-Purpose Input/Output Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppgpio(pub u32);
impl Ppgpio {
#[doc="GPIO Port A Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Port A Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="GPIO Port B Present"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="GPIO Port B Present"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="GPIO Port C Present"]
   #[inline] pub fn p2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="GPIO Port C Present"]
   #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="GPIO Port D Present"]
   #[inline] pub fn p3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="GPIO Port D Present"]
   #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="GPIO Port E Present"]
   #[inline] pub fn p4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="GPIO Port E Present"]
   #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="GPIO Port F Present"]
   #[inline] pub fn p5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="GPIO Port F Present"]
   #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="GPIO Port G Present"]
   #[inline] pub fn p6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="GPIO Port G Present"]
   #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="GPIO Port H Present"]
   #[inline] pub fn p7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="GPIO Port H Present"]
   #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="GPIO Port J Present"]
   #[inline] pub fn p8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO Port J Present"]
   #[inline] pub fn set_p8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="GPIO Port K Present"]
   #[inline] pub fn p9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="GPIO Port K Present"]
   #[inline] pub fn set_p9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="GPIO Port L Present"]
   #[inline] pub fn p10(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="GPIO Port L Present"]
   #[inline] pub fn set_p10<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="GPIO Port M Present"]
   #[inline] pub fn p11(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="GPIO Port M Present"]
   #[inline] pub fn set_p11<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="GPIO Port N Present"]
   #[inline] pub fn p12(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="GPIO Port N Present"]
   #[inline] pub fn set_p12<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="GPIO Port P Present"]
   #[inline] pub fn p13(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="GPIO Port P Present"]
   #[inline] pub fn set_p13<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="GPIO Port Q Present"]
   #[inline] pub fn p14(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="GPIO Port Q Present"]
   #[inline] pub fn set_p14<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

}
impl ::core::fmt::Display for Ppgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      if self.p2() != 0 { try!(write!(f, " p2"))}
      if self.p3() != 0 { try!(write!(f, " p3"))}
      if self.p4() != 0 { try!(write!(f, " p4"))}
      if self.p5() != 0 { try!(write!(f, " p5"))}
      if self.p6() != 0 { try!(write!(f, " p6"))}
      if self.p7() != 0 { try!(write!(f, " p7"))}
      if self.p8() != 0 { try!(write!(f, " p8"))}
      if self.p9() != 0 { try!(write!(f, " p9"))}
      if self.p10() != 0 { try!(write!(f, " p10"))}
      if self.p11() != 0 { try!(write!(f, " p11"))}
      if self.p12() != 0 { try!(write!(f, " p12"))}
      if self.p13() != 0 { try!(write!(f, " p13"))}
      if self.p14() != 0 { try!(write!(f, " p14"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Micro Direct Memory Access Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppdma(pub u32);
impl Ppdma {
#[doc="uDMA Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="uDMA Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EPI Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppepi(pub u32);
impl Ppepi {
#[doc="EPI Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EPI Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Hibernation Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pphib(pub u32);
impl Pphib {
#[doc="Hibernation Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Hibernation Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pphib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pphib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Asynchronous Receiver/Transmitter Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppuart(pub u32);
impl Ppuart {
#[doc="UART Module 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Module 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Module 1 Present"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Module 1 Present"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Module 2 Present"]
   #[inline] pub fn p2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Module 2 Present"]
   #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Module 3 Present"]
   #[inline] pub fn p3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Module 3 Present"]
   #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Module 4 Present"]
   #[inline] pub fn p4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Module 4 Present"]
   #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Module 5 Present"]
   #[inline] pub fn p5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Module 5 Present"]
   #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Module 6 Present"]
   #[inline] pub fn p6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Module 6 Present"]
   #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Module 7 Present"]
   #[inline] pub fn p7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Module 7 Present"]
   #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Ppuart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppuart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      if self.p2() != 0 { try!(write!(f, " p2"))}
      if self.p3() != 0 { try!(write!(f, " p3"))}
      if self.p4() != 0 { try!(write!(f, " p4"))}
      if self.p5() != 0 { try!(write!(f, " p5"))}
      if self.p6() != 0 { try!(write!(f, " p6"))}
      if self.p7() != 0 { try!(write!(f, " p7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Synchronous Serial Interface Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppssi(pub u32);
impl Ppssi {
#[doc="SSI Module 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="SSI Module 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="SSI Module 1 Present"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="SSI Module 1 Present"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="SSI Module 2 Present"]
   #[inline] pub fn p2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="SSI Module 2 Present"]
   #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="SSI Module 3 Present"]
   #[inline] pub fn p3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="SSI Module 3 Present"]
   #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Ppssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      if self.p2() != 0 { try!(write!(f, " p2"))}
      if self.p3() != 0 { try!(write!(f, " p3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Inter-Integrated Circuit Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppi2c(pub u32);
impl Ppi2c {
#[doc="I2C Module 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="I2C Module 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="I2C Module 1 Present"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="I2C Module 1 Present"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="I2C Module 2 Present"]
   #[inline] pub fn p2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="I2C Module 2 Present"]
   #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="I2C Module 3 Present"]
   #[inline] pub fn p3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="I2C Module 3 Present"]
   #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="I2C Module 4 Present"]
   #[inline] pub fn p4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="I2C Module 4 Present"]
   #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="I2C Module 5 Present"]
   #[inline] pub fn p5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="I2C Module 5 Present"]
   #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="I2C Module 6 Present"]
   #[inline] pub fn p6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="I2C Module 6 Present"]
   #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="I2C Module 7 Present"]
   #[inline] pub fn p7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="I2C Module 7 Present"]
   #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="I2C Module 8 Present"]
   #[inline] pub fn p8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="I2C Module 8 Present"]
   #[inline] pub fn set_p8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="I2C Module 9 Present"]
   #[inline] pub fn p9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="I2C Module 9 Present"]
   #[inline] pub fn set_p9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}
impl ::core::fmt::Display for Ppi2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppi2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      if self.p2() != 0 { try!(write!(f, " p2"))}
      if self.p3() != 0 { try!(write!(f, " p3"))}
      if self.p4() != 0 { try!(write!(f, " p4"))}
      if self.p5() != 0 { try!(write!(f, " p5"))}
      if self.p6() != 0 { try!(write!(f, " p6"))}
      if self.p7() != 0 { try!(write!(f, " p7"))}
      if self.p8() != 0 { try!(write!(f, " p8"))}
      if self.p9() != 0 { try!(write!(f, " p9"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Serial Bus Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppusb(pub u32);
impl Ppusb {
#[doc="USB Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="USB Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet PHY Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppephy(pub u32);
impl Ppephy {
#[doc="Ethernet PHY Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet PHY Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Controller Area Network Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppcan(pub u32);
impl Ppcan {
#[doc="CAN Module 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CAN Module 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="CAN Module 1 Present"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="CAN Module 1 Present"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Ppcan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppcan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog-to-Digital Converter Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppadc(pub u32);
impl Ppadc {
#[doc="ADC Module 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ADC Module 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="ADC Module 1 Present"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="ADC Module 1 Present"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Ppadc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppadc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog Comparator Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppacmp(pub u32);
impl Ppacmp {
#[doc="Analog Comparator Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Analog Comparator Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppacmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppacmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pulse Width Modulator Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pppwm(pub u32);
impl Pppwm {
#[doc="PWM Module 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PWM Module 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pppwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pppwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Quadrature Encoder Interface Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppqei(pub u32);
impl Ppqei {
#[doc="QEI Module 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="QEI Module 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Low Pin Count Interface Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pplpc(pub u32);
impl Pplpc {
#[doc="LPC Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="LPC Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pplpc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pplpc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Platform Environment Control Interface Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pppeci(pub u32);
impl Pppeci {
#[doc="PECI Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PECI Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pppeci {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pppeci {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Fan Control Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppfan(pub u32);
impl Ppfan {
#[doc="FAN Module 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="FAN Module 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppfan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppfan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EEPROM Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppeeprom(pub u32);
impl Ppeeprom {
#[doc="EEPROM Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EEPROM Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppeeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppeeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="32/64-Bit Wide General-Purpose Timer Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppwtimer(pub u32);
impl Ppwtimer {
#[doc="32/64-Bit Wide General-Purpose Timer 0 Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="32/64-Bit Wide General-Purpose Timer 0 Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppwtimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppwtimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Remote Temperature Sensor Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pprts(pub u32);
impl Pprts {
#[doc="RTS Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="RTS Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pprts {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pprts {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC and Cryptographic Modules Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppccm(pub u32);
impl Ppccm {
#[doc="CRC and Cryptographic Modules Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CRC and Cryptographic Modules Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="LCD Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pplcd(pub u32);
impl Pplcd {
#[doc="LCD Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="LCD Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pplcd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pplcd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="1-Wire Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppowire(pub u32);
impl Ppowire {
#[doc="1-Wire Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="1-Wire Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppowire {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppowire {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ppemac(pub u32);
impl Ppemac {
#[doc="Ethernet Controller Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet Controller Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ppemac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ppemac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Human Interface Master Peripheral Present"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pphim(pub u32);
impl Pphim {
#[doc="HIM Module Present"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="HIM Module Present"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pphim {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pphim {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srwd(pub u32);
impl Srwd {
#[doc="Watchdog Timer 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Watchdog Timer 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Watchdog Timer 1 Software Reset"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Watchdog Timer 1 Software Reset"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Srwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="16/32-Bit General-Purpose Timer Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srtimer(pub u32);
impl Srtimer {
#[doc="16/32-Bit General-Purpose Timer 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="16/32-Bit General-Purpose Timer 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 1 Software Reset"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="16/32-Bit General-Purpose Timer 1 Software Reset"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 2 Software Reset"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="16/32-Bit General-Purpose Timer 2 Software Reset"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 3 Software Reset"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="16/32-Bit General-Purpose Timer 3 Software Reset"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 4 Software Reset"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="16/32-Bit General-Purpose Timer 4 Software Reset"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 5 Software Reset"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="16/32-Bit General-Purpose Timer 5 Software Reset"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 6 Software Reset"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="16/32-Bit General-Purpose Timer 6 Software Reset"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 7 Software Reset"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="16/32-Bit General-Purpose Timer 7 Software Reset"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Srtimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srtimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="General-Purpose Input/Output Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srgpio(pub u32);
impl Srgpio {
#[doc="GPIO Port A Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Port A Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="GPIO Port B Software Reset"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="GPIO Port B Software Reset"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="GPIO Port C Software Reset"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="GPIO Port C Software Reset"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="GPIO Port D Software Reset"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="GPIO Port D Software Reset"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="GPIO Port E Software Reset"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="GPIO Port E Software Reset"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="GPIO Port F Software Reset"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="GPIO Port F Software Reset"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="GPIO Port G Software Reset"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="GPIO Port G Software Reset"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="GPIO Port H Software Reset"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="GPIO Port H Software Reset"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="GPIO Port J Software Reset"]
   #[inline] pub fn r8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO Port J Software Reset"]
   #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="GPIO Port K Software Reset"]
   #[inline] pub fn r9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="GPIO Port K Software Reset"]
   #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="GPIO Port L Software Reset"]
   #[inline] pub fn r10(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="GPIO Port L Software Reset"]
   #[inline] pub fn set_r10<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="GPIO Port M Software Reset"]
   #[inline] pub fn r11(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="GPIO Port M Software Reset"]
   #[inline] pub fn set_r11<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="GPIO Port N Software Reset"]
   #[inline] pub fn r12(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="GPIO Port N Software Reset"]
   #[inline] pub fn set_r12<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="GPIO Port P Software Reset"]
   #[inline] pub fn r13(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="GPIO Port P Software Reset"]
   #[inline] pub fn set_r13<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="GPIO Port Q Software Reset"]
   #[inline] pub fn r14(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="GPIO Port Q Software Reset"]
   #[inline] pub fn set_r14<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

}
impl ::core::fmt::Display for Srgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      if self.r8() != 0 { try!(write!(f, " r8"))}
      if self.r9() != 0 { try!(write!(f, " r9"))}
      if self.r10() != 0 { try!(write!(f, " r10"))}
      if self.r11() != 0 { try!(write!(f, " r11"))}
      if self.r12() != 0 { try!(write!(f, " r12"))}
      if self.r13() != 0 { try!(write!(f, " r13"))}
      if self.r14() != 0 { try!(write!(f, " r14"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Micro Direct Memory Access Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srdma(pub u32);
impl Srdma {
#[doc="uDMA Module Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="uDMA Module Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Srdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EPI Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srepi(pub u32);
impl Srepi {
#[doc="EPI Module Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EPI Module Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Srepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Hibernation Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srhib(pub u32);
impl Srhib {
#[doc="Hibernation Module Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Hibernation Module Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Srhib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srhib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Asynchronous Receiver/Transmitter Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sruart(pub u32);
impl Sruart {
#[doc="UART Module 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Module 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Module 1 Software Reset"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Module 1 Software Reset"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Module 2 Software Reset"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Module 2 Software Reset"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Module 3 Software Reset"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Module 3 Software Reset"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Module 4 Software Reset"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Module 4 Software Reset"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Module 5 Software Reset"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Module 5 Software Reset"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Module 6 Software Reset"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Module 6 Software Reset"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Module 7 Software Reset"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Module 7 Software Reset"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Sruart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sruart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Synchronous Serial Interface Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srssi(pub u32);
impl Srssi {
#[doc="SSI Module 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="SSI Module 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="SSI Module 1 Software Reset"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="SSI Module 1 Software Reset"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="SSI Module 2 Software Reset"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="SSI Module 2 Software Reset"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="SSI Module 3 Software Reset"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="SSI Module 3 Software Reset"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Srssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Inter-Integrated Circuit Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sri2c(pub u32);
impl Sri2c {
#[doc="I2C Module 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="I2C Module 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="I2C Module 1 Software Reset"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="I2C Module 1 Software Reset"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="I2C Module 2 Software Reset"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="I2C Module 2 Software Reset"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="I2C Module 3 Software Reset"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="I2C Module 3 Software Reset"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="I2C Module 4 Software Reset"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="I2C Module 4 Software Reset"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="I2C Module 5 Software Reset"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="I2C Module 5 Software Reset"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="I2C Module 6 Software Reset"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="I2C Module 6 Software Reset"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="I2C Module 7 Software Reset"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="I2C Module 7 Software Reset"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="I2C Module 8 Software Reset"]
   #[inline] pub fn r8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="I2C Module 8 Software Reset"]
   #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="I2C Module 9 Software Reset"]
   #[inline] pub fn r9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="I2C Module 9 Software Reset"]
   #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}
impl ::core::fmt::Display for Sri2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sri2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      if self.r8() != 0 { try!(write!(f, " r8"))}
      if self.r9() != 0 { try!(write!(f, " r9"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Serial Bus Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srusb(pub u32);
impl Srusb {
#[doc="USB Module Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="USB Module Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Srusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet PHY Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srephy(pub u32);
impl Srephy {
#[doc="Ethernet PHY Module Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet PHY Module Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Srephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Controller Area Network Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srcan(pub u32);
impl Srcan {
#[doc="CAN Module 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CAN Module 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="CAN Module 1 Software Reset"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="CAN Module 1 Software Reset"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Srcan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srcan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog-to-Digital Converter Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sradc(pub u32);
impl Sradc {
#[doc="ADC Module 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ADC Module 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="ADC Module 1 Software Reset"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="ADC Module 1 Software Reset"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Sradc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sradc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog Comparator Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sracmp(pub u32);
impl Sracmp {
#[doc="Analog Comparator Module 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Analog Comparator Module 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Sracmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sracmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pulse Width Modulator Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srpwm(pub u32);
impl Srpwm {
#[doc="PWM Module 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PWM Module 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Srpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Quadrature Encoder Interface Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srqei(pub u32);
impl Srqei {
#[doc="QEI Module 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="QEI Module 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Srqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EEPROM Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sreeprom(pub u32);
impl Sreeprom {
#[doc="EEPROM Module Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EEPROM Module Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Sreeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sreeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC and Cryptographic Modules Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Srccm(pub u32);
impl Srccm {
#[doc="CRC and Cryptographic Modules Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CRC and Cryptographic Modules Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Srccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC Software Reset"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sremac(pub u32);
impl Sremac {
#[doc="Ethernet Controller MAC Module 0 Software Reset"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet Controller MAC Module 0 Software Reset"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Sremac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sremac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcwd(pub u32);
impl Rcgcwd {
#[doc="Watchdog Timer 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Watchdog Timer 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Watchdog Timer 1 Run Mode Clock Gating Control"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Watchdog Timer 1 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Rcgcwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="16/32-Bit General-Purpose Timer Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgctimer(pub u32);
impl Rcgctimer {
#[doc="16/32-Bit General-Purpose Timer 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="16/32-Bit General-Purpose Timer 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 1 Run Mode Clock Gating Control"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="16/32-Bit General-Purpose Timer 1 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 2 Run Mode Clock Gating Control"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="16/32-Bit General-Purpose Timer 2 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 3 Run Mode Clock Gating Control"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="16/32-Bit General-Purpose Timer 3 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 4 Run Mode Clock Gating Control"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="16/32-Bit General-Purpose Timer 4 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 5 Run Mode Clock Gating Control"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="16/32-Bit General-Purpose Timer 5 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 6 Run Mode Clock Gating Control"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="16/32-Bit General-Purpose Timer 6 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 7 Run Mode Clock Gating Control"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="16/32-Bit General-Purpose Timer 7 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Rcgctimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgctimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="General-Purpose Input/Output Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcgpio(pub u32);
impl Rcgcgpio {
#[doc="GPIO Port A Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Port A Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="GPIO Port B Run Mode Clock Gating Control"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="GPIO Port B Run Mode Clock Gating Control"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="GPIO Port C Run Mode Clock Gating Control"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="GPIO Port C Run Mode Clock Gating Control"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="GPIO Port D Run Mode Clock Gating Control"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="GPIO Port D Run Mode Clock Gating Control"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="GPIO Port E Run Mode Clock Gating Control"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="GPIO Port E Run Mode Clock Gating Control"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="GPIO Port F Run Mode Clock Gating Control"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="GPIO Port F Run Mode Clock Gating Control"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="GPIO Port G Run Mode Clock Gating Control"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="GPIO Port G Run Mode Clock Gating Control"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="GPIO Port H Run Mode Clock Gating Control"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="GPIO Port H Run Mode Clock Gating Control"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="GPIO Port J Run Mode Clock Gating Control"]
   #[inline] pub fn r8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO Port J Run Mode Clock Gating Control"]
   #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="GPIO Port K Run Mode Clock Gating Control"]
   #[inline] pub fn r9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="GPIO Port K Run Mode Clock Gating Control"]
   #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="GPIO Port L Run Mode Clock Gating Control"]
   #[inline] pub fn r10(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="GPIO Port L Run Mode Clock Gating Control"]
   #[inline] pub fn set_r10<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="GPIO Port M Run Mode Clock Gating Control"]
   #[inline] pub fn r11(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="GPIO Port M Run Mode Clock Gating Control"]
   #[inline] pub fn set_r11<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="GPIO Port N Run Mode Clock Gating Control"]
   #[inline] pub fn r12(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="GPIO Port N Run Mode Clock Gating Control"]
   #[inline] pub fn set_r12<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="GPIO Port P Run Mode Clock Gating Control"]
   #[inline] pub fn r13(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="GPIO Port P Run Mode Clock Gating Control"]
   #[inline] pub fn set_r13<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="GPIO Port Q Run Mode Clock Gating Control"]
   #[inline] pub fn r14(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="GPIO Port Q Run Mode Clock Gating Control"]
   #[inline] pub fn set_r14<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

}
impl ::core::fmt::Display for Rcgcgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      if self.r8() != 0 { try!(write!(f, " r8"))}
      if self.r9() != 0 { try!(write!(f, " r9"))}
      if self.r10() != 0 { try!(write!(f, " r10"))}
      if self.r11() != 0 { try!(write!(f, " r11"))}
      if self.r12() != 0 { try!(write!(f, " r12"))}
      if self.r13() != 0 { try!(write!(f, " r13"))}
      if self.r14() != 0 { try!(write!(f, " r14"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Micro Direct Memory Access Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcdma(pub u32);
impl Rcgcdma {
#[doc="uDMA Module Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="uDMA Module Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgcdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EPI Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcepi(pub u32);
impl Rcgcepi {
#[doc="EPI Module Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EPI Module Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgcepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Hibernation Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgchib(pub u32);
impl Rcgchib {
#[doc="Hibernation Module Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Hibernation Module Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgchib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgchib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcuart(pub u32);
impl Rcgcuart {
#[doc="UART Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Module 1 Run Mode Clock Gating Control"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Module 1 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Module 2 Run Mode Clock Gating Control"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Module 2 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Module 3 Run Mode Clock Gating Control"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Module 3 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Module 4 Run Mode Clock Gating Control"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Module 4 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Module 5 Run Mode Clock Gating Control"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Module 5 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Module 6 Run Mode Clock Gating Control"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Module 6 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Module 7 Run Mode Clock Gating Control"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Module 7 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Rcgcuart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcuart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Synchronous Serial Interface Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcssi(pub u32);
impl Rcgcssi {
#[doc="SSI Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="SSI Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="SSI Module 1 Run Mode Clock Gating Control"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="SSI Module 1 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="SSI Module 2 Run Mode Clock Gating Control"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="SSI Module 2 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="SSI Module 3 Run Mode Clock Gating Control"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="SSI Module 3 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Rcgcssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Inter-Integrated Circuit Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgci2c(pub u32);
impl Rcgci2c {
#[doc="I2C Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="I2C Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="I2C Module 1 Run Mode Clock Gating Control"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="I2C Module 1 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="I2C Module 2 Run Mode Clock Gating Control"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="I2C Module 2 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="I2C Module 3 Run Mode Clock Gating Control"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="I2C Module 3 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="I2C Module 4 Run Mode Clock Gating Control"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="I2C Module 4 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="I2C Module 5 Run Mode Clock Gating Control"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="I2C Module 5 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="I2C Module 6 Run Mode Clock Gating Control"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="I2C Module 6 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="I2C Module 7 Run Mode Clock Gating Control"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="I2C Module 7 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="I2C Module 8 Run Mode Clock Gating Control"]
   #[inline] pub fn r8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="I2C Module 8 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="I2C Module 9 Run Mode Clock Gating Control"]
   #[inline] pub fn r9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="I2C Module 9 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}
impl ::core::fmt::Display for Rcgci2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgci2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      if self.r8() != 0 { try!(write!(f, " r8"))}
      if self.r9() != 0 { try!(write!(f, " r9"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Serial Bus Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcusb(pub u32);
impl Rcgcusb {
#[doc="USB Module Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="USB Module Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgcusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet PHY Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcephy(pub u32);
impl Rcgcephy {
#[doc="Ethernet PHY Module Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet PHY Module Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgcephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Controller Area Network Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgccan(pub u32);
impl Rcgccan {
#[doc="CAN Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CAN Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="CAN Module 1 Run Mode Clock Gating Control"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="CAN Module 1 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Rcgccan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgccan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog-to-Digital Converter Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcadc(pub u32);
impl Rcgcadc {
#[doc="ADC Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ADC Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="ADC Module 1 Run Mode Clock Gating Control"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="ADC Module 1 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Rcgcadc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcadc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog Comparator Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcacmp(pub u32);
impl Rcgcacmp {
#[doc="Analog Comparator Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Analog Comparator Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgcacmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcacmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pulse Width Modulator Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcpwm(pub u32);
impl Rcgcpwm {
#[doc="PWM Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PWM Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgcpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Quadrature Encoder Interface Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcqei(pub u32);
impl Rcgcqei {
#[doc="QEI Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="QEI Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgcqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EEPROM Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgceeprom(pub u32);
impl Rcgceeprom {
#[doc="EEPROM Module Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EEPROM Module Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgceeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgceeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC and Cryptographic Modules Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcccm(pub u32);
impl Rcgcccm {
#[doc="CRC and Cryptographic Modules Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CRC and Cryptographic Modules Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgcccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC Run Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rcgcemac(pub u32);
impl Rcgcemac {
#[doc="Ethernet MAC Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet MAC Module 0 Run Mode Clock Gating Control"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Rcgcemac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcgcemac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcwd(pub u32);
impl Scgcwd {
#[doc="Watchdog Timer 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Watchdog Timer 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Watchdog Timer 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Watchdog Timer 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Scgcwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      if self.s1() != 0 { try!(write!(f, " s1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgctimer(pub u32);
impl Scgctimer {
#[doc="16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 6 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="16/32-Bit General-Purpose Timer 6 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 7 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="16/32-Bit General-Purpose Timer 7 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Scgctimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgctimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      if self.s1() != 0 { try!(write!(f, " s1"))}
      if self.s2() != 0 { try!(write!(f, " s2"))}
      if self.s3() != 0 { try!(write!(f, " s3"))}
      if self.s4() != 0 { try!(write!(f, " s4"))}
      if self.s5() != 0 { try!(write!(f, " s5"))}
      if self.s6() != 0 { try!(write!(f, " s6"))}
      if self.s7() != 0 { try!(write!(f, " s7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="General-Purpose Input/Output Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcgpio(pub u32);
impl Scgcgpio {
#[doc="GPIO Port A Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Port A Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="GPIO Port B Sleep Mode Clock Gating Control"]
   #[inline] pub fn s1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="GPIO Port B Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="GPIO Port C Sleep Mode Clock Gating Control"]
   #[inline] pub fn s2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="GPIO Port C Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="GPIO Port D Sleep Mode Clock Gating Control"]
   #[inline] pub fn s3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="GPIO Port D Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="GPIO Port E Sleep Mode Clock Gating Control"]
   #[inline] pub fn s4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="GPIO Port E Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="GPIO Port F Sleep Mode Clock Gating Control"]
   #[inline] pub fn s5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="GPIO Port F Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="GPIO Port G Sleep Mode Clock Gating Control"]
   #[inline] pub fn s6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="GPIO Port G Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="GPIO Port H Sleep Mode Clock Gating Control"]
   #[inline] pub fn s7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="GPIO Port H Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="GPIO Port J Sleep Mode Clock Gating Control"]
   #[inline] pub fn s8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO Port J Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="GPIO Port K Sleep Mode Clock Gating Control"]
   #[inline] pub fn s9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="GPIO Port K Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="GPIO Port L Sleep Mode Clock Gating Control"]
   #[inline] pub fn s10(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="GPIO Port L Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s10<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="GPIO Port M Sleep Mode Clock Gating Control"]
   #[inline] pub fn s11(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="GPIO Port M Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s11<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="GPIO Port N Sleep Mode Clock Gating Control"]
   #[inline] pub fn s12(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="GPIO Port N Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s12<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="GPIO Port P Sleep Mode Clock Gating Control"]
   #[inline] pub fn s13(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="GPIO Port P Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s13<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="GPIO Port Q Sleep Mode Clock Gating Control"]
   #[inline] pub fn s14(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="GPIO Port Q Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s14<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

}
impl ::core::fmt::Display for Scgcgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      if self.s1() != 0 { try!(write!(f, " s1"))}
      if self.s2() != 0 { try!(write!(f, " s2"))}
      if self.s3() != 0 { try!(write!(f, " s3"))}
      if self.s4() != 0 { try!(write!(f, " s4"))}
      if self.s5() != 0 { try!(write!(f, " s5"))}
      if self.s6() != 0 { try!(write!(f, " s6"))}
      if self.s7() != 0 { try!(write!(f, " s7"))}
      if self.s8() != 0 { try!(write!(f, " s8"))}
      if self.s9() != 0 { try!(write!(f, " s9"))}
      if self.s10() != 0 { try!(write!(f, " s10"))}
      if self.s11() != 0 { try!(write!(f, " s11"))}
      if self.s12() != 0 { try!(write!(f, " s12"))}
      if self.s13() != 0 { try!(write!(f, " s13"))}
      if self.s14() != 0 { try!(write!(f, " s14"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Micro Direct Memory Access Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcdma(pub u32);
impl Scgcdma {
#[doc="uDMA Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="uDMA Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgcdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EPI Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcepi(pub u32);
impl Scgcepi {
#[doc="EPI Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EPI Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgcepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Hibernation Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgchib(pub u32);
impl Scgchib {
#[doc="Hibernation Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Hibernation Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgchib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgchib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcuart(pub u32);
impl Scgcuart {
#[doc="UART Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Module 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Module 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Module 2 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Module 2 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Module 3 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Module 3 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Module 4 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Module 4 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Module 5 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Module 5 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Module 6 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Module 6 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Module 7 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Module 7 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Scgcuart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcuart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      if self.s1() != 0 { try!(write!(f, " s1"))}
      if self.s2() != 0 { try!(write!(f, " s2"))}
      if self.s3() != 0 { try!(write!(f, " s3"))}
      if self.s4() != 0 { try!(write!(f, " s4"))}
      if self.s5() != 0 { try!(write!(f, " s5"))}
      if self.s6() != 0 { try!(write!(f, " s6"))}
      if self.s7() != 0 { try!(write!(f, " s7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Synchronous Serial Interface Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcssi(pub u32);
impl Scgcssi {
#[doc="SSI Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="SSI Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="SSI Module 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="SSI Module 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="SSI Module 2 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="SSI Module 2 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="SSI Module 3 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="SSI Module 3 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Scgcssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      if self.s1() != 0 { try!(write!(f, " s1"))}
      if self.s2() != 0 { try!(write!(f, " s2"))}
      if self.s3() != 0 { try!(write!(f, " s3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgci2c(pub u32);
impl Scgci2c {
#[doc="I2C Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="I2C Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="I2C Module 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="I2C Module 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="I2C Module 2 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="I2C Module 2 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="I2C Module 3 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="I2C Module 3 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="I2C Module 4 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="I2C Module 4 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="I2C Module 5 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="I2C Module 5 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="I2C Module 6 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="I2C Module 6 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="I2C Module 7 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="I2C Module 7 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="I2C Module 8 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="I2C Module 8 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="I2C Module 9 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="I2C Module 9 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}
impl ::core::fmt::Display for Scgci2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgci2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      if self.s1() != 0 { try!(write!(f, " s1"))}
      if self.s2() != 0 { try!(write!(f, " s2"))}
      if self.s3() != 0 { try!(write!(f, " s3"))}
      if self.s4() != 0 { try!(write!(f, " s4"))}
      if self.s5() != 0 { try!(write!(f, " s5"))}
      if self.s6() != 0 { try!(write!(f, " s6"))}
      if self.s7() != 0 { try!(write!(f, " s7"))}
      if self.s8() != 0 { try!(write!(f, " s8"))}
      if self.s9() != 0 { try!(write!(f, " s9"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Serial Bus Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcusb(pub u32);
impl Scgcusb {
#[doc="USB Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="USB Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgcusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet PHY Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcephy(pub u32);
impl Scgcephy {
#[doc="PHY Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PHY Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgcephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Controller Area Network Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgccan(pub u32);
impl Scgccan {
#[doc="CAN Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CAN Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="CAN Module 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="CAN Module 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Scgccan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgccan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      if self.s1() != 0 { try!(write!(f, " s1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog-to-Digital Converter Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcadc(pub u32);
impl Scgcadc {
#[doc="ADC Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ADC Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="ADC Module 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="ADC Module 1 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Scgcadc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcadc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      if self.s1() != 0 { try!(write!(f, " s1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog Comparator Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcacmp(pub u32);
impl Scgcacmp {
#[doc="Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgcacmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcacmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pulse Width Modulator Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcpwm(pub u32);
impl Scgcpwm {
#[doc="PWM Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PWM Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgcpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Quadrature Encoder Interface Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcqei(pub u32);
impl Scgcqei {
#[doc="QEI Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="QEI Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgcqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EEPROM Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgceeprom(pub u32);
impl Scgceeprom {
#[doc="EEPROM Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EEPROM Module Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgceeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgceeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcccm(pub u32);
impl Scgcccm {
#[doc="CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgcccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scgcemac(pub u32);
impl Scgcemac {
#[doc="Ethernet MAC Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn s0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet MAC Module 0 Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_s0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Scgcemac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scgcemac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.s0() != 0 { try!(write!(f, " s0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcwd(pub u32);
impl Dcgcwd {
#[doc="Watchdog Timer 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Watchdog Timer 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Watchdog Timer 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Watchdog Timer 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Dcgcwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      if self.d1() != 0 { try!(write!(f, " d1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgctimer(pub u32);
impl Dcgctimer {
#[doc="16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Dcgctimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgctimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      if self.d1() != 0 { try!(write!(f, " d1"))}
      if self.d2() != 0 { try!(write!(f, " d2"))}
      if self.d3() != 0 { try!(write!(f, " d3"))}
      if self.d4() != 0 { try!(write!(f, " d4"))}
      if self.d5() != 0 { try!(write!(f, " d5"))}
      if self.d6() != 0 { try!(write!(f, " d6"))}
      if self.d7() != 0 { try!(write!(f, " d7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcgpio(pub u32);
impl Dcgcgpio {
#[doc="GPIO Port A Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Port A Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="GPIO Port B Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="GPIO Port B Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="GPIO Port C Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="GPIO Port C Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="GPIO Port D Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="GPIO Port D Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="GPIO Port E Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="GPIO Port E Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="GPIO Port F Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="GPIO Port F Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="GPIO Port G Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="GPIO Port G Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="GPIO Port H Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="GPIO Port H Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="GPIO Port J Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO Port J Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="GPIO Port K Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="GPIO Port K Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="GPIO Port L Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d10(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="GPIO Port L Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d10<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="GPIO Port M Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d11(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="GPIO Port M Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d11<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="GPIO Port N Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d12(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="GPIO Port N Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d12<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="GPIO Port P Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d13(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="GPIO Port P Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d13<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d14(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d14<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

}
impl ::core::fmt::Display for Dcgcgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      if self.d1() != 0 { try!(write!(f, " d1"))}
      if self.d2() != 0 { try!(write!(f, " d2"))}
      if self.d3() != 0 { try!(write!(f, " d3"))}
      if self.d4() != 0 { try!(write!(f, " d4"))}
      if self.d5() != 0 { try!(write!(f, " d5"))}
      if self.d6() != 0 { try!(write!(f, " d6"))}
      if self.d7() != 0 { try!(write!(f, " d7"))}
      if self.d8() != 0 { try!(write!(f, " d8"))}
      if self.d9() != 0 { try!(write!(f, " d9"))}
      if self.d10() != 0 { try!(write!(f, " d10"))}
      if self.d11() != 0 { try!(write!(f, " d11"))}
      if self.d12() != 0 { try!(write!(f, " d12"))}
      if self.d13() != 0 { try!(write!(f, " d13"))}
      if self.d14() != 0 { try!(write!(f, " d14"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcdma(pub u32);
impl Dcgcdma {
#[doc="uDMA Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="uDMA Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgcdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EPI Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcepi(pub u32);
impl Dcgcepi {
#[doc="EPI Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EPI Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgcepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Hibernation Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgchib(pub u32);
impl Dcgchib {
#[doc="Hibernation Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Hibernation Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgchib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgchib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcuart(pub u32);
impl Dcgcuart {
#[doc="UART Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Module 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Module 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Module 2 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Module 2 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Module 3 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Module 3 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Module 4 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Module 4 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Module 5 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Module 5 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Module 6 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Module 6 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Module 7 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Module 7 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Dcgcuart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcuart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      if self.d1() != 0 { try!(write!(f, " d1"))}
      if self.d2() != 0 { try!(write!(f, " d2"))}
      if self.d3() != 0 { try!(write!(f, " d3"))}
      if self.d4() != 0 { try!(write!(f, " d4"))}
      if self.d5() != 0 { try!(write!(f, " d5"))}
      if self.d6() != 0 { try!(write!(f, " d6"))}
      if self.d7() != 0 { try!(write!(f, " d7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcssi(pub u32);
impl Dcgcssi {
#[doc="SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Dcgcssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      if self.d1() != 0 { try!(write!(f, " d1"))}
      if self.d2() != 0 { try!(write!(f, " d2"))}
      if self.d3() != 0 { try!(write!(f, " d3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgci2c(pub u32);
impl Dcgci2c {
#[doc="I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="I2C Module 4 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="I2C Module 4 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="I2C Module 5 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="I2C Module 5 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="I2C Module 6 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="I2C Module 6 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="I2C Module 7 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="I2C Module 7 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="I2C Module 8 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="I2C Module 8 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="I2C Module 9 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="I2C Module 9 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}
impl ::core::fmt::Display for Dcgci2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgci2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      if self.d1() != 0 { try!(write!(f, " d1"))}
      if self.d2() != 0 { try!(write!(f, " d2"))}
      if self.d3() != 0 { try!(write!(f, " d3"))}
      if self.d4() != 0 { try!(write!(f, " d4"))}
      if self.d5() != 0 { try!(write!(f, " d5"))}
      if self.d6() != 0 { try!(write!(f, " d6"))}
      if self.d7() != 0 { try!(write!(f, " d7"))}
      if self.d8() != 0 { try!(write!(f, " d8"))}
      if self.d9() != 0 { try!(write!(f, " d9"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcusb(pub u32);
impl Dcgcusb {
#[doc="USB Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="USB Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgcusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet PHY Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcephy(pub u32);
impl Dcgcephy {
#[doc="PHY Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PHY Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgcephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Controller Area Network Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgccan(pub u32);
impl Dcgccan {
#[doc="CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CAN Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="CAN Module 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Dcgccan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgccan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      if self.d1() != 0 { try!(write!(f, " d1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcadc(pub u32);
impl Dcgcadc {
#[doc="ADC Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ADC Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="ADC Module 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="ADC Module 1 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Dcgcadc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcadc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      if self.d1() != 0 { try!(write!(f, " d1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog Comparator Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcacmp(pub u32);
impl Dcgcacmp {
#[doc="Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgcacmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcacmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pulse Width Modulator Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcpwm(pub u32);
impl Dcgcpwm {
#[doc="PWM Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PWM Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgcpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcqei(pub u32);
impl Dcgcqei {
#[doc="QEI Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="QEI Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgcqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EEPROM Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgceeprom(pub u32);
impl Dcgceeprom {
#[doc="EEPROM Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EEPROM Module Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgceeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgceeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcccm(pub u32);
impl Dcgcccm {
#[doc="CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgcccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC Deep-Sleep Mode Clock Gating Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dcgcemac(pub u32);
impl Dcgcemac {
#[doc="Ethernet MAC Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn d0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet MAC Module 0 Deep-Sleep Mode Clock Gating Control"]
   #[inline] pub fn set_d0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Dcgcemac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcgcemac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.d0() != 0 { try!(write!(f, " d0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcwd(pub u32);
impl Pcwd {
#[doc="Watchdog Timer 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Watchdog Timer 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Watchdog Timer 1 Power Control"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Watchdog Timer 1 Power Control"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Pcwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="16/32-Bit General-Purpose Timer Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pctimer(pub u32);
impl Pctimer {
#[doc="General-Purpose Timer 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="General-Purpose Timer 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="General-Purpose Timer 1 Power Control"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="General-Purpose Timer 1 Power Control"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="General-Purpose Timer 2 Power Control"]
   #[inline] pub fn p2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="General-Purpose Timer 2 Power Control"]
   #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="General-Purpose Timer 3 Power Control"]
   #[inline] pub fn p3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="General-Purpose Timer 3 Power Control"]
   #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="General-Purpose Timer 4 Power Control"]
   #[inline] pub fn p4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="General-Purpose Timer 4 Power Control"]
   #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="General-Purpose Timer 5 Power Control"]
   #[inline] pub fn p5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="General-Purpose Timer 5 Power Control"]
   #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="General-Purpose Timer 6 Power Control"]
   #[inline] pub fn p6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="General-Purpose Timer 6 Power Control"]
   #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="General-Purpose Timer 7 Power Control"]
   #[inline] pub fn p7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="General-Purpose Timer 7 Power Control"]
   #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Pctimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pctimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      if self.p2() != 0 { try!(write!(f, " p2"))}
      if self.p3() != 0 { try!(write!(f, " p3"))}
      if self.p4() != 0 { try!(write!(f, " p4"))}
      if self.p5() != 0 { try!(write!(f, " p5"))}
      if self.p6() != 0 { try!(write!(f, " p6"))}
      if self.p7() != 0 { try!(write!(f, " p7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="General-Purpose Input/Output Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcgpio(pub u32);
impl Pcgpio {
#[doc="GPIO Port A Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Port A Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="GPIO Port B Power Control"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="GPIO Port B Power Control"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="GPIO Port C Power Control"]
   #[inline] pub fn p2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="GPIO Port C Power Control"]
   #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="GPIO Port D Power Control"]
   #[inline] pub fn p3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="GPIO Port D Power Control"]
   #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="GPIO Port E Power Control"]
   #[inline] pub fn p4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="GPIO Port E Power Control"]
   #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="GPIO Port F Power Control"]
   #[inline] pub fn p5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="GPIO Port F Power Control"]
   #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="GPIO Port G Power Control"]
   #[inline] pub fn p6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="GPIO Port G Power Control"]
   #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="GPIO Port H Power Control"]
   #[inline] pub fn p7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="GPIO Port H Power Control"]
   #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="GPIO Port J Power Control"]
   #[inline] pub fn p8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO Port J Power Control"]
   #[inline] pub fn set_p8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="GPIO Port K Power Control"]
   #[inline] pub fn p9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="GPIO Port K Power Control"]
   #[inline] pub fn set_p9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="GPIO Port L Power Control"]
   #[inline] pub fn p10(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="GPIO Port L Power Control"]
   #[inline] pub fn set_p10<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="GPIO Port M Power Control"]
   #[inline] pub fn p11(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="GPIO Port M Power Control"]
   #[inline] pub fn set_p11<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="GPIO Port N Power Control"]
   #[inline] pub fn p12(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="GPIO Port N Power Control"]
   #[inline] pub fn set_p12<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="GPIO Port P Power Control"]
   #[inline] pub fn p13(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="GPIO Port P Power Control"]
   #[inline] pub fn set_p13<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="GPIO Port Q Power Control"]
   #[inline] pub fn p14(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="GPIO Port Q Power Control"]
   #[inline] pub fn set_p14<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

}
impl ::core::fmt::Display for Pcgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      if self.p2() != 0 { try!(write!(f, " p2"))}
      if self.p3() != 0 { try!(write!(f, " p3"))}
      if self.p4() != 0 { try!(write!(f, " p4"))}
      if self.p5() != 0 { try!(write!(f, " p5"))}
      if self.p6() != 0 { try!(write!(f, " p6"))}
      if self.p7() != 0 { try!(write!(f, " p7"))}
      if self.p8() != 0 { try!(write!(f, " p8"))}
      if self.p9() != 0 { try!(write!(f, " p9"))}
      if self.p10() != 0 { try!(write!(f, " p10"))}
      if self.p11() != 0 { try!(write!(f, " p11"))}
      if self.p12() != 0 { try!(write!(f, " p12"))}
      if self.p13() != 0 { try!(write!(f, " p13"))}
      if self.p14() != 0 { try!(write!(f, " p14"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Micro Direct Memory Access Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcdma(pub u32);
impl Pcdma {
#[doc="uDMA Module Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="uDMA Module Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pcdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="External Peripheral Interface Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcepi(pub u32);
impl Pcepi {
#[doc="EPI Module Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EPI Module Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pcepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Hibernation Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pchib(pub u32);
impl Pchib {
#[doc="Hibernation Module Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Hibernation Module Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pchib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pchib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Asynchronous Receiver/Transmitter Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcuart(pub u32);
impl Pcuart {
#[doc="UART Module 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Module 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Module 1 Power Control"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Module 1 Power Control"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Module 2 Power Control"]
   #[inline] pub fn p2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Module 2 Power Control"]
   #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Module 3 Power Control"]
   #[inline] pub fn p3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Module 3 Power Control"]
   #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Module 4 Power Control"]
   #[inline] pub fn p4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Module 4 Power Control"]
   #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Module 5 Power Control"]
   #[inline] pub fn p5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Module 5 Power Control"]
   #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Module 6 Power Control"]
   #[inline] pub fn p6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Module 6 Power Control"]
   #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Module 7 Power Control"]
   #[inline] pub fn p7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Module 7 Power Control"]
   #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Pcuart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcuart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      if self.p2() != 0 { try!(write!(f, " p2"))}
      if self.p3() != 0 { try!(write!(f, " p3"))}
      if self.p4() != 0 { try!(write!(f, " p4"))}
      if self.p5() != 0 { try!(write!(f, " p5"))}
      if self.p6() != 0 { try!(write!(f, " p6"))}
      if self.p7() != 0 { try!(write!(f, " p7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Synchronous Serial Interface Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcssi(pub u32);
impl Pcssi {
#[doc="SSI Module 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="SSI Module 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="SSI Module 1 Power Control"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="SSI Module 1 Power Control"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="SSI Module 2 Power Control"]
   #[inline] pub fn p2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="SSI Module 2 Power Control"]
   #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="SSI Module 3 Power Control"]
   #[inline] pub fn p3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="SSI Module 3 Power Control"]
   #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Pcssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      if self.p2() != 0 { try!(write!(f, " p2"))}
      if self.p3() != 0 { try!(write!(f, " p3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Inter-Integrated Circuit Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pci2c(pub u32);
impl Pci2c {
#[doc="I2C Module 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="I2C Module 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="I2C Module 1 Power Control"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="I2C Module 1 Power Control"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="I2C Module 2 Power Control"]
   #[inline] pub fn p2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="I2C Module 2 Power Control"]
   #[inline] pub fn set_p2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="I2C Module 3 Power Control"]
   #[inline] pub fn p3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="I2C Module 3 Power Control"]
   #[inline] pub fn set_p3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="I2C Module 4 Power Control"]
   #[inline] pub fn p4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="I2C Module 4 Power Control"]
   #[inline] pub fn set_p4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="I2C Module 5 Power Control"]
   #[inline] pub fn p5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="I2C Module 5 Power Control"]
   #[inline] pub fn set_p5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="I2C Module 6 Power Control"]
   #[inline] pub fn p6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="I2C Module 6 Power Control"]
   #[inline] pub fn set_p6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="I2C Module 7 Power Control"]
   #[inline] pub fn p7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="I2C Module 7 Power Control"]
   #[inline] pub fn set_p7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="I2C Module 8 Power Control"]
   #[inline] pub fn p8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="I2C Module 8 Power Control"]
   #[inline] pub fn set_p8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="I2C Module 9 Power Control"]
   #[inline] pub fn p9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="I2C Module 9 Power Control"]
   #[inline] pub fn set_p9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}
impl ::core::fmt::Display for Pci2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pci2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      if self.p2() != 0 { try!(write!(f, " p2"))}
      if self.p3() != 0 { try!(write!(f, " p3"))}
      if self.p4() != 0 { try!(write!(f, " p4"))}
      if self.p5() != 0 { try!(write!(f, " p5"))}
      if self.p6() != 0 { try!(write!(f, " p6"))}
      if self.p7() != 0 { try!(write!(f, " p7"))}
      if self.p8() != 0 { try!(write!(f, " p8"))}
      if self.p9() != 0 { try!(write!(f, " p9"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Serial Bus Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcusb(pub u32);
impl Pcusb {
#[doc="USB Module Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="USB Module Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pcusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet PHY Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcephy(pub u32);
impl Pcephy {
#[doc="Ethernet PHY Module Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet PHY Module Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pcephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Controller Area Network Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pccan(pub u32);
impl Pccan {
#[doc="CAN Module 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CAN Module 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="CAN Module 1 Power Control"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="CAN Module 1 Power Control"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Pccan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pccan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog-to-Digital Converter Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcadc(pub u32);
impl Pcadc {
#[doc="ADC Module 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ADC Module 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="ADC Module 1 Power Control"]
   #[inline] pub fn p1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="ADC Module 1 Power Control"]
   #[inline] pub fn set_p1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Pcadc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcadc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      if self.p1() != 0 { try!(write!(f, " p1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog Comparator Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcacmp(pub u32);
impl Pcacmp {
#[doc="Analog Comparator Module 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Analog Comparator Module 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pcacmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcacmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pulse Width Modulator Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcpwm(pub u32);
impl Pcpwm {
#[doc="PWM Module 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PWM Module 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pcpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Quadrature Encoder Interface Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcqei(pub u32);
impl Pcqei {
#[doc="QEI Module 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="QEI Module 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pcqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EEPROM Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pceeprom(pub u32);
impl Pceeprom {
#[doc="EEPROM Module 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EEPROM Module 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pceeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pceeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC and Cryptographic Modules Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcccm(pub u32);
impl Pcccm {
#[doc="CRC and Cryptographic Modules Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CRC and Cryptographic Modules Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pcccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC Power Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pcemac(pub u32);
impl Pcemac {
#[doc="Ethernet MAC Module 0 Power Control"]
   #[inline] pub fn p0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet MAC Module 0 Power Control"]
   #[inline] pub fn set_p0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pcemac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pcemac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.p0() != 0 { try!(write!(f, " p0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prwd(pub u32);
impl Prwd {
#[doc="Watchdog Timer 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Watchdog Timer 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Watchdog Timer 1 Peripheral Ready"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Watchdog Timer 1 Peripheral Ready"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Prwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prwd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="16/32-Bit General-Purpose Timer Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prtimer(pub u32);
impl Prtimer {
#[doc="16/32-Bit General-Purpose Timer 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="16/32-Bit General-Purpose Timer 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 1 Peripheral Ready"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="16/32-Bit General-Purpose Timer 1 Peripheral Ready"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 2 Peripheral Ready"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="16/32-Bit General-Purpose Timer 2 Peripheral Ready"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 3 Peripheral Ready"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="16/32-Bit General-Purpose Timer 3 Peripheral Ready"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 4 Peripheral Ready"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="16/32-Bit General-Purpose Timer 4 Peripheral Ready"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 5 Peripheral Ready"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="16/32-Bit General-Purpose Timer 5 Peripheral Ready"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 6 Peripheral Ready"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="16/32-Bit General-Purpose Timer 6 Peripheral Ready"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="16/32-Bit General-Purpose Timer 7 Peripheral Ready"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="16/32-Bit General-Purpose Timer 7 Peripheral Ready"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Prtimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prtimer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="General-Purpose Input/Output Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prgpio(pub u32);
impl Prgpio {
#[doc="GPIO Port A Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="GPIO Port A Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="GPIO Port B Peripheral Ready"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="GPIO Port B Peripheral Ready"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="GPIO Port C Peripheral Ready"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="GPIO Port C Peripheral Ready"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="GPIO Port D Peripheral Ready"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="GPIO Port D Peripheral Ready"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="GPIO Port E Peripheral Ready"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="GPIO Port E Peripheral Ready"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="GPIO Port F Peripheral Ready"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="GPIO Port F Peripheral Ready"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="GPIO Port G Peripheral Ready"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="GPIO Port G Peripheral Ready"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="GPIO Port H Peripheral Ready"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="GPIO Port H Peripheral Ready"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="GPIO Port J Peripheral Ready"]
   #[inline] pub fn r8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="GPIO Port J Peripheral Ready"]
   #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="GPIO Port K Peripheral Ready"]
   #[inline] pub fn r9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="GPIO Port K Peripheral Ready"]
   #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="GPIO Port L Peripheral Ready"]
   #[inline] pub fn r10(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="GPIO Port L Peripheral Ready"]
   #[inline] pub fn set_r10<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="GPIO Port M Peripheral Ready"]
   #[inline] pub fn r11(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="GPIO Port M Peripheral Ready"]
   #[inline] pub fn set_r11<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="GPIO Port N Peripheral Ready"]
   #[inline] pub fn r12(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="GPIO Port N Peripheral Ready"]
   #[inline] pub fn set_r12<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="GPIO Port P Peripheral Ready"]
   #[inline] pub fn r13(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="GPIO Port P Peripheral Ready"]
   #[inline] pub fn set_r13<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="GPIO Port Q Peripheral Ready"]
   #[inline] pub fn r14(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="GPIO Port Q Peripheral Ready"]
   #[inline] pub fn set_r14<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

}
impl ::core::fmt::Display for Prgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prgpio {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      if self.r8() != 0 { try!(write!(f, " r8"))}
      if self.r9() != 0 { try!(write!(f, " r9"))}
      if self.r10() != 0 { try!(write!(f, " r10"))}
      if self.r11() != 0 { try!(write!(f, " r11"))}
      if self.r12() != 0 { try!(write!(f, " r12"))}
      if self.r13() != 0 { try!(write!(f, " r13"))}
      if self.r14() != 0 { try!(write!(f, " r14"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Micro Direct Memory Access Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prdma(pub u32);
impl Prdma {
#[doc="uDMA Module Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="uDMA Module Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Prdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prdma {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EPI Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prepi(pub u32);
impl Prepi {
#[doc="EPI Module Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EPI Module Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Prepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prepi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Hibernation Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prhib(pub u32);
impl Prhib {
#[doc="Hibernation Module Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Hibernation Module Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Prhib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prhib {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pruart(pub u32);
impl Pruart {
#[doc="UART Module 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="UART Module 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="UART Module 1 Peripheral Ready"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="UART Module 1 Peripheral Ready"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="UART Module 2 Peripheral Ready"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="UART Module 2 Peripheral Ready"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="UART Module 3 Peripheral Ready"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UART Module 3 Peripheral Ready"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="UART Module 4 Peripheral Ready"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="UART Module 4 Peripheral Ready"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="UART Module 5 Peripheral Ready"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="UART Module 5 Peripheral Ready"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="UART Module 6 Peripheral Ready"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="UART Module 6 Peripheral Ready"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="UART Module 7 Peripheral Ready"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="UART Module 7 Peripheral Ready"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Pruart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pruart {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Synchronous Serial Interface Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prssi(pub u32);
impl Prssi {
#[doc="SSI Module 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="SSI Module 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="SSI Module 1 Peripheral Ready"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="SSI Module 1 Peripheral Ready"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="SSI Module 2 Peripheral Ready"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="SSI Module 2 Peripheral Ready"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="SSI Module 3 Peripheral Ready"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="SSI Module 3 Peripheral Ready"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for Prssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prssi {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Inter-Integrated Circuit Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pri2c(pub u32);
impl Pri2c {
#[doc="I2C Module 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="I2C Module 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="I2C Module 1 Peripheral Ready"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="I2C Module 1 Peripheral Ready"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="I2C Module 2 Peripheral Ready"]
   #[inline] pub fn r2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="I2C Module 2 Peripheral Ready"]
   #[inline] pub fn set_r2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="I2C Module 3 Peripheral Ready"]
   #[inline] pub fn r3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="I2C Module 3 Peripheral Ready"]
   #[inline] pub fn set_r3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="I2C Module 4 Peripheral Ready"]
   #[inline] pub fn r4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="I2C Module 4 Peripheral Ready"]
   #[inline] pub fn set_r4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="I2C Module 5 Peripheral Ready"]
   #[inline] pub fn r5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="I2C Module 5 Peripheral Ready"]
   #[inline] pub fn set_r5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="I2C Module 6 Peripheral Ready"]
   #[inline] pub fn r6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="I2C Module 6 Peripheral Ready"]
   #[inline] pub fn set_r6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="I2C Module 7 Peripheral Ready"]
   #[inline] pub fn r7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="I2C Module 7 Peripheral Ready"]
   #[inline] pub fn set_r7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="I2C Module 8 Peripheral Ready"]
   #[inline] pub fn r8(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="I2C Module 8 Peripheral Ready"]
   #[inline] pub fn set_r8<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="I2C Module 9 Peripheral Ready"]
   #[inline] pub fn r9(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="I2C Module 9 Peripheral Ready"]
   #[inline] pub fn set_r9<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

}
impl ::core::fmt::Display for Pri2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pri2c {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      if self.r2() != 0 { try!(write!(f, " r2"))}
      if self.r3() != 0 { try!(write!(f, " r3"))}
      if self.r4() != 0 { try!(write!(f, " r4"))}
      if self.r5() != 0 { try!(write!(f, " r5"))}
      if self.r6() != 0 { try!(write!(f, " r6"))}
      if self.r7() != 0 { try!(write!(f, " r7"))}
      if self.r8() != 0 { try!(write!(f, " r8"))}
      if self.r9() != 0 { try!(write!(f, " r9"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Universal Serial Bus Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prusb(pub u32);
impl Prusb {
#[doc="USB Module Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="USB Module Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Prusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prusb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet PHY Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prephy(pub u32);
impl Prephy {
#[doc="Ethernet PHY Module Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet PHY Module Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Prephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prephy {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Controller Area Network Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prcan(pub u32);
impl Prcan {
#[doc="CAN Module 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CAN Module 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="CAN Module 1 Peripheral Ready"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="CAN Module 1 Peripheral Ready"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Prcan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prcan {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog-to-Digital Converter Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pradc(pub u32);
impl Pradc {
#[doc="ADC Module 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="ADC Module 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="ADC Module 1 Peripheral Ready"]
   #[inline] pub fn r1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="ADC Module 1 Peripheral Ready"]
   #[inline] pub fn set_r1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Pradc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pradc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      if self.r1() != 0 { try!(write!(f, " r1"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Analog Comparator Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pracmp(pub u32);
impl Pracmp {
#[doc="Analog Comparator Module 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Analog Comparator Module 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Pracmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Pracmp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Pulse Width Modulator Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prpwm(pub u32);
impl Prpwm {
#[doc="PWM Module 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PWM Module 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Prpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prpwm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Quadrature Encoder Interface Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prqei(pub u32);
impl Prqei {
#[doc="QEI Module 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="QEI Module 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Prqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prqei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="EEPROM Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Preeprom(pub u32);
impl Preeprom {
#[doc="EEPROM Module Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="EEPROM Module Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Preeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Preeprom {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CRC and Cryptographic Modules Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prccm(pub u32);
impl Prccm {
#[doc="CRC and Cryptographic Modules Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="CRC and Cryptographic Modules Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Prccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prccm {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Ethernet MAC Peripheral Ready"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Premac(pub u32);
impl Premac {
#[doc="Ethernet MAC Module 0 Peripheral Ready"]
   #[inline] pub fn r0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Ethernet MAC Module 0 Peripheral Ready"]
   #[inline] pub fn set_r0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Premac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Premac {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.r0() != 0 { try!(write!(f, " r0"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait Rcgc {
   fn rcgc(&self) -> u32;
   fn set_rcgc(&self, value: u32);
}

impl Sysctl {
   #[inline] pub fn rcgc<P: Rcgc>(&self, p: &P) -> u32 {
      p.rcgc()
   }
   #[inline] pub fn set_rcgc<P: Rcgc>(&self, p: &P, value: u32) {
      p.set_rcgc(value)
   }
}

pub trait Pr {
   fn pr(&self) -> u32;
   fn set_pr(&self, value: u32);
}

impl Sysctl {
   #[inline] pub fn pr<P: Pr>(&self, p: &P) -> u32 {
      p.pr()
   }
   #[inline] pub fn set_pr<P: Pr>(&self, p: &P, value: u32) {
      p.set_pr(value)
   }
}

impl Rcgc for super::watchdog::Watchdog0 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcwd().r0().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcwd(|r| r.set_r0(value)); }
}

impl Rcgc for super::watchdog::Watchdog1 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcwd().r1().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcwd(|r| r.set_r1(value)); }
}

impl Rcgc for super::timer::Timer0 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r0().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r0(value)); }
}

impl Rcgc for super::timer::Timer1 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r1().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r1(value)); }
}

impl Rcgc for super::timer::Timer2 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r2().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r2(value)); }
}

impl Rcgc for super::timer::Timer3 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r3().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r3(value)); }
}

impl Rcgc for super::timer::Timer4 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r4().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r4(value)); }
}

impl Rcgc for super::timer::Timer5 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r5().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r5(value)); }
}

impl Rcgc for super::timer::Timer6 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r6().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r6(value)); }
}

impl Rcgc for super::timer::Timer7 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgctimer().r7().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgctimer(|r| r.set_r7(value)); }
}

impl Rcgc for super::gpio::Gpioa {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r0().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r0(value)); }
}

impl Rcgc for super::gpio::Gpiob {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r1().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r1(value)); }
}

impl Rcgc for super::gpio::Gpioc {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r2().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r2(value)); }
}

impl Rcgc for super::gpio::Gpiod {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r3().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r3(value)); }
}

impl Rcgc for super::gpio::Gpioe {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r4().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r4(value)); }
}

impl Rcgc for super::gpio::Gpiof {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r5().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r5(value)); }
}

impl Rcgc for super::gpio::Gpiog {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r6().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r6(value)); }
}

impl Rcgc for super::gpio::Gpioh {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r7().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r7(value)); }
}

impl Rcgc for super::gpio::Gpioj {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r8().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r8(value)); }
}

impl Rcgc for super::gpio::Gpiok {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r9().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r9(value)); }
}

impl Rcgc for super::gpio::Gpiol {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r10().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r10(value)); }
}

impl Rcgc for super::gpio::Gpiom {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r11().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r11(value)); }
}

impl Rcgc for super::gpio::Gpion {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r12().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r12(value)); }
}

impl Rcgc for super::gpio::Gpiop {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r13().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r13(value)); }
}

impl Rcgc for super::gpio::Gpioq {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcgpio().r14().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcgpio(|r| r.set_r14(value)); }
}

impl Rcgc for super::udma::Udma {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcdma().r0().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcdma(|r| r.set_r0(value)); }
}

impl Rcgc for super::uart::Uart0 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r0().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r0(value)); }
}

impl Rcgc for super::uart::Uart1 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r1().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r1(value)); }
}

impl Rcgc for super::uart::Uart2 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r2().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r2(value)); }
}

impl Rcgc for super::uart::Uart3 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r3().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r3(value)); }
}

impl Rcgc for super::uart::Uart4 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r4().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r4(value)); }
}

impl Rcgc for super::uart::Uart5 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r5().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r5(value)); }
}

impl Rcgc for super::uart::Uart6 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r6().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r6(value)); }
}

impl Rcgc for super::uart::Uart7 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcuart().r7().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcuart(|r| r.set_r7(value)); }
}

impl Rcgc for super::adc::Adc0 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcadc().r0().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcadc(|r| r.set_r0(value)); }
}

impl Rcgc for super::adc::Adc1 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcadc().r1().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcadc(|r| r.set_r1(value)); }
}

impl Rcgc for super::pwm::Pwm0 {
   #[inline] fn rcgc(&self) -> u32 { SYSCTL.rcgcpwm().r0().into() }
   #[inline] fn set_rcgc(&self, value: u32) { SYSCTL.with_rcgcpwm(|r| r.set_r0(value)); }
}

impl Pr for super::watchdog::Watchdog0 {
   #[inline] fn pr(&self) -> u32 { SYSCTL.prwd().r0().into() }
   #[inline] fn set_pr(&self, value: u32) { SYSCTL.with_prwd(|r| r.set_r0(value)); }
}

impl Pr for super::watchdog::Watchdog1 {
   #[inline] fn pr(&self) -> u32 { SYSCTL.prwd().r1().into() }
   #[inline] fn set_pr(&self, value: u32) { SYSCTL.with_prwd(|r| r.set_r1(value)); }
}


