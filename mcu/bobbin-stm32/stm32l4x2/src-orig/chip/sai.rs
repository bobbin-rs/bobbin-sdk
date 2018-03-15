#[allow(unused_imports)] use bobbin_common::*;

periph!( SAI1, Sai1, _SAI1, SaiPeriph, 0x40015400);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SAI Peripheral"]
pub struct SaiPeriph(pub usize); 



impl SaiPeriph {
    #[doc="Get the *mut pointer for the BCR1 register."]
    #[inline] pub fn bcr1_mut(&self) -> *mut Bcr1 { 
        (self.0 + 0x24) as *mut Bcr1
    }

    #[doc="Get the *const pointer for the BCR1 register."]
    #[inline] pub fn bcr1_ptr(&self) -> *const Bcr1 { 
           self.bcr1_mut()
    }

    #[doc="Read the BCR1 register."]
    #[inline] pub fn bcr1(&self) -> Bcr1 { 
        unsafe {
            read_volatile(self.bcr1_ptr())
        }
    }

    #[doc="Write the BCR1 register."]
    #[inline] pub fn set_bcr1<F: FnOnce(Bcr1) -> Bcr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bcr1_mut(), f(Bcr1(0)));
        }
        self
    }

    #[doc="Modify the BCR1 register."]
    #[inline] pub fn with_bcr1<F: FnOnce(Bcr1) -> Bcr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bcr1_mut(), f(self.bcr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BCR2 register."]
    #[inline] pub fn bcr2_mut(&self) -> *mut Bcr2 { 
        (self.0 + 0x28) as *mut Bcr2
    }

    #[doc="Get the *const pointer for the BCR2 register."]
    #[inline] pub fn bcr2_ptr(&self) -> *const Bcr2 { 
           self.bcr2_mut()
    }

    #[doc="Read the BCR2 register."]
    #[inline] pub fn bcr2(&self) -> Bcr2 { 
        unsafe {
            read_volatile(self.bcr2_ptr())
        }
    }

    #[doc="Write the BCR2 register."]
    #[inline] pub fn set_bcr2<F: FnOnce(Bcr2) -> Bcr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bcr2_mut(), f(Bcr2(0)));
        }
        self
    }

    #[doc="Modify the BCR2 register."]
    #[inline] pub fn with_bcr2<F: FnOnce(Bcr2) -> Bcr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bcr2_mut(), f(self.bcr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BFRCR register."]
    #[inline] pub fn bfrcr_mut(&self) -> *mut Bfrcr { 
        (self.0 + 0x2c) as *mut Bfrcr
    }

    #[doc="Get the *const pointer for the BFRCR register."]
    #[inline] pub fn bfrcr_ptr(&self) -> *const Bfrcr { 
           self.bfrcr_mut()
    }

    #[doc="Read the BFRCR register."]
    #[inline] pub fn bfrcr(&self) -> Bfrcr { 
        unsafe {
            read_volatile(self.bfrcr_ptr())
        }
    }

    #[doc="Write the BFRCR register."]
    #[inline] pub fn set_bfrcr<F: FnOnce(Bfrcr) -> Bfrcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bfrcr_mut(), f(Bfrcr(0)));
        }
        self
    }

    #[doc="Modify the BFRCR register."]
    #[inline] pub fn with_bfrcr<F: FnOnce(Bfrcr) -> Bfrcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bfrcr_mut(), f(self.bfrcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BSLOTR register."]
    #[inline] pub fn bslotr_mut(&self) -> *mut Bslotr { 
        (self.0 + 0x30) as *mut Bslotr
    }

    #[doc="Get the *const pointer for the BSLOTR register."]
    #[inline] pub fn bslotr_ptr(&self) -> *const Bslotr { 
           self.bslotr_mut()
    }

    #[doc="Read the BSLOTR register."]
    #[inline] pub fn bslotr(&self) -> Bslotr { 
        unsafe {
            read_volatile(self.bslotr_ptr())
        }
    }

    #[doc="Write the BSLOTR register."]
    #[inline] pub fn set_bslotr<F: FnOnce(Bslotr) -> Bslotr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bslotr_mut(), f(Bslotr(0)));
        }
        self
    }

    #[doc="Modify the BSLOTR register."]
    #[inline] pub fn with_bslotr<F: FnOnce(Bslotr) -> Bslotr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bslotr_mut(), f(self.bslotr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BIM register."]
    #[inline] pub fn bim_mut(&self) -> *mut Bim { 
        (self.0 + 0x34) as *mut Bim
    }

    #[doc="Get the *const pointer for the BIM register."]
    #[inline] pub fn bim_ptr(&self) -> *const Bim { 
           self.bim_mut()
    }

    #[doc="Read the BIM register."]
    #[inline] pub fn bim(&self) -> Bim { 
        unsafe {
            read_volatile(self.bim_ptr())
        }
    }

    #[doc="Write the BIM register."]
    #[inline] pub fn set_bim<F: FnOnce(Bim) -> Bim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bim_mut(), f(Bim(0)));
        }
        self
    }

    #[doc="Modify the BIM register."]
    #[inline] pub fn with_bim<F: FnOnce(Bim) -> Bim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bim_mut(), f(self.bim()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BSR register."]
    #[inline] pub fn bsr_mut(&self) -> *mut Bsr { 
        (self.0 + 0x38) as *mut Bsr
    }

    #[doc="Get the *const pointer for the BSR register."]
    #[inline] pub fn bsr_ptr(&self) -> *const Bsr { 
           self.bsr_mut()
    }

    #[doc="Read the BSR register."]
    #[inline] pub fn bsr(&self) -> Bsr { 
        unsafe {
            read_volatile(self.bsr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the BCLRFR register."]
    #[inline] pub fn bclrfr_mut(&self) -> *mut Bclrfr { 
        (self.0 + 0x3c) as *mut Bclrfr
    }

    #[doc="Get the *const pointer for the BCLRFR register."]
    #[inline] pub fn bclrfr_ptr(&self) -> *const Bclrfr { 
           self.bclrfr_mut()
    }

    #[doc="Write the BCLRFR register."]
    #[inline] pub fn set_bclrfr<F: FnOnce(Bclrfr) -> Bclrfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bclrfr_mut(), f(Bclrfr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the BDR register."]
    #[inline] pub fn bdr_mut(&self) -> *mut Bdr { 
        (self.0 + 0x40) as *mut Bdr
    }

    #[doc="Get the *const pointer for the BDR register."]
    #[inline] pub fn bdr_ptr(&self) -> *const Bdr { 
           self.bdr_mut()
    }

    #[doc="Read the BDR register."]
    #[inline] pub fn bdr(&self) -> Bdr { 
        unsafe {
            read_volatile(self.bdr_ptr())
        }
    }

    #[doc="Write the BDR register."]
    #[inline] pub fn set_bdr<F: FnOnce(Bdr) -> Bdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bdr_mut(), f(Bdr(0)));
        }
        self
    }

    #[doc="Modify the BDR register."]
    #[inline] pub fn with_bdr<F: FnOnce(Bdr) -> Bdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bdr_mut(), f(self.bdr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ACR1 register."]
    #[inline] pub fn acr1_mut(&self) -> *mut Acr1 { 
        (self.0 + 0x4) as *mut Acr1
    }

    #[doc="Get the *const pointer for the ACR1 register."]
    #[inline] pub fn acr1_ptr(&self) -> *const Acr1 { 
           self.acr1_mut()
    }

    #[doc="Read the ACR1 register."]
    #[inline] pub fn acr1(&self) -> Acr1 { 
        unsafe {
            read_volatile(self.acr1_ptr())
        }
    }

    #[doc="Write the ACR1 register."]
    #[inline] pub fn set_acr1<F: FnOnce(Acr1) -> Acr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.acr1_mut(), f(Acr1(0)));
        }
        self
    }

    #[doc="Modify the ACR1 register."]
    #[inline] pub fn with_acr1<F: FnOnce(Acr1) -> Acr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.acr1_mut(), f(self.acr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ACR2 register."]
    #[inline] pub fn acr2_mut(&self) -> *mut Acr2 { 
        (self.0 + 0x8) as *mut Acr2
    }

    #[doc="Get the *const pointer for the ACR2 register."]
    #[inline] pub fn acr2_ptr(&self) -> *const Acr2 { 
           self.acr2_mut()
    }

    #[doc="Read the ACR2 register."]
    #[inline] pub fn acr2(&self) -> Acr2 { 
        unsafe {
            read_volatile(self.acr2_ptr())
        }
    }

    #[doc="Write the ACR2 register."]
    #[inline] pub fn set_acr2<F: FnOnce(Acr2) -> Acr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.acr2_mut(), f(Acr2(0)));
        }
        self
    }

    #[doc="Modify the ACR2 register."]
    #[inline] pub fn with_acr2<F: FnOnce(Acr2) -> Acr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.acr2_mut(), f(self.acr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AFRCR register."]
    #[inline] pub fn afrcr_mut(&self) -> *mut Afrcr { 
        (self.0 + 0xc) as *mut Afrcr
    }

    #[doc="Get the *const pointer for the AFRCR register."]
    #[inline] pub fn afrcr_ptr(&self) -> *const Afrcr { 
           self.afrcr_mut()
    }

    #[doc="Read the AFRCR register."]
    #[inline] pub fn afrcr(&self) -> Afrcr { 
        unsafe {
            read_volatile(self.afrcr_ptr())
        }
    }

    #[doc="Write the AFRCR register."]
    #[inline] pub fn set_afrcr<F: FnOnce(Afrcr) -> Afrcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.afrcr_mut(), f(Afrcr(0)));
        }
        self
    }

    #[doc="Modify the AFRCR register."]
    #[inline] pub fn with_afrcr<F: FnOnce(Afrcr) -> Afrcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.afrcr_mut(), f(self.afrcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ASLOTR register."]
    #[inline] pub fn aslotr_mut(&self) -> *mut Aslotr { 
        (self.0 + 0x10) as *mut Aslotr
    }

    #[doc="Get the *const pointer for the ASLOTR register."]
    #[inline] pub fn aslotr_ptr(&self) -> *const Aslotr { 
           self.aslotr_mut()
    }

    #[doc="Read the ASLOTR register."]
    #[inline] pub fn aslotr(&self) -> Aslotr { 
        unsafe {
            read_volatile(self.aslotr_ptr())
        }
    }

    #[doc="Write the ASLOTR register."]
    #[inline] pub fn set_aslotr<F: FnOnce(Aslotr) -> Aslotr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.aslotr_mut(), f(Aslotr(0)));
        }
        self
    }

    #[doc="Modify the ASLOTR register."]
    #[inline] pub fn with_aslotr<F: FnOnce(Aslotr) -> Aslotr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.aslotr_mut(), f(self.aslotr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AIM register."]
    #[inline] pub fn aim_mut(&self) -> *mut Aim { 
        (self.0 + 0x14) as *mut Aim
    }

    #[doc="Get the *const pointer for the AIM register."]
    #[inline] pub fn aim_ptr(&self) -> *const Aim { 
           self.aim_mut()
    }

    #[doc="Read the AIM register."]
    #[inline] pub fn aim(&self) -> Aim { 
        unsafe {
            read_volatile(self.aim_ptr())
        }
    }

    #[doc="Write the AIM register."]
    #[inline] pub fn set_aim<F: FnOnce(Aim) -> Aim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.aim_mut(), f(Aim(0)));
        }
        self
    }

    #[doc="Modify the AIM register."]
    #[inline] pub fn with_aim<F: FnOnce(Aim) -> Aim>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.aim_mut(), f(self.aim()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ASR register."]
    #[inline] pub fn asr_mut(&self) -> *mut Asr { 
        (self.0 + 0x18) as *mut Asr
    }

    #[doc="Get the *const pointer for the ASR register."]
    #[inline] pub fn asr_ptr(&self) -> *const Asr { 
           self.asr_mut()
    }

    #[doc="Read the ASR register."]
    #[inline] pub fn asr(&self) -> Asr { 
        unsafe {
            read_volatile(self.asr_ptr())
        }
    }

    #[doc="Write the ASR register."]
    #[inline] pub fn set_asr<F: FnOnce(Asr) -> Asr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.asr_mut(), f(Asr(0)));
        }
        self
    }

    #[doc="Modify the ASR register."]
    #[inline] pub fn with_asr<F: FnOnce(Asr) -> Asr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.asr_mut(), f(self.asr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ACLRFR register."]
    #[inline] pub fn aclrfr_mut(&self) -> *mut Aclrfr { 
        (self.0 + 0x1c) as *mut Aclrfr
    }

    #[doc="Get the *const pointer for the ACLRFR register."]
    #[inline] pub fn aclrfr_ptr(&self) -> *const Aclrfr { 
           self.aclrfr_mut()
    }

    #[doc="Read the ACLRFR register."]
    #[inline] pub fn aclrfr(&self) -> Aclrfr { 
        unsafe {
            read_volatile(self.aclrfr_ptr())
        }
    }

    #[doc="Write the ACLRFR register."]
    #[inline] pub fn set_aclrfr<F: FnOnce(Aclrfr) -> Aclrfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.aclrfr_mut(), f(Aclrfr(0)));
        }
        self
    }

    #[doc="Modify the ACLRFR register."]
    #[inline] pub fn with_aclrfr<F: FnOnce(Aclrfr) -> Aclrfr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.aclrfr_mut(), f(self.aclrfr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADR register."]
    #[inline] pub fn adr_mut(&self) -> *mut Adr { 
        (self.0 + 0x20) as *mut Adr
    }

    #[doc="Get the *const pointer for the ADR register."]
    #[inline] pub fn adr_ptr(&self) -> *const Adr { 
           self.adr_mut()
    }

    #[doc="Read the ADR register."]
    #[inline] pub fn adr(&self) -> Adr { 
        unsafe {
            read_volatile(self.adr_ptr())
        }
    }

    #[doc="Write the ADR register."]
    #[inline] pub fn set_adr<F: FnOnce(Adr) -> Adr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adr_mut(), f(Adr(0)));
        }
        self
    }

    #[doc="Modify the ADR register."]
    #[inline] pub fn with_adr<F: FnOnce(Adr) -> Adr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adr_mut(), f(self.adr()));
        }
        self
    }

}

