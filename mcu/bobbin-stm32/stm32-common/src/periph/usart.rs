
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USART Peripheral"]
pub struct UsartPeriph(pub usize); 

impl UsartPeriph {
    #[doc="Get the CR1 Register."]
    #[inline] pub fn cr1_reg(&self) -> Register<Cr1> { 
        Register::new(self.0 as *mut Cr1, 0x0)
    }

    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        self.cr1_reg().read()
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn write_cr1(&self, value: Cr1) -> &Self { 
        self.cr1_reg().write(value);
        self
    }

    #[doc="Set the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().set(f);
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().with(f);
        self
    }

    #[doc="Get the CR2 Register."]
    #[inline] pub fn cr2_reg(&self) -> Register<Cr2> { 
        Register::new(self.0 as *mut Cr2, 0x4)
    }

    #[doc="Get the *mut pointer for the CR2 register."]
    #[inline] pub fn cr2_mut(&self) -> *mut Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR2 register."]
    #[inline] pub fn cr2_ptr(&self) -> *const Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Read the CR2 register."]
    #[inline] pub fn cr2(&self) -> Cr2 { 
        self.cr2_reg().read()
    }

    #[doc="Write the CR2 register."]
    #[inline] pub fn write_cr2(&self, value: Cr2) -> &Self { 
        self.cr2_reg().write(value);
        self
    }

    #[doc="Set the CR2 register."]
    #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().set(f);
        self
    }

    #[doc="Modify the CR2 register."]
    #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().with(f);
        self
    }

    #[doc="Get the CR3 Register."]
    #[inline] pub fn cr3_reg(&self) -> Register<Cr3> { 
        Register::new(self.0 as *mut Cr3, 0x8)
    }

    #[doc="Get the *mut pointer for the CR3 register."]
    #[inline] pub fn cr3_mut(&self) -> *mut Cr3 { 
        self.cr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR3 register."]
    #[inline] pub fn cr3_ptr(&self) -> *const Cr3 { 
        self.cr3_reg().ptr()
    }

    #[doc="Read the CR3 register."]
    #[inline] pub fn cr3(&self) -> Cr3 { 
        self.cr3_reg().read()
    }

    #[doc="Write the CR3 register."]
    #[inline] pub fn write_cr3(&self, value: Cr3) -> &Self { 
        self.cr3_reg().write(value);
        self
    }

    #[doc="Set the CR3 register."]
    #[inline] pub fn set_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
        self.cr3_reg().set(f);
        self
    }

    #[doc="Modify the CR3 register."]
    #[inline] pub fn with_cr3<F: FnOnce(Cr3) -> Cr3>(&self, f: F) -> &Self {
        self.cr3_reg().with(f);
        self
    }

    #[doc="Get the BRR Register."]
    #[inline] pub fn brr_reg(&self) -> Register<Brr> { 
        Register::new(self.0 as *mut Brr, 0xc)
    }

    #[doc="Get the *mut pointer for the BRR register."]
    #[inline] pub fn brr_mut(&self) -> *mut Brr { 
        self.brr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BRR register."]
    #[inline] pub fn brr_ptr(&self) -> *const Brr { 
        self.brr_reg().ptr()
    }

    #[doc="Read the BRR register."]
    #[inline] pub fn brr(&self) -> Brr { 
        self.brr_reg().read()
    }

    #[doc="Write the BRR register."]
    #[inline] pub fn write_brr(&self, value: Brr) -> &Self { 
        self.brr_reg().write(value);
        self
    }

    #[doc="Set the BRR register."]
    #[inline] pub fn set_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
        self.brr_reg().set(f);
        self
    }

    #[doc="Modify the BRR register."]
    #[inline] pub fn with_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
        self.brr_reg().with(f);
        self
    }

    #[doc="Get the GTPR Register."]
    #[inline] pub fn gtpr_reg(&self) -> Register<Gtpr> { 
        Register::new(self.0 as *mut Gtpr, 0x10)
    }

    #[doc="Get the *mut pointer for the GTPR register."]
    #[inline] pub fn gtpr_mut(&self) -> *mut Gtpr { 
        self.gtpr_reg().ptr()
    }

    #[doc="Get the *const pointer for the GTPR register."]
    #[inline] pub fn gtpr_ptr(&self) -> *const Gtpr { 
        self.gtpr_reg().ptr()
    }

    #[doc="Read the GTPR register."]
    #[inline] pub fn gtpr(&self) -> Gtpr { 
        self.gtpr_reg().read()
    }

    #[doc="Write the GTPR register."]
    #[inline] pub fn write_gtpr(&self, value: Gtpr) -> &Self { 
        self.gtpr_reg().write(value);
        self
    }

    #[doc="Set the GTPR register."]
    #[inline] pub fn set_gtpr<F: FnOnce(Gtpr) -> Gtpr>(&self, f: F) -> &Self {
        self.gtpr_reg().set(f);
        self
    }

    #[doc="Modify the GTPR register."]
    #[inline] pub fn with_gtpr<F: FnOnce(Gtpr) -> Gtpr>(&self, f: F) -> &Self {
        self.gtpr_reg().with(f);
        self
    }

    #[doc="Get the RTOR Register."]
    #[inline] pub fn rtor_reg(&self) -> Register<Rtor> { 
        Register::new(self.0 as *mut Rtor, 0x14)
    }

    #[doc="Get the *mut pointer for the RTOR register."]
    #[inline] pub fn rtor_mut(&self) -> *mut Rtor { 
        self.rtor_reg().ptr()
    }

    #[doc="Get the *const pointer for the RTOR register."]
    #[inline] pub fn rtor_ptr(&self) -> *const Rtor { 
        self.rtor_reg().ptr()
    }

    #[doc="Read the RTOR register."]
    #[inline] pub fn rtor(&self) -> Rtor { 
        self.rtor_reg().read()
    }

    #[doc="Write the RTOR register."]
    #[inline] pub fn write_rtor(&self, value: Rtor) -> &Self { 
        self.rtor_reg().write(value);
        self
    }

    #[doc="Set the RTOR register."]
    #[inline] pub fn set_rtor<F: FnOnce(Rtor) -> Rtor>(&self, f: F) -> &Self {
        self.rtor_reg().set(f);
        self
    }

    #[doc="Modify the RTOR register."]
    #[inline] pub fn with_rtor<F: FnOnce(Rtor) -> Rtor>(&self, f: F) -> &Self {
        self.rtor_reg().with(f);
        self
    }

    #[doc="Get the RQR Register."]
    #[inline] pub fn rqr_reg(&self) -> Register<Rqr> { 
        Register::new(self.0 as *mut Rqr, 0x18)
    }

    #[doc="Get the *mut pointer for the RQR register."]
    #[inline] pub fn rqr_mut(&self) -> *mut Rqr { 
        self.rqr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RQR register."]
    #[inline] pub fn rqr_ptr(&self) -> *const Rqr { 
        self.rqr_reg().ptr()
    }

    #[doc="Read the RQR register."]
    #[inline] pub fn rqr(&self) -> Rqr { 
        self.rqr_reg().read()
    }

    #[doc="Write the RQR register."]
    #[inline] pub fn write_rqr(&self, value: Rqr) -> &Self { 
        self.rqr_reg().write(value);
        self
    }

    #[doc="Set the RQR register."]
    #[inline] pub fn set_rqr<F: FnOnce(Rqr) -> Rqr>(&self, f: F) -> &Self {
        self.rqr_reg().set(f);
        self
    }

    #[doc="Modify the RQR register."]
    #[inline] pub fn with_rqr<F: FnOnce(Rqr) -> Rqr>(&self, f: F) -> &Self {
        self.rqr_reg().with(f);
        self
    }

    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> Register<Isr> { 
        Register::new(self.0 as *mut Isr, 0x1c)
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        self.isr_reg().read()
    }

    #[doc="Get the ICR Register."]
    #[inline] pub fn icr_reg(&self) -> Register<Icr> { 
        Register::new(self.0 as *mut Icr, 0x20)
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        self.icr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
        self.icr_reg().ptr()
    }

    #[doc="Read the ICR register."]
    #[inline] pub fn icr(&self) -> Icr { 
        self.icr_reg().read()
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn write_icr(&self, value: Icr) -> &Self { 
        self.icr_reg().write(value);
        self
    }

    #[doc="Set the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        self.icr_reg().set(f);
        self
    }

    #[doc="Modify the ICR register."]
    #[inline] pub fn with_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        self.icr_reg().with(f);
        self
    }

    #[doc="Get the RDR Register."]
    #[inline] pub fn rdr_reg(&self) -> Register<Rdr> { 
        Register::new(self.0 as *mut Rdr, 0x24)
    }

    #[doc="Get the *mut pointer for the RDR register."]
    #[inline] pub fn rdr_mut(&self) -> *mut Rdr { 
        self.rdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RDR register."]
    #[inline] pub fn rdr_ptr(&self) -> *const Rdr { 
        self.rdr_reg().ptr()
    }

    #[doc="Read the RDR register."]
    #[inline] pub fn rdr(&self) -> Rdr { 
        self.rdr_reg().read()
    }

    #[doc="Get the TDR Register."]
    #[inline] pub fn tdr_reg(&self) -> Register<Tdr> { 
        Register::new(self.0 as *mut Tdr, 0x28)
    }

    #[doc="Get the *mut pointer for the TDR register."]
    #[inline] pub fn tdr_mut(&self) -> *mut Tdr { 
        self.tdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TDR register."]
    #[inline] pub fn tdr_ptr(&self) -> *const Tdr { 
        self.tdr_reg().ptr()
    }

    #[doc="Read the TDR register."]
    #[inline] pub fn tdr(&self) -> Tdr { 
        self.tdr_reg().read()
    }

    #[doc="Write the TDR register."]
    #[inline] pub fn write_tdr(&self, value: Tdr) -> &Self { 
        self.tdr_reg().write(value);
        self
    }

    #[doc="Set the TDR register."]
    #[inline] pub fn set_tdr<F: FnOnce(Tdr) -> Tdr>(&self, f: F) -> &Self {
        self.tdr_reg().set(f);
        self
    }

    #[doc="Modify the TDR register."]
    #[inline] pub fn with_tdr<F: FnOnce(Tdr) -> Tdr>(&self, f: F) -> &Self {
        self.tdr_reg().with(f);
        self
    }

}

