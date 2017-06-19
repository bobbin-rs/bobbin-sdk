
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

  #[inline] pub fn es_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
  #[inline] pub fn es_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
  #[inline] pub fn es(&self) -> Es { 
     unsafe {
        Es(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }

  #[inline] pub fn erq_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
  #[inline] pub fn erq_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
  #[inline] pub fn erq(&self) -> Erq { 
     unsafe {
        Erq(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  #[inline] pub fn set_erq(&self, value: Erq) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_erq<F: FnOnce(Erq) -> Erq>(&self, f: F) -> &Self {
     let tmp = self.erq();
     self.set_erq(f(tmp))
  }

  #[inline] pub fn eei_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
  #[inline] pub fn eei_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
  #[inline] pub fn eei(&self) -> Eei { 
     unsafe {
        Eei(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
  #[inline] pub fn set_eei(&self, value: Eei) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_eei<F: FnOnce(Eei) -> Eei>(&self, f: F) -> &Self {
     let tmp = self.eei();
     self.set_eei(f(tmp))
  }

  #[inline] pub fn ceei_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x18) as *const u8
  }
  #[inline] pub fn ceei_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x18) as *mut u8
  }
  #[inline] pub fn set_ceei(&self, value: Ceei) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u8, value.0);
     }
     self
  }

  #[inline] pub fn seei_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x19) as *const u8
  }
  #[inline] pub fn seei_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x19) as *mut u8
  }
  #[inline] pub fn set_seei(&self, value: Seei) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x19) as *mut u8, value.0);
     }
     self
  }

  #[inline] pub fn cerq_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1a) as *const u8
  }
  #[inline] pub fn cerq_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1a) as *mut u8
  }
  #[inline] pub fn set_cerq(&self, value: Cerq) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1a) as *mut u8, value.0);
     }
     self
  }

  #[inline] pub fn serq_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1b) as *const u8
  }
  #[inline] pub fn serq_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1b) as *mut u8
  }
  #[inline] pub fn set_serq(&self, value: Serq) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1b) as *mut u8, value.0);
     }
     self
  }

  #[inline] pub fn cdne_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1c) as *const u8
  }
  #[inline] pub fn cdne_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1c) as *mut u8
  }
  #[inline] pub fn set_cdne(&self, value: Cdne) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u8, value.0);
     }
     self
  }

  #[inline] pub fn ssrt_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1d) as *const u8
  }
  #[inline] pub fn ssrt_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1d) as *mut u8
  }
  #[inline] pub fn set_ssrt(&self, value: Ssrt) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1d) as *mut u8, value.0);
     }
     self
  }

  #[inline] pub fn cerr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1e) as *const u8
  }
  #[inline] pub fn cerr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1e) as *mut u8
  }
  #[inline] pub fn set_cerr(&self, value: Cerr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1e) as *mut u8, value.0);
     }
     self
  }

  #[inline] pub fn cint_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1f) as *const u8
  }
  #[inline] pub fn cint_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1f) as *mut u8
  }
  #[inline] pub fn set_cint(&self, value: Cint) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1f) as *mut u8, value.0);
     }
     self
  }

  #[inline] pub fn int_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
  #[inline] pub fn int_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
  #[inline] pub fn int(&self) -> Int { 
     unsafe {
        Int(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
  #[inline] pub fn set_int(&self, value: Int) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_int<F: FnOnce(Int) -> Int>(&self, f: F) -> &Self {
     let tmp = self.int();
     self.set_int(f(tmp))
  }

  #[inline] pub fn err_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
  #[inline] pub fn err_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
  #[inline] pub fn err(&self) -> Err { 
     unsafe {
        Err(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
  #[inline] pub fn set_err(&self, value: Err) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_err<F: FnOnce(Err) -> Err>(&self, f: F) -> &Self {
     let tmp = self.err();
     self.set_err(f(tmp))
  }

  #[inline] pub fn hrs_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
  #[inline] pub fn hrs_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
  #[inline] pub fn hrs(&self) -> Hrs { 
     unsafe {
        Hrs(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }

  #[inline] pub fn dchpri_ptr(&self, index: usize) -> *const u8 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x100 + (index)) as *const u8
  }
  #[inline] pub fn dchpri_mut(&self, index: usize) -> *mut u8 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x100 + (index)) as *mut u8
  }
  #[inline] pub fn dchpri(&self, index: usize) -> Dchpri { 
     assert!(index < 16);
     unsafe {
        Dchpri(::core::ptr::read_volatile(((self.0 as usize) + 0x100 + (index)) as *const u8))
     }
  }
  #[inline] pub fn set_dchpri(&self, index: usize, value: Dchpri) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100 + (index)) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_dchpri<F: FnOnce(Dchpri) -> Dchpri>(&self, index: usize, f: F) -> &Self {
     let tmp = self.dchpri(index);
     self.set_dchpri(index, f(tmp))
  }

  #[inline] pub fn tcd_saddr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1000 + (index * 32)) as *const u32
  }
  #[inline] pub fn tcd_saddr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1000 + (index * 32)) as *mut u32
  }
  #[inline] pub fn tcd_saddr(&self, index: usize) -> TcdSaddr { 
     assert!(index < 16);
     unsafe {
        TcdSaddr(::core::ptr::read_volatile(((self.0 as usize) + 0x1000 + (index * 32)) as *const u32))
     }
  }
  #[inline] pub fn set_tcd_saddr(&self, index: usize, value: TcdSaddr) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1000 + (index * 32)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_saddr<F: FnOnce(TcdSaddr) -> TcdSaddr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_saddr(index);
     self.set_tcd_saddr(index, f(tmp))
  }

  #[inline] pub fn tcd_soff_ptr(&self, index: usize) -> *const u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1004 + (index * 32)) as *const u16
  }
  #[inline] pub fn tcd_soff_mut(&self, index: usize) -> *mut u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1004 + (index * 32)) as *mut u16
  }
  #[inline] pub fn tcd_soff(&self, index: usize) -> TcdSoff { 
     assert!(index < 16);
     unsafe {
        TcdSoff(::core::ptr::read_volatile(((self.0 as usize) + 0x1004 + (index * 32)) as *const u16))
     }
  }
  #[inline] pub fn set_tcd_soff(&self, index: usize, value: TcdSoff) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1004 + (index * 32)) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_soff<F: FnOnce(TcdSoff) -> TcdSoff>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_soff(index);
     self.set_tcd_soff(index, f(tmp))
  }

  #[inline] pub fn tcd_attr_ptr(&self, index: usize) -> *const u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1006 + (index * 32)) as *const u16
  }
  #[inline] pub fn tcd_attr_mut(&self, index: usize) -> *mut u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1006 + (index * 32)) as *mut u16
  }
  #[inline] pub fn tcd_attr(&self, index: usize) -> TcdAttr { 
     assert!(index < 16);
     unsafe {
        TcdAttr(::core::ptr::read_volatile(((self.0 as usize) + 0x1006 + (index * 32)) as *const u16))
     }
  }
  #[inline] pub fn set_tcd_attr(&self, index: usize, value: TcdAttr) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1006 + (index * 32)) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_attr<F: FnOnce(TcdAttr) -> TcdAttr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_attr(index);
     self.set_tcd_attr(index, f(tmp))
  }

  #[inline] pub fn tcd_nbytes_mlno_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1008 + (index * 32)) as *const u32
  }
  #[inline] pub fn tcd_nbytes_mlno_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1008 + (index * 32)) as *mut u32
  }
  #[inline] pub fn tcd_nbytes_mlno(&self, index: usize) -> TcdNbytesMlno { 
     assert!(index < 16);
     unsafe {
        TcdNbytesMlno(::core::ptr::read_volatile(((self.0 as usize) + 0x1008 + (index * 32)) as *const u32))
     }
  }
  #[inline] pub fn set_tcd_nbytes_mlno(&self, index: usize, value: TcdNbytesMlno) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1008 + (index * 32)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_nbytes_mlno<F: FnOnce(TcdNbytesMlno) -> TcdNbytesMlno>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_nbytes_mlno(index);
     self.set_tcd_nbytes_mlno(index, f(tmp))
  }

  #[inline] pub fn tcd_nbytes_mloffno_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1008 + (index * 32)) as *const u32
  }
  #[inline] pub fn tcd_nbytes_mloffno_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1008 + (index * 32)) as *mut u32
  }
  #[inline] pub fn tcd_nbytes_mloffno(&self, index: usize) -> TcdNbytesMloffno { 
     assert!(index < 16);
     unsafe {
        TcdNbytesMloffno(::core::ptr::read_volatile(((self.0 as usize) + 0x1008 + (index * 32)) as *const u32))
     }
  }
  #[inline] pub fn set_tcd_nbytes_mloffno(&self, index: usize, value: TcdNbytesMloffno) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1008 + (index * 32)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_nbytes_mloffno<F: FnOnce(TcdNbytesMloffno) -> TcdNbytesMloffno>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_nbytes_mloffno(index);
     self.set_tcd_nbytes_mloffno(index, f(tmp))
  }

  #[inline] pub fn tcd_nbytes_mloffyes_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1008 + (index * 32)) as *const u32
  }
  #[inline] pub fn tcd_nbytes_mloffyes_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1008 + (index * 32)) as *mut u32
  }
  #[inline] pub fn tcd_nbytes_mloffyes(&self, index: usize) -> TcdNbytesMloffyes { 
     assert!(index < 16);
     unsafe {
        TcdNbytesMloffyes(::core::ptr::read_volatile(((self.0 as usize) + 0x1008 + (index * 32)) as *const u32))
     }
  }
  #[inline] pub fn set_tcd_nbytes_mloffyes(&self, index: usize, value: TcdNbytesMloffyes) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1008 + (index * 32)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_nbytes_mloffyes<F: FnOnce(TcdNbytesMloffyes) -> TcdNbytesMloffyes>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_nbytes_mloffyes(index);
     self.set_tcd_nbytes_mloffyes(index, f(tmp))
  }

  #[inline] pub fn tcd_slast_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x100c + (index * 32)) as *const u32
  }
  #[inline] pub fn tcd_slast_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x100c + (index * 32)) as *mut u32
  }
  #[inline] pub fn tcd_slast(&self, index: usize) -> TcdSlast { 
     assert!(index < 16);
     unsafe {
        TcdSlast(::core::ptr::read_volatile(((self.0 as usize) + 0x100c + (index * 32)) as *const u32))
     }
  }
  #[inline] pub fn set_tcd_slast(&self, index: usize, value: TcdSlast) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x100c + (index * 32)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_slast<F: FnOnce(TcdSlast) -> TcdSlast>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_slast(index);
     self.set_tcd_slast(index, f(tmp))
  }

  #[inline] pub fn tcd_daddr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1010 + (index * 32)) as *const u32
  }
  #[inline] pub fn tcd_daddr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1010 + (index * 32)) as *mut u32
  }
  #[inline] pub fn tcd_daddr(&self, index: usize) -> TcdDaddr { 
     assert!(index < 16);
     unsafe {
        TcdDaddr(::core::ptr::read_volatile(((self.0 as usize) + 0x1010 + (index * 32)) as *const u32))
     }
  }
  #[inline] pub fn set_tcd_daddr(&self, index: usize, value: TcdDaddr) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1010 + (index * 32)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_daddr<F: FnOnce(TcdDaddr) -> TcdDaddr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_daddr(index);
     self.set_tcd_daddr(index, f(tmp))
  }

  #[inline] pub fn tcd_doff_ptr(&self, index: usize) -> *const u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1014 + (index * 32)) as *const u16
  }
  #[inline] pub fn tcd_doff_mut(&self, index: usize) -> *mut u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1014 + (index * 32)) as *mut u16
  }
  #[inline] pub fn tcd_doff(&self, index: usize) -> TcdDoff { 
     assert!(index < 16);
     unsafe {
        TcdDoff(::core::ptr::read_volatile(((self.0 as usize) + 0x1014 + (index * 32)) as *const u16))
     }
  }
  #[inline] pub fn set_tcd_doff(&self, index: usize, value: TcdDoff) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1014 + (index * 32)) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_doff<F: FnOnce(TcdDoff) -> TcdDoff>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_doff(index);
     self.set_tcd_doff(index, f(tmp))
  }

  #[inline] pub fn tcd_citer_elinkno_ptr(&self, index: usize) -> *const u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1016 + (index * 32)) as *const u16
  }
  #[inline] pub fn tcd_citer_elinkno_mut(&self, index: usize) -> *mut u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1016 + (index * 32)) as *mut u16
  }
  #[inline] pub fn tcd_citer_elinkno(&self, index: usize) -> TcdCiterElinkno { 
     assert!(index < 16);
     unsafe {
        TcdCiterElinkno(::core::ptr::read_volatile(((self.0 as usize) + 0x1016 + (index * 32)) as *const u16))
     }
  }
  #[inline] pub fn set_tcd_citer_elinkno(&self, index: usize, value: TcdCiterElinkno) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1016 + (index * 32)) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_citer_elinkno<F: FnOnce(TcdCiterElinkno) -> TcdCiterElinkno>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_citer_elinkno(index);
     self.set_tcd_citer_elinkno(index, f(tmp))
  }

  #[inline] pub fn tcd_citer_elinkyes_ptr(&self, index: usize) -> *const u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1016 + (index * 32)) as *const u16
  }
  #[inline] pub fn tcd_citer_elinkyes_mut(&self, index: usize) -> *mut u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1016 + (index * 32)) as *mut u16
  }
  #[inline] pub fn tcd_citer_elinkyes(&self, index: usize) -> TcdCiterElinkyes { 
     assert!(index < 16);
     unsafe {
        TcdCiterElinkyes(::core::ptr::read_volatile(((self.0 as usize) + 0x1016 + (index * 32)) as *const u16))
     }
  }
  #[inline] pub fn set_tcd_citer_elinkyes(&self, index: usize, value: TcdCiterElinkyes) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1016 + (index * 32)) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_citer_elinkyes<F: FnOnce(TcdCiterElinkyes) -> TcdCiterElinkyes>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_citer_elinkyes(index);
     self.set_tcd_citer_elinkyes(index, f(tmp))
  }

  #[inline] pub fn tcd_dlastsga_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1018 + (index * 32)) as *const u32
  }
  #[inline] pub fn tcd_dlastsga_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x1018 + (index * 32)) as *mut u32
  }
  #[inline] pub fn tcd_dlastsga(&self, index: usize) -> TcdDlastsga { 
     assert!(index < 16);
     unsafe {
        TcdDlastsga(::core::ptr::read_volatile(((self.0 as usize) + 0x1018 + (index * 32)) as *const u32))
     }
  }
  #[inline] pub fn set_tcd_dlastsga(&self, index: usize, value: TcdDlastsga) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1018 + (index * 32)) as *mut u32, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_dlastsga<F: FnOnce(TcdDlastsga) -> TcdDlastsga>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_dlastsga(index);
     self.set_tcd_dlastsga(index, f(tmp))
  }

  #[inline] pub fn tcd_csr_ptr(&self, index: usize) -> *const u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x101c + (index * 32)) as *const u16
  }
  #[inline] pub fn tcd_csr_mut(&self, index: usize) -> *mut u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x101c + (index * 32)) as *mut u16
  }
  #[inline] pub fn tcd_csr(&self, index: usize) -> TcdCsr { 
     assert!(index < 16);
     unsafe {
        TcdCsr(::core::ptr::read_volatile(((self.0 as usize) + 0x101c + (index * 32)) as *const u16))
     }
  }
  #[inline] pub fn set_tcd_csr(&self, index: usize, value: TcdCsr) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x101c + (index * 32)) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_csr<F: FnOnce(TcdCsr) -> TcdCsr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_csr(index);
     self.set_tcd_csr(index, f(tmp))
  }

  #[inline] pub fn tcd_biter_elinkno_ptr(&self, index: usize) -> *const u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x101e + (index * 32)) as *const u16
  }
  #[inline] pub fn tcd_biter_elinkno_mut(&self, index: usize) -> *mut u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x101e + (index * 32)) as *mut u16
  }
  #[inline] pub fn tcd_biter_elinkno(&self, index: usize) -> TcdBiterElinkno { 
     assert!(index < 16);
     unsafe {
        TcdBiterElinkno(::core::ptr::read_volatile(((self.0 as usize) + 0x101e + (index * 32)) as *const u16))
     }
  }
  #[inline] pub fn set_tcd_biter_elinkno(&self, index: usize, value: TcdBiterElinkno) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x101e + (index * 32)) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_biter_elinkno<F: FnOnce(TcdBiterElinkno) -> TcdBiterElinkno>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_biter_elinkno(index);
     self.set_tcd_biter_elinkno(index, f(tmp))
  }

  #[inline] pub fn tcd_biter_elinkyes_ptr(&self, index: usize) -> *const u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x101e + (index * 32)) as *const u16
  }
  #[inline] pub fn tcd_biter_elinkyes_mut(&self, index: usize) -> *mut u16 { 
     assert!(index < 16);
     ((self.0 as usize) + 0x101e + (index * 32)) as *mut u16
  }
  #[inline] pub fn tcd_biter_elinkyes(&self, index: usize) -> TcdBiterElinkyes { 
     assert!(index < 16);
     unsafe {
        TcdBiterElinkyes(::core::ptr::read_volatile(((self.0 as usize) + 0x101e + (index * 32)) as *const u16))
     }
  }
  #[inline] pub fn set_tcd_biter_elinkyes(&self, index: usize, value: TcdBiterElinkyes) -> &Self {
     assert!(index < 16);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x101e + (index * 32)) as *mut u16, value.0);
     }
     self
  }
  #[inline] pub fn with_tcd_biter_elinkyes<F: FnOnce(TcdBiterElinkyes) -> TcdBiterElinkyes>(&self, index: usize, f: F) -> &Self {
     let tmp = self.tcd_biter_elinkyes(index);
     self.set_tcd_biter_elinkyes(index, f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
  #[inline] pub fn edbg(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_edbg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn erca(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_erca(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn hoe(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_hoe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn halt(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_halt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn clm(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_clm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn emlm(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_emlm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn ecx(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_ecx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn cx(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
  #[inline] pub fn set_cx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
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
      if self.edbg() != 0 { try!(write!(f, " edbg"))}
      if self.erca() != 0 { try!(write!(f, " erca"))}
      if self.hoe() != 0 { try!(write!(f, " hoe"))}
      if self.halt() != 0 { try!(write!(f, " halt"))}
      if self.clm() != 0 { try!(write!(f, " clm"))}
      if self.emlm() != 0 { try!(write!(f, " emlm"))}
      if self.ecx() != 0 { try!(write!(f, " ecx"))}
      if self.cx() != 0 { try!(write!(f, " cx"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Es(pub u32);
impl Es {
  #[inline] pub fn dbe(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_dbe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn sbe(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_sbe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn sge(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_sge(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn nce(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_nce(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn doe(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_doe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn dae(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_dae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn soe(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_soe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn sae(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_sae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn errchn(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_errchn(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn cpe(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_cpe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline] pub fn ecx(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  #[inline] pub fn set_ecx(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

  #[inline] pub fn vld(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_vld(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Es {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Es {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbe() != 0 { try!(write!(f, " dbe"))}
      if self.sbe() != 0 { try!(write!(f, " sbe"))}
      if self.sge() != 0 { try!(write!(f, " sge"))}
      if self.nce() != 0 { try!(write!(f, " nce"))}
      if self.doe() != 0 { try!(write!(f, " doe"))}
      if self.dae() != 0 { try!(write!(f, " dae"))}
      if self.soe() != 0 { try!(write!(f, " soe"))}
      if self.sae() != 0 { try!(write!(f, " sae"))}
      if self.errchn() != 0 { try!(write!(f, " errchn=0x{:x}", self.errchn()))}
      if self.cpe() != 0 { try!(write!(f, " cpe"))}
      if self.ecx() != 0 { try!(write!(f, " ecx"))}
      if self.vld() != 0 { try!(write!(f, " vld"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Erq(pub u32);
impl Erq {
  #[inline] pub fn erq0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_erq0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn erq1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_erq1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn erq2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_erq2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn erq3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_erq3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn erq4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_erq4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn erq5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_erq5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn erq6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_erq6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn erq7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_erq7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn erq8(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  #[inline] pub fn set_erq8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn erq9(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  #[inline] pub fn set_erq9(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn erq10(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  #[inline] pub fn set_erq10(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn erq11(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  #[inline] pub fn set_erq11(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  #[inline] pub fn erq12(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  #[inline] pub fn set_erq12(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  #[inline] pub fn erq13(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  #[inline] pub fn set_erq13(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  #[inline] pub fn erq14(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  #[inline] pub fn set_erq14(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  #[inline] pub fn erq15(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  #[inline] pub fn set_erq15(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Erq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Erq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.erq0() != 0 { try!(write!(f, " erq0"))}
      if self.erq1() != 0 { try!(write!(f, " erq1"))}
      if self.erq2() != 0 { try!(write!(f, " erq2"))}
      if self.erq3() != 0 { try!(write!(f, " erq3"))}
      if self.erq4() != 0 { try!(write!(f, " erq4"))}
      if self.erq5() != 0 { try!(write!(f, " erq5"))}
      if self.erq6() != 0 { try!(write!(f, " erq6"))}
      if self.erq7() != 0 { try!(write!(f, " erq7"))}
      if self.erq8() != 0 { try!(write!(f, " erq8"))}
      if self.erq9() != 0 { try!(write!(f, " erq9"))}
      if self.erq10() != 0 { try!(write!(f, " erq10"))}
      if self.erq11() != 0 { try!(write!(f, " erq11"))}
      if self.erq12() != 0 { try!(write!(f, " erq12"))}
      if self.erq13() != 0 { try!(write!(f, " erq13"))}
      if self.erq14() != 0 { try!(write!(f, " erq14"))}
      if self.erq15() != 0 { try!(write!(f, " erq15"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Eei(pub u32);
impl Eei {
  #[inline] pub fn eei(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_eei(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Eei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Eei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.eei(0) != 0 { try!(write!(f, " eei[0]"))}
      if self.eei(1) != 0 { try!(write!(f, " eei[1]"))}
      if self.eei(2) != 0 { try!(write!(f, " eei[2]"))}
      if self.eei(3) != 0 { try!(write!(f, " eei[3]"))}
      if self.eei(4) != 0 { try!(write!(f, " eei[4]"))}
      if self.eei(5) != 0 { try!(write!(f, " eei[5]"))}
      if self.eei(6) != 0 { try!(write!(f, " eei[6]"))}
      if self.eei(7) != 0 { try!(write!(f, " eei[7]"))}
      if self.eei(8) != 0 { try!(write!(f, " eei[8]"))}
      if self.eei(9) != 0 { try!(write!(f, " eei[9]"))}
      if self.eei(10) != 0 { try!(write!(f, " eei[10]"))}
      if self.eei(11) != 0 { try!(write!(f, " eei[11]"))}
      if self.eei(12) != 0 { try!(write!(f, " eei[12]"))}
      if self.eei(13) != 0 { try!(write!(f, " eei[13]"))}
      if self.eei(14) != 0 { try!(write!(f, " eei[14]"))}
      if self.eei(15) != 0 { try!(write!(f, " eei[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ceei(pub u8);
impl Ceei {
  #[inline] pub fn ceei(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_ceei(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn caee(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_caee(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn nop(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_nop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Ceei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ceei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ceei() != 0 { try!(write!(f, " ceei=0x{:x}", self.ceei()))}
      if self.caee() != 0 { try!(write!(f, " caee"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Seei(pub u8);
impl Seei {
  #[inline] pub fn seei(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_seei(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn saee(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_saee(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn nop(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_nop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Seei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Seei {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.seei() != 0 { try!(write!(f, " seei=0x{:x}", self.seei()))}
      if self.saee() != 0 { try!(write!(f, " saee"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cerq(pub u8);
impl Cerq {
  #[inline] pub fn cerq(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_cerq(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn caer(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_caer(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn nop(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_nop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Cerq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cerq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cerq() != 0 { try!(write!(f, " cerq=0x{:x}", self.cerq()))}
      if self.caer() != 0 { try!(write!(f, " caer"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Serq(pub u8);
impl Serq {
  #[inline] pub fn serq(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_serq(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn saer(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_saer(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn nop(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_nop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Serq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Serq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.serq() != 0 { try!(write!(f, " serq=0x{:x}", self.serq()))}
      if self.saer() != 0 { try!(write!(f, " saer"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cdne(pub u8);
impl Cdne {
  #[inline] pub fn cdne(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_cdne(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn cadn(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_cadn(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn nop(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_nop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Cdne {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cdne {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cdne() != 0 { try!(write!(f, " cdne=0x{:x}", self.cdne()))}
      if self.cadn() != 0 { try!(write!(f, " cadn"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ssrt(pub u8);
impl Ssrt {
  #[inline] pub fn ssrt(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_ssrt(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn sast(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_sast(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn nop(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_nop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Ssrt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ssrt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ssrt() != 0 { try!(write!(f, " ssrt=0x{:x}", self.ssrt()))}
      if self.sast() != 0 { try!(write!(f, " sast"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cerr(pub u8);
impl Cerr {
  #[inline] pub fn cerr(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_cerr(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn caei(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_caei(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn nop(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_nop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Cerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cerr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cerr() != 0 { try!(write!(f, " cerr=0x{:x}", self.cerr()))}
      if self.caei() != 0 { try!(write!(f, " caei"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cint(pub u8);
impl Cint {
  #[inline] pub fn cint(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_cint(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn cair(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_cair(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn nop(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_nop(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Cint {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cint {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cint() != 0 { try!(write!(f, " cint=0x{:x}", self.cint()))}
      if self.cair() != 0 { try!(write!(f, " cair"))}
      if self.nop() != 0 { try!(write!(f, " nop"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Int(pub u32);
impl Int {
  #[inline] pub fn int(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_int(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Int {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Int {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.int(0) != 0 { try!(write!(f, " int[0]"))}
      if self.int(1) != 0 { try!(write!(f, " int[1]"))}
      if self.int(2) != 0 { try!(write!(f, " int[2]"))}
      if self.int(3) != 0 { try!(write!(f, " int[3]"))}
      if self.int(4) != 0 { try!(write!(f, " int[4]"))}
      if self.int(5) != 0 { try!(write!(f, " int[5]"))}
      if self.int(6) != 0 { try!(write!(f, " int[6]"))}
      if self.int(7) != 0 { try!(write!(f, " int[7]"))}
      if self.int(8) != 0 { try!(write!(f, " int[8]"))}
      if self.int(9) != 0 { try!(write!(f, " int[9]"))}
      if self.int(10) != 0 { try!(write!(f, " int[10]"))}
      if self.int(11) != 0 { try!(write!(f, " int[11]"))}
      if self.int(12) != 0 { try!(write!(f, " int[12]"))}
      if self.int(13) != 0 { try!(write!(f, " int[13]"))}
      if self.int(14) != 0 { try!(write!(f, " int[14]"))}
      if self.int(15) != 0 { try!(write!(f, " int[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Err(pub u32);
impl Err {
  #[inline] pub fn err(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_err(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Err {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Err {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.err(0) != 0 { try!(write!(f, " err[0]"))}
      if self.err(1) != 0 { try!(write!(f, " err[1]"))}
      if self.err(2) != 0 { try!(write!(f, " err[2]"))}
      if self.err(3) != 0 { try!(write!(f, " err[3]"))}
      if self.err(4) != 0 { try!(write!(f, " err[4]"))}
      if self.err(5) != 0 { try!(write!(f, " err[5]"))}
      if self.err(6) != 0 { try!(write!(f, " err[6]"))}
      if self.err(7) != 0 { try!(write!(f, " err[7]"))}
      if self.err(8) != 0 { try!(write!(f, " err[8]"))}
      if self.err(9) != 0 { try!(write!(f, " err[9]"))}
      if self.err(10) != 0 { try!(write!(f, " err[10]"))}
      if self.err(11) != 0 { try!(write!(f, " err[11]"))}
      if self.err(12) != 0 { try!(write!(f, " err[12]"))}
      if self.err(13) != 0 { try!(write!(f, " err[13]"))}
      if self.err(14) != 0 { try!(write!(f, " err[14]"))}
      if self.err(15) != 0 { try!(write!(f, " err[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Hrs(pub u32);
impl Hrs {
  #[inline] pub fn hrs(&self, index: usize) -> u32 {
     assert!(index < 16);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  #[inline] pub fn set_hrs(mut self, index: usize, value: u32) -> Self {
     assert!(index < 16);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Hrs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Hrs {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.hrs(0) != 0 { try!(write!(f, " hrs[0]"))}
      if self.hrs(1) != 0 { try!(write!(f, " hrs[1]"))}
      if self.hrs(2) != 0 { try!(write!(f, " hrs[2]"))}
      if self.hrs(3) != 0 { try!(write!(f, " hrs[3]"))}
      if self.hrs(4) != 0 { try!(write!(f, " hrs[4]"))}
      if self.hrs(5) != 0 { try!(write!(f, " hrs[5]"))}
      if self.hrs(6) != 0 { try!(write!(f, " hrs[6]"))}
      if self.hrs(7) != 0 { try!(write!(f, " hrs[7]"))}
      if self.hrs(8) != 0 { try!(write!(f, " hrs[8]"))}
      if self.hrs(9) != 0 { try!(write!(f, " hrs[9]"))}
      if self.hrs(10) != 0 { try!(write!(f, " hrs[10]"))}
      if self.hrs(11) != 0 { try!(write!(f, " hrs[11]"))}
      if self.hrs(12) != 0 { try!(write!(f, " hrs[12]"))}
      if self.hrs(13) != 0 { try!(write!(f, " hrs[13]"))}
      if self.hrs(14) != 0 { try!(write!(f, " hrs[14]"))}
      if self.hrs(15) != 0 { try!(write!(f, " hrs[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dchpri(pub u8);
impl Dchpri {
  #[inline] pub fn chpri(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_chpri(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn dpa(&self) -> u8 {
     ((self.0 as u8) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_dpa(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn ecp(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_ecp(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Dchpri {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dchpri {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chpri() != 0 { try!(write!(f, " chpri=0x{:x}", self.chpri()))}
      if self.dpa() != 0 { try!(write!(f, " dpa"))}
      if self.ecp() != 0 { try!(write!(f, " ecp"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdSaddr(pub u32);
impl TcdSaddr {
  #[inline] pub fn saddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_saddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for TcdSaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdSaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdSoff(pub u16);
impl TcdSoff {
  #[inline] pub fn soff(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_soff(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for TcdSoff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdSoff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.soff() != 0 { try!(write!(f, " soff=0x{:x}", self.soff()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdAttr(pub u16);
impl TcdAttr {
  #[inline] pub fn dsize(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x7 // [2:0]
  }
  #[inline] pub fn set_dsize(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn dmod(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1f // [7:3]
  }
  #[inline] pub fn set_dmod(mut self, value: u16) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn ssize(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
  #[inline] pub fn set_ssize(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn smod(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1f // [15:11]
  }
  #[inline] pub fn set_smod(mut self, value: u16) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 11);
     self.0 |= value << 11;
     self
  }

}
impl ::core::fmt::Display for TcdAttr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdAttr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dsize() != 0 { try!(write!(f, " dsize=0x{:x}", self.dsize()))}
      if self.dmod() != 0 { try!(write!(f, " dmod=0x{:x}", self.dmod()))}
      if self.ssize() != 0 { try!(write!(f, " ssize=0x{:x}", self.ssize()))}
      if self.smod() != 0 { try!(write!(f, " smod=0x{:x}", self.smod()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdNbytesMlno(pub u32);
impl TcdNbytesMlno {
  #[inline] pub fn nbytes(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_nbytes(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for TcdNbytesMlno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdNbytesMlno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdNbytesMloffno(pub u32);
impl TcdNbytesMloffno {
  #[inline] pub fn nbytes(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3fffffff // [29:0]
  }
  #[inline] pub fn set_nbytes(mut self, value: u32) -> Self {
     assert!((value & !0x3fffffff) == 0);
     self.0 &= !(0x3fffffff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn dmloe(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline] pub fn set_dmloe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline] pub fn smloe(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_smloe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for TcdNbytesMloffno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdNbytesMloffno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.nbytes() != 0 { try!(write!(f, " nbytes=0x{:x}", self.nbytes()))}
      if self.dmloe() != 0 { try!(write!(f, " dmloe"))}
      if self.smloe() != 0 { try!(write!(f, " smloe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdNbytesMloffyes(pub u32);
impl TcdNbytesMloffyes {
  #[inline] pub fn nbytes(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3ff // [9:0]
  }
  #[inline] pub fn set_nbytes(mut self, value: u32) -> Self {
     assert!((value & !0x3ff) == 0);
     self.0 &= !(0x3ff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn mloff(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0xfffff // [29:10]
  }
  #[inline] pub fn set_mloff(mut self, value: u32) -> Self {
     assert!((value & !0xfffff) == 0);
     self.0 &= !(0xfffff << 10);
     self.0 |= value << 10;
     self
  }

  #[inline] pub fn dmloe(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
  #[inline] pub fn set_dmloe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

  #[inline] pub fn smloe(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  #[inline] pub fn set_smloe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for TcdNbytesMloffyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdNbytesMloffyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.nbytes() != 0 { try!(write!(f, " nbytes=0x{:x}", self.nbytes()))}
      if self.mloff() != 0 { try!(write!(f, " mloff=0x{:x}", self.mloff()))}
      if self.dmloe() != 0 { try!(write!(f, " dmloe"))}
      if self.smloe() != 0 { try!(write!(f, " smloe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdSlast(pub u32);
impl TcdSlast {
  #[inline] pub fn slast(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_slast(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for TcdSlast {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdSlast {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdDaddr(pub u32);
impl TcdDaddr {
  #[inline] pub fn daddr(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_daddr(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for TcdDaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdDaddr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdDoff(pub u16);
impl TcdDoff {
  #[inline] pub fn doff(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
  #[inline] pub fn set_doff(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for TcdDoff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdDoff {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.doff() != 0 { try!(write!(f, " doff=0x{:x}", self.doff()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdCiterElinkno(pub u16);
impl TcdCiterElinkno {
  #[inline] pub fn citer(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x7fff // [14:0]
  }
  #[inline] pub fn set_citer(mut self, value: u16) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn elink(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline] pub fn set_elink(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for TcdCiterElinkno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdCiterElinkno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.citer() != 0 { try!(write!(f, " citer=0x{:x}", self.citer()))}
      if self.elink() != 0 { try!(write!(f, " elink"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdCiterElinkyes(pub u16);
impl TcdCiterElinkyes {
  #[inline] pub fn citer(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1ff // [8:0]
  }
  #[inline] pub fn set_citer(mut self, value: u16) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn linkch(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0xf // [12:9]
  }
  #[inline] pub fn set_linkch(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn elink(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline] pub fn set_elink(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for TcdCiterElinkyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdCiterElinkyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.citer() != 0 { try!(write!(f, " citer=0x{:x}", self.citer()))}
      if self.linkch() != 0 { try!(write!(f, " linkch=0x{:x}", self.linkch()))}
      if self.elink() != 0 { try!(write!(f, " elink"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdDlastsga(pub u32);
impl TcdDlastsga {
  #[inline] pub fn dlastsga(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  #[inline] pub fn set_dlastsga(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for TcdDlastsga {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdDlastsga {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdCsr(pub u16);
impl TcdCsr {
  #[inline] pub fn start(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_start(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn intmajor(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_intmajor(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn inthalf(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_inthalf(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn dreq(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
  #[inline] pub fn set_dreq(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  #[inline] pub fn esg(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
  #[inline] pub fn set_esg(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  #[inline] pub fn majorelink(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
  #[inline] pub fn set_majorelink(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  #[inline] pub fn active(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
  #[inline] pub fn set_active(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  #[inline] pub fn done(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_done(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  #[inline] pub fn majorlinkch(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0xf // [11:8]
  }
  #[inline] pub fn set_majorlinkch(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  #[inline] pub fn bwc(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x3 // [15:14]
  }
  #[inline] pub fn set_bwc(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

}
impl ::core::fmt::Display for TcdCsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdCsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.start() != 0 { try!(write!(f, " start"))}
      if self.intmajor() != 0 { try!(write!(f, " intmajor"))}
      if self.inthalf() != 0 { try!(write!(f, " inthalf"))}
      if self.dreq() != 0 { try!(write!(f, " dreq"))}
      if self.esg() != 0 { try!(write!(f, " esg"))}
      if self.majorelink() != 0 { try!(write!(f, " majorelink"))}
      if self.active() != 0 { try!(write!(f, " active"))}
      if self.done() != 0 { try!(write!(f, " done"))}
      if self.majorlinkch() != 0 { try!(write!(f, " majorlinkch=0x{:x}", self.majorlinkch()))}
      if self.bwc() != 0 { try!(write!(f, " bwc=0x{:x}", self.bwc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdBiterElinkno(pub u16);
impl TcdBiterElinkno {
  #[inline] pub fn biter(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x7fff // [14:0]
  }
  #[inline] pub fn set_biter(mut self, value: u16) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn elink(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline] pub fn set_elink(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for TcdBiterElinkno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdBiterElinkno {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.biter() != 0 { try!(write!(f, " biter=0x{:x}", self.biter()))}
      if self.elink() != 0 { try!(write!(f, " elink"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct TcdBiterElinkyes(pub u16);
impl TcdBiterElinkyes {
  #[inline] pub fn biter(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1ff // [8:0]
  }
  #[inline] pub fn set_biter(mut self, value: u16) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn linkch(&self) -> u16 {
     ((self.0 as u16) >> 9) & 0xf // [12:9]
  }
  #[inline] pub fn set_linkch(mut self, value: u16) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 9);
     self.0 |= value << 9;
     self
  }

  #[inline] pub fn elink(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
  #[inline] pub fn set_elink(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for TcdBiterElinkyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for TcdBiterElinkyes {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.biter() != 0 { try!(write!(f, " biter=0x{:x}", self.biter()))}
      if self.linkch() != 0 { try!(write!(f, " linkch=0x{:x}", self.linkch()))}
      if self.elink() != 0 { try!(write!(f, " elink"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