#[doc="BConfiguration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcr1(pub u32);
impl Bcr1 {
    #[doc="Master clock divider"]
    #[inline] pub fn mcjdiv(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if MCJDIV != 0"]
    #[inline] pub fn test_mcjdiv(&self) -> bool {
        self.mcjdiv() != 0
    }

    #[doc="Sets the MCJDIV field."]
    #[inline] pub fn set_mcjdiv<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="No divider"]
    #[inline] pub fn nodiv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if NODIV != 0"]
    #[inline] pub fn test_nodiv(&self) -> bool {
        self.nodiv() != 0
    }

    #[doc="Sets the NODIV field."]
    #[inline] pub fn set_nodiv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DMA enable"]
    #[inline] pub fn dmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Audio block B enable"]
    #[inline] pub fn saiben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SAIBEN != 0"]
    #[inline] pub fn test_saiben(&self) -> bool {
        self.saiben() != 0
    }

    #[doc="Sets the SAIBEN field."]
    #[inline] pub fn set_saiben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Output drive"]
    #[inline] pub fn outdri(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if OUTDRI != 0"]
    #[inline] pub fn test_outdri(&self) -> bool {
        self.outdri() != 0
    }

    #[doc="Sets the OUTDRI field."]
    #[inline] pub fn set_outdri<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Mono mode"]
    #[inline] pub fn mono(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if MONO != 0"]
    #[inline] pub fn test_mono(&self) -> bool {
        self.mono() != 0
    }

    #[doc="Sets the MONO field."]
    #[inline] pub fn set_mono<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn syncen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if SYNCEN != 0"]
    #[inline] pub fn test_syncen(&self) -> bool {
        self.syncen() != 0
    }

    #[doc="Sets the SYNCEN field."]
    #[inline] pub fn set_syncen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clock strobing edge"]
    #[inline] pub fn ckstr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CKSTR != 0"]
    #[inline] pub fn test_ckstr(&self) -> bool {
        self.ckstr() != 0
    }

    #[doc="Sets the CKSTR field."]
    #[inline] pub fn set_ckstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Least significant bit first"]
    #[inline] pub fn lsbfirst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LSBFIRST != 0"]
    #[inline] pub fn test_lsbfirst(&self) -> bool {
        self.lsbfirst() != 0
    }

    #[doc="Sets the LSBFIRST field."]
    #[inline] pub fn set_lsbfirst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data size"]
    #[inline] pub fn ds(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if DS != 0"]
    #[inline] pub fn test_ds(&self) -> bool {
        self.ds() != 0
    }

    #[doc="Sets the DS field."]
    #[inline] pub fn set_ds<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Protocol configuration"]
    #[inline] pub fn prtcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if PRTCFG != 0"]
    #[inline] pub fn test_prtcfg(&self) -> bool {
        self.prtcfg() != 0
    }

    #[doc="Sets the PRTCFG field."]
    #[inline] pub fn set_prtcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Audio block mode"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bcr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Bcr1(other)
    }
}