#[doc="Control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Word length - Bit 1"]
    #[inline] pub fn m1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if M1 != 0"]
    #[inline] pub fn test_m1(&self) -> bool {
        self.m1() != 0
    }

    #[doc="Sets the M1 field."]
    #[inline] pub fn set_m1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="End of Block interrupt enable"]
    #[inline] pub fn eobie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if EOBIE != 0"]
    #[inline] pub fn test_eobie(&self) -> bool {
        self.eobie() != 0
    }

    #[doc="Sets the EOBIE field."]
    #[inline] pub fn set_eobie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Receiver timeout interrupt enable"]
    #[inline] pub fn rtoie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if RTOIE != 0"]
    #[inline] pub fn test_rtoie(&self) -> bool {
        self.rtoie() != 0
    }

    #[doc="Sets the RTOIE field."]
    #[inline] pub fn set_rtoie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Driver Enable assertion time"]
    #[inline] pub fn deat(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1f) as u8) } // [25:21]
    }

    #[doc="Returns true if DEAT != 0"]
    #[inline] pub fn test_deat(&self) -> bool {
        self.deat() != 0
    }

    #[doc="Sets the DEAT field."]
    #[inline] pub fn set_deat<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Driver Enable deassertion time"]
    #[inline] pub fn dedt(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
    }

    #[doc="Returns true if DEDT != 0"]
    #[inline] pub fn test_dedt(&self) -> bool {
        self.dedt() != 0
    }

    #[doc="Sets the DEDT field."]
    #[inline] pub fn set_dedt<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Oversampling mode"]
    #[inline] pub fn over8(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if OVER8 != 0"]
    #[inline] pub fn test_over8(&self) -> bool {
        self.over8() != 0
    }

    #[doc="Sets the OVER8 field."]
    #[inline] pub fn set_over8<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Character match interrupt enable"]
    #[inline] pub fn cmie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CMIE != 0"]
    #[inline] pub fn test_cmie(&self) -> bool {
        self.cmie() != 0
    }

    #[doc="Sets the CMIE field."]
    #[inline] pub fn set_cmie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Mute mode enable"]
    #[inline] pub fn mme(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if MME != 0"]
    #[inline] pub fn test_mme(&self) -> bool {
        self.mme() != 0
    }

    #[doc="Sets the MME field."]
    #[inline] pub fn set_mme<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Word length - Bit 0"]
    #[inline] pub fn m0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if M0 != 0"]
    #[inline] pub fn test_m0(&self) -> bool {
        self.m0() != 0
    }

    #[doc="Sets the M0 field."]
    #[inline] pub fn set_m0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receiver wakeup method"]
    #[inline] pub fn wake(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WAKE != 0"]
    #[inline] pub fn test_wake(&self) -> bool {
        self.wake() != 0
    }

    #[doc="Sets the WAKE field."]
    #[inline] pub fn set_wake<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Parity control enable"]
    #[inline] pub fn pce(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PCE != 0"]
    #[inline] pub fn test_pce(&self) -> bool {
        self.pce() != 0
    }

    #[doc="Sets the PCE field."]
    #[inline] pub fn set_pce<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Parity selection"]
    #[inline] pub fn ps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PS != 0"]
    #[inline] pub fn test_ps(&self) -> bool {
        self.ps() != 0
    }

    #[doc="Sets the PS field."]
    #[inline] pub fn set_ps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="PE interrupt enable"]
    #[inline] pub fn peie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PEIE != 0"]
    #[inline] pub fn test_peie(&self) -> bool {
        self.peie() != 0
    }

    #[doc="Sets the PEIE field."]
    #[inline] pub fn set_peie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="interrupt enable"]
    #[inline] pub fn txeie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXEIE != 0"]
    #[inline] pub fn test_txeie(&self) -> bool {
        self.txeie() != 0
    }

    #[doc="Sets the TXEIE field."]
    #[inline] pub fn set_txeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmission complete interrupt enable"]
    #[inline] pub fn tcie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="RXNE interrupt enable"]
    #[inline] pub fn rxneie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXNEIE != 0"]
    #[inline] pub fn test_rxneie(&self) -> bool {
        self.rxneie() != 0
    }

    #[doc="Sets the RXNEIE field."]
    #[inline] pub fn set_rxneie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="IDLE interrupt enable"]
    #[inline] pub fn idleie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IDLEIE != 0"]
    #[inline] pub fn test_idleie(&self) -> bool {
        self.idleie() != 0
    }

    #[doc="Sets the IDLEIE field."]
    #[inline] pub fn set_idleie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmitter enable"]
    #[inline] pub fn te(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TE != 0"]
    #[inline] pub fn test_te(&self) -> bool {
        self.te() != 0
    }

    #[doc="Sets the TE field."]
    #[inline] pub fn set_te<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receiver enable"]
    #[inline] pub fn re(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RE != 0"]
    #[inline] pub fn test_re(&self) -> bool {
        self.re() != 0
    }

    #[doc="Sets the RE field."]
    #[inline] pub fn set_re<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="USART enable in Stop mode"]
    #[inline] pub fn uesm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if UESM != 0"]
    #[inline] pub fn test_uesm(&self) -> bool {
        self.uesm() != 0
    }

    #[doc="Sets the UESM field."]
    #[inline] pub fn set_uesm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="USART enable"]
    #[inline] pub fn ue(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UE != 0"]
    #[inline] pub fn test_ue(&self) -> bool {
        self.ue() != 0
    }

    #[doc="Sets the UE field."]
    #[inline] pub fn set_ue<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr1(other)
    }
}

impl ::core::fmt::Display for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.m1() != 0 { try!(write!(f, " m1"))}
        if self.eobie() != 0 { try!(write!(f, " eobie"))}
        if self.rtoie() != 0 { try!(write!(f, " rtoie"))}
        if self.deat() != 0 { try!(write!(f, " deat=0x{:x}", self.deat()))}
        if self.dedt() != 0 { try!(write!(f, " dedt=0x{:x}", self.dedt()))}
        if self.over8() != 0 { try!(write!(f, " over8"))}
        if self.cmie() != 0 { try!(write!(f, " cmie"))}
        if self.mme() != 0 { try!(write!(f, " mme"))}
        if self.m0() != 0 { try!(write!(f, " m0"))}
        if self.wake() != 0 { try!(write!(f, " wake"))}
        if self.pce() != 0 { try!(write!(f, " pce"))}
        if self.ps() != 0 { try!(write!(f, " ps"))}
        if self.peie() != 0 { try!(write!(f, " peie"))}
        if self.txeie() != 0 { try!(write!(f, " txeie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.rxneie() != 0 { try!(write!(f, " rxneie"))}
        if self.idleie() != 0 { try!(write!(f, " idleie"))}
        if self.te() != 0 { try!(write!(f, " te"))}
        if self.re() != 0 { try!(write!(f, " re"))}
        if self.uesm() != 0 { try!(write!(f, " uesm"))}
        if self.ue() != 0 { try!(write!(f, " ue"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="Address of the USART node"]
    #[inline] pub fn add4(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if ADD4 != 0"]
    #[inline] pub fn test_add4(&self) -> bool {
        self.add4() != 0
    }

    #[doc="Sets the ADD4 field."]
    #[inline] pub fn set_add4<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Address of the USART node"]
    #[inline] pub fn add0(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if ADD0 != 0"]
    #[inline] pub fn test_add0(&self) -> bool {
        self.add0() != 0
    }

    #[doc="Sets the ADD0 field."]
    #[inline] pub fn set_add0<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Receiver timeout enable"]
    #[inline] pub fn rtoen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if RTOEN != 0"]
    #[inline] pub fn test_rtoen(&self) -> bool {
        self.rtoen() != 0
    }

    #[doc="Sets the RTOEN field."]
    #[inline] pub fn set_rtoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Auto baud rate mode"]
    #[inline] pub fn abrmod(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x3) as u8) } // [22:21]
    }

    #[doc="Returns true if ABRMOD != 0"]
    #[inline] pub fn test_abrmod(&self) -> bool {
        self.abrmod() != 0
    }

    #[doc="Sets the ABRMOD field."]
    #[inline] pub fn set_abrmod<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Auto baud rate enable"]
    #[inline] pub fn abren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if ABREN != 0"]
    #[inline] pub fn test_abren(&self) -> bool {
        self.abren() != 0
    }

    #[doc="Sets the ABREN field."]
    #[inline] pub fn set_abren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Most significant bit first"]
    #[inline] pub fn msbfirst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if MSBFIRST != 0"]
    #[inline] pub fn test_msbfirst(&self) -> bool {
        self.msbfirst() != 0
    }

    #[doc="Sets the MSBFIRST field."]
    #[inline] pub fn set_msbfirst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Binary data inversion"]
    #[inline] pub fn datainv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DATAINV != 0"]
    #[inline] pub fn test_datainv(&self) -> bool {
        self.datainv() != 0
    }

    #[doc="Sets the DATAINV field."]
    #[inline] pub fn set_datainv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TX pin active level inversion"]
    #[inline] pub fn txinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TXINV != 0"]
    #[inline] pub fn test_txinv(&self) -> bool {
        self.txinv() != 0
    }

    #[doc="Sets the TXINV field."]
    #[inline] pub fn set_txinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="RX pin active level inversion"]
    #[inline] pub fn rxinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RXINV != 0"]
    #[inline] pub fn test_rxinv(&self) -> bool {
        self.rxinv() != 0
    }

    #[doc="Sets the RXINV field."]
    #[inline] pub fn set_rxinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Swap TX/RX pins"]
    #[inline] pub fn swap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SWAP != 0"]
    #[inline] pub fn test_swap(&self) -> bool {
        self.swap() != 0
    }

    #[doc="Sets the SWAP field."]
    #[inline] pub fn set_swap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="LIN mode enable"]
    #[inline] pub fn linen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if LINEN != 0"]
    #[inline] pub fn test_linen(&self) -> bool {
        self.linen() != 0
    }

    #[doc="Sets the LINEN field."]
    #[inline] pub fn set_linen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="STOP bits"]
    #[inline] pub fn stop(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Clock enable"]
    #[inline] pub fn clken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CLKEN != 0"]
    #[inline] pub fn test_clken(&self) -> bool {
        self.clken() != 0
    }

    #[doc="Sets the CLKEN field."]
    #[inline] pub fn set_clken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Clock polarity"]
    #[inline] pub fn cpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Clock phase"]
    #[inline] pub fn cpha(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CPHA != 0"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Sets the CPHA field."]
    #[inline] pub fn set_cpha<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Last bit clock pulse"]
    #[inline] pub fn lbcl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LBCL != 0"]
    #[inline] pub fn test_lbcl(&self) -> bool {
        self.lbcl() != 0
    }

    #[doc="Sets the LBCL field."]
    #[inline] pub fn set_lbcl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LIN break detection interrupt enable"]
    #[inline] pub fn lbdie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LBDIE != 0"]
    #[inline] pub fn test_lbdie(&self) -> bool {
        self.lbdie() != 0
    }

    #[doc="Sets the LBDIE field."]
    #[inline] pub fn set_lbdie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="LIN break detection length"]
    #[inline] pub fn lbdl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LBDL != 0"]
    #[inline] pub fn test_lbdl(&self) -> bool {
        self.lbdl() != 0
    }

    #[doc="Sets the LBDL field."]
    #[inline] pub fn set_lbdl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="7-bit Address Detection/4-bit Address Detection"]
    #[inline] pub fn addm7(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ADDM7 != 0"]
    #[inline] pub fn test_addm7(&self) -> bool {
        self.addm7() != 0
    }

    #[doc="Sets the ADDM7 field."]
    #[inline] pub fn set_addm7<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr2(other)
    }
}