impl ::core::fmt::Display for Bcr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bcr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcjdiv() != 0 { try!(write!(f, " mcjdiv=0x{:x}", self.mcjdiv()))}
        if self.nodiv() != 0 { try!(write!(f, " nodiv"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.saiben() != 0 { try!(write!(f, " saiben"))}
        if self.outdri() != 0 { try!(write!(f, " outdri"))}
        if self.mono() != 0 { try!(write!(f, " mono"))}
        if self.syncen() != 0 { try!(write!(f, " syncen=0x{:x}", self.syncen()))}
        if self.ckstr() != 0 { try!(write!(f, " ckstr"))}
        if self.lsbfirst() != 0 { try!(write!(f, " lsbfirst"))}
        if self.ds() != 0 { try!(write!(f, " ds=0x{:x}", self.ds()))}
        if self.prtcfg() != 0 { try!(write!(f, " prtcfg=0x{:x}", self.prtcfg()))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BConfiguration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bcr2(pub u32);
impl Bcr2 {
    #[doc="Companding mode"]
    #[inline] pub fn comp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if COMP != 0"]
    #[inline] pub fn test_comp(&self) -> bool {
        self.comp() != 0
    }

    #[doc="Sets the COMP field."]
    #[inline] pub fn set_comp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Complement bit"]
    #[inline] pub fn cpl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CPL != 0"]
    #[inline] pub fn test_cpl(&self) -> bool {
        self.cpl() != 0
    }

    #[doc="Sets the CPL field."]
    #[inline] pub fn set_cpl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Mute counter"]
    #[inline] pub fn mutecn(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x3f) as u8) } // [12:7]
    }

    #[doc="Returns true if MUTECN != 0"]
    #[inline] pub fn test_mutecn(&self) -> bool {
        self.mutecn() != 0
    }

    #[doc="Sets the MUTECN field."]
    #[inline] pub fn set_mutecn<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Mute value"]
    #[inline] pub fn muteval(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if MUTEVAL != 0"]
    #[inline] pub fn test_muteval(&self) -> bool {
        self.muteval() != 0
    }

    #[doc="Sets the MUTEVAL field."]
    #[inline] pub fn set_muteval<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Mute"]
    #[inline] pub fn mute(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MUTE != 0"]
    #[inline] pub fn test_mute(&self) -> bool {
        self.mute() != 0
    }

    #[doc="Sets the MUTE field."]
    #[inline] pub fn set_mute<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Tristate management on data line"]
    #[inline] pub fn tris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TRIS != 0"]
    #[inline] pub fn test_tris(&self) -> bool {
        self.tris() != 0
    }

    #[doc="Sets the TRIS field."]
    #[inline] pub fn set_tris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO flush"]
    #[inline] pub fn fflus(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FFLUS != 0"]
    #[inline] pub fn test_fflus(&self) -> bool {
        self.fflus() != 0
    }

    #[doc="Sets the FFLUS field."]
    #[inline] pub fn set_fflus<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO threshold"]
    #[inline] pub fn fth(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if FTH != 0"]
    #[inline] pub fn test_fth(&self) -> bool {
        self.fth() != 0
    }

    #[doc="Sets the FTH field."]
    #[inline] pub fn set_fth<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bcr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Bcr2(other)
    }
}