impl ::core::fmt::Display for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.add4() != 0 { try!(write!(f, " add4=0x{:x}", self.add4()))}
        if self.add0() != 0 { try!(write!(f, " add0=0x{:x}", self.add0()))}
        if self.rtoen() != 0 { try!(write!(f, " rtoen"))}
        if self.abrmod() != 0 { try!(write!(f, " abrmod=0x{:x}", self.abrmod()))}
        if self.abren() != 0 { try!(write!(f, " abren"))}
        if self.msbfirst() != 0 { try!(write!(f, " msbfirst"))}
        if self.datainv() != 0 { try!(write!(f, " datainv"))}
        if self.txinv() != 0 { try!(write!(f, " txinv"))}
        if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
        if self.swap() != 0 { try!(write!(f, " swap"))}
        if self.linen() != 0 { try!(write!(f, " linen"))}
        if self.stop() != 0 { try!(write!(f, " stop=0x{:x}", self.stop()))}
        if self.clken() != 0 { try!(write!(f, " clken"))}
        if self.cpol() != 0 { try!(write!(f, " cpol"))}
        if self.cpha() != 0 { try!(write!(f, " cpha"))}
        if self.lbcl() != 0 { try!(write!(f, " lbcl"))}
        if self.lbdie() != 0 { try!(write!(f, " lbdie"))}
        if self.lbdl() != 0 { try!(write!(f, " lbdl"))}
        if self.addm7() != 0 { try!(write!(f, " addm7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr3(pub u32);
impl Cr3 {
    #[doc="Wakeup from Stop mode interrupt enable"]
    #[inline] pub fn wufie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if WUFIE != 0"]
    #[inline] pub fn test_wufie(&self) -> bool {
        self.wufie() != 0
    }

    #[doc="Sets the WUFIE field."]
    #[inline] pub fn set_wufie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Wakeup from Stop mode interrupt flag selection"]
    #[inline] pub fn wus(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if WUS != 0"]
    #[inline] pub fn test_wus(&self) -> bool {
        self.wus() != 0
    }

    #[doc="Sets the WUS field."]
    #[inline] pub fn set_wus<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Smartcard auto-retry count"]
    #[inline] pub fn scarcnt(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x7) as u8) } // [19:17]
    }

    #[doc="Returns true if SCARCNT != 0"]
    #[inline] pub fn test_scarcnt(&self) -> bool {
        self.scarcnt() != 0
    }

    #[doc="Sets the SCARCNT field."]
    #[inline] pub fn set_scarcnt<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Driver enable polarity selection"]
    #[inline] pub fn dep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DEP != 0"]
    #[inline] pub fn test_dep(&self) -> bool {
        self.dep() != 0
    }

    #[doc="Sets the DEP field."]
    #[inline] pub fn set_dep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Driver enable mode"]
    #[inline] pub fn dem(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DEM != 0"]
    #[inline] pub fn test_dem(&self) -> bool {
        self.dem() != 0
    }

    #[doc="Sets the DEM field."]
    #[inline] pub fn set_dem<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="DMA Disable on Reception Error"]
    #[inline] pub fn ddre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DDRE != 0"]
    #[inline] pub fn test_ddre(&self) -> bool {
        self.ddre() != 0
    }

    #[doc="Sets the DDRE field."]
    #[inline] pub fn set_ddre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Overrun Disable"]
    #[inline] pub fn ovrdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if OVRDIS != 0"]
    #[inline] pub fn test_ovrdis(&self) -> bool {
        self.ovrdis() != 0
    }

    #[doc="Sets the OVRDIS field."]
    #[inline] pub fn set_ovrdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="One sample bit method enable"]
    #[inline] pub fn onebit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ONEBIT != 0"]
    #[inline] pub fn test_onebit(&self) -> bool {
        self.onebit() != 0
    }

    #[doc="Sets the ONEBIT field."]
    #[inline] pub fn set_onebit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CTS interrupt enable"]
    #[inline] pub fn ctsie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTSIE != 0"]
    #[inline] pub fn test_ctsie(&self) -> bool {
        self.ctsie() != 0
    }

    #[doc="Sets the CTSIE field."]
    #[inline] pub fn set_ctsie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CTS enable"]
    #[inline] pub fn ctse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTSE != 0"]
    #[inline] pub fn test_ctse(&self) -> bool {
        self.ctse() != 0
    }

    #[doc="Sets the CTSE field."]
    #[inline] pub fn set_ctse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="RTS enable"]
    #[inline] pub fn rtse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RTSE != 0"]
    #[inline] pub fn test_rtse(&self) -> bool {
        self.rtse() != 0
    }

    #[doc="Sets the RTSE field."]
    #[inline] pub fn set_rtse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DMA enable transmitter"]
    #[inline] pub fn dmat(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DMAT != 0"]
    #[inline] pub fn test_dmat(&self) -> bool {
        self.dmat() != 0
    }

    #[doc="Sets the DMAT field."]
    #[inline] pub fn set_dmat<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DMA enable receiver"]
    #[inline] pub fn dmar(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DMAR != 0"]
    #[inline] pub fn test_dmar(&self) -> bool {
        self.dmar() != 0
    }

    #[doc="Sets the DMAR field."]
    #[inline] pub fn set_dmar<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Smartcard mode enable"]
    #[inline] pub fn scen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SCEN != 0"]
    #[inline] pub fn test_scen(&self) -> bool {
        self.scen() != 0
    }

    #[doc="Sets the SCEN field."]
    #[inline] pub fn set_scen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Smartcard NACK enable"]
    #[inline] pub fn nack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NACK != 0"]
    #[inline] pub fn test_nack(&self) -> bool {
        self.nack() != 0
    }

    #[doc="Sets the NACK field."]
    #[inline] pub fn set_nack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Half-duplex selection"]
    #[inline] pub fn hdsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HDSEL != 0"]
    #[inline] pub fn test_hdsel(&self) -> bool {
        self.hdsel() != 0
    }

    #[doc="Sets the HDSEL field."]
    #[inline] pub fn set_hdsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IrDA low-power"]
    #[inline] pub fn irlp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IRLP != 0"]
    #[inline] pub fn test_irlp(&self) -> bool {
        self.irlp() != 0
    }

    #[doc="Sets the IRLP field."]
    #[inline] pub fn set_irlp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IrDA mode enable"]
    #[inline] pub fn iren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IREN != 0"]
    #[inline] pub fn test_iren(&self) -> bool {
        self.iren() != 0
    }

    #[doc="Sets the IREN field."]
    #[inline] pub fn set_iren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn eie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EIE != 0"]
    #[inline] pub fn test_eie(&self) -> bool {
        self.eie() != 0
    }

    #[doc="Sets the EIE field."]
    #[inline] pub fn set_eie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr3(other)
    }
}

impl ::core::fmt::Display for Cr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wufie() != 0 { try!(write!(f, " wufie"))}
        if self.wus() != 0 { try!(write!(f, " wus=0x{:x}", self.wus()))}
        if self.scarcnt() != 0 { try!(write!(f, " scarcnt=0x{:x}", self.scarcnt()))}
        if self.dep() != 0 { try!(write!(f, " dep"))}
        if self.dem() != 0 { try!(write!(f, " dem"))}
        if self.ddre() != 0 { try!(write!(f, " ddre"))}
        if self.ovrdis() != 0 { try!(write!(f, " ovrdis"))}
        if self.onebit() != 0 { try!(write!(f, " onebit"))}
        if self.ctsie() != 0 { try!(write!(f, " ctsie"))}
        if self.ctse() != 0 { try!(write!(f, " ctse"))}
        if self.rtse() != 0 { try!(write!(f, " rtse"))}
        if self.dmat() != 0 { try!(write!(f, " dmat"))}
        if self.dmar() != 0 { try!(write!(f, " dmar"))}
        if self.scen() != 0 { try!(write!(f, " scen"))}
        if self.nack() != 0 { try!(write!(f, " nack"))}
        if self.hdsel() != 0 { try!(write!(f, " hdsel"))}
        if self.irlp() != 0 { try!(write!(f, " irlp"))}
        if self.iren() != 0 { try!(write!(f, " iren"))}
        if self.eie() != 0 { try!(write!(f, " eie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Baud rate register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc="mantissa of USARTDIV"]
    #[inline] pub fn div_mantissa(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
    }

    #[doc="Returns true if DIV_Mantissa != 0"]
    #[inline] pub fn test_div_mantissa(&self) -> bool {
        self.div_mantissa() != 0
    }

    #[doc="Sets the DIV_Mantissa field."]
    #[inline] pub fn set_div_mantissa<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="fraction of USARTDIV"]
    #[inline] pub fn div_fraction(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DIV_Fraction != 0"]
    #[inline] pub fn test_div_fraction(&self) -> bool {
        self.div_fraction() != 0
    }

    #[doc="Sets the DIV_Fraction field."]
    #[inline] pub fn set_div_fraction<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Brr {
    #[inline]
    fn from(other: u32) -> Self {
         Brr(other)
    }
}