impl ::core::fmt::Display for Bcr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bcr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.comp() != 0 { try!(write!(f, " comp=0x{:x}", self.comp()))}
        if self.cpl() != 0 { try!(write!(f, " cpl"))}
        if self.mutecn() != 0 { try!(write!(f, " mutecn=0x{:x}", self.mutecn()))}
        if self.muteval() != 0 { try!(write!(f, " muteval"))}
        if self.mute() != 0 { try!(write!(f, " mute"))}
        if self.tris() != 0 { try!(write!(f, " tris"))}
        if self.fflus() != 0 { try!(write!(f, " fflus"))}
        if self.fth() != 0 { try!(write!(f, " fth=0x{:x}", self.fth()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BFRCR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bfrcr(pub u32);
impl Bfrcr {
    #[doc="Frame synchronization offset"]
    #[inline] pub fn fsoff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FSOFF != 0"]
    #[inline] pub fn test_fsoff(&self) -> bool {
        self.fsoff() != 0
    }

    #[doc="Sets the FSOFF field."]
    #[inline] pub fn set_fsoff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Frame synchronization polarity"]
    #[inline] pub fn fspol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if FSPOL != 0"]
    #[inline] pub fn test_fspol(&self) -> bool {
        self.fspol() != 0
    }

    #[doc="Sets the FSPOL field."]
    #[inline] pub fn set_fspol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Frame synchronization definition"]
    #[inline] pub fn fsdef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if FSDEF != 0"]
    #[inline] pub fn test_fsdef(&self) -> bool {
        self.fsdef() != 0
    }

    #[doc="Sets the FSDEF field."]
    #[inline] pub fn set_fsdef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Frame synchronization active level length"]
    #[inline] pub fn fsall(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if FSALL != 0"]
    #[inline] pub fn test_fsall(&self) -> bool {
        self.fsall() != 0
    }

    #[doc="Sets the FSALL field."]
    #[inline] pub fn set_fsall<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame length"]
    #[inline] pub fn frl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FRL != 0"]
    #[inline] pub fn test_frl(&self) -> bool {
        self.frl() != 0
    }

    #[doc="Sets the FRL field."]
    #[inline] pub fn set_frl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bfrcr {
    #[inline]
    fn from(other: u32) -> Self {
         Bfrcr(other)
    }
}

impl ::core::fmt::Display for Bfrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bfrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsoff() != 0 { try!(write!(f, " fsoff"))}
        if self.fspol() != 0 { try!(write!(f, " fspol"))}
        if self.fsdef() != 0 { try!(write!(f, " fsdef"))}
        if self.fsall() != 0 { try!(write!(f, " fsall=0x{:x}", self.fsall()))}
        if self.frl() != 0 { try!(write!(f, " frl=0x{:x}", self.frl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BSlot register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bslotr(pub u32);
impl Bslotr {
    #[doc="Slot enable"]
    #[inline] pub fn sloten(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if SLOTEN != 0"]
    #[inline] pub fn test_sloten(&self) -> bool {
        self.sloten() != 0
    }

    #[doc="Sets the SLOTEN field."]
    #[inline] pub fn set_sloten<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Number of slots in an audio frame"]
    #[inline] pub fn nbslot(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if NBSLOT != 0"]
    #[inline] pub fn test_nbslot(&self) -> bool {
        self.nbslot() != 0
    }

    #[doc="Sets the NBSLOT field."]
    #[inline] pub fn set_nbslot<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Slot size"]
    #[inline] pub fn slotsz(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if SLOTSZ != 0"]
    #[inline] pub fn test_slotsz(&self) -> bool {
        self.slotsz() != 0
    }

    #[doc="Sets the SLOTSZ field."]
    #[inline] pub fn set_slotsz<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="First bit offset"]
    #[inline] pub fn fboff(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if FBOFF != 0"]
    #[inline] pub fn test_fboff(&self) -> bool {
        self.fboff() != 0
    }

    #[doc="Sets the FBOFF field."]
    #[inline] pub fn set_fboff<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bslotr {
    #[inline]
    fn from(other: u32) -> Self {
         Bslotr(other)
    }
}