impl ::core::fmt::Display for Brr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Brr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.div_mantissa() != 0 { try!(write!(f, " div_mantissa=0x{:x}", self.div_mantissa()))}
        if self.div_fraction() != 0 { try!(write!(f, " div_fraction=0x{:x}", self.div_fraction()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Guard time and prescaler register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gtpr(pub u32);
impl Gtpr {
    #[doc="Guard time value"]
    #[inline] pub fn gt(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if GT != 0"]
    #[inline] pub fn test_gt(&self) -> bool {
        self.gt() != 0
    }

    #[doc="Sets the GT field."]
    #[inline] pub fn set_gt<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Prescaler value"]
    #[inline] pub fn psc(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PSC != 0"]
    #[inline] pub fn test_psc(&self) -> bool {
        self.psc() != 0
    }

    #[doc="Sets the PSC field."]
    #[inline] pub fn set_psc<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Gtpr {
    #[inline]
    fn from(other: u32) -> Self {
         Gtpr(other)
    }
}

impl ::core::fmt::Display for Gtpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gtpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gt() != 0 { try!(write!(f, " gt=0x{:x}", self.gt()))}
        if self.psc() != 0 { try!(write!(f, " psc=0x{:x}", self.psc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receiver timeout register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rtor(pub u32);
impl Rtor {
    #[doc="Block Length"]
    #[inline] pub fn blen(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if BLEN != 0"]
    #[inline] pub fn test_blen(&self) -> bool {
        self.blen() != 0
    }

    #[doc="Sets the BLEN field."]
    #[inline] pub fn set_blen<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Receiver timeout value"]
    #[inline] pub fn rto(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if RTO != 0"]
    #[inline] pub fn test_rto(&self) -> bool {
        self.rto() != 0
    }

    #[doc="Sets the RTO field."]
    #[inline] pub fn set_rto<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rtor {
    #[inline]
    fn from(other: u32) -> Self {
         Rtor(other)
    }
}

impl ::core::fmt::Display for Rtor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rtor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.blen() != 0 { try!(write!(f, " blen=0x{:x}", self.blen()))}
        if self.rto() != 0 { try!(write!(f, " rto=0x{:x}", self.rto()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Request register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rqr(pub u32);
impl Rqr {
    #[doc="Transmit data flush request"]
    #[inline] pub fn txfrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXFRQ != 0"]
    #[inline] pub fn test_txfrq(&self) -> bool {
        self.txfrq() != 0
    }

    #[doc="Sets the TXFRQ field."]
    #[inline] pub fn set_txfrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receive data flush request"]
    #[inline] pub fn rxfrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXFRQ != 0"]
    #[inline] pub fn test_rxfrq(&self) -> bool {
        self.rxfrq() != 0
    }

    #[doc="Sets the RXFRQ field."]
    #[inline] pub fn set_rxfrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Mute mode request"]
    #[inline] pub fn mmrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MMRQ != 0"]
    #[inline] pub fn test_mmrq(&self) -> bool {
        self.mmrq() != 0
    }

    #[doc="Sets the MMRQ field."]
    #[inline] pub fn set_mmrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Send break request"]
    #[inline] pub fn sbkrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SBKRQ != 0"]
    #[inline] pub fn test_sbkrq(&self) -> bool {
        self.sbkrq() != 0
    }

    #[doc="Sets the SBKRQ field."]
    #[inline] pub fn set_sbkrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Auto baud rate request"]
    #[inline] pub fn abrrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ABRRQ != 0"]
    #[inline] pub fn test_abrrq(&self) -> bool {
        self.abrrq() != 0
    }

    #[doc="Sets the ABRRQ field."]
    #[inline] pub fn set_abrrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rqr {
    #[inline]
    fn from(other: u32) -> Self {
         Rqr(other)
    }
}

impl ::core::fmt::Display for Rqr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rqr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfrq() != 0 { try!(write!(f, " txfrq"))}
        if self.rxfrq() != 0 { try!(write!(f, " rxfrq"))}
        if self.mmrq() != 0 { try!(write!(f, " mmrq"))}
        if self.sbkrq() != 0 { try!(write!(f, " sbkrq"))}
        if self.abrrq() != 0 { try!(write!(f, " abrrq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt & status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Receive enable acknowledge flag"]
    #[inline] pub fn reack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if REACK != 0"]
    #[inline] pub fn test_reack(&self) -> bool {
        self.reack() != 0
    }

    #[doc="Sets the REACK field."]
    #[inline] pub fn set_reack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Transmit enable acknowledge flag"]
    #[inline] pub fn teack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if TEACK != 0"]
    #[inline] pub fn test_teack(&self) -> bool {
        self.teack() != 0
    }

    #[doc="Sets the TEACK field."]
    #[inline] pub fn set_teack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Wakeup from Stop mode flag"]
    #[inline] pub fn wuf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if WUF != 0"]
    #[inline] pub fn test_wuf(&self) -> bool {
        self.wuf() != 0
    }

    #[doc="Sets the WUF field."]
    #[inline] pub fn set_wuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Receiver wakeup from Mute mode"]
    #[inline] pub fn rwu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RWU != 0"]
    #[inline] pub fn test_rwu(&self) -> bool {
        self.rwu() != 0
    }

    #[doc="Sets the RWU field."]
    #[inline] pub fn set_rwu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Send break flag"]
    #[inline] pub fn sbkf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if SBKF != 0"]
    #[inline] pub fn test_sbkf(&self) -> bool {
        self.sbkf() != 0
    }

    #[doc="Sets the SBKF field."]
    #[inline] pub fn set_sbkf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="character match flag"]
    #[inline] pub fn cmf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CMF != 0"]
    #[inline] pub fn test_cmf(&self) -> bool {
        self.cmf() != 0
    }

    #[doc="Sets the CMF field."]
    #[inline] pub fn set_cmf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Busy flag"]
    #[inline] pub fn busy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Auto baud rate flag"]
    #[inline] pub fn abrf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ABRF != 0"]
    #[inline] pub fn test_abrf(&self) -> bool {
        self.abrf() != 0
    }

    #[doc="Sets the ABRF field."]
    #[inline] pub fn set_abrf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Auto baud rate error"]
    #[inline] pub fn abre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ABRE != 0"]
    #[inline] pub fn test_abre(&self) -> bool {
        self.abre() != 0
    }

    #[doc="Sets the ABRE field."]
    #[inline] pub fn set_abre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="End of block flag"]
    #[inline] pub fn eobf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if EOBF != 0"]
    #[inline] pub fn test_eobf(&self) -> bool {
        self.eobf() != 0
    }

    #[doc="Sets the EOBF field."]
    #[inline] pub fn set_eobf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receiver timeout"]
    #[inline] pub fn rtof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RTOF != 0"]
    #[inline] pub fn test_rtof(&self) -> bool {
        self.rtof() != 0
    }

    #[doc="Sets the RTOF field."]
    #[inline] pub fn set_rtof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CTS flag"]
    #[inline] pub fn cts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CTS != 0"]
    #[inline] pub fn test_cts(&self) -> bool {
        self.cts() != 0
    }

    #[doc="Sets the CTS field."]
    #[inline] pub fn set_cts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="CTS interrupt flag"]
    #[inline] pub fn ctsif(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTSIF != 0"]
    #[inline] pub fn test_ctsif(&self) -> bool {
        self.ctsif() != 0
    }

    #[doc="Sets the CTSIF field."]
    #[inline] pub fn set_ctsif<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="LIN break detection flag"]
    #[inline] pub fn lbdf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LBDF != 0"]
    #[inline] pub fn test_lbdf(&self) -> bool {
        self.lbdf() != 0
    }

    #[doc="Sets the LBDF field."]
    #[inline] pub fn set_lbdf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmit data register empty"]
    #[inline] pub fn txe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TXE field."]
    #[inline] pub fn set_txe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmission complete"]
    #[inline] pub fn tc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TC != 0"]
    #[inline] pub fn test_tc(&self) -> bool {
        self.tc() != 0
    }

    #[doc="Sets the TC field."]
    #[inline] pub fn set_tc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Read data register not empty"]
    #[inline] pub fn rxne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RXNE != 0"]
    #[inline] pub fn test_rxne(&self) -> bool {
        self.rxne() != 0
    }

    #[doc="Sets the RXNE field."]
    #[inline] pub fn set_rxne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Idle line detected"]
    #[inline] pub fn idle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IDLE != 0"]
    #[inline] pub fn test_idle(&self) -> bool {
        self.idle() != 0
    }

    #[doc="Sets the IDLE field."]
    #[inline] pub fn set_idle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Overrun error"]
    #[inline] pub fn ore(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ORE != 0"]
    #[inline] pub fn test_ore(&self) -> bool {
        self.ore() != 0
    }

    #[doc="Sets the ORE field."]
    #[inline] pub fn set_ore<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Noise detected flag"]
    #[inline] pub fn nf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NF != 0"]
    #[inline] pub fn test_nf(&self) -> bool {
        self.nf() != 0
    }

    #[doc="Sets the NF field."]
    #[inline] pub fn set_nf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Framing error"]
    #[inline] pub fn fe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FE != 0"]
    #[inline] pub fn test_fe(&self) -> bool {
        self.fe() != 0
    }

    #[doc="Sets the FE field."]
    #[inline] pub fn set_fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Parity error"]
    #[inline] pub fn pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Isr(other)
    }
}