impl ::core::fmt::Display for Bslotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bslotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sloten() != 0 { try!(write!(f, " sloten=0x{:x}", self.sloten()))}
        if self.nbslot() != 0 { try!(write!(f, " nbslot=0x{:x}", self.nbslot()))}
        if self.slotsz() != 0 { try!(write!(f, " slotsz=0x{:x}", self.slotsz()))}
        if self.fboff() != 0 { try!(write!(f, " fboff=0x{:x}", self.fboff()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BInterrupt mask register2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bim(pub u32);
impl Bim {
    #[doc="Late frame synchronization detection interrupt enable"]
    #[inline] pub fn lfsdetie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDETIE != 0"]
    #[inline] pub fn test_lfsdetie(&self) -> bool {
        self.lfsdetie() != 0
    }

    #[doc="Sets the LFSDETIE field."]
    #[inline] pub fn set_lfsdetie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Anticipated frame synchronization detection interrupt enable"]
    #[inline] pub fn afsdetie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AFSDETIE != 0"]
    #[inline] pub fn test_afsdetie(&self) -> bool {
        self.afsdetie() != 0
    }

    #[doc="Sets the AFSDETIE field."]
    #[inline] pub fn set_afsdetie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Codec not ready interrupt enable"]
    #[inline] pub fn cnrdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDYIE != 0"]
    #[inline] pub fn test_cnrdyie(&self) -> bool {
        self.cnrdyie() != 0
    }

    #[doc="Sets the CNRDYIE field."]
    #[inline] pub fn set_cnrdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO request interrupt enable"]
    #[inline] pub fn freqie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FREQIE != 0"]
    #[inline] pub fn test_freqie(&self) -> bool {
        self.freqie() != 0
    }

    #[doc="Sets the FREQIE field."]
    #[inline] pub fn set_freqie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wrong clock configuration interrupt enable"]
    #[inline] pub fn wckcfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection interrupt enable"]
    #[inline] pub fn mutedet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Overrun/underrun interrupt enable"]
    #[inline] pub fn ovrudrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDRIE != 0"]
    #[inline] pub fn test_ovrudrie(&self) -> bool {
        self.ovrudrie() != 0
    }

    #[doc="Sets the OVRUDRIE field."]
    #[inline] pub fn set_ovrudrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bim {
    #[inline]
    fn from(other: u32) -> Self {
         Bim(other)
    }
}

impl ::core::fmt::Display for Bim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfsdetie() != 0 { try!(write!(f, " lfsdetie"))}
        if self.afsdetie() != 0 { try!(write!(f, " afsdetie"))}
        if self.cnrdyie() != 0 { try!(write!(f, " cnrdyie"))}
        if self.freqie() != 0 { try!(write!(f, " freqie"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudrie() != 0 { try!(write!(f, " ovrudrie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BStatus register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bsr(pub u32);
impl Bsr {
    #[doc="FIFO level threshold"]
    #[inline] pub fn flvl(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if FLVL != 0"]
    #[inline] pub fn test_flvl(&self) -> bool {
        self.flvl() != 0
    }

    #[doc="Sets the FLVL field."]
    #[inline] pub fn set_flvl<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Late frame synchronization detection"]
    #[inline] pub fn lfsdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDET != 0"]
    #[inline] pub fn test_lfsdet(&self) -> bool {
        self.lfsdet() != 0
    }

    #[doc="Sets the LFSDET field."]
    #[inline] pub fn set_lfsdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Anticipated frame synchronization detection"]
    #[inline] pub fn afsdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AFSDET != 0"]
    #[inline] pub fn test_afsdet(&self) -> bool {
        self.afsdet() != 0
    }

    #[doc="Sets the AFSDET field."]
    #[inline] pub fn set_afsdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Codec not ready"]
    #[inline] pub fn cnrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDY != 0"]
    #[inline] pub fn test_cnrdy(&self) -> bool {
        self.cnrdy() != 0
    }

    #[doc="Sets the CNRDY field."]
    #[inline] pub fn set_cnrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO request"]
    #[inline] pub fn freq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FREQ != 0"]
    #[inline] pub fn test_freq(&self) -> bool {
        self.freq() != 0
    }

    #[doc="Sets the FREQ field."]
    #[inline] pub fn set_freq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wrong clock configuration flag"]
    #[inline] pub fn wckcfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection"]
    #[inline] pub fn mutedet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Overrun / underrun"]
    #[inline] pub fn ovrudr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDR != 0"]
    #[inline] pub fn test_ovrudr(&self) -> bool {
        self.ovrudr() != 0
    }

    #[doc="Sets the OVRUDR field."]
    #[inline] pub fn set_ovrudr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bsr {
    #[inline]
    fn from(other: u32) -> Self {
         Bsr(other)
    }
}

impl ::core::fmt::Display for Bsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flvl() != 0 { try!(write!(f, " flvl=0x{:x}", self.flvl()))}
        if self.lfsdet() != 0 { try!(write!(f, " lfsdet"))}
        if self.afsdet() != 0 { try!(write!(f, " afsdet"))}
        if self.cnrdy() != 0 { try!(write!(f, " cnrdy"))}
        if self.freq() != 0 { try!(write!(f, " freq"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudr() != 0 { try!(write!(f, " ovrudr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BClear flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bclrfr(pub u32);
impl Bclrfr {
    #[doc="Clear late frame synchronization detection flag"]
    #[inline] pub fn lfsdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDET != 0"]
    #[inline] pub fn test_lfsdet(&self) -> bool {
        self.lfsdet() != 0
    }

    #[doc="Sets the LFSDET field."]
    #[inline] pub fn set_lfsdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clear anticipated frame synchronization detection flag"]
    #[inline] pub fn cafsdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CAFSDET != 0"]
    #[inline] pub fn test_cafsdet(&self) -> bool {
        self.cafsdet() != 0
    }

    #[doc="Sets the CAFSDET field."]
    #[inline] pub fn set_cafsdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clear codec not ready flag"]
    #[inline] pub fn cnrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDY != 0"]
    #[inline] pub fn test_cnrdy(&self) -> bool {
        self.cnrdy() != 0
    }

    #[doc="Sets the CNRDY field."]
    #[inline] pub fn set_cnrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear wrong clock configuration flag"]
    #[inline] pub fn wckcfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection flag"]
    #[inline] pub fn mutedet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear overrun / underrun"]
    #[inline] pub fn ovrudr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDR != 0"]
    #[inline] pub fn test_ovrudr(&self) -> bool {
        self.ovrudr() != 0
    }

    #[doc="Sets the OVRUDR field."]
    #[inline] pub fn set_ovrudr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bclrfr {
    #[inline]
    fn from(other: u32) -> Self {
         Bclrfr(other)
    }
}

impl ::core::fmt::Display for Bclrfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bclrfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfsdet() != 0 { try!(write!(f, " lfsdet"))}
        if self.cafsdet() != 0 { try!(write!(f, " cafsdet"))}
        if self.cnrdy() != 0 { try!(write!(f, " cnrdy"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudr() != 0 { try!(write!(f, " ovrudr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BData register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdr(pub u32);
impl Bdr {
    #[doc="Data"]
    #[inline] pub fn data(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Bdr {
    #[inline]
    fn from(other: u32) -> Self {
         Bdr(other)
    }
}

impl ::core::fmt::Display for Bdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AConfiguration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr1(pub u32);
impl Acr1 {
    #[doc="Master clock divider"]
    #[inline] pub fn mcjdiv(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if MCJDIV != 0"]
    #[inline] pub fn test_mcjdiv(&self) -> bool {
        self.mcjdiv() != 0
    }

    #[doc="Sets the MCJDIV field."]
    #[inline] pub fn set_mcjdiv<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="No divider"]
    #[inline] pub fn nodiv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if NODIV != 0"]
    #[inline] pub fn test_nodiv(&self) -> bool {
        self.nodiv() != 0
    }

    #[doc="Sets the NODIV field."]
    #[inline] pub fn set_nodiv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DMA enable"]
    #[inline] pub fn dmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Audio block A enable"]
    #[inline] pub fn saiaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if SAIAEN != 0"]
    #[inline] pub fn test_saiaen(&self) -> bool {
        self.saiaen() != 0
    }

    #[doc="Sets the SAIAEN field."]
    #[inline] pub fn set_saiaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Output drive"]
    #[inline] pub fn outdri(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if OUTDRI != 0"]
    #[inline] pub fn test_outdri(&self) -> bool {
        self.outdri() != 0
    }

    #[doc="Sets the OUTDRI field."]
    #[inline] pub fn set_outdri<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Mono mode"]
    #[inline] pub fn mono(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if MONO != 0"]
    #[inline] pub fn test_mono(&self) -> bool {
        self.mono() != 0
    }

    #[doc="Sets the MONO field."]
    #[inline] pub fn set_mono<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Synchronization enable"]
    #[inline] pub fn syncen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if SYNCEN != 0"]
    #[inline] pub fn test_syncen(&self) -> bool {
        self.syncen() != 0
    }

    #[doc="Sets the SYNCEN field."]
    #[inline] pub fn set_syncen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clock strobing edge"]
    #[inline] pub fn ckstr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CKSTR != 0"]
    #[inline] pub fn test_ckstr(&self) -> bool {
        self.ckstr() != 0
    }

    #[doc="Sets the CKSTR field."]
    #[inline] pub fn set_ckstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Least significant bit first"]
    #[inline] pub fn lsbfirst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LSBFIRST != 0"]
    #[inline] pub fn test_lsbfirst(&self) -> bool {
        self.lsbfirst() != 0
    }

    #[doc="Sets the LSBFIRST field."]
    #[inline] pub fn set_lsbfirst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Data size"]
    #[inline] pub fn ds(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if DS != 0"]
    #[inline] pub fn test_ds(&self) -> bool {
        self.ds() != 0
    }

    #[doc="Sets the DS field."]
    #[inline] pub fn set_ds<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Protocol configuration"]
    #[inline] pub fn prtcfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if PRTCFG != 0"]
    #[inline] pub fn test_prtcfg(&self) -> bool {
        self.prtcfg() != 0
    }

    #[doc="Sets the PRTCFG field."]
    #[inline] pub fn set_prtcfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Audio block mode"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Acr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Acr1(other)
    }
}

impl ::core::fmt::Display for Acr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcjdiv() != 0 { try!(write!(f, " mcjdiv=0x{:x}", self.mcjdiv()))}
        if self.nodiv() != 0 { try!(write!(f, " nodiv"))}
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.saiaen() != 0 { try!(write!(f, " saiaen"))}
        if self.outdri() != 0 { try!(write!(f, " outdri"))}
        if self.mono() != 0 { try!(write!(f, " mono"))}
        if self.syncen() != 0 { try!(write!(f, " syncen=0x{:x}", self.syncen()))}
        if self.ckstr() != 0 { try!(write!(f, " ckstr"))}
        if self.lsbfirst() != 0 { try!(write!(f, " lsbfirst"))}
        if self.ds() != 0 { try!(write!(f, " ds=0x{:x}", self.ds()))}
        if self.prtcfg() != 0 { try!(write!(f, " prtcfg=0x{:x}", self.prtcfg()))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AConfiguration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr2(pub u32);
impl Acr2 {
    #[doc="Companding mode"]
    #[inline] pub fn comp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if COMP != 0"]
    #[inline] pub fn test_comp(&self) -> bool {
        self.comp() != 0
    }

    #[doc="Sets the COMP field."]
    #[inline] pub fn set_comp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Complement bit"]
    #[inline] pub fn cpl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CPL != 0"]
    #[inline] pub fn test_cpl(&self) -> bool {
        self.cpl() != 0
    }

    #[doc="Sets the CPL field."]
    #[inline] pub fn set_cpl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Mute counter"]
    #[inline] pub fn mutecn(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x3f) as u8) } // [12:7]
    }

    #[doc="Returns true if MUTECN != 0"]
    #[inline] pub fn test_mutecn(&self) -> bool {
        self.mutecn() != 0
    }

    #[doc="Sets the MUTECN field."]
    #[inline] pub fn set_mutecn<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Mute value"]
    #[inline] pub fn muteval(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if MUTEVAL != 0"]
    #[inline] pub fn test_muteval(&self) -> bool {
        self.muteval() != 0
    }

    #[doc="Sets the MUTEVAL field."]
    #[inline] pub fn set_muteval<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Mute"]
    #[inline] pub fn mute(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MUTE != 0"]
    #[inline] pub fn test_mute(&self) -> bool {
        self.mute() != 0
    }

    #[doc="Sets the MUTE field."]
    #[inline] pub fn set_mute<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Tristate management on data line"]
    #[inline] pub fn tris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TRIS != 0"]
    #[inline] pub fn test_tris(&self) -> bool {
        self.tris() != 0
    }

    #[doc="Sets the TRIS field."]
    #[inline] pub fn set_tris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO flush"]
    #[inline] pub fn fflus(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FFLUS != 0"]
    #[inline] pub fn test_fflus(&self) -> bool {
        self.fflus() != 0
    }

    #[doc="Sets the FFLUS field."]
    #[inline] pub fn set_fflus<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FIFO threshold"]
    #[inline] pub fn fth(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if FTH != 0"]
    #[inline] pub fn test_fth(&self) -> bool {
        self.fth() != 0
    }

    #[doc="Sets the FTH field."]
    #[inline] pub fn set_fth<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Acr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Acr2(other)
    }
}