impl ::core::fmt::Display for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.reack() != 0 { try!(write!(f, " reack"))}
        if self.teack() != 0 { try!(write!(f, " teack"))}
        if self.wuf() != 0 { try!(write!(f, " wuf"))}
        if self.rwu() != 0 { try!(write!(f, " rwu"))}
        if self.sbkf() != 0 { try!(write!(f, " sbkf"))}
        if self.cmf() != 0 { try!(write!(f, " cmf"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.abrf() != 0 { try!(write!(f, " abrf"))}
        if self.abre() != 0 { try!(write!(f, " abre"))}
        if self.eobf() != 0 { try!(write!(f, " eobf"))}
        if self.rtof() != 0 { try!(write!(f, " rtof"))}
        if self.cts() != 0 { try!(write!(f, " cts"))}
        if self.ctsif() != 0 { try!(write!(f, " ctsif"))}
        if self.lbdf() != 0 { try!(write!(f, " lbdf"))}
        if self.txe() != 0 { try!(write!(f, " txe"))}
        if self.tc() != 0 { try!(write!(f, " tc"))}
        if self.rxne() != 0 { try!(write!(f, " rxne"))}
        if self.idle() != 0 { try!(write!(f, " idle"))}
        if self.ore() != 0 { try!(write!(f, " ore"))}
        if self.nf() != 0 { try!(write!(f, " nf"))}
        if self.fe() != 0 { try!(write!(f, " fe"))}
        if self.pe() != 0 { try!(write!(f, " pe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt flag clear register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="Wakeup from Stop mode clear flag"]
    #[inline] pub fn wucf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if WUCF != 0"]
    #[inline] pub fn test_wucf(&self) -> bool {
        self.wucf() != 0
    }

    #[doc="Sets the WUCF field."]
    #[inline] pub fn set_wucf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Character match clear flag"]
    #[inline] pub fn cmcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CMCF != 0"]
    #[inline] pub fn test_cmcf(&self) -> bool {
        self.cmcf() != 0
    }

    #[doc="Sets the CMCF field."]
    #[inline] pub fn set_cmcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="End of timeout clear flag"]
    #[inline] pub fn eobcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if EOBCF != 0"]
    #[inline] pub fn test_eobcf(&self) -> bool {
        self.eobcf() != 0
    }

    #[doc="Sets the EOBCF field."]
    #[inline] pub fn set_eobcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receiver timeout clear flag"]
    #[inline] pub fn rtocf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RTOCF != 0"]
    #[inline] pub fn test_rtocf(&self) -> bool {
        self.rtocf() != 0
    }

    #[doc="Sets the RTOCF field."]
    #[inline] pub fn set_rtocf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="CTS clear flag"]
    #[inline] pub fn ctscf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CTSCF != 0"]
    #[inline] pub fn test_ctscf(&self) -> bool {
        self.ctscf() != 0
    }

    #[doc="Sets the CTSCF field."]
    #[inline] pub fn set_ctscf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="LIN break detection clear flag"]
    #[inline] pub fn lbdcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LBDCF != 0"]
    #[inline] pub fn test_lbdcf(&self) -> bool {
        self.lbdcf() != 0
    }

    #[doc="Sets the LBDCF field."]
    #[inline] pub fn set_lbdcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Transmission complete clear flag"]
    #[inline] pub fn tccf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TCCF != 0"]
    #[inline] pub fn test_tccf(&self) -> bool {
        self.tccf() != 0
    }

    #[doc="Sets the TCCF field."]
    #[inline] pub fn set_tccf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Idle line detected clear flag"]
    #[inline] pub fn idlecf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IDLECF != 0"]
    #[inline] pub fn test_idlecf(&self) -> bool {
        self.idlecf() != 0
    }

    #[doc="Sets the IDLECF field."]
    #[inline] pub fn set_idlecf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Overrun error clear flag"]
    #[inline] pub fn orecf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ORECF != 0"]
    #[inline] pub fn test_orecf(&self) -> bool {
        self.orecf() != 0
    }

    #[doc="Sets the ORECF field."]
    #[inline] pub fn set_orecf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Noise detected clear flag"]
    #[inline] pub fn ncf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NCF != 0"]
    #[inline] pub fn test_ncf(&self) -> bool {
        self.ncf() != 0
    }

    #[doc="Sets the NCF field."]
    #[inline] pub fn set_ncf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Framing error clear flag"]
    #[inline] pub fn fecf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FECF != 0"]
    #[inline] pub fn test_fecf(&self) -> bool {
        self.fecf() != 0
    }

    #[doc="Sets the FECF field."]
    #[inline] pub fn set_fecf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Parity error clear flag"]
    #[inline] pub fn pecf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PECF != 0"]
    #[inline] pub fn test_pecf(&self) -> bool {
        self.pecf() != 0
    }

    #[doc="Sets the PECF field."]
    #[inline] pub fn set_pecf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
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
        if self.wucf() != 0 { try!(write!(f, " wucf"))}
        if self.cmcf() != 0 { try!(write!(f, " cmcf"))}
        if self.eobcf() != 0 { try!(write!(f, " eobcf"))}
        if self.rtocf() != 0 { try!(write!(f, " rtocf"))}
        if self.ctscf() != 0 { try!(write!(f, " ctscf"))}
        if self.lbdcf() != 0 { try!(write!(f, " lbdcf"))}
        if self.tccf() != 0 { try!(write!(f, " tccf"))}
        if self.idlecf() != 0 { try!(write!(f, " idlecf"))}
        if self.orecf() != 0 { try!(write!(f, " orecf"))}
        if self.ncf() != 0 { try!(write!(f, " ncf"))}
        if self.fecf() != 0 { try!(write!(f, " fecf"))}
        if self.pecf() != 0 { try!(write!(f, " pecf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Receive data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdr(pub u32);
impl Rdr {
    #[doc="Receive data value"]
    #[inline] pub fn rdr(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if RDR != 0"]
    #[inline] pub fn test_rdr(&self) -> bool {
        self.rdr() != 0
    }

    #[doc="Sets the RDR field."]
    #[inline] pub fn set_rdr<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdr {
    #[inline]
    fn from(other: u32) -> Self {
         Rdr(other)
    }
}

impl ::core::fmt::Display for Rdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rdr() != 0 { try!(write!(f, " rdr=0x{:x}", self.rdr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Transmit data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdr(pub u32);
impl Tdr {
    #[doc="Transmit data value"]
    #[inline] pub fn tdr(&self) -> bits::U9 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1ff) as u16) } // [8:0]
    }

    #[doc="Returns true if TDR != 0"]
    #[inline] pub fn test_tdr(&self) -> bool {
        self.tdr() != 0
    }

    #[doc="Sets the TDR field."]
    #[inline] pub fn set_tdr<V: Into<bits::U9>>(mut self, value: V) -> Self {
        let value: bits::U9 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdr {
    #[inline]
    fn from(other: u32) -> Self {
         Tdr(other)
    }
}

impl ::core::fmt::Display for Tdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdr() != 0 { try!(write!(f, " tdr=0x{:x}", self.tdr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