impl ::core::fmt::Display for Acr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.comp() != 0 { try!(write!(f, " comp=0x{:x}", self.comp()))}
        if self.cpl() != 0 { try!(write!(f, " cpl"))}
        if self.mutecn() != 0 { try!(write!(f, " mutecn=0x{:x}", self.mutecn()))}
        if self.muteval() != 0 { try!(write!(f, " muteval"))}
        if self.mute() != 0 { try!(write!(f, " mute"))}
        if self.tris() != 0 { try!(write!(f, " tris"))}
        if self.fflus() != 0 { try!(write!(f, " fflus"))}
        if self.fth() != 0 { try!(write!(f, " fth=0x{:x}", self.fth()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AFRCR"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Afrcr(pub u32);
impl Afrcr {
    #[doc="Frame synchronization offset"]
    #[inline] pub fn fsoff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if FSOFF != 0"]
    #[inline] pub fn test_fsoff(&self) -> bool {
        self.fsoff() != 0
    }

    #[doc="Sets the FSOFF field."]
    #[inline] pub fn set_fsoff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Frame synchronization polarity"]
    #[inline] pub fn fspol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if FSPOL != 0"]
    #[inline] pub fn test_fspol(&self) -> bool {
        self.fspol() != 0
    }

    #[doc="Sets the FSPOL field."]
    #[inline] pub fn set_fspol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Frame synchronization definition"]
    #[inline] pub fn fsdef(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if FSDEF != 0"]
    #[inline] pub fn test_fsdef(&self) -> bool {
        self.fsdef() != 0
    }

    #[doc="Sets the FSDEF field."]
    #[inline] pub fn set_fsdef<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Frame synchronization active level length"]
    #[inline] pub fn fsall(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7f) as u8) } // [14:8]
    }

    #[doc="Returns true if FSALL != 0"]
    #[inline] pub fn test_fsall(&self) -> bool {
        self.fsall() != 0
    }

    #[doc="Sets the FSALL field."]
    #[inline] pub fn set_fsall<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame length"]
    #[inline] pub fn frl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if FRL != 0"]
    #[inline] pub fn test_frl(&self) -> bool {
        self.frl() != 0
    }

    #[doc="Sets the FRL field."]
    #[inline] pub fn set_frl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Afrcr {
    #[inline]
    fn from(other: u32) -> Self {
         Afrcr(other)
    }
}

impl ::core::fmt::Display for Afrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Afrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsoff() != 0 { try!(write!(f, " fsoff"))}
        if self.fspol() != 0 { try!(write!(f, " fspol"))}
        if self.fsdef() != 0 { try!(write!(f, " fsdef"))}
        if self.fsall() != 0 { try!(write!(f, " fsall=0x{:x}", self.fsall()))}
        if self.frl() != 0 { try!(write!(f, " frl=0x{:x}", self.frl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ASlot register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Aslotr(pub u32);
impl Aslotr {
    #[doc="Slot enable"]
    #[inline] pub fn sloten(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if SLOTEN != 0"]
    #[inline] pub fn test_sloten(&self) -> bool {
        self.sloten() != 0
    }

    #[doc="Sets the SLOTEN field."]
    #[inline] pub fn set_sloten<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Number of slots in an audio frame"]
    #[inline] pub fn nbslot(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if NBSLOT != 0"]
    #[inline] pub fn test_nbslot(&self) -> bool {
        self.nbslot() != 0
    }

    #[doc="Sets the NBSLOT field."]
    #[inline] pub fn set_nbslot<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Slot size"]
    #[inline] pub fn slotsz(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if SLOTSZ != 0"]
    #[inline] pub fn test_slotsz(&self) -> bool {
        self.slotsz() != 0
    }

    #[doc="Sets the SLOTSZ field."]
    #[inline] pub fn set_slotsz<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="First bit offset"]
    #[inline] pub fn fboff(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if FBOFF != 0"]
    #[inline] pub fn test_fboff(&self) -> bool {
        self.fboff() != 0
    }

    #[doc="Sets the FBOFF field."]
    #[inline] pub fn set_fboff<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Aslotr {
    #[inline]
    fn from(other: u32) -> Self {
         Aslotr(other)
    }
}

impl ::core::fmt::Display for Aslotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Aslotr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sloten() != 0 { try!(write!(f, " sloten=0x{:x}", self.sloten()))}
        if self.nbslot() != 0 { try!(write!(f, " nbslot=0x{:x}", self.nbslot()))}
        if self.slotsz() != 0 { try!(write!(f, " slotsz=0x{:x}", self.slotsz()))}
        if self.fboff() != 0 { try!(write!(f, " fboff=0x{:x}", self.fboff()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AInterrupt mask register2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Aim(pub u32);
impl Aim {
    #[doc="Late frame synchronization detection interrupt enable"]
    #[inline] pub fn lfsdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDET != 0"]
    #[inline] pub fn test_lfsdet(&self) -> bool {
        self.lfsdet() != 0
    }

    #[doc="Sets the LFSDET field."]
    #[inline] pub fn set_lfsdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Anticipated frame synchronization detection interrupt enable"]
    #[inline] pub fn afsdetie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AFSDETIE != 0"]
    #[inline] pub fn test_afsdetie(&self) -> bool {
        self.afsdetie() != 0
    }

    #[doc="Sets the AFSDETIE field."]
    #[inline] pub fn set_afsdetie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Codec not ready interrupt enable"]
    #[inline] pub fn cnrdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDYIE != 0"]
    #[inline] pub fn test_cnrdyie(&self) -> bool {
        self.cnrdyie() != 0
    }

    #[doc="Sets the CNRDYIE field."]
    #[inline] pub fn set_cnrdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO request interrupt enable"]
    #[inline] pub fn freqie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FREQIE != 0"]
    #[inline] pub fn test_freqie(&self) -> bool {
        self.freqie() != 0
    }

    #[doc="Sets the FREQIE field."]
    #[inline] pub fn set_freqie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wrong clock configuration interrupt enable"]
    #[inline] pub fn wckcfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection interrupt enable"]
    #[inline] pub fn mutedet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Overrun/underrun interrupt enable"]
    #[inline] pub fn ovrudrie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDRIE != 0"]
    #[inline] pub fn test_ovrudrie(&self) -> bool {
        self.ovrudrie() != 0
    }

    #[doc="Sets the OVRUDRIE field."]
    #[inline] pub fn set_ovrudrie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Aim {
    #[inline]
    fn from(other: u32) -> Self {
         Aim(other)
    }
}

impl ::core::fmt::Display for Aim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Aim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfsdet() != 0 { try!(write!(f, " lfsdet"))}
        if self.afsdetie() != 0 { try!(write!(f, " afsdetie"))}
        if self.cnrdyie() != 0 { try!(write!(f, " cnrdyie"))}
        if self.freqie() != 0 { try!(write!(f, " freqie"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudrie() != 0 { try!(write!(f, " ovrudrie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AStatus register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Asr(pub u32);
impl Asr {
    #[doc="FIFO level threshold"]
    #[inline] pub fn flvl(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if FLVL != 0"]
    #[inline] pub fn test_flvl(&self) -> bool {
        self.flvl() != 0
    }

    #[doc="Sets the FLVL field."]
    #[inline] pub fn set_flvl<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Late frame synchronization detection"]
    #[inline] pub fn lfsdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDET != 0"]
    #[inline] pub fn test_lfsdet(&self) -> bool {
        self.lfsdet() != 0
    }

    #[doc="Sets the LFSDET field."]
    #[inline] pub fn set_lfsdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Anticipated frame synchronization detection"]
    #[inline] pub fn afsdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AFSDET != 0"]
    #[inline] pub fn test_afsdet(&self) -> bool {
        self.afsdet() != 0
    }

    #[doc="Sets the AFSDET field."]
    #[inline] pub fn set_afsdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Codec not ready"]
    #[inline] pub fn cnrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDY != 0"]
    #[inline] pub fn test_cnrdy(&self) -> bool {
        self.cnrdy() != 0
    }

    #[doc="Sets the CNRDY field."]
    #[inline] pub fn set_cnrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO request"]
    #[inline] pub fn freq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FREQ != 0"]
    #[inline] pub fn test_freq(&self) -> bool {
        self.freq() != 0
    }

    #[doc="Sets the FREQ field."]
    #[inline] pub fn set_freq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Wrong clock configuration flag. This bit is read only"]
    #[inline] pub fn wckcfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection"]
    #[inline] pub fn mutedet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Overrun / underrun"]
    #[inline] pub fn ovrudr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDR != 0"]
    #[inline] pub fn test_ovrudr(&self) -> bool {
        self.ovrudr() != 0
    }

    #[doc="Sets the OVRUDR field."]
    #[inline] pub fn set_ovrudr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Asr {
    #[inline]
    fn from(other: u32) -> Self {
         Asr(other)
    }
}

impl ::core::fmt::Display for Asr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Asr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flvl() != 0 { try!(write!(f, " flvl=0x{:x}", self.flvl()))}
        if self.lfsdet() != 0 { try!(write!(f, " lfsdet"))}
        if self.afsdet() != 0 { try!(write!(f, " afsdet"))}
        if self.cnrdy() != 0 { try!(write!(f, " cnrdy"))}
        if self.freq() != 0 { try!(write!(f, " freq"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudr() != 0 { try!(write!(f, " ovrudr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AClear flag register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Aclrfr(pub u32);
impl Aclrfr {
    #[doc="Clear late frame synchronization detection flag"]
    #[inline] pub fn lfsdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LFSDET != 0"]
    #[inline] pub fn test_lfsdet(&self) -> bool {
        self.lfsdet() != 0
    }

    #[doc="Sets the LFSDET field."]
    #[inline] pub fn set_lfsdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clear anticipated frame synchronization detection flag"]
    #[inline] pub fn cafsdet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CAFSDET != 0"]
    #[inline] pub fn test_cafsdet(&self) -> bool {
        self.cafsdet() != 0
    }

    #[doc="Sets the CAFSDET field."]
    #[inline] pub fn set_cafsdet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Clear codec not ready flag"]
    #[inline] pub fn cnrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CNRDY != 0"]
    #[inline] pub fn test_cnrdy(&self) -> bool {
        self.cnrdy() != 0
    }

    #[doc="Sets the CNRDY field."]
    #[inline] pub fn set_cnrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clear wrong clock configuration flag"]
    #[inline] pub fn wckcfg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WCKCFG != 0"]
    #[inline] pub fn test_wckcfg(&self) -> bool {
        self.wckcfg() != 0
    }

    #[doc="Sets the WCKCFG field."]
    #[inline] pub fn set_wckcfg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Mute detection flag"]
    #[inline] pub fn mutedet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MUTEDET != 0"]
    #[inline] pub fn test_mutedet(&self) -> bool {
        self.mutedet() != 0
    }

    #[doc="Sets the MUTEDET field."]
    #[inline] pub fn set_mutedet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clear overrun / underrun"]
    #[inline] pub fn ovrudr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVRUDR != 0"]
    #[inline] pub fn test_ovrudr(&self) -> bool {
        self.ovrudr() != 0
    }

    #[doc="Sets the OVRUDR field."]
    #[inline] pub fn set_ovrudr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Aclrfr {
    #[inline]
    fn from(other: u32) -> Self {
         Aclrfr(other)
    }
}

impl ::core::fmt::Display for Aclrfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Aclrfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lfsdet() != 0 { try!(write!(f, " lfsdet"))}
        if self.cafsdet() != 0 { try!(write!(f, " cafsdet"))}
        if self.cnrdy() != 0 { try!(write!(f, " cnrdy"))}
        if self.wckcfg() != 0 { try!(write!(f, " wckcfg"))}
        if self.mutedet() != 0 { try!(write!(f, " mutedet"))}
        if self.ovrudr() != 0 { try!(write!(f, " ovrudr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AData register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adr(pub u32);
impl Adr {
    #[doc="Data"]
    #[inline] pub fn data(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Adr {
    #[inline]
    fn from(other: u32) -> Self {
         Adr(other)
    }
}

impl ::core::fmt::Display for Adr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}


